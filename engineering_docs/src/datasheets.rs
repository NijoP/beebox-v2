//! # Datasheet register (evidence for the component-engineering phase)
//!
//! Every numeric value in [`crate::component_specs`], [`crate::passives`] and
//! [`crate::power_budget`] is traceable to one of these manufacturer documents. Archived copies
//! live in `bom/datasheets/` (added 2026-11-02; existing files were not overwritten).
//! Marketplace pages were **not** used as technical evidence.
//!
//! Documents that could not be downloaded in this phase are listed with `local: None` and their
//! dependent records carry [`crate::component_model::EngStatus::Unverified`].

/// A manufacturer document.
#[derive(Debug, Clone, Copy)]
pub struct Datasheet {
    /// Register key.
    pub id: &'static str,
    /// Manufacturer.
    pub manufacturer: &'static str,
    /// Document title / number.
    pub document: &'static str,
    /// Revision or version as printed.
    pub revision: &'static str,
    /// Archived copy (project-relative), if downloaded.
    pub local: Option<&'static str>,
    /// Manufacturer URL.
    pub url: &'static str,
}

/// Datasheet register.
pub const DATASHEETS: &[Datasheet] = &[
    Datasheet { id: "DS-ESP-MOD", manufacturer: "Espressif", document: "ESP32-S3-WROOM-1 & WROOM-1U Datasheet", revision: "v1.8", local: Some("bom/datasheets/Espressif_ESP32-S3-WROOM-1_datasheet_v1.8.pdf"), url: "https://www.espressif.com/sites/default/files/documentation/esp32-s3-wroom-1_wroom-1u_datasheet_en.pdf" },
    Datasheet { id: "DS-ESP-SOC", manufacturer: "Espressif", document: "ESP32-S3 Series Datasheet", revision: "v2.2", local: Some("bom/datasheets/Espressif_ESP32-S3_datasheet.pdf"), url: "https://www.espressif.com/sites/default/files/documentation/esp32-s3_datasheet_en.pdf" },
    Datasheet { id: "DS-ESP-HWDG", manufacturer: "Espressif", document: "ESP32-S3 Hardware Design Guidelines", revision: "latest (2026-11-02)", local: Some("bom/datasheets/Espressif_ESP32-S3_hardware_design_guidelines.pdf"), url: "https://docs.espressif.com/projects/esp-hardware-design-guidelines/en/latest/esp32s3/" },
    Datasheet { id: "DS-BQ24074", manufacturer: "Texas Instruments", document: "BQ2407x SLUS810", revision: "N (Oct 2021)", local: Some("bom/datasheets/TI_BQ24074_SLUS810.pdf"), url: "https://www.ti.com/lit/ds/symlink/bq24074.pdf" },
    Datasheet { id: "DS-TPS6302X", manufacturer: "Texas Instruments", document: "TPS63020/TPS63021 SLVS916", revision: "I (Oct 2019)", local: Some("bom/datasheets/TI_TPS63020_TPS63021_SLVS916I.pdf"), url: "https://www.ti.com/lit/ds/symlink/tps63020.pdf" },
    Datasheet { id: "DS-TPS255X", manufacturer: "Texas Instruments", document: "TPS2552/TPS2553 SLVS841", revision: "G (Oct 2026)", local: Some("bom/datasheets/TI_TPS2552_TPS2553.pdf"), url: "https://www.ti.com/lit/ds/symlink/tps2553.pdf" },
    Datasheet { id: "DS-TPS22918", manufacturer: "Texas Instruments", document: "TPS22918 SLVSD76", revision: "C (Jul 2017)", local: Some("bom/datasheets/TI_TPS22918_SLVSD76C.pdf"), url: "https://www.ti.com/lit/ds/symlink/tps22918.pdf" },
    Datasheet { id: "DS-LV595A", manufacturer: "Texas Instruments", document: "SN74LV595A", revision: "as downloaded 2026-11-02", local: Some("bom/datasheets/TI_SN74LV595A.pdf"), url: "https://www.ti.com/lit/ds/symlink/sn74lv595a.pdf" },
    Datasheet { id: "DS-HC595", manufacturer: "Texas Instruments", document: "SN74HC595", revision: "as downloaded 2026-11-02", local: Some("bom/datasheets/TI_SN74HC595.pdf"), url: "https://www.ti.com/lit/ds/symlink/sn74hc595.pdf" },
    Datasheet { id: "DS-TMUX1511", manufacturer: "Texas Instruments", document: "TMUX1511 SCDS390", revision: "B (Mar 2025)", local: Some("bom/datasheets/TI_TMUX1511_SCDS390B.pdf"), url: "https://www.ti.com/lit/ds/symlink/tmux1511.pdf" },
    Datasheet { id: "DS-CB3Q3253", manufacturer: "Texas Instruments", document: "SN74CB3Q3253", revision: "as downloaded 2026-11-02", local: Some("bom/datasheets/TI_SN74CB3Q3253.pdf"), url: "https://www.ti.com/lit/ds/symlink/sn74cb3q3253.pdf" },
    Datasheet { id: "DS-TMUX1209", manufacturer: "Texas Instruments", document: "TMUX1208/TMUX1209 SCDS389", revision: "C (Dec 2018)", local: Some("bom/datasheets/TI_TMUX1208_TMUX1209_SCDS389C.pdf"), url: "https://www.ti.com/lit/ds/symlink/tmux1209.pdf" },
    Datasheet { id: "DS-TMUX136", manufacturer: "Texas Instruments", document: "TMUX136 SCDS367", revision: "E (Jun 2023)", local: Some("bom/datasheets/TI_TMUX136.pdf"), url: "https://www.ti.com/lit/ds/symlink/tmux136.pdf" },
    Datasheet { id: "DS-TMUX1109", manufacturer: "Texas Instruments", document: "TMUX1108/TMUX1109", revision: "as downloaded 2026-11-01", local: Some("bom/datasheets/TI_TMUX1109.pdf"), url: "https://www.ti.com/lit/ds/symlink/tmux1109.pdf" },
    Datasheet { id: "DS-HC238", manufacturer: "Texas Instruments", document: "CD74HC138/CD74HC238 SCHS147", revision: "J (Nov 2021)", local: Some("bom/datasheets/TI_CD74HC238_138.pdf"), url: "https://www.ti.com/lit/ds/symlink/cd74hc238.pdf" },
    Datasheet { id: "DS-NONE-DIODE", manufacturer: "-", document: "Arm-sense diode datasheet", revision: "NOT DOWNLOADED (Nexperia/onsemi/Diodes blocked)", local: None, url: "-" },
    Datasheet { id: "DS-HC4066", manufacturer: "Texas Instruments", document: "CD74HC4066", revision: "as downloaded 2026-11-02", local: Some("bom/datasheets/TI_CD74HC4066.pdf"), url: "https://www.ti.com/lit/ds/symlink/cd74hc4066.pdf" },
    Datasheet { id: "DS-LVC138A", manufacturer: "Texas Instruments", document: "SN74LVC138A", revision: "as downloaded 2026-11-02", local: Some("bom/datasheets/TI_SN74LVC138A.pdf"), url: "https://www.ti.com/lit/ds/symlink/sn74lvc138a.pdf" },
    Datasheet { id: "DS-LVC1G10", manufacturer: "Texas Instruments", document: "SN74LVC1G10", revision: "as downloaded 2026-11-02", local: Some("bom/datasheets/TI_SN74LVC1G10.pdf"), url: "https://www.ti.com/lit/ds/symlink/sn74lvc1g10.pdf" },
    Datasheet { id: "DS-LVC1G04", manufacturer: "Texas Instruments", document: "SN74LVC1G04", revision: "as downloaded 2026-11-02", local: Some("bom/datasheets/TI_SN74LVC1G04.pdf"), url: "https://www.ti.com/lit/ds/symlink/sn74lvc1g04.pdf" },
    Datasheet { id: "DS-TPD4E05U06", manufacturer: "Texas Instruments", document: "TPDxE05U06 SLVSBO7", revision: "O (Aug 2024)", local: Some("bom/datasheets/TI_TPD4E05U06_SLVSBO7O.pdf"), url: "https://www.ti.com/lit/ds/symlink/tpd4e05u06.pdf" },
    Datasheet { id: "DS-PCA9546A", manufacturer: "Texas Instruments", document: "PCA9546A", revision: "as downloaded 2026-11-02", local: Some("bom/datasheets/TI_PCA9546A_rejected.pdf"), url: "https://www.ti.com/lit/ds/symlink/pca9546a.pdf" },
    Datasheet { id: "DS-TPS62162", manufacturer: "Texas Instruments", document: "TPS62160/TPS62162", revision: "as downloaded 2026-11-02", local: Some("bom/datasheets/TI_TPS62162.pdf"), url: "https://www.ti.com/lit/ds/symlink/tps62162.pdf" },
    Datasheet { id: "DS-SHT4X", manufacturer: "Sensirion", document: "SHT4x Datasheet", revision: "v7.3 (Jun 2026)", local: Some("bom/datasheets/Sensirion_SHT4x_Datasheet_v7.3.pdf"), url: "https://sensirion.com/resource/datasheet/sht4x" },
    Datasheet { id: "DS-SCD4X", manufacturer: "Sensirion", document: "SCD4x Datasheet", revision: "D1 (archived earlier)", local: Some("bom/datasheets/Sensirion_SCD4x_Datasheet_D1.pdf"), url: "https://sensirion.com/products/catalog/SCD41" },
    Datasheet { id: "DS-PUI-MIC", manufacturer: "PUI Audio", document: "DMM-4026-B-I2S-R Data Sheet", revision: "rev 5/26/2021 (I2S data added)", local: Some("bom/datasheets/PUI_DMM-4026-B-I2S-R_datasheet.pdf"), url: "https://api.puiaudio.com/filename/DMM-4026-B-I2S-R.pdf" },
    Datasheet { id: "DS-FH34-CAT", manufacturer: "Hirose", document: "FH34 series catalog", revision: "as downloaded 2026-11-02", local: Some("bom/datasheets/Hirose_FH34SRJ_series_catalog.pdf"), url: "https://www.hirose.com/en/product/series/FH34SRJ" },
    Datasheet { id: "DS-FH34-SPEC", manufacturer: "Hirose", document: "FH34SRJ-12S-0.5SH(50) product specification sheet (CL0580-1253-0-50)", revision: "as downloaded 2026-11-02", local: Some("bom/datasheets/Hirose_FH34SRJ-12S-0.5SH50_spec_sheet.pdf"), url: "https://www.hirose.com/en/product/p/CL0580-1253-0-50" },
    Datasheet { id: "DS-TPS61023", manufacturer: "Texas Instruments", document: "TPS61023 boost converter", revision: "as downloaded 2026-11-01 (rev D)", local: Some("bom/datasheets/TI_TPS61023.pdf"), url: "https://www.ti.com/lit/ds/symlink/tps61023.pdf" },
    Datasheet { id: "DS-TPS62840", manufacturer: "Texas Instruments", document: "TPS62840 step-down converter", revision: "as downloaded 2026-11-01 (rev D)", local: Some("bom/datasheets/TI_TPS62840.pdf"), url: "https://www.ti.com/lit/ds/symlink/tps62840.pdf" },
    Datasheet { id: "DS-TPS63802", manufacturer: "Texas Instruments", document: "TPS63802 buck-boost (alternative to TPS63021)", revision: "as downloaded 2026-11-01 (rev D)", local: Some("bom/datasheets/TI_TPS63802.pdf"), url: "https://www.ti.com/lit/ds/symlink/tps63802.pdf" },
    Datasheet { id: "DS-JST-GH", manufacturer: "JST", document: "GH connector catalog", revision: "NOT DOWNLOADED (JST blocks automated download, HTTP 403)", local: None, url: "https://www.jst-mfg.com/product/pdf/eng/eGH.pdf" },
    Datasheet { id: "DS-JST-PH", manufacturer: "JST", document: "PH connector catalog (ePH)", revision: "as downloaded 2026-11-02", local: Some("bom/datasheets/JST_PH_connector_catalog.pdf"), url: "https://www.jst-mfg.com/product/pdf/eng/ePH.pdf" },
    Datasheet { id: "DS-GCT-USB4105", manufacturer: "GCT", document: "USB4105 drawing", revision: "as downloaded 2026-11-02", local: Some("bom/datasheets/GCT_USB4105_drawing.pdf"), url: "https://gct.co/files/drawings/usb4105.pdf" },
    Datasheet { id: "DS-MAX17048", manufacturer: "Analog Devices", document: "MAX17048/MAX17049", revision: "NOT DOWNLOADED (connection reset)", local: None, url: "https://www.analog.com/media/en/technical-documentation/data-sheets/MAX17048-MAX17049.pdf" },
    Datasheet { id: "DS-USBLC6", manufacturer: "STMicroelectronics", document: "USBLC6-2", revision: "NOT DOWNLOADED (timeout)", local: None, url: "https://www.st.com/resource/en/datasheet/usblc6-2.pdf" },
    Datasheet { id: "DS-JST-SH", manufacturer: "JST", document: "SH connector catalog", revision: "NOT DOWNLOADED (not attempted after JST block)", local: None, url: "https://www.jst-mfg.com/product/pdf/eng/eSH.pdf" },
    Datasheet { id: "DS-UM10204", manufacturer: "NXP", document: "UM10204 I2C-bus specification", revision: "Rev 7 (referenced, not archived)", local: None, url: "https://www.nxp.com/docs/en/user-guide/UM10204.pdf" },
    Datasheet { id: "DS-V1-REPO", manufacturer: "BeeBox V1 project", document: "NijoP/BeeBox repository (BOM, docs 03/18/19/23/24, netlist)", revision: "main @ clone 2026-11-01", local: None, url: "https://github.com/NijoP/BeeBox" },
];

/// Find a datasheet by id.
pub fn find(id: &str) -> Option<&'static Datasheet> {
    DATASHEETS.iter().find(|d| d.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn ids_unique_and_local_paths_in_bom_folder() {
        let mut s = HashSet::new();
        for d in DATASHEETS {
            assert!(s.insert(d.id), "duplicate {}", d.id);
            if let Some(p) = d.local {
                assert!(p.starts_with("bom/datasheets/"), "{}", d.id);
            }
        }
    }
}
