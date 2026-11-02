# 08 — Open Questions / Missing Information

Priority: **B** = blocks schematic capture of the affected block · **H** = needed before PCB/prototype · **M** = later.
Source: `[p.20]` = listed in the source's open-decision table; `KB` = gap found during the KB study.

## A. Client / product requirements
| ID | Question | Pri | Src |
|---|---|---|---|
| OQ-001 | Who is the client / end user, and who approves requirements (P0 sign-off)? What is the deployment scenario (number of hives, location, climate)? | H | KB |
| OQ-002 | Confirm **four AUX headers per patch** (32 ports on a full chain), not four in total for the system | **B** | p.20 |
| OQ-003 | Which measurements matter for the client: sampling interval per sensor type, audio sample rate / bit depth / clip length, and what methane/CO data is wanted? | H | KB, p.20 |
| OQ-004 | Data handling: local storage (flash/SD?), timestamps/RTC, Wi-Fi/BLE upload, cloud (out of scope per p.3, but is any link needed?) | H | KB |
| OQ-005 | Battery-life / autonomy target, charging method (USB? solar?), service interval | H | KB, p.20 |
| OQ-006 | Environmental spec: operating temperature/humidity range, condensation, ingress protection, propolis exposure, enclosure material, certification needs | H | KB, p.12 |

## B. Auxiliary expansion (gating — source says conditional GO only after these close)
| ID | Question | Pri | Src |
|---|---|---|---|
| OQ-007 | How do AUX_IO_A/B map onto the 3 shared chain lines for each protocol (I²C, UART, analog, PWM)? | **B** | KB (14 §2) |
| OQ-008 | Is Q3–Q7 enough, or is a second 595 / decoder needed? Is per-port power required or is shared V_AUX acceptable? | **B** | p.20 |
| OQ-009 | AUX identification method: resistor code, EEPROM, manual firmware profile, other | **B** | p.20 |
| OQ-011 | Exact methane, CO and other module MPNs, datasheets, preferred vendors (2–4 representative modules, P1) | **B** | p.20 |
| OQ-012 | V_AUX: 3.3 V / 5 V / other; current per port; max simultaneous current; boost/regulator; thermal limit; external supply allowed? | **B** | p.20 |
| OQ-013 | AUX connector geometry: pin count, pitch, locking/keying, THT/SMT, cable vs board mating, insertion-cycle target | **B** | p.20 |
| OQ-014 | Concurrency: is multi-port simultaneous operation a future requirement? (Affects mux/power sizing now) | H | p.20 |
| OQ-015 | Pre-heat / continuous-heater modules: acceptable to keep one powered continuously? Battery/heat implications | H | KB (14 §3) |
| OQ-016 | Analog AUX input voltage range to support (0–3.3 V? 0–5 V?) and required accuracy | H | KB |

## C. Main board / primary sensors
| ID | Question | Pri | Src |
|---|---|---|---|
| OQ-010 | ESP32-S3 module MPN (flash/PSRAM, antenna type) and final GPIO map | **B** for M02/M03 | p.6, p.20 |
| OQ-017 | Display module MPN, I²C address, voltage, onboard pull-ups; display content | H | p.20 |
| OQ-019 | Current-production I²S mic MPN, sample rate, bit depth, BCLK, acoustic port (top/bottom) | **B** for P03 mic variant | p.20 |
| OQ-020 | Battery cell + protection MPN, charger path, cutoff voltage, 3.3 V regulator (buck-boost?), optional 5 V AUX rail | **B** for M01 | p.20 |
| OQ-021 | Where do the SENSOR_CLK/SYNC I²C pull-ups sit (main vs patch, switchable?), given I²S push-pull reuse? | H | KB (14 §2) |
| OQ-022 | Battery/rail telemetry and fuel gauge selection | M | p.6 |
| OQ-023 | Status LED(s) / user button requirements on main board | M | KB |

## D. Mechanical / interconnect
| ID | Question | Pri | Src |
|---|---|---|---|
| OQ-018 | Patch dimensions/outline, FPC segment length, total chain length, enclosure, gas & acoustic apertures | H | p.20 |
| OQ-024 | FFC type (same-side vs opposite-side contacts) → pin mirroring between IN and OUT | **B** for P01 layout | p.7, KB |
| OQ-025 | Pogo implementation: contact MPN, geometry, compression, retention, cycle target | M | p.20 |
| OQ-026 | Main-board chain connector: FPC only or also pogo on the main board? | H | KB |
| OQ-027 | Is mid-chain patch removal / hot-plug a requirement (currently a documented limitation)? | M | p.17 |

## E. Process / tooling
| ID | Question | Pri | Src |
|---|---|---|---|
| OQ-028 | Should main board and patch be separate EasyEDA projects or boards within BEE BOX V2 (currently Board1 / Schematic1 / PCB1 / Panel1)? | **B** for P4 | KB |
| OQ-029 | Library policy: JLC/LCSC library parts vs custom symbols; assembly house (JLCPCB?) and its part availability | H | KB |
| OQ-030 | PyAgent validation tooling: exact installation and commands (source says "to be confirmed") | M | p.13 |
| OQ-031 | Relationship to the GitHub repo NijoP/BeeBox: which V1/V2 files are still valid (e.g. old pin assignment to be replaced) | H | p.7, R1 |
| OQ-032 | Units for the 0404 metric passive target: confirm the assembler supports it | M | p.12, p.15 |
