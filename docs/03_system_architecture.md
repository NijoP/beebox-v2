# 03 — System Architecture

Figure: `00_source/fig1_system_architecture_p5.png`

```
                       12-contact FPC / pogo chain (same logical pinout)
┌───────────────────┐   ┌─────────┐   ┌─────────┐         ┌─────────┐
│ MAIN CONTROL PCB  │──▶│ PATCH 1 │──▶│ PATCH 2 │── ... ─▶│ PATCH 8 │
│ ESP32-S3          │   │ 595+mux │   │ 595+mux │         │ 595+mux │
│ ADC / I²C / I²S   │   │ ID+sens │   │ ID+sens │         │ ID+sens │
│ Display + Debug   │   │ J_AUX1-4│   │ J_AUX1-4│         │ J_AUX1-4│
└───────────────────┘   └─────────┘   └─────────┘         └─────────┘
Only one patch / one auxiliary module path is active at a time until concurrency is proven.
```

## Functional partition [p.5]
| Block | Responsibilities |
|---|---|
| Main controller PCB | ESP32-S3 runtime control. **64-bit shift-register control image** (8 patches × 8 bits). Patch scanning, ADC identification, sensor-bus mode selection, display/status, power supervision, debug |
| 12-contact chain | Always-on patch logic supply, common grounds, **3 time-multiplexed sensor/data lines**, **4 shift-register control/cascade signals** |
| Sensor patch base | FPC IN/OUT, one SN74HC595-class register, local sensor power switch, signal isolation, patch-type ID, primary T/RH or I²S footprints |
| Auxiliary expansion | 4 module headers, module-power control, input protection/conditioning, port selection/multiplexing, future module-ID strategy |
| Firmware and tools | Safe state machine, sensor drivers, module profiles, project-controlled EasyEDA scripts, independent validation records |

**Architectural rule [p.5]:** never wire the 4 AUX headers directly in parallel to one signal bus. A selected-port
switch/mux prevents bus contention, driver collisions and accidental powering through signal pins.

## Addressing model [p.11]
- **Position** comes from the serial order of the shift-register cascade (patch 1 is nearest the main board).
- **Type** comes from a switched ID path (resistor → ADC) or a future module-profile mechanism.

## Main controller [p.6]
- ESP32-S3 module. Exact MPN, flash/PSRAM and GPIO map are **TBD**. They are frozen from the module datasheet and the final PCB allocation.
- GPIO function groups: see `04_interfaces_ICD.md`.
- Design gates:
  1. Reserve an **ADC1-capable** GPIO for SENSOR_DATA_ID. GPIO38 is rejected.
  2. Confirm the same pad supports I²S DIN on the chosen module and firmware.
  3. Confirm output levels, strapping (bootstrapping) pins, USB/debug needs and antenna keep-out.
  4. Keep display pull-ups and power independent of the sensor bus.
  5. Do not freeze the power stage before the battery, patch-load and AUX power plans exist.

## Power architecture [p.12]
| Rail / block | Purpose | Decision status |
|---|---|---|
| Battery input | Baseline 1-cell LiPo | Cell, protection, charger, connector, safe cutoff **TBD** |
| Main 3.3 V | ESP32-S3 + main logic | Regulator must cover controller peaks + other loads |
| +3V3_CHAIN | Always-on patch logic + input to local sensor switches | Budget 8 registers, switches, pull-ups, leakage, **cable drop** |
| Primary sensor VDD | One selected T/RH or mic at a time | Local patch load switch, default OFF; confirm startup/discharge |
| V_AUX | User modules on J_AUX1..4 | Voltage options, boost/regulation, current limit, per-port switching **TBD** from module list |
| Display rail | Separate display supply / I²C | Verify module voltage, address, onboard pull-ups |

**Regulator warning [p.12]:** the earlier baseline used a 1S LiPo plus a TPS62162-class **buck**. A buck alone cannot
hold 3.3 V across the full cell discharge range. Choose a **buck-boost**, or define and validate a battery cutoff
that keeps the rail in regulation.

**Gas-module power warning [p.10]:** do not assume +3V3_CHAIN can power methane/CO modules. 5 V or heater/optical
sources may need a dedicated boost, extra regulation, current limiting or an external supply. Calculate worst-case
chain current before approving a module.

## Mechanical / PCB rules [p.12]
- FPC orientation, pin-1 and IN/OUT labels must be unmistakable. Validate with a 1:1 print and a continuity fixture.
- Keep gas apertures, acoustic ports and the T/RH area clear of enclosure blockage and heat sources.
- Hive environment: condensation, dust, **propolis**, vibration, flex bend radius, strain relief, pogo compression/cycle life.
- Place decoupling at IC supply pins. Size sensor bypass from datasheets and inrush analysis.
- Route high-impedance analog away from switching power and fast digital. Keep ground return continuous.
- Use small passives only where suitable. Capacitance, voltage rating, tolerance, derating and yield take priority.
- Metric vs imperial package codes: 0404 metric ≠ 0402 imperial.
