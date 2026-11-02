//! # Module and component research (datasheet evidence)
//!
//! All numeric values here were **read from the manufacturer documents** archived in
//! `bom/datasheets/` or from the cited manufacturer page on 2026-11-01. Values marked
//! `Unverified` were not available in the documents reviewed. Nothing is inferred.
//!
//! ## Verdict summary for the I²C-only AUX ports
//! | Module | Gas | Interface | Verdict |
//! |---|---|---|---|
//! | Sensirion SCD41 (sensor IC) | CO2 | I²C 0x62 | **Compatible class** at 3.3 V; specific breakout must be verified |
//! | Sensirion STCC4 (sensor IC) | CO2 | I²C 0x64/0x65 | **Compatible class** at 3.3 V; specific breakout must be verified |
//! | Winsen MH-Z19C | CO2 | UART/PWM, 5 V | Not compatible (not I²C, 5 V supply) |
//! | Senseair S8 LP | CO2 | UART Modbus, 4.5-5.25 V | Not compatible |
//! | Winsen MH-440D | CH4 | UART/analog, 3.6-5 V | Not compatible |
//! | Winsen MQ-4 | CH4 | analog heater sensor, 5 V | Not compatible (also ~1 W continuous) |
//! | Figaro TGS2611-E00 | CH4 | analog heater sensor, 5 V | Not compatible |
//! | DFRobot SEN0565 / SEN0440 | CH4 (MEMS) | analog breakout | Not compatible |
//! | DFRobot SEN0654 (TDLAS) | CH4 %LEL | not stated on product page | **Unverified** - wiki/datasheet needed |
//! | Winsen MG811 | CO2 | manufacturer datasheet not obtainable (404) | **Unverified** - not compatible until an exact I²C breakout is documented |
//! | Adafruit SCD-4x breakout | CO2 | I²C 0x62, VIN 3-5 V, on-board 3.3 V regulator ("at least 100 mA"), level shifter, 10 kΩ pull-ups, LED | **Conditional** - regulator dropout at 3.3 V VIN and 205 mA peak vs regulator rating must be bench-tested |
//!
//! MQ-series and MG811 boards that add an I²C ADC still need a 5-6 V heater supply (MQ-4: 5 V, about 1 W, ≥ 48 h preheat),
//! so they remain incompatible with a 3.3 V, power-cycled AUX port.
//!
//! **Methane finding:** no methane module with a verified I²C interface that runs at 3.3 V and
//! tolerates power cycling was identified. Every methane device reviewed is specified in
//! leak/flammability ranges (MQ-4 300-10 000 ppm, TGS2611 1-25 %LEL, MH-440D 0-10 %vol,
//! SEN0654 3-100 %LEL), and several need continuous heating or preheat (MQ-4: >= 48 h preheat).

use crate::model::Status;

/// Interface verdict for an I²C-only, 3.3 V, power-cycled AUX port.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuxVerdict {
    /// Electrical interface compatible; the specific breakout board still needs verification.
    CompatibleClass,
    /// Not compatible with the confirmed AUX interface.
    NotCompatible,
    /// Data insufficient.
    Unverified,
}

/// Value read from a document or explicitly unavailable.
#[derive(Debug, Clone, Copy)]
pub enum Spec {
    /// Text exactly as specified (units included).
    Stated(&'static str),
    /// Not found in the reviewed document.
    Unverified,
}

/// Datasheet record of a gas sensor / module.
#[derive(Debug, Clone, Copy)]
pub struct ModuleRecord {
    /// Manufacturer and part.
    pub part: &'static str,
    /// Target gas.
    pub gas: &'static str,
    /// Technology.
    pub technology: &'static str,
    /// Supply voltage.
    pub supply: Spec,
    /// Peak current.
    pub peak_current: Spec,
    /// Normal / average current.
    pub normal_current: Spec,
    /// Warm-up / preheat.
    pub warm_up: Spec,
    /// Interface.
    pub interface: Spec,
    /// Signal / logic level.
    pub signal_level: Spec,
    /// Dimensions.
    pub dimensions: Spec,
    /// Measurement range.
    pub range: Spec,
    /// Other electrical requirements.
    pub notes: &'static str,
    /// Evidence document.
    pub document: &'static str,
    /// Verdict for AUX.
    pub verdict: AuxVerdict,
}

/// Researched modules.
pub const MODULES: &[ModuleRecord] = &[
    ModuleRecord { part: "Adafruit SCD-40/41/43 breakout (sensor SCD4x)", gas: "CO2", technology: "Photoacoustic NDIR on breakout", supply: Spec::Stated("VIN = logic level of host (3-5 V); on-board 3.3 V regulator, 3Vo provides at least 100 mA"), peak_current: Spec::Stated("Sensor 205 mA max @3.3 V (Sensirion); breakout overhead not stated"), normal_current: Spec::Unverified, warm_up: Spec::Stated("Per SCD4x"), interface: Spec::Stated("I2C 0x62 (STEMMA QT + 0.1 in header)"), signal_level: Spec::Stated("Level-shifted to VIN; 10 kohm pull-ups on SCL/SDA"), dimensions: Spec::Unverified, range: Spec::Stated("Per SCD4x"), notes: "Regulator in dropout at 3.3 V VIN; regulator rating vs 205 mA peak; LED current - conditional, bench test required.", document: "https://learn.adafruit.com/adafruit-scd-40-and-scd-41/pinouts", verdict: AuxVerdict::Unverified },
    ModuleRecord { part: "Winsen MG811", gas: "CO2", technology: "Solid electrolyte (per Winsen naming)", supply: Spec::Unverified, peak_current: Spec::Unverified, normal_current: Spec::Unverified, warm_up: Spec::Unverified, interface: Spec::Unverified, signal_level: Spec::Unverified, dimensions: Spec::Unverified, range: Spec::Unverified, notes: "Manufacturer documents returned 404; no specification claimed. Not compatible unless an exact breakout documents I2C at 3.3 V within port limits.", document: "winsen-sensor.com (unavailable 2026-11-01)", verdict: AuxVerdict::NotCompatible },
    ModuleRecord { part: "Sensirion SCD41 (SCD4x family)", gas: "CO2", technology: "Photoacoustic NDIR", supply: Spec::Stated("2.4-5.5 V (typ 3.3 or 5.0 V); VDD and VDDH same supply"), peak_current: Spec::Stated("175 mA typ / 205 mA max @3.3 V; 115/137 mA @5 V"), normal_current: Spec::Stated("15/18 mA periodic (5 s) @3.3 V; 3.2/3.5 mA low-power periodic; 0.45/0.5 mA single shot every 5 min"), warm_up: Spec::Stated("Power-up/soft reset <= 30 ms; measure_single_shot max 5000 ms"), interface: Spec::Stated("I2C 0x62, standard and fast mode (<= 400 kHz)"), signal_level: Spec::Stated("VIH >= 0.65 x VDD, VIL <= 0.3 x VDD; SDA/SCL max VDD + 0.3 V"), dimensions: Spec::Stated("10.1 x 10.1 x 6.5 mm"), range: Spec::Stated("Specified 400-5000 ppm (SCD41 single shot listed)"), notes: "Unloaded supply ripple <= 30 mV p-p; supply must be kept constant; LDO able to handle peak recommended; external pull-ups required.", document: "bom/datasheets/Sensirion_SCD4x_Datasheet_D1.pdf", verdict: AuxVerdict::CompatibleClass },
    ModuleRecord { part: "Sensirion STCC4", gas: "CO2", technology: "Thermal conductivity", supply: Spec::Stated("2.7-5.5 V (typ 3.3 V)"), peak_current: Spec::Stated("3.7 typ / 4.2 max mA"), normal_current: Spec::Stated("~100 uA average in single-shot mode (datasheet condition footnote applies)"), warm_up: Spec::Unverified, interface: Spec::Stated("I2C 0x64 (alt 0x65 via ADDR); has secondary controller port for SHT4x"), signal_level: Spec::Unverified, dimensions: Spec::Unverified, range: Spec::Stated("Output range 380-32 000 ppm; accuracy +/-(100 ppm + 10 %)"), notes: "Unloaded supply ripple 30 mV. Lower power than SCD41.", document: "bom/datasheets/Sensirion_STCC4_Datasheet_D1.pdf", verdict: AuxVerdict::CompatibleClass },
    ModuleRecord { part: "Winsen MH-Z19C", gas: "CO2", technology: "NDIR", supply: Spec::Stated("5.0 +/- 0.1 V DC"), peak_current: Spec::Stated("125 mA @5 V"), normal_current: Spec::Stated("< 40 mA @5 V"), warm_up: Spec::Stated("Preheat 1 min; T90 < 120 s"), interface: Spec::Stated("UART (TTL 3.3 V) and PWM"), signal_level: Spec::Stated("3.3 V (compatible with 5 V)"), dimensions: Spec::Unverified, range: Spec::Stated("400-2000/5000/10000 ppm (optional)"), notes: "Not I2C.", document: "bom/datasheets/Winsen_MH-Z19C_manual_v1.2.pdf", verdict: AuxVerdict::NotCompatible },
    ModuleRecord { part: "Senseair S8 LP", gas: "CO2", technology: "NDIR", supply: Spec::Stated("4.5-5.25 V"), peak_current: Spec::Stated("300 mA"), normal_current: Spec::Stated("18 mA average"), warm_up: Spec::Stated("< 60 s"), interface: Spec::Stated("UART (Modbus)"), signal_level: Spec::Unverified, dimensions: Spec::Stated("33.9 x 19.8 x 8.7 mm"), range: Spec::Unverified, notes: "Not I2C.", document: "bom/datasheets/Senseair_S8_LP_PSH1944.pdf", verdict: AuxVerdict::NotCompatible },
    ModuleRecord { part: "Winsen MH-440D", gas: "CH4", technology: "NDIR", supply: Spec::Stated("3.6-5 V DC (manual: power via safety barrier)"), peak_current: Spec::Unverified, normal_current: Spec::Stated("85 mA average"), warm_up: Spec::Stated("3 min"), interface: Spec::Stated("UART (3.0 V) and analog 0.4-2.0 V"), signal_level: Spec::Stated("3.0 V"), dimensions: Spec::Stated("diameter 20 x 22.4 mm"), range: Spec::Stated("0-10 %vol selectable"), notes: "Not I2C; leak range.", document: "bom/datasheets/Winsen_MH-440D_manual.pdf", verdict: AuxVerdict::NotCompatible },
    ModuleRecord { part: "Winsen MQ-4", gas: "CH4", technology: "Heated metal-oxide", supply: Spec::Stated("VC 5.0 +/- 0.1 V; VH 5.0 +/- 0.1 V AC/DC"), peak_current: Spec::Unverified, normal_current: Spec::Stated("Heater RH 28 +/- 3 ohm; heater consumption <= ~1 W (as printed)"), warm_up: Spec::Stated("Preheat not less than 48 h"), interface: Spec::Stated("Analog (load-resistor divider)"), signal_level: Spec::Stated("Vs 2.5-4.0 V in 5000 ppm CH4"), dimensions: Spec::Unverified, range: Spec::Stated("300-10 000 ppm"), notes: "Not I2C; continuous heater.", document: "bom/datasheets/Winsen_MQ-4_manual.pdf", verdict: AuxVerdict::NotCompatible },
    ModuleRecord { part: "Figaro TGS2611-E00", gas: "CH4", technology: "Heated metal-oxide", supply: Spec::Stated("VH 5.0 +/- 0.2 V; VC 5.0 +/- 0.2 V"), peak_current: Spec::Unverified, normal_current: Spec::Stated("IH 56 +/- 5 mA; PH 280 +/- 25 mW"), warm_up: Spec::Stated("Preheating before test: 4 days (as extracted; confirm on document)"), interface: Spec::Stated("Analog"), signal_level: Spec::Unverified, dimensions: Spec::Stated("diameter 9.2 mm"), range: Spec::Stated("1-25 %LEL"), notes: "Not I2C; continuous heater.", document: "bom/datasheets/Figaro_TGS2611-E00_product_information_rev02.pdf", verdict: AuxVerdict::NotCompatible },
    ModuleRecord { part: "DFRobot SEN0565 Fermion MEMS CH4", gas: "CH4", technology: "MEMS heated", supply: Spec::Unverified, peak_current: Spec::Unverified, normal_current: Spec::Stated("<= 20 mA (product page)"), warm_up: Spec::Unverified, interface: Spec::Stated("Analog breakout; qualitative only (product page)"), signal_level: Spec::Unverified, dimensions: Spec::Stated("13 x 13 x 2.5 mm (sensor)"), range: Spec::Stated("1-10 000 ppm (title)"), notes: "Not I2C.", document: "https://www.dfrobot.com/product-2708.html", verdict: AuxVerdict::NotCompatible },
    ModuleRecord { part: "DFRobot SEN0654 Gravity laser CH4", gas: "CH4", technology: "TDLAS", supply: Spec::Unverified, peak_current: Spec::Unverified, normal_current: Spec::Unverified, warm_up: Spec::Stated("15 s"), interface: Spec::Unverified, signal_level: Spec::Unverified, dimensions: Spec::Unverified, range: Spec::Stated("3-100 %LEL"), notes: "Ships with 'Gravity 4-pin' cable; interface/supply must be confirmed from wiki/datasheet before any claim.", document: "https://www.dfrobot.com/product-2973.html", verdict: AuxVerdict::Unverified },
];

/// Switch / interconnect evidence.
pub const COMPONENT_FACTS: &[(&str, &str, &str, Status)] = &[
    ("TI TMUX1109 (selected candidate: per-patch 4:1 port mux)", "Supply 1.08-5.5 V; @3.3 V RON 4 typ / 8.75 max (25 C), 9.75 max (-40..125 C); CS(off) 7 pF, CD(off) 32 pF, CON 38 pF (1 MHz); EN active-high, all switches off when EN low; 1.8 V logic thresholds (VIH 1.35-1.49 V); IDD 0.006 uA typ; break-before-make; fail-safe logic", "ti.com/lit/ds/symlink/tmux1109.pdf", Status::Candidate),
    ("TI TMUX136 (selected candidate: per-patch bus isolator)", "VCC 2.3-4.8 V; RON 5.7 typ / 9-9.5 max (-40..85 C), 13 max (-40..125 C); CON/COFF 1.4-1.6 typ, 2 pF max; EN active-low, EN high = both ports high-Z; IOFF power-off isolation; VI/O 0-3.6 V; ICC 30 typ / 50 max uA; enable 100 us, disable 200 ns", "ti.com/lit/ds/symlink/tmux136.pdf", Status::Candidate),
    ("TI CD74HC238 / CD74HC138 (decoders)", "Enabled when G2=H, G1=L, G0=L; 238 outputs active-high, 138 active-low; all outputs inactive when disabled; ICC 8 uA (25 C) / 80 uA (-40..85 C) / 160 uA (125 C) at 6 V", "ti.com/lit/ds/symlink/cd74hc238.pdf", Status::Candidate),
    ("TI TPS2553 (selected candidate: per-port current-limited switch)", "VIN 2.5-6.5 V; rDS(on) 85 typ / 95 max mohm (25 C), 135 max (-40..125 C); ILIM: IOSmin = 25230/R^1.016, IOSnom = 23950/R^0.977, IOSmax = 22980/R^0.94 (mA, R in kohm); EN active-high (TPS2553); reverse-voltage protection; off current 0.1-1 uA; on current 120-150 uA (20 kohm, 6.5 V); t_on/t_off 3 ms (CL 1 uF, RL 100 ohm); QOD variant exists", "ti.com/lit/ds/symlink/tps2553.pdf", Status::Candidate),
    ("TI SN74HC595", "Outputs high-impedance when OE high; ICC 8 uA (25 C) / 80 uA / 160 uA at 6 V", "ti.com/lit/ds/symlink/sn74hc595.pdf", Status::Candidate),
    ("TI CD74HC4066 (existing primary switch)", "Switch input capacitance 5 pF; ICC 18.5 uA (25 C) / 20 uA (-55..85 C) at 6 V", "ti.com/lit/ds/symlink/cd74hc4066.pdf", Status::Candidate),
    ("Espressif ESP32-S3", "VIH min 0.75 x VDD; VIL max 0.25 x VDD; pin capacitance 2 pF", "espressif.com esp32-s3_datasheet_en.pdf", Status::Candidate),
    ("Hirose FH34SRJ spec sheet", "Contact resistance 100 mohm max (initial and after environmental tests); rated 0.5 A; -55..+105 C; mating durability 20 cycles", "hirose.com spec sheet CL0580-1253-0-50", Status::Candidate),
    ("Hirose FH34SRJ-12S-0.5SH(50)", "Rated current 0.5 A; 0.5 mm pitch; 0.3 mm FPC", "hirose.com product page CL0580-1253-0-50", Status::Candidate),
    ("TI PCA9546A", "Power-up all channels deselected; RESET; address 0x70-0x77; Cio(off) 15 typ / 19 max pF upstream - REJECTED: consumes an address users may need and adds 15-19 pF per patch", "ti.com/lit/ds/symlink/pca9546a.pdf", Status::Rejected),
    ("TI CD74HC4052", "Dual 4:1 analog mux, 2-6 V; rON in the ~90-225 ohm range at 1 mA (3.3 V not tabulated) - REJECTED: R_ON too high for I2C low-level budget", "ti.com/lit/ds/symlink/cd74hc4052.pdf", Status::Rejected),
];

/// Connector evidence status. JST datasheets could not be downloaded (HTTP 403).
pub const AUX_CONNECTOR_EVIDENCE: &str = "No AUX connector rating verified yet; JST site blocked automated download. Ratings must be taken from the selected manufacturer drawing.";
