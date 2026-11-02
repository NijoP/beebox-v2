//! Provenance: primary source document, client instructions, external references and
//! the legacy Markdown knowledge base.
//!
//! # Authority order
//! 1. Explicit client instructions ([`CLIENT_INSTRUCTIONS`]).
//! 2. Primary source document [`PRIMARY_DOCUMENT`].
//! 3. Engineering analysis in this crate (always marked [`crate::model::Source::Analysis`]).
//!
//! The Markdown knowledge base under `docs/` (created 01 Nov 2026) is **superseded** by this
//! crate. It is kept unmodified as history. Where it contained derived claims, they were
//! re-reviewed here rather than copied blindly.

/// Metadata of the primary source document.
#[derive(Debug, Clone, Copy)]
pub struct SourceDocument {
    /// Document identifier.
    pub id: &'static str,
    /// Revision label.
    pub revision: &'static str,
    /// Issue date.
    pub date: &'static str,
    /// Title.
    pub title: &'static str,
    /// Project-relative path of the archived copy.
    pub path: &'static str,
    /// Release status declared by the document.
    pub declared_status: &'static str,
}

/// BBX-ENG-PR-003 Rev 0.2 - the primary source of truth.
pub const PRIMARY_DOCUMENT: SourceDocument = SourceDocument {
    id: "BBX-ENG-PR-003",
    revision: "0.2 - Expanded engineering baseline",
    date: "2026-11-01",
    title: "BeeBox V2 End-to-End Project Report",
    path: "docs/00_source/BBX-ENG-PR-003_Rev0.2_BeeBox_V2_End_to_End_Project_Report.pdf",
    declared_status:
        "Engineering proposal - conditional GO for prototype; not released for manufacture",
};

/// A client instruction that overrides or extends the source document.
#[derive(Debug, Clone, Copy)]
pub struct ClientInstruction {
    /// Date given.
    pub date: &'static str,
    /// Content summary.
    pub summary: &'static str,
}

/// Client instructions received so far.
pub const CLIENT_INSTRUCTIONS: &[ClientInstruction] = &[
    ClientInstruction {
        date: "2026-11-01",
        summary: "REV D DECISIONS: APR-1 option B approved subject to power validation (+5V_CHAIN on pin 2, local switching \
                  regulator +3V3_LOCAL, 400 mA provisional contact ceiling); APR-2 JST-GH 1.25 mm 4-pin VCC-GND-SDA-SCL; \
                  APR-3 qualified 3.3 V I2C breakouts only, CO2 first, methane = background ppm, no methane hardware until a \
                  verified module exists; APR-4 provisional context (USB-C charging, Wi-Fi/BLE, 1 min sampling, configurable \
                  audio, 100 mm segments, manufacturer-neutral). CONFIRMED: 3000 mAh rechargeable LiPo main battery.",
    },
    ClientInstruction {
        date: "2026-11-01",
        summary: "Treat the PDF as the primary source of truth; build a persistent knowledge base; no EasyEDA modification.",
    },
    ClientInstruction {
        date: "2026-11-01",
        summary: "Confirmed: one ESP32-S3 main controller; up to 8 passive patches; no MCU/battery on patches; \
                  one primary sensor (T/RH or I2S mic) per patch; FOUR AUX headers (AUX1-AUX4) on EVERY patch for \
                  external modules such as methane, CO2 and other user-selected sensors; 12-contact chain is the \
                  baseline; preserve shift-register cascade and patch ID unless analysis shows a necessary change. \
                  Documentation as a Rust crate (cargo doc), no Markdown, no schematic/PCB/firmware yet.",
    },
    ClientInstruction {
        date: "2026-11-01",
        summary: "CORRECTION: AUX1-AUX4 are standard 4-pin I2C headers (1 VCC, 2 GND, 3 SDA, 4 SCL) for ready-made I2C                   breakouts (CO2, methane, other). No generic analog interface, no technology-specific connectors.                   Only verified-compatible modules supported. Frozen: single active path, outputs OFF at reset/fault,                   no 5 V to ESP32-S3, no SPI on AUX, no GPIO38 for ID.",
    },
    ClientInstruction {
        date: "2026-11-02",
        summary: "Component-engineering phase: audit V1 (NijoP/BeeBox), select and verify all V2 components against                   manufacturer datasheets, calculate passives/power/drop, document in this crate. Do not modify EasyEDA;                   do not start schematic generation until this phase is reviewed.",
    },
];

/// External reference.
#[derive(Debug, Clone, Copy)]
pub struct Reference {
    /// Tag used in the source document, e.g. `R3`.
    pub tag: &'static str,
    /// Title.
    pub title: &'static str,
    /// URL.
    pub url: &'static str,
}

/// References R1-R12 carried forward from the source (p.20). Not yet downloaded or reviewed.
pub const REFERENCES: &[Reference] = &[
    Reference { tag: "R1", title: "BeeBox repository (V1/V2 source documents)", url: "https://github.com/NijoP/BeeBox" },
    Reference { tag: "R2", title: "Hirose FH34 series", url: "https://www.hirose.com/en/product/p/CL0580-1253-0-50" },
    Reference { tag: "R3", title: "TI SN74HC595", url: "https://www.ti.com/product/SN74HC595" },
    Reference { tag: "R4", title: "TI CD74HC4066", url: "https://www.ti.com/product/CD74HC4066" },
    Reference { tag: "R5", title: "TI TPS22918", url: "https://www.ti.com/product/TPS22918" },
    Reference { tag: "R6", title: "ESP32-S3 ADC programming guide", url: "https://docs.espressif.com/projects/esp-idf/en/latest/esp32s3/api-reference/peripherals/adc/index.html" },
    Reference { tag: "R7", title: "ESP32-S3 I2S programming guide", url: "https://docs.espressif.com/projects/esp-idf/en/latest/esp32s3/api-reference/peripherals/i2s.html" },
    Reference { tag: "R8", title: "Sensirion SHT45", url: "https://sensirion.com/products/catalog/SHT45" },
    Reference { tag: "R9", title: "TDK InvenSense ICS-43434 (EOL)", url: "https://www.invensense.tdk.com/en-us/products/microphone/ics-43434" },
    Reference { tag: "R10", title: "PUI Audio DMM-4026-B-I2S-R", url: "https://puiaudio.com/product/microphones/dmm-4026-b-i2s-r" },
    Reference { tag: "R11", title: "PUI Audio DMM-4026-B-I2S evaluation document", url: "https://api.puiaudio.com/filename/DMM-4026-B-I2S-EB-R.pdf" },
    Reference { tag: "R12", title: "Mill-Max 0906-series catalog", url: "https://www.mill-max.com/sites/default/files/external/catalog/2021-02/023-028.pdf" },
];

/// Review findings about the earlier (agent-generated) Markdown knowledge base.
///
/// Recorded so that later readers do not re-import unreviewed claims.
pub const LEGACY_KB_REVIEW: &[&str] = &[
    "OQ-002 (four headers per patch?) is CLOSED by client instruction 2026-11-01: four AUX headers on every patch.",
    "Gas-sensor power/warm-up figures in docs/14 were indicative class values without datasheets; retained here only as Quantity::Indicative.",
    "The 'electrochemical modules are usually <= tens of mA' statement is unverified and is not used for any decision.",
    "The PDF lists 'methane, CO'; the client instruction lists 'methane, CO2'. Latest client instruction names methane and CO2; CO is retained only as source context.",
    "API bridge status in the PDF ('retrying') is outdated: handshake and read-only query verified 2026-11-01 (see phases::TOOLING_STATUS).",
];
