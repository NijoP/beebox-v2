# Sensor-patch four-configuration audit (from exported netlist)

Source: `netlist/patch_full.net` (EasyEDA Schematic2, 51 parts, 36 nets).

DNP group T (fit only with SHT45): ['U5', 'C10', 'R14']

DNP group M (fit only with mic): ['U6', 'C11', 'R15']

AND gates: ['U10', 'U11']


## Configuration A (SHT45 only)
DNP: ['U6', 'C11', 'R15']

- Floating (single-pin) nets: **none**; empty nets: none
- GND-function pins on non-GND nets (shorts): **none**
- SHT45 operational: **YES**
- Microphone operational: **not fitted**
- Chain pass-through pins 1-7,9-12: **PASS**; pin-8 cascade: **PASS**
- AUX header (U9) on MOD_3V3/SDA/SCL: **PASS**
- ID reads (V at ESP32 ADC, ±0.32 V worst-case leakage): patch-ID 2.70 V · PRES_T 1.89 V · PRES_M 0.00 V

| Net | Members after DNP |
|---|---|
| SENSOR_CLK | FPC1-4(4), FPC2-4(4), U4-3(D1), U8-3(D1) |
| SENSOR_SYNC | FPC1-5(5), FPC2-5(5), U4-6(D2), U8-6(D2) |
| SENSOR_DATA_ID | FPC1-7(7), FPC2-7(7), U4-8(D3), U4-11(D4), U8-8(D3), U8-11(D4) |
| MOD_3V3 | U7-6(OUT), C13-1(1), U11-2(B), U5-3(VDD), U9-1(1), D1-4(D2+), C10-1(1) |
| MOD_SDA | U8-5(S2), U5-1(SDA), U9-3(3), D1-1(D1+) |
| MOD_SCL | U8-2(S1), U5-2(SCL), U9-4(4), D1-2() |
| MOD_SEL | U11-4(Y), U8-1(SEL1), U8-4(SEL2) |
| PRI_VDD | U3-5(QOD), U3-6(VOUT), C8-1(1), U10-2(B) |
| MIC_SEL | U10-4(Y), U4-1(SEL1), U4-4(SEL2), U4-10(SEL3) |
| CLK_LOC | U4-2(S1), R16-1(1) |
| SYNC_LOC | U4-5(S2), R17-1(1) |
| SD_LOC | U4-9(S3), R18-1(1) |
| ID_NODE | R9-2(2), U4-12(S4) |
| PRES_T | R14-2(2), R19-1(1), U8-9(S3) |
| PRES_M | R20-1(1), U8-12(S4) |

## Configuration B (mic only)
DNP: ['U5', 'C10', 'R14']

- Floating (single-pin) nets: **none**; empty nets: none
- GND-function pins on non-GND nets (shorts): **none**
- SHT45 operational: **not fitted**
- Microphone operational: **YES**
- Chain pass-through pins 1-7,9-12: **PASS**; pin-8 cascade: **PASS**
- AUX header (U9) on MOD_3V3/SDA/SCL: **PASS**
- ID reads (V at ESP32 ADC, ±0.32 V worst-case leakage): patch-ID 2.70 V · PRES_T 0.00 V · PRES_M 1.89 V

| Net | Members after DNP |
|---|---|
| SENSOR_CLK | FPC1-4(4), FPC2-4(4), U4-3(D1), U8-3(D1) |
| SENSOR_SYNC | FPC1-5(5), FPC2-5(5), U4-6(D2), U8-6(D2) |
| SENSOR_DATA_ID | FPC1-7(7), FPC2-7(7), U4-8(D3), U4-11(D4), U8-8(D3), U8-11(D4) |
| MOD_3V3 | U7-6(OUT), C13-1(1), U11-2(B), U9-1(1), D1-4(D2+) |
| MOD_SDA | U8-5(S2), U9-3(3), D1-1(D1+) |
| MOD_SCL | U8-2(S1), U9-4(4), D1-2() |
| MOD_SEL | U11-4(Y), U8-1(SEL1), U8-4(SEL2) |
| PRI_VDD | U3-5(QOD), U3-6(VOUT), C8-1(1), U10-2(B), U6-5(VDD), C11-1(1) |
| MIC_SEL | U10-4(Y), U4-1(SEL1), U4-4(SEL2), U4-10(SEL3) |
| CLK_LOC | U4-2(S1), U6-4(SCK), R16-1(1) |
| SYNC_LOC | U4-5(S2), U6-1(WS), R17-1(1) |
| SD_LOC | U4-9(S3), U6-6(SD), R18-1(1) |
| ID_NODE | R9-2(2), U4-12(S4) |
| PRES_T | R19-1(1), U8-9(S3) |
| PRES_M | R15-2(2), R20-1(1), U8-12(S4) |

## Configuration C (both)
DNP: none

- Floating (single-pin) nets: **none**; empty nets: none
- GND-function pins on non-GND nets (shorts): **none**
- SHT45 operational: **YES**
- Microphone operational: **YES**
- Chain pass-through pins 1-7,9-12: **PASS**; pin-8 cascade: **PASS**
- AUX header (U9) on MOD_3V3/SDA/SCL: **PASS**
- ID reads (V at ESP32 ADC, ±0.32 V worst-case leakage): patch-ID 2.70 V · PRES_T 1.89 V · PRES_M 1.89 V

| Net | Members after DNP |
|---|---|
| SENSOR_CLK | FPC1-4(4), FPC2-4(4), U4-3(D1), U8-3(D1) |
| SENSOR_SYNC | FPC1-5(5), FPC2-5(5), U4-6(D2), U8-6(D2) |
| SENSOR_DATA_ID | FPC1-7(7), FPC2-7(7), U4-8(D3), U4-11(D4), U8-8(D3), U8-11(D4) |
| MOD_3V3 | U7-6(OUT), C13-1(1), U11-2(B), U5-3(VDD), U9-1(1), D1-4(D2+), C10-1(1) |
| MOD_SDA | U8-5(S2), U5-1(SDA), U9-3(3), D1-1(D1+) |
| MOD_SCL | U8-2(S1), U5-2(SCL), U9-4(4), D1-2() |
| MOD_SEL | U11-4(Y), U8-1(SEL1), U8-4(SEL2) |
| PRI_VDD | U3-5(QOD), U3-6(VOUT), C8-1(1), U10-2(B), U6-5(VDD), C11-1(1) |
| MIC_SEL | U10-4(Y), U4-1(SEL1), U4-4(SEL2), U4-10(SEL3) |
| CLK_LOC | U4-2(S1), U6-4(SCK), R16-1(1) |
| SYNC_LOC | U4-5(S2), U6-1(WS), R17-1(1) |
| SD_LOC | U4-9(S3), U6-6(SD), R18-1(1) |
| ID_NODE | R9-2(2), U4-12(S4) |
| PRES_T | R14-2(2), R19-1(1), U8-9(S3) |
| PRES_M | R15-2(2), R20-1(1), U8-12(S4) |

## Configuration D (neither)
DNP: ['U5', 'C10', 'R14', 'U6', 'C11', 'R15']

- Floating (single-pin) nets: **none**; empty nets: none
- GND-function pins on non-GND nets (shorts): **none**
- SHT45 operational: **not fitted**
- Microphone operational: **not fitted**
- Chain pass-through pins 1-7,9-12: **PASS**; pin-8 cascade: **PASS**
- AUX header (U9) on MOD_3V3/SDA/SCL: **PASS**
- ID reads (V at ESP32 ADC, ±0.32 V worst-case leakage): patch-ID 2.70 V · PRES_T 0.00 V · PRES_M 0.00 V

| Net | Members after DNP |
|---|---|
| SENSOR_CLK | FPC1-4(4), FPC2-4(4), U4-3(D1), U8-3(D1) |
| SENSOR_SYNC | FPC1-5(5), FPC2-5(5), U4-6(D2), U8-6(D2) |
| SENSOR_DATA_ID | FPC1-7(7), FPC2-7(7), U4-8(D3), U4-11(D4), U8-8(D3), U8-11(D4) |
| MOD_3V3 | U7-6(OUT), C13-1(1), U11-2(B), U9-1(1), D1-4(D2+) |
| MOD_SDA | U8-5(S2), U9-3(3), D1-1(D1+) |
| MOD_SCL | U8-2(S1), U9-4(4), D1-2() |
| MOD_SEL | U11-4(Y), U8-1(SEL1), U8-4(SEL2) |
| PRI_VDD | U3-5(QOD), U3-6(VOUT), C8-1(1), U10-2(B) |
| MIC_SEL | U10-4(Y), U4-1(SEL1), U4-4(SEL2), U4-10(SEL3) |
| CLK_LOC | U4-2(S1), R16-1(1) |
| SYNC_LOC | U4-5(S2), R17-1(1) |
| SD_LOC | U4-9(S3), R18-1(1) |
| ID_NODE | R9-2(2), U4-12(S4) |
| PRES_T | R19-1(1), U8-9(S3) |
| PRES_M | R20-1(1), U8-12(S4) |

## Hardware gating evidence (all configurations)
- **PRI_SIG_EN**: U2-1(QB), R5-1(1), U10-1(A)
- **MIC_SEL**: U10-4(Y), U4-1(SEL1), U4-4(SEL2), U4-10(SEL3)
- **MOD_SIG_EN**: U2-4(QE), R8-1(1), U11-1(A)
- **MOD_SEL**: U11-4(Y), U8-1(SEL1), U8-4(SEL2)
- **PRES_T_EN**: U2-5(QF), R10-1(1), U8-10(SEL3)
- **PRES_M_EN**: U2-6(QG), R13-1(1), U8-13(SEL4)
- **ID_EN**: U2-2(QC), R6-1(1), U4-13(SEL4)

## Verification matrix
| Config | Floating nets | Shorts | SHT45 | Mic | Pass-through | AUX | ID / PRES_T / PRES_M (V) | Decode |
|---|---|---|---|---|---|---|---|---|
| A (SHT45 only) | none | none | ✓ | DNP | ✓ | ✓ | 2.70 / 1.89 / 0.00 | SHT45 only |
| B (mic only) | none | none | DNP | ✓ | ✓ | ✓ | 2.70 / 0.00 / 1.89 | mic only |
| C (both) | none | none | ✓ | ✓ | ✓ | ✓ | 2.70 / 1.89 / 1.89 | both |
| D (neither) | none | none | DNP | DNP | ✓ | ✓ | 2.70 / 0.00 / 0.00 | neither |