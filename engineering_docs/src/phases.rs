//! # 14. Implementation phases and release criteria

/// Phase progress.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Progress {
    /// Not started.
    NotStarted,
    /// In progress.
    InProgress,
    /// Partly complete; see note.
    Partial,
    /// Blocked by approvals.
    Blocked,
    /// Complete with evidence.
    Complete,
}

/// An execution phase (p.18).
#[derive(Debug, Clone, Copy)]
pub struct Phase {
    /// Phase ID.
    pub id: &'static str,
    /// Name.
    pub name: &'static str,
    /// Work package.
    pub work: &'static str,
    /// Exit gate / deliverable.
    pub exit_gate: &'static str,
    /// Current progress.
    pub progress: Progress,
    /// Note.
    pub note: &'static str,
}

/// Phases P0-P8.
pub const PHASES: &[Phase] = &[
    Phase { id: "P0", name: "Baseline update", work: "Approve AUX scope, module families, patch outline, FPC lengths, battery and power envelope.", exit_gate: "Signed requirements + ICD; open questions tracked.", progress: Progress::InProgress, note: "Revision C validated architecture; awaiting APR-1..APR-4; sign-off not yet requested." },
    Phase { id: "P1", name: "Module survey", work: "2-4 representative methane/CO/CO2 module datasheets and MPNs; protocol/supply/current matrix.", exit_gate: "Approved shortlist + port compatibility table.", progress: Progress::Partial, note: "9 CO2/CH4 devices reviewed from datasheets (research); CO2 I2C classes verified; no I2C methane module verified; breakout-level approval pending." },
    Phase { id: "P2", name: "Architecture closure", work: "I2C AUX mux part, per-port switch, pull-ups/RON budget, chain drop, GPIO map.", exit_gate: "Updated architecture, GPIO/control map, power budget, net dictionary.", progress: Progress::Blocked, note: "Architecture recommended (REC-01..05); needs APR-1/APR-2, cable datasheet and bench tests V-15..V-18." },
    Phase { id: "P3", name: "API / tooling setup", work: "EasyEDA API Gateway; read-only smoke tests; confirm schema and target.", exit_gate: "Verified handshake + safe API test report.", progress: Progress::Partial, note: "Handshake and read-only query OK on 2026-11-01; formal report pending." },
    Phase { id: "P4", name: "Schematic capture", work: "Main and patch sheets as separate JS subsystems.", exit_gate: "Schematic blocks, component list, netlist, pin map.", progress: Progress::Blocked, note: "Not started by instruction." },
    Phase { id: "P5", name: "Independent audit", work: "Symbol/footprint mapping, connectivity, ERC, netlist parity.", exit_gate: "Audit report; no critical mismatch.", progress: Progress::NotStarted, note: "" },
    Phase { id: "P6", name: "Two-patch proof", work: "Main + T/RH patch + mic patch + >=1 AUX path.", exit_gate: "Measured prototype report; stop-gate pass.", progress: Progress::NotStarted, note: "" },
    Phase { id: "P7", name: "Eight-position validation", work: "Mixed population, missing tail, all ports, max cable, worst-case power.", exit_gate: "Eight-patch validation matrix with raw evidence.", progress: Progress::NotStarted, note: "" },
    Phase { id: "P8", name: "Controlled release", work: "Close critical/high risks; BOM, Gerbers, drill, PnP, drawings, firmware, fixture.", exit_gate: "Manufacturing release package under revision control.", progress: Progress::NotStarted, note: "" },
];

/// Release criteria (p.16-p.20).
pub const RELEASE_CRITERIA: &[&str] = &[
    "All Critical and High risks closed with evidence or formally accepted.",
    "V-01..V-14 passed with recorded artefacts at eight-patch scale.",
    "Every BOM line meets components::BOM_RULES; approved AUX module list published with per-port limits.",
    "Release package per manufacturing::RELEASE_PACKAGE complete and revision-controlled.",
    "No hardware decision in the release remains in Status::Proposal or Status::Open.",
];

/// Tooling status observations.
pub const TOOLING_STATUS: &[(&str, &str)] = &[
    ("2026-11-01", "EasyEDA bridge restarted (127.0.0.1:49620); handshake OK; read-only getCurrentProjectInfo -> 'BEE BOX V2'. No project modification."),
    ("2026-11-01", "Rust toolchain (stable, x86_64-pc-windows-gnu, minimal profile) installed in user profile for this crate."),
];
