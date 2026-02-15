use prometheus::{Encoder, IntCounter, IntGauge, Registry, TextEncoder};
use std::sync::Arc;
use std::sync::Mutex;

use crate::neuromorphic::envelope::EnvelopeStatus;

#[derive(Clone)]
pub struct SafetyMetrics {
    registry: Registry,
    pub last_composite_margin: IntGauge,
    pub caution_total: IntCounter,
    pub hard_deny_total: IntCounter,
}

impl SafetyMetrics {
    pub fn new() -> Arc<Mutex<Self>> {
        let registry = Registry::new();
        let last_composite_margin =
            IntGauge::new("facecloud_envelope_margin_x100", "Composite margin x100").unwrap();
        let caution_total =
            IntCounter::new("facecloud_envelope_caution_total", "Caution events").unwrap();
        let hard_deny_total =
            IntCounter::new("facecloud_envelope_hard_deny_total", "Hard deny events").unwrap();

        registry
            .register(Box::new(last_composite_margin.clone()))
            .unwrap();
        registry.register(Box::new(caution_total.clone())).unwrap();
        registry.register(Box::new(hard_deny_total.clone())).unwrap();

        Arc::new(Mutex::new(Self {
            registry,
            last_composite_margin,
            caution_total,
            hard_deny_total,
        }))
    }

    pub fn observe_status(&self, status: EnvelopeStatus, composite_margin: f32) {
        self.last_composite_margin
            .set((composite_margin * 100.0) as i64);
        match status {
            EnvelopeStatus::Safe => {}
            EnvelopeStatus::Caution => self.caution_total.inc(),
            EnvelopeStatus::HardDeny => self.hard_deny_total.inc(),
        }
    }

    pub fn export_prometheus(&self) -> String {
        let mut buffer = Vec::new();
        let encoder = TextEncoder::new();
        let mf = self.registry.gather();
        encoder.encode(&mf, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap_or_default()
    }
}
