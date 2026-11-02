# 11 — Verification and Acceptance Plan [p.16]

"A clean-looking schematic or successful script execution is not evidence of electrical correctness. Each gate needs a
reproducible artifact."

| ID | Test | Acceptance evidence |
|---|---|---|
| V-01 | Safe power-up | OE_N disables outputs; sensor and AUX supplies OFF until selected. Scope + current log |
| V-02 | 64-bit chain order | Walking-one/zero maps all 8 patches, verifies pin-8 direction. Logic-analyser log |
| V-03 | Primary patch ID | T/RH / mic / empty classify correctly over tolerance and rail variation. ADC CSV / calibration table |
| V-04 | AUX port select | J_AUX1..4 individually selected; inactive ports cannot drive/load shared lines. Test report |
| V-05 | AUX module power | Correct rail and current limit at each header; inactive port off; no back-power. DMM / current trace |
| V-06 | AUX input range | Analog/digital levels within GPIO/ADC limits; protection response recorded. Scope + calculations |
| V-07 | I²C T/RH | Only selected SHT45 responds; bus rise time and sink current pass. I²C capture |
| V-08 | I²S microphone | Only selected mic drives SD; capture works at patch 1 and patch 8. Audio file + scope |
| V-09 | AUX protocol profiles | Each approved module MPN communicates per defined protocol. Raw logs |
| V-10 | Power-off backfeed | Inactive module rail and I/O protection paths don't energize sensors. Current/voltage log |
| V-11 | Full-chain power / SI | Patch-8 rail, I²C timing, I²S edges, module startup pass at max chain length. Scope |
| V-12 | FPC / pogo map | All 12 contacts, orientation and pin-8 cascade pass automated continuity fixture |
| V-13 | Fault recovery | Missing module, invalid ID, timeout, reset, open chain → all outputs safe. Firmware log |
| V-14 | Mechanical / thermal | No connector lift, flex damage, blocked port, heating, enclosure interference. Inspection record |

## First prototype stop-gate [p.16]
Build **1 main controller + 2 patches** (one T/RH, one I²S mic). Include an AUX proof on ≥1 patch with
representative external module(s). Do **not** scale to 8 patches until chain order, ID, primary isolation, AUX port
switching, module power and back-power tests pass.

## Design-stage checks (before fabrication) [p.3, p.14]
Design calculations, ERC, DRC, netlist export/compare (pin-to-net CSV vs ICD), visual sheet inspection, symbol↔footprint
pad mapping vs manufacturer drawings, independent audit (P5) with no unresolved critical pin/net mismatch.
