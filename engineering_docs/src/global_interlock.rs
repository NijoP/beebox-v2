//! # Global AUX interlock — hardware proof (mandatory item A) and pin-7 resolution (item B)
//!
//! ## Problem
//! Rev C enforces "one port" only *within* a patch (shared 2-bit address). Across the eight patches it
//! relied on firmware plus an ESP32-read "AUX-armed" sense injected onto **pin 7**, which also carries
//! the patch ID voltage and I²S SD. A firmware fault could power/connect ports on two patches, and the
//! pin-7 sharing could be corrupted by a microphone driving SD (a strong push-pull output), so it is
//! **not** a hardware proof.
//!
//! ## Proposed solution (requires approval A-01: pin 12 GND → AUX_ARM)
//! * Dedicated analog line **AUX_ARM on pin 12**. Pin 7 returns to ID + I²S SD only (item B solved:
//!   no microphone or ID function is sacrificed or time-shared).
//! * Main board: R_PD 100 kΩ 0.1 % from AUX_ARM to GND, monitored by ESP32 ADC1 (diagnostic only).
//! * Each patch: `+3V3_LOCAL → TMUX1101 (SEL = Q7) → R_ARM 150 kΩ 0.1 % → AUX_ARM`. An armed patch
//!   sources a ratiometric current; an unarmed patch is a ≤ 0.3 nA leakage (TMUX1101, 85 °C).
//! * Each patch: window comparator TLV7032 on the RC-filtered line (10 kΩ / 10 nF) against a local
//!   ladder 187 k / 75.0 k / 69.8 k (0.1 %) from `+3V3_LOCAL`: `V_LOW ≈ 0.21·V`, `V_HIGH ≈ 0.44·V`.
//!   `WIN_BAD = NAND(line > V_LOW, line < V_HIGH)` (SN74LVC1G00).
//! * Each patch: `Q7D = Schmitt(RC(Q7))` (100 kΩ / 22 nF + SN74LVC1G17), ≥ 1.14 ms delayed arm.
//! * Two SN74LVC138A replace CD74HC238/CD74HC138 (not 3.3 V-specified):
//!   power decoder `A=Q3, B=Q4, C=Q7D, G1=Q6, G2A=PRI(Q0∨Q1), G2B=WIN_BAD` → `/Y4../Y7` → TPS2552DBVR-1 `EN`
//!   (active low); bus decoder `A=GND, B=Q6, C=Q7D, G1=Q5, G2A=PRI, G2B=WIN_BAD` → `/Y6` → TMUX136 `EN`.
//!   TMUX1109 `EN=Q5`, `A1:A0=Q4:Q3` (behind the isolator). `PRI = Q0 ∨ Q1` via SN74LVC1G32.
//!
//! ## Proof (hardware, independent of firmware)
//! 1. *Electrical lemma* (tests `tests::window_lemma_all_corners`): over all tolerance corners
//!    (rails ±1.5 %, 0.1 % resistors + drift, comparator offset 8 mV + hysteresis 17 mV, ground
//!    offsets 0…0.42 V on every patch, leakage), an **armed** patch reads `WIN_OK` **iff exactly one**
//!    patch in the system is armed; with zero armed it never reads `WIN_OK`.
//! 2. *Patch lemma* (test `tests::patch_logic_exhaustive`): a port is powered or connected only if
//!    its own `Q7D = 1` and `WIN_OK = 1`, and the decoder makes the powered port unique and equal to the
//!    mux address; the bus is connected only when that same port is powered.
//! 3. *Theorem*: if ports on two different patches were powered/connected, both patches would be armed
//!    (`Q7D = 1 ⇒ Q7 = 1`), so n ≥ 2, so neither reads `WIN_OK` (lemma 1) — contradiction. Hence at
//!    most one of the 32 ports is powered and at most one is connected (test
//!    `tests::system_never_two_ports`).
//! 4. *Transients* (test `tests::arm_delay_exceeds_window_settling`): a second patch that arms while
//!    a port is connected cannot use a stale `WIN_OK`, because its `Q7D` rises ≥ 1.14 ms after `Q7`,
//!    while the line and filter settle in ≤ 0.45 ms; the first patch drops its port within ≈ 0.45 ms.
//! 5. *Single faults*: a stuck-OK comparator on one patch cannot create a second connection (the
//!    other patch still reads n = 2); a broken AUX_ARM line or missing main pull-down drives the line
//!    to ≈ V_LOCAL (> V_HIGH) or 0 V (< V_LOW) — both disable AUX.
//!
//! Fallback if A-01 is refused: keep the arm sense on pin 7 (rev C). Then cross-patch exclusion is a
//! firmware rule with ADC verification only, **not** a hardware guarantee (see
//! [`PIN7_FALLBACK_LIMITATION`]).

/// Arm resistor (Ω).
pub const R_ARM: f64 = 150_000.0;
/// Main-board pull-down on AUX_ARM (Ω).
pub const R_PD: f64 = 100_000.0;
/// Precision resistor tolerance (0.1 %) + drift (0.2 %).
pub const R_TOL_PREC: f64 = 0.003;
/// Threshold ladder top / middle / bottom (Ω).
pub const LADDER: (f64, f64, f64) = (187_000.0, 75_000.0, 69_800.0);
/// Patch 3.3 V rail tolerance (TPS62840 ±1.5 %).
pub const RAIL_TOL: f64 = crate::chain_power_5v::TPS62840_ACC;
/// Comparator error: TLV7032 VIO ±8 mV + hysteresis ≤ 17 mV.
pub const COMP_ERR: f64 = 0.008 + 0.017;
/// Maximum patch ground offset vs. main ground (V): 0.394 A × 1.04 Ω (3 grounds, patch 8).
pub const GROUND_OFFSET_MAX: f64 = 0.42;
/// Line leakage (A): 8 × TMUX1101 0.3 nA + ESP32 pin 50 nA + ESD 10 nA.
pub const LINE_LEAK: f64 = 8.0 * 0.3e-9 + 50e-9 + 10e-9;

/// Window thresholds (V_LOW, V_HIGH) for local rail `v` and ladder tolerance corner `t` (−1/+1 per resistor).
pub fn thresholds(v: f64, t: (f64, f64, f64)) -> (f64, f64) {
    let a = LADDER.0 * (1.0 + t.0 * R_TOL_PREC);
    let b = LADDER.1 * (1.0 + t.1 * R_TOL_PREC);
    let c = LADDER.2 * (1.0 + t.2 * R_TOL_PREC);
    let s = a + b + c;
    (v * c / s, v * (b + c) / s)
}

/// Line voltage seen by an observer patch.
///
/// `sources`: (rail V, ground offset V) of every armed patch; `r_arm`, `r_pd` actual values;
/// `g_obs`: observer ground offset. Unarmed patches contribute leakage only.
pub fn seen(sources: &[(f64, f64)], r_arm: f64, r_pd: f64, g_obs: f64) -> f64 {
    let g_arm = 1.0 / r_arm;
    let g_pd = 1.0 / r_pd;
    let num: f64 = sources.iter().map(|(v, g)| (v + g) * g_arm).sum::<f64>() + LINE_LEAK;
    num / (g_pd + sources.len() as f64 * g_arm) - g_obs
}

/// Window result with comparator error applied in the unfavourable direction for `want_ok`.
pub fn window_ok(v_seen: f64, thr: (f64, f64), favour_ok: bool) -> bool {
    let e = if favour_ok { COMP_ERR } else { -COMP_ERR };
    v_seen > thr.0 - e && v_seen < thr.1 + e
}

/// Per-patch control inputs (595 outputs after the OE gate) plus the window result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PatchCtl {
    /// Q0 primary power.
    pub q0: bool,
    /// Q1 primary signal.
    pub q1: bool,
    /// Q3 address bit 0.
    pub q3: bool,
    /// Q4 address bit 1.
    pub q4: bool,
    /// Q5 bus connect.
    pub q5: bool,
    /// Q6 port power.
    pub q6: bool,
    /// Q7 arm (TMUX1101 SEL).
    pub q7: bool,
    /// Q7D delayed arm (decoder C inputs). Steady state: equals Q7.
    pub q7d: bool,
    /// Local BLOCK (startup latch / supervisor) forcing all 595 outputs Hi-Z → 0.
    pub block: bool,
}

/// Patch outputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PatchOut {
    /// Powered port 0..3 (TPS2552 EN low), if any.
    pub port_powered: Option<u8>,
    /// TMUX136 closed (bus connected to chain pins 4/5).
    pub bus_connected: bool,
    /// TMUX1109 selected port behind the isolator.
    pub mux_port: Option<u8>,
    /// TMUX1101 sourcing arm current.
    pub arming: bool,
}

/// Gate-level model of one patch (SN74LVC138A function table: enabled when G1 = H, G2A = G2B = L).
pub fn patch_logic(c: PatchCtl, win_ok: bool) -> PatchOut {
    let c = if c.block {
        PatchCtl {
            q0: false,
            q1: false,
            q3: false,
            q4: false,
            q5: false,
            q6: false,
            q7: false,
            q7d: false,
            block: true,
        }
    } else {
        c
    };
    let pri = c.q0 || c.q1; // SN74LVC1G32
    let win_bad = !win_ok; // TLV7032 + SN74LVC1G00
    let addr = (c.q3 as u8) | ((c.q4 as u8) << 1);
    let pwr_en = c.q6 && !pri && !win_bad;
    let port_powered = if pwr_en && c.q7d { Some(addr) } else { None }; // C = 1 → Y4..Y7
    let bus_en = c.q5 && !pri && !win_bad;
    let bus_connected = bus_en && c.q7d && c.q6; // /Y6: C=1, B=1, A=0
    PatchOut {
        port_powered,
        bus_connected,
        mux_port: if c.q5 { Some(addr) } else { None },
        arming: c.q7,
    }
}

/// Arm delay Q7 → Q7D (ms): RC 100 kΩ ±1.5 %, 22 nF ±10 %, SN74LVC1G17 VT+ 1.48–1.92 V (VCC 3 V table;
/// max extrapolated to 2.11 V at 3.35 V). Returns (min, max).
pub fn q7d_delay_ms() -> (f64, f64) {
    let vmax = 3.3 * (1.0 + RAIL_TOL);
    let vmin = 3.3 * (1.0 - RAIL_TOL);
    let tmin = 100e3 * 0.985 * 22e-9 * 0.9 * (vmax / (vmax - 1.48)).ln();
    let tmax = 100e3 * 1.015 * 22e-9 * 1.1 * (vmin / (vmin - 2.11)).ln();
    (tmin * 1e3, tmax * 1e3)
}

/// Worst-case window settling (ms): line τ (R_ARM‖R_PD × 150 pF) + filter τ (10.15 kΩ × 11 nF), to 3.5 τ,
/// + TLV7032 3 µs.
pub fn window_settle_ms() -> f64 {
    let r_th = 1.0 / (1.0 / (R_ARM * 1.003) + 1.0 / (R_PD * 1.003));
    let tau_line = r_th * 150e-12;
    let tau_f = 10e3 * 1.015 * 10e-9 * 1.1;
    (3.5 * (tau_line + tau_f) + 3e-6) * 1e3
}

/// Limitation if the pin-12 change is refused.
pub const PIN7_FALLBACK_LIMITATION: &str = "With the arm sense on pin 7, a microphone on another patch (push-pull SD) or an enabled ID resistor can pull the shared line into the 'one armed' window, so two armed patches could both read WIN_OK. Cross-patch exclusion then depends on firmware (one ActivePath + ADC verification) and is not a hardware proof.";

#[cfg(test)]
mod tests {
    use super::*;

    fn corners3() -> Vec<(f64, f64, f64)> {
        let s = [-1.0, 1.0];
        let mut v = vec![];
        for a in s {
            for b in s {
                for c in s {
                    v.push((a, b, c));
                }
            }
        }
        v
    }

    #[test]
    fn window_lemma_all_corners() {
        let vs = [3.3 * (1.0 - RAIL_TOL), 3.3 * (1.0 + RAIL_TOL)];
        let ra = [R_ARM * (1.0 - R_TOL_PREC), R_ARM * (1.0 + R_TOL_PREC)];
        let rp = [R_PD * (1.0 - R_TOL_PREC), R_PD * (1.0 + R_TOL_PREC)];
        let gs = [0.0, GROUND_OFFSET_MAX];
        let mut min_margin = f64::MAX;
        for n in 0..=8usize {
            for &v_obs in &vs {
                for &v_oth in &vs {
                    for &r_a in &ra {
                        for &r_p in &rp {
                            for &g_obs in &gs {
                                for &g_oth in &gs {
                                    for t in corners3() {
                                        let thr = thresholds(v_obs, t);
                                        let mut src = vec![];
                                        if n >= 1 {
                                            src.push((v_obs, g_obs));
                                        } // observer is armed when n ≥ 1
                                        for _ in 1..n {
                                            src.push((v_oth, g_oth));
                                        }
                                        let s = seen(&src, r_a, r_p, g_obs);
                                        if n == 1 {
                                            assert!(
                                                window_ok(s, thr, false),
                                                "availability n=1: {s} {thr:?}"
                                            );
                                            min_margin = min_margin.min((s - thr.0).min(thr.1 - s));
                                        } else {
                                            assert!(
                                                !window_ok(s, thr, true),
                                                "safety n={n}: seen {s} thr {thr:?}"
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        assert!(min_margin > COMP_ERR, "{min_margin}");
    }

    #[test]
    fn patch_logic_exhaustive() {
        for bits in 0u16..1024 {
            let b = |i: u16| bits & (1 << i) != 0;
            let c = PatchCtl {
                q0: b(0),
                q1: b(1),
                q3: b(2),
                q4: b(3),
                q5: b(4),
                q6: b(5),
                q7: b(6),
                q7d: b(6) && b(7),
                block: b(8),
            };
            let win = b(9);
            let o = patch_logic(c, win);
            if let Some(p) = o.port_powered {
                assert!(win && c.q7d && c.q7 && c.q6 && !c.q0 && !c.q1 && !c.block);
                assert_eq!(p, (c.q3 as u8) | ((c.q4 as u8) << 1));
            }
            if o.bus_connected {
                assert!(
                    o.port_powered.is_some() && o.port_powered == o.mux_port,
                    "{c:?}"
                );
            }
            if c.block {
                assert!(o.port_powered.is_none() && !o.bus_connected && !o.arming);
            }
            if !win {
                assert!(o.port_powered.is_none() && !o.bus_connected);
            }
        }
    }

    #[test]
    fn system_never_two_ports() {
        // 8 patches; per-patch control bits from a deterministic pseudo-random sequence plus all
        // 256 arm subsets. WIN_OK follows the proven lemma: true iff exactly one patch is armed.
        let mut seed: u64 = 0x2545F4914F6CDD1D;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        for arm_mask in 0u16..256 {
            for _ in 0..200 {
                let n = arm_mask.count_ones();
                let mut powered = 0;
                let mut connected = 0;
                for p in 0..8 {
                    let r = next();
                    let q7 = arm_mask & (1 << p) != 0;
                    let c = PatchCtl {
                        q0: r & 1 != 0 && r & 64 == 0,
                        q1: r & 2 != 0 && r & 128 == 0,
                        q3: r & 4 != 0,
                        q4: r & 8 != 0,
                        q5: r & 16 != 0,
                        q6: r & 32 != 0,
                        q7,
                        q7d: q7,
                        block: false,
                    };
                    let o = patch_logic(c, n == 1);
                    powered += o.port_powered.is_some() as u32;
                    connected += o.bus_connected as u32;
                }
                assert!(powered <= 1 && connected <= 1, "mask {arm_mask:08b}");
            }
        }
    }

    #[test]
    fn arm_delay_exceeds_window_settling() {
        let (dmin, dmax) = q7d_delay_ms();
        let settle = window_settle_ms();
        assert!(dmin > 2.0 * settle, "Q7D {dmin} ms vs settle {settle} ms");
        assert!(dmax < 3.0, "{dmax}");
    }

    #[test]
    fn ladder_values() {
        let (l, h) = thresholds(3.3, (0.0, 0.0, 0.0));
        assert!(
            (l - 0.694).abs() < 0.002 && (h - 1.440).abs() < 0.002,
            "{l} {h}"
        );
        // Unarmed system: line is at leakage level only.
        assert!(seen(&[], R_ARM, R_PD, 0.0) < 0.01);
        // Broken line (no main pull-down): one armed patch drives the line to its rail > V_HIGH.
        assert!(3.3 * (1.0 - RAIL_TOL) > thresholds(3.3 * (1.0 + RAIL_TOL), (1.0, 1.0, 1.0)).1);
    }
}
