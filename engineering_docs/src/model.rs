//! Shared typed vocabulary used by every documentation module.
//!
//! Every engineering statement in this crate carries a [`Status`] and at least one
//! [`Source`], so a reader can always tell *who said it* and *how settled it is*.

/// How settled a statement is.
///
/// Ordered roughly from most to least authoritative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Status {
    /// Explicitly confirmed by the client (project owner) instruction.
    ClientConfirmed,
    /// Stated as a baseline requirement/decision in BBX-ENG-PR-003 Rev 0.2.
    SourceBaseline,
    /// Engineering proposal by the documentation author. **Not approved.** Must not be
    /// implemented until it appears as [`Status::Approved`].
    Proposal,
    /// Proposal that has been explicitly approved by the client.
    Approved,
    /// Working assumption. Must be validated or replaced.
    Assumption,
    /// The source says it is open / TBD, and no proposal is made yet.
    Open,
    /// Candidate part or concept named in the source. Not qualified and not selected.
    Candidate,
    /// Explicitly rejected (e.g. EOL part, unacceptable GPIO).
    Rejected,
    /// Replaced by a newer statement.
    Superseded,
}

/// Where a statement comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Source {
    /// Page of the primary source document BBX-ENG-PR-003 Rev 0.2.
    Pdf {
        /// 1-based page number.
        page: u8,
    },
    /// Client instruction, identified by the date it was given.
    Client {
        /// ISO date of the instruction.
        date: &'static str,
    },
    /// Engineering analysis by the documentation author (derived, reviewable, not authoritative).
    Analysis,
    /// Direct observation of the tool environment (e.g. EasyEDA bridge query).
    Observation {
        /// ISO date of the observation.
        date: &'static str,
    },
}

/// Priority / severity scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Priority {
    /// Blocks schematic capture of the affected block.
    Blocking,
    /// Critical technical risk (source terminology).
    Critical,
    /// Needed before PCB / prototype.
    High,
    /// Needed before release.
    Medium,
    /// Nice to settle.
    Low,
}

/// A requirement statement.
#[derive(Debug, Clone, Copy)]
pub struct Requirement {
    /// Stable identifier, e.g. `REQ-AUX-01`.
    pub id: &'static str,
    /// Short title.
    pub title: &'static str,
    /// Normative statement ("shall").
    pub statement: &'static str,
    /// How settled this is.
    pub status: Status,
    /// Provenance.
    pub sources: &'static [Source],
    /// Verification IDs (see [`crate::verification`]) that close this requirement.
    pub verified_by: &'static [&'static str],
}

/// An engineering decision or proposal.
#[derive(Debug, Clone, Copy)]
pub struct Decision {
    /// Stable identifier, e.g. `DEC-013`.
    pub id: &'static str,
    /// Topic.
    pub topic: &'static str,
    /// The decision / proposal text.
    pub statement: &'static str,
    /// Settledness.
    pub status: Status,
    /// Why.
    pub rationale: &'static str,
    /// Provenance.
    pub sources: &'static [Source],
}

/// One selectable option of a decision awaiting client approval.
#[derive(Debug, Clone, Copy)]
pub struct DecisionOption {
    /// Option label, e.g. `"A"`.
    pub label: &'static str,
    /// What the option is.
    pub description: &'static str,
    /// Technical / project consequences of choosing it.
    pub consequences: &'static str,
}

/// A risk register entry.
#[derive(Debug, Clone, Copy)]
pub struct Risk {
    /// Stable identifier.
    pub id: &'static str,
    /// Risk title.
    pub title: &'static str,
    /// Impact if it materialises.
    pub impact: &'static str,
    /// Mitigation / closure evidence.
    pub mitigation: &'static str,
    /// Severity.
    pub priority: Priority,
    /// Who identified it.
    pub origin: Source,
}

/// An assumption register entry.
#[derive(Debug, Clone, Copy)]
pub struct Assumption {
    /// Stable identifier.
    pub id: &'static str,
    /// The assumption.
    pub statement: &'static str,
    /// Who made it.
    pub origin: Source,
    /// How it will be validated or retired.
    pub validation: &'static str,
}

/// A quantity that may not be known yet. Prevents placeholder numbers being mistaken for data.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Quantity {
    /// Known value with unit and provenance.
    Known {
        /// Numeric value.
        value: f64,
        /// Unit string (SI).
        unit: &'static str,
        /// Provenance.
        source: Source,
    },
    /// Indicative class value - **not** datasheet-backed. Never use for design sign-off.
    Indicative {
        /// Lower bound.
        min: f64,
        /// Upper bound.
        max: f64,
        /// Unit string.
        unit: &'static str,
    },
    /// To be determined; the string names what closes it.
    Tbd(&'static str),
}
