//! # Main-board component architecture (sheets M01–M05)
//!
//! Recommended component architecture for the ESP32-S3 main controller PCB. Component records:
//! [`crate::component_specs`]; values: [`crate::passives`]; budget: [`crate::power_budget`].
//!
//! ```text
//! M01  USB4105 ─ USBLC6 (D±) ─ BQ24074 (IN 1µF/25V; ISET 1.78k; ILIM 1.62k; TMR 68.1k; TS←NTC)
//!      JST-PH BATT ─ BAT 10µF          OUT 10µF ─ TPS63021 (L 1.5µH, 2×10µF in, 3×22µF out) ─ +3V3
//!      VBAT ─ 1M/330k ─ 100nF ─ GPIO2 (ADC1)        MAX17048 on system I²C
//! M02  ESP32-S3-WROOM-1-N16R2 (22µF+100nF, CHIP_PU 10k/1µF, BOOT GPIO0, USB GPIO19/20, UART0 header)
//! M03  GPIO11-14 ─33Ω─ PATCH_SHIFT_DATA/CLK/LATCH/OE_N (rev D: OE_N pull-up 47k on each patch to +3V3_LOCAL; CLK/LATCH 100k pull-down)
//!      GPIO1 ─ SENSOR_DATA_ID (100k 1% pull-down, NO filter cap); GPIO4/5 ─33Ω─ SENSOR_CLK/SYNC
//!      TPS61023 +5V0 ─ TPS2553-1 (RILIM 76.8k, EN←GPIO7, FAULT→GPIO8) ─ +5V_CHAIN (rev D); SENSOR_PU_EN (GPIO6) ─ 2×2.2k ─ SENSOR_CLK/SYNC
//! M04  FH34SRJ-12S-0.5SH(50) + 2 × TPD4E05U06 (7 chain signals) + test pads
//! M05  TPS22918 → +3V3_DISPLAY; JST-SH display (GND,+3V3_DISPLAY,SCL,SDA) 2.2k pull-ups; RGB status LEDs
//! ```

use crate::model::Status;

/// One ESP32-S3 GPIO allocation.
#[derive(Debug, Clone, Copy)]
pub struct GpioAlloc {
    /// GPIO number.
    pub gpio: u8,
    /// Net.
    pub net: &'static str,
    /// Peripheral / mode.
    pub function: &'static str,
    /// Reset-state handling.
    pub boot_state: &'static str,
}

/// Proposed GPIO map for ESP32-S3-WROOM-1-N16R2 (closes OQ-010 when approved).
///
/// Rules applied: SENSOR_DATA_ID on ADC1 (GPIO1-10); GPIO38 not used for ID; strapping pins
/// GPIO0/3/45/46 left for boot functions; GPIO19/20 USB; GPIO43/44 UART0; GPIO35-37 usable because
/// the R2 variant has quad (not octal) PSRAM (DS-ESP-MOD pin note b).
pub const GPIO_MAP: &[GpioAlloc] = &[
    GpioAlloc { gpio: 0, net: "BOOT", function: "Strapping, BOOT button to GND, 10 k pull-up", boot_state: "high = SPI boot" },
    GpioAlloc { gpio: 1, net: "SENSOR_DATA_ID", function: "ADC1_CH0 (ID) / I²S DIN (mic)", boot_state: "input, 100 k pull-down on board" },
    GpioAlloc { gpio: 2, net: "VBAT_SENSE", function: "ADC1_CH1, ATTEN2", boot_state: "input" },
    GpioAlloc { gpio: 4, net: "SENSOR_CLK", function: "I²C SCL (open-drain) / I²S BCLK (push-pull), 33 Ω series", boot_state: "input" },
    GpioAlloc { gpio: 5, net: "SENSOR_SYNC", function: "I²C SDA (open-drain) / I²S WS, 33 Ω series", boot_state: "input" },
    GpioAlloc { gpio: 6, net: "SENSOR_PU_EN", function: "Drives the 2 × 2.2 k SENSOR_CLK/SYNC pull-ups (PRP-MB-02): high in primary I²C mode, Hi-Z in I²S/AUX/ID modes", boot_state: "input (pull-ups off)" },
    GpioAlloc { gpio: 7, net: "CHAIN_PWR_EN", function: "TPS2553-1 EN (active high)", boot_state: "100 k pull-down -> chain OFF" },
    GpioAlloc { gpio: 8, net: "CHAIN_FAULT_N", function: "TPS2553-1 FAULT (open drain)", boot_state: "100 k pull-up" },
    GpioAlloc { gpio: 9, net: "SYS_SDA", function: "Display + MAX17048 I²C SDA", boot_state: "2.2 k pull-up" },
    GpioAlloc { gpio: 10, net: "SYS_SCL", function: "Display + MAX17048 I²C SCL", boot_state: "2.2 k pull-up" },
    GpioAlloc { gpio: 11, net: "PATCH_SHIFT_DATA", function: "595 SER of patch 1, 33 Ω series", boot_state: "input; chain unpowered" },
    GpioAlloc { gpio: 12, net: "PATCH_SHIFT_CLK", function: "595 SRCLK (all patches), 33 Ω series", boot_state: "100 k pull-down" },
    GpioAlloc { gpio: 13, net: "PATCH_LATCH", function: "595 RCLK (all patches), 33 Ω series", boot_state: "100 k pull-down" },
    GpioAlloc { gpio: 14, net: "PATCH_OE_N", function: "595 OE (all patches), 33 Ω series", boot_state: "Rev D: 47 k pull-up on every patch to +3V3_LOCAL -> outputs Hi-Z whenever the chain is powered; GPIO driven low or Hi-Z while the chain is off" },
    GpioAlloc { gpio: 15, net: "DISP_PWR_EN", function: "TPS22918 ON for +3V3_DISPLAY", boot_state: "47 k pull-down" },
    GpioAlloc { gpio: 16, net: "REG_PS_SYNC", function: "TPS63021 PS/SYNC: high = forced PWM (ID, audio)", boot_state: "100 k pull-down = power save" },
    GpioAlloc { gpio: 17, net: "CHG_EN1", function: "BQ24074 EN1", boot_state: "100 k pull-up to +3V3" },
    GpioAlloc { gpio: 18, net: "CHG_EN2", function: "BQ24074 EN2", boot_state: "100 k pull-down (USB500 default)" },
    GpioAlloc { gpio: 19, net: "USB_D-", function: "USB Serial/JTAG", boot_state: "-" },
    GpioAlloc { gpio: 20, net: "USB_D+", function: "USB Serial/JTAG", boot_state: "-" },
    GpioAlloc { gpio: 21, net: "CHG_STAT_N", function: "BQ24074 CHG (open drain)", boot_state: "100 k pull-up" },
    GpioAlloc { gpio: 35, net: "CHG_PGOOD_N", function: "BQ24074 PGOOD (open drain)", boot_state: "100 k pull-up" },
    GpioAlloc { gpio: 36, net: "REG_PG", function: "TPS63021 PG (open drain)", boot_state: "100 k pull-up" },
    GpioAlloc { gpio: 37, net: "GAUGE_ALRT_N", function: "MAX17048 ALRT", boot_state: "100 k pull-up" },
    GpioAlloc { gpio: 39, net: "LED_R", function: "Status LED", boot_state: "off" },
    GpioAlloc { gpio: 40, net: "LED_G", function: "Status LED", boot_state: "off" },
    GpioAlloc { gpio: 41, net: "LED_B", function: "Status LED", boot_state: "off" },
    GpioAlloc { gpio: 43, net: "UART0_TX", function: "Debug header", boot_state: "-" },
    GpioAlloc { gpio: 44, net: "UART0_RX", function: "Debug header", boot_state: "-" },
];

/// GPIOs that must not be used for new functions.
pub const RESERVED_GPIO: &[(u8, &str)] = &[
    (3, "strapping (JTAG source)"),
    (45, "strapping (VDD_SPI)"),
    (46, "strapping (ROM log)"),
    (38, "rejected for ADC ID by source; left spare"),
    (42, "spare"),
    (47, "spare"),
    (48, "spare"),
];

/// Main-board design rules derived in this phase.
pub const RULES: &[(&str, Status)] = &[
    ("SENSOR_DATA_ID carries the 100 k ID pull-down only. No ADC filter capacitor: the PUI mic SD output is rated for 140 pF total load.", Status::Proposal),
    ("Hold REG_PS_SYNC high (forced PWM, VOUT 3.267-3.333 V) for every ID measurement; PS mode can raise +3V3 by up to 5 % and push the T/RH ID voltage beyond the 2.9 V ADC range.", Status::Proposal),
    ("Power the chain (CHAIN_PWR_EN) only for scans; run CHAIN_INIT (all-zero image, latch) before releasing PATCH_OE_N after every power-up.", Status::Proposal),
    ("Before switching the chain off, drive PATCH_* and SENSOR_* low or Hi-Z and set SENSOR_PU_EN Hi-Z: the rev C HC595/HC4066 have no Ioff and would be back-powered through their input clamps (TMUX136 is power-off isolated).", Status::Proposal),
    ("Sensor-bus pull-ups: main-board 2.2 k driven from SENSOR_PU_EN in primary (SHT45) I²C mode only; port-side 2.2 k to the port VCC in AUX mode (rev C PULL_UP_RULES); none in I²S mode.", Status::Proposal),
    ("BQ24074 starts in USB100 (EN1 low until +3V3 exists), then USB500; ILIM mode (≈ 1 A) only after firmware detects a ≥ 1.5 A Type-C source (CD-4 / D-10).", Status::Proposal),
    ("Display uses a different connector family than AUX (pin orders differ; cross-mating would reverse power).", Status::Proposal),
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn gpio_map_rules() {
        let mut g = HashSet::new();
        for a in GPIO_MAP {
            assert!(g.insert(a.gpio), "duplicate GPIO{}", a.gpio);
            assert!(
                !RESERVED_GPIO.iter().any(|r| r.0 == a.gpio),
                "GPIO{} reserved",
                a.gpio
            );
        }
        let id = GPIO_MAP.iter().find(|a| a.net == "SENSOR_DATA_ID").unwrap();
        assert!((1..=10).contains(&id.gpio), "ADC1 = GPIO1..10");
        assert_ne!(id.gpio, 38);
        // Module pins exist on WROOM-1 (no GPIO22-34 except 33/34 internal on octal parts).
        assert!(GPIO_MAP.iter().all(|a| !(22..=34).contains(&a.gpio)));
        // four patch-control signals present
        for n in [
            "PATCH_SHIFT_DATA",
            "PATCH_SHIFT_CLK",
            "PATCH_LATCH",
            "PATCH_OE_N",
        ] {
            assert!(GPIO_MAP.iter().any(|a| a.net == n));
        }
    }
}
