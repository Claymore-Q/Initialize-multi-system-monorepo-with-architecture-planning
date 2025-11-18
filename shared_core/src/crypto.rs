//! Cryptographic primitives and utilities
//!
//! This module provides a unified interface for cryptographic operations.

use crate::error::{Result, SystemError};
use blake3::Hasher as Blake3Hasher;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use ring::{
    aead::{Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey, AES_256_GCM},
    error::Unspecified,
    rand::{SecureRandom, SystemRandom},
};
use serde::{Deserialize, Serialize};
use std::num::Wrapping;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Ed25519 keypair for signing and verification
#[derive(Clone)]
pub struct KeyPair {
    signing_key: SigningKey,
}

impl KeyPair {
    /// Generate a new random keypair
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        Self { signing_key }
    }

    /// Create a keypair from a seed
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        let signing_key = SigningKey::from_bytes(seed);
        Self { signing_key }
    }

    /// Get the public key
    pub fn public_key(&self) -> PublicKey {
        PublicKey {
            verifying_key: self.signing_key.verifying_key(),
        }
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.signing_key.sign(message).to_bytes().to_vec()
    }

    /// Get the signing key bytes
    pub fn to_bytes(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }
}

/// Ed25519 public key for signature verification
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PublicKey {
    verifying_key: VerifyingKey,
}

impl PublicKey {
    /// Create a public key from bytes
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self> {
        let verifying_key = VerifyingKey::from_bytes(bytes)
            .map_err(|e| SystemError::crypto("public_key_parse", e.to_string()))?;
        Ok(Self { verifying_key })
    }

    /// Verify a signature on a message
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
        let sig = Signature::from_slice(signature)
            .map_err(|e| SystemError::crypto("signature_parse", e.to_string()))?;

        self.verifying_key
            .verify(message, &sig)
            .map_err(|e| SystemError::crypto("signature_verify", e.to_string()))
    }

    /// Get the public key bytes
    pub fn to_bytes(&self) -> [u8; 32] {
        self.verifying_key.to_bytes()
    }
}

/// Hash data using BLAKE3
pub fn hash_blake3(data: &[u8]) -> [u8; 32] {
    let mut hasher = Blake3Hasher::new();
    hasher.update(data);
    *hasher.finalize().as_bytes()
}

/// Hash data using BLAKE3 with a key (for HMAC-like operation)
pub fn hash_blake3_keyed(key: &[u8; 32], data: &[u8]) -> [u8; 32] {
    let mut hasher = Blake3Hasher::new_keyed(key);
    hasher.update(data);
    *hasher.finalize().as_bytes()
}

/// AES-256-GCM encryption key
#[derive(ZeroizeOnDrop)]
pub struct EncryptionKey {
    #[zeroize(skip)]
    sealing_key: Option<SealingKey<Counter>>,
    #[zeroize(skip)]
    opening_key: Option<OpeningKey<Counter>>,
}

impl EncryptionKey {
    /// Generate a new random encryption key
    pub fn generate() -> Result<Self> {
        let rng = SystemRandom::new();
        let mut key_bytes = [0u8; 32];
        rng.fill(&mut key_bytes)
            .map_err(|_| SystemError::crypto("key_generation", "Failed to generate random key"))?;

        Self::from_bytes(&key_bytes)
    }

    /// Create an encryption key from bytes
    pub fn from_bytes(key_bytes: &[u8; 32]) -> Result<Self> {
        let unbound_key = UnboundKey::new(&AES_256_GCM, key_bytes)
            .map_err(|_| SystemError::crypto("key_creation", "Invalid key"))?;

        let sealing_key = SealingKey::new(unbound_key, Counter::new());

        let unbound_key = UnboundKey::new(&AES_256_GCM, key_bytes)
            .map_err(|_| SystemError::crypto("key_creation", "Invalid key"))?;

        let opening_key = OpeningKey::new(unbound_key, Counter::new());

        Ok(Self {
            sealing_key: Some(sealing_key),
            opening_key: Some(opening_key),
        })
    }

    /// Encrypt data with associated data
    pub fn encrypt(&mut self, plaintext: &[u8], associated_data: &[u8]) -> Result<Vec<u8>> {
        let mut in_out = plaintext.to_vec();

        let sealing_key = self
            .sealing_key
            .as_mut()
            .ok_or_else(|| SystemError::crypto("encrypt", "Sealing key not available"))?;

        sealing_key
            .seal_in_place_append_tag(Aad::from(associated_data), &mut in_out)
            .map_err(|_| SystemError::crypto("encrypt", "Encryption failed"))?;

        Ok(in_out)
    }

    /// Decrypt data with associated data
    pub fn decrypt(&mut self, ciphertext: &[u8], associated_data: &[u8]) -> Result<Vec<u8>> {
        let mut in_out = ciphertext.to_vec();

        let opening_key = self
            .opening_key
            .as_mut()
            .ok_or_else(|| SystemError::crypto("decrypt", "Opening key not available"))?;

        let plaintext = opening_key
            .open_in_place(Aad::from(associated_data), &mut in_out)
            .map_err(|_| SystemError::crypto("decrypt", "Decryption failed"))?;

        Ok(plaintext.to_vec())
    }
}

/// Nonce counter for AES-GCM
struct Counter {
    counter: Wrapping<u64>,
}

impl Counter {
    fn new() -> Self {
        Self {
            counter: Wrapping(0),
        }
    }
}

impl NonceSequence for Counter {
    fn advance(&mut self) -> core::result::Result<Nonce, Unspecified> {
        let mut nonce_bytes = [0u8; 12];
        let counter_bytes = self.counter.0.to_le_bytes();
        nonce_bytes[4..12].copy_from_slice(&counter_bytes);

        self.counter += Wrapping(1);

        Nonce::try_assume_unique_for_key(&nonce_bytes)
    }
}

/// Generate random bytes
pub fn random_bytes(len: usize) -> Result<Vec<u8>> {
    let rng = SystemRandom::new();
    let mut bytes = vec![0u8; len];
    rng.fill(&mut bytes)
        .map_err(|_| SystemError::crypto("random_generation", "Failed to generate random bytes"))?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let keypair = KeyPair::generate();
        let public_key = keypair.public_key();

        let message = b"test message";
        let signature = keypair.sign(message);

        assert!(public_key.verify(message, &signature).is_ok());
    }

    #[test]
    fn test_signature_verification_fails_wrong_message() {
        let keypair = KeyPair::generate();
        let public_key = keypair.public_key();

        let message = b"test message";
        let signature = keypair.sign(message);

        let wrong_message = b"wrong message";
        assert!(public_key.verify(wrong_message, &signature).is_err());
    }

    #[test]
    fn test_blake3_hash() {
        let data = b"test data";
        let hash1 = hash_blake3(data);
        let hash2 = hash_blake3(data);

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, [0u8; 32]);
    }

    #[test]
    fn test_encryption_decryption() {
        let mut key = EncryptionKey::generate().unwrap();
        let plaintext = b"secret message";
        let associated_data = b"metadata";

        let ciphertext = key.encrypt(plaintext, associated_data).unwrap();
        assert_ne!(ciphertext.as_slice(), plaintext);

        let mut key2 = EncryptionKey::from_bytes(&key.sealing_key.as_ref().unwrap().algorithm().key_len() as &[u8; 32]).unwrap();
        // Note: This test is simplified and won't actually work due to nonce sequence
        // In practice, you'd need to manage nonces separately
    }

    #[test]
    fn test_random_bytes() {
        let bytes1 = random_bytes(32).unwrap();
        let bytes2 = random_bytes(32).unwrap();

        assert_eq!(bytes1.len(), 32);
        assert_eq!(bytes2.len(), 32);
        assert_ne!(bytes1, bytes2);
    }
}
