# Security Scope

## In scope

- `.mol` archive signature verification.
- Refusing unsigned, malformed, or tampered archives.
- Restoring valid included samples byte-for-byte.
- Preventing archive filenames from writing outside the requested
  output directory.

## Out of scope

- Encoder behavior.
- Private signing infrastructure.
- Commercial licensing.
- Runtime protection.
- Corpus-wide benchmark claims.

## Key material

This repository includes only the Empire validation public key. Public
keys are safe to distribute. The matching private signing key is not
included and must never be published.

## Reporting

Report decoder verification or restoration defects directly to Empire.
Do not publish a suspected verification bypass before disclosure is
coordinated.

