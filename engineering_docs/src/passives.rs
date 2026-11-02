//! # Passive-component calculations
//!
//! Every resistor/capacitor value in [`crate::bom`] is derived here from a datasheet equation.
//! Functions are pure and unit-tested; worst case includes datasheet min/max factors, 1 %
//! resistor tolerance and a 0.5 % ageing/temperature drift allowance (assumption A-P1).
//!
//! | Calc | Result | Status |
//! |---|---|---|
//! | BQ24074 ISET 1.78 k | 0.50 A nom, 0.44–0.55 A | PASS (≤ 0.5 C for 2000 mAh) |
//! | BQ24074 ILIM 1.62 k | 0.99 A nom, 0.92–1.07 A | PASS (ILIM mode only; USB500 default) |
//! | BQ24074 TMR 68.1 k | 9.1 h nom, 6.7–11.5 h | PASS (> 4.5 h CC + CV need) |
//! | TPS2553-1 port RILIM 93.1 k (rev C) | 248–329 mA | PASS (≥ 205 mA SCD41 peak) |
//! | TPS2553-1 chain RILIM 76.8 k (rev D) | tps255x_window_ma(76.8) | PASS: ≥ rev D worst chain load (~231 mA), ≤ 400 mA ceiling |
//! | TPS63021 inductor 1.5 µH | Ipeak 1.34 A @1 A, 1.97 A @1.5 A | PASS with Isat ≥ 2.5 A |
//! | Patch ID windows (TMUX1511 or HC4066 ≤ 25 °C) | T/RH 2.57–2.83 V, mic 1.78–2.14 V | PASS in forced-PWM |
//! | Patch ID, TPS63021 power-save mode | T/RH up to 2.97 V | FAIL (> 2.9 V ADC range) → force PWM |
//! | Patch ID with CD74HC4066 at 85 °C (±1 µA × 16 off channels) | T/RH min 2.31 V | FAIL → TMUX1511 (D-03) |
//! | Battery divider | 1.065 V max at 4.23 V | PASS (ATTEN2 0–1.6 V) |
//! | I²C 2.2 k, 257 pF (rev C model) | 479 ns, ≤ 1.6 mA own pull-ups | PASS 100 kHz; FAIL 400 kHz |
//! | 595 pull-downs 47 k | 0.235 V at 5 µA IOZ | PASS (< 0.45 V lowest VIL); 100 k would give 0.5 V = TPS22918 VIL → FAIL |
//! | CHIP_PU RC 10 k/1 µF | τ = 10 ms | PASS (Espressif recommendation) |

/// Assumption A-P1: resistor drift allowance added to the 1 % purchase tolerance.
pub const R_DRIFT: f64 = 0.005;
/// 1 % resistor tolerance.
pub const R_TOL_1PCT: f64 = 0.01;

/// E96 decade mantissas.
pub const E96: [f64; 96] = [
    1.00, 1.02, 1.05, 1.07, 1.10, 1.13, 1.15, 1.18, 1.21, 1.24, 1.27, 1.30, 1.33, 1.37, 1.40, 1.43,
    1.47, 1.50, 1.54, 1.58, 1.62, 1.65, 1.69, 1.74, 1.78, 1.82, 1.87, 1.91, 1.96, 2.00, 2.05, 2.10,
    2.15, 2.21, 2.26, 2.32, 2.37, 2.43, 2.49, 2.55, 2.61, 2.67, 2.74, 2.80, 2.87, 2.94, 3.01, 3.09,
    3.16, 3.24, 3.32, 3.40, 3.48, 3.57, 3.65, 3.74, 3.83, 3.92, 4.02, 4.12, 4.22, 4.32, 4.42, 4.53,
    4.64, 4.75, 4.87, 4.99, 5.11, 5.23, 5.36, 5.49, 5.62, 5.76, 5.90, 6.04, 6.19, 6.34, 6.49, 6.65,
    6.81, 6.98, 7.15, 7.32, 7.50, 7.68, 7.87, 8.06, 8.25, 8.45, 8.66, 8.87, 9.09, 9.31, 9.53, 9.76,
];

/// Largest E96 value ≤ `x` (x > 0).
pub fn e96_floor(x: f64) -> f64 {
    let dec = 10f64.powf(x.log10().floor());
    let m = x / dec;
    let mut best = E96[0];
    for v in E96 {
        if v <= m + 1e-9 {
            best = v;
        }
    }
    best * dec
}

// ---------------------------------------------------------------- BQ24074 (DS-BQ24074)

/// BQ24074 K_ISET (A·Ω) min/typ/max — electrical characteristics.
pub const BQ_KISET: (f64, f64, f64) = (797.0, 890.0, 975.0);
/// BQ24074 K_ILIM (A·Ω) min/typ/max for ILIM = 0.5–1.5 A.
pub const BQ_KILIM: (f64, f64, f64) = (1500.0, 1610.0, 1720.0);
/// BQ24074 K_TMR (s/kΩ) min/typ/max.
pub const BQ_KTMR: (f64, f64, f64) = (36.0, 48.0, 60.0);
/// Allowed R_ISET range (Ω), pin-function table.
pub const BQ_RISET_RANGE: (f64, f64) = (590.0, 8_900.0);
/// Allowed R_ILIM range (Ω).
pub const BQ_RILIM_RANGE: (f64, f64) = (1_100.0, 8_000.0);
/// Allowed R_TMR range (Ω) for the programmable timer.
pub const BQ_RTMR_RANGE: (f64, f64) = (18_000.0, 72_000.0);

/// Selected BQ24074 resistors (Ω).
pub const R_ISET: f64 = 1_780.0;
/// Selected ILIM resistor (Ω).
pub const R_ILIM: f64 = 1_620.0;
/// Selected TMR resistor (Ω).
pub const R_TMR: f64 = 68_100.0;

/// (min, nom, max) of `k / R` including resistor tolerance+drift.
pub fn k_over_r(k: (f64, f64, f64), r: f64) -> (f64, f64, f64) {
    let t = R_TOL_1PCT + R_DRIFT;
    (k.0 / (r * (1.0 + t)), k.1 / r, k.2 / (r * (1.0 - t)))
}

/// Fast-charge timer (hours) min/nom/max: `t = 10 · K_TMR · R_TMR`.
pub fn bq_tmax_hours(r_tmr: f64) -> (f64, f64, f64) {
    let t = R_TOL_1PCT + R_DRIFT;
    let rk = r_tmr / 1000.0;
    (
        10.0 * BQ_KTMR.0 * rk * (1.0 - t) / 3600.0,
        10.0 * BQ_KTMR.1 * rk / 3600.0,
        10.0 * BQ_KTMR.2 * rk * (1.0 + t) / 3600.0,
    )
}

/// Minimum time needed to charge `cap_mah` at worst-case current `i_min_a` (CC) plus a CV
/// allowance `cv_h` (assumption A-P2: 1.0 h).
pub fn charge_time_needed_h(cap_mah: f64, i_min_a: f64, cv_h: f64) -> f64 {
    cap_mah / 1000.0 / i_min_a + cv_h
}

/// Linear charger dissipation (W): `(VIN − VOUT)·I_IN + (VOUT − VBAT)·I_CHG`.
pub fn bq_dissipation_w(vin: f64, vout: f64, i_in: f64, vbat: f64, i_chg: f64) -> f64 {
    (vin - vout) * i_in + (vout - vbat) * i_chg
}

// ---------------------------------------------------------------- TPS2552/3 (DS-TPS255X)

/// TPS255x current-limit thresholds (mA) min/nom/max for `r_kohm` (Fig. 8-1 equations 1–3).
///
/// ```
/// let (mn, _, mx) = beebox_v2_engineering::passives::tps255x_ios_ma(105.0);
/// assert!(mn > 220.0 && mx < 290.0);
/// ```
pub fn tps255x_ios_ma(r_kohm: f64) -> (f64, f64, f64) {
    (
        25_230.0 / r_kohm.powf(1.016),
        23_950.0 / r_kohm.powf(0.977),
        22_980.0 / r_kohm.powf(0.94),
    )
}

/// Worst-case window including 1 % + drift on R_ILIM.
pub fn tps255x_window_ma(r_kohm: f64) -> (f64, f64) {
    let t = R_TOL_1PCT + R_DRIFT;
    (
        tps255x_ios_ma(r_kohm * (1.0 + t)).0,
        tps255x_ios_ma(r_kohm * (1.0 - t)).2,
    )
}

/// Largest E96 R_ILIM (kΩ) whose minimum limit still meets `i_min_ma`.
pub fn tps255x_rilim_for_min(i_min_ma: f64) -> f64 {
    let exact = (25_230.0 / i_min_ma).powf(1.0 / 1.016) / (1.0 + R_TOL_1PCT + R_DRIFT);
    e96_floor(exact)
}

/// Recommended R_ILIM range (kΩ).
pub const TPS255X_RILIM_RANGE_K: (f64, f64) = (15.0, 232.0);
/// AUX port R_ILIM (kΩ).
pub const R_ILIM_PORT_K: f64 = 93.1;
/// Main-board +5V_CHAIN R_ILIM (kΩ). Rev D: 76.8 k (was 66.5 k -> 349-451 mA, which exceeded the
/// client 400 mA provisional contact ceiling).
pub const R_ILIM_CHAIN_K: f64 = 76.8;

/// Time (ms) to charge `c_uf` to `v` at constant `i_ma` (inrush under current limit).
pub fn charge_time_ms(c_uf: f64, v: f64, i_ma: f64) -> f64 {
    c_uf * v / i_ma
}

// ---------------------------------------------------------------- TPS63021 (DS-TPS6302X)

/// TPS63021 output (V) min/typ/max in PWM mode.
pub const TPS63021_VOUT_PWM: (f64, f64, f64) = (3.267, 3.3, 3.333);
/// Maximum positive offset in power-save mode, referenced to PWM (+5 %).
pub const TPS63021_PS_OFFSET_MAX: f64 = 0.05;
/// Switching frequency min (Hz).
pub const TPS63021_FSW_MIN: f64 = 2.2e6;

/// Boost-mode inductor peak current (A), datasheet equations 1–2.
pub fn tps63021_ipeak_a(iout: f64, vin: f64, vout: f64, eta: f64, f: f64, l: f64) -> f64 {
    let d = ((vout - vin) / vout).max(0.0);
    iout / (eta * (1.0 - d)) + vin * d / (2.0 * f * l)
}

// ---------------------------------------------------------------- dividers / ID

/// Divider output `v · r_bot / (r_top + r_bot)`.
pub fn divider(v: f64, r_top: f64, r_bot: f64) -> f64 {
    v * r_bot / (r_top + r_bot)
}

/// Patch ID check inputs.
#[derive(Debug, Clone, Copy)]
pub struct IdCheck {
    /// Chain rail at the source (V) min.
    pub v_min: f64,
    /// Chain rail at the source (V) max.
    pub v_max: f64,
    /// Idle chain drop to the farthest patch (V).
    pub drop_max: f64,
    /// ADC total error (V), DS-ESP-SOC Table 5-6 ATTEN3 (±50 mV).
    pub adc_err: f64,
    /// Total leakage on SENSOR_DATA_ID (A).
    pub leak_a: f64,
}

/// Worst-case ID voltage window (V) for `r_id` against `r_pd`.
pub fn id_window(c: &IdCheck, r_id: f64, r_pd: f64) -> (f64, f64) {
    let t = R_TOL_1PCT + R_DRIFT;
    let rth = |a: f64, b: f64| a * b / (a + b);
    let lo = divider(c.v_min - c.drop_max, r_id * (1.0 + t), r_pd * (1.0 - t))
        - c.adc_err
        - c.leak_a * rth(r_id, r_pd);
    let hi = divider(c.v_max, r_id * (1.0 - t), r_pd * (1.0 + t))
        + c.adc_err
        + c.leak_a * rth(r_id, r_pd);
    (lo, hi)
}

/// Main-board ID pull-down (Ω).
pub const R_ID_PD: f64 = 100_000.0;
/// T/RH ID resistor (Ω).
pub const R_ID_TRH: f64 = 22_000.0;
/// Mic ID resistor (Ω).
pub const R_ID_MIC: f64 = 68_000.0;
/// ESP32-S3 ADC1 ATTEN3 effective range upper limit (V), DS-ESP-SOC Table 5-6.
pub const ADC_ATTEN3_MAX_V: f64 = 2.9;
/// Leakage on SENSOR_DATA_ID excluding the primary switches: ESP32 50 nA + ESD 2 × 10 nA +
/// 8 arm-sense diodes × 50 nA (requirement on the diode, rev C AUX-armed sense).
pub const ID_LEAK_OTHER_A: f64 = 50e-9 + 20e-9 + 8.0 * 50e-9;
/// Off primary-switch channels on pin 7 seen during ID (2 per patch: SD and ID channels).
pub const ID_OFF_CHANNELS: f64 = 16.0;
/// TMUX1511 IS(OFF) max (A), -40..125 °C.
pub const TMUX1511_ISOFF: f64 = 100e-9;
/// CD74HC4066 IZ max (A): 25 °C and -55..85 °C.
pub const HC4066_IZ_25C: f64 = 0.1e-6;
/// CD74HC4066 IZ max -55..85 °C (A).
pub const HC4066_IZ_85C: f64 = 1.0e-6;
/// SENSOR_DATA_ID leakage with TMUX1511 (recommended).
pub const ID_LEAK_A: f64 = ID_OFF_CHANNELS * TMUX1511_ISOFF + ID_LEAK_OTHER_A;

/// ID check in forced-PWM mode (PS/SYNC high during ID).
pub const ID_PWM: IdCheck = IdCheck {
    v_min: 3.267,
    v_max: 3.333,
    drop_max: 0.010,
    adc_err: 0.050,
    leak_a: ID_LEAK_A,
};
/// ID check if TPS63021 is left in power-save mode (+5 %).
pub const ID_PS: IdCheck = IdCheck {
    v_min: 3.267,
    v_max: 3.333 * 1.05,
    drop_max: 0.010,
    adc_err: 0.050,
    leak_a: ID_LEAK_A,
};
/// ID check with the rev C baseline CD74HC4066 at its 85 °C leakage limit.
pub const ID_HC4066_85C: IdCheck = IdCheck {
    v_min: 3.267,
    v_max: 3.333,
    drop_max: 0.010,
    adc_err: 0.050,
    leak_a: ID_OFF_CHANNELS * HC4066_IZ_85C + ID_LEAK_OTHER_A,
};
/// ID check with CD74HC4066 at its 25 °C leakage limit.
pub const ID_HC4066_25C: IdCheck = IdCheck {
    v_min: 3.267,
    v_max: 3.333,
    drop_max: 0.010,
    adc_err: 0.050,
    leak_a: ID_OFF_CHANNELS * HC4066_IZ_25C + ID_LEAK_OTHER_A,
};

/// Battery divider (Ω) and expected ADC voltage at 4.23 V (BQ24074 VBAT(REG) max).
pub const R_VBAT_TOP: f64 = 1_000_000.0;
/// Battery divider bottom.
pub const R_VBAT_BOT: f64 = 330_000.0;
/// ESP32-S3 ADC1 ATTEN2 effective range (V).
pub const ADC_ATTEN2_MAX_V: f64 = 1.6;

// ---------------------------------------------------------------- I²C (DS-UM10204, DS-SHT4X)

/// Rise time 30 %→70 % (ns): `0.8473 · Rp · Cb`.
pub fn i2c_rise_ns(rp_ohm: f64, cb_pf: f64) -> f64 {
    0.8473 * rp_ohm * cb_pf * 1e-3
}
/// Minimum pull-up (Ω) for `iol` (A) at `vol` (V).
pub fn rp_min(vdd: f64, vol: f64, iol: f64) -> f64 {
    (vdd - vol) / iol
}
/// Maximum pull-up (Ω) for rise time `tr_ns` with `cb_pf`.
pub fn rp_max(tr_ns: f64, cb_pf: f64) -> f64 {
    tr_ns / (0.8473 * cb_pf * 1e-3)
}
/// Parallel resistance.
pub fn par(a: f64, b: f64) -> f64 {
    a * b / (a + b)
}

/// Bus-capacitance estimate inputs (pF). Values marked *A* are assumptions.
#[derive(Debug, Clone, Copy)]
pub struct BusCap {
    /// Patches on the chain.
    pub patches: u32,
    /// Per patch: AUX isolator off capacitance on pins 4/5 (TMUX136 2 pF max, DS-TMUX136).
    pub mux_off: f64,
    /// Per patch: primary switch capacitance (CD74HC4066 CI 5 pF; TMUX1511 COFF 4 pF).
    pub sw_off: f64,
    /// Per patch: two FH34 contacts + pass-through trace (*A*: 3 pF, rev C value).
    pub patch_pcb: f64,
    /// FFC per metre (*A*: 100 pF/m rev C value; FFC MPN open, OQ-024).
    pub ffc_pf_per_m: f64,
    /// Total FFC length (m) (*A*: 8 × 0.1 m).
    pub ffc_len_m: f64,
    /// Main board pin + trace (ESP32 CIN 2 pF, *A* 5 pF trace).
    pub main: f64,
    /// Active branch: TMUX136 on 2 pF + TMUX1109 CON 38 pF + module side (*A*: 50 pF, rev C value).
    pub active_branch: f64,
}

impl BusCap {
    /// Total estimate (pF).
    pub fn total(&self) -> f64 {
        self.patches as f64 * (self.mux_off + self.sw_off + self.patch_pcb)
            + self.ffc_pf_per_m * self.ffc_len_m
            + self.main
            + self.active_branch
    }
}

/// Worst-case SDA/SCL capacitance with an AUX module at patch 8 (rev C: TMUX136 isolator 2 pF off,
/// CD74HC4066 CI 5 pF; active branch TMUX136 2 pF + TMUX1109 CON 38 pF + module side 50 pF; FFC 100 pF/m as in
/// `calculations`).
pub const I2C_BUS_WORST: BusCap = BusCap {
    patches: 8,
    mux_off: 2.0,
    sw_off: 5.0,
    patch_pcb: 3.0,
    ffc_pf_per_m: 100.0,
    ffc_len_m: 0.8,
    main: 7.0,
    active_branch: 90.0,
};

/// Patch-side pull-up per AUX port and for the SHT45 (Ω).
pub const R_PULLUP_PATCH: f64 = 2_200.0;
/// Display-bus pull-up on the main board (Ω).
pub const R_PULLUP_DISPLAY: f64 = 2_200.0;

// ---------------------------------------------------------------- misc

/// Voltage developed by a leakage `i` through a pull-down `r`.
pub fn leak_voltage(i: f64, r: f64) -> f64 {
    i * r
}
/// SN74HC595 / SN74LV595A IOZ max (A), -40..85 °C (both ±5 µA).
pub const LV595A_IOZ: f64 = 5e-6;
/// Pull-down on every used 595 output (Ω).
pub const R_PD_595: f64 = 47_000.0;
/// Lowest VIL among loads on 595 outputs (TMUX1511 0.45 V; TPS22918 ON 0.5 V).
pub const MIN_VIL_LOADS: f64 = 0.45;

/// RC time constant (s).
pub fn tau(r: f64, c: f64) -> f64 {
    r * c
}

/// Decoupling / bulk capacitor schedule (part, value, reason, datasheet).
pub const DECOUPLING: &[(&str, &str, &str, &str)] = &[
    (
        "ESP32-S3 module 3V3",
        "22 µF X5R 10 V 0805 + 100 nF X7R 0402",
        ">= 10 µF at power entrance (HWDG) + HF bypass",
        "DS-ESP-HWDG",
    ),
    (
        "ESP32-S3 CHIP_PU",
        "10 kΩ to 3V3 + 1 µF to GND",
        "Recommended RC delay R = 10 k, C = 1 µF",
        "DS-ESP-HWDG",
    ),
    (
        "BQ24074 IN",
        "1 µF X7R 25 V 0603 + 10 nF",
        "1-10 µF required; IN tolerates 26 V",
        "DS-BQ24074",
    ),
    (
        "BQ24074 OUT",
        "10 µF X5R 10 V 0805",
        "4.7-47 µF required (TPS63021 2 × 10 µF also on this net)",
        "DS-BQ24074",
    ),
    (
        "BQ24074 BAT",
        "10 µF X5R 10 V 0805",
        "4.7-47 µF required",
        "DS-BQ24074",
    ),
    (
        "TPS63021 VIN/VINA",
        "2 × 10 µF X5R 10 V 0805 + 100 nF",
        "Typical application 2 × 10 µF",
        "DS-TPS6302X",
    ),
    (
        "TPS63021 VOUT",
        "3 × 22 µF X5R 10 V 0805",
        "Typical application with 1.5 µH (Table 1 note 3)",
        "DS-TPS6302X",
    ),
    (
        "TPS2553 chain IN / OUT",
        "1 µF X7R 0603 / 10 µF X5R 0805",
        ">= 0.1 µF ceramic at IN; OUT bulk for FFC",
        "DS-TPS255X",
    ),
    (
        "TPS2553 port IN / OUT",
        "1 µF X7R 0603 / 10 µF X5R 0805 + 100 nF at header",
        "Module peak buffering, ESD return",
        "DS-TPS255X",
    ),
    (
        "TPS22918 VIN / VOUT",
        "1 µF X7R 0603 / 100 nF",
        "CIN 1 µF; CT 1 nF sets rise time",
        "DS-TPS22918",
    ),
    (
        "Patch +3V3_CHAIN entry",
        "10 µF X5R 10 V 0805 + 100 nF",
        "Local reservoir for port switch-on (A-P3)",
        "DS-TPS255X",
    ),
    (
        "SN74HC595, CD74HC4066, TMUX136, TMUX1109, CD74HC238, CD74HC138",
        "100 nF X7R 0402 each",
        "Standard logic bypass; TMUX1109 0.1-10 µF",
        "DS-TMUX1109",
    ),
    (
        "SHT45",
        "100 nF X7R 0402",
        "Typical application bypass",
        "DS-SHT4X",
    ),
    (
        "DMM-4026-B-I2S-R",
        "100 nF X7R 0402",
        "Pin 3: decouple with 0.1 µF",
        "DS-PUI-MIC",
    ),
    (
        "VBAT ADC",
        "100 nF X7R 0402",
        "ADC characterised with 100 nF; τ = 248 k·100 n = 24.8 ms",
        "DS-ESP-SOC",
    ),
    (
        "SENSOR_DATA_ID",
        "NO capacitor (≤ 10 pF if any)",
        "Shared with I²S SD (mic load ≤ 140 pF) - an ADC filter cap is forbidden",
        "DS-PUI-MIC",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }

    #[test]
    fn e96_selection() {
        assert!(close(e96_floor(106.4), 105.0, 1e-9));
        assert!(close(e96_floor(1.623), 1.62, 1e-9));
    }

    #[test]
    fn bq24074_charge_current_and_limits() {
        assert!(R_ISET >= BQ_RISET_RANGE.0 && R_ISET <= BQ_RISET_RANGE.1);
        let (mn, nom, mx) = k_over_r(BQ_KISET, R_ISET);
        assert!(close(nom, 0.5, 0.001));
        assert!(mn > 0.44 && mx < 0.56);
        assert!(mx <= 2.0 * 0.5); // <= 0.5 C of 2000 mAh with 2x margin to 1 C
        let (imn, inom, imx) = k_over_r(BQ_KILIM, R_ILIM);
        assert!(R_ILIM >= BQ_RILIM_RANGE.0 && R_ILIM <= BQ_RILIM_RANGE.1);
        assert!(close(inom, 0.994, 0.001) && imn > 0.9 && imx < 1.08);
    }

    #[test]
    fn bq24074_timer_covers_full_charge() {
        assert!(R_TMR >= BQ_RTMR_RANGE.0 && R_TMR <= BQ_RTMR_RANGE.1);
        let (tmin, tnom, _) = bq_tmax_hours(R_TMR);
        let need = charge_time_needed_h(2000.0, k_over_r(BQ_KISET, R_ISET).0, 1.0);
        assert!(tmin > need, "timer {tmin} h < need {need} h");
        assert!(close(tnom, 9.08, 0.01));
        // V1 value fails the same check:
        assert!(bq_tmax_hours(46_400.0).0 < need);
    }

    #[test]
    fn bq24074_thermal_worst_case() {
        // USB 5.25 V, OUT 4.4 V, input limit 1.07 A, battery 3.0 V charging 0.55 A.
        let p = bq_dissipation_w(5.25, 4.4, 1.07, 3.0, 0.55);
        let tj = 40.0 + p * 44.5; // RθJA 44.5 °C/W, 40 °C ambient (assumption A-T1)
        assert!(tj < 125.0, "TJ {tj}"); // thermal regulation threshold 125 °C
    }

    #[test]
    fn port_and_chain_current_limits() {
        assert!(
            R_ILIM_PORT_K >= TPS255X_RILIM_RANGE_K.0 && R_ILIM_PORT_K <= TPS255X_RILIM_RANGE_K.1
        );
        let (pmin, pmax) = tps255x_window_ma(R_ILIM_PORT_K);
        assert!(pmin >= 205.0, "port min {pmin}"); // SCD41 peak max @3.3 V
        assert!(tps255x_rilim_for_min(205.0) >= R_ILIM_PORT_K);
        // Same equations as rev C calculations::tps2553_ilim_ma
        let a = tps255x_ios_ma(93.1);
        let b = crate::calculations::tps2553_ilim_ma(93.1);
        assert!(close(a.0, b.0, 1e-9) && close(a.2, b.2, 1e-9));
        let _ = pmax;
        let (cmin, cmax) = tps255x_window_ma(R_ILIM_CHAIN_K);
        // Rev D (+5V_CHAIN): worst normal chain load = SCD41 + overhead at patch 8, eta 0.80.
        let c = crate::calculations::Chain::baseline(0.10);
        let worst_chain = crate::power::option_b_operating_point(
            &c,
            8,
            3.3 * 0.225,
            0.80,
            8.0 * 0.000314 * 3.3 / 0.80,
            5.0,
        )
        .unwrap()
        .0;
        assert!(cmin >= worst_chain, "chain min {cmin} < load {worst_chain}");
        assert!(
            cmax <= crate::power::CONTACT_CEILING_MA,
            "chain max {cmax} > 400 mA provisional ceiling"
        );
        // Inrush of 8 patches (8 × 11 µF) + main 10 µF at the minimum limit, well below 7.5 ms deglitch.
        // Inrush: 8 patches × 11 µF + main 10 µF at the minimum limit, well below 7.5 ms deglitch.
        assert!(charge_time_ms(98.0, 5.0, cmin) < 7.5 / 2.0);
    }

    #[test]
    fn tps63021_inductor_peak() {
        // L −30 %, f min, 2.9 V battery cutoff, η 0.9 (datasheet assumption value)
        let l = 1.5e-6 * 0.7;
        let i1 = tps63021_ipeak_a(1.0, 2.9, 3.3, 0.9, TPS63021_FSW_MIN, l);
        let i15 = tps63021_ipeak_a(1.5, 2.9, 3.3, 0.9, TPS63021_FSW_MIN, l);
        assert!(i1 < 1.40 && i15 < 2.0 && i15 < 2.5);
    }

    #[test]
    fn id_windows_pass_in_pwm_and_fail_in_ps_mode() {
        let trh = id_window(&ID_PWM, R_ID_TRH, R_ID_PD);
        let mic = id_window(&ID_PWM, R_ID_MIC, R_ID_PD);
        assert!(
            trh.0 >= 2.50 && trh.1 <= 2.95 && trh.1 <= ADC_ATTEN3_MAX_V,
            "{trh:?}"
        );
        assert!(mic.0 >= 1.70 && mic.1 <= 2.20, "{mic:?}");
        assert!(mic.1 < trh.0); // windows never overlap
        let trh_ps = id_window(&ID_PS, R_ID_TRH, R_ID_PD);
        assert!(
            trh_ps.1 > ADC_ATTEN3_MAX_V,
            "PS-mode must be shown to fail: {trh_ps:?}"
        );
        // CD74HC4066 baseline: passes at its 25 °C leakage, fails at its 85 °C limit.
        let h25 = id_window(&ID_HC4066_25C, R_ID_TRH, R_ID_PD);
        let m25 = id_window(&ID_HC4066_25C, R_ID_MIC, R_ID_PD);
        assert!(
            h25.0 >= 2.50 && h25.1 <= 2.9 && m25.0 >= 1.70 && m25.1 <= 2.20,
            "{h25:?} {m25:?}"
        );
        let h85 = id_window(&ID_HC4066_85C, R_ID_TRH, R_ID_PD);
        assert!(h85.0 < 2.50, "HC4066 85 °C must be shown to fail: {h85:?}");
    }

    #[test]
    fn battery_divider_within_adc_range() {
        let t = R_TOL_1PCT + R_DRIFT;
        let v = divider(4.23, R_VBAT_TOP * (1.0 - t), R_VBAT_BOT * (1.0 + t));
        assert!(v < ADC_ATTEN2_MAX_V && v > 1.0);
        assert!(4.23 / (R_VBAT_TOP + R_VBAT_BOT) < 3.3e-6);
    }

    #[test]
    fn i2c_pullups_and_capacitance() {
        let cb = I2C_BUS_WORST.total();
        assert!(cb > 200.0 && cb < 400.0, "Cb {cb}"); // SHT4x limit 400 pF
                                                      // Cross-check against rev C calculations::bus_capacitance_pf (0.10 m segments, isolated).
        let rc = crate::calculations::bus_capacitance_pf(0.10, true);
        assert!((cb - rc).abs() / rc < 0.05, "{cb} vs {rc}");
        let tr = i2c_rise_ns(R_PULLUP_PATCH, cb);
        assert!(tr <= 1000.0, "standard mode {tr}");
        assert!(tr > 300.0, "fast mode must be shown to fail at worst case");
        // Sink current incl. common breakout pull-ups (10 k, 4.7 k) stays <= 3 mA.
        for module_rp in [10_000.0, 4_700.0] {
            let i = 3.5 / par(R_PULLUP_PATCH, module_rp);
            assert!(i <= 3.0e-3, "{module_rp}: {i}");
        }
        // A 2.2 k module pull-up in parallel is NOT allowed (DNI ours):
        assert!(3.5 / par(R_PULLUP_PATCH, 2_200.0) > 3.0e-3);
        assert!(R_PULLUP_PATCH >= rp_min(3.5, 0.4, 3e-3));
        assert!(R_PULLUP_PATCH >= 390.0); // SHT4x Rp min
        assert!(rp_max(1000.0, cb) > R_PULLUP_PATCH);
    }

    #[test]
    fn shift_register_pulldowns_hold_off() {
        assert!(leak_voltage(LV595A_IOZ, R_PD_595) < MIN_VIL_LOADS);
        assert!(leak_voltage(LV595A_IOZ, 100_000.0) >= 0.5); // 100 k-class would reach TPS22918 VIL
    }

    #[test]
    fn chip_pu_rc() {
        assert!(close(tau(10e3, 1e-6), 0.010, 1e-9));
    }
}
