//! # 7. Power and thermal design (revision D: APR-1 option B, 3000 mAh battery)
//!
//! ## Approved direction (client, APR-1 option B, *subject to power validation*)
//! * Pin positions unchanged; pin 2 is redefined from `+3V3_CHAIN` to **`+5V_CHAIN`**.
//! * Every patch has a local switching regulator generating **`+3V3_LOCAL`** (patch logic,
//!   primary sensor, AUX ports). All AUX headers supply regulated 3.3 V only.
//! * Contact current ceiling: **400 mA provisional** (FH34 rated 0.5 A). It is enforced in hardware by
//!   the main-board chain switch (306–388 mA, TI equations). Thermal verification is still required.
//!
//! ## Power tree (candidates; manufacturer-neutral BOM until quotes)
//! ```text
//! USB-C 5 V (sink, CC 5.1 kΩ Rd) ─► BQ24074 linear charger + power path ─► SYS (VBAT…4.4 V)
//!                                         │ VBAT(REG) 4.2 V
//!                        3000 mAh LiPo pack (protection: pack or on-board, TBD)
//! SYS ─► TPS63021 buck-boost ─► +3V3 (ESP32-S3-WROOM-1-N16R2, display, gauge, main logic)  [alt. TPS63802]
//! SYS ─► TPS61023 boost (true disconnect) ─► +5V0 ─► TPS2553 (R_ILIM 76.8 kΩ, 306–388 mA) ─► pin 2 +5V_CHAIN
//! patch: +5V_CHAIN ─► TPS62840 buck ─► +3V3_LOCAL ─► 595/decoders/switches, primary sensor, 4 × TPS2553 ─► AUX VCC
//! ```
//!
//! ## Key results (see functions and tests)
//! | Item | Result |
//! |---|---|
//! | 525 mA "failure" | It was a voltage-limited *capacity* figure, not a load. Under option B the chain is hard-limited to **≤ 388 mA** (< 400 mA ceiling < 500 mA rating) |
//! | Worst real chain load: SCD41 (205 mA + 20 mA overhead) at patch 8, 0.10 m, η 0.80–0.88 | **206–234 mA**, patch input **4.02–4.13 V**, ground offset 0.16–0.18 V |
//! | Chain switch window incl. R tolerance + drift (passives::tps255x_window_ma(76.8)) | **302–394 mA** - min above the worst load, max below the 400 mA ceiling |
//! | Port fault coordination | Port limit (248–329 mA at 3.3 V) trips first on near patches; on far patches a port short can also trip the chain switch (latch) - fail-safe, recovered by firmware |
//! | Battery-life envelope (1 min sampling; `cargo run --example revd_numbers`) | optimistic ≈ 36 d (3.1 mA), typical ≈ 15 d (7.2 mA), worst ≈ 1.5 d (59 mA) - **estimates, not claims** |
//! | Battery peak (Wi-Fi TX + chain at limit, 3.0 V, worst η) | ≈ 1.23 A if overlapped; ≈ 0.79 A if Wi-Fi TX is never overlapped with chain activity |

use crate::model::{Quantity, Source, Status};

/// A supply rail.
#[derive(Debug, Clone, Copy)]
pub struct Rail {
    /// Net name.
    pub net: &'static str,
    /// Purpose.
    pub purpose: &'static str,
    /// Decision / design value.
    pub decision: &'static str,
    /// Status.
    pub status: Status,
}

/// Rails (revision D).
pub const RAILS: &[Rail] = &[
    Rail { net: "VBUS", purpose: "USB-C 5 V charging input", decision: "Sink only; CC1/CC2 5.1 kohm Rd; input limit default 500 mA unless the source Rp advertises more (open item).", status: Status::Proposal },
    Rail { net: "VBAT", purpose: "3000 mAh LiPo pack", decision: "Capacity CONFIRMED. Cell configuration, voltages, currents, protection: pending pack datasheet (APR-4 open items).", status: Status::ClientConfirmed },
    Rail { net: "SYS", purpose: "Charger power-path output", decision: "BQ24074: VO(REG) 4.4 V when USB present, else VBAT.", status: Status::Candidate },
    Rail { net: "+3V3", purpose: "ESP32-S3, display, main logic", decision: "TPS63021 buck-boost (component_specs BUCKBOOST: VIN 1.8-5.5 V, 2 A at VIN > 2.5 V, IQ 25/50 uA); TPS63802 (11 uA IQ) is a lower-IQ alternative to evaluate.", status: Status::Candidate },
    Rail { net: "+5V0", purpose: "Chain source", decision: "TPS61023 boost (0.5-5.5 V in, 3.7 A valley limit, true disconnect); disabled during sleep.", status: Status::Candidate },
    Rail { net: "+5V_CHAIN", purpose: "Pin 2 of the 12-pin chain", decision: "TPS2553, R_ILIM 76.8 kohm -> 306/345/388 mA min/nom/max; off during sleep.", status: Status::Approved },
    Rail { net: "+3V3_LOCAL", purpose: "Per-patch logic, primary sensor, AUX ports", decision: "TPS62840 buck (VIN 1.8-6.5 V, 750 mA, 60 nA IQ, 100 % mode).", status: Status::Approved },
    Rail { net: "AUX_VCCn", purpose: "One AUX port at a time", decision: "TPS2553 from +3V3_LOCAL, R_ILIM 93.1 kohm -> 252-324 mA.", status: Status::Proposal },
];

/// Hardware chain-current ceiling enforced by the main-board TPS2553 (mA, TI max equation at 76.8 kΩ).
pub const CHAIN_LIMIT_MAX_MA: f64 = 388.0;
/// Client provisional contact ceiling (mA).
pub const CONTACT_CEILING_MA: f64 = 400.0;
/// Minimum patch regulator input assumed for full regulation (V) - design criterion.
pub const PATCH_VIN_MIN_V: f64 = 3.6;

/// Patch input voltage and chain current (V, mA) for a 3.3 V-side load `p_out_w` on patch `k`,
/// with patch-regulator efficiency `eta` and the other patches' base load `base_w`.
/// Solves P = I·(V_src − I·R) for the chain current (option B, constant-power load).
pub fn option_b_operating_point(
    chain: &crate::calculations::Chain,
    k: u32,
    p_out_w: f64,
    eta: f64,
    base_w: f64,
    v_src: f64,
) -> Option<(f64, f64, f64)> {
    let r = chain.r_supply(k) + chain.r_return(k);
    let p_in = p_out_w / eta + base_w;
    let d = v_src * v_src - 4.0 * r * p_in;
    if d < 0.0 {
        return None;
    }
    let i = (v_src - d.sqrt()) / (2.0 * r);
    Some((i * 1000.0, v_src - i * r, i * chain.r_return(k)))
}

/// Battery facts and open items.
pub const BATTERY: &[(&str, Quantity, Status)] = &[
    (
        "Nominal capacity",
        Quantity::Known {
            value: 3000.0,
            unit: "mAh",
            source: Source::Client { date: "2026-11-01" },
        },
        Status::ClientConfirmed,
    ),
    (
        "Cell configuration",
        Quantity::Tbd("pack datasheet / MPN (1S assumed for calculations only)"),
        Status::Open,
    ),
    (
        "Nominal voltage",
        Quantity::Tbd("pack datasheet (3.7 V used as a calculation placeholder)"),
        Status::Open,
    ),
    (
        "Maximum charge voltage",
        Quantity::Tbd("pack datasheet; BQ24074 regulates 4.2 V (4.23 V max) - must match"),
        Status::Open,
    ),
    (
        "Discharge cutoff",
        Quantity::Tbd("pack datasheet; firmware cutoff set above pack protection threshold"),
        Status::Open,
    ),
    (
        "Continuous / peak discharge",
        Quantity::Tbd("pack datasheet; system requirement >= 1.3 A peak (calculated)"),
        Status::Open,
    ),
    (
        "Protection (OV/UV/OC/short, NTC)",
        Quantity::Tbd("integrated in pack or on main board; BQ24074 TS input supports an NTC"),
        Status::Open,
    ),
];

/// Operating-mode currents (verified datasheet values unless marked).
pub const MODE_DATA: &[(&str, &str)] = &[
    (
        "ESP32-S3 deep sleep (RTC memory + peripherals)",
        "8 uA typ (Espressif table 5-10)",
    ),
    ("ESP32-S3 light sleep", "240 uA typ"),
    (
        "ESP32-S3 modem sleep 160 MHz, one core active",
        "39.9 / 54.6 mA typ (two conditions in table 5-9)",
    ),
    (
        "ESP32-S3 Wi-Fi TX 802.11b 21 dBm / 802.11n HT20",
        "340 / 283 mA peak; RX 88 mA",
    ),
    ("ESP32-S3 BLE TX 0 dBm / RX", "176 / 93 mA peak"),
    (
        "SHT4x",
        "idle 0.08 uA typ; measuring 320 typ / 500 max uA (VDD 1.08-3.6 V)",
    ),
    (
        "SCD41",
        "peak 205 mA max @3.3 V; single shot 0.45/0.5 mA avg at 5-min interval",
    ),
    (
        "Patch always-on logic (datasheet maxima, all 8)",
        "8 x 0.314 mA = 2.5 mA at 3.3 V (only while the chain is powered)",
    ),
    (
        "Microphone (PUI DMM-4026-B-I2S)",
        "UNVERIFIED - document download failed",
    ),
    ("Display", "UNVERIFIED - module not selected"),
    (
        "TPS63021 / BQ24074 / MAX17048 quiescent",
        "25/50 uA; BAT sleep 4.3/6.5 uA; ~4 uA hibernate (MAX17048 unverified)",
    ),
    (
        "TPS61023 / TPS62840 / TPS2553",
        "IQ 0.9/3 uA (disabled: true disconnect); 60 nA; off 0.1-1 uA",
    ),
];

/// Battery-life scenario inputs. **All non-datasheet values are assumptions.**
#[derive(Debug, Clone, Copy)]
pub struct Scenario {
    /// Name.
    pub name: &'static str,
    /// Usable fraction of 3000 mAh (cutoff, ageing, temperature) - assumption.
    pub usable: f64,
    /// Buck-boost efficiency (assumed until curves are read for the operating point).
    pub eta_buck_boost: f64,
    /// 5 V boost efficiency (assumed).
    pub eta_boost: f64,
    /// Patch buck efficiency (assumed).
    pub eta_patch: f64,
    /// Battery-side sleep current, µA: ESP32-S3 8 + TPS63021 25 typ/50 max + BQ24074 BAT 4.3/6.5
    /// + MAX17048 ~4 hibernate / ~23 active (unverified) + pack protection (assumed 3-10) + chain switch off 1.
    pub sleep_ua: f64,
    /// ESP32 active time per cycle (s) and current (mA, datasheet table values).
    pub cpu_s: f64,
    /// ESP32 active current (mA).
    pub cpu_ma: f64,
    /// ESP32 light-sleeps during CO2 conversions.
    pub light_sleep_waits: bool,
    /// Number of CO2 modules read per cycle.
    pub n_aux: u32,
    /// Charge per SCD41 single-shot (mA·s) - derived from the datasheet 5-min average (upper-bound estimate).
    pub aux_mas: f64,
    /// Wi-Fi session length (s) and average current (mA) - assumptions.
    pub wifi_s: f64,
    /// Wi-Fi average current during the session (mA).
    pub wifi_ma: f64,
    /// Upload interval (minutes) - assumption.
    pub upload_every_min: f64,
    /// Display average current (mA) - assumption (module unselected).
    pub display_ma: f64,
}

/// Sampling interval under evaluation (s) - APR-4 provisional.
pub const SAMPLE_PERIOD_S: f64 = 60.0;
/// Placeholder pack nominal voltage until the pack datasheet is supplied (V).
pub const VNOM_PLACEHOLDER: f64 = 3.7;

/// Scenarios.
pub const SCENARIOS: [Scenario; 3] = [
    Scenario {
        name: "optimistic",
        usable: 0.90,
        eta_buck_boost: 0.92,
        eta_boost: 0.90,
        eta_patch: 0.90,
        sleep_ua: 42.0,
        cpu_s: 0.3,
        cpu_ma: 39.9,
        light_sleep_waits: true,
        n_aux: 1,
        aux_mas: 135.0,
        wifi_s: 1.0,
        wifi_ma: 120.0,
        upload_every_min: 15.0,
        display_ma: 0.0,
    },
    Scenario {
        name: "typical",
        usable: 0.85,
        eta_buck_boost: 0.90,
        eta_boost: 0.88,
        eta_patch: 0.88,
        sleep_ua: 46.0,
        cpu_s: 0.6,
        cpu_ma: 39.9,
        light_sleep_waits: true,
        n_aux: 2,
        aux_mas: 135.0,
        wifi_s: 2.5,
        wifi_ma: 120.0,
        upload_every_min: 5.0,
        display_ma: 0.0,
    },
    Scenario {
        name: "worst",
        usable: 0.70,
        eta_buck_boost: 0.85,
        eta_boost: 0.82,
        eta_patch: 0.80,
        sleep_ua: 100.0,
        cpu_s: 1.5,
        cpu_ma: 54.6,
        light_sleep_waits: false,
        n_aux: 4,
        aux_mas: 150.0,
        wifi_s: 5.0,
        wifi_ma: 150.0,
        upload_every_min: 1.0,
        display_ma: 10.0,
    },
];

impl Scenario {
    /// Average battery current (mA) at the placeholder nominal voltage.
    pub fn average_ma(&self) -> f64 {
        let t = SAMPLE_PERIOD_S;
        let vb = VNOM_PLACEHOLDER;
        let chain_eta = self.eta_patch * self.eta_boost;
        let t_chain = 0.3 + self.n_aux as f64 * 5.05;
        let e_sleep = self.sleep_ua * 1e-3 * vb * t;
        let e_cpu = 3.3 * self.cpu_ma * self.cpu_s / self.eta_buck_boost;
        let wait_ma = if self.light_sleep_waits {
            0.24
        } else {
            self.cpu_ma
        };
        let e_wait = 3.3 * wait_ma * (t_chain - 0.3) / self.eta_buck_boost;
        let e_base = 3.3 * 8.0 * 0.314 * t_chain / chain_eta;
        let e_aux = self.n_aux as f64 * 3.3 * self.aux_mas / chain_eta;
        let e_primary = 8.0 * 3.3 * 0.5 * 0.010 / chain_eta;
        let e_wifi = 3.3 * self.wifi_ma * self.wifi_s / self.eta_buck_boost / self.upload_every_min;
        let e_disp = 3.3 * self.display_ma * t / self.eta_buck_boost;
        (e_sleep + e_cpu + e_wait + e_base + e_aux + e_primary + e_wifi + e_disp) / t / vb
    }
    /// Estimated runtime (h) on the confirmed 3000 mAh capacity.
    pub fn runtime_h(&self) -> f64 {
        3000.0 * self.usable / self.average_ma()
    }
}

/// Battery peak current (mA) for given rail loads at the minimum battery voltage.
pub fn battery_peak_ma(
    wifi_ma_3v3: f64,
    chain_ma_5v: f64,
    eta_bb: f64,
    eta_boost: f64,
    vbat_min: f64,
) -> f64 {
    3.3 * wifi_ma_3v3 / (eta_bb * vbat_min) + 5.0 * chain_ma_5v / (eta_boost * vbat_min)
}

/// Power and thermal constraints / findings.
pub const CONSTRAINTS: &[(&str, Status, Source)] = &[
    ("Chain current is hardware-limited to <= 388 mA (TPS2553 76.8 kohm) - below the 400 mA provisional ceiling; thermal check of FH34 contacts still required (V-16).", Status::Proposal, Source::Analysis),
    ("+5V_CHAIN is switched off between sampling cycles; patches are unpowered in sleep and re-initialised (CHAIN_INIT) on every wake.", Status::Proposal, Source::Analysis),
    ("Back-power rule: before +5V_CHAIN is switched off, all chain GPIOs must be driven low or high-Z; PATCH_OE_N pull-up moves to each patch (+3V3_LOCAL) so no main-board pull-up feeds unpowered patches through input clamp diodes.", Status::Proposal, Source::Analysis),
    ("Firmware rule: no Wi-Fi TX while the chain powers an AUX module (battery peak 0.79 A instead of 1.23 A).", Status::Proposal, Source::Analysis),
    ("Linear charger dissipation (V_IN - V_BAT) x I_CHG: about 1 W at 0.5 A from a 3.0 V cell - thermal design and charge current depend on the pack datasheet and USB source capability.", Status::Proposal, Source::Analysis),
    ("Firmware battery cutoff must be above the pack protection threshold; TPS63802/TPS61023 operate down to 1.3/0.5 V, so they do not protect the cell.", Status::Proposal, Source::Analysis),
    ("Heated gas sensors excluded (APR-3).", Status::Approved, Source::Client { date: "2026-11-01" }),
];
