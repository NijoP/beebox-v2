# 09 — Assumptions

Each assumption must either be confirmed (→ requirement/decision) or replaced. **S** = stated in source, **KB** = made
by the KB author during the study.

| ID | Assumption | Origin | Validate by |
|---|---|---|---|
| AS-01 | The four AUX headers are per patch (J_AUX1..4 on every patch), not per system | S (p.20) | Client confirmation (OQ-002) |
| AS-02 | One AUX port / one patch path is active at a time | S (p.9) | Mux + power validation; client concurrency need (OQ-014) |
| AS-03 | The 12-contact chain pinout stays unchanged while adding AUX | S (p.9) | Holds unless the power budget forces a change (14 §3) |
| AS-04 | A 4-pos AUX header (V_AUX, GND, IO_A, IO_B) covers most target modules | S (proposal, p.10) | P1 module survey |
| AS-05 | Primary sensors run from +3V3_CHAIN (3.3 V) | S (p.8) | Datasheets (SHT45 / mic) |
| AS-06 | Power source is a 1S LiPo | S (p.12) | OQ-020 |
| AS-07 | ID resistor screening windows are adequate for the primary types | S (provisional, p.11) | V-03 tolerance & ADC test |
| AS-08 | The ESP32-S3 can switch one pad between ADC1 and I²S DIN at runtime | S (to be proven, p.11) | Bench test |
| AS-09 | The target EasyEDA project is "BEE BOX V2" (Board1, Schematic1/P1, PCB1, Panel1) currently open in EasyEDA Pro V3.2.149 | KB (observed) | User confirmation before any write |
| AS-10 | The KB folder `C:\Users\HP\OneDrive\Documents\BEEBOX V2` is the BeeBox V2 project folder from p.19 | KB | User |
| AS-11 | Patch control signals use 3.3 V logic throughout (74HC family at 3.3 V) | KB | Datasheet threshold check |
| AS-12 | Patch position 1 = patch nearest the main board; its register's bits are the *last* 8 bits shifted out | KB | V-02 walking-one test |
| AS-13 | Gas readings are raw signals only; no interpretation or safety alarm function is provided | S (p.3 boundary) | Client |
| AS-14 | Product is not intended as a certified gas-safety / hazardous-area device | KB | Client (OQ-006) |
