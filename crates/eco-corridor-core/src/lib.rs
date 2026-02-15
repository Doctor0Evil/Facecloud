pub mod indigenous_corridor;

#[cfg(test)]
mod tests {
    use super::indigenous_corridor::*;
    use std::time::SystemTime;

    #[test]
    fn high_impact_action_requires_granted_fpic() {
        let cid = CorridorId::from_str("did:example:corridor:phoenix-desert").unwrap();

        let eco = EcoImpactMetrics {
            soil_health: EcoScalar::new(0.8).unwrap(),
            water_quality: EcoScalar::new(0.7).unwrap(),
            microbiome_diversity: EcoScalar::new(0.9).unwrap(),
            corridor_resilience: EcoScalar::new(0.85).unwrap(),
        };

        let vc = VerifiableConsent {
            issuer_did: "did:example:tribal-council:xyz".to_string(),
            subject_corridor_id: cid.as_str().to_string(),
            status: ConsentStatus::Granted,
            issued_at: SystemTime::now(),
            revoked_at: None,
            signature_hex: "deadbeef".to_string(),
        };

        let neurorights = NeurorightsFlags::new(vec![
            NeurorightsFlag::ForbidFearPainCoercion,
            NeurorightsFlag::NoCovertInference,
            NeurorightsFlag::NoMentalManipulation,
        ]);

        let map = IndigenousEcoCorridorMap::new(cid.clone(), eco, Some(vc), neurorights);

        let req = CorridorActionRequest::new(
            cid,
            0.6,   // requires eco score >= 0.6
            true,  // high-impact
            false, // no FEAR/PAIN
            false, // no mental-state inference
            false, // no belief-shaping
        )
        .unwrap();

        assert!(map.check_preconditions(&req).is_ok());
    }
}
