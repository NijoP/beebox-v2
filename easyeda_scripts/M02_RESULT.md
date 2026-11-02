# M02 — ESP32-S3 Core · Capture Result

**Sheet:** P1 (schematic page `3d113e2c2213fddc`), project BEE BOX V2
**Date:** 2026-11-02 · **Style:** client net-label-driven (stub + blue net label per pin)
**Scripts:** `10_M02_place_U1.js`, `11_M02_label_U1.js`, `13_M02_passives.js` (verify: `12b`, `14`)

## Components placed (11)
| Ref | Part | MPN | LCSC |
|-----|------|-----|------|
| U1 | ESP32-S3 module | ESP32-S3-WROOM-1-N16R2 | C2913205 |
| C1 | +3V3 bulk 22µF | CL21A226MAQNNNE | C45783 |
| C2,C3 | +3V3 HF 100nF | CL05B104KO5NNNC | C1525 |
| C4 | EN RC 1µF | CL05A105KA5NQNC | C52923 |
| C5 | BOOT 10nF | CL05B103KB5NNNC | C15195 |
| R1,R2 | 10kΩ pull-ups | 0402WGF1002TCE | C25744 |
| SW1,SW2 | tactile (EN,BOOT) | TS-1187A-B-A-B | C318884 |

## Verified nets (netlist, Protel2)
- `+3V3` (6): U1.3V3, C1.1, C2.1, C3.1, R1.1, R2.1
- `GND` (12): U1.1/40/41, C1.2, C2.2, C3.2, C4.2, C5.2, SW1.2/4, SW2.2/4
- `MCU_EN` (5): U1.EN, R1.2 (pull-up), C4.1 (RC to GND), SW1.1/3 (press→GND reset)
- `MCU_BOOT` (5): U1.IO0, R2.2 (pull-up), C5.1 (debounce to GND), SW2.1/3 (press→GND)
- 28 GPIO/function stub nets (1 pin each) waiting for their owning sheets:
  SENSOR_DATA_ID, SENSOR_CLK, SENSOR_SYNC, SENSOR_PU_EN, CHAIN_PWR_EN, CHAIN_FAULT_N,
  SYS_SDA, SYS_SCL, PATCH_SHIFT_DATA, PATCH_SHIFT_CLK, PATCH_LATCH, PATCH_OE_N,
  DISP_PWR_EN, REG_PS_SYNC, CHG_EN1, CHG_EN2, USB_DM, USB_DP, CHG_STAT_N, CHG_PGOOD_N,
  REG_PG, GAUGE_ALRT_N, LED_R, LED_G, LED_B, VBAT_SENSE, UART0_TX, UART0_RX
- Reserved pins labeled by IO number: IO3, IO38, IO42, IO45, IO46, IO47, IO48

Totals: 11 components, 63 wires, 39 nets. No shorts. Additive only; nothing pre-existing overwritten.

## Open cosmetic item
Value display currently shows MPN (BOM is correct). Optionally set a short `Value` (22µF, 10kΩ…) for readability to match the reference sheet.
