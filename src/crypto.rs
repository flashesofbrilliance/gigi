//! Geometric Encryption — gauge transformations on fiber coordinates.
//!
//! v0.1 — Affine numeric only. ρ_g(v) = a·v + b. Curvature/spectral/anomaly
//! invariant by Theorem 5a.1.
//!
//! v0.2 — Per-field mode declared at schema time:
//!   • Affine          — v0.1, numeric only (default for NUMERIC/INTEGER/TIMESTAMP)
//!   • Opaque          — AEAD via AES-GCM-SIV (RFC 8452); randomized; IND-CPA
//!   • Indexed         — PRF via AES-256-CMAC; deterministic; equality-queryable
//!   • Probabilistic   — affine + Gaussian noise (Sprint D)
//!   • Isometric       — O(k) on grouped numeric (Sprint E)
//!
//! AAD (associated authenticated data) binds Opaque ciphertexts to their
//! position in the bundle (bundle name + field index + field name) so a
//! ciphertext swapped between fields fails authentication on decrypt.

use crate::types::{FieldDef, FieldType, Value};

use aes_gcm_siv::{
    aead::{Aead, KeyInit, Payload},
    Aes256GcmSiv, Nonce,
};
use cmac::{Cmac, Mac};
use aes::Aes256;

/// Per-field gauge transform. v0.2 introduces multiple variants — each fiber
/// field carries one variant matching its declared `EncryptionMode`.
#[derive(Debug, Clone)]
pub enum FieldTransform {
    /// No encryption (plaintext field). Identity on encrypt and decrypt.
    Identity,

    /// v0.1 affine numeric: v_enc = scale * v + offset. Curvature-invariant
    /// (`Var/range²` is a ratio of `a²`-scaled quantities). Numeric only.
    Affine { scale: f64, offset: f64 },

    /// AEAD via AES-GCM-SIV (RFC 8452 — nonce-misuse-resistant). Per-record
    /// random 96-bit nonce; 128-bit auth tag. IND-CPA. Not equality-queryable.
    /// On-disk wire format: [12-byte nonce | ciphertext | 16-byte tag], stored
    /// as `Value::Binary`.
    Opaque { key: [u8; 32] },

    /// PRF via AES-256-CMAC (NIST SP 800-38B). Deterministic — equal
    /// plaintexts yield equal 16-byte tags. Equality-queryable; bitmap index
    /// works verbatim. High-cardinality columns only (deterministic
    /// encryption leaks frequency on low-cardinality data — schema author
    /// must opt in).
    Indexed { key: [u8; 32] },
}

impl FieldTransform {
    /// Apply the forward gauge transform, optionally with AAD (used by AEAD
    /// modes; ignored by Affine/Identity). The AAD binds the ciphertext to
    /// its (bundle, field) position so a swap between fields fails
    /// authentication on decrypt.
    pub fn encrypt_value(&self, v: &Value, aad: &[u8]) -> Value {
        match self {
            FieldTransform::Identity => v.clone(),
            FieldTransform::Affine { scale, offset } => match v {
                Value::Float(f) => Value::Float(scale * f + offset),
                Value::Integer(i) => Value::Float(scale * (*i as f64) + offset),
                Value::Timestamp(t) => Value::Timestamp((scale * (*t as f64) + offset) as i64),
                other => other.clone(),
            },
            FieldTransform::Opaque { key } => {
                let plaintext_bytes = value_to_bytes(v);
                let ct = aead_encrypt(key, &plaintext_bytes, aad);
                Value::Binary(ct)
            }
            FieldTransform::Indexed { key } => {
                let plaintext_bytes = value_to_bytes(v);
                let tag = cmac_prf(key, &plaintext_bytes);
                Value::Binary(tag.to_vec())
            }
        }
    }

    /// Apply the inverse transform. For Indexed, the PRF is one-way — decrypt
    /// returns the stored ciphertext bytes as-is (the caller knows from the
    /// schema that the field is one-way encrypted; equality search is the
    /// supported access pattern).
    pub fn decrypt_value(&self, w: &Value, aad: &[u8]) -> Value {
        match self {
            FieldTransform::Identity => w.clone(),
            FieldTransform::Affine { scale, offset } => match w {
                Value::Float(f) => Value::Float((f - offset) / scale),
                Value::Integer(i) => Value::Float(((*i as f64) - offset) / scale),
                Value::Timestamp(t) => {
                    Value::Timestamp((((*t as f64) - offset) / scale) as i64)
                }
                other => other.clone(),
            },
            FieldTransform::Opaque { key } => match w {
                Value::Binary(bytes) => {
                    let plaintext_bytes = aead_decrypt(key, bytes, aad)
                        .expect("AEAD decrypt failed — ciphertext tampered or wrong key/AAD");
                    bytes_to_value(&plaintext_bytes)
                }
                other => other.clone(),
            },
            FieldTransform::Indexed { .. } => {
                // PRF is one-way. Return the stored ciphertext as-is.
                w.clone()
            }
        }
    }
}

/// A geometric encryption key: one FieldTransform per fiber field.
#[derive(Debug, Clone)]
pub struct GaugeKey {
    pub transforms: Vec<FieldTransform>,
}

impl GaugeKey {
    /// Derive a GaugeKey from a 32-byte seed and the fiber field definitions.
    /// Each field's transform variant is determined by its `EncryptionMode`:
    ///   - `None`           → Identity
    ///   - `Affine`         → Affine { scale, offset }   (KDF from seed + name)
    ///   - `Opaque`         → Opaque { key: 32B }        (KDF from seed + name)
    ///   - `Indexed`        → Indexed { key: 32B }       (KDF from seed + name)
    ///   - `Probabilistic`  → reserved (Sprint D — falls through to Identity for now)
    ///   - `Isometric`      → reserved (Sprint E — falls through to Identity for now)
    pub fn derive(seed: &[u8; 32], fiber_fields: &[FieldDef]) -> Self {
        let transforms = fiber_fields
            .iter()
            .map(|field| Self::derive_field_transform(seed, &field.name, &field.field_type, &field.encryption))
            .collect();
        GaugeKey { transforms }
    }

    /// Derive transform for a single field from seed + field name + mode.
    fn derive_field_transform(
        seed: &[u8; 32],
        field_name: &str,
        field_type: &FieldType,
        mode: &crate::types::EncryptionMode,
    ) -> FieldTransform {
        use crate::types::EncryptionMode;

        match mode {
            EncryptionMode::None => {
                // v0.1-shaped fallback: when no per-field mode is set, the
                // bundle-level dispatch (in parser.rs CreateBundle handler)
                // fills in default modes BEFORE calling derive. If we still
                // see None here, the field is genuinely unencrypted.
                FieldTransform::Identity
            }
            EncryptionMode::Affine => Self::derive_affine(seed, field_name),
            EncryptionMode::Opaque => Self::derive_opaque(seed, field_name),
            EncryptionMode::Indexed => Self::derive_indexed(seed, field_name),
            EncryptionMode::Probabilistic { .. } => {
                // Sprint D — placeholder; full implementation lands when
                // PROBABILISTIC mode ships.
                let _ = field_type;
                FieldTransform::Identity
            }
            EncryptionMode::Isometric => {
                // Sprint E — placeholder.
                let _ = field_type;
                FieldTransform::Identity
            }
        }
    }

    fn derive_affine(seed: &[u8; 32], field_name: &str) -> FieldTransform {
        let mut hasher_bytes = Vec::with_capacity(seed.len() + field_name.len() + 9);
        hasher_bytes.extend_from_slice(seed);
        hasher_bytes.extend_from_slice(b":affine:");
        hasher_bytes.extend_from_slice(field_name.as_bytes());

        let h1 = mix_hash(&hasher_bytes, 0x517cc1b727220a95);
        let h2 = mix_hash(&hasher_bytes, 0x6c62272e07bb0142);

        // Scale: map to range [0.1, 10.0], ensuring nonzero.
        let scale_raw = (h1 as f64) / (u64::MAX as f64);
        let scale = 0.1 + scale_raw * 9.9;

        // Offset: map to range [-1000, 1000].
        let offset_raw = (h2 as f64) / (u64::MAX as f64);
        let offset = -1000.0 + offset_raw * 2000.0;

        FieldTransform::Affine { scale, offset }
    }

    fn derive_opaque(seed: &[u8; 32], field_name: &str) -> FieldTransform {
        // Derive a 32-byte AES-256 key from the seed via two 64-bit mixes
        // domain-separated by ":opaque:" + field name.
        FieldTransform::Opaque {
            key: derive_field_key(seed, b":opaque:", field_name),
        }
    }

    fn derive_indexed(seed: &[u8; 32], field_name: &str) -> FieldTransform {
        FieldTransform::Indexed {
            key: derive_field_key(seed, b":indexed:", field_name),
        }
    }

    /// Simple deterministic hash mixing (wyhash-inspired). Public so the WAL
    /// can re-derive keys deterministically when reloading a schema.
    pub fn mix_hash(data: &[u8], seed: u64) -> u64 {
        mix_hash(data, seed)
    }

    /// Encrypt a fiber value vector (in schema field order). The `bundle_name`
    /// is used to construct AAD that binds each ciphertext to its position
    /// (bundle, field index, field name) — so a ciphertext swapped between
    /// fields or between bundles fails authentication on decrypt.
    pub fn encrypt_fiber(
        &self,
        fiber_vals: &[Value],
        bundle_name: &str,
        fiber_fields: &[FieldDef],
    ) -> Vec<Value> {
        fiber_vals
            .iter()
            .enumerate()
            .map(|(i, v)| {
                if let Some(t) = self.transforms.get(i) {
                    let aad = build_aad(bundle_name, i, fiber_fields.get(i).map(|f| f.name.as_str()).unwrap_or(""));
                    t.encrypt_value(v, &aad)
                } else {
                    v.clone()
                }
            })
            .collect()
    }

    /// Decrypt a fiber value vector (in schema field order). AAD is recomputed
    /// per field — must match the AAD used at encrypt time.
    pub fn decrypt_fiber(
        &self,
        encrypted_vals: &[Value],
        bundle_name: &str,
        fiber_fields: &[FieldDef],
    ) -> Vec<Value> {
        encrypted_vals
            .iter()
            .enumerate()
            .map(|(i, w)| {
                if let Some(t) = self.transforms.get(i) {
                    let aad = build_aad(bundle_name, i, fiber_fields.get(i).map(|f| f.name.as_str()).unwrap_or(""));
                    t.decrypt_value(w, &aad)
                } else {
                    w.clone()
                }
            })
            .collect()
    }

    /// Encrypt a single query literal for a given fiber field index. Used by
    /// the engine to translate `WHERE field = X` into a comparison in
    /// encrypted space. For `Affine`, this means pre-encrypting X. For
    /// `Indexed`, this means PRF-hashing X. For `Opaque`, equality query is
    /// not supported (callers must check the schema mode and reject).
    pub fn encrypt_literal(
        &self,
        field_idx: usize,
        value: &Value,
        bundle_name: &str,
        fiber_fields: &[FieldDef],
    ) -> Value {
        if let Some(t) = self.transforms.get(field_idx) {
            let aad = build_aad(
                bundle_name,
                field_idx,
                fiber_fields.get(field_idx).map(|f| f.name.as_str()).unwrap_or(""),
            );
            t.encrypt_value(value, &aad)
        } else {
            value.clone()
        }
    }

    /// Generate a random 32-byte seed using OS CSPRNG.
    pub fn random_seed() -> [u8; 32] {
        let mut seed = [0u8; 32];
        getrandom::getrandom(&mut seed).expect("Failed to generate random seed from OS CSPRNG");
        seed
    }
}

// ─────────────────────────────────────────────────────────────────────────
// AAD construction
// ─────────────────────────────────────────────────────────────────────────

/// Build the AAD (associated authenticated data) for an Opaque ciphertext.
/// Format: `<bundle_name>|<field_idx>|<field_name>` as raw UTF-8 bytes.
/// Binds the ciphertext to its position in the bundle so swapping ciphertexts
/// between fields fails authentication on decrypt.
fn build_aad(bundle_name: &str, field_idx: usize, field_name: &str) -> Vec<u8> {
    let mut aad = Vec::with_capacity(bundle_name.len() + field_name.len() + 16);
    aad.extend_from_slice(bundle_name.as_bytes());
    aad.push(b'|');
    aad.extend_from_slice(field_idx.to_string().as_bytes());
    aad.push(b'|');
    aad.extend_from_slice(field_name.as_bytes());
    aad
}

// ─────────────────────────────────────────────────────────────────────────
// Value ↔ bytes conversions for AEAD modes
// ─────────────────────────────────────────────────────────────────────────

/// Serialize a Value to bytes for AEAD-style encryption. The first byte is a
/// type tag so decrypt can recover the original variant.
fn value_to_bytes(v: &Value) -> Vec<u8> {
    match v {
        Value::Null => vec![0x00],
        Value::Bool(b) => vec![0x01, if *b { 1 } else { 0 }],
        Value::Integer(i) => {
            let mut buf = Vec::with_capacity(9);
            buf.push(0x02);
            buf.extend_from_slice(&i.to_le_bytes());
            buf
        }
        Value::Float(f) => {
            let mut buf = Vec::with_capacity(9);
            buf.push(0x03);
            buf.extend_from_slice(&f.to_le_bytes());
            buf
        }
        Value::Text(s) => {
            let mut buf = Vec::with_capacity(s.len() + 1);
            buf.push(0x04);
            buf.extend_from_slice(s.as_bytes());
            buf
        }
        Value::Binary(b) => {
            let mut buf = Vec::with_capacity(b.len() + 1);
            buf.push(0x05);
            buf.extend_from_slice(b);
            buf
        }
        Value::Timestamp(t) => {
            let mut buf = Vec::with_capacity(9);
            buf.push(0x06);
            buf.extend_from_slice(&t.to_le_bytes());
            buf
        }
        Value::Vector(values) => {
            let mut buf = Vec::with_capacity(values.len() * 8 + 5);
            buf.push(0x07);
            buf.extend_from_slice(&(values.len() as u32).to_le_bytes());
            for f in values {
                buf.extend_from_slice(&f.to_le_bytes());
            }
            buf
        }
    }
}

/// Inverse of `value_to_bytes`. Returns `Value::Null` on malformed input.
fn bytes_to_value(b: &[u8]) -> Value {
    if b.is_empty() {
        return Value::Null;
    }
    match b[0] {
        0x00 => Value::Null,
        0x01 => Value::Bool(b.get(1).copied().unwrap_or(0) != 0),
        0x02 if b.len() >= 9 => Value::Integer(i64::from_le_bytes(b[1..9].try_into().unwrap())),
        0x03 if b.len() >= 9 => Value::Float(f64::from_le_bytes(b[1..9].try_into().unwrap())),
        0x04 => Value::Text(String::from_utf8_lossy(&b[1..]).into_owned()),
        0x05 => Value::Binary(b[1..].to_vec()),
        0x06 if b.len() >= 9 => Value::Timestamp(i64::from_le_bytes(b[1..9].try_into().unwrap())),
        0x07 if b.len() >= 5 => {
            let n = u32::from_le_bytes(b[1..5].try_into().unwrap()) as usize;
            let mut out = Vec::with_capacity(n);
            let mut off = 5;
            for _ in 0..n {
                if off + 8 > b.len() {
                    break;
                }
                out.push(f64::from_le_bytes(b[off..off + 8].try_into().unwrap()));
                off += 8;
            }
            Value::Vector(out)
        }
        _ => Value::Null,
    }
}

// ─────────────────────────────────────────────────────────────────────────
// AEAD primitives (Opaque mode)
// ─────────────────────────────────────────────────────────────────────────

/// Encrypt with AES-GCM-SIV. Returns [12-byte nonce | ciphertext | 16-byte tag]
/// as a single Vec<u8>. Per-record random nonce drawn from OS CSPRNG.
fn aead_encrypt(key: &[u8; 32], plaintext: &[u8], aad: &[u8]) -> Vec<u8> {
    let cipher = Aes256GcmSiv::new(key.into());
    let mut nonce_bytes = [0u8; 12];
    getrandom::getrandom(&mut nonce_bytes).expect("OS CSPRNG failure");
    let nonce = Nonce::from_slice(&nonce_bytes);
    let payload = Payload {
        msg: plaintext,
        aad,
    };
    let ct_and_tag = cipher
        .encrypt(nonce, payload)
        .expect("AES-GCM-SIV encrypt failed");
    let mut out = Vec::with_capacity(12 + ct_and_tag.len());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ct_and_tag);
    out
}

/// Decrypt an AES-GCM-SIV blob. Returns `Err(())` on auth tag mismatch
/// (tamper detected).
fn aead_decrypt(key: &[u8; 32], blob: &[u8], aad: &[u8]) -> Result<Vec<u8>, ()> {
    if blob.len() < 12 + 16 {
        return Err(());
    }
    let cipher = Aes256GcmSiv::new(key.into());
    let nonce = Nonce::from_slice(&blob[..12]);
    let payload = Payload {
        msg: &blob[12..],
        aad,
    };
    cipher.decrypt(nonce, payload).map_err(|_| ())
}

// ─────────────────────────────────────────────────────────────────────────
// PRF primitives (Indexed mode)
// ─────────────────────────────────────────────────────────────────────────

/// Compute AES-256-CMAC over the input bytes with the given key. Output is
/// a 16-byte deterministic tag — equal inputs under the same key yield equal
/// tags. NIST SP 800-38B.
fn cmac_prf(key: &[u8; 32], input: &[u8]) -> [u8; 16] {
    let mut mac = <Cmac<Aes256> as Mac>::new_from_slice(key).expect("AES-256 key");
    mac.update(input);
    let result = mac.finalize().into_bytes();
    let mut tag = [0u8; 16];
    tag.copy_from_slice(&result);
    tag
}

// ─────────────────────────────────────────────────────────────────────────
// Key derivation
// ─────────────────────────────────────────────────────────────────────────

/// Derive a 32-byte field-specific key from the master seed using two
/// 64-bit wyhash-style mixes, domain-separated by purpose (`:opaque:` /
/// `:indexed:`) and the field name. Same seed + same purpose + same field
/// name → same key (deterministic across deployments using the same seed).
fn derive_field_key(seed: &[u8; 32], purpose: &[u8], field_name: &str) -> [u8; 32] {
    let mut input = Vec::with_capacity(seed.len() + purpose.len() + field_name.len());
    input.extend_from_slice(seed);
    input.extend_from_slice(purpose);
    input.extend_from_slice(field_name.as_bytes());

    // Four independent 64-bit mixes packed into a 32-byte key. Each mix uses
    // a distinct seed prime to ensure bit-independence.
    let h1 = mix_hash(&input, 0x517cc1b727220a95);
    let h2 = mix_hash(&input, 0x6c62272e07bb0142);
    let h3 = mix_hash(&input, 0xff51afd7ed558ccd);
    let h4 = mix_hash(&input, 0xc4ceb9fe1a85ec53);

    let mut key = [0u8; 32];
    key[0..8].copy_from_slice(&h1.to_le_bytes());
    key[8..16].copy_from_slice(&h2.to_le_bytes());
    key[16..24].copy_from_slice(&h3.to_le_bytes());
    key[24..32].copy_from_slice(&h4.to_le_bytes());
    key
}

fn mix_hash(data: &[u8], seed: u64) -> u64 {
    let mut h = seed;
    for &b in data {
        h = h.wrapping_mul(0x2d358dccaa6c78a5).wrapping_add(b as u64);
        h ^= h >> 33;
    }
    h = h.wrapping_mul(0xff51afd7ed558ccd);
    h ^= h >> 33;
    h = h.wrapping_mul(0xc4ceb9fe1a85ec53);
    h ^= h >> 33;
    h
}

/// Parse a hex string into a 32-byte seed.
pub fn seed_from_hex(hex: &str) -> Result<[u8; 32], String> {
    let hex = hex.trim();
    if hex.len() != 64 {
        return Err(format!(
            "Encryption seed must be 64 hex characters (32 bytes), got {}",
            hex.len()
        ));
    }
    let mut seed = [0u8; 32];
    for i in 0..32 {
        seed[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16)
            .map_err(|_| format!("Invalid hex at position {}", i * 2))?;
    }
    Ok(seed)
}

// ─────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{EncryptionMode, FieldDef, Value};

    fn test_seed() -> [u8; 32] {
        let mut s = [0u8; 32];
        for i in 0..32 {
            s[i] = (i as u8).wrapping_mul(7).wrapping_add(13);
        }
        s
    }

    fn affine_fields() -> Vec<FieldDef> {
        vec![
            FieldDef::numeric("temp").with_encryption(EncryptionMode::Affine),
            FieldDef::numeric("humidity").with_encryption(EncryptionMode::Affine),
            FieldDef::numeric("pressure").with_encryption(EncryptionMode::Affine),
        ]
    }

    fn opaque_text_fields() -> Vec<FieldDef> {
        vec![
            FieldDef::categorical("legal_name").with_encryption(EncryptionMode::Opaque),
            FieldDef::categorical("address").with_encryption(EncryptionMode::Opaque),
        ]
    }

    fn indexed_fields() -> Vec<FieldDef> {
        vec![FieldDef::categorical("kind").with_encryption(EncryptionMode::Indexed)]
    }

    // ── v0.1 affine path (regression) ──

    #[test]
    fn test_derive_deterministic() {
        let seed = test_seed();
        let fields = affine_fields();
        let k1 = GaugeKey::derive(&seed, &fields);
        let k2 = GaugeKey::derive(&seed, &fields);
        for (a, b) in k1.transforms.iter().zip(k2.transforms.iter()) {
            match (a, b) {
                (FieldTransform::Affine { scale: s1, offset: o1 }, FieldTransform::Affine { scale: s2, offset: o2 }) => {
                    assert_eq!(s1, s2);
                    assert_eq!(o1, o2);
                }
                _ => panic!("Expected matching Affine transforms"),
            }
        }
    }

    #[test]
    fn test_affine_encrypt_decrypt_roundtrip() {
        let seed = test_seed();
        let fields = affine_fields();
        let key = GaugeKey::derive(&seed, &fields);

        let plain = vec![Value::Float(-31.9), Value::Float(65.0), Value::Float(1013.25)];
        let encrypted = key.encrypt_fiber(&plain, "test_bundle", &fields);
        let decrypted = key.decrypt_fiber(&encrypted, "test_bundle", &fields);

        for (p, d) in plain.iter().zip(decrypted.iter()) {
            match (p, d) {
                (Value::Float(a), Value::Float(b)) => {
                    assert!((a - b).abs() < 1e-10, "Roundtrip failed: {a} vs {b}");
                }
                _ => panic!("Type mismatch"),
            }
        }
    }

    #[test]
    fn test_affine_encrypted_values_differ_from_plain() {
        let seed = test_seed();
        let fields = affine_fields();
        let key = GaugeKey::derive(&seed, &fields);

        let plain = vec![Value::Float(22.5), Value::Float(65.0), Value::Float(1013.25)];
        let encrypted = key.encrypt_fiber(&plain, "test_bundle", &fields);
        for (p, e) in plain.iter().zip(encrypted.iter()) {
            assert_ne!(p, e, "Encrypted value should differ from plaintext");
        }
    }

    #[test]
    fn test_seed_from_hex() {
        let hex = "0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c";
        assert!(seed_from_hex(hex).is_ok());
    }

    #[test]
    fn test_seed_from_hex_bad_length() {
        assert!(seed_from_hex("abcd").is_err());
    }

    // ── Sprint B: Opaque (AEAD / AES-GCM-SIV) ──

    #[test]
    fn test_opaque_encrypt_decrypt_roundtrip_text() {
        let seed = test_seed();
        let fields = opaque_text_fields();
        let key = GaugeKey::derive(&seed, &fields);

        let plain = vec![
            Value::Text("Alice Smith".to_string()),
            Value::Text("123 Main St".to_string()),
        ];
        let encrypted = key.encrypt_fiber(&plain, "test_bundle", &fields);
        let decrypted = key.decrypt_fiber(&encrypted, "test_bundle", &fields);

        assert_eq!(plain, decrypted, "Opaque text roundtrip should be exact");
    }

    #[test]
    fn test_opaque_encrypt_decrypt_roundtrip_binary() {
        let seed = test_seed();
        let fields = vec![FieldDef::binary("payload").with_encryption(EncryptionMode::Opaque)];
        let key = GaugeKey::derive(&seed, &fields);

        let plain = vec![Value::Binary(vec![0u8, 1, 2, 3, 4, 5, 255])];
        let encrypted = key.encrypt_fiber(&plain, "b", &fields);
        let decrypted = key.decrypt_fiber(&encrypted, "b", &fields);
        assert_eq!(plain, decrypted);
    }

    #[test]
    fn test_opaque_same_plaintext_different_ciphertext() {
        let seed = test_seed();
        let fields = opaque_text_fields();
        let key = GaugeKey::derive(&seed, &fields);

        // Encrypt the same plaintext twice; ciphertexts should differ
        // (per-record random nonce).
        let plain = vec![Value::Text("hello".to_string()), Value::Text("world".to_string())];
        let ct1 = key.encrypt_fiber(&plain, "b", &fields);
        let ct2 = key.encrypt_fiber(&plain, "b", &fields);
        assert_ne!(ct1, ct2, "Opaque encrypt of same plaintext must produce different ciphertexts (random nonce)");
    }

    #[test]
    fn test_opaque_tamper_detection() {
        let seed = test_seed();
        let fields = opaque_text_fields();
        let key = GaugeKey::derive(&seed, &fields);

        let plain = vec![Value::Text("secret".to_string()), Value::Text("data".to_string())];
        let mut encrypted = key.encrypt_fiber(&plain, "b", &fields);

        // Flip a byte in the ciphertext of the first field.
        if let Value::Binary(ref mut bytes) = encrypted[0] {
            bytes[20] ^= 0x42;
        } else {
            panic!("expected Binary");
        }

        let result = std::panic::catch_unwind(|| {
            key.decrypt_fiber(&encrypted, "b", &fields);
        });
        assert!(result.is_err(), "Tampered AEAD ciphertext must fail decrypt");
    }

    #[test]
    fn test_opaque_aad_binds_to_field_position() {
        let seed = test_seed();
        let fields = opaque_text_fields();
        let key = GaugeKey::derive(&seed, &fields);

        // Encrypt two distinct plaintexts at fields 0 and 1.
        let plain = vec![Value::Text("legal".to_string()), Value::Text("address".to_string())];
        let encrypted = key.encrypt_fiber(&plain, "b", &fields);

        // Swap the two ciphertexts. Decrypt should fail authentication on
        // both because each ciphertext's AAD names the wrong field index.
        let swapped = vec![encrypted[1].clone(), encrypted[0].clone()];
        let result = std::panic::catch_unwind(|| {
            key.decrypt_fiber(&swapped, "b", &fields);
        });
        assert!(result.is_err(), "AAD must bind ciphertext to field position");
    }

    #[test]
    fn test_opaque_aad_binds_to_bundle_name() {
        let seed = test_seed();
        let fields = opaque_text_fields();
        let key = GaugeKey::derive(&seed, &fields);

        let plain = vec![Value::Text("a".to_string()), Value::Text("b".to_string())];
        let encrypted = key.encrypt_fiber(&plain, "bundle_one", &fields);

        // Decrypt with a different bundle name should fail.
        let result = std::panic::catch_unwind(|| {
            key.decrypt_fiber(&encrypted, "bundle_two", &fields);
        });
        assert!(result.is_err(), "AAD must bind ciphertext to bundle name");
    }

    #[test]
    fn test_opaque_round_trip_many_records() {
        let seed = test_seed();
        let fields = opaque_text_fields();
        let key = GaugeKey::derive(&seed, &fields);

        for i in 0..1000 {
            let plain = vec![
                Value::Text(format!("name_{i}")),
                Value::Text(format!("addr_{i}_456_main_st")),
            ];
            let encrypted = key.encrypt_fiber(&plain, "b", &fields);
            let decrypted = key.decrypt_fiber(&encrypted, "b", &fields);
            assert_eq!(plain, decrypted, "iteration {i}");
        }
    }

    #[test]
    fn test_opaque_ciphertext_is_binary_value() {
        let seed = test_seed();
        let fields = opaque_text_fields();
        let key = GaugeKey::derive(&seed, &fields);

        let plain = vec![Value::Text("abc".to_string()), Value::Text("def".to_string())];
        let encrypted = key.encrypt_fiber(&plain, "b", &fields);

        // Encrypted text should be stored as Value::Binary on disk.
        for v in &encrypted {
            assert!(matches!(v, Value::Binary(_)), "Opaque ciphertext should be Binary");
        }
    }

    #[test]
    fn test_opaque_ciphertext_size_includes_nonce_and_tag() {
        let seed = test_seed();
        let fields = opaque_text_fields();
        let key = GaugeKey::derive(&seed, &fields);

        // Plaintext is 5 bytes ("hello"). Type tag adds 1, so 6 bytes input
        // to AEAD. Output: 12 (nonce) + 6 (ct) + 16 (tag) = 34 bytes.
        let plain = vec![Value::Text("hello".to_string()), Value::Text("world".to_string())];
        let encrypted = key.encrypt_fiber(&plain, "b", &fields);
        if let Value::Binary(ref bytes) = encrypted[0] {
            assert_eq!(bytes.len(), 12 + 6 + 16, "AEAD blob = nonce | ct | tag");
        } else {
            panic!("Opaque ciphertext should be Binary");
        }
    }

    // ── Sprint C: Indexed (PRF / AES-256-CMAC) ──

    #[test]
    fn test_indexed_deterministic() {
        let seed = test_seed();
        let fields = indexed_fields();
        let key = GaugeKey::derive(&seed, &fields);

        // Same plaintext encrypted 1000 times → same ciphertext every time.
        let plain = vec![Value::Text("user_42".to_string())];
        let first = key.encrypt_fiber(&plain, "b", &fields);
        for _ in 0..1000 {
            let again = key.encrypt_fiber(&plain, "b", &fields);
            assert_eq!(first, again, "Indexed must be deterministic");
        }
    }

    #[test]
    fn test_indexed_distinct_plaintexts_distinct_ciphertexts() {
        let seed = test_seed();
        let fields = indexed_fields();
        let key = GaugeKey::derive(&seed, &fields);

        let mut ciphertexts = std::collections::HashSet::new();
        for i in 0..1000 {
            let plain = vec![Value::Text(format!("plaintext_{i}"))];
            let ct = key.encrypt_fiber(&plain, "b", &fields);
            let bytes = match &ct[0] {
                Value::Binary(b) => b.clone(),
                _ => panic!("expected Binary"),
            };
            assert!(
                ciphertexts.insert(bytes),
                "Distinct plaintexts must produce distinct ciphertexts (no collisions)"
            );
        }
    }

    #[test]
    fn test_indexed_equal_plaintexts_equal_ciphertexts() {
        // The whole point of Indexed: equality of plaintexts implies
        // equality of ciphertexts. This is what makes equality-search work
        // on the encrypted column.
        let seed = test_seed();
        let fields = indexed_fields();
        let key = GaugeKey::derive(&seed, &fields);

        let plain_a = vec![Value::Text("user_42".to_string())];
        let plain_b = vec![Value::Text("user_42".to_string())];
        let ct_a = key.encrypt_fiber(&plain_a, "b", &fields);
        let ct_b = key.encrypt_fiber(&plain_b, "b", &fields);
        assert_eq!(ct_a, ct_b, "Indexed equal-plaintext-equal-ciphertext invariant");
    }

    #[test]
    fn test_indexed_ciphertext_is_16_bytes() {
        let seed = test_seed();
        let fields = indexed_fields();
        let key = GaugeKey::derive(&seed, &fields);

        let plain = vec![Value::Text("anything".to_string())];
        let ct = key.encrypt_fiber(&plain, "b", &fields);
        if let Value::Binary(ref bytes) = ct[0] {
            assert_eq!(bytes.len(), 16, "AES-CMAC tag is exactly 16 bytes");
        } else {
            panic!("Indexed ciphertext should be Binary");
        }
    }

    #[test]
    fn test_indexed_decrypt_is_identity() {
        // PRF is one-way. decrypt returns the stored ciphertext as-is.
        let seed = test_seed();
        let fields = indexed_fields();
        let key = GaugeKey::derive(&seed, &fields);

        let plain = vec![Value::Text("abc".to_string())];
        let ct = key.encrypt_fiber(&plain, "b", &fields);
        let decrypted = key.decrypt_fiber(&ct, "b", &fields);
        assert_eq!(ct, decrypted, "Indexed decrypt is identity (PRF is one-way)");
    }

    #[test]
    fn test_indexed_encrypt_literal_for_equality_search() {
        // The query path: WHERE field = 'user_42' gets transformed by
        // encrypting the literal through the same PRF, then comparing.
        let seed = test_seed();
        let fields = indexed_fields();
        let key = GaugeKey::derive(&seed, &fields);

        // Stored value: PRF("user_42")
        let stored_plain = vec![Value::Text("user_42".to_string())];
        let stored_ct = key.encrypt_fiber(&stored_plain, "b", &fields);

        // Query literal: PRF("user_42") via encrypt_literal
        let literal = Value::Text("user_42".to_string());
        let query_ct = key.encrypt_literal(0, &literal, "b", &fields);

        assert_eq!(stored_ct[0], query_ct, "Equality search literal must match stored ciphertext");
    }

    #[test]
    fn test_indexed_different_keys_different_ciphertexts() {
        // Same plaintext under different seeds → different ciphertexts.
        let seed_a = test_seed();
        let mut seed_b = test_seed();
        seed_b[0] ^= 0xff;
        let fields = indexed_fields();
        let key_a = GaugeKey::derive(&seed_a, &fields);
        let key_b = GaugeKey::derive(&seed_b, &fields);

        let plain = vec![Value::Text("same_plaintext".to_string())];
        let ct_a = key_a.encrypt_fiber(&plain, "b", &fields);
        let ct_b = key_b.encrypt_fiber(&plain, "b", &fields);
        assert_ne!(ct_a, ct_b);
    }

    // ── Mixed-mode bundle: realistic jg_account-style schema ──

    #[test]
    fn test_mixed_mode_bundle_roundtrip() {
        // legal_name = OPAQUE, kind = INDEXED, score = AFFINE, attempts = no-encryption
        let fields = vec![
            FieldDef::categorical("legal_name").with_encryption(EncryptionMode::Opaque),
            FieldDef::categorical("kind").with_encryption(EncryptionMode::Indexed),
            FieldDef::numeric("score").with_encryption(EncryptionMode::Affine),
            FieldDef::numeric("attempts"),
        ];
        let seed = test_seed();
        let key = GaugeKey::derive(&seed, &fields);

        let plain = vec![
            Value::Text("Alice".to_string()),
            Value::Text("paid".to_string()),
            Value::Float(0.95),
            Value::Integer(3),
        ];
        let encrypted = key.encrypt_fiber(&plain, "acct", &fields);

        // legal_name: AEAD blob
        assert!(matches!(encrypted[0], Value::Binary(_)));
        // kind: PRF tag (16 bytes)
        if let Value::Binary(ref b) = encrypted[1] { assert_eq!(b.len(), 16); }
        // score: affine-encrypted float
        assert!(matches!(encrypted[2], Value::Float(_)));
        assert_ne!(encrypted[2], plain[2]);
        // attempts: no encryption
        assert_eq!(encrypted[3], plain[3]);

        let decrypted = key.decrypt_fiber(&encrypted, "acct", &fields);
        // legal_name decrypts back to plaintext text
        assert_eq!(decrypted[0], plain[0]);
        // kind: identity (PRF one-way)
        assert_eq!(decrypted[1], encrypted[1]);
        // score: affine inverse recovers plaintext
        if let (Value::Float(a), Value::Float(b)) = (&decrypted[2], &plain[2]) {
            assert!((a - b).abs() < 1e-10);
        } else {
            panic!("score should decrypt to Float");
        }
        // attempts: identity
        assert_eq!(decrypted[3], plain[3]);
    }
}
