//! # Engineering verification and unresolved decisions (component phase, rev C-components)
//!
//! **Schematic generation is on hold until the client reviews this phase** (instruction
//! 2026-11-02). This phase selects parts for the established rev C architecture; it does not change
//! the 12-pin ICD, the cascade, the 64-bit image, the ID model, the rev C Q-map or the client AUX
//! pinout. Where a datasheet check shows a weakness of a source/rev C part, a pin- or
//! function-compatible alternative is proposed as a decision (D-xx), never applied silently.

use crate::model::Priority;

/// Outcome of a calculation check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// Passes at worst case.
    Pass,
    /// Passes only under the stated condition.
    PassConditional,
    /// Fails at worst case — documented, with mitigation.
    Fail,
    /// Cannot be closed by calculation; bench or input needed.
    Open,
}

/// One calculation / verification result (each backed by a unit test or bench item).
#[derive(Debug, Clone, Copy)]
pub struct Check {
    /// ID.
    pub id: &'static str,
    /// What was checked.
    pub item: &'static str,
    /// Result summary.
    pub result: &'static str,
    /// Outcome.
    pub outcome: Outcome,
    /// Test that enforces it.
    pub test: &'static str,
}

/// Calculation results.
pub const CHECKS: &[Check] = &[
    Check { id: "CAL-01", item: "BQ24074 charge current (RISET 1.78 k)", result: "0.50 A nom, 0.44-0.55 A worst", outcome: Outcome::Pass, test: "passives::bq24074_charge_current_and_limits" },
    Check { id: "CAL-02", item: "BQ24074 input limit (RILIM 1.62 k, ILIM mode)", result: "0.99 A nom, 0.92-1.07 A", outcome: Outcome::Pass, test: "passives::bq24074_charge_current_and_limits" },
    Check { id: "CAL-03", item: "BQ24074 safety timer (RTMR 68.1 k)", result: "6.7-11.5 h ≥ 5.5 h needed; V1 46.4 k fails (4.6 h)", outcome: Outcome::Pass, test: "passives::bq24074_timer_covers_full_charge" },
    Check { id: "CAL-04", item: "BQ24074 junction temperature", result: "1.68 W → 115 °C at 40 °C ambient (< 125 °C regulation)", outcome: Outcome::PassConditional, test: "passives::bq24074_thermal_worst_case" },
    Check { id: "CAL-05", item: "AUX port current limit (TPS2553-1, rev C 93.1 k)", result: "248-329 mA ≥ 205 mA SCD41 peak; equations identical to rev C", outcome: Outcome::Pass, test: "passives::port_and_chain_current_limits" },
    Check { id: "CAL-06", item: "Chain current limit (TPS2553-1, rev D 76.8 k on +5V_CHAIN) vs worst load and 400 mA ceiling", result: "rev D: see passives test; rev C value 349-451 mA vs 335 mA worst load; ≤ 500 mA rating; inrush 1 ms < 7.5 ms deglitch", outcome: Outcome::Pass, test: "passives::port_and_chain_current_limits" },
    Check { id: "CAL-07", item: "TPS63021 rail capacity / inductor peak", result: "0.99 A design load vs 2 A rating; Ipeak ≤ 1.35 A (L −30 %) < 2.5 A Isat", outcome: Outcome::Pass, test: "power_budget::main_rail_within_regulator_and_battery" },
    Check { id: "CAL-08", item: "Battery peak current", result: "1.34 A at 3.0 V (< 1.5 A cell spec, < 2 A JST PH)", outcome: Outcome::Pass, test: "power_budget::main_rail_within_regulator_and_battery" },
    Check { id: "CAL-09", item: "Patch ID windows, forced PWM, TMUX1511 (or CD74HC4066 at ≤ 25 °C)", result: "T/RH 2.57-2.83 V, mic 1.78-2.14 V inside 2.50-2.95 / 1.70-2.20 V and < 2.9 V ADC range", outcome: Outcome::PassConditional, test: "passives::id_windows_pass_in_pwm_and_fail_in_ps_mode" },
    Check { id: "CAL-10", item: "Patch ID with TPS63021 in power-save mode", result: "T/RH up to 2.97 V > 2.9 V ADC range → force PWM (REG_PS_SYNC)", outcome: Outcome::Fail, test: "passives::id_windows_pass_in_pwm_and_fail_in_ps_mode" },
    Check { id: "CAL-11", item: "Patch ID with rev C CD74HC4066 at its 85 °C leakage (±1 µA × 16)", result: "T/RH minimum 2.31 V < 2.50 V window", outcome: Outcome::Fail, test: "passives::id_windows_pass_in_pwm_and_fail_in_ps_mode" },
    Check { id: "CAL-12", item: "Battery divider 1 M / 330 k", result: "≤ 1.07 V at 4.23 V (ATTEN2 1.6 V); 3.2 µA", outcome: Outcome::Pass, test: "passives::battery_divider_within_adc_range" },
    Check { id: "CAL-13", item: "I²C rise time, 2.2 k, ≈ 257 pF worst (rev C capacitance model ±5 %)", result: "≈ 480 ns ≤ 1000 ns (100 kHz); > 300 ns (400 kHz not supported)", outcome: Outcome::PassConditional, test: "passives::i2c_pullups_and_capacitance" },
    Check { id: "CAL-14", item: "I²C sink current with module pull-ups", result: "≤ 2.3 mA with 10 k/4.7 k module pull-ups; 3.2 mA with 2.2 k → DNI port pull-ups", outcome: Outcome::PassConditional, test: "passives::i2c_pullups_and_capacitance" },
    Check { id: "CAL-15", item: "I²C low level through TMUX1109 + TMUX136 + chain (3 mA, SCD4x VOL 0.66 V)", result: "0.74 V < 0.82 V ESP32 VIL; fails if a 0.16 V ground offset is present", outcome: Outcome::PassConditional, test: "aux_i2c::low_level_margin_through_isolator_mux_and_chain" },
    Check { id: "CAL-16", item: "595 Hi-Z leakage on pull-downs", result: "47 k: 0.235 V < 0.45 V; rev C '100 k class': 0.5 V = TPS22918 VIL → 47 k selected", outcome: Outcome::Pass, test: "passives::shift_register_pulldowns_hold_off" },
    Check { id: "CAL-17", item: "Chain drop, worst-case contacts, 0.1 m FFC, source 3.267 V", result: "205 mA at patch 8 → 2.43 V; ≥ 2.9 V limits 504/300/213/165/135/114/98/87 mA (patch 1..8); agrees with rev C model < 3 %", outcome: Outcome::Fail, test: "power_budget::chain_drop_worst_case_documents_far_patch_limit" },
    Check { id: "CAL-18", item: "AUX-armed sense windows (diode VF 0.40-0.65 V)", result: "one armed 1.04-1.17 V, two armed ≥ 1.49 V vs rev C windows 0.85-1.25 / > 1.30 V", outcome: Outcome::PassConditional, test: "aux_i2c::arm_sense_windows_with_selected_parts" },
    Check { id: "CAL-19", item: "Mic SD load on SENSOR_DATA_ID", result: "8 × (5 pF HC4066 + 3 pF) + 80 pF FFC + 7 pF ≈ 151 pF > 140 pF (PUI max) with 100 pF/m FFC assumption; ≈ 119 pF with TMUX1511 and 60 pF/m", outcome: Outcome::Open, test: "eng_verification::mic_sd_load" },
    Check { id: "CAL-20", item: "I²S timing at BCLK ≥ 2.048 MHz over 8 patches", result: "Half period 244 ns; mic data-valid time not legible in datasheet", outcome: Outcome::Open, test: "bench V-11" },
    Check { id: "CAL-21", item: "AUX logic (all 256 register states, OE_N both levels)", result: "bus connected ⇒ same port powered, armed, primary off", outcome: Outcome::Pass, test: "aux_i2c::exhaustive_component_level_safety" },
    Check { id: "CAL-22", item: "Thermal: TPS63021, chain and port switches", result: "all TJ < 70 °C at 40 °C ambient", outcome: Outcome::Pass, test: "power_budget::thermal_margins" },
    Check { id: "CAL-23", item: "Battery runtime (example duty cycle, assumptions A-B1)", result: "≈ 3.8 mA average → ≈ 420 h (17 days) on 2000 mAh × 0.8", outcome: Outcome::Open, test: "power_budget::example_runtime" },
];

/// Engineering-recommended parts (datasheet verified; client sign-off pending).
pub const APPROVED_FOR_V2: &[&str] = &[
    "ESP32-S3-WROOM-1-N16R2",
    "BQ24074RGTR",
    "TPS63021DSJR",
    "TPS2553DBVR-1",
    "TPS22918DBVR",
    "GCT USB4105-GF-A",
    "JST S2B-PH-SM4-TB",
    "TMUX136RSER",
    "TMUX1109PWR",
];
/// Recommended subject to a named drawing/bench condition.
pub const APPROVED_WITH_CONDITION: &[(&str, &str)] = &[
    ("Hirose FH34SRJ-12S-0.5SH(50)", "transcribe pads/contact side from the 2D drawing; FFC type A/D (OQ-024); measure contact R (V-06)"),
    ("Sensirion SHT45-AD1B-R2", "confirm 0x44 on order (one ordering-table row reads 0x45); AD1F membrane option for the hive"),
    ("PUI DMM-4026-B-I2S-R", "lifecycle/availability; BCLK ≥ 2.048 MHz and ≤ 140 pF SD load over the full chain (V-11)"),
    ("TI TPD4E05U06DQAR", "confirm orderable lifecycle"),
    ("TI SN74HC595PWR", "rev C baseline; acceptable if the chain-off back-power rule is kept, else LV595A (D-02)"),
    ("TI CD74HC238PWR / CD74HC138M96", "HC not characterised at 3.3 V; inputs are 595 rail-to-rail; bench V-02"),
];
/// Parts needing an alternative or re-verification.
pub const NEED_ALTERNATIVE_OR_REVERIFY: &[(&str, &str)] = &[
    ("TI CD74HC4066PWR (rev C)", "ID leakage fails at the 85 °C limit, RON unspecified at 3.3 V, 5 pF on the mic SD line, no power-off protection → TMUX1511PWR (D-03)"),
    ("Arm-sense diode", "no datasheet obtainable in this phase; requirement IR ≤ 50 nA @ 3.3 V 85 °C, VF ≤ 0.65 V @ 10 µA"),
    ("ADI MAX17048G+T10", "datasheet download failed; re-verify or drop (VBAT divider remains)"),
    ("ST USBLC6-2SC6", "datasheet download failed; re-verify or use TI TPD2E2U06 (downloaded, review pending)"),
    ("Lite-On LTST-C193 LEDs", "datasheets not reviewed"),
    ("1.5 µH inductor", "MPN to select (Isat ≥ 2.5 A, DCR ≤ 50 mΩ)"),
    ("All commodity passives", "MPN assignment from AVL (OQ-029)"),
    ("JST SM04B-SRSS-TB display connector", "JST SH drawing not obtained; connector choice is D-12"),
    ("Second I²S microphone source", "no second current-production I²S mic datasheet could be obtained"),
];
/// Rejected or removed candidates.
pub const REJECTED: &[(&str, &str)] = &[
    ("TDK ICS-43434", "EOL (source p.15)"),
    ("TI TPS62162", "buck cannot hold 3.3 V over the 1S range"),
    ("TI TCA9546A / PCA9546A", "consumes 0x70-0x77; software-only exclusivity (rev C)"),
    ("TI CD74HC4052", "RON too high for the I²C low-level budget (rev C)"),
    ("TI TMUX1209 / TMUX1109 directly on the chain", "32-38 pF off-capacitance × 8 → 463 pF (rev C); solved by TMUX136 isolator"),
    ("TI SN74CB3Q3253", "reviewed as mux+isolator; 11 pF A-port off-capacitance and 2 mA ICC per patch - rev C TMUX136+TMUX1109 retained"),
    ("TPS26600, TPS2121, SN74LVC1G3157, ICM-42688-P, SGP40, microSD, RF header", "outside V2 scope"),
];

/// A decision that needs client approval.
#[derive(Debug, Clone, Copy)]
pub struct PendingDecision {
    /// ID.
    pub id: &'static str,
    /// Priority.
    pub priority: Priority,
    /// Question.
    pub question: &'static str,
    /// Recommendation.
    pub recommendation: &'static str,
}

/// Decisions requiring client approval before schematic capture (APR-x = rev C approvals).
pub const DECISIONS: &[PendingDecision] = &[
    PendingDecision { id: "D-01", priority: Priority::Blocking, question: "Main module ESP32-S3-WROOM-1-N16R2 instead of V1 N16R8 (85 °C rating vs 65 °C; 2 MB quad PSRAM; GPIO35-37 free)?", recommendation: "Yes." },
    PendingDecision { id: "D-02", priority: Priority::High, question: "Patch shift register: keep SN74HC595PWR (source/rev C) or use pin-identical SN74LV595APWR (3.3 V characterised, Ioff, 5.5 V-tolerant)?", recommendation: "LV595A - zero layout impact, removes the chain-off back-power constraint." },
    PendingDecision { id: "D-03", priority: Priority::Blocking, question: "Primary switch: replace CD74HC4066 by TMUX1511PWR (ID leakage 10× lower, RON ≤ 4.5 Ω, powered-off protection)?", recommendation: "Yes - the CD74HC4066 fails the ID window at its 85 °C leakage limit (CAL-11) and loads the mic SD line." },
    PendingDecision { id: "D-04", priority: Priority::Blocking, question: "Approve the rev C AUX part set with exact MPNs (TMUX136RSER, TMUX1109PWR, CD74HC238PWR, CD74HC138M96, 4 × TPS2553DBVR-1 at 93.1 k, 47 k pull-downs, TPD4E05U06 ESD)?", recommendation: "Yes (REC-02 with component values fixed)." },
    PendingDecision { id: "D-05", priority: Priority::High, question: "UPDATED rev D: TPS2553DBVR-1 now on +5V_CHAIN with 76.8 k (306-388 mA, <= 400 mA ceiling); OE_N pull-up on each patch. Original: Chain power gating: TPS2553DBVR-1 (66.5 k, 349-451 mA) switched by GPIO, chain off between scans, PATCH_OE_N pulled up to +3V3_CHAIN?", recommendation: "Yes." },
    PendingDecision { id: "D-06", priority: Priority::High, question: "Power stage: BQ24074 + TPS63021 with GPIO-forced PWM during ID/audio (resolves source 'buck-boost preferred')?", recommendation: "Yes." },
    PendingDecision { id: "D-07", priority: Priority::Blocking, question: "RESOLVED 2026-11-01: APR-1 option B APPROVED subject to power validation (+5V_CHAIN, local +3V3_LOCAL). Original: Far-patch AUX power (APR-1): accept position classes (≤ 80 mA anywhere, SCD41-class on patches 1-3) until V-06, or approve APR-1 option B (higher pin-2 voltage + local LDOs)?", recommendation: "Option A now; revisit with V-06 data." },
    PendingDecision { id: "D-08", priority: Priority::High, question: "Microphone: accept PUI DMM-4026-B-I2S-R with fs ≥ 32 kHz (BCLK ≥ 2.048 MHz) and the 140 pF SD-load limit (requires short FFC or TMUX1511)?", recommendation: "Accept for prototype; bench V-11." },
    PendingDecision { id: "D-09", priority: Priority::Medium, question: "USB input: USB500 default with firmware-selected ~1 A ILIM mode for Type-C chargers?", recommendation: "Yes." },
    PendingDecision { id: "D-10", priority: Priority::High, question: "PARTLY RESOLVED: capacity 3000 mAh CONFIRMED; cell configuration/voltages/currents/protection still need the pack datasheet. Original: Battery (APR-4): capacity, cell MPN with 10 k NTC, autonomy target.", recommendation: "≥ 2000 mAh protected 1S, 10 k NTC, ≥ 1.5 A continuous." },
    PendingDecision { id: "D-11", priority: Priority::Blocking, question: "SUPERSEDED: APR-2 approves JST-GH 1.25 mm (MPN/footprint to verify), not JST-PH. Original: AUX connector (APR-2): JST B4B-PH-K-S keyed 2.0 mm, pin order VCC-GND-SDA-SCL?", recommendation: "Yes." },
    PendingDecision { id: "D-12", priority: Priority::Medium, question: "Display connector: JST SH 4-pin (different family from AUX; source pin order GND, 3V3, SCL, SDA)?", recommendation: "Yes." },
    PendingDecision { id: "D-13", priority: Priority::Medium, question: "Confirm V2 has no solar/6 V input (V1 TPS26600/TPS2121 removed).", recommendation: "Remove unless required." },
    PendingDecision { id: "D-14", priority: Priority::Blocking, question: "PROVISIONAL: 100 mm segments for the prototype (APR-4); FPC type/spec still open. Original: FFC/FPC (APR-4, OQ-024): type (same/opposite side), segment length (≤ 0.1 m assumed), conductor size.", recommendation: "≤ 0.1 m per segment, 0.3 mm FPC per FH34; measure resistance and capacitance." },
    PendingDecision { id: "D-15", priority: Priority::Medium, question: "RESOLVED: keep manufacturer-neutral until quotes (APR-4). Original: Fabricator/AVL for passive MPN assignment (OQ-029, REC-07 original).", recommendation: "JLCPCB/LCSC unless an Indian AVL is required (V1)." },
];

/// Assumptions introduced in this phase.
pub const ASSUMPTIONS: &[(&str, &str)] = &[
    ("A-P1", "Resistor drift 0.5 % over life/temperature in addition to 1 % tolerance."),
    ("A-P2", "CV phase ≤ 1 h for charge-timer sizing."),
    ("A-C1", "FFC conductor 0.30 × 0.035 mm, 0.1 m per segment, 100 pF/m (rev C value); FH34 contacts at 100 mΩ max."),
    ("A-V1", "Port VCC ≥ 2.9 V criterion (rev C); module-specific minimum may be lower (SCD41 2.4 V)."),
    ("A-T1", "Hive enclosure ambient 40 °C for thermal checks."),
    ("A-L1", "Display ≤ 30 mA (MPN open, OQ-017)."),
    ("A-B1", "Duty cycle: 10 min period, 15 s awake at 70 mA + 25 mA chain, 2 s Wi-Fi TX at 355 mA, 30 µA sleep, η 0.85, 80 % usable capacity."),
    ("A-B2", "TPS63021 efficiency 0.85-0.90 at the loads used (curve not transcribed)."),
];

/// Bench tests that must precede release of the affected selections.
pub const BENCH: &[(&str, &str)] = &[
    ("V-06", "Measure FH34 + FFC loop resistance and patch-8 rail with 80/150/205 mA AUX loads; update power_budget::worst_case_model."),
    ("V-11", "I²S at BCLK 2.048 MHz and I²C 100 kHz at patch 8 with 8 patches: edges, ACK, data integrity, SD load."),
    ("V-03/V-08", "SENSOR_DATA_ID: ADC ID windows in forced PWM at 25 and 40 °C; I²S DIN on GPIO1 of the N16R2."),
    ("V-15", "AUX-armed sense with the selected diode (rev C)."),
    ("V-AUX-1", "Walk all 256 register states per patch on hardware; verify no back-powering of an unpowered module."),
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn mic_sd_load() {
        // Rev C baseline: CD74HC4066 CI 5 pF + 3 pF trace per patch, FFC 0.8 m × 100 pF/m, main 7 pF.
        let hc = 8.0 * (5.0 + 3.0) + 0.8 * 100.0 + 7.0;
        assert!(hc > 140.0, "documented as open: {hc}");
        // TMUX1511 (COFF 4 pF) with a 60 pF/m FFC meets the 140 pF limit.
        let tm = 8.0 * (4.0 + 3.0) + 0.8 * 60.0 + 7.0;
        assert!(tm <= 140.0, "{tm}");
    }

    #[test]
    fn ids_unique_and_failures_have_decisions() {
        let mut s = HashSet::new();
        for c in CHECKS {
            assert!(s.insert(c.id));
        }
        for d in DECISIONS {
            assert!(s.insert(d.id));
        }
        assert!(CHECKS.iter().filter(|c| c.outcome == Outcome::Fail).count() >= 3);
        for needed in ["D-03", "D-06", "D-07"] {
            assert!(DECISIONS.iter().any(|d| d.id == needed));
        }
    }

    #[test]
    fn approved_parts_are_recommended_specs() {
        for a in APPROVED_FOR_V2 {
            let m = a.trim_start_matches("GCT ").trim_start_matches("JST ");
            let s = crate::component_specs::SPECS
                .iter()
                .find(|s| m.starts_with(s.mpn) || s.mpn.starts_with(m))
                .unwrap_or_else(|| panic!("{a}"));
            assert_eq!(
                s.status,
                crate::component_model::EngStatus::Recommended,
                "{a}"
            );
        }
    }
}

#[cfg(test)]
mod report {
    /// `cargo test report -- --nocapture` prints the key computed values used in this module's text.
    #[test]
    fn print_key_values() {
        use crate::passives::*;
        let w = |c: &IdCheck, r| id_window(c, r, R_ID_PD);
        println!(
            "ID PWM trh {:?} mic {:?}",
            w(&ID_PWM, R_ID_TRH),
            w(&ID_PWM, R_ID_MIC)
        );
        println!("ID PS trh {:?}", w(&ID_PS, R_ID_TRH));
        println!(
            "ID HC85 trh {:?} HC25 trh {:?} mic {:?}",
            w(&ID_HC4066_85C, R_ID_TRH),
            w(&ID_HC4066_25C, R_ID_TRH),
            w(&ID_HC4066_25C, R_ID_MIC)
        );
        println!(
            "port {:?} chain {:?}",
            tps255x_window_ma(R_ILIM_PORT_K),
            tps255x_window_ma(R_ILIM_CHAIN_K)
        );
        let (_, pmax) = tps255x_window_ma(R_ILIM_PORT_K);
        println!(
            "chain worst {}",
            crate::power_budget::chain_worst_load_ma(pmax)
        );
        println!(
            "Cb {} tr {}",
            I2C_BUS_WORST.total(),
            i2c_rise_ns(R_PULLUP_PATCH, I2C_BUS_WORST.total())
        );
        let m = crate::power_budget::worst_case_model();
        let lim: Vec<f64> = (1..=8).map(|p| m.max_aux_ma(p, 2.9)).collect();
        println!("limits {:?} p8@205 {}", lim, m.rail_profile(205.0, 8)[7]);
        println!(
            "main rail {}",
            crate::power_budget::sum_max(crate::power_budget::MAIN_RAIL)
        );
        println!(
            "avg batt {}",
            crate::power_budget::average_battery_ma(&crate::power_budget::EXAMPLE_SCENARIO)
        );
        println!(
            "vol {}",
            crate::aux_i2c::MODULE_VOL_WORST + crate::aux_i2c::PATH_WORST.added_vol()
        );
    }
}
