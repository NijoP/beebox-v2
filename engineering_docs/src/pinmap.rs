//! # Pin mapping and footprints
//!
//! Physical pin → V2 net for every IC and connector, transcribed from the manufacturer
//! pin-function tables/drawings in `bom/datasheets/`. EasyEDA symbols/footprints are **not** created
//! in this phase; when they are, each one must be checked pad-by-pad against [`PIN_MAPS`].
//!
//! EasyEDA library status: V1 left no reviewed library UUIDs (`PASSIVE_LIBRARY = {}`), so no V1
//! library item is re-used. Every V2 footprint is [`FootprintStatus::ToCreateFromDrawing`] or
//! [`FootprintStatus::LibraryToVerify`].

use crate::component_model::{Pin, PinVerification};

/// Footprint status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FootprintStatus {
    /// Standard package (TI DBV/PW/DQA/RGT/DSJ etc.): library footprint acceptable after pad check against the TI mechanical drawing.
    LibraryToVerify,
    /// Must be drawn from the manufacturer land pattern (module, sensor, mic, connectors).
    ToCreateFromDrawing,
    /// Drawing not yet obtained.
    DrawingMissing,
}

/// Pin map of one part.
#[derive(Debug, Clone, Copy)]
pub struct PinMap {
    /// Reference designator(s).
    pub refdes: &'static str,
    /// MPN.
    pub mpn: &'static str,
    /// Package.
    pub package: &'static str,
    /// Footprint status.
    pub footprint: FootprintStatus,
    /// Footprint/land-pattern source.
    pub land_pattern: &'static str,
    /// Verification of the pin map.
    pub verified: PinVerification,
    /// Pins.
    pub pins: &'static [Pin],
}

macro_rules! p {
    ($n:expr, $name:expr, $net:expr) => {
        Pin {
            number: $n,
            name: $name,
            net: $net,
        }
    };
}

/// Pin maps.
pub const PIN_MAPS: &[PinMap] = &[
    PinMap {
        refdes: "J_IN, J_OUT (patch); J_CHAIN (main)",
        mpn: "FH34SRJ-12S-0.5SH(50)",
        package: "12-pos 0.5 mm ZIF",
        footprint: FootprintStatus::ToCreateFromDrawing,
        land_pattern:
            "Hirose 2D drawing CL0580-1253-0-50; confirm contact side vs FFC type A/D (OQ-024)",
        verified: PinVerification::DatasheetDrawing,
        pins: &[
            p!("1", "1", "GND"),
            p!("2", "2", "+5V_CHAIN"),
            p!("3", "3", "GND"),
            p!("4", "4", "SENSOR_CLK"),
            p!("5", "5", "SENSOR_SYNC"),
            p!("6", "6", "GND"),
            p!("7", "7", "SENSOR_DATA_ID"),
            p!(
                "8",
                "8",
                "PATCH_SHIFT_DATA (IN: -> U_SR.14 SER; OUT: <- 33R <- U_SR.9 QH')"
            ),
            p!("9", "9", "PATCH_SHIFT_CLK"),
            p!("10", "10", "PATCH_LATCH"),
            p!("11", "11", "PATCH_OE_N"),
            p!("12", "12", "GND"),
            p!("MP", "mount tabs", "GND"),
        ],
    },
    PinMap {
        refdes: "U_SR",
        mpn: "SN74HC595PWR (SN74LV595APWR alt, same pinout)",
        package: "TSSOP-16",
        footprint: FootprintStatus::LibraryToVerify,
        land_pattern: "TI PW0016A",
        verified: PinVerification::DatasheetTable,
        pins: &[
            p!("1", "QB", "PRI_SIG_EN (Q1)"),
            p!("2", "QC", "ID_EN (Q2)"),
            p!("3", "QD", "AUX_A0 (Q3)"),
            p!("4", "QE", "AUX_A1 (Q4)"),
            p!("5", "QF", "AUX_SIG_EN (Q5)"),
            p!("6", "QG", "AUX_PWR_EN (Q6)"),
            p!("7", "QH", "AUX_ARM (Q7)"),
            p!("8", "GND", "GND"),
            p!("9", "QH'", "SHIFT_OUT -> 33R -> J_OUT.8"),
            p!("10", "SRCLR", "+3V3_LOCAL"),
            p!("11", "SRCLK", "PATCH_SHIFT_CLK"),
            p!("12", "RCLK", "PATCH_LATCH"),
            p!("13", "OE", "PATCH_OE_N"),
            p!("14", "SER", "J_IN.8"),
            p!("15", "QA", "PRI_PWR_EN (Q0)"),
            p!("16", "VCC", "+3V3_LOCAL"),
        ],
    },
    PinMap {
        refdes: "U_PRISW",
        mpn: "CD74HC4066PWR",
        package: "TSSOP-14",
        footprint: FootprintStatus::LibraryToVerify,
        land_pattern: "TI PW0014A",
        verified: PinVerification::DatasheetTable,
        pins: &[
            p!("1", "1Y", "SENSOR_CLK"),
            p!("2", "1Z", "PRI_CLK (SHT45 SCL / mic SCK)"),
            p!("3", "2Z", "PRI_SYNC (SHT45 SDA / mic WS)"),
            p!("4", "2Y", "SENSOR_SYNC"),
            p!("5", "2E", "PRI_SIG_EN"),
            p!("6", "3E", "PRI_SIG_EN"),
            p!("7", "GND", "GND"),
            p!("8", "3Y", "SENSOR_DATA_ID"),
            p!("9", "3Z", "PRI_SD (mic SD; NC on T/RH)"),
            p!("10", "4Z", "ID_NODE (R_ID to +3V3_LOCAL)"),
            p!("11", "4Y", "SENSOR_DATA_ID"),
            p!("12", "4E", "ID_EN"),
            p!("13", "1E", "PRI_SIG_EN"),
            p!("14", "VCC", "+3V3_LOCAL"),
        ],
    },
    PinMap {
        refdes: "U_PRISW (alternate D-03)",
        mpn: "TMUX1511PWR",
        package: "TSSOP-14",
        footprint: FootprintStatus::LibraryToVerify,
        land_pattern: "TI PW0014A (different pinout from CD74HC4066)",
        verified: PinVerification::DatasheetTable,
        pins: &[
            p!("1", "SEL1", "PRI_SIG_EN"),
            p!("2", "S1", "PRI_CLK"),
            p!("3", "D1", "SENSOR_CLK"),
            p!("4", "SEL2", "PRI_SIG_EN"),
            p!("5", "S2", "PRI_SYNC"),
            p!("6", "D2", "SENSOR_SYNC"),
            p!("7", "GND", "GND"),
            p!("8", "D3", "SENSOR_DATA_ID"),
            p!("9", "S3", "PRI_SD"),
            p!("10", "SEL3", "PRI_SIG_EN"),
            p!("11", "D4", "SENSOR_DATA_ID"),
            p!("12", "S4", "ID_NODE"),
            p!("13", "SEL4", "ID_EN"),
            p!("14", "VDD", "+3V3_LOCAL"),
        ],
    },
    PinMap {
        refdes: "U_PRIPWR",
        mpn: "TPS22918DBVR",
        package: "SOT-23-6",
        footprint: FootprintStatus::LibraryToVerify,
        land_pattern: "TI DBV0006",
        verified: PinVerification::DatasheetTable,
        pins: &[
            p!("1", "VIN", "+3V3_LOCAL"),
            p!("2", "GND", "GND"),
            p!("3", "ON", "PRI_PWR_EN"),
            p!("4", "CT", "1 nF to GND"),
            p!("5", "QOD", "PRI_VDD"),
            p!("6", "VOUT", "PRI_VDD"),
        ],
    },
    PinMap {
        refdes: "U_TRH (T/RH variant)",
        mpn: "SHT45-AD1B-R2",
        package: "DFN-4 1.5 × 1.5 mm",
        footprint: FootprintStatus::ToCreateFromDrawing,
        land_pattern:
            "DS-SHT4X Fig. 17 (0.5 × 0.3 mm pads, 0.8 mm pitch, 1.4 mm span, no centre copper)",
        verified: PinVerification::DatasheetTable,
        pins: &[
            p!("1", "SDA", "PRI_SYNC"),
            p!("2", "SCL", "PRI_CLK"),
            p!("3", "VDD", "PRI_VDD"),
            p!("4", "VSS", "GND"),
            p!("DP", "die pad", "not connected, no copper"),
        ],
    },
    PinMap {
        refdes: "U_MIC (mic variant)",
        mpn: "DMM-4026-B-I2S-R",
        package: "LGA-7 4 × 3 mm bottom port",
        footprint: FootprintStatus::ToCreateFromDrawing,
        land_pattern: "DS-PUI-MIC advisory land pattern; port hole Ø ≥ 0.25 mm inside pad-4 ring",
        verified: PinVerification::DatasheetDrawing,
        pins: &[
            p!("1", "LR", "GND (left channel)"),
            p!("2", "CONFIG", "GND"),
            p!("3", "VDD", "PRI_VDD"),
            p!("4", "GND", "GND (ring around port)"),
            p!("5", "WS", "PRI_SYNC"),
            p!("6", "SCK", "PRI_CLK"),
            p!("7", "SD", "PRI_SD"),
        ],
    },
    PinMap {
        refdes: "U_AUXISO",
        mpn: "TMUX136RSER",
        package: "UQFN-10 2.0 × 1.5",
        footprint: FootprintStatus::LibraryToVerify,
        land_pattern: "TI RSE0010A",
        verified: PinVerification::DatasheetTable,
        pins: &[
            p!("1", "A1", "AUX_SCL_MUX (TMUX1109 DA)"),
            p!("2", "A2", "AUX_SDA_MUX (TMUX1109 DB)"),
            p!("3", "B1", "NC"),
            p!("4", "B2", "NC"),
            p!("5", "GND", "GND"),
            p!("6", "EN", "AUX_BUS_EN_N (CD74HC138 /Y6)"),
            p!("7", "COM2", "SENSOR_SYNC"),
            p!("8", "COM1", "SENSOR_CLK"),
            p!("9", "SEL", "GND (COM to A port)"),
            p!("10", "VCC", "+3V3_LOCAL"),
        ],
    },
    PinMap {
        refdes: "U_AUXMUX",
        mpn: "TMUX1109PWR",
        package: "TSSOP-16",
        footprint: FootprintStatus::LibraryToVerify,
        land_pattern: "TI PW0016A",
        verified: PinVerification::DatasheetTable,
        pins: &[
            p!("1", "A0", "AUX_A0"),
            p!("2", "EN", "AUX_SIG_EN"),
            p!("3", "VSS", "GND"),
            p!("4", "S1A", "AUX1_SCL"),
            p!("5", "S2A", "AUX2_SCL"),
            p!("6", "S3A", "AUX3_SCL"),
            p!("7", "S4A", "AUX4_SCL"),
            p!("8", "DA", "AUX_SCL_MUX"),
            p!("9", "DB", "AUX_SDA_MUX"),
            p!("10", "S4B", "AUX4_SDA"),
            p!("11", "S3B", "AUX3_SDA"),
            p!("12", "S2B", "AUX2_SDA"),
            p!("13", "S1B", "AUX1_SDA"),
            p!("14", "VDD", "+3V3_LOCAL"),
            p!("15", "GND", "GND"),
            p!("16", "A1", "AUX_A1"),
        ],
    },
    PinMap {
        refdes: "U_AUXDEC_PWR",
        mpn: "CD74HC238PWR",
        package: "TSSOP-16",
        footprint: FootprintStatus::LibraryToVerify,
        land_pattern: "TI PW0016A",
        verified: PinVerification::DatasheetTable,
        pins: &[
            p!("1", "A0", "AUX_A0"),
            p!("2", "A1", "AUX_A1"),
            p!("3", "A2", "AUX_ARM"),
            p!("4", "G0", "PRI_PWR_EN"),
            p!("5", "G1", "PRI_SIG_EN"),
            p!("6", "G2", "AUX_PWR_EN"),
            p!("7", "Y7", "AUX4_PWR_EN"),
            p!("8", "GND", "GND"),
            p!("9", "Y6", "AUX3_PWR_EN"),
            p!("10", "Y5", "AUX2_PWR_EN"),
            p!("11", "Y4", "AUX1_PWR_EN"),
            p!("12", "Y3", "NC"),
            p!("13", "Y2", "NC"),
            p!("14", "Y1", "NC"),
            p!("15", "Y0", "NC"),
            p!("16", "VCC", "+3V3_LOCAL"),
        ],
    },
    PinMap {
        refdes: "U_AUXDEC_BUS",
        mpn: "CD74HC138M96",
        package: "SOIC-16",
        footprint: FootprintStatus::LibraryToVerify,
        land_pattern: "TI D0016A",
        verified: PinVerification::DatasheetTable,
        pins: &[
            p!("1", "A0", "GND"),
            p!("2", "A1", "AUX_PWR_EN"),
            p!("3", "A2", "AUX_ARM"),
            p!("4", "G0", "PRI_PWR_EN"),
            p!("5", "G1", "PRI_SIG_EN"),
            p!("6", "G2", "AUX_SIG_EN"),
            p!("7", "Y7", "NC"),
            p!("8", "GND", "GND"),
            p!("9", "Y6", "AUX_BUS_EN_N"),
            p!("10", "Y5", "NC"),
            p!("11", "Y4", "NC"),
            p!("12", "Y3", "NC"),
            p!("13", "Y2", "NC"),
            p!("14", "Y1", "NC"),
            p!("15", "Y0", "NC"),
            p!("16", "VCC", "+3V3_LOCAL"),
        ],
    },
    PinMap {
        refdes: "U_AUXPWR1..4",
        mpn: "TPS2553DBVR-1",
        package: "SOT-23-6",
        footprint: FootprintStatus::LibraryToVerify,
        land_pattern: "TI DBV0006",
        verified: PinVerification::DatasheetTable,
        pins: &[
            p!("1", "IN", "+3V3_LOCAL"),
            p!("2", "GND", "GND"),
            p!("3", "EN", "AUXn_PWR_EN (active high)"),
            p!("4", "FAULT", "AUXn_FAULT_N (100 k pull-up, test pad)"),
            p!("5", "ILIM", "93.1 k to GND"),
            p!("6", "OUT", "AUXn_VCC"),
        ],
    },
    PinMap {
        refdes: "D_ARM + R_ARM",
        mpn: "low-leakage diode (TBD) + 150 kΩ 1 %",
        package: "SOD-323 + 0402",
        footprint: FootprintStatus::DrawingMissing,
        land_pattern: "per selected diode",
        verified: PinVerification::Pending,
        pins: &[
            p!("A", "anode", "AUX_ARM (595 QH)"),
            p!("K", "cathode", "R_ARM 150 k → SENSOR_DATA_ID"),
        ],
    },
    PinMap {
        refdes: "J_AUX1..4",
        mpn: "B4B-PH-K-S(LF)(SN)",
        package: "JST PH 4-pos THT",
        footprint: FootprintStatus::ToCreateFromDrawing,
        land_pattern: "DS-JST-PH: Ø0.7 mm holes, 2.0 mm pitch; circuit 1 marked",
        verified: PinVerification::DatasheetDrawing,
        pins: &[
            p!("1", "1", "AUXn_VCC"),
            p!("2", "2", "GND"),
            p!("3", "3", "AUXn_SDA"),
            p!("4", "4", "AUXn_SCL"),
        ],
    },
    PinMap {
        refdes: "D_AUX1..4",
        mpn: "TPD4E05U06DQAR",
        package: "USON-10",
        footprint: FootprintStatus::LibraryToVerify,
        land_pattern: "TI DQA0010A (flow-through)",
        verified: PinVerification::DatasheetTable,
        pins: &[
            p!("1", "D1+", "AUXn_SDA"),
            p!("2", "D1-", "AUXn_SCL"),
            p!("3", "GND", "GND"),
            p!("4", "D2+", "AUXn_VCC"),
            p!("5", "D2-", "NC"),
            p!("6..10", "NC/GND", "pin 8 GND, others NC"),
        ],
    },
    // ---------------------------------------------------------------- main board
    PinMap {
        refdes: "U_MCU",
        mpn: "ESP32-S3-WROOM-1-N16R2",
        package: "41-pad module",
        footprint: FootprintStatus::ToCreateFromDrawing,
        land_pattern: "DS-ESP-MOD recommended PCB land pattern + antenna keep-out",
        verified: PinVerification::DatasheetTable,
        pins: &[
            p!("1", "GND", "GND"),
            p!("2", "3V3", "+3V3"),
            p!("3", "EN", "CHIP_PU (10 k / 1 µF)"),
            p!("4", "IO4", "SENSOR_CLK"),
            p!("5", "IO5", "SENSOR_SYNC"),
            p!("6", "IO6", "SENSOR_PU_EN"),
            p!("7", "IO7", "CHAIN_PWR_EN"),
            p!("8", "IO15", "DISP_PWR_EN"),
            p!("9", "IO16", "REG_PS_SYNC"),
            p!("10", "IO17", "CHG_EN1"),
            p!("11", "IO18", "CHG_EN2"),
            p!("12", "IO8", "CHAIN_FAULT_N"),
            p!("13", "IO19", "USB_D-"),
            p!("14", "IO20", "USB_D+"),
            p!("15", "IO3", "NC (strapping)"),
            p!("16", "IO46", "NC (strapping)"),
            p!("17", "IO9", "SYS_SDA"),
            p!("18", "IO10", "SYS_SCL"),
            p!("19", "IO11", "PATCH_SHIFT_DATA"),
            p!("20", "IO12", "PATCH_SHIFT_CLK"),
            p!("21", "IO13", "PATCH_LATCH"),
            p!("22", "IO14", "PATCH_OE_N"),
            p!("23", "IO21", "CHG_STAT_N"),
            p!("24", "IO47", "spare TP"),
            p!("25", "IO48", "spare TP"),
            p!("26", "IO45", "NC (strapping)"),
            p!("27", "IO0", "BOOT"),
            p!("28", "IO35", "CHG_PGOOD_N"),
            p!("29", "IO36", "REG_PG"),
            p!("30", "IO37", "GAUGE_ALRT_N"),
            p!("31", "IO38", "spare TP"),
            p!("32", "IO39", "LED_R"),
            p!("33", "IO40", "LED_G"),
            p!("34", "IO41", "LED_B"),
            p!("35", "IO42", "spare TP"),
            p!("36", "RXD0", "UART0_RX"),
            p!("37", "TXD0", "UART0_TX"),
            p!("38", "IO2", "VBAT_SENSE"),
            p!("39", "IO1", "SENSOR_DATA_ID"),
            p!("40", "GND", "GND"),
            p!("41", "EPAD", "GND"),
        ],
    },
    PinMap {
        refdes: "U_CHG",
        mpn: "BQ24074RGTR",
        package: "VQFN-16 3 × 3",
        footprint: FootprintStatus::LibraryToVerify,
        land_pattern: "TI RGT0016 + thermal vias",
        verified: PinVerification::DatasheetTable,
        pins: &[
            p!("1", "TS", "BATT_NTC (10 k NTC)"),
            p!("2", "BAT", "VBAT"),
            p!("3", "BAT", "VBAT"),
            p!("4", "CE", "GND (charge enabled)"),
            p!("5", "EN2", "CHG_EN2"),
            p!("6", "EN1", "CHG_EN1"),
            p!("7", "PGOOD", "CHG_PGOOD_N"),
            p!("8", "VSS", "GND"),
            p!("9", "CHG", "CHG_STAT_N"),
            p!("10", "OUT", "VSYS"),
            p!("11", "OUT", "VSYS"),
            p!("12", "ILIM", "1.62 k to GND"),
            p!("13", "IN", "VBUS"),
            p!("14", "TMR", "68.1 k to GND"),
            p!("15", "ITERM", "NC (10 % default)"),
            p!("16", "ISET", "1.78 k to GND"),
            p!("PAD", "thermal", "GND"),
        ],
    },
    PinMap {
        refdes: "U_REG",
        mpn: "TPS63021DSJR",
        package: "VSON-14 3 × 4",
        footprint: FootprintStatus::LibraryToVerify,
        land_pattern: "TI DSJ0014 + layout example",
        verified: PinVerification::DatasheetTable,
        pins: &[
            p!("1", "VINA", "VSYS"),
            p!("2", "GND", "GND"),
            p!("3", "FB", "+3V3"),
            p!("4", "VOUT", "+3V3"),
            p!("5", "VOUT", "+3V3"),
            p!("6", "L2", "L1.2"),
            p!("7", "L2", "L1.2"),
            p!("8", "L1", "L1.1"),
            p!("9", "L1", "L1.1"),
            p!("10", "VIN", "VSYS"),
            p!("11", "VIN", "VSYS"),
            p!("12", "EN", "VSYS"),
            p!("13", "PS/SYNC", "REG_PS_SYNC"),
            p!("14", "PG", "REG_PG"),
            p!("PAD", "PGND", "GND"),
        ],
    },
    PinMap {
        refdes: "U_CHAINSW",
        mpn: "TPS2553DBVR-1",
        package: "SOT-23-6",
        footprint: FootprintStatus::LibraryToVerify,
        land_pattern: "TI DBV0006",
        verified: PinVerification::DatasheetTable,
        pins: &[
            p!("1", "IN", "+5V0"),
            p!("2", "GND", "GND"),
            p!("3", "EN", "CHAIN_PWR_EN"),
            p!("4", "FAULT", "CHAIN_FAULT_N"),
            p!("5", "ILIM", "76.8 k to GND"),
            p!("6", "OUT", "+5V_CHAIN"),
        ],
    },
    PinMap {
        refdes: "U_DISPSW",
        mpn: "TPS22918DBVR",
        package: "SOT-23-6",
        footprint: FootprintStatus::LibraryToVerify,
        land_pattern: "TI DBV0006",
        verified: PinVerification::DatasheetTable,
        pins: &[
            p!("1", "VIN", "+3V3"),
            p!("2", "GND", "GND"),
            p!("3", "ON", "DISP_PWR_EN"),
            p!("4", "CT", "1 nF"),
            p!("5", "QOD", "+3V3_DISPLAY"),
            p!("6", "VOUT", "+3V3_DISPLAY"),
        ],
    },
    PinMap {
        refdes: "J_BATT",
        mpn: "S2B-PH-SM4-TB(LF)(SN)",
        package: "JST PH 2-pos SMT",
        footprint: FootprintStatus::ToCreateFromDrawing,
        land_pattern: "DS-JST-PH SMT layout",
        verified: PinVerification::DatasheetDrawing,
        pins: &[
            p!("1", "1", "VBAT (check pack polarity)"),
            p!("2", "2", "GND"),
        ],
    },
    PinMap {
        refdes: "J_DISP",
        mpn: "SM04B-SRSS-TB(LF)(SN)",
        package: "JST SH 4-pos",
        footprint: FootprintStatus::DrawingMissing,
        land_pattern: "JST SH drawing not obtained",
        verified: PinVerification::Pending,
        pins: &[
            p!("1", "1", "GND"),
            p!("2", "2", "+3V3_DISPLAY"),
            p!("3", "3", "SYS_SCL"),
            p!("4", "4", "SYS_SDA"),
        ],
    },
    PinMap {
        refdes: "J_USB",
        mpn: "USB4105-GF-A",
        package: "USB-C 16-pin",
        footprint: FootprintStatus::ToCreateFromDrawing,
        land_pattern: "GCT USB4105 drawing",
        verified: PinVerification::DatasheetDrawing,
        pins: &[
            p!("A4/B4/A9/B9", "VBUS", "VBUS"),
            p!("A5", "CC1", "5.1 k to GND"),
            p!("B5", "CC2", "5.1 k to GND"),
            p!("A6/B6", "D+", "USB_D+"),
            p!("A7/B7", "D-", "USB_D-"),
            p!("A1/B1/A12/B12", "GND", "GND"),
            p!("S", "shield", "GND via 1 M || 4.7 nF"),
        ],
    },
    PinMap {
        refdes: "U_GAUGE",
        mpn: "MAX17048G+T10",
        package: "TDFN-8",
        footprint: FootprintStatus::DrawingMissing,
        land_pattern: "Datasheet not downloaded",
        verified: PinVerification::CarriedFromV1,
        pins: &[],
    },
    PinMap {
        refdes: "D_USB",
        mpn: "USBLC6-2SC6",
        package: "SOT-23-6",
        footprint: FootprintStatus::DrawingMissing,
        land_pattern: "Datasheet not downloaded",
        verified: PinVerification::CarriedFromV1,
        pins: &[],
    },
];

/// Test points.
pub const TEST_POINTS: &[(&str, &str)] = &[
    ("Main", "VBUS, VBAT, VSYS, +3V3, +5V0, +5V_CHAIN, +3V3_DISPLAY, GND ×2, SENSOR_CLK, SENSOR_SYNC, SENSOR_DATA_ID, PATCH_SHIFT_DATA/CLK/LATCH/OE_N, CHAIN_FAULT_N, UART0 TX/RX, BOOT, CHIP_PU"),
    ("Patch", "+5V_CHAIN (IN and OUT side), +3V3_LOCAL, GND, PRI_VDD, Q0..Q7, SHIFT_OUT, ID_NODE, AUX_BUS_EN_N, AUX1..4_VCC, AUX1..4_FAULT_N"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn pin_numbers_unique_per_part() {
        for m in PIN_MAPS {
            let mut s = HashSet::new();
            for p in m.pins {
                assert!(s.insert(p.number), "{} pin {}", m.refdes, p.number);
            }
        }
    }

    #[test]
    fn chain_connector_matches_icd() {
        let fpc = PIN_MAPS.iter().find(|m| m.mpn.starts_with("FH34")).unwrap();
        for (i, icd) in crate::interfaces::CHAIN_PINS.iter().enumerate() {
            assert!(
                fpc.pins[i].net.starts_with(icd.net),
                "pin {}: {} vs {}",
                i + 1,
                fpc.pins[i].net,
                icd.net
            );
        }
    }

    #[test]
    fn aux_header_matches_client_pinout() {
        let j = PIN_MAPS
            .iter()
            .find(|m| m.refdes.starts_with("J_AUX"))
            .unwrap();
        let sig: Vec<_> = j.pins.iter().map(|p| p.net.replace("AUXn_", "")).collect();
        assert_eq!(sig, ["VCC", "GND", "SDA", "SCL"]);
    }

    #[test]
    fn esp_map_consistent_with_gpio_map() {
        let m = PIN_MAPS.iter().find(|m| m.refdes == "U_MCU").unwrap();
        for g in crate::main_board::GPIO_MAP {
            let name = match g.gpio {
                43 => "TXD0".to_string(),
                44 => "RXD0".to_string(),
                n => format!("IO{n}"),
            };
            let pin = m
                .pins
                .iter()
                .find(|p| p.name == name)
                .unwrap_or_else(|| panic!("{name}"));
            assert_eq!(pin.net, g.net, "GPIO{}", g.gpio);
        }
        assert_eq!(m.pins.len(), 41);
    }

    #[test]
    fn shift_register_bits_follow_register_map() {
        let sr = PIN_MAPS.iter().find(|m| m.refdes == "U_SR").unwrap();
        let q = |name: &str| sr.pins.iter().find(|p| p.name == name).unwrap().net;
        for (bit, name) in ["QA", "QB", "QC", "QD", "QE", "QF", "QG", "QH"]
            .iter()
            .enumerate()
        {
            let net = crate::sensor_patch::REGISTER_MAP[bit].net;
            assert!(q(name).starts_with(net), "{name}: {} vs {net}", q(name));
        }
    }
}

/// Rev D note: pin maps of TPS61023 (main) and TPS62840 (patch) are taken from their datasheets in the
/// schematic phase; until then their records in `component_specs::REV_D_SPECS` carry package data only.
pub const REV_D_PINMAP_PENDING: &[&str] =
    &["U7 TPS61023DRLR (SOT563)", "U14 TPS62840DLCR (SON-8 DLC)"];
