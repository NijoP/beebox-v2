# 01 — Project Overview

## Identity
- Product: **BeeBox V2**, a modular honeybee-colony sensor platform [p.1]
- Controlling document: BBX-ENG-PR-003, Rev 0.2, 01 Nov 2026 [p.1]
- Lineage: Rev 0.1 = "Sensor Chain V2 feasibility baseline" (one controller, up to eight MCU-free patches, 12-contact
  chain, T/RH or I²S microphone). Rev 0.2 adds **four auxiliary module headers on every patch** [p.2]
- Prior design repository: https://github.com/NijoP/BeeBox (V1/V2 source documents) [R1, p.20]

## Product intent [p.1, p.3]
A modular sensor system that collects hive-related environmental and acoustic measurements through:
- **one main controller** (ESP32-S3), and
- a **daisy chain of up to 8 batteryless sensor patches**, with no MCU on the patches.

Each patch carries one **primary sensor**: temperature/humidity (SHT45-class) **or** a digital I²S microphone.
Each patch also has **four auxiliary headers J_AUX1..J_AUX4** for approved external modules (methane, CO, other).

## What the system does [p.3]
1. Collects environmental measurements from a selected T/RH patch.
2. Captures audio from a selected I²S microphone patch.
3. Lets optional external modules connect through four auxiliary headers per patch, **subject to an approved
   module compatibility list**.
4. Uses one central controller for patch selection, module power, signal routing, identification and display/status.
5. Scales to at most 8 patches on one chain, with the same logical 12-contact interface for FPC and pogo builds.

## Design principles [p.3]
| Principle | Meaning |
|---|---|
| Centralized control | Patches have no MCU and no battery. Small logic, switching and power devices are allowed |
| One active path | Only one patch / one external-module path is active at a time, unless concurrency is proven |
| Modularity | Repeatable patch PCB, clearly labelled ports, replaceable sensor modules |
| Fail-safe | All sensor power and signal paths default **OFF** during reset, shift-register updates and faults |
| Evidence-led release | Calculations, ERC/DRC, netlist checks and measurements are required before fabrication |

## Product boundary (explicitly out of scope) [p.3]
The platform does **not** claim: validated bee-health diagnosis, colony-health scoring, methane interpretation,
CO interpretation, cloud analytics or field performance. These are separate product and algorithm requirements.

## Engineering status [p.1, p.20]
- Conditional GO for schematic and two-patch proof work **after the auxiliary-module interface is closed**.
- **Not released for manufacture.**
- None of the following is claimed yet: hardware test, full-chain signal-integrity proof, AUX power qualification,
  API bridge handshake, environmental field trial.
  - Update 01 Nov 2026 (KB): the EasyEDA API bridge handshake has since been verified. See `15_status_log.md`.

## Stakeholders / client
- The source does **not** name the client, end users, deployment owner or acceptance authority. See OQ-001.
- EasyEDA account observed: `nijoponnachan` (Personal workspace).
