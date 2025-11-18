//! Attestation lifecycle integration tests

use universal_attestation_authority::{AttestationAuthority, AttestationConfig, AttestationRequest};

#[tokio::test]
async fn test_attestation_issue_and_verify() {
    let config = AttestationConfig::default();
    let authority = AttestationAuthority::new(config).expect("Failed to create authority");

    let request = AttestationRequest {
        identity: "test-service".to_string(),
        claims: serde_json::Map::new(),
        validity_seconds: 3600,
    };

    // Issue attestation
    let attestation = authority
        .issue(request)
        .await
        .expect("Failed to issue attestation");

    // Verify attestation
    let is_valid = authority
        .verify(&attestation)
        .await
        .expect("Failed to verify attestation");

    assert!(is_valid, "Attestation should be valid");
}

#[tokio::test]
async fn test_multiple_attestations() {
    let config = AttestationConfig::default();
    let authority = AttestationAuthority::new(config).expect("Failed to create authority");

    let mut attestations = vec![];

    for i in 0..10 {
        let request = AttestationRequest {
            identity: format!("service-{}", i),
            claims: serde_json::Map::new(),
            validity_seconds: 3600,
        };

        let attestation = authority.issue(request).await.expect("Failed to issue");
        attestations.push(attestation);
    }

    // Verify all
    for attestation in &attestations {
        let is_valid = authority.verify(attestation).await.expect("Failed to verify");
        assert!(is_valid);
    }
}
