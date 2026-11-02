//! # 8. Component selection and BOM
//!
//! **No component is selected.** Entries are source-named candidates with their qualification
//! criteria (p.15). Selection happens only after datasheet review, and is recorded as an approved decision.

use crate::model::Status;

/// A candidate part / concept.
#[derive(Debug, Clone, Copy)]
pub struct Candidate {
    /// Function.
    pub function: &'static str,
    /// Candidate named in the source.
    pub candidate: &'static str,
    /// Qualification criteria.
    pub qualify: &'static str,
    /// Reference tag(s).
    pub refs: &'static str,
    /// Status.
    pub status: Status,
}

/// Candidate list (p.7, p.12, p.15).
pub const CANDIDATES: &[Candidate] = &[
    Candidate { function: "Main MCU", candidate: "ESP32-S3 module (MPN open)", qualify: "Memory, antenna/keep-out, ADC1 + I2S DIN on same pad, supply peaks, lifecycle.", refs: "R6,R7", status: Status::Open },
    Candidate { function: "T/RH", candidate: "Sensirion SHT45", qualify: "Supply, accuracy, port placement, decoupling, humidity exposure, footprint.", refs: "R8", status: Status::Candidate },
    Candidate { function: "I2S microphone", candidate: "PUI DMM-4026-B-I2S family (one candidate)", qualify: "Lifecycle, BCLK limits, format, acoustic port, sample rate, full-chain timing.", refs: "R10,R11", status: Status::Candidate },
    Candidate { function: "I2S microphone (old baseline)", candidate: "TDK ICS-43434", qualify: "EOL per source.", refs: "R9", status: Status::Rejected },
    Candidate { function: "Shift register", candidate: "TI SN74HC595", qualify: "Supply range, output current, clock, QH' pin, SRCLR/OE behaviour, package.", refs: "R3", status: Status::Candidate },
    Candidate { function: "Primary signal switch", candidate: "CD74HC4066-class quad switch", qualify: "R_ON at 3.3 V, bandwidth, off leakage, I2S waveform, power-off isolation.", refs: "R4", status: Status::Candidate },
    Candidate { function: "Load switch", candidate: "TPS22918-class", qualify: "V_IN, current, turn-on, reverse blocking, discharge, default-off.", refs: "R5", status: Status::Candidate },
    Candidate { function: "AUX signal selector", candidate: "TI TMUX1109 (port mux) + TI TMUX136 (bus isolator); CD74HC4052 and PCA9546A rejected", qualify: "RON at 3.3 V vs calculations budget, off-isolation, capacitance, power-off behaviour, active-high/low enable.", refs: "-", status: Status::Open },
    Candidate { function: "AUX power", candidate: "TI TPS2553 (R_ILIM 93.1 kohm -> 252-324 mA) per port, enabled by CD74HC238 Y4..Y7; QOD variant to be checked", qualify: "Current limit >= approved module peak, <= chain allowance; output discharge; reverse-current blocking; default-off.", refs: "-", status: Status::Open },
    Candidate { function: "Chain connector", candidate: "Hirose FH34SRJ-12S-0.5SH(50)", qualify: "Drawing, land pattern, contact side, insertion, stiffener, bend, current.", refs: "R2", status: Status::Candidate },
    Candidate { function: "Pogo contacts", candidate: "Mill-Max 0906 series", qualify: "Exact MPN, compression, retention, cycle life.", refs: "R12", status: Status::Candidate },
    Candidate { function: "AUX headers (x4 per patch)", candidate: "4-pin VCC/GND/SDA/SCL; type per approval APR-2", qualify: "Pitch, keying, locking, current, insertion cycles, hive environment.", refs: "-", status: Status::Open },
    Candidate { function: "3.3 V regulator", candidate: "Buck-boost class (TPS62162 buck-only concept superseded)", qualify: "Full 1S range, peak current, quiescent current.", refs: "p.12", status: Status::Open },
];

/// BOM acceptance rules (p.15).
pub const BOM_RULES: &[&str] = &[
    "Every line: unique RefDes, manufacturer, exact MPN, quantity, value, tolerance, voltage/current rating, package, footprint.",
    "Verify symbol-to-footprint pin mapping and pad numbering against the manufacturer drawing.",
    "Check lifecycle and record at least one alternate for critical parts.",
    "Use 0404 metric passives only where suitable; 0404 metric != 0402 imperial; choose larger packages for capacitance, voltage, power, derating or yield.",
    "For external modules record MPN, supply voltage, max/average current, protocol, logic level, warm-up, calibration needs, output limits.",
];

/// Per-patch AUX bill of materials (candidates, revision C). Quantities per patch.
pub const AUX_BOM_PER_PATCH: &[(&str, u8, &str)] = &[
    ("TI TMUX1109 dual 4:1 mux", 1, "port selection"),
    ("TI TMUX136 2-ch switch", 1, "chain bus isolator"),
    (
        "TI CD74HC238 3-to-8 decoder (active-high)",
        1,
        "port power enable",
    ),
    (
        "TI CD74HC138 3-to-8 decoder (active-low)",
        1,
        "bus connect enable",
    ),
    ("TI TPS2553 current-limited switch", 4, "port VCC"),
    ("R_ILIM 93.1 kohm 1 %", 4, "current limit"),
    ("Pull-up 2.2 kohm", 8, "SDA/SCL per port to port VCC"),
    ("Pull-down (value TBD)", 5, "Q3-Q7 safe state"),
    ("Small-signal diode + 150 kohm", 1, "AUX-armed sense"),
    ("ESD array (TBD)", 4, "per header VCC/SDA/SCL"),
    ("4-pin header (APR-2)", 4, "J_AUX1..J_AUX4"),
];
