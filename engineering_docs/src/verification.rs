//! # 12. Testing and verification
//!
//! "A clean-looking schematic or successful script execution is not evidence of electrical
//! correctness" (p.16). Each test needs a reproducible artefact.

/// Verification test.
#[derive(Debug, Clone, Copy)]
pub struct Test {
    /// ID.
    pub id: &'static str,
    /// Name.
    pub name: &'static str,
    /// Acceptance criterion.
    pub acceptance: &'static str,
    /// Evidence artefact.
    pub evidence: &'static str,
}

/// Tests V-01..V-14 (p.16).
pub const TESTS: &[Test] = &[
    Test { id: "V-01", name: "Safe power-up", acceptance: "OE_N disables outputs; sensor and AUX supplies OFF until selected.", evidence: "Scope + current log" },
    Test { id: "V-02", name: "Chain order", acceptance: "Walking-one/zero maps all 8 patches (full image length) and verifies pin-8 direction.", evidence: "Logic-analyser log" },
    Test { id: "V-03", name: "Primary patch ID", acceptance: "T/RH / mic / empty classify correctly over tolerance and rail variation.", evidence: "ADC CSV / calibration table" },
    Test { id: "V-04", name: "AUX port select", acceptance: "J_AUX1..4 individually selected; inactive ports cannot drive or load shared lines.", evidence: "Test report" },
    Test { id: "V-05", name: "AUX module power", acceptance: "Correct rail and current limit at each header; inactive port off; no back-power.", evidence: "DMM / current trace" },
    Test { id: "V-06", name: "AUX input range", acceptance: "AUX SDA/SCL never exceed 3.3 V; I2C low level at ESP32-S3 within VIL incl. mux RON; ESD response recorded.", evidence: "Scope + calculations" },
    Test { id: "V-07", name: "I2C T/RH", acceptance: "Only selected SHT45 responds; rise time and sink current pass.", evidence: "I2C capture" },
    Test { id: "V-08", name: "I2S microphone", acceptance: "Only selected mic drives SD; capture works at patch 1 and patch 8.", evidence: "Audio file + scope" },
    Test { id: "V-09", name: "AUX protocol profiles", acceptance: "Each approved breakout answers at its profile address on every port of patch 1 and patch 8.", evidence: "Raw logs" },
    Test { id: "V-10", name: "Power-off backfeed", acceptance: "Inactive module rails and I/O protection paths do not energise sensors.", evidence: "Current / voltage log" },
    Test { id: "V-11", name: "Full-chain power / SI", acceptance: "Patch-8 rail, I2C timing, I2S edges, module start-up pass at max chain length.", evidence: "Scope" },
    Test { id: "V-12", name: "FPC / pogo map", acceptance: "All 12 contacts, orientation and pin-8 cascade pass continuity fixture.", evidence: "Fixture log" },
    Test { id: "V-13", name: "Fault recovery", acceptance: "Missing module, invalid ID, timeout, reset, open chain leave outputs safe.", evidence: "Firmware log" },
    Test { id: "V-15", name: "AUX-armed sense", acceptance: "ADC distinguishes 0, 1 and >=2 armed patches at every position over rail and temperature range.", evidence: "ADC CSV" },
    Test { id: "V-16", name: "Per-position supply drop", acceptance: "Port VCC >= 2.9 V at the published current rating for every patch position; contact + cable resistance measured.", evidence: "DMM / scope at patch 1, 4, 8" },
    Test { id: "V-17", name: "AUX I2C margins", acceptance: "Rise time <= 1000 ns, VOL at ESP32-S3 <= 0.825 V incl. ground offset, at patch 8 with each approved module.", evidence: "Scope" },
    Test { id: "V-18", name: "Exclusivity fault injection", acceptance: "Deliberately corrupted image (two patches armed) is detected before any port powers; system enters FAULT with all outputs off.", evidence: "Firmware log + scope" },
    Test { id: "V-14", name: "Mechanical / thermal", acceptance: "No connector lift, flex damage, blocked port, unacceptable heating, interference.", evidence: "Inspection record" },
];

/// First prototype stop-gate (p.16).
pub const PROTOTYPE_STOP_GATE: &str = "Build 1 main + 2 patches (one T/RH, one I2S mic) with an AUX proof on at least one patch using representative module(s). Do not scale to 8 patches until chain order, ID, primary isolation, AUX switching, module power and back-power tests pass.";

/// Design-stage checks before fabrication (p.3, p.14).
pub const DESIGN_CHECKS: &[&str] = &[
    "Design calculations recorded (ID tolerance, power budget, chain drop, thermal).",
    "ERC clean or every waiver justified.",
    "Netlist export compared against interfaces::CHAIN_PINS and pin-to-net CSV.",
    "Symbol-to-footprint pad mapping verified against manufacturer drawings.",
    "Independent audit (phase P5) with no unresolved critical mismatch.",
];
