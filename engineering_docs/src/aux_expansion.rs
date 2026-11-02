//! # 5. Four-port AUX expansion system (I²C-only, revision C - validated architecture)
//!
//! ## Confirmed client requirements (frozen)
//! * Four standard 4-pin I²C headers AUX1..AUX4 (`J_AUX1`..`J_AUX4`) on every patch, 32 per
//!   system. Pinout **1 VCC, 2 GND, 3 SDA, 4 SCL** ([`AUX_HEADER_PINOUT`]).
//! * Exactly **one** active sensor/AUX path across the whole eight-patch chain. Only one of the 32 ports
//!   may be powered and connected to the shared I²C bus at a time.
//! * All outputs OFF at startup, reset and faults. No 5 V signal may reach the ESP32-S3. No SPI on AUX.
//! * 12-pin backbone and 74HC595 cascade preserved.
//!
//! ## Validated per-patch circuit (engineering proposal - datasheet-verified parts, not yet approved)
//! ```text
//!  chain pin 4 SCL ─┬─ primary CD74HC4066 (Q1) ─ SHT45 / mic
//!  chain pin 5 SDA ─┤
//!                   └─ TMUX136 bus isolator (/EN from CD74HC138 Y6) ── TMUX1109 dual 4:1 (EN=Q5, A1:A0=Q4:Q3)
//!                          ≤2 pF off, ≤13 Ω on                         ≤9.75 Ω on, 38 pF on
//!                                                                         ├─ J_AUX1..4 SDA/SCL (+2.2 kΩ pull-ups to that port's VCC)
//!  +5V_CHAIN ─ TPS62840 ─ +3V3_LOCAL ── 4 × TPS2553 (EN = CD74HC238 Y4..Y7, I_LIM 248–329 mA incl. tolerance) ── J_AUX1..4 VCC (rev D)
//!  J_AUX1..4 = JST-GH 1.25 mm 4-pin (APR-2), 1 VCC 2 GND 3 SDA 4 SCL
//!  Q7 AUX_ARM ── diode ── 150 kΩ ── chain pin 7 (AUX-armed sense read by ESP32-S3 ADC1)
//! ```
//!
//! ## How exclusivity is achieved
//! | Scope | Mechanism | Strength |
//! |---|---|---|
//! | Within a patch | One 2-bit address drives both the power decoder and the I²C mux, so only one port can be powered, and the bus connects only to that same port | **Hardware** (decoder truth table) |
//! | Primary vs AUX on a patch | Q0/Q1 drive the active-low enables of both decoders, so any primary activity blocks AUX power and AUX bus | **Hardware** |
//! | Across the 8 patches | Firmware builds the 64-bit image from one [`crate::firmware::ActivePath`] value (at most one non-zero patch byte). It then **verifies** with the AUX-armed sense that exactly one patch is armed before power, and again before bus connection | Firmware + **hardware detection** |
//!
//! True hardware *prevention* across patches is impossible without an extra chain signal
//! (the 12-pin backbone has no free contact and no read-back of the shift chain). The staged
//! arm → verify → power → verify → connect sequence turns any multi-patch error into a detected
//! fault before a second module is connected to the bus. The per-port current limit bounds
//! the brief exposure window. See [`ARM_SENSE`].

use crate::model::Status;

/// Reference designators of the four ports. Fixed on every patch.
pub const AUX_PORT_DESIGNATORS: [&str; crate::purpose::AUX_PORTS_PER_PATCH] =
    ["J_AUX1", "J_AUX2", "J_AUX3", "J_AUX4"];

/// Client-confirmed AUX header pinout (pin, signal). **Do not change.**
pub const AUX_HEADER_PINOUT: [(u8, &str); 4] = [(1, "VCC"), (2, "GND"), (3, "SDA"), (4, "SCL")];
/// Status of [`AUX_HEADER_PINOUT`].
pub const AUX_HEADER_PINOUT_STATUS: Status = Status::ClientConfirmed;
/// AUX protocol.
pub const AUX_PROTOCOL: &str =
    "I2C standard-mode 100 kHz (fast-mode not supported on the full chain, see calculations)";

/// AUX VCC voltage policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoltagePolicy {
    /// VCC = 3.3 V nominal from the chain supply. **Recommended.**
    Only3v3,
    /// Additional 5 V per patch with level translation. Rejected unless a required module needs it.
    Add5v,
}
/// Recommended policy.
pub const RECOMMENDED_VOLTAGE_POLICY: VoltagePolicy = VoltagePolicy::Only3v3;

/// One output of the patch 74HC595.
#[derive(Debug, Clone, Copy)]
pub struct ControlBit {
    /// Register output.
    pub output: &'static str,
    /// Net.
    pub net: &'static str,
    /// Drives.
    pub drives: &'static str,
}

/// Complete Q0-Q7 allocation (Q0-Q2 baseline, Q3-Q7 proposal). Every net has a 100 kΩ-class
/// pull-down (value to be fixed in P2), so high-Z outputs (PATCH_OE_N high) read as 0. The
/// all-zero image is the safe state.
pub const Q_MAP: [ControlBit; 8] = [
    ControlBit {
        output: "Q0",
        net: "PRI_PWR_EN",
        drives: "Primary load switch EN; CD74HC238 G0 (active-low); CD74HC138 G0 (active-low)",
    },
    ControlBit {
        output: "Q1",
        net: "PRI_SIG_EN",
        drives:
            "Primary CD74HC4066 controls (3 lines); CD74HC238 G1; CD74HC138 G1 (both active-low)",
    },
    ControlBit {
        output: "Q2",
        net: "ID_EN",
        drives: "CD74HC4066 ID-resistor channel (unchanged baseline)",
    },
    ControlBit {
        output: "Q3",
        net: "AUX_A0",
        drives: "CD74HC238 A0; TMUX1109 A0",
    },
    ControlBit {
        output: "Q4",
        net: "AUX_A1",
        drives: "CD74HC238 A1; TMUX1109 A1",
    },
    ControlBit {
        output: "Q5",
        net: "AUX_SIG_EN",
        drives: "TMUX1109 EN (active-high); CD74HC138 G2 (active-high)",
    },
    ControlBit {
        output: "Q6",
        net: "AUX_PWR_EN",
        drives: "CD74HC238 G2 (active-high); CD74HC138 A1",
    },
    ControlBit {
        output: "Q7",
        net: "AUX_ARM",
        drives: "CD74HC238 A2; CD74HC138 A2; AUX-armed sense (diode + 150 kΩ to chain pin 7)",
    },
];

/// Gate equations, derived from the CD74HC238/CD74HC138 function tables (TI SCHS147):
/// enabled when G2 = H, G1 = L, G0 = L; 238 outputs active-high, 138 outputs active-low.
pub const LOGIC_EQUATIONS: &[&str] = &[
    "PORT_n_PWR (TPS2553 EN) = 238.Y(4+n) = Q6 & !Q1 & !Q0 & Q7 & (Q4:Q3 == n)",
    "TMUX136 /EN = 138./Y6 -> bus connected = Q5 & !Q1 & !Q0 & Q7 & Q6",
    "TMUX1109 EN = Q5; selected source = Q4:Q3 (same address as power)",
    "AUX_ARMED sense = Q7",
    "238.Y0..Y3 and 138 Y0..Y5, Y7 unused (left open)",
];

/// Output state of one patch for a given register byte (bit n = Qn).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PatchOutputs {
    /// Primary sensor load switch on.
    pub primary_power: bool,
    /// Primary sensor signal switches closed.
    pub primary_signal: bool,
    /// ID resistor branch connected to pin 7.
    pub id_branch: bool,
    /// Powered AUX port (0..=3), if any.
    pub aux_port_powered: Option<u8>,
    /// AUX I²C connected to chain pins 4/5 (through TMUX136).
    pub aux_bus_connected: bool,
    /// Port selected by TMUX1109 (behind the isolator) when its EN is high.
    pub mux_port: Option<u8>,
    /// AUX-armed sense current injected onto pin 7.
    pub aux_armed: bool,
}

/// Behavioural model of the proposed patch logic. `oe_n = true` (PATCH_OE_N high) makes every
/// 595 output high-impedance, and the pull-downs force 0.
pub const fn patch_outputs(q_in: u8, oe_n: bool) -> PatchOutputs {
    let q = if oe_n { 0 } else { q_in };
    let q0 = q & 0x01 != 0;
    let q1 = q & 0x02 != 0;
    let q2 = q & 0x04 != 0;
    let q5 = q & 0x20 != 0;
    let q6 = q & 0x40 != 0;
    let q7 = q & 0x80 != 0;
    let addr = (q >> 3) & 0b11;
    // CD74HC238: G2 = Q6, G1 = Q1, G0 = Q0, A2 = Q7, A1 = Q4, A0 = Q3; ports on Y4..Y7.
    let dec_en = q6 && !q1 && !q0;
    let aux_port_powered = if dec_en && q7 { Some(addr) } else { None };
    // CD74HC138: G2 = Q5, G1 = Q1, G0 = Q0, A2 = Q7, A1 = Q6, A0 = GND; /Y6 drives TMUX136 /EN.
    let bus_dec_en = q5 && !q1 && !q0;
    let aux_bus_connected = bus_dec_en && q7 && q6;
    PatchOutputs {
        primary_power: q0,
        primary_signal: q1,
        id_branch: q2,
        aux_port_powered,
        aux_bus_connected,
        mux_port: if q5 { Some(addr) } else { None },
        aux_armed: q7,
    }
}

/// Representative truth-table rows (patch level). Exhaustively verified over all 256 states
/// by unit tests.
pub const TRUTH_TABLE: &[(&str, &str, &str)] = &[
    // (condition, Q7..Q0, result)
    (
        "Power-up / ESP32 reset / fault (OE_N high)",
        "high-Z -> 0000_0000",
        "All off: no primary, no AUX power, bus isolated",
    ),
    ("CHAIN_INIT zero image", "0000_0000", "All off"),
    ("Primary identify", "0000_0100", "ID branch only"),
    (
        "Primary T/RH measure",
        "0000_0011",
        "Primary power + signal; AUX decoders disabled",
    ),
    (
        "AUX armed, port n",
        "1000_nn00",
        "Sense on pin 7; no power, bus isolated",
    ),
    (
        "AUX powered, port n",
        "1100_nn00",
        "Port n VCC on; bus isolated",
    ),
    (
        "AUX connected, port n",
        "1110_nn00",
        "Port n VCC on; port n SDA/SCL on chain pins 5/4",
    ),
    (
        "Q6 without Q7",
        "0100_nn00",
        "238 selects Y0..Y3 (unused): no port powered",
    ),
    (
        "Q5 without Q6/Q7",
        "0010_nn00",
        "138 /Y6 high: bus isolated (mux enabled behind isolator only)",
    ),
    (
        "Any AUX bits + Q0 or Q1",
        "111x_nn0x / 111x_nn1x",
        "AUX power and bus blocked by decoder G0/G1",
    ),
];

/// AUX-armed sense: detects, before power and before bus connection, how many patches are armed.
/// Sense description and provisional windows (diode VF range is a placeholder until the diode is selected).
pub const ARM_SENSE: &[&str] = &[
    "Each patch: Q7 -> small-signal diode (anode at Q7) -> 150 kohm -> chain pin 7. Main board 100 kohm pull-down and ADC1 already exist for patch ID.",
    "V(n armed) = (V_Q7 - V_F) x 100k / (150k/n + 100k). With V_Q7 - V_F = 2.35-3.00 V: n=1 -> 0.94-1.20 V, n=2 -> 1.34-1.71 V, n=0 -> ~0 V.",
    "Provisional windows: <0.40 V none; 0.85-1.25 V exactly one; >1.30 V two or more (fault).",
    "Pin 7 is otherwise idle in AUX mode (primary I2S off, ID branch off by firmware rule). Diodes block when Q7 = 0, so I2S SD and ID are not loaded except through 150 kohm + diode capacitance.",
    "Must be validated (V-15): diode VF/leakage, ADC accuracy, chain-rail variation.",
];

/// Port connect / disconnect sequence. Each step is a separate latch, so ordering never relies on
/// gate delays. Timings from TI datasheets: TPS2553 t_on/t_off 3 ms (CL 1 uF, RL 100 ohm);
/// TMUX136 enable 100 us, disable 200 ns.
pub const CONNECT_SEQUENCE: &[&str] = &[
    "0. Precondition: whole image zero except the target patch; main-board sensor-bus pull-ups disabled (port-side pull-ups used).",
    "1. ARM: Q7 + address (Q4:Q3) -> latch -> ADC: exactly one patch armed? else FAULT (OE_N high).",
    "2. POWER: + Q6 -> latch -> wait max(TPS2553 t_on 3 ms x margin, module power-up time from profile, e.g. SCD4x 30 ms) -> ADC sense re-check.",
    "3. CONNECT: + Q5 -> latch -> wait TMUX136 enable 100 us -> I2C traffic.",
];
/// Disconnect sequence (signals first, then power).
pub const DISCONNECT_SEQUENCE: &[&str] = &[
    "1. ISOLATE: clear Q5 -> latch (TMUX136 disables in 200 ns).",
    "2. UNPOWER: clear Q6 -> latch -> wait TPS2553 t_off + output discharge (QOD variant / bleed, value per P2).",
    "3. DISARM: clear Q7 -> latch -> ADC: zero armed.",
    "Emergency (fault, ESP32 reset): PATCH_OE_N high clears Q5 and Q6 together; the bus isolator (200 ns) still opens orders of magnitude before the load switch output decays (ms), so signals are isolated before supply removal.",
];

/// Pull-up and level rules (see calculations for numbers).
pub const PULL_UP_RULES: &[(&str, Status)] = &[
    ("Pull-ups for AUX mode sit on each port, referenced to that port's switched VCC (2.2 kohm proposal), so the bus high level never exceeds module VDD (SCD4x abs max SDA/SCL = VDD + 0.3 V).", Status::Proposal),
    ("Main-board sensor-bus pull-ups are switched OFF in AUX mode (PRP-MB-02).", Status::Proposal),
    ("Effective pull-up (port || module on-board) must lie between R_min = 967 ohm (3 mA at 0.4 V) and R_max from rise-time limit; modules with stronger on-board pull-ups are rejected in their profile.", Status::Proposal),
    ("Port VCC must stay >= 2.9 V (design criterion) so the high level meets ESP32-S3 VIH = 0.75 x VDD (2.475 V) with margin and module VDD minimums are met.", Status::Proposal),
];

/// Module identification policy.
pub const IDENTIFICATION_POLICY: &[&str] = &[
    "An address scan only yields candidate addresses. Different devices share addresses (e.g. SCD40/41/43 all at 0x62), so a scan alone does NOT identify a module.",
    "Identification = address match + profile-specific identity read with CRC check (e.g. SCD4x get_sensor_variant 0x202F / serial number), plus plausibility of the first reading.",
    "Probe only addresses listed in approved profiles (no blind write probing of the full 0x08-0x77 range); a generic scan is a diagnostic mode only.",
    "UNKNOWN module (no profile match, identity mismatch, NACK, or extra unexpected addresses): ISOLATE + UNPOWER immediately, mark port UNKNOWN with address list in the log/display, do not retry until user action or next maintenance scan.",
    "Address conflicts: none between modules (one active path); none with infrastructure (no I2C mux IC on the bus; primary SHT45 isolated by interlock). Multi-device breakouts must have no internal conflict - checked in the profile.",
];
