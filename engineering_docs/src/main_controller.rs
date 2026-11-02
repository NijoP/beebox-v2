//! # 3. Main controller
//!
//! One ESP32-S3-based board (client-confirmed). It owns the chain control image, the
//! identification ADC, sensor-bus mode switching, the display, power supervision and debug.
//!
//! The exact module MPN, flash/PSRAM and GPIO map are **open**. They depend on the
//! data-destination decision ([`crate::approvals`] APR-4) and the final pin allocation.

use crate::model::{Source, Status};

/// A design gate that must close before the main-board schematic is frozen.
#[derive(Debug, Clone, Copy)]
pub struct DesignGate {
    /// Gate text.
    pub gate: &'static str,
    /// Status.
    pub status: Status,
    /// Provenance.
    pub source: Source,
}

/// Main-board design gates (p.6, p.11).
pub const DESIGN_GATES: &[DesignGate] = &[
    DesignGate { gate: "Reserve an ADC1-capable GPIO for SENSOR_DATA_ID (GPIO38 rejected).", status: Status::SourceBaseline, source: Source::Pdf { page: 6 } },
    DesignGate { gate: "Bench-prove the same pad works as ADC1 input and I2S DIN on the exact module.", status: Status::Open, source: Source::Pdf { page: 11 } },
    DesignGate { gate: "Confirm GPIO output levels, strapping pins, USB/debug needs and antenna keep-out for the exact module.", status: Status::Open, source: Source::Pdf { page: 6 } },
    DesignGate { gate: "Keep display pull-ups and display power independent from the sensor-chain bus.", status: Status::SourceBaseline, source: Source::Pdf { page: 6 } },
    DesignGate { gate: "Do not freeze the power stage until battery range, maximum patch load and AUX power plan are calculated.", status: Status::SourceBaseline, source: Source::Pdf { page: 6 } },
];

/// Planned main-board schematic sheets (p.14).
pub const SHEETS: &[(&str, &str)] = &[
    ("M01 Power input", "Battery connector, protection, charge interface if selected, regulator(s), power test points."),
    ("M02 ESP32-S3 core", "Module, boot/reset, programming/debug, decoupling, GPIO map."),
    ("M03 Chain control + ADC", "Shift data/clock/latch/OE, ID ADC with 100 k pull-down, safe-state pulls, test points."),
    ("M04 12-pin FPC / pogo", "Exact 12-pin map, orientation, labels, continuity points."),
    ("M05 Display + status", "Independent 4-wire I2C display, status LED, test points."),
];

/// Engineering proposals for the main board (not approved).
pub const PROPOSALS: &[(&str, &str)] = &[
    ("PRP-MB-01", "External pull-up on PATCH_OE_N so all patch outputs are disabled before firmware runs."),
    ("PRP-MB-02", "Firmware-switchable (GPIO-controlled) I2C pull-ups on SENSOR_CLK/SENSOR_SYNC, disabled in I2S mode, so push-pull I2S does not fight static pull-ups."),
    ("PRP-MB-03", "Series source-termination resistors on all four chain control outputs and SENSOR_CLK."),
    ("PRP-MB-04", "No level translation anywhere: chain and AUX are 3.3 V only (aux_expansion::RECOMMENDED_VOLTAGE_POLICY)."),
    ("PRP-MB-05", "Sensor-bus I2C pull-up value chosen from the mux RON budget (calculations::r_pullup_max_ohm) and bus capacitance; default 100 kHz."),
    ("PRP-MB-06", "+3V3_CHAIN fed through a current-limited switch on the main board sized from calculations::Chain per-position results."),
];
