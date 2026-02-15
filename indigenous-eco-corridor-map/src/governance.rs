//! FPIC/IDS compliance status and auditable governance fields.
//! Non-actuating; ledgers live in other crates.

#![forbid(unsafe_code)]

use std::time::SystemTime;

/// Minimal community identifier for ALN / DID / IDS ledgers.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CommunityId(pub String);

/// FPIC status for a given corridor and use-case.
#[derive(Clone, Debug)]
pub enum FpicStatus {
    Pending,
    /// Granted under specific terms, tied to community IDs and time.
    Granted {
        at: SystemTime,
        communities: Vec<CommunityId>,
        terms_reference: String, // e.g., hash/URI into an FPIC ledger
    },
    /// Withheld or revoked, with explanation.
    Withheld {
        at: SystemTime,
        reason: String,
    },
}

impl FpicStatus {
    pub fn is_active_grant(&self) -> bool {
        matches!(self, FpicStatus::Granted { .. })
    }
}

/// Indigenous Data Sovereignty (IDS) scope tags for this corridor.
#[derive(Clone, Debug)]
pub struct IdsScope {
    /// Whether Indigenous data is present at all.
    pub contains_indigenous_data: bool,
    /// True if governed under an Indigenous data sovereignty framework
    /// (e.g., OCAP, CARE); ledger details live in overlays.
    pub governed_by_ids_framework: bool,
    /// Optional reference to IDS governance shard / ledger entry.
    pub governance_ref: Option<String>,
}
