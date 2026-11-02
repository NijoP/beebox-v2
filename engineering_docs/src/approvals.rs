//! # Validation status and approvals (revision C)
//!
//! Final technical validation of the I²C AUX architecture. Each item is classified as:
//! * [`Evidence::Proven`]: follows from manufacturer datasheet values and/or exhaustive logic
//!   checks (unit tests in this crate).
//! * [`Evidence::NeedsTest`]: plausible, but depends on unspecified or assumed values. Needs bench evidence.
//! * Approval items in [`APPROVALS`]: genuine client decisions.
//!
//! Sign-off is **not** requested until APR-1 is decided and the NeedsTest items with
//! `blocks_schematic = true` are resolved by datasheet selection.

/// Evidence class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Evidence {
    /// Proven by datasheets / exhaustive logic model.
    Proven,
    /// Requires bench test or a missing datasheet.
    NeedsTest,
}

/// One validation finding.
#[derive(Debug, Clone, Copy)]
pub struct Finding {
    /// ID.
    pub id: &'static str,
    /// Statement.
    pub statement: &'static str,
    /// Evidence class.
    pub evidence: Evidence,
    /// Basis / next action.
    pub basis: &'static str,
    /// Whether it blocks schematic capture.
    pub blocks_schematic: bool,
}

/// Validation findings.
pub const FINDINGS: &[Finding] = &[
    Finding { id: "VAL-01", statement: "On any patch at most one AUX port can be powered, and the bus can connect only to that powered port.", evidence: Evidence::Proven, basis: "CD74HC238/138 function tables; exhaustive 256-state test of aux_expansion::patch_outputs.", blocks_schematic: false },
    Finding { id: "VAL-02", statement: "Primary activity (Q0 or Q1) blocks AUX power and AUX bus on the same patch.", evidence: Evidence::Proven, basis: "Decoder G0/G1 active-low enables; exhaustive test.", blocks_schematic: false },
    Finding { id: "VAL-03", statement: "Startup, ESP32 reset and fault (PATCH_OE_N high) leave every port unpowered and isolated.", evidence: Evidence::Proven, basis: "SN74HC595 high-Z with OE high + pull-downs -> all-zero state -> all off (test). Pull-up on PATCH_OE_N required.", blocks_schematic: false },
    Finding { id: "VAL-04", statement: "Firmware cannot express two simultaneously active paths.", evidence: Evidence::Proven, basis: "firmware::ActivePath produces at most one non-zero patch byte (test).", blocks_schematic: false },
    Finding { id: "VAL-05", statement: "System-wide exclusivity is detected in hardware before power and before bus connection (AUX-armed sense).", evidence: Evidence::NeedsTest, basis: "Window separation 0.94-1.20 V (one) vs 1.34-1.71 V (two) computed with placeholder diode VF; V-15.", blocks_schematic: false },
    Finding { id: "VAL-06", statement: "Hardware cannot *prevent* a second patch from being activated by a corrupted shift; only detect it.", evidence: Evidence::Proven, basis: "No free chain contact and no shift-chain read-back on the 12-pin backbone.", blocks_schematic: false },
    Finding { id: "VAL-07", statement: "Signals are isolated before power is removed, in normal and emergency shutdown.", evidence: Evidence::Proven, basis: "Separate latches; TMUX136 disable 200 ns vs TPS2553 t_off 3 ms (TI datasheets).", blocks_schematic: false },
    Finding { id: "VAL-08", statement: "Module supply stable before I2C connection.", evidence: Evidence::NeedsTest, basis: "Firmware delay >= TPS2553 t_on 3 ms + module power-up; no power-good feedback to the main board; V-05 scope check.", blocks_schematic: false },
    Finding { id: "VAL-09", statement: "CD74HC4052-class mux rejected; TMUX1109 + TMUX136 meet the R_ON budget (≈42 mV at 1.83 mA).", evidence: Evidence::Proven, basis: "TI datasheet max R_ON; calculations::vol_at_esp32_v.", blocks_schematic: false },
    Finding { id: "VAL-10", statement: "TMUX1109 directly on the chain exceeds 400 pF (≈463 pF at 0.10 m); per-patch TMUX136 isolator required.", evidence: Evidence::Proven, basis: "TMUX1109 CD(off) 32 pF typ x 7 unselected patches.", blocks_schematic: false },
    Finding { id: "VAL-11", statement: "Bus capacitance with isolator ≈255 pF (0.10 m), ≈375 pF (0.25 m), ≈575 pF (0.50 m): 100 kHz only; segment length must stay ≲0.28 m.", evidence: Evidence::NeedsTest, basis: "Cable pF/m, module-side and trace capacitance are ASSUMED; measure on the 8-segment chain (V-11).", blocks_schematic: false },
    Finding { id: "VAL-12", statement: "With FH34 max contact resistance alone (no cable) a 229 mA load on patch 8 drops 0.52 V -> port 2.78 V, below the 2.9 V criterion.", evidence: Evidence::Proven, basis: "Hirose 100 mohm max x 16 supply contacts + ground share + TPS2553 135 mohm max x 2.", blocks_schematic: true },
    Finding { id: "VAL-13", statement: "Per-position worst-case current at 0.10 m segments: patch 1 ≈525 mA, patch 4 ≈179 mA, patch 8 ≈95 mA.", evidence: Evidence::NeedsTest, basis: "Cable resistance ASSUMED (1.64 ohm/m); typical contact resistance is lower but unspecified.", blocks_schematic: true },
    Finding { id: "VAL-14", statement: "STCC4-class modules (4.2 mA peak) are supportable at every position; SCD41-class (205 mA peak) are not guaranteed beyond the first patches.", evidence: Evidence::Proven, basis: "VAL-12/13 with datasheet peaks.", blocks_schematic: true },
    Finding { id: "VAL-15", statement: "Total chain current bounded: base ≤ ~2.8 mA (8 patches, datasheet maxima) + one port ≤ 324 mA (TPS2553 max limit) < 500 mA FH34 rating.", evidence: Evidence::Proven, basis: "Datasheet maxima; one powered port system-wide.", blocks_schematic: false },
    Finding { id: "VAL-16", statement: "An address scan does not identify modules; identity commands per approved profile are required; unknown -> isolate + unpower.", evidence: Evidence::Proven, basis: "SCD40/41/43 share 0x62 (Sensirion/Adafruit).", blocks_schematic: false },
    Finding { id: "VAL-17", statement: "No researched methane module is a verified 3.3 V I2C module; MH-Z19C, S8, MQ-4, TGS2611, MH-440D are not I2C; MG811 unverified.", evidence: Evidence::Proven, basis: "research::MODULES.", blocks_schematic: false },
    Finding { id: "VAL-18", statement: "Adafruit SCD-4x breakout at 3.3 V VIN: regulator dropout and 205 mA peak vs 'at least 100 mA' regulator rating.", evidence: Evidence::NeedsTest, basis: "Adafruit learn page; bench test.", blocks_schematic: false },
    Finding { id: "VAL-19", statement: "FH34 mating durability is 20 cycles - patches cannot be re-cabled frequently in service.", evidence: Evidence::Proven, basis: "Hirose spec sheet.", blocks_schematic: false },
];

/// A client approval item with options.
#[derive(Debug, Clone, Copy)]
pub struct Approval {
    /// ID.
    pub id: &'static str,
    /// Question.
    pub question: &'static str,
    /// Options with consequences.
    pub options: &'static [crate::model::DecisionOption],
    /// Engineering recommendation.
    pub recommendation: &'static str,
}

/// Remaining approvals (grouped).
pub const APPROVALS: &[Approval] = &[
    Approval { id: "APR-1", question: "Chain power for high-current I2C modules (SCD41-class) at far patches.", options: &[
        crate::model::DecisionOption { label: "A", description: "Keep 3.3 V on pin 2; publish a per-position current rating; SCD41-class only on near patches, low-power modules (STCC4-class) anywhere.", consequences: "No backbone change; module placement restrictions; still needs measured contact/cable resistance." },
        crate::model::DecisionOption { label: "B (recommended if SCD41-class must work on any patch)", description: "Same pinout, but pin 2 carries a higher chain voltage (e.g. 5 V from the main board) and every patch has a local 3.3 V LDO feeding its logic and AUX ports.", consequences: "Pin positions unchanged but the electrical definition of pin 2 changes (approval required). Worst-case drop budget rises from ~0.4 V to ~1.4 V (≈330 mA at patch 8, 0.10 m). ID reference becomes the local 3.3 V (windows widened). +8 LDOs, main-board 5 V boost, extra heat/battery loss." },
        crate::model::DecisionOption { label: "C", description: "Convert one GND contact to a second supply contact.", consequences: "Changes the pinout; insufficient alone at patch 8 with cable resistance; not recommended." },
    ], recommendation: "Decide whether SCD41-class modules must work on every patch. If yes -> B; if no -> A." },
    Approval { id: "APR-2", question: "Physical AUX connector with the fixed VCC-GND-SDA-SCL order.", options: &[
        crate::model::DecisionOption { label: "A", description: "2.54 mm straight header.", consequences: "Direct Dupont wiring; no keying; reverse-plug risk; weak retention." },
        crate::model::DecisionOption { label: "B (recommended)", description: "Keyed, locking 4-pin wire-to-board connector, same pin order; adapter cable per breakout.", consequences: "Polarised and retained; requires adapter cables (Qwiic/Grove orders differ). Connector ratings to be verified from its datasheet." },
    ], recommendation: "B." },
    Approval { id: "APR-3", question: "Accept restricted module list and methane deferral.", options: &[], recommendation: "First release supports only modules with an approved profile (verified I2C at 3.3 V, within the per-position current rating, tolerant of power cycling). Methane deferred until such a module is identified; state whether ambient ppm or leak %LEL is wanted." },
    Approval { id: "APR-4", question: "System context still open: battery autonomy/charging, data destination and sampling/audio, patch size and segment length (I2C needs ≲0.28 m per segment), fabricator.", options: &[], recommendation: "Provide values; segment length directly limits I2C capacitance and supply drop." },
];

/// Engineering recommendations adopted in revision C (sign-off with APR items).
pub const RECOMMENDATIONS: &[(&str, &str)] = &[
    ("REC-01", "AUX SCL on chain pin 4, SDA on pin 5; pin 7 used only for ID, I2S SD and AUX-armed sense."),
    ("REC-02", "Per patch: TMUX136 bus isolator + TMUX1109 dual 4:1 port mux + CD74HC238 power decoder + CD74HC138 bus decoder + 4 x TPS2553 (R_ILIM 93.1 kohm, 252-324 mA)."),
    ("REC-03", "Q0-Q7 allocation per aux_expansion::Q_MAP; no second 74HC595; 64-bit image."),
    ("REC-04", "Staged arm -> verify -> power -> verify -> connect; isolate -> unpower -> disarm."),
    ("REC-05", "Port-side 2.2 kohm pull-ups to the port VCC; main-board sensor-bus pull-ups off in AUX mode; 100 kHz."),
    ("REC-06", "Identification by profile address + identity command + CRC; unknown -> isolate, unpower, log."),
    ("REC-07", "One EasyEDA project, boards MAIN and PATCH."),
];

/// Rev D: client decisions of 2026-11-01 and their engineering status.
pub const REV_D_DECISIONS: &[(&str, &str, crate::model::Status)] = &[
    ("APR-1", "Option B APPROVED subject to power validation: pin positions unchanged; pin 2 = +5V_CHAIN; local switching regulator +3V3_LOCAL on every patch; AUX VCC regulated 3.3 V only; SDA/SCL never 5 V. Validation in crate::power (chain <= 394 mA incl. tolerance; 400 mA provisional ceiling; thermal check open).", crate::model::Status::Approved),
    ("APR-2", "JST-GH 1.25 mm 4-pin keyed locking APPROVED as preferred candidate; order VCC, GND, SDA, SCL; keyed adapter cables for Qwiic/Grove/others. Exact MPN, footprint, cable assembly and clearances still to verify (JST documents not downloadable here).", crate::model::Status::Approved),
    ("APR-3", "Rev A supports qualified 3.3 V I2C breakout boards only; CO2 initial use case; methane target = background ppm; no methane hardware in the approved BOM until a genuine 3.3 V I2C module with manufacturer datasheet is found; UART/PWM/analog/heated modules only via a separately approved adapter; unknown modules isolated and OFF; approval per exact breakout.", crate::model::Status::Approved),
    ("APR-4 battery", "3000 mAh LiPo CONFIRMED (capacity). Cell configuration, voltages, currents and protection NOT yet verified - pack datasheet required.", crate::model::Status::ClientConfirmed),
    ("APR-4 other", "Provisional assumptions (not requirements): USB-C 5 V charging (solar later); Wi-Fi data, BLE configuration; offline storage to evaluate; 1-minute sampling to evaluate; audio parameters configurable; 100 mm segments for the prototype (280 mm not validated); manufacturer-neutral design.", crate::model::Status::Assumption),
];

/// Unresolved client questions after rev D (only items that need client input).
pub const CLIENT_QUESTIONS_REV_D: &[(&str, &str)] = &[
    ("Q-1 Battery pack", "Provide the 3000 mAh pack MPN/datasheet: cell configuration (1S?), nominal and maximum charge voltage (4.2 V vs 4.35 V HV), discharge cutoff, continuous/peak discharge rating (system needs >= 1.3 A peak), built-in protection, NTC (10 k?) and connector."),
    ("Q-2 USB-C charging rate", "Acceptable charge time? Default USB 500 mA input gives roughly 6-7 h for 3000 mAh (estimate); faster charging needs source-capability detection and more charger heat."),
    ("Q-3 Number of AUX modules in typical use", "Battery life scales almost linearly with the number of power-cycled CO2 modules read per minute (typical scenario assumes 2)."),
    ("Q-4 Upload interval", "Wi-Fi every 1, 5 or 15 minutes? (dominant term in the worst-case estimate)."),
    ("Q-5 Display", "Display module and whether it is always on or on demand (always-on adds ~10 mA assumed, i.e. ~1/3 of the worst-case budget)."),
    ("Q-6 Methane", "Continue searching for a 3.3 V I2C ppm-level methane breakout, or accept CO2-only Rev A?"),
    ("Q-7 V1 repository", "GitHub was unreachable from this machine during rev D; confirm the V1 audit records in v1_audit are complete."),
];
