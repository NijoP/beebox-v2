# Main Controller — Design QA & Verification (pre-build)

Date 2026-11-02 · all devices from LCSC lib `0819f05c4eef4c71ace90d822a990e87` · style: net-label driven.

## Power architecture (validated)
`USB-C VBUS → BQ24074 (charger+path+protections) → VSYS → {TPS63802→+3V3, TPS61023→+5V0→TPS2553→+5V_CHAIN}`
Battery (1S Li-ion 3000 mAh) on BQ24074 BAT. Firmware upload via ESP32-S3 native USB (D±).

## Verified parameters (datasheet-checked)
| Block | Param | Formula / source | Value | Result |
|---|---|---|---|---|
| TPS63802 +3V3 | FB 562k/100k | Vout=0.5·(1+Rt/Rb) | 562k/100k | **3.31 V** ✓ (ESP32 3.0–3.6) |
| TPS61023 +5V0 | FB 750k/100k | Vout=0.6·(1+Rt/Rb) | 750k/100k | **5.10 V** ✓ (<5.5) |
| TPS2553-1 chain | R_ILIM | I_OS=K/R_ILIM | 76.8k | **302–394 mA** ✓ overcurrent-protected |
| BQ24074 charge | R_ISET | I_CHG=890/R | 2.2k | **0.40 A** ✓ (0.13C, safe) |
| BQ24074 input | R_ILIM | I_IN=1610/R | 2.2k | **0.73 A** ceiling ✓ |
| BQ24074 timer | R_TMR | 18k<R<72k | 47k | in range ✓ |
| BQ24074 TS | no NTC | datasheet: 10k→VSS | 10k | TS valid ✓ |
| BQ24074 TD(pin15) | termination | GND=enable | →GND | term. enabled ✓ (**symbol mislabels it "ITERM"; real fn = Termination Disable**) |
| BQ24074 CE#(4) | enable | low=enable | →GND | charger on ✓ |
| BQ24074 EN1/EN2 | mode | EN2=0,EN1=1→USB500 | 100k pu/pd | safe default USB500 ✓ |
| USB-C CC1/CC2 | sink Rd | 5.1k→GND | 5.1k ×2 | UFP sink ✓ |
| VBAT sense | divider | 1M/330k | ratio 0.248 | 4.2V→1.04V ✓ ADC |
| SHT45 | decouple | Sensirion app | 100nF | ✓ |
| ICS-43434 | decouple + L/R | TDK datasheet | 100nF, L/R→GND(left) | ✓ |
| PATCH_OE_N | series | startup_safety mirror-insert | 1k | ✓ |
| data/clk/latch, sensor clk/sync | series term | main_board | 33Ω | ✓ |

## Protections
- USB D± ESD: TPD2E2U06. Chain 8 signals ESD: 2× TPD4E05U06.
- Chain overcurrent/latch-off/reverse: TPS2553-1. Battery charge OVP(6.5V)/thermal/termination/timer: BQ24074.
- Safe default: chain off (CHAIN_PWR_EN pull-down on ESP32 side), USB500 at plug-in.

## 12-pin chain nets (FPC + POGO share identical nets; A-01 pin12=AUX_ARM)
`1 GND · 2 +5V_CHAIN · 3 GND · 4 SENSOR_CLK_C · 5 SENSOR_SYNC_C · 6 GND · 7 SENSOR_DATA_ID · 8 PATCH_DATA_C · 9 PATCH_CLK_C · 10 PATCH_LATCH_C · 11 PATCH_OE_N_C · 12 AUX_ARM`
FPC shield pins 13/14 → GND. Series R separate MCU-side (PATCH_SHIFT_*) from chain-side (*_C). SENSOR_DATA_ID direct (100k pull-down). AUX_ARM 100k pull-down on main board.

## Onboard sensors
- SHT45 (U6) on SYS I²C (SYS_SDA/SCL, shared w/ display), 100nF, EP→GND.
- ICS-43434 (U7) on I²S (SCK=SENSOR_CLK, WS=SENSOR_SYNC, SD=SENSOR_DATA_ID, MCU-side), L/R→GND, 100nF.

## Net completeness (no floating signal nets)
All M02 GPIO nets consumed: USB_DM/DP→USB; CHG_*→BQ; REG_PS_SYNC→MODE, REG_PG→PG; CHAIN_*→TPS2553; VBAT_SENSE→divider; SENSOR_*→series R/sensors/chain; SYS_SDA/SCL→display+SHT45; LED_*→LEDs; MCU_EN/BOOT & UART0_*→debug hdr; DISP_PWR_EN, GAUGE_ALRT_N, IO38, IO42, AUX_ARM→debug/expansion hdr.
Intentional spares left as labeled stubs: IO3, IO45, IO46 (strapping), IO47, IO48.

## Grounding
Single GND net; IC thermal pads (BQ24074 EP, SHT45 EP) → GND; USB shield → GND; ground pour at PCB stage.
