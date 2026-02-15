#![forbid(unsafe_code)]

use std::time::SystemTime;

/// CorridorId: DID-like, non-empty, validated at construction.
/// This is the anchor for an Indigenous eco-corridor identity.[file:3][file:4]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CorridorId(String);

impl CorridorId {
    /// Create a CorridorId from a string-like value, enforcing simple
    /// non-empty and prefix rules. You can later strengthen this to full DID
    /// validation without changing call sites.[file:3]
    pub fn from_str(value: &str) -> Result<Self, String> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err("CorridorId must not be empty".into());
        }
        // Minimal DID-style guard: ensure there is at least one ':' separator.
        if !trimmed.contains(':') {
            return Err("CorridorId must contain a DID-style prefix (e.g. did:...)".into());
        }
        Ok(CorridorId(trimmed.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Normalized scalar in [0.0, 1.0]. 1.0 = best (least harm / highest integrity).[file:3][file:4]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EcoScalar(f32);

impl EcoScalar {
    pub fn new(value: f32) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&value) {
            return Err("EcoScalar must be within [0.0, 1.0]".into());
        }
        Ok(EcoScalar(value))
    }

    pub fn value(self) -> f32 {
        self.0
    }
}

/// EcoImpactMetrics: biophysical heartbeat of the corridor.[file:3][file:4]
/// All fields are normalized scores in [0,1] where 1.0 is healthiest / least harmful.
#[derive(Clone, Debug)]
pub struct EcoImpactMetrics {
    pub soil_health: EcoScalar,
    pub water_quality: EcoScalar,
    pub microbiome_diversity: EcoScalar,
    /// Composite resilience score (optional, but useful for policy thresholds).
    pub corridor_resilience: EcoScalar,
}

impl EcoImpactMetrics {
    /// Simple aggregate; you can replace with weighted or corridor-specific logic later.
    pub fn aggregate_score(&self) -> EcoScalar {
        let s = self.soil_health.value()
            + self.water_quality.value()
            + self.microbiome_diversity.value()
            + self.corridor_resilience.value();
        // Average of four metrics.
        EcoScalar((s / 4.0).clamp(0.0, 1.0))
    }
}

/// Status of a verifiable consent credential (FPIC / IDS).[file:3][file:4]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConsentStatus {
    Granted,
    Revoked,
    Pending,
}

/// Minimal verifiable consent capsule, aligned with W3C VC patterns.[file:3]
#[derive(Clone, Debug)]
pub struct VerifiableConsent {
    /// DID of issuing authority (e.g., Indigenous council).
    pub issuer_did: String,
    /// Subject corridor id (string form) to which this consent applies.
    pub subject_corridor_id: String,
    /// Current status of consent.
    pub status: ConsentStatus,
    /// Issuance timestamp.
    pub issued_at: SystemTime,
    /// Optional revocation timestamp.
    pub revoked_at: Option<SystemTime>,
    /// Detached signature or reference; to be checked by a ledger / VC layer.
    pub signature_hex: String,
}

impl VerifiableConsent {
    /// Quick helper: true only if status is Granted and not revoked yet.
    pub fn is_effectively_granted(&self) -> bool {
        self.status == ConsentStatus::Granted && self.revoked_at.is_none()
    }
}

/// Neurorights-sensitive capabilities within the corridor.[file:3][file:1]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NeurorightsFlag {
    /// Disallow any use of FEAR / PAIN as coercive channels.
    ForbidFearPainCoercion,
    /// Disallow mental manipulation or belief-shaping without explicit, sovereign consent.
    NoMentalManipulation,
    /// Disallow covert inference of mental states from telemetry.
    NoCovertInference,
}

/// Simple bitset-style container for neurorights constraints.[file:3]
#[derive(Clone, Debug, Default)]
pub struct NeurorightsFlags {
    inner: Vec<NeurorightsFlag>,
}

impl NeurorightsFlags {
    pub fn new(flags: Vec<NeurorightsFlag>) -> Self {
        NeurorightsFlags { inner: flags }
    }

    pub fn contains(&self, flag: &NeurorightsFlag) -> bool {
        self.inner.iter().any(|f| f == flag)
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &NeurorightsFlag> {
        self.inner.iter()
    }
}

/// FPIC / IDS gate for corridor operations. Optional: some corridors may
/// be in a pre-consultation state, but high-impact actions must check for
/// a `Some` value with `ConsentStatus::Granted` and cryptographic validity.[file:3][file:4]
pub type FpicIdsState = Option<VerifiableConsent>;

/// Foundational, non-actuating, machine-checkable kernel.[file:3][file:4]
#[derive(Clone, Debug)]
pub struct IndigenousEcoCorridorMap {
    pub corridor_id: CorridorId,
    pub eco_metrics: EcoImpactMetrics,
    pub fpic_ids_state: FpicIdsState,
    pub neurorights_flags: NeurorightsFlags,
}

impl IndigenousEcoCorridorMap {
    /// Constructor enforces that corridor_id is present and eco_metrics are valid.
    /// Neurorights flags may be empty, but high-impact policies should typically
    /// require at least some constraints here.[file:3][file:4]
    pub fn new(
        corridor_id: CorridorId,
        eco_metrics: EcoImpactMetrics,
        fpic_ids_state: FpicIdsState,
        neurorights_flags: NeurorightsFlags,
    ) -> Self {
        IndigenousEcoCorridorMap {
            corridor_id,
            eco_metrics,
            fpic_ids_state,
            neurorights_flags,
        }
    }

    /// Non-actuating governance precondition check for high-impact actions.[file:3][file:4]
    /// Returns Ok(()) if the request MAY proceed subject to downstream checks;
    /// returns Err(reason) if the action must be denied at this layer.
    pub fn check_preconditions(
        &self,
        request: &CorridorActionRequest,
    ) -> Result<(), String> {
        // 1. Corridor binding: action corridor must match map corridor.
        if request.corridor_id.as_str() != self.corridor_id.as_str() {
            return Err("Corridor mismatch: action corridor_id does not match map corridor_id"
                .into());
        }

        // 2. Eco-impact guard: deny if requested eco load exceeds thresholds.[file:3][file:4]
        // Here we use a simple threshold on aggregate score; you can refine later.
        let agg = self.eco_metrics.aggregate_score().value();
        if request.required_min_eco_score > agg {
            return Err(format!(
                "EcoImpact guard: required_min_eco_score {:.3} > corridor aggregate {:.3}",
                request.required_min_eco_score, agg
            ));
        }

        // 3. FPIC / IDS: for high-impact actions, consent must be granted and valid.[file:3][file:4]
        if request.high_impact {
            match &self.fpic_ids_state {
                None => {
                    return Err("FPIC/IDS guard: no consent credential present for high-impact action"
                        .into());
                }
                Some(vc) if !vc.is_effectively_granted() => {
                    return Err("FPIC/IDS guard: consent not granted or already revoked".into());
                }
                Some(_vc) => {
                    // Cryptographic / ledger checks are performed by higher layers;
                    // this kernel only enforces presence + logical status.
                }
            }
        }

        // 4. Neurorights guardrails: forbid coercive neuromorphic modes at kernel level.[file:3][file:1]
        if request.may_use_fear_pain_channels
            && self
                .neurorights_flags
                .contains(&NeurorightsFlag::ForbidFearPainCoercion)
        {
            return Err(
                "Neurorights guard: FEAR/PAIN channels are forbidden in this corridor".into(),
            );
        }

        if request.may_infer_mental_state
            && self
                .neurorights_flags
                .contains(&NeurorightsFlag::NoCovertInference)
        {
            return Err(
                "Neurorights guard: covert mental-state inference is forbidden in this corridor"
                    .into(),
            );
        }

        if request.may_attempt_belief_shaping
            && self
                .neurorights_flags
                .contains(&NeurorightsFlag::NoMentalManipulation)
        {
            return Err(
                "Neurorights guard: mental manipulation / belief-shaping is forbidden".into(),
            );
        }

        Ok(())
    }
}

/// Description of a proposed action that must query the corridor map
/// BEFORE any high-impact, actuating system considers running.[file:3][file:4]
#[derive(Clone, Debug)]
pub struct CorridorActionRequest {
    pub corridor_id: CorridorId,
    /// Minimal ecological integrity acceptable for this action, 0–1.
    /// For example, reforestation simulation might require >= 0.5,
    /// invasive extraction might require >= 0.9 (and likely be denied). [file:4]
    pub required_min_eco_score: f32,
    /// Whether this action is considered high-impact (actuation, large-scale change, etc.).
    pub high_impact: bool,
    /// Whether the downstream system plans to route FEAR/PAIN as FEEDBACK channels.
    pub may_use_fear_pain_channels: bool,
    /// Whether the system would infer mental state from telemetry.
    pub may_infer_mental_state: bool,
    /// Whether the system attempts belief-shaping or persuasion.
    pub may_attempt_belief_shaping: bool,
}

impl CorridorActionRequest {
    pub fn new(
        corridor_id: CorridorId,
        required_min_eco_score: f32,
        high_impact: bool,
        may_use_fear_pain_channels: bool,
        may_infer_mental_state: bool,
        may_attempt_belief_shaping: bool,
    ) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&required_min_eco_score) {
            return Err("required_min_eco_score must be within [0.0, 1.0]".into());
        }
        Ok(CorridorActionRequest {
            corridor_id,
            required_min_eco_score,
            high_impact,
            may_use_fear_pain_channels,
            may_infer_mental_state,
            may_attempt_belief_shaping,
        })
    }
}
