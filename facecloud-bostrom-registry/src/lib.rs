use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddressKind {
    Primary,
    Alternate,
    SafeAlternate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainKind {
    Bostrom,
    ERC20Compatible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceFlags {
    pub aln_kyc_did_compliant: bool,
    pub quantum_ready: bool,
    pub requires_rt_monitoring: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredAddress {
    pub label: String,
    pub addr: String,
    pub kind: AddressKind,
    pub chain: ChainKind,
    pub governance: GovernanceFlags,
}

pub fn default_registry() -> Vec<RegisteredAddress> {
    vec![
        RegisteredAddress {
            label: "Primary Bostrom".to_string(),
            addr: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
            kind: AddressKind::Primary,
            chain: ChainKind::Bostrom,
            governance: GovernanceFlags {
                aln_kyc_did_compliant: true,
                quantum_ready: true,
                requires_rt_monitoring: false,
            },
        },
        RegisteredAddress {
            label: "Alternate Bostrom (Google linked)".to_string(),
            addr: "bostrom1ldgmtf20d6604a24ztr0jxht7xt7az4jhkmsrc".to_string(),
            kind: AddressKind::Alternate,
            chain: ChainKind::Bostrom,
            governance: GovernanceFlags {
                aln_kyc_did_compliant: true,
                quantum_ready: true,
                requires_rt_monitoring: true,
            },
        },
        RegisteredAddress {
            label: "Safe alternate zeta".to_string(),
            addr: "zeta12x0up66pzyeretzyku8p4ccuxrjqtqpdc4y4x8".to_string(),
            kind: AddressKind::SafeAlternate,
            chain: ChainKind::Bostrom,
            governance: GovernanceFlags {
                aln_kyc_did_compliant: true,
                quantum_ready: true,
                requires_rt_monitoring: false,
            },
        },
        RegisteredAddress {
            label: "Safe alternate ERC-20".to_string(),
            addr: "0x519fC0eB4111323Cac44b70e1aE31c30e405802D".to_string(),
            kind: AddressKind::SafeAlternate,
            chain: ChainKind::ERC20Compatible,
            governance: GovernanceFlags {
                aln_kyc_did_compliant: true,
                quantum_ready: true,
                requires_rt_monitoring: false,
            },
        },
    ]
}
