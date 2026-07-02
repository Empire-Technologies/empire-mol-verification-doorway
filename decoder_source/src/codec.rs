//! Payload decode dispatch.
//!
//! Unknown payload selectors fail closed.

use crate::transform::apply_transform;
use std::io::Read;

type BoxErr = Box<dyn std::error::Error>;

fn decompress_a(data: &[u8]) -> Result<Vec<u8>, BoxErr> {
    use lz4_flex::frame::FrameDecoder;
    let mut decoder = FrameDecoder::new(data);
    let mut out = Vec::new();
    decoder.read_to_end(&mut out)?;
    Ok(out)
}

fn decompress_b(data: &[u8]) -> Result<Vec<u8>, BoxErr> {
    Ok(zstd::decode_all(data)?)
}

/// Decode a payload and return restored bytes. The caller verifies
/// the result against the signed archive metadata.
pub fn decode_payload(selector: u8, data: &[u8]) -> Result<Vec<u8>, BoxErr> {
    match selector {
        0x00 => Ok(data.to_vec()),
        0x01 => decompress_a(data),
        0x02 => {
            let mut v = decompress_a(data)?;
            apply_transform(&mut v);
            Ok(v)
        }
        0x03 => decompress_b(data),
        0x04 => {
            let mut v = decompress_b(data)?;
            apply_transform(&mut v);
            Ok(v)
        }
        other => Err(format!("unknown selector 0x{:02x}", other).into()),
    }
}
