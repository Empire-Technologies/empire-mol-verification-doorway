//! Archive authenticity verification.
//!
//! The archive carries a 64-byte Ed25519 signature appended to the
//! end of the file. Verification binds the signature to the archive's
//! identity (filename, sizes, and content). Any tampering breaks the
//! signature.
//!
//! The signing private key stays on Empire infrastructure. This decoder
//! holds only the matching public key as a compile-time constant.
//!
//! Verification is mandatory and fail-loud. There is no override.

use ed25519_dalek::{Signature, Verifier, VerifyingKey, SIGNATURE_LENGTH};
use serde::Deserialize;
use sha2::{Digest, Sha256};

const MAGIC: &[u8; 4] = b"MOL4";

/// Empire validation public key — Ed25519, 32 raw bytes.
///
/// The matching private key stays on Empire infrastructure. Any
/// archive not signed by that private key fails this check.
pub const EMPIRE_VERIFICATION_PUBKEY: [u8; 32] = [
    0x6f, 0x79, 0xe3, 0xd8, 0x11, 0xe1, 0xf4, 0x98, 0x38, 0xa0, 0xab, 0xdf, 0x7f, 0x67, 0x46, 0xd2,
    0x12, 0x38, 0x28, 0x30, 0x13, 0x26, 0xa4, 0xb9, 0x15, 0xe2, 0x7d, 0x7a, 0xdc, 0x2b, 0x43, 0x74,
];

/// Domain separator — 32 bytes.
const DOMAIN: [u8; 32] = *b"EMPIRE_MOL4_ARCHIVE_COMMIT_V1\x00\x00\x00";

#[derive(Deserialize)]
struct HeaderFileMeta {
    name: String,
    size: u64,
}

#[derive(Deserialize)]
struct HeaderForCommit {
    file: HeaderFileMeta,
}

struct HeaderBind {
    name: String,
    size: u64,
}

/// Read declared archive identity from the header for use in
/// signature verification.
fn read_binding(body: &[u8]) -> Result<HeaderBind, Box<dyn std::error::Error>> {
    if body.len() < 16 || &body[0..4] != MAGIC {
        return Err("bad archive header during binding probe".into());
    }
    let jlen = u64::from_le_bytes(body[8..16].try_into()?) as usize;
    if body.len() < 16 + jlen {
        return Err("header length overflow during binding probe".into());
    }
    let header: HeaderForCommit = serde_json::from_slice(&body[16..16 + jlen])?;
    Ok(HeaderBind {
        name: header.file.name,
        size: header.file.size,
    })
}

/// Build the signed value.
fn build_signed_value(
    body: &[u8],
    name: &str,
    original_size: u64,
    compressed_size: u64,
) -> Vec<u8> {
    let body_hash = Sha256::digest(body);
    let name_bytes = name.as_bytes();
    let name_len = name_bytes.len() as u64;
    let mut out = Vec::with_capacity(32 + 8 + name_bytes.len() + 8 + 8 + 32);
    out.extend_from_slice(&DOMAIN);
    out.extend_from_slice(&name_len.to_le_bytes());
    out.extend_from_slice(name_bytes);
    out.extend_from_slice(&original_size.to_le_bytes());
    out.extend_from_slice(&compressed_size.to_le_bytes());
    out.extend_from_slice(&body_hash);
    out
}

/// Verify an archive's trailing signature using the embedded
/// Empire public key.
///
/// On success returns the archive bytes with the signature stripped.
/// On failure returns an Err — fail-loud, no override.
pub fn verify_signature(raw: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if raw.len() < SIGNATURE_LENGTH + 16 {
        return Err("archive too small to be signed".into());
    }

    let split = raw.len() - SIGNATURE_LENGTH;
    let body = &raw[..split];
    let sig_bytes = &raw[split..];

    let bind = read_binding(body)?;
    let compressed_size = body.len() as u64;

    let signed_value = build_signed_value(body, &bind.name, bind.size, compressed_size);

    let vkey = VerifyingKey::from_bytes(&EMPIRE_VERIFICATION_PUBKEY)
        .map_err(|_| "embedded public key rejected by Ed25519 parser")?;

    let sig = Signature::from_bytes(
        sig_bytes
            .try_into()
            .map_err(|_| "signature length mismatch")?,
    );

    vkey.verify(&signed_value, &sig)
        .map_err(|_| "archive signature rejected — not produced by Empire")?;

    Ok(body.to_vec())
}
