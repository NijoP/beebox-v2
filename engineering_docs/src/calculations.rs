//! # Engineering calculations (revision C, checked by unit tests)
//!
//! **Verified inputs** come from manufacturer documents (see [`crate::research`]).
//! **Assumed inputs** are named `*_ASSUMED`. They must be replaced by the selected cable and
//! module datasheets or by measurement. All results are worst-case unless stated.
//!
//! ## Key results
//! | Result | Value | Basis |
//! |---|---|---|
//! | Per-port current limit (TPS2553, R_ILIM 93.1 kΩ) | 252 / 286 / 324 mA min/nom/max | TI equations |
//! | Drop to patch 8 at 229 mA, **contacts + switches only** (no cable) | 0.52 V → port 2.78 V | Hirose 100 mΩ max, TPS2553 135 mΩ max |
//! | Max module current at patch 8 for port ≥ 2.9 V, 0.10 m segments | ≈ 95 mA | + assumed cable |
//! | Same at patch 1 / patch 4 | ≈ 525 / 179 mA | |
//! | Bus capacitance, 0.10 m segments, with TMUX136 isolator | ≈ 255 pF | 400 pF I²C limit |
//! | Same, TMUX1109 directly on the chain (no isolator) | ≈ 463 pF → **reject** | 32 pF drain-off per patch |
//! | Same with 0.25 m / 0.50 m segments | ≈ 375 / 575 pF | 0.50 m exceeds 400 pF |
//! | Pull-up window, 100 kHz, 255 pF | 967 Ω … 4.6 kΩ | UM10204 3 mA, t_r ≤ 1000 ns |
//! | Mux R_ON effect (TMUX136 13 Ω + TMUX1109 9.75 Ω max, 1.83 mA) | ≈ 42 mV | vs ESP32-S3 VIL 0.825 V |
//! | Same with CD74HC4052-class 180 Ω | ≈ 330 mV → **reject** | |
//!
//! **Conclusion:** SCD41-class modules (205 mA peak) are **not** guaranteed by worst-case
//! datasheet values beyond the first few patches on a 3.3 V chain supply. Low-current modules
//! (e.g. STCC4, 4.2 mA peak) are supported at every position. See approval item APR-1.

/// Hirose FH34SRJ contact resistance, maximum (Ω) - Hirose spec sheet.
pub const FH34_CONTACT_MAX_OHM: f64 = 0.100;
/// Hirose FH34SRJ rated current per contact (mA).
pub const FH34_RATED_MA: f64 = 500.0;
/// Hirose FH34SRJ mating durability (cycles) - spec sheet.
pub const FH34_MATING_CYCLES: u32 = 20;
/// TPS2553 r_DS(on) max over -40..125 °C (Ω).
pub const TPS2553_RDS_MAX_OHM: f64 = 0.135;
/// TMUX1109 R_ON max at 3.3 V, -40..125 °C (Ω).
pub const TMUX1109_RON_MAX_OHM: f64 = 9.75;
/// TMUX1109 on-capacitance typ (pF) and drain-off capacitance typ (pF).
pub const TMUX1109_CON_PF: f64 = 38.0;
/// TMUX1109 drain-off capacitance typ (pF).
pub const TMUX1109_CDOFF_PF: f64 = 32.0;
/// TMUX136 R_ON max, -40..125 °C (Ω).
pub const TMUX136_RON_MAX_OHM: f64 = 13.0;
/// TMUX136 on/off capacitance max (pF).
pub const TMUX136_C_MAX_PF: f64 = 2.0;
/// CD74HC4066 switch input capacitance (pF).
pub const CD4066_CI_PF: f64 = 5.0;
/// ESP32-S3 pin capacitance typ (pF), VIL max factor, VIH min factor.
pub const ESP32S3_CIN_PF: f64 = 2.0;
/// ESP32-S3 VIL max = 0.25 × VDD.
pub const ESP32S3_VIL_FACTOR: f64 = 0.25;
/// ESP32-S3 VIH min = 0.75 × VDD.
pub const ESP32S3_VIH_FACTOR: f64 = 0.75;
/// SCD41 peak current max at 3.3 V (mA).
pub const SCD41_PEAK_MAX_MA: f64 = 205.0;
/// STCC4 peak current max (mA).
pub const STCC4_PEAK_MAX_MA: f64 = 4.2;

/// ASSUMED: breakout overhead (LED, regulator, pull-ups) per module (mA).
pub const BREAKOUT_OVERHEAD_MA_ASSUMED: f64 = 20.0;
/// ASSUMED: FFC conductor resistance (Ω/m), derived from a 0.30 × 0.035 mm copper conductor.
/// Replace with the selected cable datasheet (Würth/Molex documents could not be downloaded).
pub const CABLE_OHM_PER_M_ASSUMED: f64 = 1.64;
/// ASSUMED: FFC line capacitance per conductor (pF/m).
pub const CABLE_PF_PER_M_ASSUMED: f64 = 100.0;
/// ASSUMED: module side (ESD, header, module cable, breakout) capacitance (pF).
pub const MODULE_SIDE_PF_ASSUMED: f64 = 50.0;
/// ASSUMED: per-patch pass-through trace + connector capacitance (pF).
pub const PATCH_TRACE_PF_ASSUMED: f64 = 3.0;
/// ASSUMED: main-board trace capacitance (pF).
pub const MAIN_TRACE_PF_ASSUMED: f64 = 5.0;

/// Worst-case always-on patch electronics current (µA), datasheet maxima at their test
/// conditions: 74HC595 80 + 74HC238 80 + 74HC138 80 + CD74HC4066 20 + TMUX136 50 + 4×TPS2553 off 1
/// + TMUX1109 ~0 (typ 0.006). Primary load-switch off-current not yet included (TBD).
pub const PATCH_BASE_UA_MAX: f64 = 80.0 + 80.0 + 80.0 + 20.0 + 50.0 + 4.0;

/// TPS2553 current-limit threshold (min, nom, max) in mA for `r_ilim_kohm` (TI equations).
pub fn tps2553_ilim_ma(r_ilim_kohm: f64) -> (f64, f64, f64) {
    (
        25230.0 / r_ilim_kohm.powf(1.016),
        23950.0 / r_ilim_kohm.powf(0.977),
        22980.0 / r_ilim_kohm.powf(0.94),
    )
}

/// Copper trace resistance (Ω), resistivity 1.72e-8 Ω·m.
pub fn trace_ohm(length_mm: f64, width_mm: f64, copper_um: f64) -> f64 {
    1.72e-8 * (length_mm * 1e-3) / ((width_mm * 1e-3) * (copper_um * 1e-6))
}

/// Chain parameters for drop calculations.
#[derive(Debug, Clone, Copy)]
pub struct Chain {
    /// Contact resistance per contact (Ω).
    pub contact_ohm: f64,
    /// Cable conductor resistance (Ω/m).
    pub cable_ohm_per_m: f64,
    /// Segment length (m).
    pub segment_m: f64,
    /// Pass-through supply trace per patch (Ω).
    pub trace_ohm: f64,
    /// Series switch resistance on main board + port (Ω, total).
    pub switches_ohm: f64,
    /// Parallel supply contacts/conductors (1 in the baseline pinout).
    pub supply_contacts: f64,
    /// Parallel ground contacts/conductors (4 in the baseline pinout).
    pub ground_contacts: f64,
}

impl Chain {
    /// Baseline: FH34 max contact R, assumed cable, 30 mm × 0.5 mm 1 oz trace, two TPS2553-class switches.
    pub fn baseline(segment_m: f64) -> Self {
        Chain {
            contact_ohm: FH34_CONTACT_MAX_OHM,
            cable_ohm_per_m: CABLE_OHM_PER_M_ASSUMED,
            segment_m,
            trace_ohm: trace_ohm(30.0, 0.5, 35.0),
            switches_ohm: 2.0 * TPS2553_RDS_MAX_OHM,
            supply_contacts: 1.0,
            ground_contacts: 4.0,
        }
    }
    /// Supply-path resistance to a port on patch `k` (1..=8). Contacts = 2k (main + k IN + k-1 OUT).
    pub fn r_supply(&self, k: u32) -> f64 {
        let k = k as f64;
        (2.0 * k * self.contact_ohm + k * self.cable_ohm_per_m * self.segment_m)
            / self.supply_contacts
            + k * self.trace_ohm
            + self.switches_ohm
    }
    /// Return-path resistance from patch `k` (equal current sharing assumed).
    pub fn r_return(&self, k: u32) -> f64 {
        let k = k as f64;
        (2.0 * k * self.contact_ohm
            + k * self.cable_ohm_per_m * self.segment_m
            + k * self.trace_ohm)
            / self.ground_contacts
    }
    /// Port VCC (relative to the patch ground) at load `i_ma` on patch `k`, from `v_src`.
    pub fn port_voltage(&self, k: u32, i_ma: f64, v_src: f64) -> f64 {
        v_src - i_ma / 1000.0 * (self.r_supply(k) + self.r_return(k))
    }
    /// Maximum load (mA) on patch `k` keeping port VCC >= `v_min`.
    pub fn max_current_ma(&self, k: u32, v_src: f64, v_min: f64) -> f64 {
        (v_src - v_min) / (self.r_supply(k) + self.r_return(k)) * 1000.0
    }
}

/// Worst-case I²C bus capacitance (pF) with the TMUX136 isolator (`isolated = true`) or with the
/// TMUX1109 drain directly on the chain (`false`).
pub fn bus_capacitance_pf(segment_m: f64, isolated: bool) -> f64 {
    let per_patch_unselected = CD4066_CI_PF
        + PATCH_TRACE_PF_ASSUMED
        + if isolated {
            TMUX136_C_MAX_PF
        } else {
            TMUX1109_CDOFF_PF
        };
    let selected_extra = if isolated {
        TMUX1109_CON_PF
    } else {
        TMUX1109_CON_PF - TMUX1109_CDOFF_PF
    };
    ESP32S3_CIN_PF
        + MAIN_TRACE_PF_ASSUMED
        + 8.0 * segment_m * CABLE_PF_PER_M_ASSUMED
        + 8.0 * per_patch_unselected
        + selected_extra
        + MODULE_SIDE_PF_ASSUMED
}

/// Maximum pull-up (Ω) for rise time `t_r_ns` (0.3→0.7 VDD: t_r = 0.8473 R C).
pub fn r_pullup_max_ohm(t_r_ns: f64, c_pf: f64) -> f64 {
    t_r_ns * 1e-9 / (0.8473 * c_pf * 1e-12)
}

/// Minimum pull-up (Ω): (VDD - 0.4 V) / 3 mA.
pub fn r_pullup_min_ohm(vdd: f64) -> f64 {
    (vdd - 0.4) / 0.003
}

/// Low level seen at the ESP32-S3: module VOL + sink current × (switch + conductor R) + ground offset.
pub fn vol_at_esp32_v(
    module_vol: f64,
    i_sink_ma: f64,
    series_ohm: f64,
    ground_offset_v: f64,
) -> f64 {
    module_vol + i_sink_ma / 1000.0 * series_ohm + ground_offset_v
}

/// AUX-armed sense voltage (V) for `n` armed patches, source `v_drive` (V_Q7 - V_F).
pub fn arm_sense_v(n: u32, v_drive: f64) -> f64 {
    if n == 0 {
        return 0.0;
    }
    v_drive * 100.0 / (150.0 / n as f64 + 100.0)
}

/// Full-scan minimum time (s).
pub fn scan_time_s(ports: u32, per_port_ms: f64) -> f64 {
    ports as f64 * per_port_ms / 1000.0
}
