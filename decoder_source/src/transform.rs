//! Archive restore helper.

use crate::params::derive_offset;

/// Apply the restore helper in place.
#[inline]
pub fn apply_transform(buf: &mut [u8]) {
    let s = derive_offset();
    let n = buf.len();
    if n <= s {
        return;
    }
    for i in (0..(n - s)).rev() {
        buf[i] ^= buf[i + s];
    }
}
