//! Biophysical metrics (soil, water, microbiome) as normalized scores.
//! Non-actuating, observational only.

#![forbid(unsafe_code)]

/// Normalized scalar in [0.0, 1.0].
/// 1.0 = best ecological integrity / least harm.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Score(f32);

impl Score {
    pub fn new(value: f32) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&value) {
            return Err("Score must be within [0.0, 1.0]".into());
        }
        Ok(Self(value))
    }

    pub fn get(self) -> f32 {
        self.0
    }
}

/// Soil health metrics.
#[derive(Clone, Debug)]
pub struct SoilMetrics {
    pub fertility: Score,
    pub erosion_risk: Score,
    pub contamination: Score,
}

/// Water system metrics.
#[derive(Clone, Debug)]
pub struct WaterMetrics {
    pub quality: Score,
    pub flow_resilience: Score,
    pub ecological_flow_support: Score,
}

/// Microbiome / biodiversity metrics.
#[derive(Clone, Debug)]
pub struct MicrobiomeMetrics {
    pub diversity: Score,
    pub keystone_presence: Score,
    pub disturbance_resilience: Score,
}

/// Composite environmental metrics for a corridor.
#[derive(Clone, Debug)]
pub struct EnvironmentalMetrics {
    pub soil: SoilMetrics,
    pub water: WaterMetrics,
    pub microbiome: MicrobiomeMetrics,
}

impl EnvironmentalMetrics {
    /// Simple aggregate score; overlays may refine this.
    pub fn aggregate_score(&self) -> Score {
        let components = [
            self.soil.fertility.get(),
            self.soil.erosion_risk.get(),
            self.soil.contamination.get(),
            self.water.quality.get(),
            self.water.flow_resilience.get(),
            self.water.ecological_flow_support.get(),
            self.microbiome.diversity.get(),
            self.microbiome.keystone_presence.get(),
            self.microbiome.disturbance_resilience.get(),
        ];
        let mean = components.iter().sum::<f32>() / components.len() as f32;
        // Safe because each component is within [0,1].
        Score(mean)
    }
}
