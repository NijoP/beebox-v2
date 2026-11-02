# BeeBox V2 — Project Knowledge Base

**Primary source of truth:** `docs/00_source/BBX-ENG-PR-003_Rev0.2_BeeBox_V2_End_to_End_Project_Report.pdf`
(Document ID BBX-ENG-PR-003, Rev 0.2 "Expanded engineering baseline", dated 01 Nov 2026).

**Project status (per source):** Engineering proposal — *conditional GO* for schematic and two-patch proof work
after the auxiliary-module interface is closed. **Not released for manufacture.** No hardware, measurement or
field result exists.

**Knowledge-base status:** Study phase complete (KB rev A). No schematic, script, PCB or EasyEDA change has been made.

## Conventions used in this knowledge base

| Tag | Meaning |
|---|---|
| `[p.N]` | Statement taken from page N of the source PDF |
| **CONFIRMED** | Explicitly required or decided in the source |
| **PROVISIONAL** | Candidate / concept in the source, explicitly *not frozen* |
| **TBD** | The source says it is open |
| **KB-ANALYSIS** | Derived by the knowledge-base author (agent), **not** in the source. Must be reviewed before use |

If a KB file conflicts with the source PDF, the PDF wins unless a newer approved revision says otherwise.

## Schematic method reference

**[`SCHEMATIC_METHODS.md`](SCHEMATIC_METHODS.md)** — a reusable playbook for the way these schematics
were built: EasyEDA Pro driven by small per-subsystem JavaScript scripts over the EasyEDA API bridge,
with netlist-verified connectivity. Intended as a reference template for future projects.

## Index

| File | Content |
|---|---|
| `docs/01_project_overview.md` | Purpose, operating concept, product boundary, design principles, status |
| `docs/02_requirements.md` | Confirmed requirements with IDs and page traceability |
| `docs/03_system_architecture.md` | Functional partition, main controller, power architecture |
| `docs/04_interfaces_ICD.md` | 12-contact chain pinout, main-board GPIO functions, display, AUX header concept |
| `docs/05_sensor_patch_design.md` | Patch blocks, shift-register bits, isolation, ID, **J_AUX1..J_AUX4** expansion |
| `docs/06_identification_and_firmware.md` | ID resistor model, ADC windows, firmware state machine |
| `docs/07_design_decisions.md` | Decision log (confirmed vs provisional) |
| `docs/08_open_questions.md` | Unresolved questions / missing information |
| `docs/09_assumptions.md` | Working assumptions (source and KB) |
| `docs/10_risk_register.md` | Source risk register plus KB-identified risks |
| `docs/11_verification_plan.md` | V-01..V-14 acceptance tests and prototype stop-gate |
| `docs/12_methodology_and_workflow.md` | Phases P0–P8, schematic sheet plan, JS script rules, BOM rules, folder structure, tooling |
| `docs/13_component_candidates_and_references.md` | Candidate parts and references R1–R12 |
| `docs/14_engineering_analysis_notes.md` | KB-ANALYSIS: control-bit budget, signal routing, power, ID checks |
| `docs/15_status_log.md` | Revision history of this KB and observed environment status |
| `docs/00_source/` | Source PDF, text extract and rendered figures |
