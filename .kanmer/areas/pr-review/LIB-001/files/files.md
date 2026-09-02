# Files — LIB-001

## Where the change lands

| Path | Why |
|---|---|
| `src-tauri/src/lib.rs` | Construct the concrete application state and register the final typed CRUD commands. Incorrect wiring here leaves otherwise-correct code unreachable. |
| `src-tauri/src/domain/asset.rs` | Add the canonical `Asset`, prompt type data, validated ID/type/status/source/tag values, and request/response types shared by persistence, bundles, and UI. |
| `src-tauri/src/domain/error.rs` | Add the serializable `PengError` contract and stable validation/not-found/storage codes. |
| `src-tauri/src/domain/mod.rs` | Export the one coherent domain surface without a parallel prompt model. |
| `src-tauri/src/application/assets.rs` | Implement the smallest concrete CRUD operation between commands and SQLite. Avoid repository/service frameworks not needed by this ticket. |
| `src-tauri/src/application/mod.rs` | Expose the one current application operation. |
| `src-tauri/src/storage/assets.rs` | Provide the minimum real SQLite-backed CRUD store required for a functioning command path. LIB-002 must extend this store, not create a second one. |
| `src-tauri/src/storage/mod.rs` | Open/own the application database and expose the concrete store without adding FTS or a general migration framework yet. |
| `src-tauri/src/commands/assets.rs` | Add thin typed Tauri handlers that validate/delegate once and return typed results/errors; no SQL or filesystem logic belongs here. |
| `src-tauri/src/commands/mod.rs` | Collect command registration without duplicating domain logic. |
| `src-tauri/Cargo.toml` | Name only dependencies justified by the resolved SQLite/UUID/time/error requirements; never rely directly on transitive lockfile crates. |
| `src-tauri/Cargo.lock` | Mechanical dependency result; review for unrelated package churn. |
| `src/lib/assets.ts` | Add the focused typed IPC DTO/client used by the production caller, if the existing source layout supports this smallest placement. |
| `src/App.svelte` | Add only the minimal create/read/update/delete invocation proving the Rust path is user-reachable; UI-001 owns the full editor/autosave/search experience. |
| `src-tauri/tests/asset_domain.rs` and/or colocated unit tests | Prove valid construction, realistic field failures, stable error serialization, unknown valid asset-type preservation, tag uniqueness, SQLite CRUD, and command/application behavior. Use the smallest test placement that can exercise the real code. |

## Context files

| Path | What it tells the implementer |
|---|---|
| `AGENTS.md` | Use one Asset aggregate, thin commands, explicit present-purpose dependencies, no fallback/parallel implementation, and a named production caller. |
| `docs/frd/FRD-001-phase-zero-vertical-slice.md` | Exact Phase 0 fields and the later create/search/export/delete/import proof that these types must preserve. |
| `docs/prd/PRD-001-core-library.md` | The shared model is broader than prompts, although this ticket implements only the prompt slice. |
| `docs/adr/ADR-002-canonical-storage.md` | SQLite is canonical; one mutable draft and immutable snapshots are distinct; generated outputs are not a second store. |
| `docs/architecture.md` | Required direction is Svelte → typed IPC → thin command → application service → repository. |
| `Peng_Design_Pack/Peng_Axiomatic_Design_Spec.md` | Canonical fields, prompt type data, structured error shape, and autosave/version semantics. |
| `Peng_Design_Pack/Peng_Bundle_1.0.schema.json` | Concrete v1 limits and open/closed value sets; the domain must permit later lossless bundle round trips. |
| `src-tauri/src/lib.rs` | Current composition root is only the generated Tauri runner. |
| `src-tauri/Cargo.toml` | Current direct Rust dependencies are only serde, serde_json, and tauri. |
| `src-tauri/capabilities/default.json` | Current least-privilege capability; typed domain IPC does not justify filesystem/process expansion. |
| `src/App.svelte` | Current UI is a disabled scaffold and has no production command caller. |
| `HZN-001/context.md` | Limit work to Phase 0, use real Rust/SQLite production boundaries, and avoid compatibility or in-memory paths. |

## Ripple effects

- LIB-002 must adopt the minimum schema/repository into its ordered migration path, then add foreign keys, WAL, restart recovery, FTS5, and search without a duplicate store or retained parallel initializer.
- UI-001 must extend the same typed IPC client and minimal caller into the complete prompt editor/autosave/search interaction.
- BND-001 and BND-002 must map the same ID/status/source/type-data semantics to v1 documents without lossy converters.
- Domain validation and SQLite CRUD tests become prerequisites for persistence/search and bundle negative-path coverage.
- Any serialization rename affects every later consumer and should replace the unreleased contract directly, without aliases.
- Tauri capabilities should remain unchanged; dependency changes must remain limited to the explicit implementation need.

## Out of scope

- Ordered migration infrastructure, foreign-key/WAL hardening, restart-recovery proof, FTS5, and search commands owned by LIB-002.
- The complete prompt editor, debounced autosave, search interaction, keyboard path, and visual refinement owned by UI-001; only a minimal production CRUD caller is included here.
- Bundle ZIP creation, deterministic ordering, checksums, embedded schema, import staging/conflicts/rollback owned by BND-001/BND-002.
- Attachments, notes, collections, relationships, immutable snapshot UI, all five complete editors, backup/restore, target adapters, execution, Git, sync, or cloud behavior.
- Compatibility shims, dual models, in-memory fallbacks, placeholder services, generic repository frameworks, event buses, or speculative extension points.
