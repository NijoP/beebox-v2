//! # Power budget, chain voltage drop and thermal analysis (rev C, 3.3 V chain)
//!
//! **Rev D note (APR-1 option B approved 2026-11-01):** the chain-distribution parts of this module
//! (+3V3_CHAIN, chain drop, chain switch 66.5 k) are **superseded** by [`crate::power`] (+5V_CHAIN,
//! TPS61023 + per-patch TPS62840, chain switch 76.8 k). Charger, +3V3 rail and thermal results remain
//! valid; they are kept unchanged for traceability.
//!
//! ## Rail tree (V2 recommendation)
//! ```text
//! USB-C VBUS ─ BQ24074 (OVP 10.5 V, USB100→USB500 default) ─ OUT 4.4 V / VBAT ─ TPS63021 ─ +3V3 (3.267–3.333 V PWM, ≤ +5 % PS)
//!                    │                                                    ├─ ESP32-S3-WROOM-1-N16R2 (≥ 0.5 A)
//!  1S LiPo ──────────┘ (BAT, NTC → TS)                                    ├─ TPS22918 → +3V3_DISPLAY
//!                                                                         ├─ MAX17048 / LEDs / pull-ups
//!                                                                         └─ TPS2553-1 (349–451 mA) → +3V3_CHAIN → FH34 pin 2 → 8 patches
//!   each patch: +3V3_CHAIN ─ logic (595, 4066, 238, 138, TMUX136, TMUX1109) ─ TPS22918 → PRI_VDD
//!                                                                          └─ 4 × TPS2553-1 (248–329 mA) → AUXn_VCC
//! ```
//!
//! ## Key results (computed below and asserted in tests)
//! * Patch idle load ≤ 0.32 mA (rev C HC logic + TMUX136), 8 patches ≤ 2.6 mA.
//! * Worst-case +3V3_CHAIN load (8 idle patches + one AUX port at its 328.6 mA limit + pull-ups +
//!   pull-downs + bleeder + arm sense) = 335 mA → chain switch minimum limit 349 mA. **PASS**
//!   (also below the 350 mA FH34 70 % note).
//! * +3V3 rail design load 0.99 A (ESP32 0.5 A supply requirement + chain limit 0.45 A + display
//!   0.03 A *assumption* + pull-ups/LEDs) → TPS63021 rated 2 A at VIN > 2.5 V. **PASS**.
//! * **Chain drop (worst case: FH34 100 mΩ max per contact, 100 mm FFC *assumed*, source 3.267 V)**:
//!   with 205 mA at patch 8 the patch-8 rail falls to 2.43 V. Allowed AUX current for port VCC ≥ 2.9 V
//!   (rev C criterion): 504 / 300 / 213 / 165 / 135 / 114 / 98 / 87 mA for patches 1…8.
//!   **SCD41-class (205 mA) only on patches 1–3 at worst case** → APR-1 / D-08 and bench V-06.
//!   Cross-checked against rev C [`crate::calculations::Chain`] (< 3 % difference).
//! * Ground offset at patch 8 with 205 mA return = 0.16 V; no I²C transfers during module current
//!   peaks (FW-AUX-3).
//! * BQ24074 worst dissipation 1.68 W → TJ 115 °C at 40 °C ambient (thermal regulation at 125 °C):
//!   **PASS with derating**; charging slows in a hot enclosure.

/// One load line.
#[derive(Debug, Clone, Copy)]
pub struct Load {
    /// Part / function.
    pub item: &'static str,
    /// Rail.
    pub rail: &'static str,
    /// Typical current (mA).
    pub typ_ma: f64,
    /// Maximum current used in worst case (mA).
    pub max_ma: f64,
    /// Condition and datasheet.
    pub basis: &'static str,
}

/// Per-patch always-on logic (chain powered, all paths off) — rev C baseline parts.
pub const PATCH_IDLE: &[Load] = &[
    Load {
        item: "SN74HC595 (LV595A alt: 20 µA)",
        rail: "+3V3_CHAIN",
        typ_ma: 0.008,
        max_ma: 0.080,
        basis: "ICC 8/80 µA @6 V (DS-HC595)",
    },
    Load {
        item: "CD74HC4066 (TMUX1511 alt: 70 µA)",
        rail: "+3V3_CHAIN",
        typ_ma: 0.002,
        max_ma: 0.020,
        basis: "ICC 20 µA -55..85 °C @6 V (DS-HC4066)",
    },
    Load {
        item: "CD74HC238",
        rail: "+3V3_CHAIN",
        typ_ma: 0.008,
        max_ma: 0.080,
        basis: "ICC 8/80 µA @6 V (DS-HC238)",
    },
    Load {
        item: "CD74HC138",
        rail: "+3V3_CHAIN",
        typ_ma: 0.008,
        max_ma: 0.080,
        basis: "ICC 8/80 µA @6 V (DS-HC238)",
    },
    Load {
        item: "TMUX136",
        rail: "+3V3_CHAIN",
        typ_ma: 0.030,
        max_ma: 0.050,
        basis: "ICC 30/50 µA @4.8 V (DS-TMUX136)",
    },
    Load {
        item: "TMUX1109",
        rail: "+3V3_CHAIN",
        typ_ma: 0.000006,
        max_ma: 0.001,
        basis: "IDD 0.006 µA typ; 1 µA bound assumed",
    },
    Load {
        item: "TPS22918 (off)",
        rail: "+3V3_CHAIN",
        typ_ma: 0.0005,
        max_ma: 0.0035,
        basis: "ISD 0.5/3.5 µA @3.3 V (DS-TPS22918)",
    },
    Load {
        item: "TPS2553-1 × 4 (off)",
        rail: "+3V3_CHAIN",
        typ_ma: 0.0004,
        max_ma: 0.004,
        basis: "IIN_off 0.1/1 µA each (DS-TPS255X)",
    },
];

/// Extra chain current for the single active AUX path (worst case). Primary paths are off:
/// the rev C decoders block AUX while Q0/Q1 are set.
pub const ACTIVE_PATH_EXTRA: &[Load] = &[
    Load {
        item: "TPS2553-1 on",
        rail: "+3V3_CHAIN",
        typ_ma: 0.12,
        max_ma: 0.17,
        basis: "IIN_on (DS-TPS255X)",
    },
    Load {
        item: "Port pull-ups 2 × 2.2 k, both low",
        rail: "AUXn_VCC",
        typ_ma: 1.5,
        max_ma: 3.0,
        basis: "3.3 V / 2.2 k × 2 (≤ 3 mA I²C sink)",
    },
    Load {
        item: "595 pull-downs 47 k, up to 5 bits high (Q3-Q7)",
        rail: "+3V3_CHAIN",
        typ_ma: 0.28,
        max_ma: 0.372,
        basis: "3.5 V / 47 k × 5",
    },
    Load {
        item: "Port bleeder 10 k",
        rail: "AUXn_VCC",
        typ_ma: 0.33,
        max_ma: 0.35,
        basis: "3.5 V / 10 k",
    },
    Load {
        item: "AUX-armed sense (Q7 → diode → 150 k → 100 k)",
        rail: "+3V3_CHAIN",
        typ_ma: 0.011,
        max_ma: 0.014,
        basis: "3.5 V / 250 k",
    },
];

/// Sum of the max column.
pub fn sum_max(l: &[Load]) -> f64 {
    l.iter().map(|x| x.max_ma).sum()
}
/// Sum of the typ column.
pub fn sum_typ(l: &[Load]) -> f64 {
    l.iter().map(|x| x.typ_ma).sum()
}

/// Number of patches.
pub const PATCHES: usize = 8;

/// Worst-case +3V3_CHAIN load (mA) when the active AUX port draws `port_ma`.
pub fn chain_worst_load_ma(port_ma: f64) -> f64 {
    PATCHES as f64 * sum_max(PATCH_IDLE) + sum_max(ACTIVE_PATH_EXTRA) + port_ma
}

/// Main +3V3 rail design load (mA).
pub const MAIN_RAIL: &[Load] = &[
    Load {
        item: "ESP32-S3-WROOM-1 supply capability",
        rail: "+3V3",
        typ_ma: 70.0,
        max_ma: 500.0,
        basis: "IVDD >= 0.5 A required; TX 802.11b 355 mA (DS-ESP-MOD)",
    },
    Load {
        item: "+3V3_CHAIN limit (max)",
        rail: "+3V3",
        typ_ma: 3.0,
        max_ma: 450.9,
        basis: "TPS2553-1 window upper bound, 66.5 k",
    },
    Load {
        item: "Display (assumption A-L1, MPN open OQ-017)",
        rail: "+3V3_DISPLAY",
        typ_ma: 10.0,
        max_ma: 30.0,
        basis: "assumption",
    },
    Load {
        item: "LEDs 3 × ~1.3 mA",
        rail: "+3V3",
        typ_ma: 0.0,
        max_ma: 4.0,
        basis: "1 k series, Vf to verify",
    },
    Load {
        item: "Display-bus pull-ups 2 × 2.2 k",
        rail: "+3V3",
        typ_ma: 0.0,
        max_ma: 3.0,
        basis: "both low",
    },
    Load {
        item: "Sensor-bus pull-ups 2 × 2.2 k from SENSOR_PU_EN (primary I²C mode)",
        rail: "GPIO6",
        typ_ma: 0.0,
        max_ma: 3.0,
        basis: "PRP-MB-02",
    },
    Load {
        item: "MAX17048 (unverified)",
        rail: "+3V3",
        typ_ma: 0.023,
        max_ma: 0.05,
        basis: "V1 evidence; re-verify DS-MAX17048",
    },
];

/// TPS63021 rated output current at VIN > 2.5 V (A).
pub const TPS63021_IOUT_RATED_A: f64 = 2.0;

/// Battery-side current (A) for the +3V3 load `iout_a` at `vbat`, efficiency `eta`, including the
/// BQ24074 BAT→OUT drop (≤ 100 mV at 1 A).
pub fn battery_current_a(iout_a: f64, vbat: f64, eta: f64) -> f64 {
    3.333 * iout_a / (eta * (vbat - 0.1))
}

// ------------------------------------------------------------------ chain model

/// Resistive model of the 12-pin chain with distributed patch base currents.
#[derive(Debug, Clone, Copy)]
pub struct ChainModel {
    /// FH34 contact resistance (Ω) per mated contact (spec max 0.100 incl. FPC bulk).
    pub contact_ohm: f64,
    /// FFC conductor resistance per segment (Ω).
    pub ffc_ohm: f64,
    /// Patch IN→OUT pass-through trace (Ω).
    pub pcb_ohm: f64,
    /// Parallel ground contacts (pins 1, 3, 6, 12).
    pub ground_contacts: f64,
    /// Main chain switch on-resistance (Ω).
    pub switch_ohm: f64,
    /// Source voltage (V).
    pub v_source: f64,
    /// Always-on current per patch (mA).
    pub base_ma: f64,
}

/// Copper resistivity at 20 °C (Ω·m).
pub const RHO_CU: f64 = 1.72e-8;

/// Conductor resistance (Ω) for `len_m`, `w_mm`, `t_mm`.
pub fn conductor_ohm(len_m: f64, w_mm: f64, t_mm: f64) -> f64 {
    RHO_CU * len_m / (w_mm * 1e-3 * t_mm * 1e-3)
}

/// Port switch on-resistance used for the module voltage (TPS2553 DBV, 135 mΩ max).
pub const PORT_SWITCH_OHM: f64 = 0.135;

/// Worst-case model: FH34 100 mΩ max; FFC 100 mm of 0.30 × 0.035 mm (*assumption A-C1*, FFC MPN
/// open OQ-024); patch trace 30 mm × 0.5 mm × 35 µm; TPS2553 135 mΩ max (DBV); TPS63021 PWM min.
pub fn worst_case_model() -> ChainModel {
    ChainModel {
        contact_ohm: 0.100,
        ffc_ohm: conductor_ohm(0.100, 0.30, 0.035),
        pcb_ohm: conductor_ohm(0.030, 0.5, 0.035),
        ground_contacts: 4.0,
        switch_ohm: 0.135,
        v_source: 3.267,
        base_ma: sum_max(PATCH_IDLE),
    }
}

impl ChainModel {
    /// Rail voltage at each patch input (index 0 = patch 1) with `load_ma` at `load_pos` (1-based).
    pub fn rail_profile(&self, load_ma: f64, load_pos: usize) -> [f64; PATCHES] {
        let mut i = [self.base_ma; PATCHES];
        if (1..=PATCHES).contains(&load_pos) {
            i[load_pos - 1] += load_ma;
        }
        let total: f64 = i.iter().sum::<f64>() / 1000.0;
        let mut v = self.v_source - total * self.switch_ohm;
        let seg = 2.0 * self.contact_ohm + self.ffc_ohm;
        let mut out = [0.0; PATCHES];
        for k in 0..PATCHES {
            let down: f64 = i[k..].iter().sum::<f64>() / 1000.0;
            v -= down * (seg + seg / self.ground_contacts);
            out[k] = v;
            if k + 1 < PATCHES {
                let down2: f64 = i[k + 1..].iter().sum::<f64>() / 1000.0;
                v -= down2 * (self.pcb_ohm + self.pcb_ohm / self.ground_contacts);
            }
        }
        out
    }

    /// Port VCC at `pos` (V) including the port switch drop.
    pub fn module_voltage(&self, load_ma: f64, pos: usize) -> f64 {
        self.rail_profile(load_ma, pos)[pos - 1] - load_ma / 1000.0 * PORT_SWITCH_OHM
    }

    /// Largest AUX current (mA) at `pos` keeping the port VCC ≥ `v_min`.
    pub fn max_aux_ma(&self, pos: usize, v_min: f64) -> f64 {
        let (mut lo, mut hi) = (0.0, 2000.0);
        for _ in 0..60 {
            let m = (lo + hi) / 2.0;
            if self.module_voltage(m, pos) >= v_min {
                lo = m
            } else {
                hi = m
            }
        }
        lo
    }

    /// Ground offset (V) between main board and patch `pos` with `load_ma` returning from it.
    pub fn ground_offset(&self, load_ma: f64, pos: usize) -> f64 {
        let seg = 2.0 * self.contact_ohm + self.ffc_ohm;
        let r = pos as f64 * seg / self.ground_contacts
            + (pos as f64 - 1.0) * self.pcb_ohm / self.ground_contacts;
        load_ma / 1000.0 * r
    }
}

/// Design-target minimum port VCC (V): rev C criterion 2.9 V (`aux_expansion::PULL_UP_RULES`).
pub const AUX_VMIN_TARGET: f64 = 2.9;
/// ESP32-S3 VIH min at VDD max (V): 0.75 × 3.333 (DS-ESP-MOD).
pub const ESP_VIH_MIN_AT_VDD_MAX: f64 = 0.75 * 3.333;

// ------------------------------------------------------------------ battery runtime

/// Duty-cycle scenario (all *assumptions* until APR-4 defines sampling/upload; A-B1).
#[derive(Debug, Clone, Copy)]
pub struct Scenario {
    /// Scan period (s).
    pub period_s: f64,
    /// CPU awake time per period (s).
    pub awake_s: f64,
    /// Awake current (mA): ESP32-S3 240 MHz dual-core ~66 mA (DS-ESP-SOC Table 5-9) rounded up.
    pub awake_ma: f64,
    /// Chain + sensors average while awake (mA).
    pub chain_ma: f64,
    /// Radio TX time per period (s) at 355 mA (DS-ESP-MOD).
    pub tx_s: f64,
    /// Sleep current on +3V3 (mA): deep sleep 8 µA + switches off + gauge hibernate (A).
    pub sleep_ma: f64,
    /// Battery-side quiescent (mA): TPS63021 IQ 50 µA max + BQ24074 6.5 µA + divider 3.2 µA.
    pub batt_q_ma: f64,
    /// Converter efficiency (assumption 0.85).
    pub eta: f64,
}

/// Example scenario: 10 min period; 15 s awake (8 primary reads, 2 AUX single shots, audio);
/// 2 s Wi-Fi TX.
pub const EXAMPLE_SCENARIO: Scenario = Scenario {
    period_s: 600.0,
    awake_s: 15.0,
    awake_ma: 70.0,
    chain_ma: 25.0,
    tx_s: 2.0,
    sleep_ma: 0.03,
    batt_q_ma: 0.060,
    eta: 0.85,
};

/// Average battery current (mA) at 3.7 V nominal.
pub fn average_battery_ma(s: &Scenario) -> f64 {
    let mas = s.awake_s * (s.awake_ma + s.chain_ma)
        + s.tx_s * 355.0
        + (s.period_s - s.awake_s - s.tx_s) * s.sleep_ma;
    let i3v3_avg = mas / s.period_s;
    i3v3_avg * 3.3 / (s.eta * 3.7) + s.batt_q_ma
}

/// Runtime (h) for `cap_mah` with `usable` fraction (assumption 0.8: ageing, cutoff, cold).
pub fn runtime_h(cap_mah: f64, usable: f64, avg_ma: f64) -> f64 {
    cap_mah * usable / avg_ma
}

// ------------------------------------------------------------------ thermal

/// Junction temperature (°C).
pub fn tj(ta: f64, p_w: f64, rth: f64) -> f64 {
    ta + p_w * rth
}

/// Thermal checks: (part, P (W), RθJA (°C/W), datasheet).
pub const THERMAL: &[(&str, f64, f64, &str)] = &[
    (
        "BQ24074 worst charge (5.25 V in, 1.07 A, 3.0 V batt, 0.55 A)",
        1.68,
        44.5,
        "DS-BQ24074",
    ),
    (
        "TPS63021 1 A out, η 0.85 assumed",
        0.59,
        41.8,
        "DS-TPS6302X",
    ),
    (
        "TPS2553-1 chain 0.45 A × 135 mΩ",
        0.027,
        182.6,
        "DS-TPS255X",
    ),
    ("TPS2553-1 port 0.33 A × 135 mΩ", 0.015, 182.6, "DS-TPS255X"),
];
/// Design ambient for thermal checks (assumption A-T1: 40 °C hive enclosure).
pub const T_AMBIENT: f64 = 40.0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patch_idle_and_chain_worst_case() {
        let idle = sum_max(PATCH_IDLE);
        assert!(idle > 0.30 && idle < 0.33, "{idle}");
        let (_, pmax) = crate::passives::tps255x_window_ma(crate::passives::R_ILIM_PORT_K);
        let w = chain_worst_load_ma(pmax);
        assert!(w > 330.0 && w < 340.0, "{w}");
        assert!(w < 350.0); // FH34 0.5 A × 70 % note-1 derating
    }

    #[test]
    fn main_rail_within_regulator_and_battery() {
        let i = sum_max(MAIN_RAIL) / 1000.0;
        assert!(i < 1.0 && i < TPS63021_IOUT_RATED_A, "{i}");
        let ib = battery_current_a(i, 3.0, 0.85);
        assert!(ib < 1.5, "battery {ib} A"); // cell spec >= 1.5 A continuous; JST PH 2 A
        let ipk = crate::passives::tps63021_ipeak_a(i, 2.9, 3.3, 0.9, 2.2e6, 1.05e-6);
        assert!(ipk < 2.5);
    }

    #[test]
    fn chain_drop_worst_case_documents_far_patch_limit() {
        let m = worst_case_model();
        assert!((m.ffc_ohm - 0.1638).abs() < 0.001);
        let p8 = m.rail_profile(205.0, 8)[7];
        assert!(p8 < 2.46 && p8 > 2.40, "{p8}");
        let lim: Vec<f64> = (1..=8).map(|p| m.max_aux_ma(p, AUX_VMIN_TARGET)).collect();
        assert!(
            lim[2] > 205.0 && lim[3] < 205.0,
            "SCD41-class only on patches 1-3: {lim:?}"
        );
        assert!(lim[7] > 80.0 && lim[7] < 95.0, "{lim:?}");
        for w in lim.windows(2) {
            assert!(w[0] > w[1]);
        }
        let idle = m.rail_profile(0.0, 8);
        assert!(
            m.v_source - idle[7] < 0.010,
            "idle drop used by the ID check"
        );
        let g = m.ground_offset(205.0, 8);
        assert!(g > 0.15 && g < 0.17, "{g}");
        // Cross-check with rev C calculations::Chain (same inputs, no distributed base): < 3 %.
        let rc = crate::calculations::Chain::baseline(0.10);
        let rev_c_r = rc.r_supply(8) + rc.r_return(8);
        let mine = ChainModel { base_ma: 0.0, ..m };
        let r_mine = (m.v_source - mine.module_voltage(100.0, 8)) / 0.1;
        assert!(
            (r_mine - rev_c_r).abs() / rev_c_r < 0.03,
            "{r_mine} vs {rev_c_r}"
        );
        assert!(m.module_voltage(20.0, 8) > ESP_VIH_MIN_AT_VDD_MAX + 0.5);
    }

    #[test]
    fn example_runtime() {
        let a = average_battery_ma(&EXAMPLE_SCENARIO);
        assert!(a > 3.0 && a < 5.0, "{a}");
        let h = runtime_h(2000.0, 0.8, a);
        assert!(h > 300.0, "{h}");
    }

    #[test]
    fn thermal_margins() {
        for (n, p, r, _) in THERMAL {
            let t = tj(T_AMBIENT, *p, *r);
            assert!(t < 125.0, "{n}: {t}");
        }
    }
}
