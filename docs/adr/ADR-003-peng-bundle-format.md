# ADR-003: Peng bundle format 1.0

## Status

Accepted

## Decision

A `.peng` file is a deterministic ZIP-compatible archive described by `Peng_Design_Pack/Peng_Bundle_1.0.schema.json`, not a database backup.

## Consequences

- `mimetype` is the first uncompressed entry and contains `application/vnd.peng.bundle+zip`.
- Remaining entries use stable lexical order, safe forward-slash relative paths, UTF-8, and fixed timestamps in deterministic mode.
- SHA-256 protects every regular entry except the checksum file and signatures.
- Import validates path safety, schema, checksums, sizes, counts, and compression ratios before mutation.
- Secret values and automatic execution are prohibited.
- Unknown asset types and properties remain readable/preservable where the v1 contract requires it.

