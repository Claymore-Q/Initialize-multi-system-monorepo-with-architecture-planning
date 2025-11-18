# Universal Attestation Authority Specification

**System Name:** Universal Attestation Authority
**Version:** 0.1.0
**Status:** Design Phase
**Last Updated:** 2025-11-17

---

## 1. Problem Definition

### 1.1 Background

In distributed systems, establishing trust between components requires cryptographic proof of identity, integrity, and authorization. Current solutions are fragmented:

- **PKI/X.509**: Complex, centralized certificate authorities
- **JWT**: Stateless but lacks revocation mechanisms
- **SPIFFE/SPIRE**: Identity framework but requires infrastructure
- **Hardware TPM**: Device-bound, not portable

**Problem Statement:** Organizations need a unified attestation authority that can issue, verify, and revoke cryptographic attestations for services, devices, and users across heterogeneous environments without requiring centralized trust or specialized hardware.

### 1.2 Core Objectives

1. **Universal Identity**: Issue attestations for any entity (service, device, user, data)
2. **Cryptographic Proof**: Use Ed25519 signatures for fast, secure attestations
3. **Decentralized Trust**: Support multiple trust roots and delegation chains
4. **Efficient Verification**: Sub-millisecond verification in production
5. **Revocation**: Real-time revocation checking without CRLs
6. **Audit Trail**: Immutable log of all attestation operations

### 1.3 Scope

**In Scope:**
- Attestation issuance and verification
- Key management and rotation
- Delegation chains (attestor attestations)
- Revocation management
- Time-bound attestations with auto-expiry
- Batch verification for performance
- Audit logging

**Out of Scope:**
- User authentication (use OAuth2/OIDC)
- Payment/billing
- Legal compliance validation
- Physical device attestation (TPM/SGX integration possible later)

---

## 2. Target Role in Ecosystem

### 2.1 Use Cases

1. **Service-to-Service Authentication**
   ```
   Service A ──┐
               ├──▶ Attestation Authority ──▶ Issues Attestation
   Service B ──┘

   Later:
   Service A ──▶ [Request + Attestation] ──▶ Service B
                                              │
                                              ▼
                                        Verify Attestation
                                              │
                                              ▼
                                          Process Request
   ```

2. **Data Provenance**: Attest to data origin and transformations
   ```
   Data Producer → Attest(data_hash, timestamp, producer_id)
   Data Consumer → Verify attestation before trusting data
   ```

3. **API Authorization**: Prove authorization to call specific APIs
   ```
   Attestation = Sign({
       identity: "service-x",
       permissions: ["read:data", "write:logs"],
       valid_until: "2025-12-31T23:59:59Z"
   })
   ```

4. **Software Supply Chain**: Attest to build artifacts
   ```
   CI System → Attest(binary_hash, source_commit, build_timestamp)
   Deployment → Verify attestation before deploying
   ```

### 2.2 Integration Points

```
┌─────────────────────────────────────────────┐
│   Universal Attestation Authority            │
│                                              │
│  ┌──────────┐  ┌───────────┐  ┌──────────┐│
│  │ Issuer   │  │ Verifier  │  │Revoker   ││
│  └──────────┘  └───────────┘  └──────────┘│
└────────┬───────────────┬──────────┬─────────┘
         │               │          │
         ▼               ▼          ▼
┌──────────────┐  ┌──────────┐  ┌──────────┐
│ Applications │  │ Services │  │  Logs    │
└──────────────┘  └──────────┘  └──────────┘
```

---

## 3. Inputs and Outputs

### 3.1 Input: Attestation Request

```json
{
  "identity": "service-a.production.example.com",
  "claims": {
    "service_name": "service-a",
    "environment": "production",
    "permissions": ["read:users", "write:logs"],
    "ip_address": "10.0.1.42"
  },
  "validity": {
    "not_before": "2025-11-17T00:00:00Z",
    "not_after": "2025-11-18T00:00:00Z"
  },
  "attestor_hint": "root-ca-1"
}
```

### 3.2 Output: Attestation

```json
{
  "version": "1.0.0",
  "attestation_id": "att_9f8e7d6c5b4a",
  "identity": "service-a.production.example.com",
  "claims": {
    "service_name": "service-a",
    "environment": "production",
    "permissions": ["read:users", "write:logs"],
    "ip_address": "10.0.1.42"
  },
  "validity": {
    "not_before": "2025-11-17T00:00:00Z",
    "not_after": "2025-11-18T00:00:00Z",
    "issued_at": "2025-11-17T10:30:00Z"
  },
  "attestor": {
    "identity": "root-ca-1",
    "public_key": "ed25519:AABBCC..."
  },
  "signature": "base64_encoded_ed25519_signature",
  "revocation_url": "https://attestation.example.com/revocation/att_9f8e7d6c5b4a"
}
```

### 3.3 Verification Response

```json
{
  "valid": true,
  "attestation_id": "att_9f8e7d6c5b4a",
  "identity": "service-a.production.example.com",
  "claims": { ... },
  "verification_time": "2025-11-17T12:00:00Z",
  "revoked": false,
  "expired": false,
  "trust_chain": [
    {"identity": "root-ca-1", "verified": true},
    {"identity": "intermediate-ca-2", "verified": true}
  ]
}
```

---

## 4. Lifecycle States

### 4.1 Attestation States

```
┌──────────┐
│Requested │
└────┬─────┘
     │
     ▼
┌──────────┐
│  Issued  │
└────┬─────┘
     │
     ├─────▶ Active ◀──────┐
     │         │           │
     │         ▼           │
     │    ┌────────┐       │
     │    │Renewed │───────┘
     │    └────────┘
     │
     ├─────▶ Expired
     │
     └─────▶ Revoked
```

### 4.2 Trust Chain

```
Root Attestor (self-signed)
    │
    ├──▶ Intermediate Attestor 1
    │        │
    │        ├──▶ Service Attestor A
    │        └──▶ Service Attestor B
    │
    └──▶ Intermediate Attestor 2
             │
             └──▶ Service Attestor C
```

---

## 5. Trust Model

### 5.1 Trust Roots

- Each organization configures trusted root attestors
- Root attestors are self-signed (bootstrap trust)
- Intermediate attestors are signed by roots
- Leaf attestations are signed by intermediates or roots

### 5.2 Delegation Chain Verification

```
Verify(attestation):
    1. Check signature using attestor's public key
    2. Verify attestor's own attestation (recursive)
    3. Ensure chain terminates at a trusted root
    4. Check all attestations in chain are not revoked
    5. Check all attestations are within validity period
    6. Return aggregate result
```

### 5.3 Key Management

- **Root Keys**: Stored in HSM or secure offline storage
- **Intermediate Keys**: Rotated every 90 days
- **Leaf Keys**: Rotated daily or on-demand
- **Key Ceremony**: Multi-party key generation for roots
- **Backup**: Encrypted backups with M-of-N recovery

---

## 6. Failure Modes

| Failure Mode | Detection | Mitigation |
|--------------|-----------|------------|
| Key compromise | Security audit, anomaly detection | Immediate revocation, key rotation |
| Signature forgery | Verification fails | Reject attestation, alert security team |
| Time skew | Clock comparison | Use NTP, reject if skew > 5 minutes |
| Revocation service down | Health check | Cache revocation lists, degrade gracefully |
| Expired attestation used | Check validity period | Reject, force renewal |
| Trust chain broken | Chain verification | Reject, require re-attestation |

---

## 7. API Surface

### 7.1 Rust API

```rust
/// Attestation authority
pub struct AttestationAuthority {
    signing_key: KeyPair,
    storage: Arc<AttestationStore>,
    revocation_list: Arc<RwLock<RevocationList>>,
}

impl AttestationAuthority {
    /// Issue a new attestation
    pub async fn issue_attestation(
        &self,
        request: AttestationRequest,
    ) -> Result<Attestation>;

    /// Verify an attestation
    pub async fn verify_attestation(
        &self,
        attestation: &Attestation,
    ) -> Result<VerificationResult>;

    /// Revoke an attestation
    pub async fn revoke_attestation(
        &self,
        attestation_id: &AttestationId,
        reason: RevocationReason,
    ) -> Result<()>;

    /// Renew an attestation
    pub async fn renew_attestation(
        &self,
        attestation: &Attestation,
    ) -> Result<Attestation>;

    /// Batch verify attestations
    pub async fn verify_batch(
        &self,
        attestations: &[Attestation],
    ) -> Result<Vec<VerificationResult>>;
}

/// Attestation structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attestation {
    pub version: String,
    pub attestation_id: AttestationId,
    pub identity: String,
    pub claims: HashMap<String, serde_json::Value>,
    pub validity: Validity,
    pub attestor: AttestorInfo,
    pub signature: Vec<u8>,
}

/// Validity period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validity {
    pub not_before: Timestamp,
    pub not_after: Timestamp,
    pub issued_at: Timestamp,
}
```

### 7.2 HTTP API

```
POST   /api/v1/attestations              # Issue attestation
POST   /api/v1/attestations/verify       # Verify attestation
POST   /api/v1/attestations/verify-batch # Batch verify
DELETE /api/v1/attestations/{id}         # Revoke attestation
GET    /api/v1/attestations/{id}         # Get attestation details
POST   /api/v1/attestations/{id}/renew   # Renew attestation

GET    /api/v1/revocations                # Get revocation list
GET    /api/v1/revocations/{id}           # Check specific revocation

GET    /api/v1/roots                      # Get trusted roots
POST   /api/v1/roots                      # Add trusted root (admin only)
```

### 7.3 CLI

```bash
# Issue attestation
attest issue --identity service-a --claims claims.json --validity 24h

# Verify attestation
attest verify attestation.json

# Revoke attestation
attest revoke att_9f8e7d6c5b4a --reason "key_compromise"

# List attestations
attest list --identity service-a

# Export public key
attest export-key --format pem
```

---

## 8. Non-Functional Requirements

### 8.1 Performance

- **Issuance**: < 10ms per attestation
- **Verification**: < 1ms per attestation
- **Batch Verification**: < 5ms for 100 attestations
- **Revocation Check**: < 5ms
- **Throughput**: 10,000 issuances/second, 100,000 verifications/second

### 8.2 Security

- **Algorithm**: Ed25519 (fast, secure, small signatures)
- **Key Size**: 256-bit keys, 512-bit signatures
- **Randomness**: Cryptographically secure RNG
- **Side-Channel Resistance**: Constant-time operations
- **Audit**: All operations logged with tamper-evident logs

### 8.3 Availability

- **Target**: 99.99% uptime
- **Redundancy**: Active-active deployment
- **Failover**: Automatic failover < 5 seconds
- **Backup**: Real-time replication to secondary datacenter

---

## Appendix: Attestation Format Specification

### Wire Format (Binary)

```
[Version: 2 bytes]
[Attestation ID: 16 bytes]
[Identity length: 2 bytes]
[Identity: variable]
[Claims length: 4 bytes]
[Claims: JSON, variable]
[Not Before: 8 bytes, Unix timestamp ms]
[Not After: 8 bytes, Unix timestamp ms]
[Issued At: 8 bytes, Unix timestamp ms]
[Attestor Public Key: 32 bytes, Ed25519]
[Signature: 64 bytes, Ed25519]
```

### Total Size

- Minimum: ~150 bytes (without claims)
- Typical: ~300-500 bytes
- Maximum: Configurable, default 64 KB
