//! # 9. Firmware architecture
//!
//! Architecture only. **No firmware is implemented.** The state machine is from p.11. Data
//! destination, sampling and audio parameters are open (APR-4).

/// Top-level scan state machine (p.11).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanState {
    /// Hold PATCH_OE_N high; all outputs disabled; verify chain-control GPIO levels.
    SafeBoot,
    /// Shift a known all-zero image through all registers and latch; verify bit order with
    /// walking-one / walking-zero.
    ChainInit,
    /// Identify one patch position at a time via its ID branch; classify primary type.
    PatchDiscovery,
    /// Select AUX1..AUX4 individually (power, then signal); identify by I2C address + device ID;
    /// log unknown or absent modules and power the port down.
    AuxDiscovery,
    /// Select profile (primary I2C, I2S, or AUX I2C); set pull-ups; power on.
    Configure,
    /// Wait module-specific warm-up; collect data + timestamps; plausibility and fault checks.
    Acquire,
    /// Disconnect signals first, then disable power, discharge, then advance.
    Isolate,
    /// Leave failing path off; record patch / port / reason; continue only if safe.
    Fault,
}

/// Nominal order of states in a scan cycle.
pub const SCAN_SEQUENCE: [ScanState; 7] = [
    ScanState::SafeBoot,
    ScanState::ChainInit,
    ScanState::PatchDiscovery,
    ScanState::AuxDiscovery,
    ScanState::Configure,
    ScanState::Acquire,
    ScanState::Isolate,
];

/// Firmware software components (architecture-level).
pub const COMPONENTS: &[(&str, &str)] = &[
    ("chain_driver", "Builds and shifts the 64-bit control image; latch; OE control; enforces one active path system-wide."),
    ("bus_mode_manager", "Reconfigures SENSOR_CLK/SYNC/DATA_ID between ADC, I2C and I2S; switchable pull-ups; guarantees exclusive modes."),
    ("id_classifier", "ADC1 window classification for primary patch ID; I2C address/device-ID matching for AUX."),
    ("i2c_recovery", "Stuck-bus detection, clock-pulse recovery, AUX power-off on persistent fault."),
    ("primary_drivers", "SHT45-class I2C driver; I2S capture driver."),
    ("aux_profiles", "Per approved module MPN: interface, supply, warm-up, read command, conversion, limits."),
    ("scheduler", "Sampling plan and one-path-at-a-time arbitration (parameters per APR-4)."),
    ("fault_log", "Records patch position / port / reason / timestamp."),
    ("storage_comms", "Data destination - open (APR-4)."),
    ("display_status", "Separate I2C display bus."),
];

/// A per-module AUX firmware profile (structure only; populated after phase P1).
#[derive(Debug, Clone, Copy)]
pub struct AuxProfile {
    /// Module MPN.
    pub mpn: &'static str,
    /// Breakout board description (the approval unit is the breakout, not only the sensor IC).
    pub breakout: &'static str,
    /// 7-bit I2C addresses the module may answer on.
    pub i2c_addresses: &'static [u8],
    /// Supply millivolts (3300 under the recommended policy).
    pub supply_mv: u16,
    /// Peak current, mA (datasheet).
    pub peak_ma: u16,
    /// Warm-up before valid reading, milliseconds.
    pub warm_up_ms: u32,
    /// Whether the module must stay powered between reads (true = not supportable on AUX).
    pub requires_continuous_power: bool,
    /// Identity check beyond the address (command + expected response + CRC).
    pub identity_check: &'static str,
    /// Effective on-board pull-up (ohm), from the breakout schematic; 0 = none.
    pub onboard_pullup_ohm: u32,
    /// Highest patch position (1..=8) where the worst-case supply drop is acceptable.
    pub max_patch_position: u8,
}

/// Approved AUX profiles. **Empty**: no module is approved yet.
pub const APPROVED_AUX_PROFILES: &[AuxProfile] = &[];

/// Stage of an AUX port activation (separate latches, see `aux_expansion::CONNECT_SEQUENCE`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuxStage {
    /// Q7 + address: sense only.
    Armed,
    /// + Q6: port powered, bus isolated.
    Powered,
    /// + Q5: port connected to the chain I²C lines.
    Connected,
}

/// Primary-sensor activity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryStage {
    /// Q2: ID resistor branch.
    Identify,
    /// Q0: sensor powered, signals isolated.
    Powered,
    /// Q0 + Q1: sensor powered and connected.
    Connected,
}

/// The **only** way firmware describes chain activity. One value = at most one active path
/// system-wide. This makes "two patches active" unrepresentable in firmware.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePath {
    /// Everything off.
    Idle,
    /// Primary sensor on `patch` (0..=7).
    Primary {
        /// Patch index 0..=7 (0 = nearest the main board).
        patch: u8,
        /// Stage.
        stage: PrimaryStage,
    },
    /// AUX `port` (0..=3) on `patch` (0..=7).
    Aux {
        /// Patch index 0..=7.
        patch: u8,
        /// Port index 0..=3 (J_AUX1 = 0).
        port: u8,
        /// Stage.
        stage: AuxStage,
    },
}

impl ActivePath {
    /// Build the per-patch register bytes (index = patch position). Out-of-range indices yield
    /// the all-zero (safe) image.
    pub const fn image(self) -> [u8; 8] {
        let mut img = [0u8; 8];
        match self {
            ActivePath::Idle => {}
            ActivePath::Primary { patch, stage } => {
                if patch < 8 {
                    img[patch as usize] = match stage {
                        PrimaryStage::Identify => 0b0000_0100,
                        PrimaryStage::Powered => 0b0000_0001,
                        PrimaryStage::Connected => 0b0000_0011,
                    };
                }
            }
            ActivePath::Aux { patch, port, stage } => {
                if patch < 8 && port < 4 {
                    let base = 0b1000_0000 | (port << 3);
                    img[patch as usize] = match stage {
                        AuxStage::Armed => base,
                        AuxStage::Powered => base | 0b0100_0000,
                        AuxStage::Connected => base | 0b0110_0000,
                    };
                }
            }
        }
        img
    }
}
