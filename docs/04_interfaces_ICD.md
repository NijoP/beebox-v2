# 04 — Interface Control (ICD draft)

## 4.1 Twelve-contact sensor-chain interface — CONFIRMED logical baseline [p.7]
Applies to FPC IN, FPC OUT, the main-board chain connector and the pogo variant (same logical pinout).
The previous repository pin assignment must be replaced consistently in schematics, netlists, PCB, firmware and docs.

| Pin | Net | Function | Routing |
|---|---|---|---|
| 1 | GND | Ground / return | IN→OUT pass-through |
| 2 | +3V3_CHAIN | Always-on 3.3 V for patch logic and switched sensor input | pass-through |
| 3 | GND | Ground / return | pass-through |
| 4 | SENSOR_CLK | I²C SCL (T/RH mode) / I²S BCLK (mic mode) | pass-through |
| 5 | SENSOR_SYNC | I²C SDA (T/RH mode) / I²S WS (mic mode) | pass-through |
| 6 | GND | Ground / signal return | pass-through |
| 7 | SENSOR_DATA_ID | I²S SD during mic capture / ADC ID voltage during discovery | pass-through |
| 8 | PATCH_SHIFT_DATA | IN pin 8 → local 74HC595 **DS**; local **QH'/Q7S** → OUT pin 8 | **Directional through 74HC595** |
| 9 | PATCH_SHIFT_CLK | Shared shift-register clock (SRCLK) | pass-through |
| 10 | PATCH_LATCH | Shared storage latch (RCLK) | pass-through |
| 11 | PATCH_OE_N | Global active-low output enable | pass-through |
| 12 | GND | Ground / return | pass-through |

Summary: 4 × GND, 1 × power, 3 × shared sensor/data, 4 × shift-register control/cascade.

**Pin-numbering caution [p.7]:** verify connector pin numbering and mirror orientation against the exact
manufacturer drawings before layout. An FPC cable flip (same-side vs opposite-side contacts) can mirror the pinout.

Physical candidate (**PROVISIONAL**): Hirose **FH34SRJ-12S-0.5SH(50)**, 12-pos, 0.5 mm pitch ZIF, for 0.3 mm FPC/FFC
[R2]. Not a released footprint. Still to verify: drawing, land pattern, contact side, insertion direction, stiffener,
bend, current margin. Pogo contact candidates: Mill-Max 0906 series catalog [R12] (exact MPN TBD).

## 4.2 Main-board ESP32-S3 GPIO function groups [p.6]
| Group | Signals | Notes |
|---|---|---|
| Patch control (4) | PATCH_SHIFT_DATA, PATCH_SHIFT_CLK, PATCH_LATCH, PATCH_OE_N | OE_N held high at boot |
| Sensor clock/sync (2) | SENSOR_CLK, SENSOR_SYNC | I²C SCL/SDA or I²S BCLK/WS |
| Sensor data / ID (1) | SENSOR_DATA_ID | ADC1 input in discovery / I²S DIN in capture. **Must be ADC1-capable. GPIO38 rejected** |
| Display (4-wire) | GND, +3V3_DISPLAY, DISPLAY_SCL, DISPLAY_SDA | Separate I²C. Pull-ups not shared with the sensor bus |
| Debug | USB or UART/JTAG + test pads | Independent of the chain |
| Power supervision | Main rail / battery monitor | TBD |

Actual GPIO numbers: **TBD** (OQ-010).

## 4.3 Display connector [p.4]
4-wire I²C: GND, +3V3_DISPLAY, DISPLAY_SCL, DISPLAY_SDA. Display module MPN, address, voltage and onboard
pull-ups are **TBD** (OQ-017). Connector pin order is not specified.

## 4.4 Auxiliary header J_AUX1..J_AUX4 — PROVISIONAL concept only [p.9, p.10]
Every patch has four headers. Electrical status per port: "Header required; exact connector/pinout TBD".

Starting concept (proposal, **not frozen**): a 4-position header

| Pos | Signal (concept) |
|---|---|
| ? | V_AUX |
| ? | GND |
| ? | AUX_IO_A |
| ? | AUX_IO_B |

(Pin order, pitch, keying, THT/SMT and locking are all TBD.)

Possible use of the two signal pins [p.10]:
| Interface | Use | Required provisions |
|---|---|---|
| Analog | IO_A = analog out; IO_B unused/mode | Divider/buffer/filter, clamp, ADC range protection. **Never connect 5 V analog directly to the ESP32-S3** |
| I²C | IO_A = SCL, IO_B = SDA | Level compatibility, pull-up placement, address conflicts, bus capacitance, selected-port isolation |
| UART | IO_A/IO_B = RX/TX (direction to be defined) | TX/RX crossing convention, logic level, baud, optional level translation |
| PWM / digital | One signal = PWM or logic | Conditioning, level compatibility, firmware capture method |
| SPI / multi-wire | Not possible on 2 signals | Larger connector or adapter. **Do not claim SPI support** |

Modules that need enable, calibration, heater or a separate analog output may need a larger connector or an adapter board.

**Unspecified:** how AUX_IO_A/B reach the main controller through the 12-contact chain (which chain line carries
each protocol). See `14_engineering_analysis_notes.md` §2 and OQ-007.
