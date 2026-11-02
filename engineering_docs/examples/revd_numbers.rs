use beebox_v2_engineering::{calculations::Chain, power::*};
fn main() {
    for s in SCENARIOS.iter() {
        println!(
            "{} avg {:.2} mA life {:.0} h ({:.1} d)",
            s.name,
            s.average_ma(),
            s.runtime_h(),
            s.runtime_h() / 24.0
        );
    }
    let c = Chain::baseline(0.10);
    for k in [1u32, 4, 8] {
        for eta in [0.80, 0.88] {
            let r =
                option_b_operating_point(&c, k, 3.3 * 0.225, eta, 8.0 * 0.000314 * 3.3 / eta, 5.0)
                    .unwrap();
            println!(
                "k={k} eta={eta} chain {:.0} mA vin {:.2} V gnd {:.3} V",
                r.0, r.1, r.2
            );
        }
    }
    println!(
        "peak overlapped {:.0} mA, not overlapped {:.0} mA",
        battery_peak_ma(340.0, 388.0, 0.85, 0.82, 3.0),
        battery_peak_ma(0.0, 388.0, 0.85, 0.82, 3.0)
            .max(battery_peak_ma(340.0, 0.0, 0.85, 0.82, 3.0))
    );
}
