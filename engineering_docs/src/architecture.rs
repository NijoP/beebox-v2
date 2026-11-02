//! # 2. System architecture
//!
//! ```text
//!                 12-contact FPC / pogo chain (one logical pinout)
//! +---------------+   +---------+   +---------+         +---------+
//! | MAIN (ESP32-S3)|-->| PATCH 1 |-->| PATCH 2 |-- ... -->| PATCH 8 |
//! | ADC/I2C/I2S    |   | 595+mux |   | 595+mux |         | 595+mux |
//! | display, debug |   | ID+sens |   | ID+sens |         | ID+sens |
//! +---------------+   | AUX1..4 |   | AUX1..4 |         | AUX1..4 |
//!                     +---------+   +---------+         +---------+
//! ```
//! *One patch / one AUX path active at a time until concurrency is proven* (p.5, Fig. 1).
//!
//! - **Position** of a patch = its place in the serial shift-register cascade.
//! - **Type** = switched ID resistor read by the main ADC, or a future module profile (p.11).
//! - AUX ports are local to each patch and **do not add chain contacts** (p.5).

use crate::model::{Source, Status};

/// Top-level functional blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Block {
    /// ESP32-S3 controller board.
    MainController,
    /// 12-contact chain (cables/pogo + connectors).
    Chain,
    /// Patch base: FPC IN/OUT, register, power switch, isolation, ID, primary sensor.
    PatchBase,
    /// Four-port AUX expansion on each patch.
    AuxExpansion,
    /// Firmware and engineering tooling.
    FirmwareAndTools,
}

/// Functional partition entry.
#[derive(Debug, Clone, Copy)]
pub struct Partition {
    /// Block.
    pub block: Block,
    /// Responsibilities.
    pub responsibilities: &'static str,
    /// Provenance.
    pub source: Source,
}

/// Functional partition (p.5).
pub const PARTITION: &[Partition] = &[
    Partition { block: Block::MainController, responsibilities: "ESP32-S3 runtime control; shift-register control image; patch scanning; ADC identification; sensor-bus mode selection; display/status; power supervision; debug.", source: Source::Pdf { page: 5 } },
    Partition { block: Block::Chain, responsibilities: "Always-on patch logic supply, common grounds, three time-multiplexed sensor/data lines, four shift-register control/cascade signals.", source: Source::Pdf { page: 5 } },
    Partition { block: Block::PatchBase, responsibilities: "FPC IN/OUT; SN74HC595-class register; local sensor power switch; signal isolation; patch type ID; T/RH or I2S footprints.", source: Source::Pdf { page: 5 } },
    Partition { block: Block::AuxExpansion, responsibilities: "Four module headers, per-port power control, protection/conditioning, port selection/multiplexing, module identification.", source: Source::Pdf { page: 5 } },
    Partition { block: Block::FirmwareAndTools, responsibilities: "Safe state machine; drivers; module profiles; controlled EasyEDA scripts; independent validation records.", source: Source::Pdf { page: 5 } },
];

/// Architectural rules that every design artefact must obey.
pub const ARCHITECTURE_RULES: &[(&str, Status, Source)] = &[
    ("Never wire the four AUX headers in parallel to one bus; use a selected-port switch/mux.", Status::SourceBaseline, Source::Pdf { page: 5 }),
    ("AUX expansion does not change the 12-contact chain contact count at this stage.", Status::SourceBaseline, Source::Pdf { page: 9 }),
    ("Display I2C bus and its pull-ups are separate from the sensor-chain bus.", Status::SourceBaseline, Source::Pdf { page: 6 }),
    ("Any change to the cascade or ID architecture needs documented analysis and client approval.", Status::ClientConfirmed, Source::Client { date: "2026-11-01" }),
];
