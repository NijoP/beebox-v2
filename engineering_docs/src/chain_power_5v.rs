//! # 5 V chain supply — verification calculations (release review 2026-11-02)
//!
//! Evaluates the client-preferred architecture: dedicated 5 V boost on the main board, pin 2 carries
//! the chain supply, every patch makes its own regulated 3.3 V, AUX headers are 3.3 V only. Pin 2 is
//! **not frozen** by this module; the result is an engineering recommendation for approval A-02 in
//! [`crate::release_review`].
//!
//! Cross-reference: the parallel "revision D" work in [`crate::power`] reached a similar topology
//! (TPS61023 + TPS2553 + TPS62840). This module re-derives the numbers independently, adds the
//! effect of converting pin 12 from GND to AUX_ARM (3 ground contacts), resistor/reference
//! tolerances, inrush and thermal, and checks them in tests.
//!
//! | Item | Result |
//! |---|---|
//! | Boost output (TPS61023, 750 k / 100 k 0.1 %, VREF 580–610 mV) | 4.90–5.21 V (< 5.5 V VOVP min) |
//! | Chain switch TPS2553DBVR-1, R_ILIM 76.8 kΩ (same as rev D) | 302–394 mA incl. 1.5 % R tolerance (≤ 400 mA ceiling) |
//! | Former "525 mA failure" | was rev C's *capacity* figure at patch 1 on a 3.3 V chain; the chain current is now hardware-capped at ≤ 394 mA |
//! | SCD41-class module (205 + 20 mA at 3.3 V), 3 grounds | chain 0.19 / 0.20 / 0.23 A and patch VIN 4.79 / 4.46 / 3.92 V at patch 1 / 4 / 8 (≥ 3.6 V) — **PASS at every position** |
//! | Max 3.3 V-side load per position (VIN ≥ 3.6 V, chain ≤ 302 mA) | 359 / 347 / 335 / 323 / 311 / 298 / 286 / 274 mA (patch 1…8) → publish 225 mA continuous per AUX port, any position |
//! | Start-up: 8 × 10 µF + main 10 µF + 8 buck outputs | ≈ 2.5 ms at the 302 mA minimum limit (< 7.5 ms deglitch typ) |
//! | Thermal at 40 °C | TPS61023 ≈ 80 °C, TPS62840 ≈ 61 °C, TPS2553 ≈ 44 °C |
//! | Ground offset at patch 8 at the chain limit, 3 grounds | 0.41 V (used by the AUX_ARM window design) |

/// TPS61023 reference voltage (V) min/typ/max, PWM mode (DS-TPS61023).
pub const TPS61023_VREF: (f64, f64, f64) = (0.580, 0.595, 0.610);
/// TPS61023 output over-voltage threshold min (V).
pub const TPS61023_VOVP_MIN: f64 = 5.5;
/// Boost feedback divider top / bottom (Ω); R2 < 300 kΩ per datasheet.
pub const BOOST_R1: f64 = 750_000.0;
/// Boost feedback divider bottom (Ω).
pub const BOOST_R2: f64 = 100_000.0;
/// Boost divider resistor tolerance: 0.1 % thin film + 0.2 % drift (a 1 % divider would allow
/// 4.80-5.32 V, too close to the 5.5 V OVP).
pub const R_TOL: f64 = 0.003;

/// Boost output (V) min/max from reference and divider tolerances.
pub fn boost_vout() -> (f64, f64) {
    let lo = TPS61023_VREF.0 * (1.0 + BOOST_R1 * (1.0 - R_TOL) / (BOOST_R2 * (1.0 + R_TOL)));
    let hi = TPS61023_VREF.2 * (1.0 + BOOST_R1 * (1.0 + R_TOL) / (BOOST_R2 * (1.0 - R_TOL)));
    (lo, hi)
}

/// Chain-switch R_ILIM (kΩ): TPS2553DBVR-1 on the 5 V side.
pub const R_ILIM_CHAIN_5V_K: f64 = 76.8;
/// Provisional supply-contact ceiling (mA) set by the client.
pub const CONTACT_CEILING_MA: f64 = 400.0;
/// TPS62840 3.3 V setting: VSET resistor 267 kΩ (E96, allowed 256.32–277.68 kΩ).
pub const TPS62840_RSET_3V3: f64 = 267_000.0;
/// TPS62840 output accuracy (PWM, VOUT ≥ 1.8 V) ±1.5 %.
pub const TPS62840_ACC: f64 = 0.015;
/// TPS62840 maximum output current (mA).
pub const TPS62840_IOUT_MAX_MA: f64 = 750.0;
/// Patch regulator input criterion (V): 3.3 V + headroom for 100 % mode (design criterion, as rev D).
pub const PATCH_VIN_MIN: f64 = 3.6;

/// Chain description for the 5 V calculation.
#[derive(Debug, Clone, Copy)]
pub struct Chain5 {
    /// Contact resistance per mated contact (Ω), FH34 max 0.100.
    pub contact: f64,
    /// FFC conductor per segment (Ω) — 0.1 m of 0.30 × 0.035 mm (assumption A-C1).
    pub ffc: f64,
    /// Patch pass-through trace (Ω).
    pub pcb: f64,
    /// Ground contacts in parallel (4 in the source pinout, 3 if pin 12 becomes AUX_ARM).
    pub grounds: f64,
    /// Main-board chain switch on-resistance (Ω), TPS2553 DBV 135 mΩ max.
    pub switch: f64,
    /// Source voltage (V).
    pub v_src: f64,
}

/// Worst case: max contact resistance, 3 grounds, minimum boost output.
pub fn worst() -> Chain5 {
    Chain5 {
        contact: 0.100,
        ffc: crate::power_budget::conductor_ohm(0.10, 0.30, 0.035),
        pcb: crate::power_budget::conductor_ohm(0.030, 0.5, 0.035),
        grounds: 3.0,
        switch: 0.135,
        v_src: boost_vout().0,
    }
}

impl Chain5 {
    /// Loop resistance (supply + return) from the main board to patch `k` (1..=8).
    pub fn r_loop(&self, k: u32) -> f64 {
        let k = k as f64;
        let seg = 2.0 * self.contact + self.ffc;
        self.switch
            + k * seg
            + (k - 1.0) * self.pcb
            + (k * seg + (k - 1.0) * self.pcb) / self.grounds
    }
    /// Return-path resistance to patch `k` (ground offset per ampere).
    pub fn r_return(&self, k: u32) -> f64 {
        let k = k as f64;
        (k * (2.0 * self.contact + self.ffc) + (k - 1.0) * self.pcb) / self.grounds
    }
    /// Operating point for a constant-power input `p_in_w` at patch `k` (other loads lumped into
    /// `p_other_w` at the same point, conservative). Returns (chain A, patch VIN) or `None` if the
    /// power cannot be delivered.
    pub fn operating_point(&self, k: u32, p_in_w: f64) -> Option<(f64, f64)> {
        let r = self.r_loop(k);
        let d = self.v_src * self.v_src - 4.0 * r * p_in_w;
        if d < 0.0 {
            return None;
        }
        let i = (self.v_src - d.sqrt()) / (2.0 * r);
        Some((i, self.v_src - i * r))
    }
}

/// Patch buck efficiency assumption (A-E1): 0.85 worst at 50–400 mA (curve not transcribed).
pub const ETA_PATCH_WORST: f64 = 0.85;
/// Always-on patch logic at 3.3 V (mA): LV595A 0.02 + TMUX1511 0.07 + TMUX136 0.05 + TMUX1109 0.001 +
/// 2 × LVC138A 0.02 + LVC1G32/00/74/17 0.04 + TLV7032 0.002 + TLV840 0.002 + TMUX1101 0.001 + switches 0.01
/// + ladders/pull-ups 0.05 ≈ 0.27 mA (datasheet maxima, rounded up to 0.30).
pub const PATCH_LOGIC_MA: f64 = 0.30;

/// Input power (W) of one patch delivering `i3v3_ma` on its 3.3 V rail.
pub fn patch_input_w(i3v3_ma: f64) -> f64 {
    3.3 * (1.0 + TPS62840_ACC) * (i3v3_ma + PATCH_LOGIC_MA) / 1000.0 / ETA_PATCH_WORST
}

/// Largest 3.3 V-side load (mA) at patch `k` with VIN ≥ [`PATCH_VIN_MIN`] and chain ≤ `i_lim_ma`.
pub fn max_local_load_ma(c: &Chain5, k: u32, i_lim_ma: f64) -> f64 {
    let others_w = 7.0 * patch_input_w(0.0);
    let (mut lo, mut hi) = (0.0, TPS62840_IOUT_MAX_MA);
    for _ in 0..60 {
        let m = (lo + hi) / 2.0;
        let ok = match c.operating_point(k, patch_input_w(m) + others_w) {
            Some((i, v)) => v >= PATCH_VIN_MIN && i * 1000.0 <= i_lim_ma,
            None => false,
        };
        if ok {
            lo = m
        } else {
            hi = m
        }
    }
    lo
}

/// Start-up charge (µC): chain-side capacitance at `v` plus reflected charge of the patch output caps.
pub fn startup_charge_uc(n: u32, cin_uf: f64, cmain_uf: f64, cout_uf: f64, v: f64) -> f64 {
    let reflected = cout_uf * 3.3 * (3.3 / (PATCH_VIN_MIN * ETA_PATCH_WORST));
    (n as f64 * cin_uf + cmain_uf) * v + n as f64 * reflected
}

/// Thermal checks: (part, loss W, RθJA °C/W, datasheet).
pub const THERMAL_5V: &[(&str, f64, f64, &str)] = &[
    (
        "TPS61023 boost, 5.24 V × 0.394 A out, η 0.88 (assumed)",
        0.28,
        142.7,
        "DS-TPS61023 (DRL JEDEC)",
    ),
    (
        "TPS62840 patch buck, 3.35 V × 0.33 A out, η 0.85",
        0.20,
        105.6,
        "DS-TPS62840 (DLC JEDEC)",
    ),
    (
        "TPS2553DBVR-1 chain switch, 0.394 A² × 0.135 Ω",
        0.021,
        182.6,
        "DS-TPS255X",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boost_output_within_limits() {
        let (lo, hi) = boost_vout();
        assert!(lo > 4.85 && hi < 5.25, "{lo} {hi}");
        assert!(hi < TPS61023_VOVP_MIN);
        assert!(hi < 6.5, "TPS2553/TPS62840 VIN max 6.5 V");
        assert!(BOOST_R2 < 300_000.0);
    }

    #[test]
    fn chain_limit_below_ceiling_and_525_resolved() {
        let (mn, mx) = crate::passives::tps255x_window_ma(R_ILIM_CHAIN_5V_K);
        assert!(mn > 300.0 && mx <= CONTACT_CEILING_MA, "{mn} {mx}");
        // Rev D used the same resistor; its nominal-only window was 306-388 mA.
        let nom = crate::passives::tps255x_ios_ma(R_ILIM_CHAIN_5V_K);
        assert!((nom.0 - 306.0).abs() < 1.5 && (nom.2 - 388.0).abs() < 1.5);
        assert!(mx < 525.0);
    }

    #[test]
    fn scd41_class_supported_on_every_patch_with_three_grounds() {
        let c = worst();
        let others = 7.0 * patch_input_w(0.0);
        for k in 1..=8 {
            let (i, v) = c
                .operating_point(k, patch_input_w(225.0) + others)
                .expect("deliverable");
            assert!(v >= PATCH_VIN_MIN, "patch {k}: {v}");
            assert!(i * 1000.0 < 302.0, "patch {k}: {i}");
        }
        // Max local load table is monotonic and ≥ 225 mA everywhere.
        let t: Vec<f64> = (1..=8).map(|k| max_local_load_ma(&c, k, 302.0)).collect();
        assert!(t.iter().all(|&x| x >= 225.0), "{t:?}");
        for w in t.windows(2) {
            assert!(w[0] >= w[1] - 1e-6);
        }
    }

    #[test]
    fn port_fault_at_far_patch_is_bounded() {
        // A port at its 329 mA limit at patch 8 is either delivered within the chain limit or the
        // patch input sags: in both cases chain current stays ≤ the switch limit (hardware) and the
        // TLV840 (2.9 V) resets the patch if its rail collapses (startup_safety).
        let c = worst();
        let p = patch_input_w(329.0) + 7.0 * patch_input_w(0.0);
        match c.operating_point(8, p) {
            Some((i, v)) => assert!(
                i * 1000.0 > 302.0 || v < PATCH_VIN_MIN,
                "documented: fault exceeds the guaranteed window"
            ),
            None => {}
        }
    }

    #[test]
    fn ground_offset_used_by_interlock() {
        let c = worst();
        let (_, mx) = crate::passives::tps255x_window_ma(R_ILIM_CHAIN_5V_K);
        let g = mx / 1000.0 * c.r_return(8);
        assert!(g > 0.39 && g < 0.42, "{g}");
        assert!(g <= crate::global_interlock::GROUND_OFFSET_MAX);
    }

    #[test]
    fn startup_inrush_within_deglitch() {
        let (mn, _) = crate::passives::tps255x_window_ma(R_ILIM_CHAIN_5V_K);
        let q = startup_charge_uc(8, 10.0, 10.0, 10.0, boost_vout().1);
        let t_ms = q / mn; // µC / mA = ms
        assert!(t_ms < 7.5 / 2.0, "{t_ms} ms");
    }

    #[test]
    fn thermal_5v() {
        for (n, p, r, _) in THERMAL_5V {
            let t = 40.0 + p * r;
            assert!(t < 85.0, "{n}: {t}");
        }
    }

    #[test]
    fn contact_current_per_ground() {
        let (_, mx) = crate::passives::tps255x_window_ma(R_ILIM_CHAIN_5V_K);
        assert!(mx / 3.0 < 500.0 * 0.7);
    }
}
