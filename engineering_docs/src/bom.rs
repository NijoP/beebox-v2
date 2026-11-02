//! # V2 recommended master BOM (pre-schematic, rev C 2026-11-02)
//!
//! Quantities are per board. System quantity = main × 1 + patch × N (N ≤ 8; first prototype N = 2,
//! DEC-026). Passive MPNs are left to the AVL step (OQ-029) but every passive line carries value,
//! tolerance, voltage/power rating, dielectric and **imperial** package code (metric in brackets,
//! REQ-DP-04). Reference designators are provisional until schematic capture.

use crate::component_model::{Board, EngStatus, Populate};

/// One BOM line.
#[derive(Debug, Clone, Copy)]
pub struct BomLine {
    /// Reference designator(s).
    pub refdes: &'static str,
    /// Board.
    pub board: Board,
    /// Quantity per board.
    pub qty: u16,
    /// Value / description.
    pub value: &'static str,
    /// Manufacturer.
    pub manufacturer: &'static str,
    /// MPN (or `AVL` for commodity passives).
    pub mpn: &'static str,
    /// Tolerance.
    pub tolerance: &'static str,
    /// Voltage / current / power rating.
    pub rating: &'static str,
    /// Package (imperial code, metric code in brackets).
    pub package: &'static str,
    /// Population rule.
    pub populate: Populate,
    /// Link to [`crate::component_specs::SPECS`] key, if any.
    pub spec: Option<&'static str>,
    /// Engineering status.
    pub status: EngStatus,
}

const AVL: &str = "AVL (OQ-029)";
const GEN: &str = "Generic";

macro_rules! line {
    ($r:expr, $b:ident, $q:expr, $v:expr, $m:expr, $mpn:expr, $t:expr, $rt:expr, $p:expr, $pop:ident, $s:expr, $st:ident) => {
        BomLine {
            refdes: $r,
            board: Board::$b,
            qty: $q,
            value: $v,
            manufacturer: $m,
            mpn: $mpn,
            tolerance: $t,
            rating: $rt,
            package: $p,
            populate: Populate::$pop,
            spec: $s,
            status: EngStatus::$st,
        }
    };
}

/// Main controller board BOM.
pub const MAIN_BOM: &[BomLine] = &[
    line!("U1", Main, 1, "ESP32-S3 module 16 MB flash / 2 MB PSRAM", "Espressif", "ESP32-S3-WROOM-1-N16R2", "-", "3.0-3.6 V, ≥ 0.5 A", "SMD module", Fit, Some("MCU"), Recommended),
    line!("U2", Main, 1, "1S charger / power path", "Texas Instruments", "BQ24074RGTR", "-", "IN 10.5 V OVP, 1.5 A", "VQFN-16 RGT", Fit, Some("CHARGER"), Recommended),
    line!("U3", Main, 1, "Buck-boost 3.3 V fixed", "Texas Instruments", "TPS63021DSJR", "±1 % PWM", "2 A @ VIN > 2.5 V", "VSON-14 DSJ", Fit, Some("BUCKBOOST"), Recommended),
    line!("U4", Main, 1, "+5V_CHAIN current-limited switch, latch (IN = +5V0)", "Texas Instruments", "TPS2553DBVR-1", "-", "<= 400 mA (76.8 k)", "SOT-23-6", Fit, Some("CHAINSW"), Recommended),
    line!("U7", Main, 1, "+5V0 boost for chain (rev D)", "Texas Instruments", "TPS61023DRLR", "-", "0.5-5.5 V in, 3.7 A valley", "SOT563", Fit, Some("BOOST5V"), RecommendedWithCondition),
    line!("L2", Main, 1, "1.0 µH boost inductor (rev D)", AVL, AVL, "±20 %", "Isat >= valley limit + ripple (to size)", "TBD", Fit, None, MpnPending),
    line!("U5", Main, 1, "+3V3_DISPLAY load switch", "Texas Instruments", "TPS22918DBVR", "-", "2 A", "SOT-23-6", Fit, Some("DISPSW"), Recommended),
    line!("U6", Main, 1, "Fuel gauge", "Analog Devices", "MAX17048G+T10", "-", "-", "TDFN-8", Fit, Some("GAUGE"), Unverified),
    line!("D1", Main, 1, "USB ESD", "STMicroelectronics", "USBLC6-2SC6", "-", "-", "SOT-23-6", Fit, Some("USBESD"), Unverified),
    line!("D2, D3", Main, 2, "Chain-line ESD (7 signals)", "Texas Instruments", "TPD4E05U06DQAR", "-", "VRWM 5.5 V, 0.5 pF", "USON-10", Fit, Some("ESD4"), RecommendedWithCondition),
    line!("D4-D6", Main, 3, "Status LEDs R/G/B", "Lite-On", "LTST-C193KRKT-5A / KGKT-5A / KBKT-5A", "-", "to verify", "0603 [1608]", Fit, None, Unverified),
    line!("J1", Main, 1, "USB-C receptacle", "GCT", "USB4105-GF-A", "-", "5 A VBUS, 48 V", "USB-C SMT", Fit, Some("USBC"), Recommended),
    line!("J2", Main, 1, "12-pin chain connector", "Hirose", "FH34SRJ-12S-0.5SH(50)", "-", "0.5 A/contact, 50 V", "0.5 mm ZIF", Fit, Some("FPC12"), RecommendedWithCondition),
    line!("J3", Main, 1, "Battery connector", "JST", "S2B-PH-SM4-TB(LF)(SN)", "-", "2 A, 100 V", "PH 2-pos SMT", Fit, Some("BATTCONN"), Recommended),
    line!("J4", Main, 1, "Display connector (GND, 3V3, SCL, SDA)", "JST", "SM04B-SRSS-TB(LF)(SN)", "-", "to verify", "SH 4-pos SMT", Fit, Some("DISPCONN"), ClientDecision),
    line!("J5", Main, 1, "UART0 debug header (GND, TX, RX, 3V3)", GEN, "1×4 2.54 mm header", "-", "-", "THT", Fit, None, MpnPending),
    line!("SW1, SW2", Main, 2, "BOOT, RESET tactile switches", GEN, AVL, "-", "≥ 12 V 50 mA", "SMD tact", Fit, None, MpnPending),
    line!("L1", Main, 1, "1.5 µH shielded power inductor", AVL, AVL, "±20 %", "Isat ≥ 2.5 A, DCR ≤ 50 mΩ", "≈ 4 × 4 mm", Fit, None, MpnPending),
    line!("BT1", OffBoard, 1, "LiPo pack 3000 mAh (CONFIRMED); cell configuration, charge voltage, cutoff, discharge current, protection and NTC per pack datasheet (pending)", AVL, "pack MPN pending", "-", ">= 1.3 A peak (calculated)", "pack", Fit, None, ClientDecision),
    line!("R1", Main, 1, "1.78 kΩ BQ24074 ISET", AVL, AVL, "1 %, ≤ 100 ppm/K", "≥ 63 mW", "0402 [1005]", Fit, None, MpnPending),
    line!("R2", Main, 1, "1.62 kΩ BQ24074 ILIM", AVL, AVL, "1 %", "≥ 63 mW", "0402 [1005]", Fit, None, MpnPending),
    line!("R3", Main, 1, "68.1 kΩ BQ24074 TMR", AVL, AVL, "1 %", "≥ 63 mW", "0402 [1005]", Fit, None, MpnPending),
    line!("R4", Main, 1, "10.0 kΩ TS (only if pack has no NTC)", AVL, AVL, "1 %", "≥ 63 mW", "0402 [1005]", Dni, None, MpnPending),
    line!("R5, R6", Main, 2, "5.1 kΩ USB-C CC Rd", AVL, AVL, "1 %", "≥ 63 mW", "0402 [1005]", Fit, None, MpnPending),
    line!("R7", Main, 1, "1.00 MΩ VBAT divider top", AVL, AVL, "1 %", "≥ 63 mW", "0402 [1005]", Fit, None, MpnPending),
    line!("R8", Main, 1, "330 kΩ VBAT divider bottom", AVL, AVL, "1 %", "≥ 63 mW", "0402 [1005]", Fit, None, MpnPending),
    line!("R9", Main, 1, "100 kΩ SENSOR_DATA_ID pull-down", AVL, AVL, "1 %, ≤ 100 ppm/K", "≥ 63 mW", "0402 [1005]", Fit, None, MpnPending),
    line!("R10", Main, 1, "76.8 kΩ TPS2553 ILIM (rev D, <= 400 mA)", AVL, AVL, "1 %", "≥ 63 mW", "0402 [1005]", Fit, None, MpnPending),
    line!("R11", Main, 1, "10 kΩ CHIP_PU pull-up", AVL, AVL, "5 %", "≥ 63 mW", "0402 [1005]", Fit, None, MpnPending),
    line!("R12", Main, 1, "10 kΩ GPIO0 BOOT pull-up", AVL, AVL, "5 %", "≥ 63 mW", "0402 [1005]", Fit, None, MpnPending),
    line!("R13", Main, 1, "REMOVED in rev D: PATCH_OE_N pull-up moved to each patch (+3V3_LOCAL)", AVL, AVL, "-", "-", "0402 [1005]", Dni, None, MpnPending),
    line!("R34, R35", Main, 2, "TPS61023 FB divider for 5.0 V (values per datasheet equation)", AVL, AVL, "1 %", "≥ 63 mW", "0402 [1005]", Fit, None, MpnPending),
    line!("R14-R23", Main, 10, "100 kΩ pulls: CHAIN_PWR_EN↓, SHIFT_CLK↓, LATCH↓, PS_SYNC↓, CHG_EN2↓, CHG_EN1↑, CHAIN_FAULT_N↑, CHG_STAT_N↑, PGOOD_N↑, REG_PG↑", AVL, AVL, "5 %", "≥ 63 mW", "0402 [1005]", Fit, None, MpnPending),
    line!("R24", Main, 1, "47 kΩ DISP_PWR_EN pull-down", AVL, AVL, "5 %", "≥ 63 mW", "0402 [1005]", Fit, None, MpnPending),
    line!("R25-R30", Main, 6, "33 Ω series: SHIFT_DATA, SHIFT_CLK, LATCH, OE_N, SENSOR_CLK, SENSOR_SYNC", AVL, AVL, "5 %", "≥ 63 mW", "0402 [1005]", Fit, None, MpnPending),
    line!("R31, R32", Main, 2, "2.2 kΩ SYS_SDA/SYS_SCL pull-ups", AVL, AVL, "1 %", "≥ 63 mW", "0402 [1005]", Fit, None, MpnPending),
    line!("R33, R34", Main, 2, "2.2 kΩ SENSOR_CLK/SYNC pull-ups to SENSOR_PU_EN (PRP-MB-02, primary I²C mode)", AVL, AVL, "1 %", "≥ 63 mW", "0402 [1005]", Fit, None, MpnPending),
    line!("R35-R37", Main, 3, "1.00 kΩ LED series (recheck Vf)", AVL, AVL, "1 %", "≥ 63 mW", "0402 [1005]", Fit, None, MpnPending),
    line!("R38", Main, 1, "100 kΩ GAUGE_ALRT_N pull-up", AVL, AVL, "5 %", "≥ 63 mW", "0402 [1005]", Fit, None, MpnPending),
    line!("C1", Main, 1, "1 µF BQ24074 IN", AVL, AVL, "±10 %", "25 V X7R", "0603 [1608]", Fit, None, MpnPending),
    line!("C2", Main, 1, "10 nF VBUS HF", AVL, AVL, "±10 %", "25 V X7R", "0402 [1005]", Fit, None, MpnPending),
    line!("C3, C4", Main, 2, "10 µF BQ24074 OUT and BAT", AVL, AVL, "±20 %", "10 V X5R", "0805 [2012]", Fit, None, MpnPending),
    line!("C5, C6", Main, 2, "10 µF TPS63021 input", AVL, AVL, "±20 %", "10 V X5R", "0805 [2012]", Fit, None, MpnPending),
    line!("C7-C9", Main, 3, "22 µF TPS63021 output", AVL, AVL, "±20 %", "10 V X5R", "0805 [2012]", Fit, None, MpnPending),
    line!("C10", Main, 1, "22 µF ESP32 3V3 bulk", AVL, AVL, "±20 %", "10 V X5R", "0805 [2012]", Fit, None, MpnPending),
    line!("C11", Main, 1, "1 µF CHIP_PU", AVL, AVL, "±10 %", "10 V X7R", "0402 [1005]", Fit, None, MpnPending),
    line!("C12-C20", Main, 9, "100 nF bypass (U1, U2-U6, D1, VBAT ADC, TPS63021 VINA)", AVL, AVL, "±10 %", "16 V X7R", "0402 [1005]", Fit, None, MpnPending),
    line!("C21, C22", Main, 2, "1 µF TPS2553 IN, TPS22918 VIN", AVL, AVL, "±10 %", "10 V X7R", "0603 [1608]", Fit, None, MpnPending),
    line!("C23", Main, 1, "10 µF +5V_CHAIN output bulk", AVL, AVL, "±20 %", "16 V X5R", "0805 [2012]", Fit, None, MpnPending),
    line!("C26, C27", Main, 2, "10 µF TPS61023 output (>= 4 µF effective at 5 V after DC bias)", AVL, AVL, "±20 %", "16 V X5R", "0805 [2012]", Fit, None, MpnPending),
    line!("C28", Main, 1, "10 µF TPS61023 input", AVL, AVL, "±20 %", "10 V X5R", "0805 [2012]", Fit, None, MpnPending),
    line!("C24", Main, 1, "1 nF TPS22918 CT", AVL, AVL, "±10 %", "16 V X7R", "0402 [1005]", Fit, None, MpnPending),
    line!("C25", Main, 1, "1 µF MAX17048 (V1 record)", AVL, AVL, "±10 %", "10 V X7R", "0402 [1005]", Fit, None, Unverified),
    line!("F1", Main, 1, "PTC MF-MSMF050-2 (optional)", "Bourns", "MF-MSMF050-2", "-", "0.5 A hold (V1)", "1812", Dni, None, Unverified),
];

/// Sensor patch BOM (both variants; see `populate`). Topology per rev C (aux_expansion).
pub const PATCH_BOM: &[BomLine] = &[
    line!(
        "J1, J2",
        Patch,
        2,
        "12-pin chain IN / OUT",
        "Hirose",
        "FH34SRJ-12S-0.5SH(50)",
        "-",
        "0.5 A/contact",
        "0.5 mm ZIF",
        Fit,
        Some("FPC12"),
        RecommendedWithCondition
    ),
    line!(
        "U1",
        Patch,
        1,
        "Shift register (LV595A alt D-02)",
        "Texas Instruments",
        "SN74HC595PWR",
        "-",
        "2-6 V",
        "TSSOP-16",
        Fit,
        Some("SR"),
        RecommendedWithCondition
    ),
    line!(
        "U2",
        Patch,
        1,
        "Primary signal/ID switch (TMUX1511 alt D-03)",
        "Texas Instruments",
        "CD74HC4066PWR",
        "-",
        "RON ≤ 106 Ω @4.5 V",
        "TSSOP-14",
        Fit,
        Some("PRISW"),
        NeedsAlternative
    ),
    line!(
        "U3",
        Patch,
        1,
        "Primary sensor load switch",
        "Texas Instruments",
        "TPS22918DBVR",
        "-",
        "2 A",
        "SOT-23-6",
        Fit,
        Some("DISPSW"),
        Recommended
    ),
    line!(
        "U4",
        Patch,
        1,
        "T/RH sensor",
        "Sensirion",
        "SHT45-AD1B-R2",
        "±1.0 %RH / ±0.1 °C",
        "1.08-3.6 V",
        "DFN-4 1.5 × 1.5",
        TrhVariant,
        Some("SHT45"),
        RecommendedWithCondition
    ),
    line!(
        "U5",
        Patch,
        1,
        "I²S microphone",
        "PUI Audio",
        "DMM-4026-B-I2S-R",
        "-26 ± 1 dBFS",
        "1.62-3.63 V",
        "LGA-7 4 × 3",
        MicVariant,
        Some("MIC"),
        RecommendedWithCondition
    ),
    line!(
        "U6",
        Patch,
        1,
        "AUX bus isolator",
        "Texas Instruments",
        "TMUX136RSER",
        "-",
        "RON ≤ 13 Ω, 2 pF",
        "UQFN-10",
        Fit,
        Some("AUXISO"),
        Recommended
    ),
    line!(
        "U7",
        Patch,
        1,
        "AUX port mux dual 4:1",
        "Texas Instruments",
        "TMUX1109PWR",
        "-",
        "RON ≤ 9.75 Ω @3.3 V",
        "TSSOP-16",
        Fit,
        Some("AUXMUX"),
        Recommended
    ),
    line!(
        "U8",
        Patch,
        1,
        "AUX power decoder",
        "Texas Instruments",
        "CD74HC238PWR",
        "-",
        "2-6 V",
        "TSSOP-16",
        Fit,
        Some("AUXDEC_PWR"),
        RecommendedWithCondition
    ),
    line!(
        "U9",
        Patch,
        1,
        "AUX bus decoder",
        "Texas Instruments",
        "CD74HC138M96",
        "-",
        "2-6 V",
        "SOIC-16",
        Fit,
        Some("AUXDEC_BUS"),
        RecommendedWithCondition
    ),
    line!(
        "U10-U13",
        Patch,
        4,
        "AUX port power switch",
        "Texas Instruments",
        "TPS2553DBVR-1",
        "-",
        "248-329 mA limit",
        "SOT-23-6",
        Fit,
        Some("PORTSW"),
        Recommended
    ),
    line!(
        "J3-J6 (J_AUX1..4)",
        Patch,
        4,
        "AUX header 1 VCC 2 GND 3 SDA 4 SCL (APR-2 JST-GH)",
        "JST",
        "SM04B-GHS-TB(LF)(SN)",
        "-",
        "UNVERIFIED",
        "GH 4-pos SMT 1.25 mm",
        Fit,
        Some("AUXCONN"),
        Unverified
    ),
    line!(
        "U14",
        Patch,
        1,
        "+3V3_LOCAL buck (rev D)",
        "Texas Instruments",
        "TPS62840DLCR",
        "-",
        "1.8-6.5 V in, 750 mA",
        "SON-8 DLC",
        Fit,
        Some("PATCHBUCK"),
        RecommendedWithCondition
    ),
    line!(
        "L1",
        Patch,
        1,
        "2.2 µH buck inductor (rev D)",
        AVL,
        AVL,
        "±20 %",
        "Isat >= 1 A, low DCR (to size)",
        "TBD",
        Fit,
        None,
        MpnPending
    ),
    line!(
        "R36",
        Patch,
        1,
        "267 kΩ TPS62840 VSET (3.3 V)",
        AVL,
        AVL,
        "1 %, ≤ 200 ppm/K",
        "≥ 63 mW",
        "0402 [1005]",
        Fit,
        None,
        MpnPending
    ),
    line!(
        "R37",
        Patch,
        1,
        "47 kΩ PATCH_OE_N pull-up to +3V3_LOCAL (rev D, moved from main)",
        AVL,
        AVL,
        "5 %",
        "≥ 63 mW",
        "0402 [1005]",
        Fit,
        None,
        MpnPending
    ),
    line!(
        "C24",
        Patch,
        1,
        "10 µF TPS62840 output",
        AVL,
        AVL,
        "±20 %",
        "10 V X5R",
        "0805 [2012]",
        Fit,
        None,
        MpnPending
    ),
    line!(
        "D1-D4",
        Patch,
        4,
        "AUX port ESD (SDA, SCL, VCC)",
        "Texas Instruments",
        "TPD4E05U06DQAR",
        "-",
        "VRWM 5.5 V",
        "USON-10",
        Fit,
        Some("ESD4"),
        RecommendedWithCondition
    ),
    line!(
        "D5",
        Patch,
        1,
        "AUX-armed sense diode",
        "TBD",
        "TBD (low-leakage switching diode)",
        "-",
        "IR ≤ 50 nA @3.3 V 85 °C",
        "SOD-323",
        Fit,
        Some("ARMDIODE"),
        Unverified
    ),
    line!(
        "R1",
        Patch,
        1,
        "22.0 kΩ R_ID T/RH",
        AVL,
        AVL,
        "1 %, ≤ 100 ppm/K",
        "≥ 63 mW",
        "0402 [1005]",
        TrhVariant,
        None,
        MpnPending
    ),
    line!(
        "R2",
        Patch,
        1,
        "68.0 kΩ R_ID mic",
        AVL,
        AVL,
        "1 %, ≤ 100 ppm/K",
        "≥ 63 mW",
        "0402 [1005]",
        MicVariant,
        None,
        MpnPending
    ),
    line!(
        "R3",
        Patch,
        1,
        "150 kΩ arm-sense series",
        AVL,
        AVL,
        "1 %",
        "≥ 63 mW",
        "0402 [1005]",
        Fit,
        None,
        MpnPending
    ),
    line!(
        "R5-R12",
        Patch,
        8,
        "2.2 kΩ AUXn SDA/SCL pull-ups to AUXn_VCC",
        AVL,
        AVL,
        "1 %",
        "≥ 63 mW",
        "0402 [1005]",
        PerModuleProfile,
        None,
        MpnPending
    ),
    line!(
        "R13-R20",
        Patch,
        8,
        "47 kΩ pull-downs Q0-Q7",
        AVL,
        AVL,
        "5 %",
        "≥ 63 mW",
        "0402 [1005]",
        Fit,
        None,
        MpnPending
    ),
    line!(
        "R21-R24",
        Patch,
        4,
        "93.1 kΩ TPS2553 ILIM",
        AVL,
        AVL,
        "1 %",
        "≥ 63 mW",
        "0402 [1005]",
        Fit,
        None,
        MpnPending
    ),
    line!(
        "R25-R28",
        Patch,
        4,
        "10 kΩ AUXn_VCC bleeder",
        AVL,
        AVL,
        "5 %",
        "≥ 63 mW (1.2 mW actual)",
        "0402 [1005]",
        Fit,
        None,
        MpnPending
    ),
    line!(
        "R29-R32",
        Patch,
        4,
        "100 kΩ AUXn_FAULT_N pull-ups",
        AVL,
        AVL,
        "5 %",
        "≥ 63 mW",
        "0402 [1005]",
        Fit,
        None,
        MpnPending
    ),
    line!(
        "R33",
        Patch,
        1,
        "33 Ω QH' series",
        AVL,
        AVL,
        "5 %",
        "≥ 63 mW",
        "0402 [1005]",
        Fit,
        None,
        MpnPending
    ),
    line!(
        "C1",
        Patch,
        1,
        "10 µF +5V_CHAIN entry / TPS62840 input",
        AVL,
        AVL,
        "±20 %",
        "16 V X5R",
        "0805 [2012]",
        Fit,
        None,
        MpnPending
    ),
    line!(
        "C2-C8",
        Patch,
        7,
        "100 nF bypass U1, U2, U3 out, U6, U7, U8, U9",
        AVL,
        AVL,
        "±10 %",
        "16 V X7R",
        "0402 [1005]",
        Fit,
        None,
        MpnPending
    ),
    line!(
        "C9",
        Patch,
        1,
        "100 nF sensor bypass (SHT45 or mic)",
        AVL,
        AVL,
        "±10 %",
        "16 V X7R",
        "0402 [1005]",
        Fit,
        None,
        MpnPending
    ),
    line!(
        "C10",
        Patch,
        1,
        "1 µF TPS22918 VIN",
        AVL,
        AVL,
        "±10 %",
        "10 V X7R",
        "0603 [1608]",
        Fit,
        None,
        MpnPending
    ),
    line!(
        "C11",
        Patch,
        1,
        "1 nF TPS22918 CT",
        AVL,
        AVL,
        "±10 %",
        "16 V X7R",
        "0402 [1005]",
        Fit,
        None,
        MpnPending
    ),
    line!(
        "C12-C15",
        Patch,
        4,
        "1 µF TPS2553 IN",
        AVL,
        AVL,
        "±10 %",
        "10 V X7R",
        "0603 [1608]",
        Fit,
        None,
        MpnPending
    ),
    line!(
        "C16-C19",
        Patch,
        4,
        "10 µF AUXn_VCC",
        AVL,
        AVL,
        "±20 %",
        "10 V X5R",
        "0805 [2012]",
        Fit,
        None,
        MpnPending
    ),
    line!(
        "C20-C23",
        Patch,
        4,
        "100 nF at each J_AUX VCC pin",
        AVL,
        AVL,
        "±10 %",
        "16 V X7R",
        "0402 [1005]",
        Fit,
        None,
        MpnPending
    ),
];

/// Total placements per board.
pub fn placements(b: &[BomLine]) -> u32 {
    b.iter().map(|l| l.qty as u32).sum()
}

/// System quantity of an MPN for `patches` patches (counting fitted lines only, both variants counted once each).
pub fn system_qty(mpn: &str, patches: u32) -> u32 {
    let m: u32 = MAIN_BOM
        .iter()
        .filter(|l| l.mpn == mpn && l.populate != Populate::Dni)
        .map(|l| l.qty as u32)
        .sum();
    let p: u32 = PATCH_BOM
        .iter()
        .filter(|l| l.mpn == mpn && l.populate == Populate::Fit)
        .map(|l| l.qty as u32)
        .sum();
    m + p * patches
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn refdes_unique_per_board_and_specs_exist() {
        for bom in [MAIN_BOM, PATCH_BOM] {
            let mut s = HashSet::new();
            for l in bom {
                assert!(s.insert(l.refdes), "dup {}", l.refdes);
                assert!(l.qty > 0);
                if let Some(k) = l.spec {
                    let sp = crate::component_specs::spec(k).unwrap_or_else(|| panic!("{k}"));
                    assert_eq!(sp.mpn, l.mpn, "{}", l.refdes);
                }
            }
        }
    }

    #[test]
    fn bom_values_match_calculations() {
        let has = |b: &[BomLine], v: &str| b.iter().any(|l| l.value.starts_with(v));
        assert!(
            has(MAIN_BOM, "1.78 kΩ")
                && has(MAIN_BOM, "1.62 kΩ")
                && has(MAIN_BOM, "68.1 kΩ")
                && !has(MAIN_BOM, "66.5 kΩ")
        );
        assert!(
            has(PATCH_BOM, "93.1 kΩ")
                && has(PATCH_BOM, "22.0 kΩ")
                && has(PATCH_BOM, "68.0 kΩ")
                && has(PATCH_BOM, "47 kΩ")
        );
        assert_eq!(crate::passives::R_ILIM_CHAIN_K, 76.8);
        assert!(has(PATCH_BOM, "267 kΩ") && has(PATCH_BOM, "47 kΩ PATCH_OE_N"));
        assert_eq!(crate::passives::R_ILIM_PORT_K, 93.1);
    }

    #[test]
    fn system_quantities() {
        assert_eq!(system_qty("FH34SRJ-12S-0.5SH(50)", 8), 17);
        assert_eq!(system_qty("TPS2553DBVR-1", 8), 33);
        assert_eq!(system_qty("SM04B-GHS-TB(LF)(SN)", 8), 32);
        assert_eq!(system_qty("TPS62840DLCR", 8), 8);
        assert_eq!(system_qty("TPS22918DBVR", 8), 9);
        assert!(placements(PATCH_BOM) > 70);
    }

    #[test]
    fn every_patch_has_exactly_one_primary_variant_pair() {
        assert_eq!(
            PATCH_BOM
                .iter()
                .filter(|l| l.populate == Populate::TrhVariant && l.spec.is_some())
                .count(),
            1
        );
        assert_eq!(
            PATCH_BOM
                .iter()
                .filter(|l| l.populate == Populate::MicVariant && l.spec.is_some())
                .count(),
            1
        );
    }
}
