//! Empire MOL decoder — public reference implementation.
//!
//! This tool verifies Empire-signed `.mol` archives and restores the
//! original bytes. Invalid, unsigned, malformed, or tampered archives
//! fail closed.

mod codec;
mod params;
mod transform;
mod verify;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

use serde::Deserialize;
use sha2::{Digest, Sha256};

const MAGIC: &[u8; 4] = b"MOL4";
const VERSION: u32 = 4;

type BoxErr = Box<dyn std::error::Error>;

fn vread(buf: &[u8], i: usize) -> Result<(u64, usize), BoxErr> {
    let mut n: u64 = 0;
    let mut shift: u32 = 0;
    let mut pos = i;
    loop {
        if pos >= buf.len() {
            return Err("truncated varint".into());
        }
        if shift >= 64 {
            return Err("varint overflow".into());
        }
        let b = buf[pos];
        pos += 1;
        n |= ((b & 0x7F) as u64) << shift;
        if b & 0x80 == 0 {
            break;
        }
        shift += 7;
    }
    Ok((n, pos))
}

fn sha256_hex(data: &[u8]) -> String {
    hex::encode(Sha256::digest(data))
}

fn compute_check(item_hex: &[String]) -> String {
    if item_hex.is_empty() {
        return sha256_hex(b"");
    }
    let mut level: Vec<Vec<u8>> = item_hex
        .iter()
        .map(|h| hex::decode(h).expect("valid hex value"))
        .collect();
    while level.len() > 1 {
        let mut next: Vec<Vec<u8>> = Vec::with_capacity((level.len() + 1) / 2);
        let mut i = 0;
        while i < level.len() {
            let left = &level[i];
            let right = if i + 1 < level.len() {
                &level[i + 1]
            } else {
                &level[i]
            };
            let mut combined = Vec::with_capacity(left.len() + right.len());
            combined.extend_from_slice(left);
            combined.extend_from_slice(right);
            next.push(Sha256::digest(&combined).to_vec());
            i += 2;
        }
        level = next;
    }
    hex::encode(&level[0])
}

#[derive(Deserialize)]
struct Item {
    off: u64,
    len: u64,
    sha: String,
    #[serde(rename = "codec")]
    selector: u8,
}

#[derive(Deserialize)]
struct FileMeta {
    name: String,
    size: u64,
    sha256: String,
    #[serde(rename = "snapshot_root")]
    check: String,
    #[serde(rename = "leaves")]
    items: Vec<Item>,
}

#[derive(Deserialize)]
struct Header {
    file: FileMeta,
}

fn round_n(v: f64, places: u32) -> f64 {
    let m = 10f64.powi(places as i32);
    (v * m).round() / m
}

fn safe_output_name(name: &str) -> Result<&str, BoxErr> {
    if name.is_empty()
        || name.contains('/')
        || name.contains('\\')
        || name.contains(':')
        || name == "."
        || name == ".."
    {
        return Err("archive contains an unsafe output filename".into());
    }
    Ok(name)
}

fn decode_archive(in_file: &Path, out_dir: &Path) -> Result<(), BoxErr> {
    fs::create_dir_all(out_dir)?;
    let raw = fs::read(in_file)?;
    let full_sha = sha256_hex(&raw);

    let buf = verify::verify_signature(&raw)?;

    let mut p = 0usize;
    if buf.len() < 16 || &buf[p..p + 4] != MAGIC {
        return Err("bad archive header".into());
    }
    p += 4;

    let ver = u32::from_le_bytes(buf[p..p + 4].try_into()?);
    p += 4;
    if ver != VERSION {
        return Err("unsupported archive version".into());
    }

    let jlen = u64::from_le_bytes(buf[p..p + 8].try_into()?) as usize;
    p += 8;

    if p.checked_add(jlen).ok_or("header length overflow")? > buf.len() {
        return Err("archive header is truncated".into());
    }

    let header: Header = serde_json::from_slice(&buf[p..p + jlen])?;
    p += jlen;
    let out_name = safe_output_name(&header.file.name)?;

    let t0 = Instant::now();

    let mut payloads: HashMap<String, Vec<u8>> = HashMap::new();

    while p < buf.len() {
        if p.checked_add(33).ok_or("payload header overflow")? > buf.len() {
            return Err("truncated payload header".into());
        }
        let cid = hex::encode(&buf[p..p + 32]);
        p += 32;
        let flag = buf[p];
        p += 1;
        let (comp_len, new_p) = vread(&buf, p)?;
        p = new_p;
        let comp_len_usize = usize::try_from(comp_len).map_err(|_| "payload length too large")?;
        if p.checked_add(comp_len_usize)
            .ok_or("payload length overflow")?
            > buf.len()
        {
            return Err("truncated payload body".into());
        }
        let payload = &buf[p..p + comp_len_usize];
        p += comp_len_usize;

        let restored = codec::decode_payload(flag, payload)?;
        if sha256_hex(&restored) != cid {
            return Err("internal verification failed".into());
        }
        if payloads.insert(cid, restored).is_some() {
            return Err("duplicate payload id".into());
        }
    }

    let item_hashes: Vec<String> = header.file.items.iter().map(|l| l.sha.clone()).collect();
    let recomputed_check = compute_check(&item_hashes);
    if recomputed_check != header.file.check {
        return Err("archive integrity verification failed".into());
    }

    let out_size_usize =
        usize::try_from(header.file.size).map_err(|_| "declared output is too large")?;
    let mut out_buf = vec![0u8; out_size_usize];
    for item in &header.file.items {
        let payload = payloads.get(&item.sha).ok_or("archive incomplete")?;
        let _ = item.selector;
        let start = usize::try_from(item.off).map_err(|_| "item offset too large")?;
        let item_len = usize::try_from(item.len).map_err(|_| "item length too large")?;
        let end = start.checked_add(item_len).ok_or("item range overflow")?;
        if end > out_buf.len() || item_len > payload.len() {
            return Err("archive item range is invalid".into());
        }
        out_buf[start..end].copy_from_slice(&payload[..item_len]);
    }

    let restored_sha = sha256_hex(&out_buf);
    if restored_sha != header.file.sha256 {
        return Err("output verification failed".into());
    }

    let out_path = out_dir.join(out_name);
    let mut f = fs::File::create(&out_path)?;
    f.write_all(&out_buf)?;
    f.flush()?;
    drop(f);

    let t_total = t0.elapsed().as_secs_f64();
    let out_size = fs::metadata(&out_path)?.len();

    let summary = serde_json::json!({
        "verified": true,
        "signature_verified": true,
        "comp_sha256": full_sha,
        "restored_sha256": restored_sha,
        "sha256_match": true,
        "bytes_in": header.file.size,
        "bytes_out": out_size,
        "unpack_time_s": round_n(t_total, 4),
    });
    println!("{}", serde_json::to_string_pretty(&summary)?);

    Ok(())
}

fn main() -> Result<(), BoxErr> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("mol_decoder — decode Empire archives");
        eprintln!();
        eprintln!("usage: mol_decoder <input> <outdir>");
        eprintln!();
        eprintln!("Archives must be signed by Empire. Unsigned or tampered");
        eprintln!("archives are refused.");
        std::process::exit(1);
    }
    let input = PathBuf::from(&args[1]);
    let outdir = PathBuf::from(&args[2]);
    decode_archive(&input, &outdir)
}
