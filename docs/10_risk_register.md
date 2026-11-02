# 10 — Risk Register

## 10.1 Source risk register [p.17]
| ID | Risk | Impact | Mitigation / closure evidence | Priority |
|---|---|---|---|---|
| RISK-01 | AUX module voltage/current unknown | Header may damage module or brown out chain | Select representative MPNs; compute peak/inrush; define V_AUX + current limit; test at patch 8 | **Critical** |
| RISK-02 | Four AUX ports tied together | Bus contention, false readings, back-power | One-of-four mux/switch matrix; verify off-isolation, port-by-port operation | **Critical** |
| RISK-03 | Analog output exceeds ADC range | ESP32-S3 damage / wrong readings | Divider/buffer/clamp; tolerance analysis; validate max fault voltage | **Critical** |
| RISK-04 | AUX protocol diversity | Some modules can't use 2 shared lines | Approved compatibility matrix; larger header/adapter | High |
| RISK-05 | Shift-register outputs insufficient | Can't select/power all ports safely | Reallocate Q3–Q7 only after control budget; add register/decoder if needed | High |
| RISK-06 | AUX module ID not defined | Firmware can't identify module/port | Resistor ID / EEPROM / config profile per port; unknown modules fail safe | High |
| RISK-07 | 5 V or heater modules on 3.3 V chain | Rail collapse, heat, battery drain | Separate boost/power strategy; one-at-a-time power; thermal + chain current calc | **Critical** |
| RISK-08 | Shared ADC / I²S GPIO fails | Patch ID or mic capture won't work | Bench-test exact module; add mux or revise GPIO | **Critical** |
| RISK-09 | Mic lifecycle gap | BOM obsolescence | Qualify current-production I²S mic + alternate | High |
| RISK-10 | Long-chain signal degradation | Mic / I²C unreliable | Test full 8-position chain; scope at patch 1 and 8 | **Critical** |
| RISK-11 | 1S LiPo buck headroom | 3.3 V chain rail drops at low battery | Buck-boost or validated early cutoff; measure under load | High |
| RISK-12 | Connector orientation mirrored | Power/data pins reversed | MFR drawing review, pin-1 marks, 1:1 continuity fixture | **Critical** |
| RISK-13 | Mid-chain patch removal | Downstream patches unreachable | Document service limitation; bypass only if required | Medium |

## 10.2 Additional risks identified in KB study (KB-ANALYSIS — to be reviewed)
| ID | Risk | Impact | Proposed mitigation | Suggested priority |
|---|---|---|---|---|
| RISK-K01 | 5 free bits cannot provide per-port power + signal one-hot control | Forced redesign late | Decide option A–D in 14 §1 before P04 capture | High |
| RISK-K02 | Single +3V3_CHAIN contact current/drop limit with AUX loads | Brown-out at far patches | Get FH34 current rating + FFC resistance; compute worst-case at patch 8 | **Critical** |
| RISK-K03 | Heater-type gas sensors need long preheat; duty cycling gives invalid data | Unusable gas readings or excessive energy | Per-module acquisition profile; prefer low-power module technologies | High |
| RISK-K04 | Heated sensors near colony bias T/RH and may affect bees | Measurement error / animal impact | Placement rules; thermal test (V-14) | Medium |
| RISK-K05 | 5 V module pull-ups back-drive 3.3 V shared bus | ESP32-S3 / patch damage | Level translation/isolation after AUX mux; V-06/V-10 | **Critical** |
| RISK-K06 | AUX analog needs ADC pin but I²C SCL convention uses CLK line | Routing ambiguity → wrong design | Close OQ-007 in P2 | High |
| RISK-K07 | 595 outputs Hi-Z when OE_N high → floating enables | Unintended power-on at boot | Pull-downs on all enable nets; pull-up on OE_N | High |
| RISK-K08 | Long multi-drop SHIFT_CLK/LATCH edges double-clock HC inputs | Corrupted control image | Series R / buffering; V-02 at max length | Medium |
| RISK-K09 | User-accessible headers inside hive (propolis, moisture, bees, ESD from users) | Corrosion, shorts, ESD damage | Protection, coatings, keyed covered connectors | High |
| RISK-K10 | Patch size/BOM growth from 4 AUX ports | Mechanical/cost targets missed | Define outline and cost target (OQ-018) | Medium |
| RISK-K11 | Old repository pin assignment leaks into new schematic/firmware | Interface mismatch | Single ICD (04) as authority; netlist compare | High |
| RISK-K12 | Agent/script writes to wrong EasyEDA project/sheet | Corrupted design | Read-only context check before every write; one writer at a time; saved revision before each script | High |
