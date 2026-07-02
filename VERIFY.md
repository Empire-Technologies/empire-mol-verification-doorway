# Verify Empire MOL Public Pack

## Use the full public artifact

If you want the real article / full public verification artifact, download it from Zenodo:

[https://doi.org/10.5281/zenodo.21115968](https://doi.org/10.5281/zenodo.21115968)

This GitHub repository does not replace that artifact.

The canonical public verification artifact is on Zenodo:

[https://doi.org/10.5281/zenodo.21115968](https://doi.org/10.5281/zenodo.21115968)

## Procedure

1. Download the full public verification pack from Zenodo.
2. Extract it.
3. Open `START_HERE.md`.
4. Open `1_VERIFY`.
5. Run `verify.bat`.
6. Review generated decoded outputs in `2_RESULTS/decoded`.
7. Confirm the verifier reports SHA-256 pass/fail.

## Success means

For the included signed sample set:

- the samples verified
- the decoder restored the files
- the decoded/restored bytes matched the declared SHA-256 values

## Failure means

Stop and inspect:

- verifier output
- expected hashes
- sample integrity
- local environment differences

## Decoder source / recreation path

Public decoder source is included in this repository under:

- `decoder_source/`

The full verification run still depends on the canonical Zenodo pack layout and files.

## Boundary

Success does not prove:

- encoder disclosure
- arbitrary-input compression performance
- universal dominance
- the full internal production method
