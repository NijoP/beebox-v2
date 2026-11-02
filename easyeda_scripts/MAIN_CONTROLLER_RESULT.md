# Main Controller — Build Result (rebuild, grid-aligned)

**Status:** electrically COMPLETE & VERIFIED. 84 components, 71 nets, **no shorts, no unexpected floating nets.**
Only intentional spares single-pin: IO3/IO45/IO46 (ESP32 strapping), SBU1/SBU2 (USB-C unused).

## Subsystem layout (schematic units, 0.01 in; y up)
| Block | Region (x, y) | Contents |
|---|---|---|
| MCU (M02) | U1 @ (2000,2400); passives y≈2620–2760 | ESP32-S3-WROOM-1-N16R2 + decoupling + boot/reset buttons |
| Power (M01) | x 300–1700, y 1000–2900 | USB-C, BQ24074, TPS63802(+3V3), TPS61023(+5V0), TPS2553(+5V_CHAIN), USB-ESD, battery; R/C/L farm y 800–1480 |
| Power button | x 1800–2400, y 1500–1650 | SW_PWR + R_PWRPD(100k) + R_HOLD(1k,PWR_HOLD) + R_SENSE(100k,PWR_BTN) → regulator EN soft-latch |
| Chain ctrl (M03) | x 2800–3050, y 2050–2650 | 33Ω series ×5, 1k OE, 100k ID/AUX pull-downs, 2.2k sensor pull-ups |
| Status LEDs | x 3300–3560, y 2220–2700 | PWR(grn), CHG(red, from BQ CHG#), RGB LED_R/G/B + 1k each |
| Connectors (M04) | x 3650–4100, y 1850–2550 | FPC FH34 + POGO 2×6 (shared nets) + 2× chain ESD |
| Display/sensors (M05) | x 1600–3000, y 1350–1550 | SHT45 (SYS I²C), ICS-43434 (I²S), 4-pad OLED header, 2×6 debug/expansion; I²C pull-ups + decoupling |

## Verified key nets (netlist)
+3V3:23 · GND:81 · VSYS:10 · VBUS:7 · VBAT:5 · +5V0:5 · +5V_CHAIN:4 · FB_3V3:3 · FB_5V0:3
SENSOR_CLK_C/SYNC_C/PATCH_*_C:4 each · SENSOR_DATA_ID:6 · AUX_ARM:5 · SYS_SDA/SCL:4 · LEDR/G/B_A:2

## New additions this pass
- **Power button (soft-latch):** SW_PWR momentarily pulls PWR_EN (regulator EN rail) high; ESP32 holds via PWR_HOLD(IO47); PWR_BTN(IO48) senses for clean shutdown. Default off (100k pull-down) — USB charging still works when off.
- **Status LEDs:** dedicated PWR (green, on +3V3) and CHG (red, driven by BQ24074 CHG#) + user RGB (LED_R/G/B).

## Known tooling limitation
`getCurrentRenderedAreaImage` returns a stale cached frame (EDA window not repainting off-screen), so the agent cannot self-verify the visual. Connectivity verified by netlist. **Please visually confirm spacing/alignment on the live EasyEDA screen.**
