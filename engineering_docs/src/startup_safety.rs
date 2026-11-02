//! # Safe start-up, reset, brownout and fault states (mandatory item C) and AUX power sequencing (item D)
//!
//! ## Patch hardware (per patch, all on `+3V3_LOCAL`)
//! * **TLV840CADL29DBVR** supervisor: VIT− 2.9 V ±2 %, hysteresis 2.5–7 % (release ≤ 3.17 V < 3.25 V
//!   rail min), open-drain active-low `RESET_N` with 100 kΩ pull-up to `+3V3_LOCAL`, CT open
//!   (minimum delay). Valid output from VDD ≥ 0.7 V (V_POR).
//! * **SN74LVC1G74** `BLOCK` latch: `PRE_N = RESET_N` (asynchronous set), `D = GND`,
//!   `CLK = PATCH_LATCH`, `CLR_N = +3V3_LOCAL`. After any local reset `BLOCK = 1` until the next
//!   RCLK edge, i.e. until the ESP32 has shifted and latched a fresh image.
//! * **SN74LVC1G32**: `OE_N(595) = PATCH_OE_N ∨ BLOCK`.
//! * `PATCH_OE_N` has a **local** 100 kΩ pull-up to `+3V3_LOCAL` on every patch — the pull-up and the
//!   595 it protects share one rail, so the pull-up can never be unpowered while the 595 is powered.
//!   No pull-up to a different (switchable) rail is used anywhere on the chain.
//! * `SRCLR_N(595) = RESET_N` (shift register cleared during reset).
//! * 47 kΩ pull-downs on Q0–Q7 (595 Hi-Z ⇒ 0 V ≤ 0.24 V at IOZ 5 µA).
//! * SN74LV595A, SN74LVC1G32/74/00/17, SN74LVC138A have Ioff; TMUX136 and TMUX1511 have power-off
//!   protection; so an unpowered patch is not back-powered by driven chain lines.
//!
//! ## Main board
//! * `CHAIN_PWR_EN` (TPS2553-1 EN) and `BOOST_EN` (TPS61023 EN) have 100 kΩ pull-downs: chain OFF
//!   whenever the ESP32 is in reset, brownout or unprogrammed.
//! * ESP32-S3 brownout detector resets the SoC; GPIOs go high-impedance ⇒ chain switch off.
//! * `PATCH_OE_N` series 1 kΩ (slow signal; limits injection if an FFC is inserted mirrored, which
//!   would put pin 2 (+5 V) on pin 11).

/// Events that must leave every patch output OFF.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    /// Chain switched on (local rail ramps 0 → 3.3 V).
    PowerUp,
    /// ESP32 in reset / brownout: chain-control GPIOs high-impedance.
    EspReset,
    /// Local rail dips below 2.9 V (overload at a far patch, contact interruption).
    LocalBrownout,
    /// Chain switched off by the main board.
    ChainOff,
    /// PATCH_OE_N conductor open upstream of this patch.
    OeLineOpen,
    /// PATCH_LATCH conductor open upstream of this patch.
    LatchLineOpen,
    /// Port overcurrent (TPS2552-1 latch-off).
    PortFault,
    /// Chain overcurrent (TPS2553-1 latch-off on the main board).
    ChainFault,
}

/// Simplified patch state used to check the event table.
#[derive(Debug, Clone, Copy)]
pub struct State {
    /// Local rail present and above the reset threshold.
    pub rail_ok: bool,
    /// RESET_N asserted.
    pub reset: bool,
    /// BLOCK latch.
    pub block: bool,
    /// Level of PATCH_OE_N at the patch.
    pub oe_n_bus: bool,
    /// 595 storage register content is known (latched by the ESP32 after the last reset).
    pub image_valid: bool,
}

/// 595 outputs enabled?
pub fn outputs_enabled(s: &State) -> bool {
    s.rail_ok && !s.reset && !(s.oe_n_bus || s.block)
}

/// State after an event, starting from a normal operating state (outputs enabled).
pub fn after(e: Event) -> State {
    let normal = State {
        rail_ok: true,
        reset: false,
        block: false,
        oe_n_bus: false,
        image_valid: true,
    };
    match e {
        Event::PowerUp | Event::LocalBrownout => State {
            reset: true,
            block: true,
            image_valid: false,
            ..normal
        },
        Event::EspReset => State {
            oe_n_bus: true,
            ..normal
        }, // GPIO Hi-Z → local pull-up
        Event::ChainOff | Event::ChainFault => State {
            rail_ok: false,
            reset: true,
            block: true,
            image_valid: false,
            ..normal
        },
        Event::OeLineOpen => State {
            oe_n_bus: true,
            ..normal
        }, // local pull-up wins
        Event::LatchLineOpen => normal, // outputs keep the last valid image; no new latch possible
        Event::PortFault => normal,     // port latched off by TPS2552-1; the rest unchanged
    }
}

/// After BLOCK is set, outputs re-enable only after an RCLK edge, which the firmware issues only after
/// shifting a complete image (CHAIN_INIT).
pub fn after_reset_then_latch() -> State {
    State {
        rail_ok: true,
        reset: false,
        block: false,
        oe_n_bus: false,
        image_valid: true,
    }
}

/// AUX port sequence with minimum waits taken from datasheets (ms).
#[derive(Debug, Clone, Copy)]
pub struct Step {
    /// Step name.
    pub name: &'static str,
    /// Action (latched as a separate RCLK cycle).
    pub action: &'static str,
    /// Minimum wait after the latch (ms).
    pub wait_ms: f64,
    /// Basis for the wait.
    pub basis: &'static str,
}

/// Connect / read / disconnect sequence for one port (all other patches: Q5 = Q6 = Q7 = 0).
pub const SEQUENCE: &[Step] = &[
    Step { name: "0 PRECONDITION", action: "All patches primary off (Q0 = Q1 = 0) for AUX; SENSOR_PU_EN Hi-Z; ADC on AUX_ARM < V_LOW (none armed)", wait_ms: 0.0, basis: "-" },
    Step { name: "1 ARM", action: "Set Q4:Q3 = port, Q7 = 1", wait_ms: 3.0, basis: "Q7D max 2.57 ms (global_interlock::q7d_delay_ms); ADC must read the n = 1 window" },
    Step { name: "2 POWER", action: "Set Q6 = 1", wait_ms: 3.0, basis: "TPS2552 ton 3 ms max (CL 1 µF, RL 100 Ω) + module start-up from its profile (e.g. SCD4x ≤ 30 ms) added by firmware" },
    Step { name: "3 CONNECT", action: "Set Q5 = 1", wait_ms: 0.2, basis: "TMUX136 enable (EN to output) 100 µs" },
    Step { name: "4 READ", action: "I²C 100 kHz transfers; none during module current peaks", wait_ms: 0.0, basis: "ground offset ≤ 0.42 V (chain_power_5v)" },
    Step { name: "5 DISCONNECT", action: "Clear Q5", wait_ms: 0.01, basis: "TMUX136 disable 200 ns" },
    Step { name: "6 UNPOWER", action: "Clear Q6", wait_ms: 485.0, basis: "TPS2552 toff 3 ms + 10 kΩ bleeder × (10 µF + ≤ 10 µF module) to < 0.3 V (2.4 τ)" },
    Step { name: "7 DISARM", action: "Clear Q7; ADC on AUX_ARM < V_LOW", wait_ms: 0.5, basis: "window settling 0.43 ms" },
];

/// Firmware-independent guarantees of the sequence (what happens if the firmware skips steps).
pub const SEQUENCE_GUARANTEES: &[&str] = &[
    "Q5/Q6/Q7 set in one latch: the port cannot power or connect until Q7D (≥ 1.14 ms) and WIN_OK are both true.",
    "Q5 without Q6: TMUX136 stays open (bus decoder needs B = Q6) — an unpowered module is never connected.",
    "Q6 cleared while Q5 set: the bus decoder disables TMUX136 in the same latch (B = Q6), so signals and power are removed together; TMUX136 opens in 200 ns, long before the port rail decays (ms).",
    "Two patches armed: neither reads WIN_OK; any connected port is dropped within ≈ 0.43 ms.",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_fault_event_disables_outputs_or_keeps_valid_image() {
        for e in [
            Event::PowerUp,
            Event::EspReset,
            Event::LocalBrownout,
            Event::ChainOff,
            Event::OeLineOpen,
            Event::ChainFault,
        ] {
            assert!(!outputs_enabled(&after(e)), "{e:?}");
        }
        for e in [Event::LatchLineOpen, Event::PortFault] {
            let s = after(e);
            assert!(!outputs_enabled(&s) || s.image_valid, "{e:?}");
        }
        assert!(outputs_enabled(&after_reset_then_latch()));
    }

    #[test]
    fn supervisor_thresholds_vs_rail() {
        let vit_minus_max = 2.9 * 1.02;
        let release_max = vit_minus_max * 1.07;
        let rail_min = 3.3 * (1.0 - crate::chain_power_5v::TPS62840_ACC);
        assert!(release_max < rail_min, "{release_max} vs {rail_min}");
        // Reset asserts before the LV595A leaves its characterised range (3.0-3.6 V) by much and
        // before TPS2552 UVLO (2.35 V) could leave a port in an undefined state.
        assert!(2.9 * 0.98 > 2.35);
    }

    #[test]
    fn sequence_waits_cover_datasheet_times() {
        let w = |n: &str| {
            SEQUENCE
                .iter()
                .find(|s| s.name.starts_with(n))
                .unwrap()
                .wait_ms
        };
        assert!(w("1") >= crate::global_interlock::q7d_delay_ms().1);
        assert!(w("2") >= 3.0);
        assert!(w("3") >= 0.1);
        assert!(w("5") >= 200e-6);
        let tau_ms = 10e3 * 20e-6 * 1e3;
        assert!(w("6") >= 3.0 + tau_ms * (3.3f64 / 0.3).ln() - 1.0);
        assert!(w("7") >= crate::global_interlock::window_settle_ms());
    }

    #[test]
    fn pulldowns_hold_off_through_hi_z() {
        assert!(5e-6 * 47_000.0 < 0.45);
    }
}
