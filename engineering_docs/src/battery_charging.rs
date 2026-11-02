//! # Battery (3000 mAh, client-confirmed), charging and USB-C input (mandatory item E)
//!
//! ## Cell requirements (pack MPN and datasheet still required — pack is NOT APPROVED until supplied)
//! | Parameter | Requirement | Reason |
//! |---|---|---|
//! | Configuration | 1S Li-ion polymer, 3000 mAh rated (client) | BQ24074 is a 1-cell charger |
//! | Nominal / max charge voltage | 3.7 V nominal, **4.20 V** charge chemistry | BQ24074 VBAT(REG) 4.16–4.23 V; a 4.35 V cell would only be under-charged |
//! | Discharge cutoff | firmware stops chain at 3.40 V, shuts down at 3.30 V; pack protection 2.5–3.0 V | stays above pack UV; TPS63802/TPS61023 run far lower and do not protect the cell |
//! | Continuous discharge | ≥ 2.0 A (0.67 C); PCM over-current trip ≥ 3 A | computed peak 1.50 A at 3.0 V if the 0.5 A ESP32 design load overlaps a chain at its limit; ≈ 0.9 A if firmware never overlaps Wi-Fi TX and chain activity |
//! | NTC | 10 kΩ NTC in the pack to BQ24074 TS (10 kΩ fixed resistor only if the pack has none) | charge temperature qualification |
//! | Connector | JST PH 2-pin lead to S2B-PH-SM4-TB, polarity checked per pack | 2 A rating |
//!
//! ## Charger configuration (BQ24074RGTR)
//! * **RISET 1.27 kΩ** (was 1.78 kΩ for 2000 mAh): ICHG 0.62–0.78 A (0.70 A nom, 0.23 C).
//! * RTMR 68.1 kΩ: safety timer 6.7–11.5 h ≥ 5.9 h needed; the timer is slowed in DPPM/thermal
//!   regulation proportionally to the current reduction (datasheet §9.3.5), so a 500 mA source does
//!   not cause a false timeout.
//! * Input current: **hardware default USB500** (EN1 100 kΩ pull-up to OUT, EN2 100 kΩ pull-down):
//!   450–500 mA, safe for any compliant source. ILIM mode (RILIM 1.62 kΩ → 0.92–1.07 A) only after the
//!   ESP32 reads a CC voltage in the ≥ 1.5 A Rp range ([`classify_cc`]); firmware never selects more
//!   than the advertised capability. Dead battery: USB500 lets the ESP32 boot (chain off).
//! * CC sensing: CC1/CC2 each have Rd 5.1 kΩ 1 % to GND and a 100 kΩ + 10 nF tap to an ADC1 pin
//!   (ATTEN2, 0–1.6 V, ±10 mV).
//!
//! ## Runtime estimates (not guarantees; inputs in [`SCENARIOS`])
//! * typical (5-min cycle, 2 SCD41 shots, 10 s audio per 30 min, Wi-Fi every 15 min): ≈ 2.2 mA → ≈ 49 days.
//! * worst (1-min cycle, 4 SCD41 shots, 10 s audio every cycle, CPU always active, Wi-Fi every minute,
//!   10 mA display, η 0.80/0.64, 70 % usable): ≈ 71 mA → ≈ 1.2 days.
//! * Peak battery current 1.50 A at 3.0 V if Wi-Fi TX overlaps a chain at its limit, 0.93 A otherwise.

/// BQ24074 KISET (A·Ω) min/typ/max.
pub const KISET: (f64, f64, f64) = crate::passives::BQ_KISET;
/// Selected ISET resistor (Ω) for 3000 mAh.
pub const R_ISET_3000: f64 = 1_270.0;
/// Rated capacity (mAh).
pub const CAPACITY_MAH: f64 = 3000.0;

/// Charge current window (A) for `r`.
pub fn ichg(r: f64) -> (f64, f64, f64) {
    crate::passives::k_over_r(KISET, r)
}

/// USB-C source capability as read on CC (sink Rd 5.1 kΩ).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CcClass {
    /// No source / no Rp.
    None,
    /// Default USB power (500 mA USB 2.0 / 900 mA USB 3.x).
    Default,
    /// 1.5 A or 3.0 A advertised.
    AtLeast1A5,
}

/// vRd ranges at the sink (USB Type-C specification, sink side; reference not archived):
/// default 0.25–0.61 V, 1.5 A 0.70–1.16 V, 3.0 A 1.31–2.04 V.
pub const VRD: [(f64, f64); 3] = [(0.25, 0.61), (0.70, 1.16), (1.31, 2.04)];
/// ADC error in ATTEN2 (V).
pub const ADC_ERR: f64 = 0.010;

/// Conservative CC classification from the higher of the two CC readings (V).
pub fn classify_cc(v: f64) -> CcClass {
    if v < 0.20 {
        CcClass::None
    } else if v < VRD[1].0 + ADC_ERR {
        CcClass::Default
    } else {
        CcClass::AtLeast1A5
    }
}

/// Input current limit (A, max) actually configured for a class.
pub fn input_limit_max_a(c: CcClass) -> f64 {
    match c {
        CcClass::None | CcClass::Default => 0.500, // USB500 IINmax 450-500 mA
        CcClass::AtLeast1A5 => {
            crate::passives::k_over_r(crate::passives::BQ_KILIM, crate::passives::R_ILIM).2
        }
    }
}

/// Runtime scenario (assumptions explicit; no operating-time guarantee is made).
#[derive(Debug, Clone, Copy)]
pub struct Scenario {
    /// Name.
    pub name: &'static str,
    /// Cycle period (s).
    pub period_s: f64,
    /// CPU active (modem-sleep) time per cycle (s).
    pub cpu_s: f64,
    /// CPU current (mA): 39.9 (160 MHz one core) … 54.6 typ (DS-ESP-SOC table 5-9).
    pub cpu_ma: f64,
    /// Chain-on time per cycle (s).
    pub chain_s: f64,
    /// CPU light-sleeps (0.24 mA) during module conversions instead of staying active.
    pub light_sleep_waits: bool,
    /// SCD41-class single shots per cycle (150 mA·s each at 3.3 V, upper bound from the 5-min average).
    pub aux_shots: f64,
    /// Microphone capture seconds per cycle (CPU active + 1.0 mA mic).
    pub mic_s: f64,
    /// Wi-Fi seconds per cycle and average current (mA, between RX 88 and TX 355).
    pub wifi_s: f64,
    /// Wi-Fi average current (mA).
    pub wifi_ma: f64,
    /// Display average (mA) — module unselected, assumption.
    pub display_ma: f64,
    /// Battery-side sleep current (µA).
    pub sleep_ua: f64,
    /// Main buck-boost efficiency.
    pub eta_main: f64,
    /// Chain efficiency (boost × patch buck).
    pub eta_chain: f64,
    /// Usable capacity fraction (cutoff 3.3 V, ageing, temperature).
    pub usable: f64,
}

/// Typical and worst-case scenarios.
pub const SCENARIOS: [Scenario; 2] = [
    Scenario {
        name: "typical",
        period_s: 300.0,
        cpu_s: 2.0,
        cpu_ma: 39.9,
        chain_s: 12.0,
        light_sleep_waits: true,
        aux_shots: 2.0,
        mic_s: 10.0 / 6.0,
        wifi_s: 3.0 / 3.0,
        wifi_ma: 120.0,
        display_ma: 0.0,
        sleep_ua: 30.0,
        eta_main: 0.90,
        eta_chain: 0.88 * 0.88,
        usable: 0.85,
    },
    Scenario {
        name: "worst",
        period_s: 60.0,
        cpu_s: 22.0,
        cpu_ma: 54.6,
        chain_s: 22.0,
        light_sleep_waits: false,
        aux_shots: 4.0,
        mic_s: 10.0,
        wifi_s: 5.0,
        wifi_ma: 200.0,
        display_ma: 10.0,
        sleep_ua: 60.0,
        eta_main: 0.85,
        eta_chain: 0.80 * 0.80,
        usable: 0.70,
    },
];

/// Nominal battery voltage for energy conversion (V).
pub const VNOM: f64 = 3.7;

impl Scenario {
    /// Average battery current (mA).
    pub fn average_ma(&self) -> f64 {
        let t = self.period_s;
        let main_mas = self.cpu_s * self.cpu_ma
            + if self.light_sleep_waits {
                0.24 * (self.chain_s - self.cpu_s).max(0.0)
            } else {
                0.0
            }
            + self.mic_s * (39.9 + 1.0)
            + self.wifi_s * self.wifi_ma
            + self.display_ma * t;
        let chain_mas =
            self.chain_s * 8.0 * crate::chain_power_5v::PATCH_LOGIC_MA + self.aux_shots * 150.0;
        let p_mws = 3.3 * main_mas / self.eta_main + 3.3 * chain_mas / self.eta_chain;
        p_mws / t / VNOM + self.sleep_ua / 1000.0
    }
    /// Estimated runtime (days).
    pub fn runtime_days(&self) -> f64 {
        CAPACITY_MAH * self.usable / self.average_ma() / 24.0
    }
}

/// Battery peak current (A) at `vbat` for ESP32 supply design load and chain at its limit.
pub fn battery_peak_a(vbat: f64, overlap: bool) -> f64 {
    let p_main = 3.3 * 0.5 / 0.85;
    let (_, chain_max) =
        crate::passives::tps255x_window_ma(crate::chain_power_5v::R_ILIM_CHAIN_5V_K);
    let p_chain = crate::chain_power_5v::boost_vout().1 * chain_max / 1000.0 / 0.85;
    let p = if overlap {
        p_main + p_chain
    } else {
        p_main.max(p_chain + 3.3 * 0.07 / 0.85)
    };
    p / (vbat - 0.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn charge_current_and_timer_for_3000mah() {
        assert!(R_ISET_3000 >= crate::passives::BQ_RISET_RANGE.0);
        let (mn, nom, mx) = ichg(R_ISET_3000);
        assert!(
            (nom - 0.70).abs() < 0.01 && mn > 0.6 && mx < 0.8,
            "{mn} {nom} {mx}"
        );
        let need = crate::passives::charge_time_needed_h(CAPACITY_MAH, mn, 1.0);
        let (tmin, _, _) = crate::passives::bq_tmax_hours(crate::passives::R_TMR);
        assert!(tmin > need, "timer {tmin} vs {need}");
        // The old 1.78 k setting would time out for 3000 mAh.
        let need_old = crate::passives::charge_time_needed_h(CAPACITY_MAH, ichg(1_780.0).0, 1.0);
        assert!(tmin < need_old);
    }

    #[test]
    fn charger_thermal_3000mah() {
        let (_, _, mx) = ichg(R_ISET_3000);
        let p = crate::passives::bq_dissipation_w(5.25, 4.4, mx + 0.1, 3.0, mx);
        assert!(40.0 + p * 44.5 < 125.0, "{p}");
    }

    #[test]
    fn usb_c_never_exceeds_advertisement() {
        // Every reading within the spec ranges ± ADC error maps to a class whose limit is ≤ advertised.
        for (i, (lo, hi)) in VRD.iter().enumerate() {
            let advertised = [0.5, 1.5, 3.0][i];
            let mut v = lo - ADC_ERR;
            while v <= hi + ADC_ERR {
                assert!(
                    input_limit_max_a(classify_cc(v)) <= advertised + 1e-9,
                    "v={v}"
                );
                v += 0.005;
            }
        }
        assert_eq!(classify_cc(0.0), CcClass::None);
        assert!(input_limit_max_a(CcClass::AtLeast1A5) < 1.5);
    }

    #[test]
    fn runtime_scenarios_are_ordered_and_bounded() {
        let typ = SCENARIOS[0].runtime_days();
        let worst = SCENARIOS[1].runtime_days();
        assert!(typ > worst && worst > 0.5 && typ < 365.0, "{typ} {worst}");
    }

    #[test]
    fn battery_peak_requirement() {
        let a = battery_peak_a(3.0, true);
        let b = battery_peak_a(3.0, false);
        assert!(a < 2.0 && b < a && b < 1.0, "{a} {b}"); // cell ≥ 2 A continuous; JST PH 2 A
    }
}
