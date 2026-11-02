# 05 — Universal Sensor Patch Design

Figure: `00_source/fig2_sensor_patch_p8.png`. The source notes: "Diagram is an architecture concept; exact mux, switch,
supply and header pinout are not yet frozen."

The patch is a **passive controlled node**. The main board addresses it through the cascade and selects its local sensor path [p.8].

## 5.1 Functional blocks [p.8]
```
12-pin FPC IN ──▶ SN74HC595 cascade + control ──▶ 12-pin FPC OUT
                  Q0 sensor power | Q1 signal enable | Q2 ID enable | Q3–Q7 provisional AUX / reserve

Primary sensor (SHT45 OR I²S mic) ─▶ Signal isolation + ID (3 lines + ID branch) ─▶ AUX selection matrix ─▶ J_AUX1..4
        ▲                                                                              ▲
+3V3_CHAIN always-on logic / local switched primary rail                  AUX supply decision: 3.3 V / 5 V / boost / limit TBD
```

| Block | Baseline function | Status |
|---|---|---|
| FPC IN / OUT | Two 12-pos connectors. Everything passes through except pin 8 | CONFIRMED |
| Shift register | One SN74HC595-class 8-bit register | CONFIRMED (class) |
| Q0 | Primary sensor power (load-switch enable) | CONFIRMED |
| Q1 | Primary sensor signal enable (isolation switch) | CONFIRMED |
| Q2 | Patch ID enable (switches ID resistor onto SENSOR_DATA_ID) | CONFIRMED |
| Q3–Q7 | Under review for AUX selection/control, or reserve | PROVISIONAL |
| Primary isolation | 3 signal paths (CLK, SYNC, DATA) + 1 switched ID-resistor branch. CD74HC4066-class candidate | PROVISIONAL part |
| Primary sensor power | Local load switch from +3V3_CHAIN, default OFF, defined discharge. TPS22918-class candidate | PROVISIONAL part |
| Primary population | Exactly one: SHT45-class T/RH **or** qualified I²S mic. Other branch DNI/isolated | CONFIRMED rule |
| ID resistor | 22 kΩ (T/RH) / 68 kΩ (mic) to +3V3_CHAIN, switched only while identifying | PROVISIONAL, to be extended |
| AUX headers | J_AUX1..J_AUX4 on every patch, each identified in schematic + silkscreen | CONFIRMED (pinout TBD) |

Note: a CD74HC4066 has 4 switches, which exactly covers 3 sensor lines + 1 ID branch, so one quad switch suffices for the
primary path (KB-ANALYSIS; consistent with source "quad switch" [p.15]).

## 5.2 Auxiliary expansion subsystem [p.9–p.10]
**Captured requirement:** every patch shall include J_AUX1..J_AUX4 for user-selected modules (methane, CO, other).

Characteristics:
- They are expansion **ports**, not new chain contacts. The patch still connects through the 12-contact chain.
- First implementation: **select one port → power it only when needed → route its signals to the main controller
  → read → return to a safe inactive state**.
- Concurrent multi-port reading is a **future** capability, after mux, power budget and protocol are validated.

Required provisions:
1. **Port-selection network.** Inactive headers must not load the shared bus, and two outputs must never drive one line.
2. **Per-port controlled power.** Load switch, default OFF (preferred: selected-port power path).
3. **Control budget.** Decide whether Q3–Q7 suffice or a second register / decoder / mux controller is needed.
4. **Identification.** 22 k / 68 k only identify primary types. A new scheme is needed for module type × 4 ports
   (resistor code, EEPROM, manual firmware profile, …).
5. **Protection.** ESD/transient/overvoltage for an exposed user connector and the module output range.
6. **Analog conditioning.** Divider/buffer/clamp. Never connect 5 V directly to the ESP32-S3.
7. **V_AUX strategy.** 3.3 V / 5 V / boost / external supply, current limit, thermal limit.

Intended module classes [p.9]:
- Methane / combustible gas (technology- and calibration-dependent)
- CO (NDIR or other; digital or analog)
- Other environmental modules, only after power, protocol, voltage range, warm-up time and environmental suitability are assessed

Possible interfaces of such modules: analog, I²C, UART, PWM, SPI, other. Possible supplies: 3.3 V, 5 V or higher. Heater
or optical-source current may far exceed the T/RH/mic load [p.9].

## 5.3 Assembly / population rules [p.8]
- Exactly one primary sensor populated per patch in the initial architecture.
- The alternate branch is DNI or isolated so it cannot load or drive the shared bus.
- Patch variant notes, DNI options, test points and configuration labels go on sheet P05.

## 5.4 Planned schematic sheets for the patch [p.14]
P01 FPC IN/OUT · P02 Register + power · P03 Primary sensor / ID · P04 AUX1–AUX4 · P05 Test / assembly
