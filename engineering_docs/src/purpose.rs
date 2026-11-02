//! # 1. Project purpose and requirements
//!
//! BeeBox V2 is a modular sensing platform. It places measurement functions at selected
//! points around a honeybee colony/hive. One ESP32-S3 main controller discovers, selects and
//! reads a daisy chain of up to eight passive sensor patches. Each patch carries one primary
//! sensor and four auxiliary module ports (p.1, p.3).
//!
//! ## Product boundary (p.3)
//! No validated bee-health diagnosis, colony scoring, methane/CO/CO2 interpretation, cloud
//! analytics or field performance is claimed. Those are separate product requirements.

use crate::model::{Requirement, Source, Status};

const CLIENT: Source = Source::Client { date: "2026-11-01" };
const fn p(page: u8) -> Source {
    Source::Pdf { page }
}

/// Maximum number of sensor patches in one physical chain.
pub const MAX_PATCHES: usize = 8;
/// Auxiliary module ports on **every** patch (client-confirmed, not open).
pub const AUX_PORTS_PER_PATCH: usize = 4;
/// Total AUX ports on a fully populated chain.
pub const MAX_AUX_PORTS_PER_SYSTEM: usize = MAX_PATCHES * AUX_PORTS_PER_PATCH;

/// Design principles (p.3).
pub const DESIGN_PRINCIPLES: &[(&str, &str)] = &[
    ("Centralized control", "Patches contain no MCU and no battery; small logic, switching and power devices are allowed."),
    ("One active path", "Activate and read one patch / one external-module path at a time unless concurrency is proven."),
    ("Modularity", "Repeatable patch PCB with clearly labelled ports and replaceable sensor modules."),
    ("Fail-safe", "All sensor power and signal paths default OFF during reset, shift-register updates and faults."),
    ("Evidence-led release", "Calculations, ERC/DRC, netlist checks and measurements are required before fabrication release."),
];

/// Primary sensor class fitted to a patch. Exactly one per patch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimarySensor {
    /// Temperature / relative humidity (SHT45-class), I2C.
    TemperatureHumidity,
    /// Digital I2S MEMS microphone.
    I2sMicrophone,
}

/// Target external-module gas classes named by the client or source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuxTargetClass {
    /// Methane (source p.9, client) - no verified I2C module yet (research).
    Methane,
    /// Carbon monoxide (source p.9; not repeated in the latest client instruction).
    CarbonMonoxide,
    /// Carbon dioxide (client instruction 2026-11-01).
    CarbonDioxide,
    /// Other user-selected environmental modules, subject to the compatibility list.
    OtherUserSelected,
}

/// Confirmed requirements. `ClientConfirmed` beats `SourceBaseline` on conflict.
pub const REQUIREMENTS: &[Requirement] = &[
    Requirement { id: "REQ-SYS-01", title: "Main controller", statement: "The system shall have exactly one ESP32-S3-based main controller board.", status: Status::ClientConfirmed, sources: &[CLIENT, p(4)], verified_by: &["V-01"] },
    Requirement { id: "REQ-SYS-02", title: "Chain length", statement: "The system shall support a daisy chain of up to eight passive sensor patches (Main -> Patch 1 -> ... -> Patch 8).", status: Status::ClientConfirmed, sources: &[CLIENT, p(4)], verified_by: &["V-02", "V-11"] },
    Requirement { id: "REQ-SYS-03", title: "Passive patches", statement: "Patches shall contain no MCU and no battery. Shift-register, analog switch/mux, ID and load-switch circuitry is permitted.", status: Status::ClientConfirmed, sources: &[CLIENT, p(4)], verified_by: &["V-01"] },
    Requirement { id: "REQ-SYS-04", title: "Primary sensor", statement: "Each patch shall carry either a T/RH sensor or an I2S microphone as primary sensor, never both live.", status: Status::ClientConfirmed, sources: &[CLIENT, p(4), p(8)], verified_by: &["V-03", "V-07", "V-08"] },
    Requirement { id: "REQ-AUX-01", title: "Four AUX headers per patch", statement: "Every sensor patch shall provide four auxiliary headers AUX1..AUX4 (J_AUX1..J_AUX4), 32 per system, for ready-made external I2C breakout modules such as CO2, methane and other environmental sensors.", status: Status::ClientConfirmed, sources: &[CLIENT, p(4), p(9)], verified_by: &["V-04", "V-05", "V-09"] },
    Requirement { id: "REQ-AUX-07", title: "AUX I2C pinout", statement: "Each AUX header shall be a standard 4-pin I2C header: 1 VCC, 2 GND, 3 SDA, 4 SCL. The ESP32-S3 communicates with AUX modules through the existing sensor chain.", status: Status::ClientConfirmed, sources: &[CLIENT], verified_by: &["V-04", "V-09", "V-12"] },
    Requirement { id: "REQ-AUX-08", title: "Verified compatibility only", statement: "Only modules whose interface and electrical specifications are verified compatible (I2C, supply, logic level, current, power-cycling) shall be supported.", status: Status::ClientConfirmed, sources: &[CLIENT], verified_by: &["V-09"] },
    Requirement { id: "REQ-SYS-07", title: "Frozen safety rules", statement: "No 5 V signal shall reach the ESP32-S3; no SPI on AUX; GPIO38 shall not be used for patch identification.", status: Status::ClientConfirmed, sources: &[CLIENT, p(6), p(10)], verified_by: &["V-06"] },
    Requirement { id: "REQ-CH-01", title: "12-contact chain baseline", statement: "The existing 12-contact chain interface (crate::interfaces::CHAIN_PINS) is the baseline for FPC and pogo builds.", status: Status::ClientConfirmed, sources: &[CLIENT, p(4), p(7)], verified_by: &["V-12"] },
    Requirement { id: "REQ-CH-02", title: "Preserve cascade and ID", statement: "The shift-register cascade and the patch identification architecture shall be preserved unless engineering analysis demonstrates a necessary change (which then requires client approval).", status: Status::ClientConfirmed, sources: &[CLIENT, p(7), p(11)], verified_by: &["V-02", "V-03"] },
    Requirement { id: "REQ-SYS-05", title: "Fail-safe defaults", statement: "All sensor and AUX power and signal paths shall default OFF during reset, shift-register updates and fault conditions.", status: Status::SourceBaseline, sources: &[p(3)], verified_by: &["V-01", "V-13"] },
    Requirement { id: "REQ-SYS-06", title: "One active path", statement: "Only one sensor/AUX path shall be active at a time system-wide; AUX ports shall never operate in parallel.", status: Status::ClientConfirmed, sources: &[CLIENT, p(3), p(9)], verified_by: &["V-04"] },
    Requirement { id: "REQ-AUX-02", title: "No parallel AUX ports", statement: "AUX headers shall not be wired in parallel to a shared bus; a selected-port switch/mux is required.", status: Status::SourceBaseline, sources: &[p(5), p(10)], verified_by: &["V-04", "V-10"] },
    Requirement { id: "REQ-AUX-03", title: "Controlled AUX power", statement: "AUX module power shall be controlled with a safe default-off state; a per-selected-port load switch is preferred.", status: Status::SourceBaseline, sources: &[p(10)], verified_by: &["V-05"] },
    Requirement { id: "REQ-AUX-04", title: "AUX protection", statement: "AUX ports shall have ESD/transient protection on VCC, SDA and SCL sized to user-connector exposure.", status: Status::SourceBaseline, sources: &[p(10)], verified_by: &["V-06", "V-10"] },
    Requirement { id: "REQ-AUX-05", title: "Compatibility list", statement: "Only modules on an approved compatibility list (exact MPN, supply, current, protocol, logic level, warm-up, calibration, output limits) are supported.", status: Status::SourceBaseline, sources: &[p(9), p(15)], verified_by: &["V-09"] },
    Requirement { id: "REQ-AUX-06", title: "AUX labelling", statement: "Each AUX port shall be individually identified in schematic and silkscreen; release documents state pinout, voltage, max current per port and warnings.", status: Status::SourceBaseline, sources: &[p(8), p(19)], verified_by: &["V-14"] },
    Requirement { id: "REQ-MB-01", title: "Display bus", statement: "The display shall use a separate 4-wire I2C connector (GND, +3V3_DISPLAY, DISPLAY_SCL, DISPLAY_SDA) independent of the sensor bus.", status: Status::SourceBaseline, sources: &[p(4), p(6)], verified_by: &[] },
    Requirement { id: "REQ-MB-02", title: "Debug access", statement: "USB or UART/JTAG programming/debug and test pads shall be independent of the sensor chain.", status: Status::SourceBaseline, sources: &[p(6)], verified_by: &[] },
    Requirement { id: "REQ-MB-03", title: "ADC1 ID pin", statement: "SENSOR_DATA_ID shall be on an ADC1-capable GPIO that also supports I2S DIN; GPIO38 is not acceptable.", status: Status::SourceBaseline, sources: &[p(6), p(11)], verified_by: &["V-03", "V-08"] },
    Requirement { id: "REQ-PWR-01", title: "Battery", statement: "Baseline supply is a 1-cell LiPo; a buck-only 3.3 V regulator is unacceptable unless a validated cutoff maintains regulation.", status: Status::SourceBaseline, sources: &[p(12)], verified_by: &["V-11"] },
    Requirement { id: "REQ-PCB-01", title: "Orientation", statement: "FPC orientation, pin 1 and IN/OUT marking shall be unmistakable and verified by 1:1 print and continuity fixture.", status: Status::SourceBaseline, sources: &[p(12)], verified_by: &["V-12"] },
];
