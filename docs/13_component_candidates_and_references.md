# 13 — Component Candidates and References

**All parts below are candidates (PROVISIONAL).** None is released. Qualify each against the listed criteria [p.15].

| Subsystem | Candidate / concept | Qualification before selection | Ref |
|---|---|---|---|
| Main MCU | ESP32-S3 module (MPN TBD) | Exact MPN, memory, antenna/keep-out, GPIO capability, ADC/I²S alternate use, supply peaks, lifecycle | R6, R7 |
| T/RH | Sensirion SHT45 | Supply, accuracy, port placement, decoupling, humidity exposure, package/footprint | R8 |
| I²S mic | Current-production digital I²S mic. PUI DMM-4026-B-I2S family is one candidate | Lifecycle, BCLK limits, output format, acoustic port, sample rate, full-chain timing. **ICS-43434 is EOL** | R9–R11 |
| Shift register | TI SN74HC595 | Supply range, output current, clock, serial-out pin, SRCLR/OE behaviour, package | R3 |
| Signal switch | CD74HC4066-class quad switch | R_ON, bandwidth, signal range, logic threshold, off leakage, I²S waveform, power-off isolation | R4 |
| Load switch | TPS22918-class or qualified alternate | V_IN, current, turn-on, reverse blocking/backfeed, discharge, default-off control | R5 |
| AUX signal mux | TBD after module shortlist | Channels, analog range, R_ON, bandwidth, leakage, direction, voltage domain, control logic | — |
| AUX power | Possible dedicated boost / regulated rail | Per-module V/I, heater/inrush, thermal, chain current, protection, runtime impact | — |
| Chain connector | Hirose FH34SRJ-12S-0.5SH(50), 12-pos 0.5 mm ZIF, 0.3 mm FPC | Drawing, land pattern, contact side, insertion direction, stiffener, bend, current margin | R2 |
| Pogo contacts | Mill-Max 0906 series (catalog) | Exact MPN, geometry, compression, retention, cycle life | R12 |
| AUX headers | 4 per patch, type TBD | Pitch, retention, keying, current, orientation, user access | — |
| Main regulator | Earlier TPS62162-class buck. Buck-boost now preferred | Headroom over full 1S range, peak current | p.12 |
| Battery | 1S LiPo (cell/protection/charger TBD) | Capacity, protection, charge path, cutoff | p.12 |

## References carried forward [p.20]
- [R1] BeeBox repo: https://github.com/NijoP/BeeBox
- [R2] Hirose FH34: https://www.hirose.com/en/product/p/CL0580-1253-0-50
- [R3] TI SN74HC595: https://www.ti.com/product/SN74HC595
- [R4] TI CD74HC4066: https://www.ti.com/product/CD74HC4066
- [R5] TI TPS22918: https://www.ti.com/product/TPS22918
- [R6] ESP32-S3 ADC guide: https://docs.espressif.com/projects/esp-idf/en/latest/esp32s3/api-reference/peripherals/adc/index.html
- [R7] ESP32-S3 I²S guide: https://docs.espressif.com/projects/esp-idf/en/latest/esp32s3/api-reference/peripherals/i2s.html
- [R8] Sensirion SHT45: https://sensirion.com/products/catalog/SHT45
- [R9] TDK ICS-43434: https://www.invensense.tdk.com/en-us/products/microphone/ics-43434
- [R10] PUI DMM-4026-B-I2S-R: https://puiaudio.com/product/microphones/dmm-4026-b-i2s-r
- [R11] PUI DMM-4026-B-I2S eval doc: https://api.puiaudio.com/filename/DMM-4026-B-I2S-EB-R.pdf
- [R12] Mill-Max 0906 catalog: https://www.mill-max.com/sites/default/files/external/catalog/2021-02/023-028.pdf

Datasheets have not yet been downloaded or reviewed in this KB. When they are, store them under `bom/datasheets/` (P1).
