//! # 10. Communication and data flow
//!
//! All patch data reaches the ESP32-S3 over the three time-multiplexed chain lines (pins 4, 5, 7).
//! Exactly one [`BusMode`] is active at a time. The display uses a separate I2C bus.
//!
//! ```text
//! sensor/module -> patch switch (selected) -> chain pins 4/5/7 -> ESP32-S3 peripheral
//!   -> driver -> sample record (patch, port, type, value, timestamp, status) -> destination (APR-4)
//! ```

/// Mode of the shared chain lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusMode {
    /// Nothing selected; all patch switches open.
    Idle,
    /// Patch or AUX identification: DATA_ID as ADC1 input.
    IdentifyAdc,
    /// Primary T/RH over I2C on CLK/SYNC.
    PrimaryI2c,
    /// Primary microphone: BCLK/WS out, SD in on DATA_ID.
    PrimaryI2s,
    /// AUX I2C module on CLK (SCL) / SYNC (SDA) - the only AUX mode (client: I2C-only).
    AuxI2c,
}

/// Use of (pin 4, pin 5, pin 7) per mode. `None` = not driven / high-Z.
pub const LINE_USAGE: &[(BusMode, Option<&str>, Option<&str>, Option<&str>)] = &[
    (BusMode::Idle, None, None, None),
    (
        BusMode::IdentifyAdc,
        None,
        None,
        Some("ADC1 in (R_ID vs 100k)"),
    ),
    (BusMode::PrimaryI2c, Some("SCL"), Some("SDA"), None),
    (
        BusMode::PrimaryI2s,
        Some("BCLK out"),
        Some("WS out"),
        Some("SD in"),
    ),
    (BusMode::AuxI2c, Some("SCL"), Some("SDA"), None),
];

/// Logical sample record (structure proposal; storage format open).
#[derive(Debug, Clone, Copy)]
pub struct SampleRecord {
    /// Patch position 1..=8.
    pub patch: u8,
    /// AUX port 1..=4, or 0 for primary sensor.
    pub port: u8,
    /// Measurement kind.
    pub kind: &'static str,
    /// Value in SI units.
    pub value: f64,
    /// Timestamp (source of time open: RTC/NTP - APR-4).
    pub timestamp_ms: u64,
    /// Plausibility / fault status.
    pub valid: bool,
}

/// Open data-flow items (APR-4).
pub const OPEN_ITEMS: &[&str] = &[
    "Data destination: display only, local storage, Wi-Fi, BLE, or combination.",
    "Sampling interval per sensor class; audio sample rate, bit depth and clip length.",
    "Time source for timestamps.",
];
