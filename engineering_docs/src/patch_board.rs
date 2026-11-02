//! # Sensor-patch component architecture (sheets P01–P05)
//!
//! The 12-pin backbone, the pin-8 cascade, the 64-bit image, the ID model and the rev C Q-map
//! ([`crate::aux_expansion::Q_MAP`]) are **unchanged**. This module fixes the components.
//!
//! ```text
//! P01  J_IN FH34 ──(pins 1-7, 9-12 pass-through)── J_OUT FH34      pin 8: J_IN → SER, QH' ─33Ω→ J_OUT
//!      +5V_CHAIN entry: 10 µF 16 V + 100 nF -> TPS62840 -> +3V3_LOCAL (rev D)
//! P02  SN74HC595PWR (LV595A alt, D-02): SRCLR→VCC, OE←PATCH_OE_N, 47 k pull-down on Q0–Q7
//!      Q0 PRI_PWR_EN → TPS22918 (CT 1 nF, QOD→VOUT) → PRI_VDD
//! P03  CD74HC4066PWR (TMUX1511 alt, D-03): ch1-3 ← Q1 (CLK, SYNC, SD), ch4 ← Q2 (ID)
//!      T/RH variant: SHT45-AD1B-R2 + R_ID 22.0 k     Mic variant: DMM-4026-B-I2S-R (LR, CONFIG → GND) + R_ID 68.0 k
//! P04  AUX (rev C): TMUX136 + TMUX1109 + CD74HC238 + CD74HC138 + 4 × TPS2553-1 + 4 × JST PH + 4 × TPD4E05U06 + arm sense
//! P05  test pads, variant labels, J_AUX1..4 silkscreen "1 VCC 2 GND 3 SDA 4 SCL"
//! ```

use crate::model::Status;

/// Patch population variants (exactly one primary sensor, REQ-PA-04).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    /// SHT45 T/RH patch.
    Trh,
    /// I²S microphone patch.
    Mic,
}

/// Variant-specific population.
pub const VARIANT_POPULATION: &[(Variant, &str, &str)] = &[
    (Variant::Trh, "Fit", "U_TRH SHT45-AD1B-R2 + 100 nF, R_ID 22.0 k 1 %; I²C pull-ups are the main-board switched 2.2 k (PRP-MB-02)"),
    (Variant::Trh, "DNI", "U_MIC + its 100 nF, R_ID 68.0 k"),
    (Variant::Mic, "Fit", "U_MIC DMM-4026-B-I2S-R + 100 nF, R_ID 68.0 k 1 %; LR and CONFIG to GND"),
    (Variant::Mic, "DNI", "U_TRH + its 100 nF, R_ID 22.0 k"),
];

/// Fail-safe analysis of the patch hardware (datasheet behaviour of the selected parts).
pub const FAIL_SAFE: &[(&str, &str)] = &[
    ("PATCH_OE_N high (boot, reset, fault)", "595 outputs Hi-Z (IOZ ≤ 5 µA) → 47 k pull-downs hold ≤ 0.24 V → TPS22918 off (VIL 0.5 V), 4066 open, CD74HC238 disabled (G2 = 0) → all TPS2553 EN low, CD74HC138 disabled → /Y6 high → TMUX136 open, TMUX1109 EN low."),
    ("Chain unpowered", "TMUX136 power-off isolation; TPS2553 off. HC595/HC4066 have no Ioff: ESP32 chain outputs must be low/Hi-Z and PATCH_OE_N pulled up on the patch to +3V3_LOCAL (rev D) to avoid back-powering through input clamps."),
    ("Random register contents after power-up", "OE_N follows +3V3_LOCAL through its on-patch pull-up, so outputs stay disabled until CHAIN_INIT latches the all-zero image and firmware drives OE_N low."),
    ("Primary and AUX bits together", "Q0/Q1 drive G0/G1 of both decoders: AUX power and bus blocked (rev C)."),
    ("Bus enable without power", "CD74HC138 /Y6 requires Q5 & Q6 & Q7: an unpowered module is never connected."),
    ("Port short / overload", "TPS2553-1 limits to 248-329 mA then latches off after 7.5 ms; chain TPS2553-1 minimum (349 mA) is above the port maximum so the port trips first."),
];

/// Patch layout/component rules.
pub const RULES: &[(&str, Status)] = &[
    ("Keep SHT45 away from TPS2553 and AUX headers; no copper under the SHT45 die (DS-SHT4X §5.3).", Status::Proposal),
    ("Mic Ø0.25 mm bottom port: PCB hole and gasket per DS-PUI-MIC; keep away from AUX modules with fans/pumps.", Status::Proposal),
    ("Route +5V_CHAIN pass-through ≥ 0.5 mm 1 oz (≤ 30 mΩ per patch, assumed in power_budget/power).", Status::Proposal),
    ("Keep the TPS62840 switch node and inductor away from the chain signal pass-throughs and the AUX I²C mux (rev D).", Status::Proposal),
    ("All four FH34 grounds tied to a continuous ground pour on both connectors.", Status::Proposal),
    ("33 Ω source termination on QH'; SRCLK/RCLK/OE are multi-drop — verify edges at patch 8 (V-02/V-11).", Status::Proposal),
    ("Pull-downs on Q0–Q7 are 47 kΩ (rev C '100 kΩ class' would allow 0.5 V on IOZ = TPS22918 VIL).", Status::Proposal),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_variant_has_fit_and_dni() {
        for v in [Variant::Trh, Variant::Mic] {
            assert!(VARIANT_POPULATION.iter().any(|p| p.0 == v && p.1 == "Fit"));
            assert!(VARIANT_POPULATION.iter().any(|p| p.0 == v && p.1 == "DNI"));
        }
    }

    #[test]
    fn register_map_unchanged_from_rev_c() {
        let nets: Vec<_> = crate::sensor_patch::REGISTER_MAP
            .iter()
            .map(|b| b.net)
            .collect();
        let q: Vec<_> = crate::aux_expansion::Q_MAP.iter().map(|b| b.net).collect();
        assert_eq!(nets, q);
        assert_eq!(
            nets,
            [
                "PRI_PWR_EN",
                "PRI_SIG_EN",
                "ID_EN",
                "AUX_A0",
                "AUX_A1",
                "AUX_SIG_EN",
                "AUX_PWR_EN",
                "AUX_ARM"
            ]
        );
    }
}
