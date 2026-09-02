# Phase 0 shared context

Deliver only the Phase 0 vertical slice from `docs/frd/FRD-001-phase-zero-vertical-slice.md`.

The canonical production path is Tauri 2 + Svelte 5 + TypeScript + Vite, with Rust application services and SQLite. The frontend must use typed Tauri commands and must not access SQL, unrestricted paths, or arbitrary processes. Bundle parsing, validation, hashing, archive safety, and transactional apply remain in Rust.

The frozen Phase 0 proof is: create a prompt, persist and find it after restart, export a deterministic `.peng`, delete the local prompt, import the archive, and recover the same identity, content, and metadata. Negative cases must leave the library unchanged and must never execute imported content.

Work remains pre-release: replace superseded scaffold behavior instead of adding compatibility paths. Do not add Phase 1 organisation/history features, Phase 2 target adapters, or Phase 3 operational capabilities.
