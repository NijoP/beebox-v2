# BeeBox V2 — Schematic Capture Plan (EasyEDA JS)

**Rev:** Plan-A · **Date:** 2026-11-02 · **Author:** Pi agent
**Target project:** `nijoponnachan/BEE BOX V2` (uuid `893fdeafb7024c9c9722081ec493d617`)
**Baseline:** Release review 2026-11-02 (rev-D), `engineering_docs/src/release_review.rs` REGISTER.
**Bridge:** `127.0.0.1:49620`, `edaConnected=true` (verified).

---

## 0. READINESS GATE — read before running any script

Per `docs/12_methodology_and_workflow.md` (§12.2, §12.4) and the crate docs, schematic
generation is **"on hold for review"**. The following decisions change the netlist and MUST be
confirmed by the client before the dependent sheets are captured:

| ID | Open decision | Blocks sheet(s) | Current baseline vs proposal |
|----|---------------|-----------------|------------------------------|
| **A-01 / APR-?** | AUX_ARM on **pin 12** (dedicated analog arm line) vs keep arm-sense on **pin 7** (rev-C) | M04, P01, P04 | `interfaces::CHAIN_PINS` pin 12 = **GND** (4 grounds). Global-interlock HW proof needs pin 12 = **AUX_ARM** (3 grounds). **Not reconciled.** |
| **APR-1** | +5V_CHAIN on pin 2 (option B) | M01, M04, P01, P02 | Adopted in rev-D CHAIN_PINS (per-patch TPS62840 → +3V3_LOCAL). Treat as approved-pending-signoff. |
| **APR-4** | Total chain length / per-segment FFC length | M04, P01 (SI only, not netlist) | Open — does not block net capture. |
| **D-01..D-16** | Part finalisation (e.g. HC595→LV595A, HC4066→TMUX1511, HC238/138→LVC138A, TPS2553→TPS2552) | all patch sheets | rev-D register selects the LVC/TMUX parts; use those. |
| **A-03** | Audio length → ESP32-S3 **N16R2 vs N16R8** | M02 | Capture N16R2 footprint; both are pin-compatible WROOM-1. |

**Recommended sequencing:** capture the **decision-independent** sheets first
(M02, M01, M05, P02, P03), then close A-01/APR-1 and capture the **interface + AUX** sheets
(M04, P01, P04). M03 depends only on APR-1 (safe to do early).

---

## 1. SUBSYSTEM DECOMPOSITION

Two physical boards → 10 schematic sheets (source §12.3). One JS script per sheet.

### MAIN BOARD (ESP32-S3 controller)
| Sheet | Subsystem | Depends on |
|-------|-----------|------------|
| M01 | Power input: USB-C, ESD, charger, fuel gauge, buck-boost +3V3, 5 V boost, chain current-limit | APR-1 |
| M02 | ESP32-S3 core: module, decoupling, boot/reset, USB, UART debug | A-03 (variant only) |
| M03 | Chain control + ADC: 595 drive, ID ADC, safe-state pulls, sensor-bus pull-ups | APR-1 |
| M04 | 12-pin chain connector + chain ESD + test pads | A-01, APR-1 |
| M05 | Display (independent I²C) + status RGB LED | — |

### SENSOR PATCH (×1 repeatable, 2 build variants: T/RH, Mic)
| Sheet | Subsystem | Depends on |
|-------|-----------|------------|
| P01 | FPC IN/OUT (2× FH34), pin-8 cascade, pass-through, +5V entry | A-01, APR-1 |
| P02 | Shift register + startup/safety latch + local regulator + primary load switch | APR-1 |
| P03 | Primary sensor path: TMUX1511 isolation, ID resistor branch, SHT45 / I²S-mic variants | — |
| P04 | AUX ×4: decoders, isolator, port mux, 4× port switch, ESD, headers, arm window | A-01 |
| P05 | Test points, DNI/variant notes, silkscreen labels | — |

---

## 2. COMPONENT + NET LIST PER SUBSYSTEM
(MPNs from rev-D REGISTER. "Approved"/"Conditional" = crate approval status.)

### M01 — Power input
**ICs:** `GCT USB4105-GF-A` (USB-C), `TPD2E2U06DRLR` (USB D± ESD), `BQ24074RGTR` (charger/path),
`TPS63802DLAR` (buck-boost → +3V3), `TPS61023DRLR` (boost → +5V0), `TPS2553DBVR-1` (chain limit),
`MAX17048G+T10` (fuel gauge, **NotApproved** → fallback: VBAT divider 1 M/330 k on GPIO2).
**Passives:** BQ24074 RISET 1.27 k, RTMR 68.1 k, IN 1 µF/25 V, BAT 10 µF, OUT 10 µF; TPS63802 L 1.5 µH,
2×10 µF in, 3×22 µF out, FB 562 k/100 k; TPS61023 L 1 µH, FB 750 k/100 k 0.1 %; TPS2553 R_ILIM 76.8 k;
VBAT sense 1 M/330 k + 100 nF; NTC → TS.
**Key nets:** `VBUS`, `USB_D+`, `USB_D-`, `VBAT`, `VSYS/OUT`, `+3V3`, `+5V0`, `+5V_CHAIN`,
`VBAT_SENSE`, `CHAIN_PWR_EN`, `CHAIN_FAULT_N`, `BOOST_EN`, `REG_PS_SYNC`, `REG_PG`,
`CHG_EN1/2`, `CHG_STAT_N`, `CHG_PGOOD_N`, `GAUGE_ALRT_N`, `SYS_SDA/SCL`, `GND`.

### M02 — ESP32-S3 core
**ICs:** `ESP32-S3-WROOM-1-N16R2` (**Conditional**, A-03).
**Passives:** bulk 22 µF + 100 nF; CHIP_PU 10 k + 1 µF; BOOT 10 k pull-up + button→GND; EN button.
**Key nets:** 3V3, EN(CHIP_PU), GPIO0(BOOT), GPIO19/20 (USB_D-/+), GPIO43/44 (UART0_TX/RX),
+ every GPIO from `main_board::GPIO_MAP` (26 allocations).

### M03 — Chain control + ADC
**Passives:** 33 Ω series on GPIO11/12/13/14 (DATA/CLK/LATCH/OE_N); 100 k pull-downs on CLK, LATCH,
CHAIN_PWR_EN; 100 k pull-up on CHAIN_FAULT_N; `SENSOR_DATA_ID` 100 k 1 % pull-down (**no cap**);
33 Ω series on GPIO4/5 (SENSOR_CLK/SYNC); 2×2.2 k sensor-bus pull-ups gated by `SENSOR_PU_EN` (GPIO6);
`PATCH_OE_N` series 1 kΩ (startup_safety: mirror-insert protection).
**Key nets:** `PATCH_SHIFT_DATA/CLK/LATCH`, `PATCH_OE_N`, `SENSOR_CLK`, `SENSOR_SYNC`,
`SENSOR_DATA_ID`, `SENSOR_PU_EN`, `CHAIN_PWR_EN`, `CHAIN_FAULT_N`.

### M04 — 12-pin chain connector  ⚠ needs A-01
**ICs:** `Hirose FH34SRJ-12S-0.5SH(50)` (**Conditional**), `TPD4E05U06DQAR ×2` (8 signal lines incl. AUX_ARM).
**Net map (rev-D `interfaces::CHAIN_PINS`):**
`1 GND · 2 +5V_CHAIN · 3 GND · 4 SENSOR_CLK · 5 SENSOR_SYNC · 6 GND · 7 SENSOR_DATA_ID ·
8 PATCH_SHIFT_DATA (directional) · 9 PATCH_SHIFT_CLK · 10 PATCH_LATCH · 11 PATCH_OE_N · 12 GND`
→ **If A-01 approved, pin 12 → AUX_ARM** and chain ESD covers it.

### M05 — Display + status
**ICs:** `TPS22918DBVR` (display power switch), JST `SM04B-SRSS-TB` (**Conditional**), RGB status LEDs
(**NotApproved** — select MPN).
**Passives:** 2×2.2 k display I²C pull-ups; 3× LED series R.
**Key nets:** `DISP_PWR_EN`→`+3V3_DISPLAY`, `DISPLAY_SCL/SDA`, `LED_R/G/B`, GND.

### P01 — FPC IN/OUT  ⚠ needs A-01
**ICs:** `FH34SRJ-12S-0.5SH(50) ×2` (J_IN, J_OUT).
**Routing:** pins 1-7, 9-12 pass-through; **pin 8**: J_IN→`SER`, `QH'`→33 Ω→J_OUT (directional cascade).
**Passives:** +5V_CHAIN entry 10 µF 16 V + 100 nF; 33 Ω on QH'.

### P02 — Register + power + startup safety
**ICs:** `SN74LV595APWR` (shift reg), `TPS62840DLCR` (+3V3_LOCAL reg, **Conditional**),
`TLV840CADL29DBVR` (supervisor), `SN74LVC1G74DCUR` (BLOCK latch), `SN74LVC1G32DBVR` (OE gate),
`TPS22918DBVR` (primary load switch).
**Passives:** 47 k pull-downs ×8 (Q0–Q7); TPS62840 RSET 267 k; supervisor 100 k pull-up on RESET_N;
primary switch 47 k pull-down + CT 1 nF; decoupling 100 nF per IC.
**Key nets:** `+3V3_LOCAL`, `PRI_PWR_EN(Q0)`, `PRI_SIG_EN(Q1)`, `ID_EN(Q2)`, `AUX_A0(Q3)`,
`AUX_A1(Q4)`, `AUX_SIG_EN(Q5)`, `AUX_PWR_EN(Q6)`, `AUX_ARM(Q7)`, `RESET_N`, `BLOCK`, `OE_N_595`,
`PRI_VDD`, `SER`, `QH'`.

### P03 — Primary sensor / ID
**ICs:** `TMUX1511PWR` (4× SPST isolation: ch1-3 = CLK/SYNC/SD ← Q1, ch4 = ID ← Q2),
variant: `SHT45-AD1B-R2` (**Conditional**) **or** `PUI DMM-4026-B-I2S-R` (**Conditional**).
**Passives:** R_ID **22.0 k 1 %** (T/RH) / **68.0 k 1 %** (mic); 100 nF per sensor.
**Nets:** `SENSOR_CLK/SYNC/DATA_ID` (switched), `PRI_VDD`, ID branch to SENSOR_DATA_ID.
ID windows: 22 k→2.704 V, 68 k→1.964 V (verified).

### P04 — AUX ×4  ⚠ needs A-01
**ICs:** `SN74LVC138APWR ×2` (power decoder + bus decoder), `TMUX136RSER` (bus isolator, **Conditional**),
`TMUX1109PWR` (dual 4:1 port mux, **Conditional**), `TPS2552DBVR-1 ×4` (port switch),
`TMUX1101DBVR` (arm source switch), `TLV7032DGKR` (window comparator), `SN74LVC1G00DBVR` (window NAND),
`SN74LVC1G17DBVR` (arm-delay Schmitt), `TPD4E05U06DQAR ×4` (port ESD), `JST B4B-PH-K-S ×4` (headers).
**Decoder logic (global_interlock):**
power dec `A=Q3,B=Q4,C=Q7D,G1=Q6,G2A=PRI(Q0∨Q1),G2B=WIN_BAD` → /Y4../Y7 → TPS2552 EN;
bus dec `A=GND,B=Q6,C=Q7D,G1=Q5,G2A=PRI,G2B=WIN_BAD` → /Y6 → TMUX136 EN;
TMUX1109 `EN=Q5, A1:A0=Q4:Q3`; `PRI=Q0∨Q1` (SN74LVC1G32, shared w/ P02 or 2nd gate).
**Passives:** R_ILIM 93.1 k ×4; 2.2 k ×8 (SDA/SCL pull-ups to AUXn_VCC); 10 k bleeder ×4;
10 µF+100 nF ×4 (port rail); 1 µF ×4 (TPS2552 IN); 100 k ×4 (FAULT pull-ups); R_ARM 150 k 0.1 %;
window ladder 187 k/75.0 k/69.8 k 0.1 %; RC filter 10 k/10 nF; Q7 delay 100 k/22 nF; 47 k ×5 (Q3–Q7).
**Header silkscreen:** `1 VCC · 2 GND · 3 SDA · 4 SCL`.

### P05 — Test / assembly
Test pads on all rails + control bits; DNI table (T/RH vs Mic); variant/config silkscreen.

---

## 3. JS EXECUTION STRATEGY (per §12.4)

1. **One script per sheet**, additive, safe to re-run, no destructive clear/delete.
2. Before writing: `getCurrentProjectInfo` + `getCurrentDocumentInfo` → confirm active doc is the
   target schematic page (type `SCHEMATIC_PAGE`).
3. **Units:** schematic = **0.01 inch (10 mil)**. Place on a 100-mil (10-unit) grid.
4. **Device resolution:** for each MPN, search the library (`lib_Device.search(MPN)` / LCSC) and
   capture the `{libraryUuid, uuid}` → place with `sch_PrimitiveComponent.create(...)`.
   Record the resolved UUIDs in a per-sheet manifest before placing.
5. **Real wires + nets:** draw `sch_PrimitiveWire` and net labels; labels alone must not hide
   missing copper (source rule).
6. After each script: visual check → pin/net check → netlist export/compare → ERC → record result,
   save a project revision, **then** prepare the next sheet (only one writer at a time).
7. Each sheet gets its own page/section; multi-page via `dmt_Schematic` pages if needed (currently
   only P1 exists).

## 4. PROPOSED ORDER
`M02 → M01 → M03 → M05 → P02 → P03` (decision-independent) → **[close A-01, APR-1]** →
`M04 → P01 → P04 → P05`.

## 5. FIRST SCRIPT
`10_M02_esp32s3_core.js` — place ESP32-S3-WROOM-1-N16R2 + decoupling + boot/reset + USB + UART,
label all GPIO nets from `main_board::GPIO_MAP`. No AUX/chain dependency.
