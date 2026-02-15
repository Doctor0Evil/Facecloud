//! Indigenous Eco-Corridor Map: a non-actuating, consent-bound map.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;

use crate::corridor::{CorridorDescriptor, CorridorId};
use crate::governance::{FpicStatus, IdsScope};
use crate::metrics::EnvironmentalMetrics;
use crate::neurorights::NeurorightsConstraints;

/// Immutable snapshot of one Indigenous eco-corridor as a living
/// ecological corridor that is governed by biophysical limits and
/// community sovereignty. No actuation, no behavior control.
#[derive(Clone, Debug)]
pub struct IndigenousEcoCorridor {
    pub descriptor: CorridorDescriptor,
    pub environmental: EnvironmentalMetrics,
    pub fpic: FpicStatus,
    pub ids_scope: IdsScope,
    pub neurorights: NeurorightsConstraints,
    /// Optional community-provided cultural metadata.
    pub cultural_notes: Option<String>,
}

impl IndigenousEcoCorridor {
    pub fn id(&self) -> &CorridorId {
        &self.descriptor.id
    }

    /// True if FPIC is actively granted under some terms.
    pub fn has_active_fpic(&self) -> bool {
        self.fpic.is_active_grant()
    }

    /// True if this corridor is strictly non-actuating for neuromorphic use.
    pub fn requires_non_actuating(&self) -> bool {
        self.neurorights.non_actuating_required
    }
}

/// Non-actuating map of corridor ID -> corridor record.
///
/// This is a foundational knowledge object that overlays (e.g.,
/// Tribal Survival Protocol Envelope) can depend on. It exposes
/// only read-like operations; no commands or behavior scheduling.
#[derive(Default)]
pub struct IndigenousEcoCorridorMap {
    corridors: BTreeMap<CorridorId, IndigenousEcoCorridor>,
}

impl IndigenousEcoCorridorMap {
    pub fn new() -> Self {
        Self {
            corridors: BTreeMap::new(),
        }
    }

    /// Insert or replace a corridor definition.
    /// Overlays can decide whether replacement is allowed by policy.
    pub fn upsert_corridor(&mut self, corridor: IndigenousEcoCorridor) {
        self.corridors.insert(corridor.id().clone(), corridor);
    }

    /// Retrieve a corridor by ID, if present.
    pub fn get(&self, id: &CorridorId) -> Option<&IndigenousEcoCorridor> {
        self.corridors.get(id)
    }

    /// Iterate over all corridors (e.g., for monitoring or reporting).
    pub fn iter(&self) -> impl Iterator<Item = (&CorridorId, &IndigenousEcoCorridor)> {
        self.corridors.iter()
    }

    /// Ensure that a corridor ID exists; overlays can call this to
    /// enforce "no missing corridor IDs" before any higher-level
    /// governance logic runs.
    pub fn ensure_exists(&self, id: &CorridorId) -> Result<(), String> {
        if self.corridors.contains_key(id) {
            Ok(())
        } else {
            Err(format!("CorridorId '{}' is not registered", id))
        }
    }
}
