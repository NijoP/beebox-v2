# 02 — Requirements Baseline (Confirmed)

Status meanings: **C** = confirmed requirement in source; **P** = provisional / to be validated.
"No specific gas sensor module is approved yet; electrical compatibility is not universal and must be proven per
module MPN." [p.4]

## System / topology
| ID | Requirement | St | Src |
|---|---|---|---|
| REQ-SYS-01 | One ESP32-S3 main controller PCB | C | p.4 |
| REQ-SYS-02 | At most **8 patches** in one physical daisy chain: Main → Patch 1 → … → Patch 8 | C | p.4 |
| REQ-SYS-03 | Patches have no local MCU and no battery. Shift register, analog switch/mux, ID and load-switch circuitry are allowed | C | p.4 |
| REQ-SYS-04 | One primary sensor type per patch: T/RH **or** digital I²S microphone | C | p.4 |
| REQ-SYS-05 | Only one patch / one AUX module path is active at a time (safe starting assumption) | C (initial) | p.3, p.4, p.9 |
| REQ-SYS-06 | All sensor power and signal paths default OFF during reset, shift-register updates and faults | C | p.3 |
| REQ-SYS-07 | The pogo-pin build keeps the same logical 12-signal interface as the FPC chain | C | p.4 |

## Main board
| ID | Requirement | St | Src |
|---|---|---|---|
| REQ-MB-01 | One 12-contact sensor-chain connector | C | p.4 |
| REQ-MB-02 | Separate display interface: 4-wire I²C connector with GND, +3V3_DISPLAY, DISPLAY_SCL, DISPLAY_SDA | C | p.4 |
| REQ-MB-03 | Display pull-ups and display power are independent of the sensor-chain bus | C | p.6 |
| REQ-MB-04 | Debug/programming access (USB or UART/JTAG + test pads) is independent of the sensor chain | C | p.4, p.6 |
| REQ-MB-05 | 4 GPIOs for patch control: PATCH_SHIFT_DATA, PATCH_SHIFT_CLK, PATCH_LATCH, PATCH_OE_N | C | p.6 |
| REQ-MB-06 | 2 shared GPIOs: I²C SCL/SDA (T/RH mode) or I²S BCLK/WS (mic mode) | C | p.6 |
| REQ-MB-07 | 1 shared data GPIO: ADC ID in discovery, I²S SD/DIN in mic capture (mutually exclusive). Must be **ADC1-capable** | C (bench proof needed) | p.6, p.11 |
| REQ-MB-08 | GPIO38 (earlier repository draft) is **not** acceptable as the ADC pin | C | p.6 |
| REQ-MB-09 | Power supervision: monitor the main regulated rail and battery state (telemetry / gauge details TBD) | P | p.6 |
| REQ-MB-10 | Do not freeze the main power stage until battery range, maximum patch load and AUX power plan are calculated | C | p.6 |

## Chain interface
| ID | Requirement | St | Src |
|---|---|---|---|
| REQ-CH-01 | Every patch has a 12-way FPC IN and a 12-way FPC OUT | C | p.4 |
| REQ-CH-02 | Pinout per `04_interfaces_ICD.md` (pins 1–12). The previous repository pin assignment is replaced everywhere | C | p.7 |
| REQ-CH-03 | Pin 8 is **directional**: IN pin 8 → local 74HC595 DS; local QH'/Q7S → OUT pin 8. All other pins pass through | C | p.4, p.7 |
| REQ-CH-04 | The AUX headers do **not** add chain contacts; the 12-contact chain is unchanged at this stage | C | p.5, p.9 |

## Patch
| ID | Requirement | St | Src |
|---|---|---|---|
| REQ-PA-01 | One SN74HC595-class register per patch: Q0 = sensor power, Q1 = primary signal enable, Q2 = ID enable. Q3–Q7 under review for AUX | C (Q0–Q2) / P (Q3–Q7) | p.8 |
| REQ-PA-02 | Primary sensor VDD comes from +3V3_CHAIN through a local load switch, default OFF, with defined discharge | C | p.8, p.12 |
| REQ-PA-03 | Primary signal isolation: 3 sensor lines + 1 switched ID-resistor branch | C | p.8 |
| REQ-PA-04 | Populate exactly one of T/RH or mic. The unpopulated branch is DNI or isolated so it cannot load or drive the bus | C | p.8 |
| REQ-PA-05 | +3V3_CHAIN powers always-on patch logic and feeds the local switched sensor rail | C | p.4 |

## Auxiliary expansion (new in Rev 0.2)
| ID | Requirement | St | Src |
|---|---|---|---|
| REQ-AUX-01 | **Every** patch has 4 user-accessible headers **J_AUX1, J_AUX2, J_AUX3, J_AUX4** | C (per-patch count to be confirmed, OQ-002) | p.4, p.9, p.20 |
| REQ-AUX-02 | Intended for selected external modules: methane / combustible gas, CO, other environmental | C (intent) | p.4, p.9 |
| REQ-AUX-03 | Each header is separately identified in the schematic **and** the silkscreen | C | p.8 |
| REQ-AUX-04 | Headers must **not** be wired in parallel to one bus. A selected-port switch/mux is mandatory | C | p.5, p.10 |
| REQ-AUX-05 | Controlled module power per selected port: load switch, default OFF | C (preferred approach) | p.10 |
| REQ-AUX-06 | ESD / transient / overvoltage protection sized to connector exposure and module output range | C | p.10 |
| REQ-AUX-07 | A 5 V analog output must never connect directly to an ESP32-S3 GPIO. Use a divider / buffer / clamp | C | p.10 |
| REQ-AUX-08 | Operating cycle: select one port, power it only when needed, route signals, read, return to a safe inactive state | C | p.9 |
| REQ-AUX-09 | An approved module compatibility list is required. Modules are not plug-and-play | C | p.9 |
| REQ-AUX-10 | No SPI support claimed on the 4-contact concept | C | p.10 |
| REQ-AUX-11 | AUX power must have its own budget. Do not assume +3V3_CHAIN can power gas modules | C | p.4, p.10, p.12 |
| REQ-AUX-12 | Release documents include AUX pinout/voltage labels, supported module list, max current per port and safety warnings | C | p.19 |

## Design-practice requirements
| ID | Requirement | Src |
|---|---|---|
| REQ-DP-01 | Keep gas openings, acoustic ports and the T/RH sensing area clear of enclosure blockage and heat sources | p.12 |
| REQ-DP-02 | FPC orientation, pin-1 and IN/OUT silkscreen unmistakable. Validate with a 1:1 print and continuity fixture | p.12 |
| REQ-DP-03 | Review condensation, dust, propolis, vibration, flex bend radius, strain relief, pogo compression / cycle life | p.12 |
| REQ-DP-04 | Distinguish metric vs imperial package codes (0404 metric ≠ 0402 imperial). Freeze footprints against MFR drawings | p.12, p.15 |
| REQ-DP-05 | Every BOM line: unique RefDes, manufacturer, exact MPN, qty, value, tolerance, V/I rating, package, footprint; lifecycle + ≥1 alternate for critical parts | p.15 |
| REQ-DP-06 | For each external module record MPN, supply voltage, max/avg current, protocol, logic level, warm-up, calibration, output limits | p.15 |

## Requirements NOT stated in the source (gaps → see 08_open_questions.md)
Battery-life target, sampling intervals, audio format/duration, data storage/logging, wireless or cloud connectivity,
operating temperature/humidity range, ingress protection, enclosure, cost target, quantity, certification
(EMC/radio/hazardous gas), maximum chain length in metres, patch dimensions.
