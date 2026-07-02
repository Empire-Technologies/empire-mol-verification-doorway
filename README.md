# Empire MOL Verification Doorway

Empire MOL is a lossless archive and verification system. This repository is the lightweight public doorway for the first public verification release, while the full canonical artifact lives on Zenodo.

## Get the real article / full public artifact here first

[https://doi.org/10.5281/zenodo.21115968](https://doi.org/10.5281/zenodo.21115968)

This GitHub repository is not the full release artifact.

Canonical full artifact:
[https://doi.org/10.5281/zenodo.21115968](https://doi.org/10.5281/zenodo.21115968)

## Results first

Selected public results from the canonical verification pack:

| Sample | Domain | Ratio | Saved |
|---|---|---:|---:|
| `raw_bias_1gb` | synthetic structured control | 471.52x | 99.79% |
| `enwik9` | natural-language text | 2.81x | 64.44% |
| `nci` | scientific binary image data | 10.08x | 90.08% |
| `samba` | source-code tree snapshot | 3.66x | 72.71% |
| `silesia.zip` | already-compressed edge case | 1.00x | -0.14% |

More detail: [RESULTS.md](RESULTS.md)

## Verification claim

The public claim is narrow and checkable:

The included signed `.mol` samples in the Zenodo pack can be verified, decoded/restored, and checked against declared SHA-256 hashes exactly.

This repository is not the canonical artifact. It is the doorway.

## What this repository is for

- show the public result surface clearly
- show the verification boundary clearly
- point directly to the canonical Zenodo pack
- provide the public decoder source/build path

## Why this exists

The goal of this doorway is simple: let technical readers see the bounded claim, inspect the public decoder path, and jump to the full verification artifact without turning GitHub into a second warehouse.

## Full verification pack

Download the full public verification pack from Zenodo:

[https://doi.org/10.5281/zenodo.21115968](https://doi.org/10.5281/zenodo.21115968)

That pack contains the signed sample set, public verification materials, and the canonical public proof bundle.

## Public decoder source

Open-source decoder/recreation path is included here:

- [decoder_source/](decoder_source/)

Build context:

```text
cd decoder_source
cargo build --release
```

Verification notes:

- [VERIFY.md](VERIFY.md)
- [receipts/verification_scope.md](receipts/verification_scope.md)
- [SECURITY.md](SECURITY.md)

## What this repository does not contain

- the full 544 MB verification pack
- large samples
- decoded/restored outputs
- encoder
- private keys
- commercial result bank
- unreleased benchmark bank

## Claim boundary

This repository does not claim:

- universal superiority
- arbitrary-input performance dominance
- all-file dominance
- encoder disclosure
- full commercial benchmark release
