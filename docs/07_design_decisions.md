# 07 — Design Decision Log

Status: **CONFIRMED** (stated as a baseline decision in the source) · **PROVISIONAL** (candidate; not frozen) · **REJECTED**.
New decisions must be appended with date, owner and evidence. Do not edit history; supersede it instead.

| ID | Decision | Status | Rationale / source |
|---|---|---|---|
| DEC-001 | Central ESP32-S3 controller + up to 8 MCU-free, batteryless patches in one daisy chain | CONFIRMED | p.3–p.5 |
| DEC-002 | Logical 12-contact chain interface (pinout in 04_ICD §4.1), shared by FPC and pogo builds | CONFIRMED | p.4, p.7 |
| DEC-003 | Pin 8 cascades through each patch's 74HC595 (DS in, QH' out); all other pins pass through | CONFIRMED | p.7 |
| DEC-004 | One SN74HC595-class register per patch; Q0 power, Q1 signal enable, Q2 ID enable | CONFIRMED | p.8 |
| DEC-005 | 64-bit control image (8 × 8) managed by the main controller | CONFIRMED (changes if DEC-020 picks a 2nd register) | p.5 |
| DEC-006 | Time-multiplexed shared sensor lines: CLK/SYNC = I²C SCL/SDA or I²S BCLK/WS; DATA_ID = ADC ID or I²S SD | CONFIRMED (bench proof pending, V-03/V-08) | p.6, p.7 |
| DEC-007 | Shared ID/data GPIO must be ADC1-capable; GPIO38 rejected | CONFIRMED / GPIO38 REJECTED | p.6 |
| DEC-008 | Display on a separate 4-wire I²C bus with independent pull-ups and power | CONFIRMED | p.4, p.6 |
| DEC-009 | Debug/programming independent of the chain | CONFIRMED | p.6 |
| DEC-010 | One primary sensor type per patch; unpopulated alternative DNI/isolated | CONFIRMED | p.8 |
| DEC-011 | One active patch / AUX path at a time for the first build | CONFIRMED (initial) | p.3, p.9 |
| DEC-012 | Fail-safe default: all sensor/AUX power and signals OFF during reset, shifting and faults; OE_N high at boot | CONFIRMED | p.3, p.11 |
| DEC-013 | **Four AUX headers J_AUX1..J_AUX4 on every patch** (supersedes the Rev 0.1 "no third sensor class" limit) | CONFIRMED (per-patch interpretation to be re-confirmed, OQ-002) | p.4, p.9, p.20 |
| DEC-014 | AUX headers do not add chain contacts | CONFIRMED (for this stage) | p.5, p.9 |
| DEC-015 | AUX headers never paralleled; selected-port mux/switch mandatory | CONFIRMED | p.5 |
| DEC-016 | Per-port controlled AUX power, default OFF (preferred) | CONFIRMED as preference | p.10 |
| DEC-017 | No SPI support claimed on the 4-contact AUX concept | CONFIRMED | p.10 |
| DEC-018 | AUX header concept 4-pos: V_AUX, GND, AUX_IO_A, AUX_IO_B | PROVISIONAL | p.10 |
| DEC-019 | Primary ID: 22 k (T/RH), 68 k (mic), 100 k pull-down, screening windows | PROVISIONAL | p.11 |
| DEC-020 | Q3–Q7 for AUX control vs second register/decoder | OPEN (see 14 §1) | p.8, p.10 |
| DEC-021 | Power: 1S LiPo; buck-only rejected unless a validated cutoff exists → buck-boost preferred | PROVISIONAL | p.12 |
| DEC-022 | Candidate parts: SHT45, SN74HC595, CD74HC4066, TPS22918, Hirose FH34SRJ-12S-0.5SH(50), PUI DMM-4026-B-I2S | PROVISIONAL | p.7, p.15 |
| DEC-023 | ICS-43434 mic baseline dropped (EOL) | REJECTED | p.15 |
| DEC-024 | EasyEDA Pro is the schematic authority; small per-subsystem JS scripts; additive/non-destructive; one writer at a time | CONFIRMED (process) | p.13, p.14 |
| DEC-025 | Schematic sheet plan M01–M05 (main), P01–P05 (patch) | CONFIRMED (plan) | p.14 |
| DEC-026 | First prototype = 1 main + 2 patches (1 T/RH, 1 mic) + AUX proof on ≥1 patch; no scaling to 8 until gates pass | CONFIRMED | p.16 |
| DEC-027 | Project folder structure docs/ schematic/ easyeda_scripts/ netlist/ bom/ pcb/ firmware/ verification/ release/ | CONFIRMED (plan) | p.19 |
