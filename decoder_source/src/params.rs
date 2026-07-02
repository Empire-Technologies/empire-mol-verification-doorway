//! Restore parameter derivation.

/// Decoder parameter seed.
const OFFSET_SEED: &str = "x7q3kp9mz2vt";

/// Returns a derived decoder parameter.
#[inline]
pub fn derive_offset() -> usize {
    OFFSET_SEED.len() - 4
}
