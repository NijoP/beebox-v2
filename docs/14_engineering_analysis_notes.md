# 14 — Engineering Analysis Notes (KB-ANALYSIS)

> **Not source content.** These notes are the KB author's engineering reading of BBX-ENG-PR-003 Rev 0.2. They
> point out consequences and gaps for review. They are **not** decisions. Figures marked "indicative" are typical
> class values and must be replaced by datasheet values.

## 1. Patch control-bit budget (feeds OQ-008, RISK-K01)
One 74HC595 = 8 bits. Q0–Q2 are allocated (primary power, primary signal enable, ID enable), leaving **5 bits (Q3–Q7)**.

| Option | Bits needed | Fits Q3–Q7? | Notes |
|---|---|---|---|
| A. One-hot: 4 × power-enable + 4 × signal-enable | 8 | **No** | Simplest logic. Needs a 2nd register per patch (control image grows to 128 bits) |
| B. Encoded: ADDR0, ADDR1, AUX_SIG_EN, AUX_PWR_EN, AUX_ID_EN + 1-of-4 decoder for power enables + 4:1 mux addressed by ADDR | 5 | **Yes, exactly, no spare** | Power and signal follow the same address, which matches the one-port-at-a-time rule. Adds a decoder IC |
| C. Encoded, shared V_AUX switch for all 4 ports | 4 | Yes | Powers every attached module at once. **Conflicts with V-05** ("inactive port off") |
| D. Second 595 per patch | 8 extra | n/a | Most flexible (room for level-shift enables, pull-up switching, future concurrency). Firmware handles 128-bit image; cascade unchanged |

Fail-safe detail: when PATCH_OE_N is high, the 595 outputs are **Hi-Z**, not low. Every enable line therefore needs
a pull-down (or pull-up for active-low enables) so OFF is guaranteed (REQ-SYS-06). PATCH_OE_N itself needs a
pull-up so it is high before the ESP32-S3 drives its GPIO. The 595's SRCLR (clear) has no chain contact, so it must be tied high locally
and power-up register contents are random until CHAIN_INIT. OE_N must therefore stay high until a zero image is latched.

## 2. AUX signal routing through the 12-contact chain (feeds OQ-007)
The chain has only **three** shared sensor lines: SENSOR_CLK (pin 4), SENSOR_SYNC (pin 5), SENSOR_DATA_ID (pin 7).
The source does not say how AUX_IO_A/B map onto them. Observations:
- **I²C AUX** (IO_A = SCL, IO_B = SDA) maps naturally onto SENSOR_CLK/SENSOR_SYNC, like the SHT45. Address conflicts
  are avoided only because one path is enabled at a time.
- **Analog AUX** (IO_A = analog) needs an ADC-capable main-board pin. At present only SENSOR_DATA_ID is required to be
  ADC1-capable. So either (a) IO_A is routed to pin 7 in analog mode and to pin 4 in I²C mode, which needs a
  protocol-dependent routing matrix on the patch, or (b) the main-board GPIOs for SENSOR_CLK/SYNC are also chosen
  ADC1-capable. Option (b) is cheaper on the patch, but constrains the GPIO map and puts the I²C pull-ups in the analog path.
- **Analog loading:** the main-board 100 k ID pull-down on SENSOR_DATA_ID stays connected and loads any analog
  source, so divider/buffer design must include it.
- **UART / PWM:** the ESP32-S3 GPIO matrix can map UART RX/TX or capture peripherals to these lines, so routing is
  feasible in principle. TX/RX direction per AUX pin still has to be defined.
- **Level translation:** many 5 V modules have onboard pull-ups to 5 V on I²C/UART. That would put 5 V onto the
  shared 3.3 V chain bus, so translation or isolation is mandatory. Placing it after the 4:1 mux (one per patch)
  costs less than one per port.
- **Bus loading / SI:** each patch adds the off-capacitance and leakage of the primary 4066 **and** the AUX mux on
  pins 4/5/7. Over 8 patches, plus the FFC, this degrades I²S BCLK edges at patch 8 (V-11). Prefer low-C switches.
- **Pull-up conflict:** I²C needs pull-ups on pins 4/5, while I²S BCLK/WS are push-pull. The source does not say where
  the sensor-bus pull-ups sit or whether they are switchable.

## 3. Power (feeds OQ-011..013, RISK-K02..K04)
- All patch and AUX current flows through **one** +3V3_CHAIN contact (pin 2) per connector, through up to 16 connector
  contacts (IN+OUT ×8) plus 8 FFC segments. Returns share 4 GND contacts.
- Illustrative only: with 300 mA delivered at patch 8 and 1 Ω total loop resistance, the drop is 0.3 V, so the patch
  sees about 3.0 V. Actual contact/FFC resistances and the connector current rating must come from the Hirose
  drawing and the cable spec.
- **Gas modules (indicative class values, verify per MPN):**
  - Metal-oxide (e.g. MQ-class methane/CO): heater at about 5 V, ~130–180 mA (≈0.7–0.9 W) continuous. Long preheat
    (minutes) and burn-in (hours to days). A 3.3 → 5 V boost at ~85 % efficiency would draw roughly 250–350 mA from
    +3V3_CHAIN, likely beyond the margin of a single 0.5 mm-pitch contact and of the main regulator plan.
  - Catalytic pellistor: similar or higher heater power.
  - NDIR (CH₄ / CO₂): often 5 V with lamp pulses of ~100–150 mA peak, and warm-up of tens of seconds to minutes.
  - Electrochemical CO: the cell itself needs very little power, but needs an AFE/potentiostat. Packaged modules
    (UART/analog) are usually ≤ tens of mA at 3.3–5 V. This is likely the best fit for a 3.3 V chain.
- **Warm-up vs one-at-a-time duty cycling:** the source wants a module powered "only when needed". Heated MOS and
  pellistor sensors need minutes of preheat to stabilise. Duty-cycling them per scan gives unstable readings, while
  continuous heating conflicts with battery life and the one-path-at-a-time model. Each approved module needs its own
  power/acquisition profile.
- **Heat in the hive:** a continuously heated sensor near the colony also affects local T/RH readings, and possibly
  the bees. The source requires keeping T/RH away from heat sources (p.12). Placement rules per module are needed.
- **Options that keep the 12-pin ICD:** low-power module classes only, local boost per patch limited to one active
  port, or a separate external V_AUX supply. **Options that change the ICD (not allowed at this stage):** repurpose a
  GND pin as VBAT/V_AUX, or add contacts.
- 1S LiPo (≈3.0–4.2 V) feeding a buck to 3.3 V drops out near ~3.4–3.5 V. The source already requires a buck-boost
  or a validated cutoff.

## 4. Identification for AUX (feeds OQ-009)
- Existing windows: empty < 0.25 V, mic 1.70–2.20 V, T/RH 2.50–2.95 V. The T/RH window top (2.95 V) is near the
  upper end of the ESP32-S3 ADC's usable range at maximum attenuation. Check the effective range in ESP-IDF [R6].
- Candidate AUX ID methods: (1) ID resistor on the module adapter, read through the AUX mux onto SENSOR_DATA_ID with a
  new code table; (2) a 1-Wire / I²C ID EEPROM on the adapter; (3) a manual firmware port profile (user declares the
  module per port); (4) presence detect only. Option (1) adds more codes to a ~3 V span, so tolerance analysis
  over 1 % resistors, rail drop at patch 8, and ADC error is mandatory.
- Separating "patch type" from "port presence" may need an extra bit (AUX_ID_EN), which is included in option B in §1.

## 5. Shift-register chain integrity
- PATCH_SHIFT_DATA is re-timed at every patch (DS → QH'), but PATCH_SHIFT_CLK, LATCH and OE_N are long, multi-drop
  passthrough nets with 8 loads. Slow or ringing clock edges on non-Schmitt HC inputs can double-clock. Consider
  termination, a series R, or Schmitt buffering, and verify in V-02/V-11.
- Clock and data travel in the same direction, so the hold margin rests on the 595's QH' propagation delay. This is
  expected to be fine at short FFC lengths, but verify at maximum chain length.

## 6. Physical/BOM growth per patch
Four headers plus per-port power switches, a mux, a level translator, ESD on ~8–12 exposed lines, and possibly a
second 595 and a boost converter. All of this greatly enlarges a "patch" compared with the Rev 0.1 concept. The
patch outline (OQ-018) and AUX connector choice should be decided together. 8 patches × 4 = **32 user-accessible
ports** in total.

## 7. Status discrepancy
The source [p.13, p.19] records the API Gateway as "Bridge not found, retrying". On 01 Nov 2026 the bridge was
restarted and the handshake plus a read-only query succeeded (see `15_status_log.md`). Phase P3's entry
criterion is partly met; a formal "safe API test report" is still a deliverable.
