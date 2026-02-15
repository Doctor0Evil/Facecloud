//! Corridor identifier types and basic descriptors.
//! Non-actuating, data only.

#![forbid(unsafe_code)]

use std::fmt;

/// Opaque, non-empty corridor identifier.
/// Examples: "territory:nation-x:river-y", "eco:desert:phoenix".
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CorridorId(String);

impl CorridorId {
    /// Create a new CorridorId, rejecting empty or whitespace-only IDs.
    pub fn new(id: impl AsRef<str>) -> Result<Self, String> {
        let s = id.as_ref().trim();
        if s.is_empty() {
            return Err("CorridorId must be non-empty".into());
        }
        // Additional normalization or pattern checks can be added here
        Ok(Self(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CorridorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// High-level ecological corridor type, purely descriptive.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CorridorKind {
    Forest,
    Wetland,
    Desert,
    River,
    Coast,
    Mountain,
    UrbanBuffer,
    Custom(String),
}

/// Static, descriptive properties of a corridor.
#[derive(Clone, Debug)]
pub struct CorridorDescriptor {
    pub id: CorridorId,
    pub kind: CorridorKind,
    /// Human-readable name as defined by the community.
    pub name: String,
    /// Optional free-text description; no actuation.
    pub description: Option<String>,
}
