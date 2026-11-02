# 06 — Identification Model and Firmware Operating Concept

## 6.1 Identification principle [p.11]
- **Position:** serial shift-register order (64-bit image, 8 bits per patch).
- **Type:** a switched ID resistor from +3V3_CHAIN onto SENSOR_DATA_ID, against a 100 k pull-down on the main board,
  read by the ESP32-S3 ADC1. AUX: a future module-profile mechanism (TBD).

## 6.2 Primary ID resistor proposal — PROVISIONAL [p.11]
V_ID = V_CHAIN × R_PD / (R_ID + R_PD), with R_PD = 100 k 1 % on the main board.

| Population | R_ID to +3V3_CHAIN | Nominal V_ID | Screening window |
|---|---|---|---|
| Empty / open | open | ~0 V | < 0.25 V |
| I²S microphone | 68 k 1 % | ~1.964 V | 1.70 – 2.20 V |
| T/RH (SHT45) | 22 k 1 % | ~2.705 V | 2.50 – 2.95 V |
| anything else | — | — | invalid |

KB check: 3.3 × 100/122 = 2.705 V ✔; 3.3 × 100/168 = 1.964 V ✔.
Source caveat: these are screening values. They need calibration, tolerance analysis and ADC tests, and they are
**not** a codebook for the four AUX ports.

## 6.3 Firmware state machine [p.11]
| State | Action |
|---|---|
| SAFE_BOOT | Hold PATCH_OE_N high. All outputs disabled. Verify chain-control GPIO levels |
| CHAIN_INIT | Shift a known all-zero image through all 8 registers, latch. Verify bit order with walking-one / walking-zero |
| PATCH_DISCOVERY | One patch position at a time: enable its ID path and classify the primary type |
| AUX_DISCOVERY | Select J_AUX1..4 individually. Apply the approved ID/config method. Log unknown/absent modules safely |
| CONFIGURE | Choose protocol profile (I²C, I²S, UART, analog, supported digital). Set pull-ups/levels and sensor power |
| ACQUIRE | Wait for module-specific warm-up/startup. Collect data + timestamps. Plausibility and fault checks |
| ISOLATE | **Disconnect signal first**, then disable power, discharge if needed, then move to the next port/patch |
| FAULT | Leave the failing path off. Record patch position / port / reason. Continue only if safe |

## 6.4 Critical pin-reuse check [p.11]
SENSOR_DATA_ID must work as an ADC input (identification) **and** as I²S DIN (audio) on the exact ESP32-S3 module.
Prove this on the bench. If it fails, revise the GPIO or add an analog mux / dedicated input.

## 6.5 Firmware topics not defined in the source (see OQ list)
Sample scheduling, audio sample rate / bit depth / duration, data storage (flash/SD), timestamps/RTC source,
radio (Wi-Fi/BLE) usage, upload/cloud, display content, power-saving/sleep strategy, OTA, logging format.
