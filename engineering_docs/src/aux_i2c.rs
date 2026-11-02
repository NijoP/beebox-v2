//! # AUX I²C expansion — component design (implements rev C, REC-02)
//!
//! The AUX **architecture** is the established rev C design in [`crate::aux_expansion`]
//! (TMUX136 isolator + TMUX1109 dual 4:1 + CD74HC238 power decoder + CD74HC138 bus decoder +
//! 4 × TPS2553 + Q7 AUX-armed sense). This module fixes exact MPNs and passive values, checks every
//! electrical limit against the datasheets, and records protection and connector choices. It does
//! not change the topology, the Q-map, the 12-pin backbone or the client pinout
//! **1 VCC, 2 GND, 3 SDA, 4 SCL**.
//!
//! ```text
//!  SENSOR_CLK (pin 4) ─ TMUX136 COM1─A1 ─┐                      ┌ S1A..S4A ─ AUXn_SCL ─ 2.2 k → AUXn_VCC ─ J_AUXn.4
//!  SENSOR_SYNC (pin 5) ─ TMUX136 COM2─A2 ┴ TMUX1109 DA / DB ───┴ S1B..S4B ─ AUXn_SDA ─ 2.2 k → AUXn_VCC ─ J_AUXn.3
//!     TMUX136 EN ← CD74HC138 /Y6, SEL = GND          TMUX1109 EN ← Q5, A1:A0 ← Q4:Q3
//!  CD74HC238 Y4..Y7 → TPS2553DBVR-1 EN (RILIM 93.1 k) : +3V3_LOCAL (TPS62840) → AUXn_VCC (10 µF + 100 nF + 10 k bleeder) → J_AUXn.1
//!  Q7 → diode → 150 k → SENSOR_DATA_ID (arm sense)     TPD4E05U06 on AUXn VCC/SDA/SCL at each header
//! ```
//!
//! | Topic | Component-level result | Evidence |
//! |---|---|---|
//! | Four ports / patch | JST B4B-PH-K-S ×4 (APR-2 option B: keyed) | DS-JST-PH: 2 A, 100 V, -40..105 °C |
//! | I²C multiplexing | TMUX1109PWR behind TMUX136RSER | RON 9.75 + ~9.5 Ω; chain sees 2 pF/patch |
//! | Address conflicts | No I²C-addressed infrastructure; one connected port system-wide | rev C + arm sense |
//! | Port selection | One A1:A0 for power and bus; exhaustive truth table | [`crate::aux_expansion::patch_outputs`] |
//! | Power switching | TPS2553DBVR-1, 248–329 mA, latch-off, reverse blocking | [`crate::passives`] |
//! | Voltage compatibility | Port VCC = regulated +3V3_LOCAL (rev D) at every position; pull-ups to port VCC; no 5 V on any I²C line | [`crate::power_budget`] |
//! | Protection | TPD4E05U06 (0.5 pF, VRWM 5.5 V, IEC ±12 kV) per port; current limit; keyed connector | DS-TPD4E05U06 |
//! | Back-power | Bus connect requires Q6 & Q7; unpowered ports isolated (TMUX136 power-off isolation, TMUX1109 off) | truth table test |

/// Electrical checks of the AUX signal path.
#[derive(Debug, Clone, Copy)]
pub struct SignalPath {
    /// Isolator RON max (Ω) — TMUX136 (rev C budget 13 Ω; datasheet 9–9.5 Ω at 2.3–2.7 V).
    pub iso_ron: f64,
    /// Mux RON max (Ω) — TMUX1109 at 3.3 V, 125 °C.
    pub mux_ron: f64,
    /// Chain conductor resistance to patch 8 (Ω) — 16 contacts + 0.8 m FFC.
    pub chain_ohm: f64,
    /// Sink current (A).
    pub i_sink: f64,
}

/// Worst-case AUX signal path.
pub const PATH_WORST: SignalPath = SignalPath {
    iso_ron: 13.0,
    mux_ron: 9.75,
    chain_ohm: 16.0 * 0.1 + 0.8 * 1.64,
    i_sink: 3.0e-3,
};

impl SignalPath {
    /// Extra low level (V) added between the module pin and the ESP32-S3.
    pub fn added_vol(&self) -> f64 {
        self.i_sink * (self.iso_ron + self.mux_ron + self.chain_ohm)
    }
}

/// Module VOL at 3 mA used in the margin check: SCD4x 0.66 V max (DS-SCD4X), the worst
/// verified module.
pub const MODULE_VOL_WORST: f64 = 0.66;
/// ESP32-S3 VIL max at VDD min (V): 0.25 × 3.267.
pub const ESP_VIL_MAX: f64 = 0.25 * 3.267;

/// Per-port component set (quantities per patch).
pub const PER_PATCH_PARTS: &[(&str, u8, &str)] = &[
    (
        "TMUX136RSER",
        1,
        "bus isolator, SEL = GND, EN = CD74HC138 /Y6",
    ),
    (
        "TMUX1109PWR",
        1,
        "dual 4:1 port mux, VSS = GND, EN = Q5, A1:A0 = Q4:Q3",
    ),
    (
        "CD74HC238PWR",
        1,
        "power decoder G2 = Q6, G1 = Q1, G0 = Q0, A2 = Q7, A1:A0 = Q4:Q3, Y4..Y7 → port EN",
    ),
    (
        "CD74HC138M96",
        1,
        "bus decoder G2 = Q5, G1 = Q1, G0 = Q0, A2 = Q7, A1 = Q6, A0 = GND, /Y6 → TMUX136 EN",
    ),
    ("TPS2553DBVR-1", 4, "port VCC switch, RILIM 93.1 k 1 %"),
    ("93.1 kΩ 1 %", 4, "RILIM"),
    (
        "2.2 kΩ 1 %",
        8,
        "SDA/SCL pull-ups to AUXn_VCC, fit/DNI per module profile",
    ),
    ("10 kΩ", 4, "AUXn_VCC bleeder (τ = 0.1 s with 10 µF)"),
    ("10 µF X5R 10 V 0805 + 100 nF", 4, "port VCC reservoir / HF"),
    ("1 µF X7R 0603", 4, "TPS2553 IN"),
    ("100 kΩ", 4, "FAULT pull-ups to test pads"),
    ("TPD4E05U06DQAR", 4, "ESD: D1+ SDA, D1- SCL, D2+ VCC"),
    ("B4B-PH-K-S(LF)(SN)", 4, "J_AUX1..4"),
    (
        "Low-leakage diode + 150 kΩ 1 %",
        1,
        "AUX-armed sense (Q7 → pin 7)",
    ),
    (
        "47 kΩ",
        5,
        "pull-downs Q3..Q7 (Q0..Q2 counted on the patch base)",
    ),
];

/// AUX module power classes per patch position (worst-case contact resistance; see power_budget).
#[derive(Debug, Clone, Copy)]
pub struct PowerClass {
    /// Class name.
    pub class: &'static str,
    /// Current (mA) that every allowed position must support.
    pub max_ma: f64,
    /// Positions allowed at worst case.
    pub positions: &'static str,
    /// Example.
    pub example: &'static str,
}

/// AUX module power classes (proposal under APR-1 option A; update after V-06).
pub const POWER_CLASSES: &[PowerClass] = &[
    PowerClass {
        class: "A-low",
        max_ma: 80.0,
        positions: "patches 1-8",
        example: "STCC4 (4.2 mA peak), SHT4x, BME/BMP-class, SGP4x",
    },
    PowerClass {
        class: "B-peak",
        max_ma: 205.0,
        positions: "patches 1-3",
        example: "SCD41 (175/205 mA peak at 3.3 V)",
    },
];

/// Firmware rules required by the component design (supplement rev C CONNECT/DISCONNECT sequences).
pub const FIRMWARE_RULES: &[(&str, &str)] = &[
    ("FW-AUX-1", "Change Q4:Q3 only with Q5 = Q6 = 0 (rev C ARM step)."),
    ("FW-AUX-2", "After UNPOWER wait ≥ 5 τ of the bleeder (0.5 s) before re-powering the same port."),
    ("FW-AUX-3", "No I²C transfer while a module is in a high-current phase (SCD41 measurement): ground offset reaches 0.16 V at patch 8."),
    ("FW-AUX-4", "100 kHz standard mode only (≈ 257 pF worst-case bus)."),
    ("FW-AUX-5", "On NACK/timeout: ISOLATE, UNPOWER (also resets a latched TPS2553-1), log FAULT(patch, port)."),
    ("FW-AUX-6", "SENSOR_PU_EN low in AUX mode (pull-ups are on the port rail)."),
];

/// Pull-up population rule per module profile.
pub const PULLUP_RULE: &str = "Fit the 2.2 k port pull-ups when the module has none or ≥ 4.7 k on-board pull-ups; DNI them when the module has ≤ 2.2 k (total sink must stay ≤ 3 mA, UM10204).";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aux_expansion::patch_outputs;

    #[test]
    fn low_level_margin_through_isolator_mux_and_chain() {
        let v = MODULE_VOL_WORST + PATH_WORST.added_vol();
        assert!(v < ESP_VIL_MAX, "VOL at ESP32 {v} V vs VIL {ESP_VIL_MAX}");
        // Same function as rev C calculations::vol_at_esp32_v (no ground offset during transfers, FW-AUX-3).
        let rc = crate::calculations::vol_at_esp32_v(
            MODULE_VOL_WORST,
            3.0,
            PATH_WORST.iso_ron + PATH_WORST.mux_ron + PATH_WORST.chain_ohm,
            0.0,
        );
        assert!((rc - v).abs() < 1e-9);
        // With the 0.16 V ground offset during an SCD41 peak the margin would be lost:
        assert!(v + 0.16 > ESP_VIL_MAX);
    }

    #[test]
    fn exhaustive_component_level_safety() {
        for q in 0u16..=255 {
            for oe_n in [false, true] {
                let o = patch_outputs(q as u8, oe_n);
                if o.aux_bus_connected {
                    // The isolator only closes when the selected port is powered and armed.
                    assert_eq!(o.aux_port_powered, o.mux_port, "q={q:08b}");
                    assert!(o.aux_armed && !o.primary_power && !o.primary_signal);
                }
                if oe_n {
                    assert!(o.aux_port_powered.is_none() && !o.aux_bus_connected);
                }
            }
        }
    }

    #[test]
    fn power_classes_consistent_with_chain_model_and_limit() {
        let m = crate::power_budget::worst_case_model();
        let v = crate::power_budget::AUX_VMIN_TARGET;
        let a = POWER_CLASSES.iter().find(|c| c.class == "A-low").unwrap();
        assert!(m.max_aux_ma(8, v) >= a.max_ma);
        let b = POWER_CLASSES.iter().find(|c| c.class == "B-peak").unwrap();
        assert!(m.max_aux_ma(3, v) >= b.max_ma && m.max_aux_ma(4, v) < b.max_ma);
        let (pmin, _) = crate::passives::tps255x_window_ma(crate::passives::R_ILIM_PORT_K);
        assert!(pmin >= b.max_ma);
    }

    #[test]
    fn arm_sense_windows_with_selected_parts() {
        // V_Q7 = patch rail (≥ 3.26 V at idle) minus 595 output drop (~0) minus diode VF (0.40-0.65 V req.).
        let lo = crate::calculations::arm_sense_v(1, 3.257 - 0.65);
        let hi = crate::calculations::arm_sense_v(1, 3.333 - 0.40);
        let two = crate::calculations::arm_sense_v(2, 3.257 - 0.65);
        // rev C windows: 0.85-1.25 V one armed, > 1.30 V two or more; ESP32 ATTEN2 ±10 mV.
        assert!(
            lo - 0.01 > 0.85 && hi + 0.01 < 1.25 && two - 0.01 > 1.30,
            "{lo} {hi} {two}"
        );
    }
}
