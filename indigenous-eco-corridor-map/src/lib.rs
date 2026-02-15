//! Indigenous Eco-Corridor Map
//!
//! Foundational, non-actuating Rust types for representing
//! Indigenous territories as living ecological corridors bounded
//! by biophysical limits and community sovereignty.
//!
//! This crate is designed as a safe base layer for governance-heavy
//! overlays (e.g., Tribal Survival Protocol Envelope, SNC, HGO).
//! It encodes:
//! - mandatory CorridorId presence,
//! - environmental metrics (soil, water, microbiome),
//! - FPIC / IDS governance status,
//! - neurorights constraints.
//!
//! There are no hardware calls, network calls, or behavior-control
//! functions here by design.

#![forbid(unsafe_code)]

pub mod corridor;
pub mod governance;
pub mod metrics;
pub mod neurorights;
pub mod map;

pub use corridor::{CorridorDescriptor, CorridorId, CorridorKind};
pub use governance::{CommunityId, FpicStatus, IdsScope};
pub use metrics::{EnvironmentalMetrics, MicrobiomeMetrics, Score, SoilMetrics, WaterMetrics};
pub use neurorights::NeurorightsConstraints;
pub use map::{IndigenousEcoCorridor, IndigenousEcoCorridorMap};
