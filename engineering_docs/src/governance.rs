//! # 13. Risks, assumptions and engineering decisions
//!
//! Decision-log rule: never edit an entry's meaning. Supersede it with a new entry. Client
//! decisions and recommendations are in [`crate::approvals`].

use crate::model::{Assumption, Decision, Priority, Risk, Source, Status};

const CLIENT: Source = Source::Client { date: "2026-11-01" };
const fn p(page: u8) -> Source {
    Source::Pdf { page }
}

/// Decision log.
pub const DECISIONS: &[Decision] = &[
    Decision { id: "DEC-001", topic: "Topology", statement: "One ESP32-S3 main controller + up to 8 passive, batteryless, MCU-free patches in one daisy chain.", status: Status::ClientConfirmed, rationale: "Client confirmation; source baseline.", sources: &[CLIENT, p(4)] },
    Decision { id: "DEC-002", topic: "Chain interface", statement: "12-contact logical interface per interfaces::CHAIN_PINS for FPC and pogo builds; preserved.", status: Status::ClientConfirmed, rationale: "Baseline confirmed.", sources: &[CLIENT, p(7)] },
    Decision { id: "DEC-003", topic: "Cascade", statement: "Pin 8 passes through each patch's 74HC595 (DS in, QH' out); preserved.", status: Status::ClientConfirmed, rationale: "Positional addressing without patch MCU.", sources: &[CLIENT, p(7)] },
    Decision { id: "DEC-004", topic: "Primary sensor", statement: "Exactly one of T/RH or I2S mic per patch.", status: Status::ClientConfirmed, rationale: "Shared bus; ID scheme.", sources: &[CLIENT, p(8)] },
    Decision { id: "DEC-005", topic: "AUX ports", statement: "Four AUX headers J_AUX1..J_AUX4 on every patch (32 per system).", status: Status::ClientConfirmed, rationale: "Client requirement.", sources: &[CLIENT, p(9)] },
    Decision { id: "DEC-006", topic: "AUX isolation", statement: "AUX ports never operate in parallel; one active sensor/AUX path system-wide.", status: Status::ClientConfirmed, rationale: "Client freeze; source p.5/p.9.", sources: &[CLIENT, p(5)] },
    Decision { id: "DEC-007", topic: "Patch ID", statement: "Switched ID resistor vs 100 k pull-down on ADC1 (22 k T/RH, 68 k mic) preserved; windows provisional.", status: Status::ClientConfirmed, rationale: "Preserve ID architecture.", sources: &[CLIENT, p(11)] },
    Decision { id: "DEC-008", topic: "ADC pin", statement: "GPIO38 not used for patch identification.", status: Status::Rejected, rationale: "Client freeze; source p.6.", sources: &[CLIENT, p(6)] },
    Decision { id: "DEC-009", topic: "Mic", statement: "ICS-43434 not used.", status: Status::Rejected, rationale: "EOL.", sources: &[p(15)] },
    Decision { id: "DEC-010", topic: "Regulator", statement: "Buck-only 3.3 V regulator from 1S LiPo without validated cutoff not acceptable.", status: Status::SourceBaseline, rationale: "Headroom at low cell voltage.", sources: &[p(12)] },
    Decision { id: "DEC-011", topic: "No SPI / no 5 V to ESP32-S3", statement: "No SPI on AUX; no 5 V signal may reach the ESP32-S3; all outputs default OFF at reset and faults.", status: Status::ClientConfirmed, rationale: "Client freeze.", sources: &[CLIENT, p(10)] },
    Decision { id: "DEC-012", topic: "Documentation", statement: "Engineering documentation maintained as this Rust crate; Markdown KB superseded and read-only.", status: Status::ClientConfirmed, rationale: "Client instruction.", sources: &[CLIENT] },
    Decision { id: "DEC-013", topic: "Shift-register expansion", statement: "Second 74HC595 per patch.", status: Status::Superseded, rationale: "Superseded by DEC-016: I2C-only AUX fits in Q3-Q7.", sources: &[Source::Analysis] },
    Decision { id: "DEC-014", topic: "AUX routing (generic)", statement: "PRP-ROUTE-01 analog/UART/PWM AUX routing incl. analog to pin 7.", status: Status::Superseded, rationale: "Superseded by client correction: AUX is I2C-only (DEC-015).", sources: &[CLIENT] },
    Decision { id: "DEC-015", topic: "AUX interface", statement: "Each AUX header is a standard 4-pin I2C header: 1 VCC, 2 GND, 3 SDA, 4 SCL, for ready-made I2C breakouts.", status: Status::ClientConfirmed, rationale: "Client correction 2026-11-01.", sources: &[CLIENT] },
    Decision { id: "DEC-017", topic: "System-wide single path", statement: "Exactly one active path across all 8 patches: hardware exclusivity per patch, ActivePath firmware construction, AUX-armed hardware sense before power and connection.", status: Status::Proposal, rationale: "Client requirement; backbone has no free contact for hardware prevention.", sources: &[CLIENT, Source::Analysis] },
    Decision { id: "DEC-018", topic: "Switch selection", statement: "TMUX1109 + TMUX136; CD74HC4052 and PCA9546A rejected.", status: Status::Proposal, rationale: "R_ON and capacitance budgets (calculations).", sources: &[Source::Analysis] },
    Decision { id: "DEC-019", topic: "Chain power (APR-1)", statement: "Pin 2 = +5V_CHAIN from TPS61023 + TPS2553 (76.8 k); TPS62840 +3V3_LOCAL on every patch.", status: Status::Approved, rationale: "Client approval subject to validation; resolves far-patch brown-out.", sources: &[CLIENT, Source::Analysis] },
    Decision { id: "DEC-020", topic: "AUX connector (APR-2)", statement: "JST-GH 1.25 mm 4-pin, VCC-GND-SDA-SCL, keyed adapter cables.", status: Status::Approved, rationale: "Client approval; MPN verification pending.", sources: &[CLIENT] },
    Decision { id: "DEC-021", topic: "Battery", statement: "3000 mAh LiPo (capacity confirmed; specification pending).", status: Status::ClientConfirmed, rationale: "Client requirement.", sources: &[CLIENT] },
    Decision { id: "DEC-016", topic: "AUX architecture", statement: "REC-01..REC-05: AUX I2C on pins 4/5, 595-controlled mux + power decoder in Q3-Q6, 3.3 V VCC, address-scan identification.", status: Status::Proposal, rationale: "See aux_expansion and approvals.", sources: &[Source::Analysis] },
];

/// Risk register.
pub const RISKS: &[Risk] = &[
    Risk { id: "RISK-01", title: "AUX module voltage/current unknown", impact: "Module damage or chain brown-out", mitigation: "Approved-profile list per breakout MPN; 3.3 V only; per-port current limit", priority: Priority::Critical, origin: p(17) },
    Risk { id: "RISK-02", title: "Four ports tied together", impact: "Contention, back-power", mitigation: "REC-02 hardware 1-of-4 selection + interlock; V-04/V-10", priority: Priority::Critical, origin: p(17) },
    Risk { id: "RISK-06", title: "AUX module ID", impact: "Wrong profile applied", mitigation: "REC-04 address + device-ID check; unknown -> power down", priority: Priority::Medium, origin: p(17) },
    Risk { id: "RISK-08", title: "Shared ADC/I2S GPIO fails", impact: "Patch ID or audio impossible", mitigation: "Bench test; mux or GPIO change", priority: Priority::Critical, origin: p(17) },
    Risk { id: "RISK-09", title: "Mic lifecycle gap", impact: "Obsolescence", mitigation: "Current-production mic + alternate", priority: Priority::High, origin: p(17) },
    Risk { id: "RISK-10", title: "Long-chain signal degradation", impact: "Unreliable I2C/I2S", mitigation: "Full 8-patch scope tests; 100 kHz default", priority: Priority::Critical, origin: p(17) },
    Risk { id: "RISK-11", title: "1S LiPo buck headroom", impact: "Chain rail drops", mitigation: "Buck-boost or validated cutoff", priority: Priority::High, origin: p(17) },
    Risk { id: "RISK-12", title: "Connector orientation mirrored", impact: "Power/data reversed", mitigation: "Drawing review, pin-1, fixture", priority: Priority::Critical, origin: p(17) },
    Risk { id: "RISK-13", title: "Mid-chain patch removal", impact: "Downstream unreachable", mitigation: "Documented limitation", priority: Priority::Medium, origin: p(17) },
    Risk { id: "RISK-B01", title: "Mux RON + pull-up current raise I2C low level", impact: "ESP32-S3 / modules misread zeros", mitigation: "Low-RON mux; weak pull-ups; check against ESP32-S3 VIL and module VIL (SCD41 0.3 x VDD); V-07/V-11", priority: Priority::High, origin: Source::Analysis },
    Risk { id: "RISK-B02", title: "Same issue on existing primary 4066 path", impact: "SHT45 I2C margin", mitigation: "Apply same RON budget to primary switch selection", priority: Priority::High, origin: Source::Analysis },
    Risk { id: "RISK-B03", title: "SCD41 205 mA peaks over long chain", impact: "Supply instability (Sensirion: supply must be constant, 30 mV ripple unloaded)", mitigation: "Per-port local bulk capacitance; measure at patch 8; consider STCC4 for low power", priority: Priority::High, origin: Source::Analysis },
    Risk { id: "RISK-B04", title: "Breakout boards with LDO/level shifters at 3.3 V input", impact: "Dropout or wrong logic level", mitigation: "Approve per breakout schematic, not per sensor IC", priority: Priority::High, origin: Source::Analysis },
    Risk { id: "RISK-B05", title: "No verified I2C methane module", impact: "Methane use case not deliverable on AUX", mitigation: "APR-3; continue survey with I2C/3.3 V/power-cycle criteria", priority: Priority::High, origin: Source::Analysis },
    Risk { id: "RISK-B06", title: "Reverse or mis-ordered plug-in (pin order differs from Qwiic/Grove)", impact: "Module damage, VCC short", mitigation: "Keyed connector (APR-2), labelled adapter cables, current-limited port switch", priority: Priority::High, origin: Source::Analysis },
    Risk { id: "RISK-B07", title: "Single +3V3_CHAIN contact (0.5 A rated)", impact: "Overheating / drop at patch 8", mitigation: "400 mA design limit; measure drop", priority: Priority::Critical, origin: Source::Analysis },
    Risk { id: "RISK-B08", title: "Scan latency with power-cycled modules", impact: "Long update interval (>=160 s for 32 SCD41)", mitigation: "Sampling plan per APR-4; prefer fast-start sensors", priority: Priority::Medium, origin: Source::Analysis },
    Risk { id: "RISK-B09", title: "Stuck I2C bus from a faulty module", impact: "Chain sensor bus blocked", mitigation: "Clear AUX bits (mux opens) + port power-off; firmware bus recovery", priority: Priority::Medium, origin: Source::Analysis },
    Risk { id: "RISK-B10", title: "Hi-Z 595 outputs while OE_N high", impact: "Unintended enable", mitigation: "Pull resistors to OFF on all enable nets; OE_N pull-up", priority: Priority::High, origin: Source::Analysis },
];

/// Assumption register.
pub const ASSUMPTIONS: &[Assumption] = &[
    Assumption { id: "AS-02", statement: "The 12-contact pinout stays unchanged with AUX expansion.", origin: Source::Analysis, validation: "Holds for I2C-only 3.3 V AUX (REC-01)" },
    Assumption { id: "AS-03", statement: "Primary sensors run from +3V3_CHAIN.", origin: p(8), validation: "SHT45 / mic datasheets" },
    Assumption { id: "AS-04", statement: "Supply is a 1S LiPo.", origin: p(12), validation: "APR-4" },
    Assumption { id: "AS-05", statement: "ESP32-S3 can switch one pad between ADC1 and I2S DIN at runtime.", origin: p(11), validation: "Bench test (RISK-08)" },
    Assumption { id: "AS-06", statement: "Patch 1 is nearest the main board; its register bits are shifted last.", origin: Source::Analysis, validation: "V-02" },
    Assumption { id: "AS-07", statement: "Gas outputs are raw measurements; no alarm/safety function or hazardous-area certification is intended.", origin: p(3), validation: "Client if a safety function is wanted" },
    Assumption { id: "AS-09", statement: "Base always-on load of 8 patches is small versus the 400 mA chain limit.", origin: Source::Analysis, validation: "Datasheet sum in P2, measurement in P6" },
    Assumption { id: "AS-10", statement: "Ready-made breakouts accepting 3.3 V VCC exist for the wanted sensors (verified for the SCD41/STCC4 sensor ICs only).", origin: Source::Analysis, validation: "Per-breakout review" },
];

/// Revision C risks (validation).
pub const RISKS_REV_C: &[Risk] = &[
    Risk {
        id: "RISK-C01",
        title: "Supply drop at far patches (FH34 100 mohm max x 16 contacts)",
        impact: "SCD41-class modules brown out beyond the first patches",
        mitigation: "APR-1; per-position rating; V-16",
        priority: Priority::Critical,
        origin: Source::Analysis,
    },
    Risk {
        id: "RISK-C02",
        title: "Bus capacitance with long segments",
        impact: "I2C rise-time violation above ~0.28 m segments",
        mitigation: "Segment-length limit (APR-4); V-11/V-17",
        priority: Priority::High,
        origin: Source::Analysis,
    },
    Risk {
        id: "RISK-C03",
        title: "Ground offset during AUX current raises VOL seen by ESP32-S3",
        impact: "Bit errors",
        mitigation: "Port-side pull-ups, 100 kHz, V-17",
        priority: Priority::High,
        origin: Source::Analysis,
    },
    Risk {
        id: "RISK-C04",
        title: "Corrupted shift image activates a second patch",
        impact: "Two ports powered briefly",
        mitigation: "Staged latches + arm-sense check + per-port current limit; V-18",
        priority: Priority::High,
        origin: Source::Analysis,
    },
    Risk {
        id: "RISK-C05",
        title: "FH34 20 mating cycles",
        impact: "Connector wear in service",
        mitigation: "Treat chain cabling as fixed; document service limits",
        priority: Priority::Medium,
        origin: Source::Analysis,
    },
    Risk {
        id: "RISK-C06",
        title: "FFC cable data not verified",
        impact: "Drop and capacitance results shift",
        mitigation: "Select cable, obtain datasheet, re-run calculations",
        priority: Priority::High,
        origin: Source::Analysis,
    },
];

/// Revision D risks.
pub const RISKS_REV_D: &[Risk] = &[
    Risk { id: "RISK-D01", title: "Cable reversal puts +5V_CHAIN on PATCH_OE_N (pin 2 <-> 11)", impact: "5 V on ESP32-S3 GPIO and patch HC inputs", mitigation: "Orientation fixture V-12; series R + clamp on main chain signals; chain current limit", priority: Priority::Critical, origin: Source::Analysis },
    Risk { id: "RISK-D02", title: "Back-power of unpowered patches from main-board GPIOs", impact: "Undefined patch state, leakage, latch-up", mitigation: "Chain GPIOs low/high-Z before chain off; OE_N pull-up on patches only", priority: Priority::High, origin: Source::Analysis },
    Risk { id: "RISK-D03", title: "Port short on a far patch trips the chain switch", impact: "Whole chain latches off", mitigation: "Fail-safe; firmware recovery and port quarantine", priority: Priority::Medium, origin: Source::Analysis },
    Risk { id: "RISK-D04", title: "Battery pack specification unknown", impact: "Charge voltage mismatch, insufficient discharge rating or protection", mitigation: "Pack datasheet (Q-1) before M01 schematic", priority: Priority::Critical, origin: Source::Analysis },
    Risk { id: "RISK-D05", title: "Switching regulator noise on 8 patches near I2C/I2S and the microphone", impact: "Audio noise, I2C errors", mitigation: "Layout rules, TPS62840 placement, V-08/V-17", priority: Priority::High, origin: Source::Analysis },
    Risk { id: "RISK-D06", title: "Ground offset 0.16-0.18 V at patch 8 raises I2C high level to ~3.5 V at the ESP32-S3", impact: "Within VDD + 0.3 V but small margin", mitigation: "V-17 measurement", priority: Priority::Medium, origin: Source::Analysis },
    Risk { id: "RISK-D07", title: "Power-cycled SCD41 single-shot every minute", impact: "Accuracy/ASC behaviour and energy per shot not characterised (135 mA.s is a derived estimate)", mitigation: "Bench energy and accuracy test", priority: Priority::High, origin: Source::Analysis },
];
