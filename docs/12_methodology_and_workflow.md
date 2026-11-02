# 12 — Methodology, Workflow and Execution Phases

## 12.1 Execution phases [p.18]
| Phase | Work package | Exit gate | Current state (KB) |
|---|---|---|---|
| P0 Baseline update | Approve 4 headers/patch; select module families; patch outline, FPC lengths, battery, power envelope | Signed requirements + ICD; open questions tracked | **In progress**: KB created, sign-off pending |
| P1 Module survey | 2–4 representative methane/CO/env module datasheets + MPNs; protocol/supply/current matrix | Approved shortlist + port compatibility table | Not started |
| P2 Architecture closure | AUX pins, supply, mux topology, ID model, shift-register controls | Updated architecture, GPIO/control map, power budget, net dictionary | Not started |
| P3 API/tooling setup | Connect EasyEDA API Gateway; read-only smoke tests; confirm library/API schema and target | Verified handshake + safe API test report | **Partly done**: handshake + read-only query OK on 01 Nov 2026; formal report pending |
| P4 Schematic capture | Main + patch sheets as separate JS subsystems; add AUX circuitry | Schematic blocks, component list, netlist, pin map | Blocked by P0–P2 |
| P5 Independent audit | Symbol/device/footprint mapping, connectivity, ERC, netlist parity, visual | Audit report; no critical mismatch | — |
| P6 Two-patch proof | Controller + 2 patches; 1 T/RH, 1 mic, ≥1 AUX path | Measured prototype report; Gate 6 pass | — |
| P7 Eight-position validation | Mixed population, missing tail, all ports, max cable, worst-case power | 8-patch validation matrix + raw evidence | — |
| P8 Controlled release | Close critical/high risks; BOM, Gerbers, drill, PnP, drawings, firmware, fixture | Manufacturing release package | — |

The schedule must be re-estimated after the module shortlist exists [p.18]. Earlier estimates exclude fabrication, lead
times, customs, assembly queues and redesign.

## 12.2 Controlled schematic development loop [p.13, Fig. 3]
1 Requirements (freeze interface + module use cases) → 2 API bridge (connect + smoke test) → 3 JS subsystem (one
controlled script per block) → 4 Capture (inspect schematic, net names + pins) → 5 Verify (ERC / netlist / pin map /
independent review) → 6 Accept (version, backup, then next block).
- Agents may review in parallel; **only one controlled writer edits the active EasyEDA project at a time.**
- No destructive regeneration. Accepted blocks are preserved. Every change has a revision and test evidence.
- **Do not start schematic generation while the bridge is retrying.**

## 12.3 Schematic sheet plan [p.14]
| Board | Sheet | Scope |
|---|---|---|
| Main | M01 Power input | Battery connector, protection, charge interface, regulator(s), power test points |
| Main | M02 ESP32-S3 core | Module, boot/reset, programming/debug, decoupling, GPIO map |
| Main | M03 Chain control + ADC | Shift data/clock/latch/OE, ID ADC, safe-state pulls, test points |
| Main | M04 12-pin FPC / pogo | Exact 12-pin map, orientation, labels, continuity points |
| Main | M05 Display + status | Independent 4-wire I²C display, status LED, test points |
| Patch | P01 FPC IN/OUT | Two connectors, pin-8 cascade, pass-through nets |
| Patch | P02 Register + power | 74HC595, defaults, local load switch, decoupling |
| Patch | P03 Primary sensor / ID | 4066 isolation, ID resistor branch, T/RH / mic variants |
| Patch | P04 AUX1–AUX4 | Four headers, port mux/switch matrix, power selection, protection, pin labels |
| Patch | P05 Test / assembly | Test points, DNI options, variant notes, config + manufacturing labels |

## 12.4 Rules for every EasyEDA JS script [p.14]
- Unique filename, subsystem ID, revision, target project/sheet and expected component references.
- Use exact library symbols, device mapping, package and MPN **only after validation**.
- Create explicit nets **and real wires**. Labels alone must not hide missing connections.
- Include the pin-to-net map, placement coordinates, orientation, values, footprints and test points.
- No destructive clear/delete. Scripts are additive or narrowly scoped, and safe to rerun where possible.
- After running: visual inspection, pin/net check, netlist export/compare, ERC, then record the result.
- Prepare the next subsystem script only after user acceptance and a saved project revision.
- KB addition (from the EasyEDA API skill): schematic coordinates are in units of 0.01 inch, PCB in mil. Use enum
  members, not raw numbers. Check the active document type before SCH/PCB calls.

## 12.5 BOM rules [p.15]
See REQ-DP-05/06. Use 0404 metric passives as a compact target only where suitable.

## 12.6 Project folder structure [p.19]
| Folder | Contents |
|---|---|
| docs/ | Report, requirements, architecture, component research, BOM, power design, interface guide, schematic plan, risk register, validation plan, revision history |
| schematic/ | Main and patch EasyEDA schematics; AUX interface variants |
| easyeda_scripts/ | One JS file per subsystem with metadata, revision and safe update behaviour |
| netlist/ | Canonical netlists, pin-to-net CSVs, connector maps, comparison output |
| bom/ | Controlled BOM, alternates, drawings, module compatibility matrix, sourcing |
| pcb/ | PCB source, Gerbers, drill, stack-up, outlines, fab notes, PnP, assembly drawings |
| firmware/ | Scan state machine, drivers, module profiles, ADC calibration, logs, test builds |
| verification/ | ERC/DRC, scope/LA captures, power records, calibration, acceptance tests |
| release/ | Approved manufacturing package, firmware version, fixture spec, change log |

Only `docs/` exists so far. Other folders are created when their phase starts.

## 12.7 Release package must include [p.19]
Editable EasyEDA project + library mappings; PDF schematics, layout, Gerbers, drill, PnP, fab/assembly drawings,
stack-up; BOM with MPNs/alternates/lifecycle; canonical netlist + connector pin map + pin-to-pad report; ERC/DRC,
review sign-off, firmware build/version, test procedure, measured evidence; **AUX header pinout/voltage labels,
supported module list, max current per port and safety warnings.**

## 12.8 Tooling [p.13]
| Tool | Role |
|---|---|
| EasyEDA Pro V3.2.149 | Schematic/PCB authority. Project "BEE BOX V2" (Board1, Schematic1 → P1, PCB1, Panel1), sheet empty |
| EasyEDA API Gateway v1.0.6 | Bridge for agent scripts. See status log |
| Herdr | Windows terminal workspace manager; BeeBox V2 workspace ID "WA" |
| Firstmate / Pi agent | Orchestration, file/process interaction, engineering review |
| PyAgent / local validation | Script checks, pin-map compare, reports (integration TBD) |
| JavaScript scripts | Small per-subsystem EasyEDA scripts |
