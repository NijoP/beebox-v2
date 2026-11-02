//! # 6. Electrical interfaces and pin assignments
//!
//! The 12-contact chain pinout is the **baseline** (p.7, client-confirmed). It replaces the
//! earlier repository assignment everywhere: schematics, netlists, PCB, firmware and docs.
//! The pinout is the same for FPC IN, FPC OUT, the main-board chain connector and the pogo variant.
//!
//! Pin numbers are **logical**. Map them to physical connector contacts only after checking the
//! manufacturer drawing and the FFC contact orientation (same-side vs opposite-side). A cable
//! flip mirrors 1<->12.

use crate::model::{Source, Status};

/// How a chain net passes through a patch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Routing {
    /// Copper connection IN pin n -> OUT pin n.
    PassThrough,
    /// IN pin -> local 74HC595 DS; local QH' (Q7S) -> OUT pin. Not a copper pass-through.
    DirectionalCascade,
}

/// Electrical class of a chain net.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetClass {
    /// Ground / return.
    Ground,
    /// Always-on supply.
    Power,
    /// Time-multiplexed sensor/data line.
    SharedSensor,
    /// Shift-register control or cascade.
    Control,
}

/// One contact of the 12-contact chain interface.
#[derive(Debug, Clone, Copy)]
pub struct ChainPin {
    /// Logical pin number 1..=12.
    pub pin: u8,
    /// Net name (exact spelling used in schematics/firmware).
    pub net: &'static str,
    /// Net class.
    pub class: NetClass,
    /// Function by operating mode.
    pub function: &'static str,
    /// Routing through a patch.
    pub routing: Routing,
}

/// Chain pinout - final revision D. Pin positions as p.7; pin 2 redefined to +5V_CHAIN (APR-1 option B).
/// **Do not edit without an approved change request.**
pub const CHAIN_PINS: [ChainPin; 12] = [
    ChainPin {
        pin: 1,
        net: "GND",
        class: NetClass::Ground,
        function: "Ground / return",
        routing: Routing::PassThrough,
    },
    ChainPin {
        pin: 2,
        net: "+5V_CHAIN",
        class: NetClass::Power,
        function: "Switched 5 V chain supply; each patch regulates +3V3_LOCAL (APR-1 option B)",
        routing: Routing::PassThrough,
    },
    ChainPin {
        pin: 3,
        net: "GND",
        class: NetClass::Ground,
        function: "Ground / return",
        routing: Routing::PassThrough,
    },
    ChainPin {
        pin: 4,
        net: "SENSOR_CLK",
        class: NetClass::SharedSensor,
        function: "I2C SCL in T/RH mode; I2S BCLK in mic mode",
        routing: Routing::PassThrough,
    },
    ChainPin {
        pin: 5,
        net: "SENSOR_SYNC",
        class: NetClass::SharedSensor,
        function: "I2C SDA in T/RH mode; I2S WS in mic mode",
        routing: Routing::PassThrough,
    },
    ChainPin {
        pin: 6,
        net: "GND",
        class: NetClass::Ground,
        function: "Ground / signal return",
        routing: Routing::PassThrough,
    },
    ChainPin {
        pin: 7,
        net: "SENSOR_DATA_ID",
        class: NetClass::SharedSensor,
        function: "I2S SD during mic capture; ADC ID voltage during discovery",
        routing: Routing::PassThrough,
    },
    ChainPin {
        pin: 8,
        net: "PATCH_SHIFT_DATA",
        class: NetClass::Control,
        function: "Shift data enters local DS; local QH'/Q7S exits to next patch",
        routing: Routing::DirectionalCascade,
    },
    ChainPin {
        pin: 9,
        net: "PATCH_SHIFT_CLK",
        class: NetClass::Control,
        function: "Shared shift-register clock (SRCLK)",
        routing: Routing::PassThrough,
    },
    ChainPin {
        pin: 10,
        net: "PATCH_LATCH",
        class: NetClass::Control,
        function: "Shared storage latch (RCLK)",
        routing: Routing::PassThrough,
    },
    ChainPin {
        pin: 11,
        net: "PATCH_OE_N",
        class: NetClass::Control,
        function: "Global active-low output enable",
        routing: Routing::PassThrough,
    },
    ChainPin {
        pin: 12,
        net: "GND",
        class: NetClass::Ground,
        function: "Ground / return",
        routing: Routing::PassThrough,
    },
];

/// Display connector, 4-wire I2C (p.4). Pin order is **not** specified by the source.
pub const DISPLAY_CONNECTOR_NETS: [&str; 4] = ["GND", "+3V3_DISPLAY", "DISPLAY_SCL", "DISPLAY_SDA"];

/// Main-board GPIO function groups (p.6). GPIO numbers are open until the ESP32-S3 module is selected.
#[derive(Debug, Clone, Copy)]
pub struct GpioGroup {
    /// Group name.
    pub group: &'static str,
    /// Nets.
    pub nets: &'static [&'static str],
    /// Constraint on the physical GPIO.
    pub constraint: &'static str,
    /// Assignment status.
    pub status: Status,
}

/// Required main-board GPIO groups.
pub const GPIO_GROUPS: &[GpioGroup] = &[
    GpioGroup { group: "Patch control", nets: &["PATCH_SHIFT_DATA", "PATCH_SHIFT_CLK", "PATCH_LATCH", "PATCH_OE_N"], constraint: "Push-pull outputs; OE_N must be high (outputs disabled) from power-on, independent of firmware -> external pull-up; avoid strapping pins.", status: Status::Open },
    GpioGroup { group: "Sensor clock/sync", nets: &["SENSOR_CLK", "SENSOR_SYNC"], constraint: "Must support I2C and I2S (BCLK/WS) via GPIO matrix. Also carry AUX I2C (SCL on pin 4, SDA on pin 5); switchable pull-ups.", status: Status::Open },
    GpioGroup { group: "Sensor data / ID", nets: &["SENSOR_DATA_ID"], constraint: "ADC1-capable AND usable as I2S DIN; GPIO38 rejected (p.6). Runtime peripheral switch must be bench-proven (p.11).", status: Status::SourceBaseline },
    GpioGroup { group: "Display", nets: &["DISPLAY_SCL", "DISPLAY_SDA"], constraint: "Separate I2C controller/pins with own pull-ups.", status: Status::SourceBaseline },
    GpioGroup { group: "Debug", nets: &["USB_D+", "USB_D-", "UART0_TX", "UART0_RX", "EN", "BOOT"], constraint: "Independent of chain; exact set depends on module and debug approach.", status: Status::Open },
    GpioGroup { group: "Power supervision", nets: &["VBAT_SENSE"], constraint: "Battery/rail monitoring method TBD (p.6).", status: Status::Open },
];

/// Source of the pinout for traceability.
pub const PINOUT_SOURCE: Source = Source::Pdf { page: 7 };

/// Candidate physical chain connector (not released): Hirose FH34SRJ-12S-0.5SH(50),
/// 12-pos, 0.5 mm pitch ZIF for 0.3 mm FPC/FFC (p.7, R2).
pub const CHAIN_CONNECTOR_CANDIDATE: &str = "Hirose FH34SRJ-12S-0.5SH(50) - candidate only; drawing, land pattern, contact side, insertion direction, stiffener, bend and current rating unverified";

/// Signal-integrity and cable requirements for the chain.
pub const CHAIN_SI_REQUIREMENTS: &[(&str, Status)] = &[
    ("Patch-8 rail voltage, I2C timing, I2S edge quality and module start-up shall pass at maximum intended chain length (V-11).", Status::SourceBaseline),
    ("Total chain length and per-segment FFC length are not defined; needed before SI analysis (APR-4).", Status::Open),
    ("PATCH_SHIFT_CLK/PATCH_LATCH/PATCH_OE_N are multi-drop over up to 8 segments; add series source termination at the main-board driver and evaluate Schmitt-input buffering if edges degrade.", Status::Proposal),
    ("Single +3V3_CHAIN contact carries all patch current: obtain contact current rating and FFC conductor resistance from manufacturer data; compute worst-case drop at patch 8.", Status::Proposal),
    ("Each patch adds switch off-capacitance on pins 4/5/7 (primary 4066 + AUX mux); prefer low-capacitance switches and verify I2S BCLK at patch 8.", Status::Proposal),
];

/// Electrical definition of one chain contact (revision D).
#[derive(Debug, Clone, Copy)]
pub struct PinElectrical {
    /// Pin.
    pub pin: u8,
    /// Driver -> receiver.
    pub direction: &'static str,
    /// Voltage domain / level.
    pub level: &'static str,
    /// Current / loading.
    pub current: &'static str,
}

/// Final 12-pin electrical interface (revision D). Approved: pin positions, +5V_CHAIN on pin 2.
/// Proposal: levels, current limits, protection.
pub const CHAIN_ELECTRICAL: [PinElectrical; 12] = [
    PinElectrical { pin: 1, direction: "-", level: "0 V reference", current: "Return; 4 GND contacts share <= 388 mA" },
    PinElectrical { pin: 2, direction: "Main -> patches", level: "+5V_CHAIN, 5.0 V at source; >= 3.6 V at any patch input (design criterion)", current: "<= 388 mA hardware limit (TPS2553 76.8 kohm); 400 mA provisional ceiling; FH34 0.5 A rated" },
    PinElectrical { pin: 3, direction: "-", level: "0 V reference", current: "Return" },
    PinElectrical { pin: 4, direction: "Bidirectional (I2C SCL) / main -> patch (I2S BCLK)", level: "3.3 V logic; never > 3.6 V at the ESP32-S3; pull-up to port VCC or main 3.3 V by mode", current: "I2C sink <= 3 mA" },
    PinElectrical { pin: 5, direction: "Bidirectional (I2C SDA) / main -> patch (I2S WS)", level: "3.3 V logic", current: "I2C sink <= 3 mA" },
    PinElectrical { pin: 6, direction: "-", level: "0 V reference", current: "Return" },
    PinElectrical { pin: 7, direction: "Patch -> main (I2S SD, ID voltage, AUX-armed sense)", level: "0-3.3 V; ADC1 input", current: "uA (ID/sense dividers); I2S driver" },
    PinElectrical { pin: 8, direction: "Main/previous QH' -> next DS", level: "3.3 V logic (re-timed per patch)", current: "CMOS input" },
    PinElectrical { pin: 9, direction: "Main -> all patches", level: "3.3 V logic, multi-drop (8 loads)", current: "CMOS inputs; series termination at source" },
    PinElectrical { pin: 10, direction: "Main -> all patches", level: "3.3 V logic, multi-drop", current: "CMOS inputs" },
    PinElectrical { pin: 11, direction: "Main -> all patches", level: "3.3 V logic, active-low; pulled up on each patch to +3V3_LOCAL", current: "CMOS inputs" },
    PinElectrical { pin: 12, direction: "-", level: "0 V reference", current: "Return" },
];

/// Fault analysis introduced by +5V_CHAIN (analysis; mitigations are proposals).
pub const CHAIN_FAULT_ANALYSIS: &[(&str, &str)] = &[
    ("Cable reversal / mirror (pin n <-> 13-n)", "Pin 2 (+5V) lands on pin 11 (PATCH_OE_N) -> 5 V on an ESP32-S3 GPIO and on patch HC inputs. Mitigation: FPC orientation fixture (V-12); series resistor + clamp to +3V3 on every main-board chain signal (value checked against I2C/I2S timing); main TPS2553 limits fault current."),
    ("Adjacent-pin short pin 2 - pin 1/3", "Both neighbours are GND: short is current-limited by the main TPS2553 (<= 388 mA) - no 5 V onto signals."),
    ("Chain unpowered while ESP32 drives control lines", "Back-power through HC input clamps. Mitigation: drive chain GPIOs low/high-Z before switching +5V_CHAIN off (power::CONSTRAINTS)."),
    ("Ground offset at patch 8 (SCD41 load)", "0.16-0.18 V: I2C low level at ESP32 <= 0.62 V (< 0.825 V VIL); high level up to ~3.48 V (<= VDD + 0.3 V). Verify in V-17."),
];
