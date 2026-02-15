use serde::{Deserialize, Serialize};

use crate::mfa::{AuthDecision, AuthEvaluation};

/// High-level policy flags for GDPR / ISO27001-style handling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceFlags {
    pub gdpr: bool,
    pub iso27001: bool,
    pub soc2: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPolicy {
    pub role_based_access: bool,
    pub access_logging: bool,
    pub data_minimization: bool,
    pub lawful_processing: bool,
    pub compliance: ComplianceFlags,
}

impl Default for AccessPolicy {
    fn default() -> Self {
        Self {
            role_based_access: true,
            access_logging: true,
            data_minimization: true,
            lawful_processing: true,
            compliance: ComplianceFlags {
                gdpr: true,
                iso27001: true,
                soc2: true,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyVerdict {
    pub allowed: bool,
    pub reasons: Vec<String>,
}

pub fn evaluate_policy(auth: &AuthEvaluation, policy: &AccessPolicy) -> PolicyVerdict {
    let mut reasons = Vec::new();
    let allowed = match auth.decision {
        AuthDecision::Allow => true,
        AuthDecision::RequireAdditionalFactors => {
            reasons.push("Additional authentication factors required.".to_string());
            false
        }
        AuthDecision::Deny => {
            reasons.push("Authentication decision = Deny.".to_string());
            false
        }
    };

    if !policy.role_based_access {
        reasons.push("Role-based access control disabled; policy expects RBAC.".to_string());
    }
    if !policy.access_logging {
        reasons.push("Access logging disabled; policy expects full audit trail.".to_string());
    }
    if !policy.data_minimization {
        reasons.push("Data minimization not enforced.".to_string());
    }
    if !policy.lawful_processing {
        reasons.push("Lawful processing flag is false.".to_string());
    }

    PolicyVerdict { allowed, reasons }
}
