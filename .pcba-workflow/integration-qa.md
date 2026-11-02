# BeeBox V2 — Main Controller ↔ Sensor Patch Integration QA
Evidence: live netlists of Schematic1 (main, 87 parts / 71 nets) and Schematic2 (patch, 39 parts / 30 nets).

## Chain pin-by-pin (main FPC1 = pogo H1  →  patch J_IN, passes to J_OUT)
| Pin | Main net | Patch net | Patch load | Check |
|---|---|---|---|---|
| 1 | GND | GND | ground pour | ✓ |
| 2 | +5V_CHAIN (TPS2553-1, 302–394 mA, latch-off, default OFF) | +5V_CHAIN | TPS62840 VIN/EN, 4.7µF+10µF+100nF | ✓ |
| 3 | GND | GND | | ✓ |
| 4 | SENSOR_CLK_C (33Ω, ESD) | SENSOR_CLK | TMUX1511 D1 (primary), TMUX1511 D1 (module) | ✓ |
| 5 | SENSOR_SYNC_C (33Ω, ESD) | SENSOR_SYNC | TMUX1511 D2 ×2 | ✓ |
| 6 | GND | GND | | ✓ |
| 7 | SENSOR_DATA_ID (100k↓, ADC1 IO1, ESD) | SENSOR_DATA_ID | TMUX1511 D3 (mic SD), D4 (ID) | ✓ |
| 8 | PATCH_DATA_C (33Ω, ESD) | J_IN8 = PATCH_DATA_IN → SER; QH'→33Ω→J_OUT8 | re-timed per pad | ✓ |
| 9 | PATCH_CLK_C (33Ω) | PATCH_CLK | 595 SRCLK (pass-through) | ✓ |
| 10 | PATCH_LATCH_C (33Ω) | PATCH_LATCH | 595 RCLK | ✓ |
| 11 | PATCH_OE_N_C (1k) | PATCH_OE_N | 595 OE + 100k pull-up to +3V3_LOCAL | ✓ |
| 12 | **GND** | **GND** | 4th return (was AUX_ARM; interlock removed) | ✓ |

## Electrical margins (worst case: 8 pads, 0.8 m FFC, 1 active pad)
- Patch-8 VIN with 0.26 A (module at 280 mA limit): ≈4.1 V ≥ 3.4 V needed by TPS62840 → ✓
- Ground offset patch 8: 0.19 V (4 GND) → ESP32 I²C VOL ≈0.69 V < VIL 0.825 V → ✓ (0.13 V margin)
- Module short: port limit ≈280 mA @3.3 V ≈0.22 A @5 V < chain min 302 mA → only the port trips → ✓
- Inrush: ≈130 µF @ 0.34 A → ≈2 ms < 7.5 ms latch → no false trip → ✓
- OE: 8×100k pull-ups (12.5k) vs ESP32 via 1k → V_low ≈0.24 V → ✓
- ID: 22k→2.70 V (T/RH), 68k→1.96 V (mic), 0 V = no pad (100k↓) → ADC windows separated → ✓

## Defects found & fixed this pass
1. I²S SD contention: main onboard mic L/R → +3V3 (right); patch mics left → no conflict, both usable.
2. Power button: GPIO saw VSYS (4.4 V) + back-power when off + shutdown press undetectable → rebuilt: BAT54WS diode-OR (button / PWR_HOLD) onto PWR_EN, AO3400 level-shift to PWR_BTN (active-low, pulled up to 3V3).
3. Library "BAT54C" symbol pinout = common-anode (wrong for this use) → rejected; single BAT54WS used.
4. TPS2552-1 active-low EN (would power port at reset) → TPS2553-1 active-high.
5. Patch simplified: 4-port AUX system removed → ONE 4-pin module header (client pinout), own current-limited switch + I²C isolation + ESD.
6. Pin 12 AUX_ARM → GND on both boards (+4th return).

## Firmware contract (required for safe operation)
- Chain ON → CHAIN_INIT (shift 64 zeros, latch) → release OE. Before chain OFF: drive chain GPIOs low, OE high.
- One pad active at a time: PWR on → 1 ms → SIG on → read → SIG off → PWR off → next.
- Discovery each boot: probe ID_EN of pad 1..8, stop at first ≈0 V.
- SENSOR_PU_EN high for I²C (SHT45/module), Hi-Z for I²S. Do not clock I²S during ID reads.
- Battery: cut off (release PWR_HOLD) at VBAT_SENSE < 3.3 V; pack must include a PCM.
- Hot-plugging FFC while chain powered not supported.

## Open (client)
Patch variant per pad (T/RH or mic); FFC type (same-side contacts) & segment length; battery pack datasheet/polarity.

## Rev: 6S battery + BMS power architecture (replaces 1S/BQ24074)
BMS (6S, external 25.2 V CC/CV charger on P+/P-) is protection only — it cannot charge from USB. BQ24074, TPS61023, 1S JST, CHG LED removed.
Path: J_BAT(XH-2, keyed) → F1 PTC 1.1A/33V → D6 SS34 40V (reverse polarity) → VIN_PROT (TVS SMBJ26A, 2×10µF/50V, 100nF/50V)
→ LMR51420 (VIN 4.5–36V, abs 38V; EN=VIN; L 10µH; CB 100nF; FB 100k/13.7k → 4.98V; Cout 2×22µF/25V) → +5V0
→ TPS63802 → +3V3 ; TPS2553-1 → +5V_CHAIN. USB VBUS → SS14 → +5V0 (diode-OR backup for programming).
VBAT_SENSE = VIN_PROT·100k/1.1M (25.2V→2.26V). VBUS_DET = VBUS·100k/147k (5V→3.40V) on IO21.
Checks: Iin ≈0.28A @18V ≪ PTC hold; D=0.2 → t_on 400ns ≫ 70ns min; TVS VRWM 26V > 25.2V; patch-8 VIN ≈3.95V ✓.
Firmware: low-battery release PWR_HOLD at VBAT ≈19.8V (3.3V/cell); BMS hard cutoff below. USB-only (VBAT_SENSE≈0): limit chain load to stay < 500 mA.
Note: TVS clamp (~42V) > LMR51420 38V abs max only under fast surge; pack is low-impedance and input has 20µF/50V — acceptable for a short pack harness; keep the BMS-to-board cable short.

## Rev: user-story alignment (2026-11-02)
- Battery best practice: BMS = Robu "6S 20A Li-ion 24V Charger Protection Board" ₹219 (SKU 456509): OC 4.28V/cell, OD 2.55V/cell, OCP 30A, short 250µs, Iq ≤30µA. + Robu "6S 22.2V Equalizer Board" ₹209 at the cell taps (balancing). Pack-level, no schematic impact.
- Main: charge jack DC1 (DC-005-5A, centre +) → F2 BSMD1812-200-33V (2A/33V) → BAT_IN (= J_BAT pin1 = BMS P+). Charge current bypasses the board's SS34/buck path. Charger: 6S CC/CV 25.2V, 1–1.5A.
  - Defect caught in verification: first placement shorted CHG_IN to GND (jack pin-3 stub ended on F2 pin) → F2 moved, re-verified clean.
- Patch: SHT45 moved to the module I²C segment (MOD_SDA/SCL, MOD_3V3, switched by Q3/Q4); ICS-43434 alone on the primary I²S path (Q0/Q1). Both fitted on every patch, electrically isolated from each other. Single ID 22k (2.70V). R 68k removed. 0x44 reserved on the module bus.

## Rev: optional sensor population (A/B/C/D) + RTC (2026-11-02)
Evidence: `netlist/patch_full.net` (exported), `netlist/config_audit.py` → `netlist/config_audit_report.md` (DNP pins removed from the real netlist per config).
- Patch: mic on PRI rail (Q0, TPS22918) + TMUX1511#1 ch1-3; SHT45 on MOD rail (Q3, TPS2553) + TMUX1511#2 ch1-2 (shared with the single AUX header J 1 3V3/2 GND/3 SDA/4 SCL).
- Hardware gating: MIC_SEL = Q1·PRI_VDD (U10 LVC1G08), MOD_SEL = Q4·MOD_3V3 (U11) → signals connect only when the sensor rail is up (no back-powering, independent of firmware).
- Rejected: powering TMUX from sensor rails (I_POFF up to ±2 µA ×32 channels × 100k = 6.4 V ID error); TMUX stay on +3V3_LOCAL (I_S(OFF) ±100 nA → ≤0.32 V worst case).
- Presence: Q2 → 22k ID 2.70 V (patch present); Q5 → TMUX2 ch3 → R14 68k (fit with SHT45) 1.89 V / 0 V; Q6 → TMUX2 ch4 → R15 68k (fit with mic) 1.89 V / 0 V. 1M pull-downs on PRES_T/PRES_M; 100k pull-downs on CLK/SYNC/SD_LOC.
- DNP groups: T = U5 SHT45, C10, R14; M = U6 ICS-43434, C11, R15.
- Matrix: A,B,C,D → 0 floating nets, 0 shorts, pass-through+cascade PASS, AUX PASS, decode unique.
- Main: RTC RV-3028-C7 (C2829066, pinout verified vs LCSC datasheet) on SYS I²C 0x52, INT#→IO17 (100k pull-up), VBACKUP = CR2032 (BS-6) + 100nF, EVI→GND, CLKOUT NC. OPEN: verify BT1 symbol pin1 = '+' against footprint silkscreen.
- OPEN (blocking decision): global AUX hardware interlock is NOT present (pin 12 = GND since simplification); single-AUX-active is firmware-enforced.
