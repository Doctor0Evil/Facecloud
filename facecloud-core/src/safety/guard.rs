use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::neuromorphic::envelope::{EnvelopeConfig, EnvelopeEvaluation, EnvelopeStatus};
use crate::neuromorphic::signals::InterfaceTelemetry;

/// Purely analytical: no actuation, only recommendations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardRecommendation {
    pub id: Uuid,
    pub evaluation: EnvelopeEvaluation,
    pub message: String,
    pub recommended_action: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GuardKernel {
    pub config: EnvelopeConfig,
}

impl GuardKernel {
    pub fn evaluate(&self, telemetry: &InterfaceTelemetry) -> GuardRecommendation {
        let eval = self.config.evaluate(telemetry);
        let (message, action) = match eval.status {
            EnvelopeStatus::Safe => (
                "Within safety envelope.".to_string(),
                "Maintain current parameters; continue monitoring.".to_string(),
            ),
            EnvelopeStatus::Caution => (
                "CAUTION_SCALE_THRESHOLD_APPROACHED".to_string(),
                "Do not increase integration density or field intensity; prefer down-scaling or simulations only."
                    .to_string(),
            ),
            EnvelopeStatus::HardDeny => (
                "HARD_DENY: envelope breached.".to_string(),
                "Reduce load, density, and exposure in models; consult safety governance before any further scaling."
                    .to_string(),
            ),
        };

        GuardRecommendation {
            id: Uuid::new_v4(),
            evaluation: eval,
            message,
            recommended_action: action,
        }
    }
}
