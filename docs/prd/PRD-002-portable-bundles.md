# PRD-002: Safe portable bundles

## Goal

Export and import an exact selected asset graph as one inspectable, recoverable `.peng` file without leaking secrets or executing content.

## Requirements

- Resolve required dependencies and let users choose optional links and data categories before export.
- Write a deterministic ZIP-compatible v1 archive with a versioned manifest, schema, asset folders, relationships, and SHA-256 checksums.
- Preview and validate imports before mutation, including conflicts and security findings.
- Apply accepted imports atomically and leave the library unchanged on invalid, cancelled, or failed imports.
- Export secret references only; never secret values.

## Out of scope

Encrypted secret transfer, automatic installation, execution, and opaque database backups.

