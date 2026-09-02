# Architecture map

```text
Svelte UI
  -> typed IPC client
    -> thin Tauri commands
      -> Rust application services
        -> repositories (SQLite + managed attachments)
        -> bundle parser/writer
        -> policy boundary
        -> target adapters (Phase 2)
```

The UI owns interaction state and presentation. Rust owns validation, persistence, archives, filesystem mutation, secrets boundaries, and later target application. SQLite is authoritative for metadata and text; the attachment directory is content storage referenced transactionally by relative path and hash.

The first production path is the Phase 0 prompt lifecycle in `docs/frd/FRD-001-phase-zero-vertical-slice.md`.

