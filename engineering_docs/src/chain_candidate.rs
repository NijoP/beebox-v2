//! # Final candidate 12-pin interface (release review 2026-11-02)
//!
//! Pin positions 1, 3–11 are unchanged from the source ICD (p.7). Two changes are **proposed, not
//! frozen** — they need client approval (A-01, A-02 in [`crate::release_review`]):
//!
//! | Pin | Source / rev C | Candidate | Status |
//! |---|---|---|---|
//! | 2 | +3V3_CHAIN | **+5V_CHAIN** (4.90–5.21 V at source, ≤ 394 mA hardware limit) | Proposed (A-02) |
//! | 12 | GND | **AUX_ARM** (global AUX interlock line, 0–3.35 V analog, µA) | Proposed (A-01) |
//!
//! Effects: 3 ground contacts (1, 3, 6) instead of 4 (return resistance × 4/3, included in
//! [`crate::chain_power_5v::worst`]); pin 7 carries only SENSOR_DATA_ID / I²S SD again.
//! No signal pin is adjacent to the 5 V pin (pins 1 and 3 are GND).
//!
//! Mirror risk: a reversed FFC maps pin 2 → 11 (PATCH_OE_N). Patch side: SN74LVC1G32 input is 5.5 V
//! tolerant. Main side: 1 kΩ series on PATCH_OE_N + TPD4E05U06 clamp limit injection into the ESP32;
//! mechanical keying and a 1:1 continuity fixture remain mandatory (REQ-DP-02).

/// Pin status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinStatus {
    /// Confirmed by the source ICD and unchanged.
    Confirmed,
    /// Proposed change awaiting client approval.
    Proposed,
}

/// One candidate pin.
#[derive(Debug, Clone, Copy)]
pub struct CandidatePin {
    /// Contact number.
    pub pin: u8,
    /// Net.
    pub net: &'static str,
    /// Electrical definition.
    pub electrical: &'static str,
    /// Status.
    pub status: PinStatus,
}

/// Candidate pinout.
pub const PINS: [CandidatePin; 12] = [
    CandidatePin { pin: 1, net: "GND", electrical: "Return", status: PinStatus::Confirmed },
    CandidatePin { pin: 2, net: "+5V_CHAIN", electrical: "4.90-5.21 V at source (TPS61023 + TPS2553-1 76.8 k, 302-394 mA); ≥ 3.6 V at any patch input", status: PinStatus::Proposed },
    CandidatePin { pin: 3, net: "GND", electrical: "Return", status: PinStatus::Confirmed },
    CandidatePin { pin: 4, net: "SENSOR_CLK", electrical: "3.3 V: I²C SCL (open drain, pull-ups on the active side) / I²S BCLK 2.048 MHz", status: PinStatus::Confirmed },
    CandidatePin { pin: 5, net: "SENSOR_SYNC", electrical: "3.3 V: I²C SDA / I²S WS", status: PinStatus::Confirmed },
    CandidatePin { pin: 6, net: "GND", electrical: "Return / signal reference between pins 5 and 7", status: PinStatus::Confirmed },
    CandidatePin { pin: 7, net: "SENSOR_DATA_ID", electrical: "0-3.3 V: patch ID voltage (ADC1) or I²S SD; no other function", status: PinStatus::Confirmed },
    CandidatePin { pin: 8, net: "PATCH_SHIFT_DATA", electrical: "3.3 V, directional through each SN74LV595A (SER in, QH' out)", status: PinStatus::Confirmed },
    CandidatePin { pin: 9, net: "PATCH_SHIFT_CLK", electrical: "3.3 V, multi-drop", status: PinStatus::Confirmed },
    CandidatePin { pin: 10, net: "PATCH_LATCH", electrical: "3.3 V, multi-drop; also clears the local BLOCK latch", status: PinStatus::Confirmed },
    CandidatePin { pin: 11, net: "PATCH_OE_N", electrical: "3.3 V, active low; local 100 kΩ pull-up to +3V3_LOCAL on every patch", status: PinStatus::Confirmed },
    CandidatePin { pin: 12, net: "AUX_ARM", electrical: "Analog 0-3.35 V; 100 kΩ to GND on main; 150 kΩ switched source per armed patch", status: PinStatus::Proposed },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_pins_2_and_12_change() {
        for (i, p) in PINS.iter().enumerate() {
            assert_eq!(p.pin as usize, i + 1);
            let src = crate::interfaces::CHAIN_PINS[i].net;
            match p.pin {
                2 => assert_eq!(p.status, PinStatus::Proposed),
                12 => assert!(p.status == PinStatus::Proposed && src == "GND"),
                _ => assert!(
                    p.status == PinStatus::Confirmed && p.net == src,
                    "pin {}",
                    p.pin
                ),
            }
        }
        assert_eq!(PINS.iter().filter(|p| p.net == "GND").count(), 3);
    }

    #[test]
    fn five_volt_pin_has_ground_neighbours() {
        assert_eq!(PINS[0].net, "GND");
        assert_eq!(PINS[2].net, "GND");
    }
}
