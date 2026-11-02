//! # Engineering release review — component and architecture baseline (2026-11-02)
//!
//! **Not manufacturing-ready. Schematic generation remains on hold.** This review applies the client
//! directions of 2026-11-02 to the rev C proposal (and to the parallel "revision D" power work found
//! in [`crate::power`] / [`crate::interfaces`]) without treating either as approved.
//!
//! Detail modules: [`crate::chain_candidate`] (12-pin), [`crate::global_interlock`] (items A, B),
//! [`crate::startup_safety`] (items C, D), [`crate::chain_power_5v`] (5 V chain),
//! [`crate::battery_charging`] (3000 mAh, item E).

/// Approval status of a component (engineering view; client sign-off still applies).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Approval {
    /// Datasheet-verified for the schematic baseline; no open electrical question.
    Approved,
    /// Verified, but a named bench test, document or client input must close first.
    Conditional,
    /// Cannot be independently verified, or fails a requirement.
    NotApproved,
    /// Removed from V2.
    Rejected,
}

/// One approval-register entry.
#[derive(Debug, Clone, Copy)]
pub struct Entry {
    /// Function.
    pub function: &'static str,
    /// Exact MPN.
    pub mpn: &'static str,
    /// Status.
    pub status: Approval,
    /// Evidence / condition / reason.
    pub note: &'static str,
}

/// Component approval register.
pub const REGISTER: &[Entry] = &[
    // ---- main board
    Entry { function: "MCU module", mpn: "ESP32-S3-WROOM-1-N16R2", status: Approval::Conditional, note: "85 °C rating; 2 MB quad PSRAM holds ≤ 15 s of 32 kHz/24-bit mono audio (psram_audio_seconds); longer clips need N16R8 with PSRAM ECC (85 °C, 7.5 MB) - depends on the audio requirement (A-03)" },
    Entry { function: "Charger / power path", mpn: "BQ24074RGTR", status: Approval::Conditional, note: "RISET 1.27 k (0.62-0.78 A), RTMR 68.1 k, USB500 default; TJ ≤ 122 °C worst at 40 °C - bench thermal V-20" },
    Entry { function: "+3V3 buck-boost", mpn: "TPS63802DLAR", status: Approval::Approved, note: "1.3-5.5 V in, 2 A at VI ≥ 2.3 V, VFB 500 mV ±1 %, MODE pin for forced PWM; FB 562 k / 100 k → 3.31 V (TPS63021DSJR remains a verified alternate)" },
    Entry { function: "5 V chain boost", mpn: "TPS61023DRLR", status: Approval::Approved, note: "true disconnect in shutdown; VREF 580-610 mV; FB 750 k / 100 k 0.1 % → 4.90-5.21 V < 5.5 V OVP" },
    Entry { function: "Chain current limit", mpn: "TPS2553DBVR-1", status: Approval::Approved, note: "R_ILIM 76.8 k → 302-394 mA (≤ 400 mA ceiling), latch-off 7.5 ms, reverse blocking, ton ≤ 3 ms" },
    Entry { function: "Display power switch", mpn: "TPS22918DBVR", status: Approval::Approved, note: "2 A, QOD, ON active high" },
    Entry { function: "USB ESD", mpn: "TPD2E2U06DRLR", status: Approval::Approved, note: "replaces unverifiable USBLC6-2SC6; 1.5 pF, VRWM 5.5 V, IEC ±25 kV contact; orderable Active" },
    Entry { function: "Chain-line ESD (8 signals incl. AUX_ARM)", mpn: "TPD4E05U06DQAR ×2", status: Approval::Approved, note: "0.5 pF, VRWM 5.5 V; orderable Active" },
    Entry { function: "USB-C receptacle", mpn: "GCT USB4105-GF-A", status: Approval::Approved, note: "5 A VBUS, 20 000 cycles" },
    Entry { function: "Chain connector", mpn: "Hirose FH34SRJ-12S-0.5SH(50)", status: Approval::Conditional, note: "0.5 A, 100 mΩ max, 20 cycles; pad map from 2D drawing; contact R / temperature rise V-06, V-16" },
    Entry { function: "Battery connector", mpn: "JST S2B-PH-SM4-TB(LF)(SN)", status: Approval::Approved, note: "2 A, 100 V, -40..105 °C; pack polarity check" },
    Entry { function: "Display connector", mpn: "JST SM04B-SRSS-TB(LF)(SN)", status: Approval::Conditional, note: "SH series verified (1 A, 50 V, -25..85 °C, 20 mΩ); exact MPN drawing not obtained; display module open" },
    Entry { function: "Fuel gauge", mpn: "MAX17048G+T10", status: Approval::NotApproved, note: "datasheet unobtainable (analog.com connection reset, 3 attempts); VBAT divider 1 M/330 k on ADC1 is the verified fallback" },
    Entry { function: "USB ESD (V1)", mpn: "USBLC6-2SC6", status: Approval::NotApproved, note: "datasheet unobtainable (st.com timeout); replaced by TPD2E2U06" },
    Entry { function: "Status LEDs", mpn: "Lite-On LTST-C193KRKT/KGKT/KBKT-5A", status: Approval::NotApproved, note: "datasheets not obtained" },
    Entry { function: "Inductors (1.5 µH TPS63802, 1 µH TPS61023, 2.2 µH TPS62840)", mpn: "not selected", status: Approval::NotApproved, note: "values from datasheets; MPN/Isat/DCR to select" },
    Entry { function: "Commodity R/C", mpn: "AVL", status: Approval::NotApproved, note: "values and tolerances fixed in passives/chain_power_5v/global_interlock; MPNs pending AVL" },
    Entry { function: "Battery pack", mpn: "client 3000 mAh 1S", status: Approval::NotApproved, note: "capacity confirmed; pack datasheet (voltages, PCM, NTC, current) required" },
    // ---- patch
    Entry { function: "Shift register", mpn: "SN74LV595APWR", status: Approval::Approved, note: "characterised at 3.3 V, Ioff 5 µA, 5.5 V-tolerant inputs, IOZ ±5 µA; pin-identical to SN74HC595" },
    Entry { function: "Primary switch", mpn: "TMUX1511PWR", status: Approval::Conditional, note: "4 independent SPST (SEL1-4 active high, 6 MΩ pull-downs), RON ≤ 4.5 Ω, COFF ≤ 4 pF, IS(OFF) ±100 nA; I²S at patch 8 bench V-11" },
    Entry { function: "Primary sensor switch", mpn: "TPS22918DBVR", status: Approval::Approved, note: "default off via 47 k pull-down; QOD" },
    Entry { function: "Patch regulator", mpn: "TPS62840DLCR", status: Approval::Conditional, note: "1.8-6.5 V in, 750 mA, 60 nA IQ, RSET 267 k → 3.3 V ±1.5 %, 100 % mode, STOP pin; EMI near mic/T-RH to bench (V-21)" },
    Entry { function: "Patch supervisor", mpn: "TLV840CADL29DBVR", status: Approval::Approved, note: "2.9 V ±2 %, open-drain active-low, valid from 0.7 V" },
    Entry { function: "Start-up BLOCK latch", mpn: "SN74LVC1G74DCUR", status: Approval::Approved, note: "PRE from RESET_N, cleared by PATCH_LATCH" },
    Entry { function: "OE gate / PRI gate", mpn: "SN74LVC1G32DBVR ×2", status: Approval::Approved, note: "Ioff, 5.5 V-tolerant inputs" },
    Entry { function: "Window NAND", mpn: "SN74LVC1G00DBVR", status: Approval::Approved, note: "WIN_BAD" },
    Entry { function: "Arm delay Schmitt", mpn: "SN74LVC1G17DBVR", status: Approval::Approved, note: "VT+ 1.48-1.92 V at 3 V" },
    Entry { function: "Window comparator", mpn: "TLV7032DGKR", status: Approval::Approved, note: "push-pull, VIO ±8 mV, hysteresis 2-17 mV, IB 2 pA, VCM to VCC + 0.1 V" },
    Entry { function: "Arm source switch", mpn: "TMUX1101DBVR", status: Approval::Approved, note: "replaces the unverifiable diode; IS(OFF) ±0.3 nA (85 °C), RON ≤ 4.5 Ω" },
    Entry { function: "AUX decoders", mpn: "SN74LVC138APWR ×2", status: Approval::Approved, note: "1.65-3.6 V characterised; replace CD74HC238/CD74HC138" },
    Entry { function: "AUX bus isolator", mpn: "TMUX136RSER", status: Approval::Conditional, note: "verified (2 pF, power-off isolation, tZH 100 µs, tHZ 200 ns, IOZ ±2 µA); full-chain I²C bench V-11" },
    Entry { function: "AUX port mux", mpn: "TMUX1109PWR", status: Approval::Conditional, note: "RON ≤ 9.75 Ω at 3.3 V, 38 pF behind isolator; bench V-11" },
    Entry { function: "AUX port switch", mpn: "TPS2552DBVR-1 ×4", status: Approval::Approved, note: "active-low EN (LVC138A outputs); R_ILIM 93.1 k → 248-329 mA; latch-off; ton ≤ 3 ms" },
    Entry { function: "AUX ESD", mpn: "TPD4E05U06DQAR ×4", status: Approval::Approved, note: "orderable Active; D1+ SDA, D1- SCL, D2+ VCC" },
    Entry { function: "AUX header", mpn: "JST B4B-PH-K-S(LF)(SN) ×4", status: Approval::Conditional, note: "2 A, 100 V, -40..105 °C, keyed; cable assembly (PHR-4 + SPH crimp) and enclosure fit to verify; mating housing datasheet not obtained" },
    Entry { function: "T/RH sensor", mpn: "SHT45-AD1B-R2", status: Approval::Conditional, note: "confirm 0x44 on order; membrane variant for hive" },
    Entry { function: "I²S microphone", mpn: "PUI DMM-4026-B-I2S-R", status: Approval::Conditional, note: "lifecycle/availability unconfirmed; BCLK ≥ 2.048 MHz; SD load ≤ 140 pF needs TMUX1511 and ≤ 60 pF/m FFC; full-chain timing V-11" },
    Entry { function: "Patch connectors", mpn: "Hirose FH34SRJ-12S-0.5SH(50) ×2", status: Approval::Conditional, note: "as main board" },
    // ---- not approved / rejected
    Entry { function: "Rev C power decoder", mpn: "CD74HC238PWR", status: Approval::NotApproved, note: "HC not specified at 3.3 V (client rule)" },
    Entry { function: "Rev C bus decoder", mpn: "CD74HC138M96", status: Approval::NotApproved, note: "HC not specified at 3.3 V (client rule)" },
    Entry { function: "Rev C primary switch", mpn: "CD74HC4066PWR", status: Approval::Rejected, note: "ID window fails at its 85 °C leakage; no power-off protection" },
    Entry { function: "Rev C shift register", mpn: "SN74HC595PWR", status: Approval::Rejected, note: "superseded by SN74LV595A (client preference); no Ioff" },
    Entry { function: "Rev C arm-sense diode", mpn: "unselected", status: Approval::Rejected, note: "no datasheet obtainable; replaced by TMUX1101" },
    Entry { function: "Rev C/D port switch", mpn: "TPS2553DBVR-1 (ports)", status: Approval::Rejected, note: "active-high EN incompatible with LVC138A outputs; TPS2552DBVR-1 (same datasheet, active-low) used" },
    Entry { function: "Earlier candidates", mpn: "ICS-43434, TPS62162, TCA/PCA9546A, CD74HC4052, TMUX1209, SN74CB3Q3253", status: Approval::Rejected, note: "EOL / topology / capacitance (see eng_verification::REJECTED)" },
];

/// Firmware-memory check: seconds of mono audio that fit in `psram_bytes` at `fs` and `bytes_per_sample`,
/// keeping `reserve` bytes for heap/Wi-Fi buffers.
pub fn psram_audio_seconds(psram_bytes: f64, reserve: f64, fs: f64, bytes_per_sample: f64) -> f64 {
    (psram_bytes - reserve) / (fs * bytes_per_sample)
}

/// One GPIO of the candidate map.
pub type Gpio = (u8, &'static str, bool);

/// Candidate GPIO map for the release baseline (`true` = needs ADC1). Supersedes
/// [`crate::main_board::GPIO_MAP`] only if A-01/A-02 are approved.
pub const GPIO_MAP_CANDIDATE: &[Gpio] = &[
    (0, "BOOT", false),
    (1, "SENSOR_DATA_ID", true),
    (2, "VBAT_SENSE", true),
    (4, "SENSOR_CLK", false),
    (5, "SENSOR_SYNC", false),
    (6, "AUX_ARM_SENSE", true),
    (7, "CC1_SENSE", true),
    (8, "CC2_SENSE", true),
    (9, "VCHAIN_SENSE (100 k / 56.2 k)", true),
    (11, "PATCH_SHIFT_DATA", false),
    (12, "PATCH_SHIFT_CLK", false),
    (13, "PATCH_LATCH", false),
    (14, "PATCH_OE_N (1 kΩ series)", false),
    (15, "CHAIN_PWR_EN", false),
    (16, "BOOST_EN", false),
    (17, "CHG_EN1 (open drain, 100 k to OUT)", false),
    (18, "CHG_EN2", false),
    (19, "USB_D-", false),
    (20, "USB_D+", false),
    (21, "CHG_STAT_N", false),
    (35, "CHG_PGOOD_N", false),
    (36, "REG_MODE (TPS63802 forced PWM)", false),
    (37, "CHAIN_FAULT_N", false),
    (38, "SENSOR_PU_EN", false),
    (39, "LED_R", false),
    (40, "LED_G", false),
    (41, "LED_B", false),
    (42, "DISP_PWR_EN", false),
    (43, "UART0_TX", false),
    (44, "UART0_RX", false),
    (47, "SYS_SDA", false),
    (48, "SYS_SCL", false),
];

/// Review of rev C and of the parallel revision-D files (findings, not edits).
pub const REVIEW_FINDINGS: &[(&str, &str)] = &[
    ("RC-1", "Rev C cross-patch exclusion is firmware + ADC only; not a hardware proof → global_interlock (pin 12)."),
    ("RC-2", "Rev C AUX-armed sense shares pin 7 with ID and I²S SD; a microphone can corrupt it → moved to pin 12."),
    ("RC-3", "Rev C decoders CD74HC238/CD74HC138 are not specified at 3.3 V → SN74LVC138A ×2 (and TPS2552-1 for active-low enables)."),
    ("RC-4", "Rev C pull-downs '100 k class' reach the TPS22918 VIL at IOZ max → 47 kΩ."),
    ("RD-1", "power.rs / interfaces.rs (parallel session) mark +5V_CHAIN and the patch buck as Status::Approved; per the client instruction pin 2 is NOT frozen. Treated here as Proposed (A-02)."),
    ("RD-2", "interfaces.rs rev D still assigns the AUX-armed sense to pin 7 (conflict B unresolved there)."),
    ("RD-3", "power.rs lists the microphone current as unverified; the PUI datasheet is archived (0.82-1.0 mA)."),
    ("RD-4", "Rev D chain limit 76.8 kΩ is confirmed (302-394 mA incl. resistor tolerance); TPS61023 true disconnect confirmed in its datasheet."),
    ("RD-5", "Rev D patch PATCH_OE_N pull-up to +3V3_LOCAL agrees with startup_safety; the BLOCK latch and supervisor are added here."),
];

/// Decision history (append only).
pub const HISTORY: &[(&str, &str, &str)] = &[
    ("2026-11-01", "Client", "Four I²C AUX headers per patch, pinout VCC-GND-SDA-SCL; single active path; no 5 V to ESP32."),
    ("2026-11-01", "Engineering (other session)", "Rev C AUX architecture: TMUX136 + TMUX1109 + HC238/HC138 + TPS2553 + pin-7 arm sense (proposal)."),
    ("2026-11-02", "Engineering (this terminal)", "Component phase: V1 audit, datasheet-verified BOM, passives, power budget (eng_verification)."),
    ("2026-11-02", "Engineering (other session)", "Revision D power files: +5V_CHAIN, TPS61023, TPS62840, TPS63802 (found during this review)."),
    ("2026-11-02", "Client", "Directions: N16R2 conditional, LV595A and TMUX1511 preferred, HC238/138 not approved, 5 V chain preferred but not frozen, 3000 mAh battery confirmed, 400 mA contact ceiling."),
    ("2026-11-02", "Engineering (this terminal)", "Release review: pin-12 AUX_ARM hardware interlock, BLOCK latch start-up, 5 V chain verified, BQ24074 RISET 1.27 k, USB-C CC policy."),
];

/// Decisions that need client approval.
pub const APPROVALS_REQUIRED: &[(&str, &str, &str)] = &[
    ("A-01", "Convert pin 12 from GND to AUX_ARM (3 grounds remain).", "Only route found to a hardware-proven global one-of-32 interlock without sacrificing pin 7 (ID / I²S SD)."),
    ("A-02", "Freeze pin 2 as +5V_CHAIN with per-patch TPS62840 3.3 V.", "All calculations pass: SCD41-class at every patch, ≤ 394 mA, inrush 2.5 ms, thermal OK; cost: ~16 % more AUX energy than a 3.3 V chain."),
    ("A-03", "Audio capture length / rate → N16R2 (≤ 15 s clips at 32 kHz/24-bit) or N16R8 with PSRAM ECC.", "Memory sizing depends on the audio requirement."),
    ("A-04", "Accept the added patch logic (supervisor, latch, window comparator, 5 small gates).", "Price of hardware-guaranteed OFF states and global exclusion."),
    ("A-05", "Battery pack MPN + datasheet meeting battery_charging requirements; charge current 0.70 A.", "3000 mAh charge 5-6 h from a ≥ 1.5 A Type-C source, slower (timer-extended) from USB500."),
    ("A-06", "Fuel gauge: drop MAX17048 (unverifiable) and use the VBAT ADC divider, or supply an alternative gauge to verify.", "No datasheet could be obtained."),
    ("A-07", "Microphone sample rate ≥ 32 kHz (PUI BCLK ≥ 2.048 MHz) or nominate another current I²S mic.", "Lifecycle and full-chain timing still open."),
    ("A-08", "FFC specification (type A/D, length ≤ 0.1 m/segment, ≤ 60 pF/m for the mic SD load).", "Drop, capacitance and mirroring depend on it."),
    ("A-09", "Display module and connector (JST SH, source pin order).", "Display current/address unknown."),
];

/// Bench tests required before schematic release of the affected blocks.
pub const BENCH_TESTS: &[(&str, &str)] = &[
    ("V-02", "Shift chain at patch 8: SRCLK/RCLK/OE edges, no double clocking, BLOCK latch behaviour after brownout."),
    ("V-06", "FH34 + FFC contact and conductor resistance; patch-8 VIN at 225 mA and at the 394 mA limit."),
    ("V-11", "I²S 2.048 MHz SD timing and ≤ 140 pF load; I²C 100 kHz through TMUX1109 + TMUX136 at patch 8."),
    ("V-15", "AUX_ARM window: n = 0/1/2 armed with forced ground offsets up to 0.42 V; double-arm drop-out time."),
    ("V-16", "FH34 temperature rise at 0.394 A continuous."),
    ("V-17", "Start-up inrush of 8 patches; TPS2553-1 must not latch."),
    ("V-18", "USB-C: default / 1.5 A / 3.0 A sources → CC readings and selected input limit."),
    ("V-19", "Battery: runtime against the typical and worst scenarios; peak current with Wi-Fi TX."),
    ("V-20", "BQ24074 temperature at 0.78 A charge, 5.25 V input, 40 °C ambient."),
    ("V-21", "TPS62840 noise on mic/SHT45 (STOP pin use during captures)."),
    ("V-03/V-08", "ID windows on the local 3.3 V rail; I²S DIN on GPIO1 of the N16R2."),
];

/// Unresolved risks.
pub const RISKS: &[(&str, &str)] = &[
    ("RK-1", "Patch component count grows (≈ 20 ICs); layout area and cost vs OQ-018."),
    ("RK-2", "Ground offset 0.42 V at patch 8 at the chain limit erodes I²C low-level margin; transfers must avoid module peaks."),
    ("RK-3", "FFC mirrored insertion puts 5 V on PATCH_OE_N; only mechanical keying fully prevents it."),
    ("RK-4", "Microphone single-source, lifecycle unknown."),
    ("RK-5", "Several datasheets unobtainable (MAX17048, USBLC6, small diodes, JST housings); affected parts not approved."),
    ("RK-6", "Two sessions edited this crate concurrently; power.rs/interfaces.rs contain statuses inconsistent with the client instruction (RD-1, RD-2)."),
];

/// Additional evidence archived in this review (`bom/datasheets/`).
pub const NEW_EVIDENCE: &[&str] = &[
    "TI_TLV7031_TLV7032_SLVSE13J.pdf",
    "TI_TMUX1101_TMUX1102_SCDS410D.pdf",
    "TI_TLV840_SNVSBC3C.pdf",
    "TI_SN74LVC1G32.pdf",
    "TI_SN74LVC1G00.pdf",
    "TI_SN74LVC1G74.pdf",
    "TI_SN74LVC1G17.pdf",
    "TI_TPD2E2U06_SLLSEG9C.pdf",
    "JST_SH_connector_catalog.pdf",
    "TI_TPS61023.pdf",
    "TI_TPS62840.pdf",
    "TI_TPS63802.pdf",
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn evidence_files_exist() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../bom/datasheets");
        for f in NEW_EVIDENCE {
            assert!(dir.join(f).exists(), "{f}");
        }
    }

    #[test]
    fn client_rules_reflected() {
        let st = |m: &str| {
            REGISTER
                .iter()
                .find(|e| e.mpn.starts_with(m))
                .unwrap_or_else(|| panic!("{m}"))
                .status
        };
        assert_ne!(st("CD74HC238"), Approval::Approved);
        assert_ne!(st("CD74HC138"), Approval::Approved);
        assert_eq!(st("SN74LV595A"), Approval::Approved);
        assert_eq!(st("ESP32-S3-WROOM-1-N16R2"), Approval::Conditional);
        assert_eq!(st("MAX17048"), Approval::NotApproved);
        assert_eq!(st("USBLC6"), Approval::NotApproved);
        assert!(REGISTER
            .iter()
            .all(|e| !(e.status == Approval::Approved
                && e.note.to_lowercase().contains("not obtained"))));
    }

    #[test]
    fn psram_sizing() {
        let n16r2 = psram_audio_seconds(2.0 * 1024.0 * 1024.0, 512.0 * 1024.0, 32_000.0, 3.0);
        let n16r8_ecc = psram_audio_seconds(
            8.0 * 1024.0 * 1024.0 * 15.0 / 16.0,
            512.0 * 1024.0,
            32_000.0,
            3.0,
        );
        assert!(n16r2 > 15.0 && n16r2 < 17.0, "{n16r2}");
        assert!(n16r8_ecc > 70.0);
    }

    #[test]
    fn gpio_candidate_rules() {
        let mut s = HashSet::new();
        for (g, n, adc) in GPIO_MAP_CANDIDATE {
            assert!(s.insert(*g), "dup {g}");
            assert!(![3u8, 45, 46].contains(g), "strapping {g}");
            assert!(!(22..=34).contains(g));
            if *adc {
                assert!((1..=10).contains(g), "{n} needs ADC1");
            }
        }
        assert!(!GPIO_MAP_CANDIDATE
            .iter()
            .any(|(g, n, _)| *g == 38 && n.contains("ID")));
    }

    #[test]
    fn ids_unique() {
        let mut s = HashSet::new();
        for (i, _, _) in APPROVALS_REQUIRED {
            assert!(s.insert(*i));
        }
        for (i, _) in BENCH_TESTS {
            assert!(s.insert(*i));
        }
        for (i, _) in RISKS {
            assert!(s.insert(*i));
        }
        for (i, _) in REVIEW_FINDINGS {
            assert!(s.insert(*i));
        }
    }
}

#[cfg(test)]
mod report {
    /// `cargo test --lib release_review::report -- --nocapture` prints the computed review values.
    #[test]
    fn print_review_values() {
        use crate::chain_power_5v::*;
        let c = worst();
        println!("boost {:?}", boost_vout());
        println!(
            "chain lim {:?}",
            crate::passives::tps255x_window_ma(R_ILIM_CHAIN_5V_K)
        );
        let o = 7.0 * patch_input_w(0.0);
        for k in [1, 4, 8] {
            println!(
                "scd41 p{k} {:?}",
                c.operating_point(k, patch_input_w(225.0) + o)
            );
        }
        println!(
            "max local {:?}",
            (1..=8)
                .map(|k| max_local_load_ma(&c, k, 302.0).round())
                .collect::<Vec<_>>()
        );
        println!(
            "inrush ms {}",
            startup_charge_uc(8, 10.0, 10.0, 10.0, boost_vout().1)
                / crate::passives::tps255x_window_ma(R_ILIM_CHAIN_5V_K).0
        );
        println!(
            "gnd off {}",
            crate::passives::tps255x_window_ma(R_ILIM_CHAIN_5V_K).1 / 1000.0 * c.r_return(8)
        );
        println!(
            "q7d {:?} settle {}",
            crate::global_interlock::q7d_delay_ms(),
            crate::global_interlock::window_settle_ms()
        );
        println!(
            "thr {:?}",
            crate::global_interlock::thresholds(3.3, (0.0, 0.0, 0.0))
        );
        for s in crate::battery_charging::SCENARIOS.iter() {
            println!(
                "{} {:.2} mA {:.1} d",
                s.name,
                s.average_ma(),
                s.runtime_days()
            );
        }
        println!(
            "peak {} {}",
            crate::battery_charging::battery_peak_a(3.0, true),
            crate::battery_charging::battery_peak_a(3.0, false)
        );
        println!("ichg {:?}", crate::battery_charging::ichg(1270.0));
        println!("port {:?}", crate::passives::tps255x_window_ma(93.1));
    }
}
