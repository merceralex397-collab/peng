# Post-implementation report — LIB-001

## Result

Implemented the production-reachable Phase 0 prompt CRUD slice through one canonical Rust `Asset` aggregate, one concrete SQLite store and application service, four thin typed Tauri commands, and one typed frontend IPC caller. SQLite is bundled into the desktop binary through `rusqlite`'s `bundled` feature. No in-memory fallback, search/FTS, migration framework, bundle behavior, expanded capability, or Phase 1 subsystem was added.

Pinned delivery base: `3ee6e3549ffbe3deeb2e4ea1b2f5e34329ff82a1` on `main`.

## Files changed

- `src-tauri/Cargo.toml` — declares the sole new direct dependency, bundled `rusqlite`.
- `src-tauri/Cargo.lock` — locks the SQLite dependency and its required transitive crates.
- `src-tauri/src/lib.rs` — opens app-data `peng.sqlite3`, manages one `AssetService`, and registers exactly four commands.
- `src-tauri/src/domain/mod.rs` — exports the coherent domain surface.
- `src-tauri/src/domain/asset.rs` — defines the canonical aggregate, CRUD DTOs, open asset-type token validation, closed status/source values, forward-readable unknown fields, and boundary tests.
- `src-tauri/src/domain/error.rs` — defines and tests the structured `PengError` contract.
- `src-tauri/src/application/mod.rs` — exports the concrete service.
- `src-tauri/src/application/assets.rs` — validates and coordinates CRUD and tests rejected-input no-mutation behavior.
- `src-tauri/src/storage/mod.rs` — exports the concrete store.
- `src-tauri/src/storage/assets.rs` — implements the bounded baseline table, SQLite UUID generation, JSON persistence, atomic CRUD, optimistic revisions, and real file-backed tests.
- `src-tauri/src/commands/mod.rs` — contains the command module boundary.
- `src-tauri/src/commands/assets.rs` — delegates each typed handler once to `AssetService`.
- `src/lib/assets.ts` — provides the sole typed Tauri IPC contract and four invoke functions.
- `src/App.svelte` — provides a reachable prompt create/reload/update/delete workflow with pending, success, and structured-error states.
- `src/styles.css` — preserves the three-region slate/teal shell while making the bounded caller usable and focus-visible.

Generated Tauri schemas were observed as stat-dirty on Windows after builds but have no byte-level Git diff and are not part of this change. Baseline `src-tauri/build.rs` and `src-tauri/src/main.rs` remain byte-identical to the pinned base.

## Governing-document mapping

- `docs/prd/PRD-001-core-library.md`: one shared `Asset` model and one canonical local CRUD path now exist; no prompt-only parallel system was introduced.
- `docs/frd/FRD-001-phase-zero-vertical-slice.md`: the stable identity, prompt content/metadata, and reachable create/read/update/delete foundation are implemented for the later Phase 0 round-trip work.
- `docs/adr/ADR-002-canonical-storage.md`: SQLite is the only production store and mutations use one mutable working record with explicit revision conflict detection.

## Validation evidence

Final revised-plan rail, run from `.worktrees/lib-001`:

- `rustfmt --edition 2024 --check src-tauri/src/domain/mod.rs src-tauri/src/domain/asset.rs src-tauri/src/domain/error.rs src-tauri/src/application/mod.rs src-tauri/src/application/assets.rs src-tauri/src/storage/mod.rs src-tauri/src/storage/assets.rs src-tauri/src/commands/mod.rs src-tauri/src/commands/assets.rs src-tauri/src/lib.rs` — exit 0.
- `cargo test --manifest-path src-tauri/Cargo.toml` — exit 0; 7 passed, 0 failed, plus 0 main/doc tests.
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings` — exit 0.
- `npm run check` — exit 0; 0 errors and 0 warnings.
- `npm run build` — exit 0; Vite production build completed.
- `npm run tauri build -- --no-bundle` — exit 0; produced `src-tauri/target/release/peng.exe` with bundled SQLite linked.
- `git diff --check` — exit 0.
- Scope audit: `git diff --name-only` plus untracked-file census contained only packet-authorized paths; the two generated schema entries reported by Windows status have no real diff.

Preserved failure history:

- The first `cargo test --manifest-path src-tauri/Cargo.toml domain` exited 1 because the initial Tauri macro path used command re-exports, a test required `AssetStore: Debug`, and imports were unused. Those defects were corrected in authorized files; the later domain and full suites passed.
- The original repo-wide `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check` exited 1 because it inspected forbidden pinned-baseline trailing blank lines. The controller performed the one allowed replan, replacing that contradictory command with the scoped Rust 2024 formatting rail above; the revised rail passed.

## Risks and follow-ups

- LIB-002 must replace the bounded `CREATE TABLE IF NOT EXISTS` bootstrap with the single ordered migration system and add WAL, foreign keys, restart recovery, FTS5, and search without retaining a parallel initializer.
- UI-001 must extend the existing `src/lib/assets.ts` contract and production caller into autosave and search behavior.
- BND-001/BND-002 must map this same aggregate without lossy conversion.
- Timestamp generation is currently SQLite-owned as planned; later migration work must preserve stored values.

## Required merged-result verification

On the exact merge SHA, rerun the complete revised validation rail above; confirm `src-tauri/target/release/peng.exe` is produced; inspect command registration and unchanged capabilities; and rerun file-backed CRUD/reopen, malformed JSON, not-found, conflict, and rejected-input no-mutation tests. Verify the merge diff contains no search/FTS, migration framework, bundle path, compatibility layer, fallback store, second aggregate, or additional dependency.

## Remediation round 1

- **F-001 — schema-version component count:** changed `src-tauri/src/domain/asset.rs` so `schemaVersion` accepts exactly two non-empty decimal components (`major.minor`) and rejects both too few (`1`) and too many (`1.2.3`) components. Added focused assertions for both rejected values without weakening existing boundary coverage.
- Remediation commit: `f29b9914ceca3fbbc23675ee7e865b2148e805e7`.
- Existing PR updated in place: https://github.com/merceralex397-collab/peng/pull/1; no second PR was created.
- Public review finding: https://github.com/merceralex397-collab/peng/pull/1#issuecomment-5517442078.

Remediation validation from `.worktrees/lib-001`: focused domain Rust 2024 formatting and domain tests exit 0; the complete scoped Rust 2024 formatting rail exits 0; full Rust tests exit 0 with 7 passed; clippy with `-D warnings` exits 0; Svelte check exits 0 with 0 errors and 0 warnings; Vite build exits 0; no-bundle Tauri release build exits 0 and produces `src-tauri/target/release/peng.exe`; `git diff --check` exits 0. The delta contains only the authorized domain source file; generated schema paths remain byte-identical despite Windows stat dirtiness.
