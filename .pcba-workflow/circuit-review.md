# BeeBox V2 Main Controller — Circuit Review
Method: `design-and-review-circuit` skill checklist · Source of truth: EasyEDA project "BEE BOX V2", page P1 · Evidence: live netlist (Protel2), datasheets in `bom/datasheets/`.

## Operating principle
Battery/USB-powered ESP32-S3 controller. USB-C charges a 1S Li-ion via BQ24074 (power-path); system rail VSYS feeds TPS63802→+3V3 (logic/sensors) and TPS61023→+5V0→TPS2553(current-limited)→+5V_CHAIN (sensor-patch chain, pins shared by FPC + pogo). ESP32-S3 is flashed/debugged over native USB. Onboard SHT45 (I²C) and ICS-43434 (I²S) sensors; 4-pad OLED header; status LEDs; soft-latch power button.

## Power tree
USB VBUS(5V) → BQ24074 → VSYS → {TPS63802→+3V3 (3.31 V), TPS61023→+5V0 (5.1 V)→TPS2553→+5V_CHAIN(≤0.39 A)} ; VBAT(1S) ↔ BQ24074 BAT.

## Voltage domains
+5V (VBUS/VSYS≤4.2) power-only · +5V_CHAIN (off-board) · +3V3 logic · all MCU IO 3.3 V. No >3.6 V into ESP32. ✓

## Operating states
- **Off (default):** PWR_EN pulled low (R22 100k) → regulators off; USB charging still works. ✓
- **Power-on:** SW_PWR → PWR_EN high → regs on → ESP32 boots → holds via PWR_HOLD(IO47); PWR_BTN(IO48) senses for shutdown. ✓
- **Programming:** native USB (D± on IO19/20, ESD-protected); auto download-mode, no button needed. ✓
- **Chain:** off until firmware (CHAIN_PWR_EN pull-down added). ✓

## Findings
| # | Sev | Net | Condition | Action | Status |
|---|-----|-----|-----------|--------|--------|
| 1 | MAJOR | CHAIN_PWR_EN | No pull-down → TPS2553 EN floats at boot (chain power undefined) | Added 100 kΩ↓ | **FIXED** |
| 2 | MINOR | REG_PS_SYNC | TPS63802 MODE no default pull | Added 100 kΩ↓ | **FIXED** |
| 3 | MINOR | LED1/2/4/5 | Green/blue symbols number pins reversed vs red | Verified anode = right pin (netB) for all colors → all forward-biased | **VERIFIED OK** |
| 4 | MINOR | AUX_ARM | Not routed to an ADC (no spare ADC GPIO) | Diagnostic only; interlock is patch-side HW | USER_REVIEW |
| 5 | MINOR | IO45/IO46 | ESP32 strapping pins left floating | Acceptable for WROOM-1 module (internal defaults) | USER_REVIEW |
| 6 | MINOR | CC1/CC2, VBUS | No ESD/TVS (D± protected; BQ OVP covers VBUS) | Optional add | INFO |
| 7 | INFO | VBAT_SENSE | 1M/330k always-on ~3 µA battery drain | Negligible | INFO |
| 8 | INFO | VBAT / CN1 | Battery connector polarity vs pack unconfirmed | Confirm before build | USER_REVIEW |
| 9 | INFO | SENSOR_DATA_ID | Onboard mic shares I²S SD with chain/ADC | Firmware: one source active at a time | INFO |
| 10 | INFO | spares | UART0_TX/RX, DISP_PWR_EN, GAUGE_ALRT_N, IO3/38/42/45/46, SBU1/2 single-pin | Intentional; add NC flags for 0-warning ERC if desired | INFO |

## Checklist results
- Protection: USB D± ESD (TPD2E2U06), chain 8-signal ESD (2×TPD4E05U06), chain OC (TPS2553 latch-off), charge OVP/thermal/termination (BQ24074). ✓
- Decoupling: every supply pin (ESP32 22µF+2×100nF, regs 10µF in/out, BQ 1µF/10µF/10µF, sensors 100nF). ✓
- Feedback dividers datasheet-verified (3.31 V, 5.1 V). Charge 0.40 A, chain limit 0.30 A. ✓
- Enable polarity/defaults: PWR_EN↓, CHAIN_PWR_EN↓, CHG_EN USB500 default, open-drain pull-ups on CHG#/PGOOD#/FAULT#. ✓
- Connectivity: 85 components, 71 nets, 0 unexpected floating nets. ✓

## Gate: USER_REVIEW
No critical/major defect remains (both fixed). Open engineering confirmations: battery polarity (#8), optional AUX_ARM ADC (#4), strap ties (#5), spare-pin NC flags (#10). Hand readability to schematic-humanizer, parts to sourcing, layout to pcb-layout-review.
