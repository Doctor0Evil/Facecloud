use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Abstract representation of a DNA-derived factor (hash, token, or reference).
/// No raw biometrics are stored here; this is metadata only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnaFactor {
    pub id: Uuid,
    pub hash_reference: String,
    pub confidence: f32,
}

/// Conventional factors used alongside DNA-like factor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeFactor {
    pub present: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PossessionFactor {
    pub present: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiLayerContext {
    pub knowledge: KnowledgeFactor,
    pub possession: PossessionFactor,
    pub dna: Option<DnaFactor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthDecision {
    Deny,
    RequireAdditionalFactors,
    Allow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthEvaluation {
    pub decision: AuthDecision,
    pub explanation: String,
}

pub fn evaluate_mfa(ctx: &MultiLayerContext) -> AuthEvaluation {
    let has_knowledge = ctx.knowledge.present;
    let has_possession = ctx.possession.present;
    let dna_ok = ctx.dna.as_ref().map(|d| d.confidence >= 0.9).unwrap_or(false);

    let decision = match (has_knowledge, has_possession, dna_ok) {
        (true, true, true) => AuthDecision::Allow,
        (true, true, false) => AuthDecision::RequireAdditionalFactors,
        _ => AuthDecision::Deny,
    };

    let explanation = match decision {
        AuthDecision::Allow => {
            "All three layers satisfied (knowledge, possession, DNA-like factor).".to_string()
        }
        AuthDecision::RequireAdditionalFactors => {
            "Knowledge and possession present; DNA-like factor insufficient or missing.".to_string()
        }
        AuthDecision::Deny => {
            "Authentication factors incomplete; access denied by policy.".to_string()
        }
    };

    AuthEvaluation {
        decision,
        explanation,
    }
}
