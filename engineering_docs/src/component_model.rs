//! Shared types for the component-engineering phase (rev C, 2026-11-02).
//!
//! These types extend [`crate::model`] with component records. Every record names the
//! datasheet it was checked against ([`DatasheetId`]) and the assumptions that still apply.
//! A component can only be marked [`EngStatus::Recommended`] by engineering; client approval
//! is a separate step recorded in [`crate::governance`].

/// Key into [`crate::datasheets::DATASHEETS`].
pub type DatasheetId = &'static str;

/// V1-to-V2 compatibility classification (client-defined categories).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Compat {
    /// Compatible with V2: retain unchanged.
    Retain,
    /// Compatible with modifications: revise (value, variant, configuration or role).
    Revise,
    /// Incompatible with V2: replace (or remove when the V2 scope has no such function).
    Replace,
    /// Function did not exist in V1: selected new for V2.
    NewForV2,
}

/// Engineering status of a V2 selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EngStatus {
    /// Datasheet-verified and engineering-recommended. Awaits client sign-off only.
    Recommended,
    /// Recommended, but a named bench/drawing check must pass before release.
    RecommendedWithCondition,
    /// Selected function and value, exact MPN still to be assigned (e.g. commodity passives, AVL).
    MpnPending,
    /// Candidate whose datasheet could not be (re)verified in this phase.
    Unverified,
    /// The current candidate is not adequate; an alternative is required.
    NeedsAlternative,
    /// Depends on a client decision (see [`crate::eng_verification::DECISIONS`]).
    ClientDecision,
}

/// Lifecycle status as read from the manufacturer document / orderable addendum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lifecycle {
    /// Manufacturer lists the orderable part as Active / in production.
    Active,
    /// Manufacturer lists the series as current but the orderable lifecycle was not confirmed.
    NotConfirmed,
    /// End of life / not recommended for new designs.
    Eol,
}

/// Which PCB a part sits on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Board {
    /// Main controller PCB (one per system).
    Main,
    /// Universal sensor patch (up to eight per system).
    Patch,
    /// Off-board item (battery, cable, FFC).
    OffBoard,
}

/// Assembly population rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Populate {
    /// Always fitted.
    Fit,
    /// Do not install (footprint only).
    Dni,
    /// Fitted only on the T/RH patch variant.
    TrhVariant,
    /// Fitted only on the I²S microphone patch variant.
    MicVariant,
    /// Fit or DNI per approved AUX module profile.
    PerModuleProfile,
}

/// One physical pin of a part and the V2 net it connects to.
#[derive(Debug, Clone, Copy)]
pub struct Pin {
    /// Physical pin / pad number from the manufacturer drawing.
    pub number: &'static str,
    /// Manufacturer pin name.
    pub name: &'static str,
    /// V2 net name (or `NC`).
    pub net: &'static str,
}

/// How a pin map / footprint was verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinVerification {
    /// Read from the manufacturer pin-function table in the archived datasheet.
    DatasheetTable,
    /// Read from the manufacturer drawing (graphic) in the archived datasheet.
    DatasheetDrawing,
    /// Carried from V1 evidence; not re-verified in this phase (download failed).
    CarriedFromV1,
    /// Not yet verified.
    Pending,
}

/// A named electrical rating or characteristic, copied from the datasheet.
pub type Rating = (&'static str, &'static str);
