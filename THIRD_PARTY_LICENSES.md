# Third-Party Licenses

This public decoder depends on third-party open-source components. **Each remains
under its own license**, and those licenses govern the respective components.
Empire gratefully acknowledges these projects and their maintainers.

| Component | Role (high level) | License (as published by the project) |
|---|---|---|
| LZ4 / lz4_flex | compression component | BSD-2-Clause / MIT |
| Zstandard / zstd | compression component | BSD (dual BSD/GPL upstream) |
| RustCrypto SHA-2 (sha2) | SHA-256 hashing | MIT / Apache-2.0 |
| serde, serde_json | (de)serialization | MIT / Apache-2.0 |
| hex | hex encoding/decoding | MIT / Apache-2.0 |
| ed25519-dalek | signature verification | BSD-3-Clause |
| Rust standard library / toolchain | language runtime | MIT / Apache-2.0 |

Empire's contribution in this release is **orchestration and verification** —
coordinating these established components and verifying results. The compression
and cryptographic primitives listed above are the work of their respective
projects and **remain under their own licenses**.

For exact license texts and the precise versions used, see each project's own
repository and the pinned dependency versions in `Cargo.lock`.

**Empire gratefully acknowledges these projects and their maintainers.**
