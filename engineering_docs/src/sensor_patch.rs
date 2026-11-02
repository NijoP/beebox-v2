//! # 4. Sensor-patch architecture
//!
//! A patch is a passive controlled node. It has no MCU and no battery (client-confirmed).
//! The main controller addresses it through the shift-register cascade and selects one
//! local path at a time (p.8, Fig. 2).
//!
//! ## Patch identification (preserved, p.11)
//! The ID resistor `R_ID` runs from +3V3_LOCAL (rev D; was +3V3_CHAIN) to SENSOR_DATA_ID and is switched in by Q2.
//! With a locally regulated reference, ID windows must include the TPS62840 output tolerance (to verify).
//! It forms a divider with a 100 kΩ pull-down on the main board:
//! `V_ID = V_CHAIN * R_PD / (R_ID + R_PD)`. See [`nominal_id_millivolts`] and [`ID_CODES`].

use crate::model::{Source, Status};

/// Number of 74HC595 output bits per register.
pub const BITS_PER_REGISTER: usize = 8;
/// Baseline control image length (8 patches x 1 register x 8 bits), p.5.
pub const BASELINE_CONTROL_IMAGE_BITS: usize = crate::purpose::MAX_PATCHES * BITS_PER_REGISTER;

/// One output bit of the patch shift register.
#[derive(Debug, Clone, Copy)]
pub struct RegisterBit {
    /// Output name (Q0..Q7).
    pub output: &'static str,
    /// Proposed net name.
    pub net: &'static str,
    /// Function.
    pub function: &'static str,
    /// Allocation status.
    pub status: Status,
}

/// Register allocation. Q0-Q2 are baseline (p.8). Q3-Q7 are the proposed AUX allocation
/// ([`crate::aux_expansion::Q_MAP`], status Proposal).
pub const REGISTER_MAP: [RegisterBit; 8] = [
    RegisterBit {
        output: "Q0",
        net: "PRI_PWR_EN",
        function: "Primary sensor load-switch enable",
        status: Status::SourceBaseline,
    },
    RegisterBit {
        output: "Q1",
        net: "PRI_SIG_EN",
        function: "Primary sensor signal-isolation enable (3 lines)",
        status: Status::SourceBaseline,
    },
    RegisterBit {
        output: "Q2",
        net: "ID_EN",
        function: "Patch ID resistor branch enable",
        status: Status::SourceBaseline,
    },
    RegisterBit {
        output: "Q3",
        net: "AUX_A0",
        function: "AUX port address bit 0",
        status: Status::Proposal,
    },
    RegisterBit {
        output: "Q4",
        net: "AUX_A1",
        function: "AUX port address bit 1",
        status: Status::Proposal,
    },
    RegisterBit {
        output: "Q5",
        net: "AUX_SIG_EN",
        function: "TMUX1109 EN; CD74HC138 G2 (bus connect stage)",
        status: Status::Proposal,
    },
    RegisterBit {
        output: "Q6",
        net: "AUX_PWR_EN",
        function: "CD74HC238 G2 (power stage)",
        status: Status::Proposal,
    },
    RegisterBit {
        output: "Q7",
        net: "AUX_ARM",
        function: "Decoder A2 (arming) + AUX-armed sense to pin 7",
        status: Status::Proposal,
    },
];

/// Main-board ID pull-down, ohms (p.11).
pub const ID_PULLDOWN_OHMS: u32 = 100_000;
/// Nominal chain rail used in the ID calculation, millivolts.
pub const V_CHAIN_NOMINAL_MV: u32 = 3_300;

/// Nominal ID voltage in millivolts (rounded down) for a given `r_id` and pull-down.
///
/// ```
/// use beebox_v2_engineering::sensor_patch::*;
/// assert_eq!(nominal_id_millivolts(22_000, ID_PULLDOWN_OHMS, V_CHAIN_NOMINAL_MV), 2704);
/// ```
pub const fn nominal_id_millivolts(r_id_ohms: u32, r_pd_ohms: u32, v_chain_mv: u32) -> u32 {
    ((v_chain_mv as u64 * r_pd_ohms as u64) / (r_id_ohms as u64 + r_pd_ohms as u64)) as u32
}

/// ID classification code.
#[derive(Debug, Clone, Copy)]
pub struct IdCode {
    /// Population.
    pub population: &'static str,
    /// R_ID in ohms (`None` = open).
    pub r_id_ohms: Option<u32>,
    /// Screening window, millivolts (inclusive).
    pub window_mv: (u32, u32),
}

/// Provisional screening windows (p.11). They need tolerance analysis and ADC calibration.
/// They are **not** a codebook for AUX modules.
pub const ID_CODES: &[IdCode] = &[
    IdCode {
        population: "Empty / open",
        r_id_ohms: None,
        window_mv: (0, 250),
    },
    IdCode {
        population: "I2S microphone",
        r_id_ohms: Some(68_000),
        window_mv: (1_700, 2_200),
    },
    IdCode {
        population: "T/RH (SHT45-class)",
        r_id_ohms: Some(22_000),
        window_mv: (2_500, 2_950),
    },
];

/// Patch circuit blocks (p.8).
pub const BLOCKS: &[(&str, &str, Status)] = &[
    ("FPC IN / OUT", "Two 12-position connectors; all nets pass through except pin 8 cascade.", Status::SourceBaseline),
    ("Shift register", "One SN74HC595-class register (cascade preserved). No second register needed for I2C-only AUX.", Status::SourceBaseline),
    ("Primary isolation", "3 sensor lines + 1 ID branch through a CD74HC4066-class quad switch (exactly 4 channels).", Status::Candidate),
    ("Primary power", "Load switch from +3V3_CHAIN, default OFF, defined discharge (TPS22918-class candidate).", Status::Candidate),
    ("Primary sensor", "Exactly one of SHT45-class T/RH or I2S mic populated; other branch DNI/isolated.", Status::ClientConfirmed),
    ("AUX expansion", "Four 4-pin I2C headers J_AUX1..J_AUX4 (VCC, GND, SDA, SCL) - see crate::aux_expansion.", Status::ClientConfirmed),
    ("Test / assembly", "Test points, DNI options, variant notes, configuration labels (sheet P05).", Status::SourceBaseline),
];

/// Planned patch schematic sheets (p.14).
pub const SHEETS: &[(&str, &str)] = &[
    ("P01 FPC IN / OUT", "Two connectors, pin-8 directional cascade, pass-through nets."),
    ("P02 Register + power", "74HC595 control, pull-downs on all enables, local load switch, decoupling."),
    ("P03 Primary sensor / ID", "4066 isolation, ID resistor branch, T/RH / mic population variants."),
    ("P04 AUX1-AUX4", "Four 4-pin I2C headers, TMUX136 isolator, TMUX1109 mux, CD74HC238/138 decoders, 4 x TPS2553, port pull-ups, arm-sense, ESD, labels."),
    ("P05 Test / assembly", "Test points, DNI options, variant notes, configuration and manufacturing labels."),
];

/// Fail-safe implementation notes (analysis; must be reflected in P02).
pub const FAIL_SAFE_NOTES: &[(&str, Source)] = &[
    ("With PATCH_OE_N high the 595 outputs are high-impedance, not low; every enable net needs a pull resistor to its OFF level.", Source::Analysis),
    ("SRCLR has no chain contact; tie it inactive locally. Register contents are random at power-up until CHAIN_INIT latches zeros, so OE_N must remain high until then.", Source::Analysis),
    ("ISOLATE order: disconnect signals first, then remove power, then discharge (p.11).", Source::Pdf { page: 11 }),
];
