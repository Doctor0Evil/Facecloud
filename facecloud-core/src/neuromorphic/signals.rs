use serde::{Deserialize, Serialize};

/// Normalized biomechanical density of non-organic material per tissue volume.
/// Purely abstract; no device control.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MechDensity(pub f32);

/// Normalized interface coherence: 1.0 = crisp boundary, 0.0 = fully blurred.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct InterfaceCoherence(pub f32);

/// Normalized EM field intensity at the interface.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EmFieldIntensity(pub f32);

/// Normalized thermal load.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ThermalLoad(pub f32);

/// Normalized systemic inflammation marker.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct InflammationIndex(pub f32);

/// Normalized neuromorphic spike energy proxy.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpikeEnergy(pub f32);

/// Telemetry bundle used by the envelope; abstract, deviceless.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceTelemetry {
    pub mech_density: MechDensity,
    pub interface_coherence: InterfaceCoherence,
    pub em_field: EmFieldIntensity,
    pub thermal_load: ThermalLoad,
    pub inflammation: InflammationIndex,
    pub spike_energy: SpikeEnergy,
}

/// Salience index: how urgently UI/monitoring should surface a warning.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Salience(pub f32);
