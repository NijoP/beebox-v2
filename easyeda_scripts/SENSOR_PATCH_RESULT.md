# Sensor Patch — Build Result (Schematic2 / P1)

No battery, no MCU. Powered by +5V_CHAIN (FPC pin 2), controlled by main-board ESP32 via 74LV595 (FPC pins 8–11).
**52 parts, 43 nets, 0 floating, 0 shorts** (netlist-verified). Schematic1 untouched (every script guarded on sheet uuid).

## Blocks
| Block | Parts | Key nets |
|---|---|---|
| Chain IN/OUT | 2× FH34SRJ-12S | pins 1–7,9–12 pass-through; pin8 IN→SER, QH'→33Ω→OUT |
| Local power | TPS62840DLCR, 2.2µH, VSET 267k (3.3 V per DS table), Cin 4.7µF, Cout 10µF, +10µF/100nF entry | +5V_CHAIN→+3V3_LOCAL |
| Shift register | SN74LV595APWR, OE 100k pull-up, SRCLR→3V3, 8×47k Q pull-downs, 100nF | fail-safe OFF |
| Primary | TPS22918 (CT 1nF, QOD→VOUT), TMUX1511 (ch1-3 CLK/SYNC/SD @Q1, ch4 ID @Q2), SHT45 + ICS-43434 (100nF each) | PRI_VDD, CLK/SYNC/SD_LOC |
| ID | R_ID 22k (T/RH) / 68k (mic) → ID_NODE → TMUX ch4 → SENSOR_DATA_ID | 2.70 V / 1.96 V with main 100k |
| AUX mux | TMUX1109 (A1:A0=Q4:Q3, EN=Q5) | AUXn_SDA/SCL ↔ SENSOR_SYNC/CLK |
| AUX power | 2× TPS2553DBVR-1 (active-high, latch-off), RILIM 93.1k (≈248–329 mA), FLT 100k pull-ups, 1µF in, 10µF out | AUX_3V3 (Q6), AUX_5V (Q7) |
| AUX ports | 4× JST B5B-PH-K-S + 4× TPD4E05U06 | 1 3V3 · 2 GND · 3 SDA · 4 SCL · 5 5V |

## Register map (patch)
Q0 PRI_PWR_EN · Q1 PRI_SIG_EN · Q2 ID_EN · Q3 AUX_A0 · Q4 AUX_A1 · Q5 AUX_SIG_EN · Q6 AUX3V3_EN · Q7 AUX5V_EN

## Corrections made during build
- TPS2552-1 (baseline) is **active-LOW EN** → would power AUX ports at reset; replaced by **TPS2553-1 (active-HIGH)**.
- AUX row label collisions (GND↔AUX5V_FLT_N, AUX_3V3↔AUX5V_ILIM, ESD NC↔AUX2_SCL) found by netlist trace; row rebuilt with collision-free spacing; re-verified clean.

## Decisions for client review
1. Mic: variant population (fit SHT45+22k OR ICS-43434+68k). Simultaneous T/RH+mic needs a 2nd control bit/register.
2. "5-pin header" implemented as 4 ports × 5-pin (pin 5 = switched 5 V, power only).
3. Q7 now AUX 5 V enable; AUX_ARM is pass-through only → cross-patch "one AUX port" is firmware-enforced (rev-D hardware window-comparator interlock omitted for simplicity).
4. Supervisor/BLOCK latch omitted; fail-safe relies on local OE pull-up + Q pull-downs.
5. AUX I²C pull-ups = main-board switched 2.2k (SENSOR_PU_EN high in AUX mode); modules must use 3.3 V logic.
6. Set sheet Size → A2 (no API).
