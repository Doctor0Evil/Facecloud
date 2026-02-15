//! Neurorights constraints as non-actuating flags.
//! Designed to be used as mandatory preconditions by overlays.

#![forbid(unsafe_code)]

/// How this corridor may be used in relation to neuromorphic systems.
#[derive(Clone, Debug)]
pub struct NeurorightsConstraints {
    /// True if any neuromorphic / behavioral system that touches this
    /// corridor must remain observational (non-actuating) with respect
    /// to community members and species in the corridor.
    pub non_actuating_required: bool,

    /// True if covert behavioral inference or coercive channels are
    /// categorically disallowed for this corridor.
    pub no_coercive_or_hidden_channels: bool,

    /// True if any FEAR/PAIN-like signals must be explicitly declared
    /// as voluntary, consent-bound discipline channels and never used
    /// as levers for behavioral control.
    pub discipline_signals_voluntary_only: bool,

    /// Optional reference (e.g., hash/URI) to a TREEEnvelope /
    /// HIT Governance Object / SNC shard describing this corridor's
    /// biophysical and neurorights envelope.
    pub envelope_ref: Option<String>,
}

impl NeurorightsConstraints {
    pub fn strict_non_actuating() -> Self {
        Self {
            non_actuating_required: true,
            no_coercive_or_hidden_channels: true,
            discipline_signals_voluntary_only: true,
            envelope_ref: None,
        }
    }
}
