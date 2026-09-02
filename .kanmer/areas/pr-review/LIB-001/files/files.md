# Files — LIB-001

## Where the change lands

| Path | Why |
|---|---|
| `src-tauri/src/lib.rs` | Register the final typed command handlers and construct the concrete application dependency they call. Today this is only the generated Tauri runner; incorrect wiring here creates unreachable code. |
| `src-tauri/src/domain/asset.rs` | New canonical `Asset`, prompt type data, validated IDs/type/status/source/tags, and request/response types. This is the shared contract consumed by persistence, bundles, and UI commands, so accidental storage- or harness-specific fields would couple later work. |
| `src-tauri/src/domain/error.rs` | New structured `PengError` contract with stable codes and field detail. Serialization shape is a frontend-visible boundary. |
| `src-tauri/src/domain/mod.rs` | Export only the coherent domain surface; avoid a second competing model. |
| `src-tauri/src/application/assets.rs` | New smallest concrete application operation(s) between thin Tauri handlers and the domain. Its exact production behavior depends on resolving the persistence/wiring question; it must not become a speculative service framework. |
| `src-tauri/src/application/mod.rs` | Module boundary for the one current application operation, only if needed by the resolved plan. |
| `src-tauri/src/commands/assets.rs` | New typed Tauri request handlers that validate/delegate once and serialize typed results/errors. No SQL, filesystem access, or arbitrary commands. |
| `src-tauri/src/commands/mod.rs` | Collect command registration without duplicating domain logic. |
| `src-tauri/Cargo.toml` | Add only dependencies explicitly approved by the final ticket brief. UUID generation/validation and timestamps cannot be imported merely because crates are transitive in `Cargo.lock`. |
| `src-tauri/Cargo.lock` | Mechanical result only if an approved direct dependency changes resolution; review for unexpected package churn. |
| `src-tauri/tests/asset_domain.rs` or colocated Rust unit tests | Prove valid prompt construction, each realistic invalid field, stable error serialization, unknown-but-valid asset type preservation, tag uniqueness, and command delegation. Test location should follow the smallest arrangement supported by visibility needs. |
| `src/App.svelte` and a focused typed IPC client under `src/` | Only in scope if the wiring question resolves that LIB-001 owns an actual production caller; otherwise UI changes remain UI-001 and must not be pulled forward. |

## Context files

| Path | What it tells the implementer |
|---|---|
| `AGENTS.md` | Peng is unreleased; use one Asset aggregate, keep commands thin, do not add packages without brief approval, do not retain fallbacks, and do not call registered/test-only code done. |
| `docs/frd/FRD-001-phase-zero-vertical-slice.md` | Exact Phase 0 prompt fields and the end-to-end lifecycle that later tickets must preserve. |
| `docs/prd/PRD-001-core-library.md` | The shared asset model is intentionally broader than prompts, while Phase 0 should implement only the smallest required slice. |
| `docs/adr/ADR-002-canonical-storage.md` | One mutable draft and immutable snapshots live in SQLite; search indexes and generated files are not canonical. |
| `docs/architecture.md` | Required dependency direction: Svelte → typed IPC → thin commands → application services → repositories/bundles/policy. |
| `Peng_Design_Pack/Peng_Axiomatic_Design_Spec.md` | Canonical fields, prompt type data, error shape, autosave/version semantics, and independence constraints. Treat performance values as targets, not current claims. |
| `Peng_Design_Pack/Peng_Bundle_1.0.schema.json` | Concrete portable v1 field limits and open/closed value sets; the domain must not make later valid round trips impossible. |
| `src-tauri/src/lib.rs` | Current construction root and command registration point; no service/repository exists yet. |
| `src-tauri/Cargo.toml` | Current direct dependency boundary is `serde`, `serde_json`, and `tauri`. |
| `src-tauri/capabilities/default.json` | Current least-privilege webview capability; domain IPC does not justify filesystem/process expansion. |
| `src/App.svelte` | Current UI is a disabled static scaffold and therefore not yet a caller of any Rust command. |
| `HZN-001/context.md` | Batch constraint: only Phase 0, canonical Rust services, no in-memory compatibility path, and exact lifecycle proof. |

## Ripple effects

- LIB-002 must reuse these domain structs and structured errors rather than defining database-owned duplicates; schema/migration mapping must preserve the same UUID and draft fields.
- UI-001 must consume the serialized command DTO/error contract through one typed client and provide the first user-facing caller if that caller is not assigned here.
- BND-001 and BND-002 must map the same identity/status/source/type-data semantics to and from the v1 schema without lossy converters.
- Domain validation tests become prerequisites for persistence, search, and bundle negative-path tests. A later rename or serialization change affects every consumer and should happen now while unreleased, not through aliases.
- If an approved dependency is added, Cargo resolution and build reproducibility change; frontend dependencies and Tauri capabilities should remain unchanged.
- Documentation needs updating only if implementation makes an intentional contract decision that differs from the current design/schema; do not rewrite the design merely to match accidental code.

## Out of scope

- SQLite migrations, WAL/foreign-key setup, repositories, FTS5, restart recovery, and search behavior owned by LIB-002.
- Prompt editor/autosave/search interaction, keyboard behavior, and visual design owned by UI-001 unless the ticket boundary is explicitly amended to require a minimal production caller here.
- Bundle ZIP creation, checksums, schema embedding, import staging, conflicts, and rollback owned by BND-001/BND-002.
- Attachments, notes, collections, relationships, immutable snapshot UI, all five full editors, backup/restore, target adapters, MCP/plugin execution, Git, sync, and cloud behavior.
- Compatibility shims, dual domain models, in-memory production fallbacks, generic repository frameworks, event buses, or speculative extension points.
