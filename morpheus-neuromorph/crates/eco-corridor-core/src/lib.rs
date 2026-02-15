#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Logical identifier for a living Indigenous eco-corridor, not a static border.
/// This must be non-empty and resolvable in the corridor/ALN registry (checked at runtime
/// by higher layers; here it is structurally non-optional). [file:4][file:3]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CorridorId(pub String);

impl CorridorId {
    pub fn new(id: impl Into<String>) -> Self {
        let s = id.into();
        assert!(
            !s.trim().is_empty(),
            "CorridorId must be non-empty and resolvable (no-corridor, no-build invariant)."
        );
        Self(s)
    }
}

/// Eco-impact metrics over soil, water, microbiomes, and biodiversity.
/// All values are normalized to 0.0–1.0 where 1.0 is least harm / best observed state,
/// matching your SNC EcoImpact patterns. [file:4][file:3]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcoImpactMetrics {
    pub soil_score: f32,
    pub water_score: f32,
    pub microbiome_score: f32,
    pub biodiversity_score: f32,
}

impl EcoImpactMetrics {
    pub fn new(soil: f32, water: f32, micro: f32, bio: f32) -> Self {
        fn clamp01(x: f32) -> f32 {
            if x < 0.0 {
                0.0
            } else if x > 1.0 {
                1.0
            } else {
                x
            }
        }

        Self {
            soil_score: clamp01(soil),
            water_score: clamp01(water),
            microbiome_score: clamp01(micro),
            biodiversity_score: clamp01(bio),
        }
    }

    /// Simple aggregate, used only for advisory scoring / classification.
    /// This must NEVER be used to drive actuators or automatic land-use changes. [file:4][file:1]
    pub fn aggregate(&self) -> f32 {
        (self.soil_score
            + self.water_score
            + self.microbiome_score
            + self.biodiversity_score)
            / 4.0
    }
}

/// FPIC / Indigenous Data Sovereignty status, treated as a mandatory, schema-level
/// precondition for any use of corridor-linked data. [file:4][file:3]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FpicStatus {
    /// No FPIC decision or scope established for this corridor/context.
    Pending,
    /// Explicit FPIC granted, with optional scope / conditions reference.
    Granted {
        /// External reference (e.g., ALN shard / ledger entry) describing scope,
        /// conditions, and revocation rules. [file:4]
        consent_ref: String,
    },
    /// FPIC withheld or revoked, with human-readable reason / link.
    Withheld {
        reason: String,
    },
}

/// Minimal neurorights constraint capsule for corridor-linked knowledge objects.
/// Importantly, this is **purely declarative** metadata: it has no methods that
/// change state or actuate anything. It only labels obligations. [file:3][file:1]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeurorightsConstraints {
    /// Mental privacy: no covert inference from corridor data into inner mental state. [file:4][file:3]
    pub mental_privacy_protection: bool,
    /// Explicit prohibition on coercive neuromorphic channels linked to this corridor. [file:4][file:3]
    pub forbid_coercive_channels: bool,
    /// Explicit prohibition on downgrades/rollbacks of rights or capabilities as a result
    /// of any analysis of this corridor. [file:3][file:1]
    pub forbid_downgrade_or_rollback: bool,
    /// True iff any discipline (FEAR/PAIN) signals tied to this corridor are explicitly
    /// voluntary, labeled, and non-propagating to non-participants. [file:4][file:3]
    pub discipline_personalized_and_noncoercive: bool,
}

impl NeurorightsConstraints {
    /// Helper to construct a "strict floor" neurorights capsule, suitable as a
    /// default for Indigenous Eco-Corridor Map objects. [file:4][file:3]
    pub fn strict_floor() -> Self {
        Self {
            mental_privacy_protection: true,
            forbid_coercive_channels: true,
            forbid_downgrade_or_rollback: true,
            discipline_personalized_and_noncoercive: true,
        }
    }
}

/// Core, non-actuating Indigenous Eco-Corridor record.
/// This is an observational/advisory knowledge object only:
/// - It encodes corridor identity, eco-impact metrics, FPIC/IDS status,
///   and neurorights constraints as **mandatory** fields.
/// - It exposes no actuation or control methods (no device or policy control).
/// - Higher-risk layers (e.g., Tribal Survival Protocol Envelope) must treat this
///   as read-only input, never as a controller. [file:4][file:3][file:1]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndigenousEcoCorridorRecord {
    pub corridor_id: CorridorId,
    pub eco_impact: EcoImpactMetrics,
    pub fpic_status: FpicStatus,
    pub neurorights: NeurorightsConstraints,
    /// Optional Facecloud / cultural knowledge reference, co-authored and governed
    /// externally; presence here never authorizes actuation. [file:4]
    pub facecloud_ref: Option<String>,
}

impl IndigenousEcoCorridorRecord {
    /// Constructor enforces presence of all core fields at the type level.
    /// Any missing corridor ID, EcoImpact, FPIC, or neurorights capsule is
    /// unrepresentable as a valid record. [file:3][file:4]
    pub fn new(
        corridor_id: CorridorId,
        eco_impact: EcoImpactMetrics,
        fpic_status: FpicStatus,
        neurorights: NeurorightsConstraints,
        facecloud_ref: Option<String>,
    ) -> Self {
        Self {
            corridor_id,
            eco_impact,
            fpic_status,
            neurorights,
            facecloud_ref,
        }
    }

    /// Purely advisory classification helper, suitable for dashboards or audits.
    /// This MUST NOT be wired to any automatic enforcement or actuation path. [file:4][file:1]
    pub fn advisory_risk_label(&self) -> &'static str {
        let eco = self.eco_impact.aggregate();

        match (&self.fpic_status, eco) {
            (FpicStatus::Withheld { .. }, _) => "blocked_fpic_withheld",
            (FpicStatus::Pending, _) => "hold_fpic_pending",
            (FpicStatus::Granted { .. }, e) if e >= 0.8 => "low_risk_observational",
            (FpicStatus::Granted { .. }, e) if e >= 0.5 => "medium_risk_review",
            _ => "high_risk_review",
        }
    }
}

/// Read-only trait for SNC / ALN governance layers to consume corridor records
/// as inputs to policy predicates (e.g., `no corridor, no build`, FPIC gates,
/// neurorights floors). It exposes **no mutating or actuating methods**. [file:3][file:4]
pub trait EcoCorridorView {
    fn corridor_id(&self) -> &CorridorId;
    fn eco_impact(&self) -> &EcoImpactMetrics;
    fn fpic_status(&self) -> &FpicStatus;
    fn neurorights(&self) -> &NeurorightsConstraints;
    fn advisory_risk_label(&self) -> &'static str;
}

impl EcoCorridorView for IndigenousEcoCorridorRecord {
    fn corridor_id(&self) -> &CorridorId {
        &self.corridor_id
    }

    fn eco_impact(&self) -> &EcoImpactMetrics {
        &self.eco_impact
    }

    fn fpic_status(&self) -> &FpicStatus {
        &self.fpic_status
    }

    fn neurorights(&self) -> &NeurorightsConstraints {
        &self.neurorights
    }

    fn advisory_risk_label(&self) -> &'static str {
        self.advisory_risk_label()
    }
}
