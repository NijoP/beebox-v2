//! # BeeBox V2 - Engineering Documentation (typed baseline)
//!
//! Authoritative pre-implementation engineering documentation for **BeeBox V2**, a modular
//! honeybee-colony sensor platform. It contains no firmware, schematic or PCB content.
//!
//! * Primary source: [`sources::PRIMARY_DOCUMENT`] (BBX-ENG-PR-003 Rev 0.2), archived under `docs/00_source/`.
//! * Client instructions override the source: [`sources::CLIENT_INSTRUCTIONS`].
//! * Every statement is typed with a [`model::Status`] and [`model::Source`]. Proposals are
//!   **not** approved designs.
//!
//! ## Start here
//! 1. [`purpose`] - purpose, boundary and confirmed requirements.
//! 2. [`approvals`] - validation findings (proven / needs test) and the remaining **approvals** (APR-1..APR-4).
//! 3. [`aux_expansion`] - the four-port **I²C** AUX system (revision C, validated logic model and truth table).
//! 4. [`research`] and [`calculations`] - datasheet evidence and checked calculations.
//!
//! ## Modules
//! | # | Module | Content |
//! |---|--------|---------|
//! | 1 | [`purpose`] | Purpose and requirements |
//! | 2 | [`architecture`] | System architecture |
//! | 3 | [`main_controller`] | Main controller |
//! | 4 | [`sensor_patch`] | Sensor-patch architecture, register map, ID model |
//! | 5 | [`aux_expansion`] | Four-port AUX expansion |
//! | 6 | [`interfaces`] | Electrical interfaces and pin assignments |
//! | 7 | [`power`] | Power and thermal |
//! | 8 | [`components`] | Component candidates and BOM rules |
//! | 9 | [`firmware`] | Firmware architecture |
//! | 10 | [`data_flow`] | Communication and data flow |
//! | 11 | [`manufacturing`] | Manufacturing, PCB, EasyEDA structure |
//! | 12 | [`verification`] | Testing and verification |
//! | 13 | [`governance`] | Risks, assumptions, decisions |
//! | 14 | [`phases`] | Phases and release criteria |
//! | - | [`approvals`] | Recommendations and client decisions |
//! | - | [`research`] | Gas-module and component datasheet evidence |
//! | - | [`calculations`] | Chain budget, I²C RON budget, latency, energy |
//! | - | [`model`], [`sources`] | Shared types, provenance |
//!
//! ## Component-engineering phase (2026-11-02, built on the rev C architecture) — schematic generation on hold for review
//! | Module | Content |
//! |---|---|
//! | [`v1_audit`] | V1 component audit and V1→V2 compatibility matrix |
//! | [`bom`] | Recommended V2 master BOM (main + patch) |
//! | [`component_specs`] | Datasheet-verified component specifications |
//! | [`passives`] | Passive-component calculations (tested) |
//! | [`power_budget`] | Power, current, chain-drop and thermal analysis (tested) |
//! | [`main_board`] | Main-board component architecture and GPIO map |
//! | [`patch_board`] | Sensor-patch component architecture and fail-safe analysis |
//! | [`aux_i2c`] | AUX I²C expansion component design and interlock truth tables |
//! | [`pinmap`] | Pin mapping and footprint sources |
//! | [`eng_verification`] | Calculation results, approvals, pending decisions D-01..D-16 |
//! | [`datasheets`], [`component_model`] | Evidence register and shared component types |
//!
//! ## Release review (2026-11-02) — start at [`release_review`]
//! Component approval register, Rev C / revision-D review, candidate 12-pin interface
//! ([`chain_candidate`]), global AUX interlock proof ([`global_interlock`]), safe start-up and
//! sequencing ([`startup_safety`]), 5 V chain ([`chain_power_5v`]), battery and USB-C
//! ([`battery_charging`]). Not manufacturing-ready; schematic generation on hold.
//!
//! ## Maintenance
//! * Build HTML: `cargo doc --no-deps` → `target/doc/beebox_v2_engineering/index.html`.
//! * Consistency checks: `cargo test` (pinout integrity, ID voltages, unique IDs, approval coverage).
//! * To approve a proposal: change its status to [`model::Status::Approved`], add a
//!   [`model::Decision`] to [`governance::DECISIONS`] and remove the corresponding approval question.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub mod model;
pub mod sources;

pub mod approvals;
pub mod architecture;
pub mod aux_expansion;
pub mod calculations;
pub mod components;
pub mod data_flow;
pub mod firmware;
pub mod governance;
pub mod interfaces;
pub mod main_controller;
pub mod manufacturing;
pub mod phases;
pub mod power;
pub mod purpose;
pub mod research;
pub mod sensor_patch;
pub mod verification;

pub mod aux_i2c;
pub mod bom;
pub mod component_model;
pub mod component_specs;
pub mod datasheets;
pub mod eng_verification;
pub mod main_board;
pub mod passives;
pub mod patch_board;
pub mod pinmap;
pub mod power_budget;
pub mod v1_audit;

// Release review 2026-11-02 (items A-E, 5 V chain, 3000 mAh battery).
pub mod battery_charging;
pub mod chain_candidate;
pub mod chain_power_5v;
pub mod global_interlock;
pub mod release_review;
pub mod startup_safety;

#[cfg(test)]
mod consistency_tests {
    use super::*;
    use interfaces::{NetClass, Routing, CHAIN_PINS};
    use std::collections::HashSet;

    #[test]
    fn chain_pinout_integrity() {
        for (i, p) in CHAIN_PINS.iter().enumerate() {
            assert_eq!(p.pin as usize, i + 1);
        }
        let directional: Vec<_> = CHAIN_PINS
            .iter()
            .filter(|p| p.routing == Routing::DirectionalCascade)
            .collect();
        assert_eq!(directional.len(), 1);
        assert_eq!(directional[0].pin, 8);
        assert_eq!(
            CHAIN_PINS
                .iter()
                .filter(|p| p.class == NetClass::Ground)
                .count(),
            4
        );
        assert_eq!(
            CHAIN_PINS
                .iter()
                .filter(|p| p.class == NetClass::Power)
                .count(),
            1
        );
        assert_eq!(
            CHAIN_PINS
                .iter()
                .filter(|p| p.class == NetClass::SharedSensor)
                .count(),
            3
        );
        assert_eq!(
            CHAIN_PINS
                .iter()
                .filter(|p| p.class == NetClass::Control)
                .count(),
            4
        );
    }

    #[test]
    fn id_voltages_match_source_and_windows() {
        use sensor_patch::*;
        let trh = nominal_id_millivolts(22_000, ID_PULLDOWN_OHMS, V_CHAIN_NOMINAL_MV);
        let mic = nominal_id_millivolts(68_000, ID_PULLDOWN_OHMS, V_CHAIN_NOMINAL_MV);
        assert_eq!((trh, mic), (2704, 1964)); // source: ~2.705 V, ~1.964 V
        for c in ID_CODES {
            if let Some(r) = c.r_id_ohms {
                let v = nominal_id_millivolts(r, ID_PULLDOWN_OHMS, V_CHAIN_NOMINAL_MV);
                assert!(v >= c.window_mv.0 && v <= c.window_mv.1, "{}", c.population);
            }
        }
    }

    #[test]
    fn aux_port_count_is_confirmed_four() {
        assert_eq!(purpose::AUX_PORTS_PER_PATCH, 4);
        assert_eq!(aux_expansion::AUX_PORT_DESIGNATORS.len(), 4);
        assert_eq!(purpose::MAX_AUX_PORTS_PER_SYSTEM, 32);
        assert_eq!(sensor_patch::BASELINE_CONTROL_IMAGE_BITS, 64);
    }

    #[test]
    fn identifiers_are_unique() {
        let mut ids = HashSet::new();
        for r in purpose::REQUIREMENTS {
            assert!(ids.insert(r.id), "dup {}", r.id);
        }
        for d in governance::DECISIONS {
            assert!(ids.insert(d.id), "dup {}", d.id);
        }
        for r in governance::RISKS {
            assert!(ids.insert(r.id), "dup {}", r.id);
        }
        for a in governance::ASSUMPTIONS {
            assert!(ids.insert(a.id), "dup {}", a.id);
        }
        for q in approvals::APPROVALS {
            assert!(ids.insert(q.id), "dup {}", q.id);
        }
        for r in approvals::RECOMMENDATIONS {
            assert!(ids.insert(r.0), "dup {}", r.0);
        }
        for f in approvals::FINDINGS {
            assert!(ids.insert(f.id), "dup {}", f.id);
        }
        for r in governance::RISKS_REV_C {
            assert!(ids.insert(r.id), "dup {}", r.id);
        }
        for r in governance::RISKS_REV_D {
            assert!(ids.insert(r.id), "dup {}", r.id);
        }
        for t in verification::TESTS {
            assert!(ids.insert(t.id), "dup {}", t.id);
        }
    }

    #[test]
    fn requirements_reference_existing_tests() {
        let tests: HashSet<_> = verification::TESTS.iter().map(|t| t.id).collect();
        for r in purpose::REQUIREMENTS {
            for v in r.verified_by {
                assert!(tests.contains(v), "{} -> {}", r.id, v);
            }
        }
    }

    #[test]
    fn aux_confirmed_items_never_reopened() {
        for q in approvals::APPROVALS {
            let t = q.question.to_lowercase();
            assert!(
                !t.contains("how many aux") && !t.contains("pinout"),
                "{}",
                q.id
            );
        }
    }

    #[test]
    fn aux_is_i2c_with_client_pinout_and_fits_existing_register() {
        use aux_expansion::*;
        assert_eq!(
            AUX_HEADER_PINOUT,
            [(1, "VCC"), (2, "GND"), (3, "SDA"), (4, "SCL")]
        );
        let outs: Vec<_> = Q_MAP.iter().map(|b| b.output).collect();
        assert_eq!(outs, ["Q0", "Q1", "Q2", "Q3", "Q4", "Q5", "Q6", "Q7"]);
        assert_eq!(sensor_patch::REGISTER_MAP[7].net, "AUX_ARM");
        assert_eq!(sensor_patch::BASELINE_CONTROL_IMAGE_BITS, 64);
        assert_eq!(RECOMMENDED_VOLTAGE_POLICY, VoltagePolicy::Only3v3);
        assert!(data_flow::LINE_USAGE
            .iter()
            .filter(|l| l.0 == data_flow::BusMode::AuxI2c)
            .all(|l| l.3.is_none()));
    }

    #[test]
    fn patch_truth_table_exhaustive() {
        use aux_expansion::patch_outputs;
        for q in 0u16..=255 {
            let q = q as u8;
            let o = patch_outputs(q, false);
            let primary = q & 0b11 != 0;
            // bus connected only to a powered port, and that port is the muxed one
            if o.aux_bus_connected {
                assert_eq!(o.aux_port_powered, Some((q >> 3) & 3), "q={q:08b}");
                assert_eq!(o.mux_port, o.aux_port_powered);
            }
            // primary activity blocks AUX power and bus
            if primary {
                assert!(
                    o.aux_port_powered.is_none() && !o.aux_bus_connected,
                    "q={q:08b}"
                );
            }
            // power needs Q6 and Q7; bus needs Q5, Q6, Q7
            if o.aux_port_powered.is_some() {
                assert!(q & 0xC0 == 0xC0);
            }
            if o.aux_bus_connected {
                assert!(q & 0xE0 == 0xE0);
            }
            // OE_N high -> everything off, regardless of register content
            let off = patch_outputs(q, true);
            assert_eq!(off, patch_outputs(0, false));
        }
        let z = patch_outputs(0, false);
        assert!(
            !z.primary_power
                && !z.primary_signal
                && !z.id_branch
                && z.aux_port_powered.is_none()
                && !z.aux_bus_connected
                && !z.aux_armed
        );
    }

    #[test]
    fn active_path_gives_single_path_system_wide() {
        use firmware::*;
        let mut paths = vec![ActivePath::Idle];
        for patch in 0..8u8 {
            for stage in [
                PrimaryStage::Identify,
                PrimaryStage::Powered,
                PrimaryStage::Connected,
            ] {
                paths.push(ActivePath::Primary { patch, stage });
            }
            for port in 0..4u8 {
                for stage in [AuxStage::Armed, AuxStage::Powered, AuxStage::Connected] {
                    paths.push(ActivePath::Aux { patch, port, stage });
                }
            }
        }
        for p in paths {
            let img = p.image();
            assert!(img.iter().filter(|b| **b != 0).count() <= 1);
            let outs: Vec<_> = img
                .iter()
                .map(|b| aux_expansion::patch_outputs(*b, false))
                .collect();
            let powered = outs.iter().filter(|o| o.aux_port_powered.is_some()).count();
            let connected = outs.iter().filter(|o| o.aux_bus_connected).count();
            let primary = outs
                .iter()
                .filter(|o| o.primary_power || o.primary_signal)
                .count();
            assert!(
                powered <= 1 && connected <= 1 && powered + primary <= 1,
                "{p:?}"
            );
            if let ActivePath::Aux {
                patch,
                port,
                stage: AuxStage::Connected,
            } = p
            {
                assert_eq!(outs[patch as usize].aux_port_powered, Some(port));
                assert!(outs[patch as usize].aux_bus_connected);
            }
        }
        assert_eq!(
            ActivePath::Aux {
                patch: 9,
                port: 0,
                stage: AuxStage::Connected
            }
            .image(),
            [0; 8]
        );
    }

    #[test]
    fn arm_sense_separates_one_from_two() {
        use calculations::arm_sense_v;
        let one_max = arm_sense_v(1, 3.00);
        let two_min = arm_sense_v(2, 2.35);
        assert!(two_min - one_max > 0.10, "{one_max} {two_min}");
        assert!((arm_sense_v(1, 2.35) - 0.94).abs() < 0.01);
    }

    #[test]
    fn calculations_match_documented_results() {
        use calculations::*;
        let (lo, nom, hi) = tps2553_ilim_ma(93.1);
        assert!((lo - 252.0).abs() < 2.0 && (nom - 286.0).abs() < 2.0 && (hi - 324.0).abs() < 2.0);
        assert!(lo > SCD41_PEAK_MAX_MA + BREAKOUT_OVERHEAD_MA_ASSUMED);
        assert!(hi + 8.0 * PATCH_BASE_UA_MAX / 1000.0 < FH34_RATED_MA);
        assert!((trace_ohm(30.0, 0.5, 35.0) - 0.0295).abs() < 0.001);
        // Contacts + switches only (no cable): patch 8 at 229 mA
        let mut c = Chain::baseline(0.0);
        c.cable_ohm_per_m = 0.0;
        c.trace_ohm = 0.0;
        let v = c.port_voltage(8, 229.0, 3.3);
        assert!((3.3 - v - 0.52).abs() < 0.01 && v < 2.9, "{v}");
        // With assumed cable, 0.10 m segments
        let c = Chain::baseline(0.10);
        assert!((c.max_current_ma(8, 3.3, 2.9) - 95.0).abs() < 2.0);
        assert!((c.max_current_ma(4, 3.3, 2.9) - 179.0).abs() < 2.0);
        assert!((c.max_current_ma(1, 3.3, 2.9) - 525.0).abs() < 3.0);
        assert!(c.max_current_ma(8, 3.3, 2.9) > STCC4_PEAK_MAX_MA);
        // Bus capacitance
        assert!((bus_capacitance_pf(0.10, true) - 255.0).abs() < 1.0);
        assert!(bus_capacitance_pf(0.10, false) > 400.0);
        assert!(bus_capacitance_pf(0.50, true) > 400.0);
        let rmax = r_pullup_max_ohm(1000.0, bus_capacitance_pf(0.10, true));
        let rmin = r_pullup_min_ohm(3.3);
        let reff = 1.0 / (1.0 / 2200.0 + 1.0 / 10_000.0);
        assert!(reff > rmin && reff < rmax);
        // R_ON budget: TMUX path vs HC4052
        let i = 3.3 / reff * 1000.0;
        let vol_tmux = vol_at_esp32_v(0.4, i, TMUX1109_RON_MAX_OHM + TMUX136_RON_MAX_OHM, 0.0);
        assert!(vol_tmux - 0.4 < 0.05);
        assert!(vol_at_esp32_v(0.4, i, 180.0, 0.0) - 0.4 > 0.3);
        assert!(vol_tmux < ESP32S3_VIL_FACTOR * 3.3);
        assert!((scan_time_s(32, 5000.0) - 160.0).abs() < 1e-9);
    }

    #[test]
    fn rev_d_power_validation() {
        use calculations::Chain;
        let c = Chain::baseline(0.10);
        for k in 1..=8u32 {
            for eta in [0.80, 0.88] {
                let (i, vin, gnd) = power::option_b_operating_point(
                    &c,
                    k,
                    3.3 * 0.225,
                    eta,
                    8.0 * 0.000314 * 3.3 / eta,
                    5.0,
                )
                .unwrap();
                assert!(i < power::CONTACT_CEILING_MA && i < 240.0, "k={k} {i}");
                assert!(vin > power::PATCH_VIN_MIN_V, "k={k} vin {vin}");
                assert!(
                    0.4 + 0.042 + gnd < calculations::ESP32S3_VIL_FACTOR * 3.3,
                    "VOL budget k={k}"
                );
            }
        }
        let (lo, hi) = passives::tps255x_window_ma(passives::R_ILIM_CHAIN_K);
        assert!(hi <= power::CONTACT_CEILING_MA && lo > 240.0, "{lo}-{hi}");
        let pin2 = interfaces::CHAIN_PINS[1];
        assert_eq!((pin2.pin, pin2.net), (2, "+5V_CHAIN"));
    }

    #[test]
    fn rev_d_battery_scenarios_are_ordered_and_documented() {
        let [o, t, w] = power::SCENARIOS;
        let (ho, ht, hw) = (o.runtime_h(), t.runtime_h(), w.runtime_h());
        assert!(ho > ht && ht > hw);
        assert!(
            (ho / 24.0 - 36.2).abs() < 1.0
                && (ht / 24.0 - 14.8).abs() < 1.0
                && (hw / 24.0 - 1.5).abs() < 0.2,
            "{ho} {ht} {hw}"
        );
        let peak = power::battery_peak_ma(340.0, 388.0, 0.85, 0.82, 3.0);
        assert!(peak > 1200.0 && peak < 1300.0);
    }

    #[test]
    fn research_verdicts() {
        use research::*;
        assert!(!MODULES
            .iter()
            .any(|m| m.gas == "CH4" && m.verdict == AuxVerdict::CompatibleClass));
        assert!(
            MODULES
                .iter()
                .filter(|m| m.gas == "CO2" && m.verdict == AuxVerdict::CompatibleClass)
                .count()
                >= 2
        );
    }

    #[test]
    fn no_aux_module_approved_yet() {
        assert!(firmware::APPROVED_AUX_PROFILES.is_empty());
    }
}
