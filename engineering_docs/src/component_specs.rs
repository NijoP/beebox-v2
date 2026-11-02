//! # Component specifications (datasheet-verified records)
//!
//! One record per selected active component / connector. Values are copied from the archived
//! manufacturer document named in `datasheet`; anything not read from that document is placed in
//! `assumptions` or marked with [`EngStatus::Unverified`].
//!
//! Lifecycle: TI parts were checked in the orderable addendum of the archived PDF
//! (status "Active"). Other manufacturers: see each record.

use crate::component_model::{DatasheetId, EngStatus, Lifecycle, Rating};

/// A component specification record.
#[derive(Debug, Clone, Copy)]
pub struct ComponentSpec {
    /// Key used by [`crate::bom`] and [`crate::pinmap`].
    pub key: &'static str,
    /// Function.
    pub function: &'static str,
    /// Manufacturer.
    pub manufacturer: &'static str,
    /// Exact orderable MPN.
    pub mpn: &'static str,
    /// Package and body size.
    pub package: &'static str,
    /// Footprint source.
    pub footprint: &'static str,
    /// Supply range.
    pub supply: &'static str,
    /// Supply / operating current.
    pub current: &'static str,
    /// Further ratings and operating conditions.
    pub ratings: &'static [Rating],
    /// Lifecycle.
    pub lifecycle: Lifecycle,
    /// Required external passives.
    pub passives: &'static [&'static str],
    /// Alternatives (with reason).
    pub alternatives: &'static [&'static str],
    /// Evidence.
    pub datasheet: DatasheetId,
    /// Assumptions still open on this record.
    pub assumptions: &'static [&'static str],
    /// Engineering status.
    pub status: EngStatus,
}

/// All selected component specifications.
pub const SPECS: &[ComponentSpec] = &[
    // ------------------------------------------------------------- main board
    ComponentSpec { key: "MCU", function: "Main controller (Wi-Fi/BLE, ADC1, I²S, USB)", manufacturer: "Espressif", mpn: "ESP32-S3-WROOM-1-N16R2",
        package: "SMD module 18.0 × 25.5 × 3.1 mm, PCB antenna", footprint: "DS-ESP-MOD recommended land pattern + antenna keep-out",
        supply: "3.0–3.6 V (3.3 V typ)", current: ">= 0.5 A supply capability; TX 802.11b 1 Mbps 355 mA; deep sleep 7–8 µA (SoC)",
        ratings: &[("Ambient", "-40..85 °C for N16R2 (R8 variants only -40..65 °C)"), ("Flash/PSRAM", "16 MB quad / 2 MB quad (GPIO35-37 free)"), ("VIH / VIL", "0.75 VDD min / 0.25 VDD max; abs max VDD + 0.3 V"), ("VOH / VOL", "0.8 VDD / 0.1 VDD; IOH 40 mA, IOL 28 mA (PAD_DRIVER 3)"), ("Internal pulls", "45 kΩ"), ("ADC1", "GPIO1-10; ATTEN3 0-2.9 V ±50 mV, ATTEN2 0-1.6 V ±10 mV after calibration")],
        lifecycle: Lifecycle::NotConfirmed, passives: &["22 µF + 100 nF on 3V3", "CHIP_PU 10 kΩ / 1 µF RC", "BOOT (GPIO0) 10 kΩ pull-up + button", "USB D+/D- direct (GPIO19/20) via USBLC6"],
        alternatives: &["ESP32-S3-WROOM-1-N8R2 (same pinout, 8 MB flash)", "ESP32-S3-WROOM-1-N16R8 only if ≤ 65 °C ambient proven or PSRAM ECC enabled (85 °C, 1/16 PSRAM lost)"],
        datasheet: "DS-ESP-MOD", assumptions: &["Orderable lifecycle not stated in datasheet - confirm on Espressif product selector before order"], status: EngStatus::Recommended },
    ComponentSpec { key: "CHARGER", function: "1S Li-ion charger + power path", manufacturer: "Texas Instruments", mpn: "BQ24074RGTR",
        package: "VQFN-16 (RGT) 3 × 3 mm, thermal pad", footprint: "TI RGT0016 land pattern (DS-BQ24074 mechanical)",
        supply: "IN 4.35–10.5 V operating, 26 V no damage, OVP 10.5 V", current: "BAT sleep 4.3/6.5 µA; ICHG = 890 AΩ / RISET",
        ratings: &[("VBAT(REG)", "4.16/4.20/4.23 V"), ("VO(REG)", "4.4 V (BQ24074)"), ("KISET", "797/890/975 AΩ, RISET 590 Ω-8.9 kΩ"), ("KILIM", "1500/1610/1720 AΩ, RILIM 1.1-8 kΩ"), ("KTMR", "36/48/60 s/kΩ, RTMR 18-72 kΩ"), ("VDO(BAT-OUT)", "50/100 mV at 1 A"), ("RθJA", "44.5 °C/W"), ("Thermal regulation", "125 °C")],
        lifecycle: Lifecycle::Active, passives: &["RISET 1.78 k", "RILIM 1.62 k", "RTMR 68.1 k", "TS: 10 k NTC in pack (or 10 k fixed)", "ITERM open (10 %)", "IN 1 µF/25 V", "OUT 10 µF", "BAT 10 µF", "CHG/PGOOD 100 k pull-ups to +3V3", "EN1 100 k pull-up to +3V3 + GPIO, EN2 100 k pull-down + GPIO (USB100 until +3V3 is up, then USB500; ILIM mode by firmware)", "CE to GND"],
        alternatives: &["BQ24072/73 (6.6 V OVP, lower input range)", "BQ25180 (I²C, not reviewed)"], datasheet: "DS-BQ24074", assumptions: &[], status: EngStatus::Recommended },
    ComponentSpec { key: "BUCKBOOST", function: "+3V3 buck-boost regulator", manufacturer: "Texas Instruments", mpn: "TPS63021DSJR",
        package: "VSON-14 (DSJ) 3 × 4 mm, thermal pad", footprint: "TI DSJ0014 land pattern", supply: "VIN 1.8–5.5 V",
        current: "IQ 25/50 µA; 2 A out at VIN > 2.5 V; switch limit 3.5/4.0/4.5 A",
        ratings: &[("VOUT PWM", "3.267/3.3/3.333 V"), ("VOUT PS mode", "+0.6 % typ / +5 % max vs PWM (PS/SYNC = GND)"), ("fSW", "2.2/2.4/2.6 MHz"), ("L", ">= 1 µH; 1.5 µH typical application"), ("COUT", "3 × 22 µF typical"), ("RθJA", "41.8 °C/W")],
        lifecycle: Lifecycle::Active, passives: &["L 1.5 µH Isat >= 2.5 A", "CIN 2 × 10 µF", "COUT 3 × 22 µF", "FB -> VOUT", "EN -> VIN", "PS/SYNC 100 k pull-down + GPIO (force PWM during ID/audio)", "PG 100 k pull-up -> GPIO"],
        alternatives: &["TPS63802 (2 A, adjustable, smaller DLA package)", "TPS63020 + FB divider"], datasheet: "DS-TPS6302X", assumptions: &["Efficiency 0.85-0.9 used in budgets; read curve at final load"], status: EngStatus::Recommended },
    ComponentSpec { key: "CHAINSW", function: "+5V_CHAIN current-limited switch (latch-off), IN = +5V0 (rev D)", manufacturer: "Texas Instruments", mpn: "TPS2553DBVR-1",
        package: "SOT-23-6 (DBV)", footprint: "TI DBV0006", supply: "2.5–6.5 V; UVLO 2.35 V rising",
        current: "IIN_off 0.1/1 µA; IIN_on 100-170 µA", ratings: &[("rDS(on)", "85/95 mΩ 25 °C, 135 mΩ max -40..125 °C (DBV)"), ("IOS", "min 25230/R^1.016, max 22980/R^0.94 mA (R in kΩ, 15-232 k)"), ("Latch", "-1: off after 7.5 ms typ deglitch"), ("Reverse", "comparator 95-190 mV"), ("EN", "active high, VIH 1.1 V, VIL 0.66 V"), ("RθJA", "182.6 °C/W")],
        lifecycle: Lifecycle::Active, passives: &["RILIM 76.8 k 1 % (rev D, <= 400 mA provisional ceiling)", "IN 1 µF", "OUT 10 µF 16 V", "FAULT 100 k pull-up -> GPIO", "EN 100 k pull-down + GPIO"],
        alternatives: &["TPS2553DBVR (auto-retry)", "TPS25200 (not reviewed)"], datasheet: "DS-TPS255X", assumptions: &[], status: EngStatus::Recommended },
    ComponentSpec { key: "DISPSW", function: "+3V3_DISPLAY load switch / patch primary sensor switch", manufacturer: "Texas Instruments", mpn: "TPS22918DBVR",
        package: "SOT-23-6 (DBV)", footprint: "TI DBV0006", supply: "VIN 1–5.5 V", current: "IQ 8.3/15 µA; ISD 0.5/3.5 µA",
        ratings: &[("RON", "53 mΩ typ at 3.3 V; 71 mΩ max at -200 mA"), ("IOUT", "2 A max"), ("ON", "active high, low threshold, do not float"), ("QOD", "internal RPD when QOD = VOUT"), ("Reverse blocking", "not provided"), ("RθJA", "183.2 °C/W")],
        lifecycle: Lifecycle::Active, passives: &["CIN 1 µF", "CL 100 nF", "CT 1 nF", "QOD -> VOUT", "ON 47 k pull-down"],
        alternatives: &["TPS22919 (fixed slew)"], datasheet: "DS-TPS22918", assumptions: &[], status: EngStatus::Recommended },
    ComponentSpec { key: "GAUGE", function: "1S fuel gauge", manufacturer: "Analog Devices", mpn: "MAX17048G+T10",
        package: "TDFN-8 2 × 2 mm (V1 record)", footprint: "Pending datasheet re-download", supply: "2.5–4.5 V class (V1 record)", current: "~23 µA active, ~4 µA hibernate (V1 record - unverified)",
        ratings: &[("I²C address", "0x36 (V1 record - verify)")], lifecycle: Lifecycle::NotConfirmed, passives: &["CELL/VDD 1 µF", "on display/system I²C"],
        alternatives: &["VBAT divider only (already present)"], datasheet: "DS-MAX17048", assumptions: &["All values carried from V1; analog.com download failed 2026-11-02"], status: EngStatus::Unverified },
    ComponentSpec { key: "USBESD", function: "USB D+/D- ESD", manufacturer: "STMicroelectronics", mpn: "USBLC6-2SC6",
        package: "SOT-23-6", footprint: "Pending datasheet re-download", supply: "VBUS reference 5 V", current: "-", ratings: &[("Lines", "2 data + VBUS ref (V1 record)")],
        lifecycle: Lifecycle::NotConfirmed, passives: &[], alternatives: &["TPD2E2U06 (TI, downloaded, not yet reviewed)"], datasheet: "DS-USBLC6", assumptions: &["st.com download timed out; pinout carried from V1"], status: EngStatus::Unverified },
    ComponentSpec { key: "ESD4", function: "4-line low-C ESD (chain lines, AUX ports)", manufacturer: "Texas Instruments", mpn: "TPD4E05U06DQAR",
        package: "USON-10 (DQA) 2.5 × 1.0 mm", footprint: "TI DQA0010A", supply: "-", current: "ILEAK 10 nA max",
        ratings: &[("VRWM", "5.5 V"), ("VBR", "6.5 V min"), ("CL", "0.42-0.5 pF typ"), ("IEC 61000-4-2", "±12 kV contact / ±15 kV air")],
        lifecycle: Lifecycle::NotConfirmed, passives: &[], alternatives: &["TPD4E02B04"], datasheet: "DS-TPD4E05U06", assumptions: &["Orderable lifecycle line for DQAR not in extracted text - confirm"], status: EngStatus::RecommendedWithCondition },
    ComponentSpec { key: "USBC", function: "USB-C receptacle (power + USB device)", manufacturer: "GCT", mpn: "USB4105-GF-A",
        package: "SMT/through-hole stake USB-C 16-pin", footprint: "GCT USB4105 drawing", supply: "-", current: "5 A collectively on VBUS",
        ratings: &[("Voltage", "48 V DC"), ("Durability", "20 000 cycles"), ("Dielectric", "100 V AC")],
        lifecycle: Lifecycle::NotConfirmed, passives: &["CC1/CC2 5.1 k 1 % to GND", "Shell to GND via 1 MΩ || 4.7 nF (EMC practice, assumption)"],
        alternatives: &["GCT USB4085 (not reviewed)"], datasheet: "DS-GCT-USB4105", assumptions: &[], status: EngStatus::Recommended },
    ComponentSpec { key: "FPC12", function: "12-pin chain connector (main ×1, patch IN/OUT ×2)", manufacturer: "Hirose", mpn: "FH34SRJ-12S-0.5SH(50)",
        package: "SMT ZIF, 0.5 mm pitch, 8.0 × 3.8 × 1.0 mm (closed)", footprint: "Hirose 2D drawing CL0580-1253-0-50 (download link via product page)", supply: "-", current: "0.5 A per contact (use 70 % when all contacts carry current)",
        ratings: &[("Voltage", "50 V AC/DC"), ("Contact resistance", "100 mΩ max incl. FPC/FFC bulk"), ("Cycles", "20 insertions"), ("Contacts", "top & bottom (either FPC side)"), ("FPC", "0.3 mm thick"), ("Temperature", "-55..105 °C incl. self-heating")],
        lifecycle: Lifecycle::Active, passives: &[], alternatives: &["Molex 503480-1200 class (V1 note, not reviewed)"], datasheet: "DS-FH34-SPEC", assumptions: &["Pad numbering and contact-side mirroring to be transcribed from the 2D drawing before footprint creation (OQ-024)"], status: EngStatus::RecommendedWithCondition },
    ComponentSpec { key: "BATTCONN", function: "Battery connector", manufacturer: "JST", mpn: "S2B-PH-SM4-TB(LF)(SN)",
        package: "PH 2-pos SMT side entry, 2.0 mm pitch", footprint: "JST PH SMT drawing", supply: "-", current: "2 A (AWG24)",
        ratings: &[("Voltage", "100 V"), ("Temperature", "-40..105 °C"), ("Contact resistance", "10 mΩ initial / 20 mΩ after test")],
        lifecycle: Lifecycle::NotConfirmed, passives: &[], alternatives: &["B2B-PH-K-S (THT)"], datasheet: "DS-JST-PH", assumptions: &["Cell lead polarity must be checked per pack (no industry standard)"], status: EngStatus::Recommended },
    ComponentSpec { key: "DISPCONN", function: "Display connector (GND, +3V3_DISPLAY, SCL, SDA per source)", manufacturer: "JST", mpn: "SM04B-SRSS-TB(LF)(SN)",
        package: "SH 4-pos SMT side entry, 1.0 mm pitch", footprint: "Pending JST SH drawing", supply: "-", current: "1 A class (unverified)",
        ratings: &[], lifecycle: Lifecycle::NotConfirmed, passives: &["2.2 k pull-ups on main", "ESD optional"],
        alternatives: &["Any keyed 4-pin family different from the AUX connector to prevent cross-mating"], datasheet: "DS-JST-SH", assumptions: &["Chosen to differ from AUX (JST PH) because AUX pin order VCC-GND-SDA-SCL would reverse power on a source-ordered display port"], status: EngStatus::ClientDecision },
    // ------------------------------------------------------------- patch
    ComponentSpec { key: "SR", function: "Patch shift register (cascade) - source / rev C baseline", manufacturer: "Texas Instruments", mpn: "SN74HC595PWR",
        package: "TSSOP-16 (PW)", footprint: "TI PW0016A", supply: "2–6 V (specified at 2 / 4.5 / 6 V only)", current: "ICC 8 µA (25 °C) / 80 µA (-40..85 °C) at 6 V",
        ratings: &[("IOZ", "±0.5 µA 25 °C, ±5 µA -40..85 °C"), ("Thresholds", "datasheet warns of wrong states in the 0.5-1.5 V region at 2 V; 3.3 V not characterised"), ("Ioff", "none - inputs clamp to VCC when unpowered"), ("Pinout", "QA 15, QB-QH 1-7, GND 8, QH' 9, SRCLR 10, SRCLK 11, RCLK 12, OE 13, SER 14, VCC 16")],
        lifecycle: Lifecycle::Active, passives: &["100 nF", "47 k pull-down on Q0-Q7", "SRCLR to VCC", "33 Ω series on QH'"],
        alternatives: &["SN74LV595APWR (recommended, D-02): pin-identical, characterised at 3.3 V ± 0.3 V, Ioff 5 µA, 5.5 V-tolerant inputs"],
        datasheet: "DS-HC595", assumptions: &["3.3 V levels interpolated between the 2 V and 4.5 V tables"], status: EngStatus::RecommendedWithCondition },
    ComponentSpec { key: "SR_ALT", function: "Patch shift register - recommended pin-compatible alternate (D-02)", manufacturer: "Texas Instruments", mpn: "SN74LV595APWR",
        package: "TSSOP-16 (PW)", footprint: "TI PW0016A (same pinout as SN74HC595PW)", supply: "2–5.5 V, characterised at 3.3 V ± 0.3 V", current: "ICC 20 µA max",
        ratings: &[("Inputs", "5.5 V tolerant; VIH 0.7 VCC; VIL 0.3 VCC (3.0-3.6 V)"), ("IOZ", "±5 µA"), ("Ioff", "5 µA max (partial power-down)"), ("Outputs", "±8 mA at 3.0-3.6 V")],
        lifecycle: Lifecycle::Active, passives: &["as SN74HC595"], alternatives: &[], datasheet: "DS-LV595A", assumptions: &[], status: EngStatus::ClientDecision },
    ComponentSpec { key: "PRISW", function: "Primary signal isolation (CLK, SYNC, SD, ID) - source / rev C baseline", manufacturer: "Texas Instruments", mpn: "CD74HC4066PWR",
        package: "TSSOP-14 (PW)", footprint: "TI PW0014A", supply: "2–10 V (HC)", current: "ICC 20 µA max (-55..85 °C, 6 V)",
        ratings: &[("RON @1 mA", "VCC 4.5 V: 25 typ / 80 max (25 °C), 106 Ω (-40..85 °C); not specified at 3.3 V (higher)"), ("IZ off leakage", "±0.1 µA 25 °C; ±1 µA -55..85 °C"), ("CI", "5 pF"), ("Powered-off", "no protection")],
        lifecycle: Lifecycle::Active, passives: &["100 nF"],
        alternatives: &["TMUX1511PWR (recommended, D-03): RON ≤ 4.5 Ω, IS(OFF) ±100 nA, powered-off protected, 6 MΩ logic pull-downs"],
        datasheet: "DS-HC4066", assumptions: &[], status: EngStatus::NeedsAlternative },
    ComponentSpec { key: "PRISW_ALT", function: "Primary signal isolation - recommended alternate (D-03)", manufacturer: "Texas Instruments", mpn: "TMUX1511PWR",
        package: "TSSOP-14 (PW)", footprint: "TI PW0014A", supply: "1.5–5.5 V", current: "IDD 37/70 µA",
        ratings: &[("RON", "2 typ / 4.5 Ω max (ISD 8 mA)"), ("COFF / CON", "2.5–4 pF / 3.3–6 pF"), ("Signal range", "0 to 2 × VDD, 5.5 V max"), ("Powered-off", "up to 3.6 V, IPOFF ±2 µA"), ("Logic", "VIH 1.2 V, VIL 0.45 V, 6 MΩ internal pull-down"), ("IS(OFF)", "±100 nA"), ("Pinout", "1 SEL1, 2 S1, 3 D1, 4 SEL2, 5 S2, 6 D2, 7 GND, 8 D3, 9 S3, 10 SEL3, 11 D4, 12 S4, 13 SEL4, 14 VDD")],
        lifecycle: Lifecycle::Active, passives: &["100 nF"], alternatives: &[], datasheet: "DS-TMUX1511", assumptions: &[], status: EngStatus::ClientDecision },
    ComponentSpec { key: "AUXISO", function: "AUX bus isolator on chain pins 4/5 (rev C)", manufacturer: "Texas Instruments", mpn: "TMUX136RSER",
        package: "UQFN-10 (RSE) 2.0 × 1.5 mm", footprint: "TI RSE0010A", supply: "2.3–4.8 V", current: "ICC 30/50 µA (70 µA to 125 °C)",
        ratings: &[("RON", "5.7 typ / 9-9.5 Ω max (VCC 2.3-2.7 V, 1.65 V, 8 mA); rev C budget 13 Ω"), ("C on/off", "1.6 / 1.4 pF typ, 2 pF max"), ("IOZ", "±2 µA (±15 µA to 125 °C)"), ("Power-off", "I/O high impedance when VCC = 0"), ("EN", "active low, VIH 1.3 V, VIL 0.6 V"), ("Pinout", "1 A1, 2 A2, 3 B1, 4 B2, 5 GND, 6 EN, 7 COM2, 8 COM1, 9 SEL, 10 VCC")],
        lifecycle: Lifecycle::Active, passives: &["100 nF", "SEL tied low (A port used)", "EN from CD74HC138 /Y6"], alternatives: &["SN74CB3Q3253 as mux + isolator in one (A-port off 11 pF; reviewed, not adopted)"],
        datasheet: "DS-TMUX136", assumptions: &[], status: EngStatus::Recommended },
    ComponentSpec { key: "AUXMUX", function: "AUX port mux SDA/SCL dual 4:1 (rev C)", manufacturer: "Texas Instruments", mpn: "TMUX1109PWR",
        package: "TSSOP-16 (PW)", footprint: "TI PW0016A", supply: "1.08–5.5 V single", current: "IDD 0.006 µA typ",
        ratings: &[("RON @3.3 V", "4 typ / 8.75 (25 °C) / 9.5 (85 °C) / 9.75 Ω (125 °C)"), ("CDOFF / CON", "32 / 38 pF typ (behind the isolator)"), ("EN", "active high"), ("Pinout", "1 A0, 2 EN, 3 VSS, 4-7 S1A-S4A, 8 DA, 9 DB, 10-13 S4B-S1B, 14 VDD, 15 GND, 16 A1")],
        lifecycle: Lifecycle::Active, passives: &["100 nF VDD", "VSS to GND"], alternatives: &["TMUX1209 (similar, 38 pF)"], datasheet: "DS-TMUX1109", assumptions: &[], status: EngStatus::Recommended },
    ComponentSpec { key: "AUXDEC_PWR", function: "AUX port power decoder, active-high Y4..Y7 (rev C)", manufacturer: "Texas Instruments", mpn: "CD74HC238PWR",
        package: "TSSOP-16 (PW)", footprint: "TI PW0016A", supply: "2–6 V", current: "ICC 8 / 80 µA (6 V)",
        ratings: &[("Enables", "G2 high, G1/G0 low"), ("Pinout", "1 A0, 2 A1, 3 A2, 4 G0, 5 G1, 6 G2, 7 Y7, 8 GND, 9-15 Y6-Y0, 16 VCC")],
        lifecycle: Lifecycle::Active, passives: &["100 nF"], alternatives: &[], datasheet: "DS-HC238", assumptions: &["3.3 V levels interpolated; all inputs are driven rail-to-rail by the 595"], status: EngStatus::RecommendedWithCondition },
    ComponentSpec { key: "AUXDEC_BUS", function: "AUX bus-connect decoder, active-low /Y6 (rev C)", manufacturer: "Texas Instruments", mpn: "CD74HC138M96",
        package: "SOIC-16 (D) - SCHS147J lists no TSSOP orderable for HC138", footprint: "TI D0016A", supply: "2–6 V", current: "ICC 8 / 80 µA (6 V)",
        ratings: &[("Enables", "G2 high, G1/G0 low"), ("Pinout", "as CD74HC238")],
        lifecycle: Lifecycle::Active, passives: &["100 nF"], alternatives: &["SN74LVC138APWR (3.3 V characterised, TSSOP, same function table)"], datasheet: "DS-HC238", assumptions: &[], status: EngStatus::RecommendedWithCondition },
    ComponentSpec { key: "PORTSW", function: "AUX port power switch, current-limited, latch-off (rev C: TPS2553)", manufacturer: "Texas Instruments", mpn: "TPS2553DBVR-1",
        package: "SOT-23-6 (DBV)", footprint: "TI DBV0006", supply: "2.5–6.5 V", current: "IIN_off 1 µA max",
        ratings: &[("EN", "active high (CD74HC238 Y4..Y7)"), ("IOS @93.1 k", "248-329 mA worst case"), ("rDS(on)", "85/95 mΩ 25 °C, 135 mΩ -40..125 °C"), ("Reverse blocking", "yes"), ("Latch", "7.5 ms deglitch"), ("tr", "0.7-1.0 ms at 2.5 V, 1 µF")],
        lifecycle: Lifecycle::Active, passives: &["RILIM 93.1 k 1 %", "IN 1 µF", "OUT 10 µF + 100 nF", "bleeder 10 k (no QOD in DBV)", "FAULT -> 100 k + test pad"],
        alternatives: &["TPS25530DRVR-1 (QOD built in, WSON-6)", "TPS2553DBVR (auto-retry)"], datasheet: "DS-TPS255X", assumptions: &[], status: EngStatus::Recommended },
    ComponentSpec { key: "ARMDIODE", function: "AUX-armed sense diode, Q7 → diode → 150 k → pin 7 (rev C)", manufacturer: "TBD", mpn: "TBD (low-leakage switching diode)",
        package: "SOD-323 / SOT-23", footprint: "per selected part", supply: "-", current: "≈ 13 µA forward",
        ratings: &[("Requirement", "VF ≤ 0.65 V at 10 µA; IR ≤ 50 nA at 3.3 V and 85 °C; C ≤ 2 pF")], lifecycle: Lifecycle::NotConfirmed, passives: &["150 k 1 % series"],
        alternatives: &[], datasheet: "DS-NONE-DIODE", assumptions: &["Nexperia/onsemi/Diodes datasheet downloads blocked in this phase"], status: EngStatus::Unverified },
    ComponentSpec { key: "AUXCONN", function: "AUX I²C header (1 VCC, 2 GND, 3 SDA, 4 SCL) - APR-2 approved family JST-GH 1.25 mm", manufacturer: "JST", mpn: "SM04B-GHS-TB(LF)(SN)",
        package: "GH 4-pos SMT side entry, 1.25 mm pitch, locking, keyed (top-entry alternative BM04B-GHS-TBT)", footprint: "JST GH drawing - NOT YET VERIFIED", supply: "-", current: "UNVERIFIED (catalog not downloaded)",
        ratings: &[("Mating housing", "GHR-04V-S (to verify)"), ("Crimp", "SSHL-002T-P0.2 (to verify)"), ("Ratings", "UNVERIFIED - JST catalog required")], lifecycle: Lifecycle::NotConfirmed, passives: &["TPD4E05U06 per port"],
        alternatives: &["BM04B-GHS-TBT(LF)(SN) top entry (enclosure-dependent)", "JST-PH B4B-PH-K-S (rev C, superseded by APR-2)"], datasheet: "DS-JST-GH", assumptions: &["Exact MPN, footprint, cable assembly and clearances to be verified before BOM approval (APR-2)"], status: EngStatus::Unverified },
    ComponentSpec { key: "SHT45", function: "Primary T/RH sensor", manufacturer: "Sensirion", mpn: "SHT45-AD1B-R2",
        package: "DFN-4 1.5 × 1.5 × 0.5 mm, 0.8 mm pitch", footprint: "DS-SHT4X Fig. 17: 4 pads 0.5 × 0.3 mm, 1.4 mm span, NO copper under die", supply: "1.08–3.6 V",
        current: "320/500 µA measuring, 0.08/1.0 µA idle; heater up to 100 mA (not used)", ratings: &[("I²C", "0x44, Rp >= 390 Ω, Cb <= 400 pF (Rp >= 820 Ω)"), ("VIH/VIL", "0.7/0.3 VDD"), ("Pins", "1 SDA, 2 SCL, 3 VDD, 4 VSS"), ("Abs max any pin", "VDD + 0.3 V"), ("tPU", "1 ms max")],
        lifecycle: Lifecycle::Active, passives: &["100 nF", "No local pull-ups: main-board switched 2.2 k pull-ups (PRP-MB-02, SENSOR_PU_EN) in primary I²C mode"], alternatives: &["SHT45-AD1F-R2 (PTFE membrane, condensation/propolis protection)", "SHT41-AD1B (lower accuracy)"],
        datasheet: "DS-SHT4X", assumptions: &["Ordering table lists SHT45-AD1B-R2 as 0x44 (one table row shows 0x45 - verify marking on order)"], status: EngStatus::RecommendedWithCondition },
    ComponentSpec { key: "MIC", function: "Primary I²S MEMS microphone", manufacturer: "PUI Audio", mpn: "DMM-4026-B-I2S-R",
        package: "LGA-7 4.00 × 3.00 × 1.00 mm, bottom port Ø0.25 mm", footprint: "DS-PUI-MIC advisory land pattern (pads 1-3 / 5-7 0.6 × 0.4 mm, pad 4 ring Ø1.05/1.65)", supply: "1.62–3.63 V (pin table); 1.5–3.6 V (spec table)",
        current: "0.82–1.0 mA normal, 5 µA sleep (clock off)", ratings: &[("Format", "I²S 24-bit (18-bit precision) in 32-bit word"), ("BCLK", "2.048–4.096 MHz normal mode (fs 32-64 kHz)"), ("Load", "140 pF max on SD"), ("Startup", "20 ms"), ("SNR", "64 dB(A)"), ("Sensitivity", "-26 ± 1 dBFS"), ("Temp", "-40..100 °C"), ("Pins", "1 LR, 2 CONFIG (GND), 3 VDD, 4 GND, 5 WS, 6 SCK, 7 SD (tri-state, 100 k pull-down)")],
        lifecycle: Lifecycle::NotConfirmed, passives: &["100 nF", "LR -> GND (left)", "CONFIG -> GND"], alternatives: &["TDK T5848 / other current I²S mic - datasheet download blocked, not reviewed"],
        datasheet: "DS-PUI-MIC", assumptions: &["Lifecycle and distributor availability not confirmed", "SD drive strength / data-valid time not legible -> full-chain timing must be bench-proved (V-11)"], status: EngStatus::RecommendedWithCondition },
];

/// Find a spec.
pub fn spec(key: &str) -> Option<&'static ComponentSpec> {
    SPECS
        .iter()
        .chain(REV_D_SPECS.iter())
        .find(|s| s.key == key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn specs_unique_and_evidenced() {
        let mut k = HashSet::new();
        for s in SPECS {
            assert!(k.insert(s.key), "dup {}", s.key);
            assert!(
                crate::datasheets::find(s.datasheet).is_some(),
                "{}",
                s.datasheet
            );
        }
    }

    #[test]
    fn undownloaded_evidence_never_recommended() {
        for s in SPECS {
            let d = crate::datasheets::find(s.datasheet).unwrap();
            if d.local.is_none() {
                assert!(
                    matches!(s.status, EngStatus::Unverified | EngStatus::ClientDecision),
                    "{}",
                    s.key
                );
            }
        }
    }

    #[test]
    fn no_eol_part_selected() {
        assert!(SPECS.iter().all(|s| s.lifecycle != Lifecycle::Eol));
        assert!(!SPECS
            .iter()
            .any(|s| s.mpn.contains("ICS-43434") || s.mpn.contains("TPS62162")));
    }
}

/// Rev D (APR-1 option B) power parts. Kept separate so rev C records stay traceable.
pub const REV_D_SPECS: &[ComponentSpec] = &[
    ComponentSpec { key: "BOOST5V", function: "+5V0 boost for +5V_CHAIN (main board)", manufacturer: "Texas Instruments", mpn: "TPS61023DRLR",
        package: "SOT563 (DRL) 1.2 × 1.6 mm", footprint: "TI DRL0006A", supply: "VIN 0.5-5.5 V; VOUT 2.2-5.5 V set by divider",
        current: "IQ into VIN 0.9 typ / 3 uA max; 3.7 A valley switch limit", ratings: &[("L effective", "0.37 / 1.0 / 2.9 uH"), ("COUT effective", "4 / 10 / 1000 uF"), ("Disconnect", "true input-output disconnection when disabled"), ("Efficiency", "94 % at 3.6 V in, 5 V out, 1.5 A (front page)")],
        lifecycle: Lifecycle::NotConfirmed, passives: &["L 1.0 uH (Isat sized from valley limit and load)", "COUT >= 10 uF effective at 5 V", "FB divider for 5.0 V"],
        alternatives: &["TPS61099 (lower IQ, not reviewed)"], datasheet: "DS-TPS61023", assumptions: &["Efficiency at 0.2-0.4 A from curves not yet read (0.82-0.90 assumed in power::SCENARIOS)"], status: EngStatus::RecommendedWithCondition },
    ComponentSpec { key: "PATCHBUCK", function: "+3V3_LOCAL step-down on every patch", manufacturer: "Texas Instruments", mpn: "TPS62840DLCR",
        package: "SON-8 (DLC) 1.5 × 2 mm", footprint: "TI DLC0008", supply: "VIN 1.8-6.5 V",
        current: "IQ 60 nA (120 nA in 100 % mode); IOUT up to 750 mA", ratings: &[("VOUT", "1.8-3.3 V via RSET; 3.3 V with RSET 267 k (E96, 1 %)"), ("L effective", "1.51 / 2.2 / 2.9 uH"), ("COUT effective", "3 / 10 / 40 uF"), ("CIN effective", "1 / 4.7 uF")],
        lifecycle: Lifecycle::NotConfirmed, passives: &["L 2.2 uH", "CIN 4.7 uF", "COUT 10 uF", "RSET 267 k 1 %"],
        alternatives: &["TPS62841 (stop-pin variant)"], datasheet: "DS-TPS62840", assumptions: &["RSET row for 3.3 V to be re-checked for the selected variant", "Efficiency 0.80-0.90 assumed until curves are read"], status: EngStatus::RecommendedWithCondition },
];
