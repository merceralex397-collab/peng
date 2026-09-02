# Threat model

## Protected assets

- canonical library content and history;
- local files outside Peng's managed directories;
- credentials and secret values;
- integrity of imports, exports, and target deployments;
- user intent at every write or execution boundary.

## Untrusted inputs

Imported archives, asset text, JSON/YAML, attachments, launch definitions, URLs, target directories, and generated deployment previews are untrusted.

## Required controls

- Reject unsafe or ambiguous archive paths and non-regular entries before extraction.
- Enforce archive size, expansion, entry-count, compression-ratio, and document-size limits.
- Validate schema and all checksums before database or destination mutation.
- Stage file changes and commit imports transactionally.
- Never evaluate or execute imported content during import or preview.
- Keep secret values out of bundles, logs, previews, and ordinary asset records.
- Represent process commands and arguments structurally and permit execution only through a later explicit allow-listed action.
- Grant each Tauri window only the native capabilities it currently needs.
- Apply a local-only Content Security Policy with no remote scripts.

