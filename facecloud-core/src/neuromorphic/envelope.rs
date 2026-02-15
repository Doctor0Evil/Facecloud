use serde::{Deserialize, Serialize};

use super::signals::{
    EmFieldIntensity, InflammationIndex, InterfaceCoherence, InterfaceTelemetry, MechDensity,
    Salience, SpikeEnergy, ThermalLoad,
};

/// Safety margins for each constraint; 1.0 = just-safe, >1.0 = margin, <1.0 = breach.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ConstraintMargins {
    pub mech_density_margin: f32,
    pub interface_coherence_margin: f32,
    pub em_field_margin: f32,
    pub thermal_margin: f32,
    pub inflammation_margin: f32,
    pub spike_energy_margin: f32,
}

impl ConstraintMargins {
    pub fn composite(&self) -> f32 {
        self.mech_density_margin
            .min(self.interface_coherence_margin)
            .min(self.em_field_margin)
            .min(self.thermal_margin)
            .min(self.inflammation_margin)
            .min(self.spike_energy_margin)
    }
}

/// High-level scalar status: replaces the “face-in-cloud” with a numeric regime.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnvelopeStatus {
    /// Well inside safe corridor.
    Safe,
    /// CAUTION_SCALE_THRESHOLD_APPROACHED: nearing boundary.
    Caution,
    /// HARD_DENY: outside envelope; scaling should be rolled back.
    HardDeny,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvelopeEvaluation {
    pub margins: ConstraintMargins,
    pub composite_margin: f32,
    pub status: EnvelopeStatus,
    pub salience: Salience,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvelopeConfig {
    pub mech_density_max: f32,
    pub interface_coherence_min: f32,
    pub em_field_max: f32,
    pub thermal_max: f32,
    pub inflammation_max: f32,
    pub spike_energy_max: f32,
    pub caution_lower: f32,
    pub caution_upper: f32,
}

impl Default for EnvelopeConfig {
    fn default() -> Self {
        Self {
            mech_density_max: 1.0,
            interface_coherence_min: 0.8,
            em_field_max: 1.0,
            thermal_max: 1.0,
            inflammation_max: 1.0,
            spike_energy_max: 1.0,
            caution_lower: 1.0,
            caution_upper: 1.1,
        }
    }
}

impl EnvelopeConfig {
    fn mech_density_margin(&self, d: MechDensity) -> f32 {
        if d.0 <= 0.0 {
            2.0
        } else {
            self.mech_density_max / d.0
        }
    }

    fn interface_coherence_margin(&self, c: InterfaceCoherence) -> f32 {
        if c.0 <= 0.0 {
            0.0
        } else {
            c.0 / self.interface_coherence_min
        }
    }

    fn upper_bounded_margin(&self, value: f32, max: f32) -> f32 {
        if value <= 0.0 {
            max
        } else {
            max / value
        }
    }

    pub fn evaluate(&self, telemetry: &InterfaceTelemetry) -> EnvelopeEvaluation {
        let mech_density_margin = self.mech_density_margin(telemetry.mech_density);
        let interface_coherence_margin =
            self.interface_coherence_margin(telemetry.interface_coherence);
        let em_field_margin =
            self.upper_bounded_margin(telemetry.em_field.0, self.em_field_max);
        let thermal_margin =
            self.upper_bounded_margin(telemetry.thermal_load.0, self.thermal_max);
        let inflammation_margin =
            self.upper_bounded_margin(telemetry.inflammation.0, self.inflammation_max);
        let spike_energy_margin =
            self.upper_bounded_margin(telemetry.spike_energy.0, self.spike_energy_max);

        let margins = ConstraintMargins {
            mech_density_margin,
            interface_coherence_margin,
            em_field_margin,
            thermal_margin,
            inflammation_margin,
            spike_energy_margin,
        };
        let composite_margin = margins.composite();

        let status = if composite_margin < self.caution_lower {
            EnvelopeStatus::HardDeny
        } else if composite_margin < self.caution_upper {
            EnvelopeStatus::Caution
        } else {
            EnvelopeStatus::Safe
        };

        let salience = Salience((self.caution_upper - composite_margin).max(0.0));

        EnvelopeEvaluation {
            margins,
            composite_margin,
            status,
            salience,
        }
    }
}
