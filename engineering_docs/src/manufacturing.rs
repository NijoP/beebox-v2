//! # 11. Manufacturing, PCB and EasyEDA project requirements

use crate::model::{DecisionOption, Status};

/// PCB and mechanical rules (p.12).
pub const PCB_RULES: &[&str] = &[
    "FPC connector orientation, pin-1 and IN/OUT silkscreen unmistakable; validate with 1:1 print and continuity fixture.",
    "Gas-sensor openings, acoustic ports and T/RH area clear of enclosure blockage and heat sources.",
    "Review condensation, dust, propolis, vibration, flex bend radius, connector strain relief, pogo compression/cycle life.",
    "Local decoupling at IC supply pins; sensor bypass from datasheets and inrush analysis.",
    "High-impedance analog away from switching power and fast digital; continuous ground return.",
    "Small footprints only where appropriate; capacitance, voltage rating, tolerance, derating and yield take priority.",
    "Freeze every footprint against the manufacturer package drawing (0404 metric != 0402 imperial).",
    "AUX headers individually labelled J_AUX1..J_AUX4 with voltage and max-current marking.",
];

/// Release package contents (p.19).
pub const RELEASE_PACKAGE: &[&str] = &[
    "Editable EasyEDA project with exact library component/device mappings.",
    "PDF schematics, layout, Gerbers, drill, pick-and-place, fabrication/assembly drawings, stack-up.",
    "BOM with exact MPNs, packages, alternates, lifecycle status.",
    "Canonical netlist, connector pin map, pin-to-pad verification report.",
    "ERC/DRC reports, design review sign-off, firmware build/version, test procedure, measured evidence.",
    "AUX header pinout/voltage labels, supported module list, max current per port, safety warnings.",
];

/// Project folder structure (p.19). Folders are created when their phase starts.
pub const FOLDERS: &[(&str, &str)] = &[
    (
        "docs/",
        "Source PDF archive and legacy (superseded) Markdown KB.",
    ),
    (
        "engineering_docs/",
        "This Rust documentation crate - authoritative engineering documentation.",
    ),
    ("schematic/", "EasyEDA schematic exports."),
    ("easyeda_scripts/", "One JS file per subsystem."),
    (
        "netlist/",
        "Canonical netlists, pin-to-net CSVs, comparison output.",
    ),
    (
        "bom/",
        "Controlled BOM, datasheets, module compatibility matrix.",
    ),
    ("pcb/", "PCB source and fabrication outputs."),
    ("firmware/", "Firmware source and test builds."),
    (
        "verification/",
        "ERC/DRC, captures, measurements, acceptance results.",
    ),
    ("release/", "Approved manufacturing package only."),
];

/// Observed EasyEDA state (01 Nov 2026): project "BEE BOX V2" with Board1, Schematic1 (page P1, empty), PCB1, Panel1.
pub const EASYEDA_OBSERVED: &str =
    "EasyEDA Pro V3.2.149; project BEE BOX V2: Board1 -> Schematic1 (P1, empty A0) + PCB1; Panel1.";

/// EasyEDA project structure options (approvals REC-06).
pub const EASYEDA_STRUCTURE_OPTIONS: &[DecisionOption] = &[
    DecisionOption { label: "A (recommended)", description: "One project 'BEE BOX V2' with two boards: MAIN (schematic M01-M05 + PCB) and PATCH (schematic P01-P05 + PCB); existing Board1 renamed to MAIN.", consequences: "Shared library/devices and one revision history; two independent netlists; panel can hold patch array." },
    DecisionOption { label: "B", description: "Two separate projects (BeeBox V2 Main, BeeBox V2 Patch).", consequences: "Independent release cycles; duplicated library management; cross-board ICD consistency checked manually." },
    DecisionOption { label: "C", description: "Single schematic for both boards.", consequences: "Not viable: two PCBs need separate netlists. Rejected.", },
];

/// Scripting rules for EasyEDA (p.14).
pub const SCRIPT_RULES: &[(&str, Status)] = &[
    ("One small JS script per subsystem with ID, revision, target project/sheet and expected RefDes.", Status::SourceBaseline),
    ("Explicit nets and real wires; labels must not hide missing connections.", Status::SourceBaseline),
    ("No destructive clear/delete; additive and re-runnable.", Status::SourceBaseline),
    ("After execution: visual check, pin/net check, netlist compare, ERC, record result.", Status::SourceBaseline),
    ("Next script only after user acceptance and a saved project revision; one writer at a time.", Status::SourceBaseline),
    ("Read-only project/sheet context check before every write.", Status::Proposal),
];
