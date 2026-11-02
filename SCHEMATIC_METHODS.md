# How these schematics were built — a reusable method

> A practical, reusable playbook for capturing **clean, netlist-verified schematics in EasyEDA Pro
> by driving the EasyEDA API bridge with small JavaScript scripts**, as actually used on BeeBox V2.
> Use this as a reference template for future projects.

The goal of this method is a schematic that is **electrically complete, collision-free, grid-aligned,
and provable by a netlist** — not just something that "looks wired". Every rule below exists to kill a
specific class of silent error (hidden missing copper, label collisions, reset-time glitches,
destructive regen, stale visual checks).

---

## 0. Toolchain

| Tool | Role |
|---|---|
| **EasyEDA Pro** (V3.2.x) | The schematic/PCB authority. The single source of truth for the design. |
| **EasyEDA API Gateway** | A local bridge (e.g. `127.0.0.1:<port>`) that lets external scripts run inside the live EasyEDA client. |
| **JavaScript scripts** (`easyeda_scripts/`) | One small script per subsystem/sheet. This is where the design is authored programmatically. |
| **Netlist tooling** (`netlist/`) | Python scripts that export, diff and audit netlists — the objective proof of connectivity. |
| Agent / terminal | Orchestration, file edits, running the loop, recording results. |

---

## 1. Decompose the design into one sheet per subsystem

Split each physical board into independent **functional sheets**, and write **one JS script per sheet**.
This keeps each script small, re-runnable, and easy to verify in isolation.

BeeBox V2 used 10 sheets across 2 boards:

**Main board (ESP32-S3 controller)**
- `M01` Power input — USB-C, ESD, charger, fuel gauge, buck-boost 3V3, 5 V boost, chain current-limit
- `M02` MCU core — ESP32-S3 module, decoupling, boot/reset, USB, UART debug
- `M03` Chain control + ADC — 595 drive, ID ADC, safe-state pulls, sensor-bus pull-ups
- `M04` 12-pin chain connector — FPC/pogo, chain ESD, test pads
- `M05` Display + status — independent I²C display, status RGB LEDs

**Sensor patch (repeatable, T/RH and Mic variants)**
- `P01` FPC IN/OUT + pin-8 cascade + pass-through
- `P02` Shift register + startup/safety latch + local regulator + load switch
- `P03` Primary sensor path (isolation mux, ID resistor branch, sensor variants)
- `P04` AUX ×4 (decoders, isolator, port mux, port switches, ESD, headers)
- `P05` Test points, DNI/variant notes, silkscreen labels

**Rule:** capture **decision-independent sheets first** (the ones not blocked by open decisions), then
close the open decisions, then capture the dependent sheets. On BeeBox V2 the order was
`M02 → M01 → M03 → M05 → P02 → P03` → *(close interface decisions)* → `M04 → P01 → P04 → P05`.

---

## 2. Do the engineering QA *before* you draw

Every component value was datasheet-checked and written into a QA sheet **before** capture
(see `easyeda_scripts/MAIN_CONTROLLER_QA.md`). For each regulator/switch/divider, record the
formula, the chosen resistors, and the resulting value with a pass/fail against the spec. Examples:

- Buck-boost 3V3: `Vout = 0.5·(1+Rt/Rb)` → 562k/100k → **3.31 V** ✓ (in 3.0–3.6 V ESP32 window)
- Boost 5V0: `Vout = 0.6·(1+Rt/Rb)` → 750k/100k → **5.10 V** ✓ (< 5.5 V)
- Current-limit switch: `I_OS = K/R_ILIM` → 76.8k → **302–394 mA** ✓
- Charger: `I_CHG = 890/R_ISET`, `I_IN = 1610/R_ILIM`, timer range check, TS/NTC handling

Drawing a schematic with un-validated values just means you discover the error later and redo the work.
QA first makes the capture scripts a transcription of an already-correct design.

---

## 3. Script rules that produce *neat, trustworthy* sheets

Every EasyEDA JS script followed these rules (from `docs/12_methodology_and_workflow.md` §12.4):

1. **Header metadata** in every script: unique filename, subsystem ID, revision, target project + sheet,
   and the expected component references it will create.
2. **Guard on the active document.** Call `getCurrentProjectInfo` + `getCurrentDocumentInfo` and confirm
   the active doc is the intended `SCHEMATIC_PAGE` before writing. On BeeBox V2 every patch script was
   *guarded on the sheet UUID* so it could never touch the wrong sheet ("Schematic1 untouched").
3. **Resolve real library devices.** For each MPN, search the library (`lib_Device.search(MPN)` / LCSC),
   capture the real `{libraryUuid, uuid}`, record them in a per-sheet manifest, then place with
   `sch_PrimitiveComponent.create(...)`. Never place a generic placeholder you intend to "fix later".
4. **Draw real wires AND nets — never labels alone.** Net labels on their own can hide missing copper.
   Use `sch_PrimitiveWire` plus net labels so connectivity is real, not cosmetic.
5. **Snap to grid.** Schematic units are **0.01 inch (10 mil)**; place everything on a **100-mil (10-unit)
   grid**. Give each subsystem its own rectangular region (x/y band) so blocks don't overlap. This single
   habit is what makes the sheet read cleanly.
6. **Additive and re-runnable.** No destructive clear/delete. Scripts are additive or narrowly scoped and
   safe to re-run. Accepted blocks are preserved, never regenerated from scratch.
7. **Use enum members, not raw magic numbers**, for API arguments where the API provides them.

---

## 4. Lay out blocks in fixed coordinate bands

Assign each subsystem a reserved region so the sheet is organised and never collides. Example from the
main controller (schematic units, 0.01 in):

| Block | Region (x, y) |
|---|---|
| MCU (M02) | U1 @ (2000, 2400); passives y≈2620–2760 |
| Power (M01) | x 300–1700, y 1000–2900 |
| Chain ctrl (M03) | x 2800–3050, y 2050–2650 |
| Status LEDs | x 3300–3560, y 2220–2700 |
| Connectors (M04) | x 3650–4100, y 1850–2550 |
| Display/sensors (M05) | x 1600–3000, y 1350–1550 |

Because placement is scripted with explicit coordinates, the layout is deterministic and reproducible.

---

## 5. The controlled capture loop (one block at a time)

For **each** sheet, run this loop (from §12.2) and only move on after acceptance:

```
1 Requirements frozen  → 2 Bridge connected + smoke-tested
→ 3 Run one JS subsystem script
→ 4 Capture: inspect schematic, check net names + pins
→ 5 Verify: netlist export/compare + pin-map + ERC + independent review
→ 6 Accept: save a project revision / backup, THEN start the next block
```

- **Only one controlled writer edits the live project at a time.** Reviewers may read in parallel, but
  two writers must never touch the same project concurrently.
- **Do not start capture while the bridge is retrying/unstable.**
- Every change gets a revision and test evidence before the next sheet begins.

---

## 6. Verify with the netlist, not with your eyes

The objective acceptance criterion is the **netlist**, exported and compared after every script:

- Target: **0 floating nets, 0 shorts**, and every signal consumed (no un-terminated GPIO).
  - Main controller result: **84 components, 71 nets, no shorts, no unexpected floating nets** — only
    intentional single-pin spares (strapping/USB-unused), explicitly listed.
  - Patch result: **52 parts, 43 nets, 0 floating, 0 shorts.**
- Count the high-fanout nets as a sanity check (e.g. `+3V3:23`, `GND:81`) — a wrong count exposes a
  missed or doubled connection instantly.
- **Label-collision trap:** net labels placed too close can silently merge. On BeeBox V2 the netlist
  trace caught real collisions (`GND↔AUX5V_FLT_N`, `AUX_3V3↔AUX5V_ILIM`, ESD `NC↔AUX2_SCL`); the row was
  rebuilt with collision-free spacing and re-verified clean. **Trust the netlist over the picture.**
- Tooling caveat observed: `getCurrentRenderedAreaImage` can return a *stale cached frame*, so the
  script cannot always self-verify the visual. Connectivity was proven by netlist; a human was asked to
  confirm spacing/alignment on the live screen. **Netlist = authority; visual = human spot-check.**

The `netlist/` folder holds the canonical nets, a "golden" vs "relayout" comparison, part/ref maps,
and an `equiv.py` / `config_audit.py` to diff them programmatically.

---

## 7. Design-for-fail-safe is part of "neat"

A clean schematic also means a schematic that is *safe at reset*. Catches made during capture:

- **Active-level traps:** a baseline load switch (`TPS2552-1`, active-LOW EN) would have powered the AUX
  ports at reset; it was replaced by the active-HIGH `TPS2553-1` so ports default OFF.
- **Default-OFF everywhere:** Q-output pull-downs (8×47k) on the shift register, OE pull-ups, pull-downs
  on `CHAIN_PWR_EN`, so the whole chain boots quiet.
- **Series termination / protection:** 33 Ω series on fast digital, 1 kΩ on `PATCH_OE_N` (mirror-insert
  protection), ESD arrays on every exposed connector line.
- **Single ground net**, IC thermal pads tied to GND, connector shields to GND, pour deferred to PCB.

Record these as explicit "corrections made during build" and "decisions for client review" in the sheet's
result file, so the reasoning is auditable.

---

## 8. Record a result file per board

Each board got a short, honest result document:
- `easyeda_scripts/MAIN_CONTROLLER_RESULT.md`, `MAIN_CONTROLLER_QA.md`
- `easyeda_scripts/SENSOR_PATCH_RESULT.md`, `M02_RESULT.md`

Each states: final part/net counts, the verified key nets, what changed during build, known tooling
limitations, and open decisions for the client. This is what makes the capture reproducible and reviewable.

---

## TL;DR checklist for the next project

- [ ] Split into functional sheets; **one JS script per sheet**.
- [ ] QA every value against the datasheet **before** drawing.
- [ ] Script header metadata; **guard on project + sheet UUID**.
- [ ] Resolve **real library devices** (record UUIDs); no placeholders.
- [ ] Draw **real wires + nets**; never rely on labels alone.
- [ ] Snap to the **100-mil grid**; give each block a fixed coordinate band.
- [ ] Scripts **additive, re-runnable, non-destructive**; one writer at a time.
- [ ] After each script: **netlist export + compare + ERC**; require **0 floating / 0 shorts**.
- [ ] Verify **active levels & reset-time defaults** (fail-safe OFF).
- [ ] Save a **revision** and write a **result file** before the next sheet.
