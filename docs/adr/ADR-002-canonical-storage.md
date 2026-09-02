# ADR-002: Canonical storage and versions

## Status

Accepted

## Decision

Use one canonical Asset aggregate in SQLite with FTS5. Store common metadata, text, structured type data, notes, relationships, and receipts in the database; store larger or binary attachments in a managed content directory by relative path and hash.

Each asset has one mutable autosaved working draft and explicit immutable snapshots. Generated harness files are projections, never canonical records.

## Consequences

- SQLite foreign keys, WAL, ordered migrations, and transactional writes are mandatory.
- Attachment bytes are staged, hashed, and atomically moved before their database reference commits.
- Search indexes are derived and rebuildable.
- Type editors share organisation, history, relationship, and bundle services.

