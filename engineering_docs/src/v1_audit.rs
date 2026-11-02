//! # V1 component audit and V1-to-V2 compatibility matrix
//!
//! Source: `NijoP/BeeBox` cloned 2026-11-01 (`bom/consolidated_bom.csv`, `netlist/components.csv`,
//! `netlist/gpio_map.csv`, `netlist/connector_pinout.csv`, docs 03, 05, 18, 19, 23, 24,
//! `schematic/footprint_verification.md`, `easyeda_scripts/`).
//!
//! ## Findings about the V1 repository itself
//! * V1 was a **single controller + one docked FPC** architecture (12-contact dock: 3 GND, 2 VDOCK,
//!   +3V3_FPC, I²C, I²S, PRESENT/ID). It has no shift-register cascade, no patch ID windows and no AUX
//!   ports, so its 12-pin pinout is **incompatible** with the V2 ICD and is replaced everywhere (p.7).
//! * V1 GPIO38 carried DOCK_ID (ADC). V2 rejects GPIO38 for ID (REQ-MB-08).
//! * V1 BOM line J1 says `TYPE-C-31-M-12` while V1 doc 03 names `USB4105-GF-A`: internal inconsistency.
//! * V1 MCU `ESP32-S3-WROOM-1-N16R8` is rated −40…65 °C ambient and uses GPIO35–37 for octal PSRAM
//!   (DS-ESP-MOD §1, pin notes). Hive enclosures in sun can exceed this.
//! * V1 EasyEDA scripts searched the live library by MPN token at run time (`eda.lib_Device.search`) and
//!   kept `PASSIVE_LIBRARY = {}` empty: **no reviewed EasyEDA symbol/footprint UUIDs exist to reuse**.
//!   V1 `footprint_verification.md` lists all footprints except the ESP32 module as unverified.
//! * V1 calculations assumed 0.24 A/0.43 A ESP32 currents and 2.2 kΩ pull-ups at 200 pF; these were
//!   assumptions, not datasheet values, and are recomputed in [`crate::passives`].
//!
//! Classification follows the client categories ([`Compat`]). "Replace" with `v2: None` means the
//! function is not part of the confirmed V2 scope and is removed.

use crate::component_model::Compat;

/// One audited V1 BOM line.
#[derive(Debug, Clone, Copy)]
pub struct V1Item {
    /// V1 reference designator(s).
    pub v1_ref: &'static str,
    /// V1 manufacturer part / specification as written in V1.
    pub v1_part: &'static str,
    /// V1 function.
    pub function: &'static str,
    /// Classification.
    pub compat: Compat,
    /// V2 selection (`None` = removed from V2 scope).
    pub v2: Option<&'static str>,
    /// Reason for the classification.
    pub reason: &'static str,
    /// Supporting datasheet ids ([`crate::datasheets`]).
    pub evidence: &'static [&'static str],
}

/// Complete V1 component audit (every line of V1 `consolidated_bom.csv`).
pub const V1_ITEMS: &[V1Item] = &[
    V1Item { v1_ref: "U1", v1_part: "Espressif ESP32-S3-WROOM-1-N16R8", function: "MCU / Wi-Fi / BLE", compat: Compat::Revise, v2: Some("ESP32-S3-WROOM-1-N16R2"), reason: "Same module family and footprint. R8 (octal PSRAM) variants are rated -40..65 °C and consume GPIO35-37; R2 (quad PSRAM) variants are -40..85 °C and keep GPIO35-37 free. 2 MB PSRAM covers ~16 s of 32 kHz x 32-bit mono audio buffering.", evidence: &["DS-ESP-MOD"] },
    V1Item { v1_ref: "U2", v1_part: "TI BQ24074RGTR", function: "1S charger / power path", compat: Compat::Revise, v2: Some("BQ24074RGTR"), reason: "Retained IC (10.5 V OVP, 4.4 V OUT regulation, power path). Programming revised: RTMR 46.4 k -> 68.1 k (V1 timer can expire at KTMR min = 4.6 h), RILIM 1.54 k -> 1.62 k with EN1/EN2 default USB500 mode, TS/ITERM defined.", evidence: &["DS-BQ24074"] },
    V1Item { v1_ref: "U3", v1_part: "TI TPS62162DSG", function: "3.3 V buck", compat: Compat::Replace, v2: Some("TPS63021DSJR"), reason: "A buck cannot hold 3.3 V over the 1S discharge range (source p.12). TPS63021 buck-boost, fixed 3.267-3.333 V (PWM), 1.8-5.5 V input, 2 A at VIN > 2.5 V.", evidence: &["DS-TPS62162", "DS-TPS6302X"] },
    V1Item { v1_ref: "U4", v1_part: "TI TCA9546APWR", function: "I²C branch switch", compat: Compat::Replace, v2: Some("TMUX1109 + TMUX136 on each patch, 595-controlled (rev C)"), reason: "V2 selects patches through the 74xx595 cascade, not I²C addresses. A PCA/TCA954x reserves 0x70-0x77, needs a healthy shared bus to deselect and gives no hardware single-port interlock (rejected in rev C).", evidence: &["DS-PCA9546A", "DS-TMUX1109", "DS-TMUX136"] },
    V1Item { v1_ref: "U5", v1_part: "ADI MAX17048G+T10 (DNI option)", function: "Fuel gauge", compat: Compat::Revise, v2: Some("MAX17048G+T10 (fitted, on display/system I²C bus)"), reason: "REQ-MB-09 requires battery telemetry; moved from option to fitted. Datasheet could not be re-downloaded in this phase -> Unverified until re-checked.", evidence: &["DS-MAX17048"] },
    V1Item { v1_ref: "U6", v1_part: "TI TPS26600RHFT", function: "~6 V external-input eFuse", compat: Compat::Replace, v2: None, reason: "V2 source defines USB/1S power only; no 6 V/solar input requirement exists. BQ24074 already tolerates 26 V without damage and has 10.5 V OVP. Re-add only if the client confirms a solar/DC input (decision D-13).", evidence: &["DS-BQ24074"] },
    V1Item { v1_ref: "U11", v1_part: "TI TPS2121RUXR", function: "USB / external source power mux", compat: Compat::Replace, v2: None, reason: "Only one input source in V2 baseline; no mux needed (decision D-13).", evidence: &[] },
    V1Item { v1_ref: "U12", v1_part: "TI SN74LVC1G3157DBVR", function: "I²S mic data-source switch", compat: Compat::Replace, v2: None, reason: "V2 has one I²S path (chain pin 7) selected by patch switches; no local/external mic selection on the main board.", evidence: &[] },
    V1Item { v1_ref: "Q1", v1_part: "TI TPS22919DCK", function: "VDOCK load switch", compat: Compat::Replace, v2: Some("TPS2553DBVR-1 (+5V_CHAIN current-limited switch, rev D)"), reason: "The chain needs a programmable current limit below the FH34 0.5 A contact rating plus fault latch-off; TPS22919 has no current limit.", evidence: &["DS-TPS255X", "DS-FH34-CAT"] },
    V1Item { v1_ref: "D6", v1_part: "ST USBLC6-2SC6", function: "USB data ESD", compat: Compat::Retain, v2: Some("USBLC6-2SC6"), reason: "Function unchanged (USB D+/D- on GPIO19/20). Datasheet re-download failed: footprint/pinout carried from V1, re-verify before release.", evidence: &["DS-USBLC6"] },
    V1Item { v1_ref: "D5", v1_part: "TI TPD4E05U06", function: "Dock ESD", compat: Compat::Revise, v2: Some("TPD4E05U06DQAR (2x main chain, 4x per patch AUX)"), reason: "0.5 pF lines, VRWM 5.5 V suit 3.3 V I²C/I²S. Fills the rev C 'ESD array (TBD)' for the user-exposed AUX headers.", evidence: &["DS-TPD4E05U06"] },
    V1Item { v1_ref: "J1", v1_part: "TYPE-C-31-M-12 (doc 03 says GCT USB4105-GF-A)", function: "USB-C receptacle", compat: Compat::Revise, v2: Some("GCT USB4105-GF-A"), reason: "Resolves V1 inconsistency in favour of the drawing that was obtained: 5 A VBUS collectively, 48 V, 20 000 cycles.", evidence: &["DS-GCT-USB4105"] },
    V1Item { v1_ref: "J2", v1_part: "Hirose FH34SRJ-12S-0.5SH(50)", function: "12-pos 0.5 mm ZIF", compat: Compat::Revise, v2: Some("FH34SRJ-12S-0.5SH(50) (main x1, patch x2)"), reason: "Same connector, new V2 pinout. Verified: 0.5 A rated (70 % when all contacts carry current), 100 mΩ max contact resistance incl. FPC bulk, 20 mating cycles, top & bottom contact, 0.3 mm FPC, -55..105 °C.", evidence: &["DS-FH34-CAT", "DS-FH34-SPEC"] },
    V1Item { v1_ref: "J3", v1_part: "Mill-Max 0906-0-15-20-75-14-11-0 x12", function: "Pogo dock contacts", compat: Compat::Revise, v2: Some("Pogo variant only (same logical 12-pin ICD); not in FPC baseline BOM"), reason: "REQ-SYS-07 keeps pogo as alternative build. Drawing not re-verified; current rating vs +3V3_CHAIN budget open (OQ-025).", evidence: &[] },
    V1Item { v1_ref: "J4", v1_part: "JST S2B-PH-SM4-TB", function: "Battery connector", compat: Compat::Retain, v2: Some("JST S2B-PH-SM4-TB"), reason: "PH series verified: 2 A (AWG24), 100 V, -40..105 °C, 10 mΩ initial contact resistance. Peak battery current ~1.3 A < 2 A.", evidence: &["DS-JST-PH"] },
    V1Item { v1_ref: "BT1", v1_part: "Protected 1S 3.7 V 2000 mAh (Indian AVL TBD)", function: "Battery", compat: Compat::Revise, v2: Some("Protected 1S LiPo >= 2000 mAh, >= 1.5 A continuous, 10 k NTC, JST-PH lead"), reason: "Spec retained; NTC added for BQ24074 TS; continuous rating raised to cover 1.3 A worst-case input of TPS63021. Exact cell MPN is a client decision (D-11).", evidence: &["DS-BQ24074", "DS-TPS6302X"] },
    V1Item { v1_ref: "MIC1, MIC2", v1_part: "TDK InvenSense ICS-43434", function: "I²S microphone", compat: Compat::Replace, v2: Some("PUI Audio DMM-4026-B-I2S-R (patch mic variant)"), reason: "ICS-43434 is EOL per source p.15. PUI part: I²S 24/32-bit, 1.62-3.63 V, 0.82-1.0 mA, BCLK 2.048-4.096 MHz, 140 pF max load, -40..100 °C.", evidence: &["DS-PUI-MIC"] },
    V1Item { v1_ref: "U7, U8", v1_part: "Sensirion SHT45", function: "T/RH", compat: Compat::Retain, v2: Some("SHT45-AD1B-R2 (patch T/RH variant; AD1F membrane option)"), reason: "1.08-3.6 V, I²C 0x44, Rp >= 390 Ω, Cb <= 400 pF; DFN 1.5x1.5 mm. Moves from main/FPC to the patch T/RH variant.", evidence: &["DS-SHT4X"] },
    V1Item { v1_ref: "U9, U-IMU", v1_part: "TDK ICM-42688-P", function: "IMU", compat: Compat::Replace, v2: None, reason: "Not in V2 confirmed scope (one primary sensor type per patch: T/RH or mic).", evidence: &[] },
    V1Item { v1_ref: "U10", v1_part: "Sensirion SGP40 (DNI)", function: "VOC option", compat: Compat::Replace, v2: None, reason: "Not a V2 primary sensor; may return as an approved AUX I²C module profile.", evidence: &[] },
    V1Item { v1_ref: "D2-D4", v1_part: "Lite-On LTST-C193KRKT/KGKT/KBKT-5A", function: "Status LEDs", compat: Compat::Revise, v2: Some("Status LEDs (MPN retained, Vf/current to re-verify)"), reason: "Function kept (OQ-023). Lite-On datasheets not reviewed in this phase; R value recalculated after Vf check.", evidence: &[] },
    V1Item { v1_ref: "J-SD", v1_part: "microSD socket (TBD, DNI)", function: "Storage", compat: Compat::Replace, v2: None, reason: "Data storage not defined for V2 (CD-4); 16 MB flash + PSRAM on module.", evidence: &[] },
    V1Item { v1_ref: "J-RF", v1_part: "RF expansion header (TBD, DNI)", function: "SPI/UART expansion", compat: Compat::Replace, v2: None, reason: "Not in V2 scope; AUX is I²C-only on patches.", evidence: &[] },
    V1Item { v1_ref: "R1-R3", v1_part: "1.00 k 1 %", function: "LED resistors", compat: Compat::Revise, v2: Some("1.00 k (recheck after LED Vf)"), reason: "Value plausible (1.3 mA red); blue Vf unverified.", evidence: &[] },
    V1Item { v1_ref: "R4-R5", v1_part: "2.20 k 1 %", function: "Main I²C pull-ups", compat: Compat::Revise, v2: Some("2.2 k on display bus; 2.2 k GPIO-switched sensor-bus pull-ups (primary mode); 2.2 k per AUX port to port VCC"), reason: "Static pull-ups on the shared chain fight I²S push-pull and back-feed unpowered modules (rev C PULL_UP_RULES); value re-derived for ≈ 257 pF.", evidence: &["DS-UM10204", "DS-SHT4X"] },
    V1Item { v1_ref: "R6-R7", v1_part: "5.10 k 1 %", function: "USB-C CC Rd", compat: Compat::Retain, v2: Some("5.1 k 1 %"), reason: "Type-C sink Rd value; unchanged.", evidence: &["DS-GCT-USB4105"] },
    V1Item { v1_ref: "C21-C22", v1_part: "10 µF X7R 10 V / 10 nF X7R 25 V", function: "VBUS bypass", compat: Compat::Revise, v2: Some("1 µF X7R 25 V at BQ24074 IN (+10 nF)"), reason: "BQ24074 IN requires 1-10 µF; 25 V rating chosen because IN survives 26 V faults.", evidence: &["DS-BQ24074"] },
    V1Item { v1_ref: "R8-R9", v1_part: "1.0 M / 330 k 1 %", function: "Battery divider", compat: Compat::Retain, v2: Some("1.0 M / 330 k 1 % + 100 nF, ADC1 (GPIO2), ATTEN2"), reason: "4.23 V -> 1.065 V max, within ATTEN2 0-1600 mV range; 3.2 µA drain. 100 nF added (Espressif ADC measurement condition).", evidence: &["DS-ESP-SOC"] },
    V1Item { v1_ref: "R10", v1_part: "100 k 1 % dock ID", function: "FPC ID", compat: Compat::Replace, v2: Some("Patch R_ID 22.0 k / 68.0 k + main 100 k pull-down"), reason: "V2 ID model (p.11) replaces the V1 dock ID.", evidence: &["DS-ESP-SOC"] },
    V1Item { v1_ref: "R11", v1_part: "1.78 k 1 %", function: "BQ24074 ISET", compat: Compat::Retain, v2: Some("1.78 k 1 %"), reason: "ICHG = 890/1780 = 0.50 A nominal (0.44-0.55 A worst case) = 0.25 C for 2000 mAh.", evidence: &["DS-BQ24074"] },
    V1Item { v1_ref: "R12", v1_part: "1.54 k 1 %", function: "BQ24074 ILIM", compat: Compat::Revise, v2: Some("1.62 k 1 % (used only in ILIM mode)"), reason: "Nominal 0.99 A (0.92-1.07 A). Default boot mode is USB500 via EN1/EN2 pulls so an unknown USB source is never overloaded.", evidence: &["DS-BQ24074"] },
    V1Item { v1_ref: "R13", v1_part: "46.4 k 1 %", function: "BQ24074 timer", compat: Compat::Revise, v2: Some("68.1 k 1 %"), reason: "46.4 k gives 4.6 h at KTMR min; a 2000 mAh cell at 0.44 A worst-case needs > 4.5 h CC + CV. 68.1 k: 6.7-11.5 h.", evidence: &["DS-BQ24074"] },
    V1Item { v1_ref: "R14-R15", v1_part: "316 k / 100 k", function: "TPS62162 feedback", compat: Compat::Replace, v2: None, reason: "TPS63021 is fixed 3.3 V; FB ties to VOUT.", evidence: &["DS-TPS6302X"] },
    V1Item { v1_ref: "C1-C20", v1_part: "100 nF / 1 µF / 22 µF X7R", function: "Decoupling / bulk", compat: Compat::Revise, v2: Some("Per-IC values in crate::passives::DECOUPLING"), reason: "Re-derived from each V2 datasheet (TPS63021 2x10 µF in / 3x22 µF out, BQ24074 OUT/BAT 4.7-47 µF, ESP32 >= 10 µF at power entry, etc.).", evidence: &["DS-TPS6302X", "DS-BQ24074", "DS-ESP-HWDG"] },
    V1Item { v1_ref: "L1", v1_part: "2.2 µH shielded >= 1.5 A", function: "Buck inductor", compat: Compat::Replace, v2: Some("1.5 µH shielded, Isat >= 2.5 A, low DCR (MPN pending)"), reason: "TPS63021 typical application uses 1.5 µH; Ipeak 1.34 A at 1 A out (2.9 V in, -30 % L) and 1.97 A at 1.5 A.", evidence: &["DS-TPS6302X"] },
    V1Item { v1_ref: "F1", v1_part: "Bourns MF-MSMF050-2", function: "Input PTC", compat: Compat::Revise, v2: Some("Optional DNI footprint"), reason: "BQ24074 provides OVP and input current limit; PTC datasheet not reviewed. Kept as DNI footprint.", evidence: &["DS-BQ24074"] },
    V1Item { v1_ref: "TP1-TP12", v1_part: "Test pads", function: "Bring-up", compat: Compat::Revise, v2: Some("Test pads per crate::pinmap::TEST_POINTS"), reason: "Re-allocated to V2 nets (chain rail, control, ID, AUX VCC, faults).", evidence: &[] },
];

/// V2 functions that did not exist in V1 (classification [`Compat::NewForV2`]).
pub const NEW_FOR_V2: &[(&str, &str, &str)] = &[
    (
        "Patch shift register",
        "SN74HC595PWR (alt SN74LV595APWR, D-02)",
        "64-bit cascade control (8 patches x 8 bits)",
    ),
    (
        "Patch primary signal switch",
        "CD74HC4066PWR (alt TMUX1511PWR, D-03)",
        "3 sensor lines + ID branch",
    ),
    (
        "Patch primary sensor power",
        "TPS22918DBVR",
        "default-OFF switched PRI_VDD with QOD discharge",
    ),
    (
        "Patch ID resistors",
        "22.0 k / 68.0 k 1 %",
        "T/RH vs mic identification on SENSOR_DATA_ID",
    ),
    (
        "AUX bus isolator",
        "TMUX136RSER",
        "2 pF off-capacitance per patch on chain pins 4/5, power-off isolation",
    ),
    ("AUX port mux", "TMUX1109PWR", "dual 4:1 SDA/SCL selection"),
    (
        "AUX decoders",
        "CD74HC238PWR + CD74HC138M96",
        "port power enable / bus connect with primary interlock",
    ),
    (
        "AUX port power",
        "TPS2553DBVR-1 x4 (93.1 k)",
        "248-329 mA per port, latch-off",
    ),
    (
        "AUX-armed sense",
        "low-leakage diode + 150 k",
        "cross-patch single-path detection on pin 7",
    ),
    (
        "AUX headers",
        "JST B4B-PH-K-S(LF)(SN) x4",
        "VCC-GND-SDA-SCL, keyed 2.0 mm (APR-2)",
    ),
    ("AUX ESD", "TPD4E05U06DQAR x4", "VCC/SDA/SCL per header"),
    (
        "Chain power switch",
        "TPS2553DBVR-1 (76.8 k)",
        "+5V_CHAIN <= 400 mA, latch, GPIO enable (rev D)",
    ),
    (
        "Chain boost (new in rev D)",
        "TPS61023DRLR",
        "+5V0 from SYS for pin 2",
    ),
    (
        "Patch regulator (new in rev D)",
        "TPS62840DLCR",
        "+3V3_LOCAL on every patch",
    ),
    (
        "AUX connector (rev D, APR-2)",
        "JST GH SM04B-GHS-TB (MPN to verify)",
        "replaces JST PH B4B-PH-K-S",
    ),
    (
        "Display power switch",
        "TPS22918DBVR",
        "independent +3V3_DISPLAY",
    ),
];

/// Summary counts by classification.
pub fn count(c: Compat) -> usize {
    V1_ITEMS.iter().filter(|i| i.compat == c).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_v1_bom_line_is_classified() {
        // V1 consolidated_bom.csv has 49 lines grouped into these audit rows.
        assert_eq!(V1_ITEMS.len(), 37);
        assert_eq!(
            count(Compat::Retain) + count(Compat::Revise) + count(Compat::Replace),
            V1_ITEMS.len()
        );
        assert!(!NEW_FOR_V2.is_empty());
    }

    #[test]
    fn eol_and_rejected_v1_parts_are_not_retained() {
        for i in V1_ITEMS {
            if i.v1_part.contains("ICS-43434") || i.v1_part.contains("TPS62162") {
                assert_eq!(i.compat, Compat::Replace, "{}", i.v1_ref);
            }
        }
    }

    #[test]
    fn evidence_ids_exist() {
        for i in V1_ITEMS {
            for e in i.evidence {
                assert!(
                    crate::datasheets::find(e).is_some(),
                    "{} -> {}",
                    i.v1_ref,
                    e
                );
            }
        }
    }
}
