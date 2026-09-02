# Plan — LIB-001: Define canonical asset domain and typed command boundary

## Objective

Deliver one production-reachable prompt CRUD slice through a single canonical `Asset` aggregate, a concrete SQLite-backed Rust application service, thin typed Tauri commands, and the smallest frontend caller that proves create/read/update/delete are wired.

## Starting state

At repository commit `3ee6e3549ffbe3deeb2e4ea1b2f5e34329ff82a1`, `src-tauri/src/lib.rs` only starts a generated Tauri builder, the Rust crate has no domain/application/storage/command modules, and `src/App.svelte` is a disabled static scaffold with no IPC caller. Direct Rust dependencies are `serde`, `serde_json`, and `tauri`; capabilities grant only `core:default`. LIB-001 has no blockers and blocks LIB-002 and UI-001. The Phase 0 group requires real Rust/SQLite production boundaries and forbids compatibility or in-memory production paths.

Evidence: `research/research.md`@`6762988d9f1201a9`, `files/files.md`@`f0b64d548a0b5a06`, `open-questions/open-questions.md`@`72a29abac0cebd92`; group `HZN-001/context.md` read 2026-09-02. No project research sources were declared.

## Governing docs

- **Meets `docs/prd/PRD-001-core-library.md`:** establishes the one shared `Asset` aggregate and canonical local SQLite CRUD path that later asset types, organisation, history, and search extend. It does not create a prompt-only parallel model or claim the later PRD capabilities.
- **Meets `docs/frd/FRD-001-phase-zero-vertical-slice.md`:** delivers the stable UUID, name, summary, status, tags, body, prompt-specific data, and user-reachable create/edit/read/delete foundation for acceptance criterion 1. Restart recovery/search, deterministic export, safe import, and the complete recovery proof remain with LIB-002, BND-001, BND-002, UI-001, and PLAT-001; this ticket preserves the identity/content/metadata contract those tickets consume.
- **Meets `docs/adr/ADR-002-canonical-storage.md`:** SQLite is the sole production store, the record represents one mutable working draft, and no generated projection or alternate canonical store is introduced. LIB-002 owns the final ordered migration path, WAL/foreign-key enforcement, restart proof, FTS5, and derived-index rebuild behavior; it must replace the bounded baseline bootstrap rather than retain parallel initialization.

No governing document is modified and no new ADR is required.

## Required changes

- Add one serializable Rust `Asset` model with stable UUID text, open validated asset-type token, closed v1 status/source-kind values, schema version, name, summary, unique tags, body, object-valued type data, current revision, timestamps, and flattened unknown JSON fields needed for forward-readable later bundle mapping.
- Add typed create/update/delete requests and the returned asset response. Create is restricted to the prompt slice in the production caller; update and delete require the caller's expected revision so concurrent outcomes are surfaced as `CONFLICT` rather than overwritten or discarded.
- Validate UUID syntax, asset-type token, nonblank name/body, schema-version token, summary/name/tag limits, unique tags, and object-valued type data in Rust. Return one structured `PengError` shape: `code`, `message`, optional `field`, optional JSON `details`, and `retryable`; include stable validation, not-found, conflict, storage, and initialization codes.
- Add only the direct `rusqlite` dependency with its bundled SQLite feature. This present requirement is necessary for a self-contained real SQLite store; UUID generation uses SQLite randomness and Rust-side RFC 4122 validation so no UUID/time/error framework dependency is added.
- Add a concrete store owning one SQLite connection behind synchronized application state. Its bounded baseline bootstrap creates one `assets` table for all asset types, stores tags/type data/unknown fields as JSON, uses SQLite UTC timestamps, and performs atomic create/read/update/delete operations. Update/delete use revision predicates and distinguish not-found from stale-revision conflict. It contains no FTS table, search command, WAL/foreign-key hardening, general migration framework, repository abstraction, fallback store, or second initializer.
- Add one concrete asset application service that validates, generates a version-4 UUID through the SQLite connection, delegates to the store, and maps storage failures to the typed error contract. Commands contain no SQL or filesystem behavior.
- Register `create_asset`, `get_asset`, `update_asset`, and `delete_asset` as thin Tauri commands. In the Tauri setup hook, resolve the application data directory, create it, open `peng.sqlite3`, initialize the concrete service once, and manage it as application state. Do not broaden webview capabilities.
- Add a typed TypeScript IPC module matching the Rust JSON contract exactly. Replace only the scaffold workspace content necessary to expose name/body inputs and explicit create, reload/read, update, and delete controls for the active prompt, with visible pending/success/error states. This is reachability proof, not the complete editor, autosave, search, import/export, navigation behavior, or visual redesign owned by UI-001.
- Add focused Rust tests for validation, error serialization, unknown valid asset-type/JSON preservation, tag uniqueness, real on-disk SQLite CRUD, persistence within reopen where supported by the bounded store, revision conflicts, missing IDs, and no mutation after rejected input. Type-check/build the production TypeScript caller and package the desktop binary to prove registration and bundled SQLite linkage.

## Expected files

| Action | Repo-root-relative path | Responsibility |
|---|---|---|
| Modify | `src-tauri/Cargo.toml` | Declare only bundled `rusqlite` for the required real SQLite boundary. |
| Modify | `src-tauri/Cargo.lock` | Mechanical dependency lock update; no unrelated churn. |
| Modify | `src-tauri/src/lib.rs` | Construct SQLite-backed state and register the four production commands. |
| Add | `src-tauri/src/domain/mod.rs` | Export the coherent domain surface. |
| Add | `src-tauri/src/domain/asset.rs` | Canonical aggregate, CRUD DTOs, validation, and focused unit tests. |
| Add | `src-tauri/src/domain/error.rs` | Structured error contract and serialization tests. |
| Add | `src-tauri/src/application/mod.rs` | Export the concrete application service. |
| Add | `src-tauri/src/application/assets.rs` | Validate and coordinate CRUD without a service framework. |
| Add | `src-tauri/src/storage/mod.rs` | Open the application database and expose the single concrete store. |
| Add | `src-tauri/src/storage/assets.rs` | Baseline schema, UUID generation, transactional/revision-safe SQLite CRUD, and on-disk tests. |
| Add | `src-tauri/src/commands/mod.rs` | Export command handlers. |
| Add | `src-tauri/src/commands/assets.rs` | Four thin typed Tauri handlers. |
| Add | `src/lib/assets.ts` | Typed production IPC DTOs and four invoke functions. |
| Modify | `src/App.svelte` | Smallest user-reachable prompt CRUD caller and visible results. |
| Modify | `src/styles.css` | Minimal existing-language styles needed to keep the caller usable and focus-visible. |

## Do not modify

- `AGENTS.md`
- `README.md`
- `docs/**`
- `Peng_Design_Pack/**`
- `src-tauri/capabilities/**`
- `src-tauri/tauri.conf.json`
- `package.json`
- `package-lock.json`
- `.github/**`
- `.kanmer/**`
- Bundle, search/FTS, ordered-migration, target-adapter, attachment, secret-resolution, and Phase 1 feature paths not listed in Expected files.

## Constraints

- This is an unreleased target state: replace the disabled scaffold content needed for the caller and add no compatibility aliases, dual models, fallback store, or deprecated path.
- One `Asset` table/model serves all type tokens; the current caller creates `prompt` assets only. Unknown syntactically valid type tokens and unknown JSON properties remain data, never executable support.
- The frontend never accesses SQL, unrestricted paths, or processes. Commands validate typed requests, call one concrete application service, and return typed values/errors.
- SQLite bytes must ship with the desktop artifact through the bundled `rusqlite` feature. No external SQLite executable or machine installation is allowed.
- The baseline table initialization is intentionally bounded to functioning LIB-001 CRUD. LIB-002 replaces it with the one ordered migration mechanism and adds WAL, foreign keys, restart recovery, FTS, and search; LIB-001 must not pre-implement those features or leave two initialization paths.
- No attachment, bundle, filesystem-import, secret, execution, event, background worker, cache, generic repository, trait/factory, or package beyond bundled `rusqlite` is introduced.
- Invalid input and stale revisions fail before mutation. Poisoned locks, directory/open/bootstrap failures, and SQL errors surface as structured actionable errors; no catch-all suppression.
- Tests use isolated real on-disk SQLite files and remove only their exact temporary files. An in-memory database may not stand in for the production storage claim.

## Ordered steps

### Step 1 — Define the canonical aggregate and error contract
- Preconditions: repository remains at or rebased cleanly from the pinned starting state; linked docs and evidence versions have not materially changed.
- Files: `src-tauri/src/domain/mod.rs`, `src-tauri/src/domain/asset.rs`, `src-tauri/src/domain/error.rs`.
- Change: implement the one `Asset` aggregate, prompt CRUD DTOs, open validated type token, closed status/source values, revision metadata, unknown-property preservation, boundary validation, and serializable `PengError` codes.
- Preserved behaviour: no runtime wiring changes yet; the existing application still builds after the new module is compiled through later composition.
- Forbidden: separate Prompt aggregate, five per-type models, compatibility aliases, bundle execution semantics, persistence code, or invented domain fixtures.
- Negative cases: malformed UUID/type/schema version, blank or oversized fields, duplicate/invalid tags, non-object type data, and invalid revisions return field-specific validation errors without panics.
- Tests: colocated unit tests in `src-tauri/src/domain/asset.rs` and `src-tauri/src/domain/error.rs`.
- Commands: `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`; `cargo test --manifest-path src-tauri/Cargo.toml domain`.
- Expected output: formatting exits 0 and domain/error tests pass with stable serialized fields.
- Done when: valid canonical/unknown-type data round-trips, every named negative case is asserted, and the error JSON contract is exact.
- Deviation stop: stop if a linked contract requires a second aggregate, a closed type enum, new package, or compatibility behavior.

### Step 2 — Implement the concrete SQLite CRUD service
- Preconditions: Step 1 contracts compile and tests pass.
- Files: `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, `src-tauri/src/application/mod.rs`, `src-tauri/src/application/assets.rs`, `src-tauri/src/storage/mod.rs`, `src-tauri/src/storage/assets.rs`.
- Change: add bundled `rusqlite`, create the single baseline `assets` table/store, generate RFC 4122 version-4 IDs using SQLite randomness, persist JSON fields, and implement one concrete validating service with atomic create/read/update/delete and optimistic revision checks.
- Preserved behaviour: one canonical record supplies later persistence, bundle, and UI tickets; rejected operations leave the database unchanged.
- Forbidden: in-memory production fallback, generic repository traits, connection pool, migration framework, FTS/search, WAL/foreign-key hardening, event system, swallowed lock/SQL errors, or any additional dependency.
- Negative cases: duplicate ID, missing ID, stale update/delete revision, malformed stored JSON, failed open/bootstrap, and invalid request surface the correct structured error and do not silently overwrite data.
- Tests: colocated on-disk SQLite tests in `src-tauri/src/storage/assets.rs` plus service tests in `src-tauri/src/application/assets.rs`.
- Commands: `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`; `cargo test --manifest-path src-tauri/Cargo.toml storage`; `cargo test --manifest-path src-tauri/Cargo.toml application`.
- Expected output: all commands exit 0; a temporary file-backed database proves CRUD, reopen/read, revision increment, conflict, and deletion behavior.
- Done when: the service returns the same identity/content/metadata written to real SQLite, concurrency conflicts are surfaced, and no forbidden subsystem exists.
- Deviation stop: stop if bundled SQLite cannot build, the store needs a second initialization path, or reliable mutation would require work owned by LIB-002.

### Step 3 — Wire thin typed Tauri commands
- Preconditions: Step 2 service and on-disk storage tests pass.
- Files: `src-tauri/src/commands/mod.rs`, `src-tauri/src/commands/assets.rs`, `src-tauri/src/lib.rs`.
- Change: add four commands that delegate once to managed `AssetService`; resolve/create the app-data directory, open `peng.sqlite3`, manage the initialized service, and register the exact command list in the production builder.
- Preserved behaviour: existing Tauri startup and least-privilege capability remain intact.
- Forbidden: SQL/filesystem behavior inside command handlers, arbitrary path parameters, process spawning, command-side fallback state, capability expansion, or registered-but-unreachable extra commands.
- Negative cases: initialization failures terminate startup with actionable context; command validation/not-found/conflict/storage failures serialize as `PengError` rather than strings or panics.
- Tests: Rust command-contract compilation plus existing domain/service tests; direct handlers remain thin enough to verify by review without a Tauri test harness.
- Commands: `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`; `cargo test --manifest-path src-tauri/Cargo.toml`; `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`.
- Expected output: all commands exit 0 and `lib.rs` visibly registers exactly `create_asset`, `get_asset`, `update_asset`, and `delete_asset` backed by managed SQLite state.
- Done when: the production composition root has one reachable typed path from command to service to SQLite and capabilities are unchanged.
- Deviation stop: stop if Tauri requires broadened webview permissions, global mutable state outside managed state, or an additional runtime/plugin dependency.

### Step 4 — Add the minimal production frontend caller
- Preconditions: Step 3 command names and serialized DTO/error shapes are stable.
- Files: `src/lib/assets.ts`, `src/App.svelte`, `src/styles.css`.
- Change: add exact TypeScript DTOs/invoke functions and a compact prompt name/body form that can create an asset, explicitly reload it through `get_asset`, update with the returned revision, and delete with the returned revision; show pending, success, and structured error states with accessible labels and focus-visible controls.
- Preserved behaviour: the three-region scaffold and slate/teal language remain recognizable; UI-001 can extend the same IPC module rather than replace a parallel caller.
- Forbidden: full editor redesign, autosave/debounce, search, import/export, fabricated saved prompts, mock data, direct SQL/filesystem access, hidden errors, or a second IPC contract.
- Negative cases: blank input is reported from the Rust contract, controls prevent duplicate pending submissions, stale revision is visible as conflict, and delete clears only after the backend succeeds.
- Tests: `svelte-check` type/accessibility analysis and Vite production build compile the real caller.
- Commands: `npm run check`; `npm run build`.
- Expected output: both commands exit 0 with no Svelte errors or warnings and the generated frontend imports `@tauri-apps/api/core` only through `src/lib/assets.ts`.
- Done when: a user can drive all four registered commands from the production window and see returned identity/revision or structured failure.
- Deviation stop: stop if proving reachability requires implementing UI-001 autosave/search scope, changing navigation architecture, or adding an npm package.

### Step 5 — Prove the bounded production path
- Preconditions: Steps 1–4 are complete and their focused checks pass.
- Files: `src-tauri/Cargo.lock`, `src-tauri/src/lib.rs`, `src-tauri/src/domain/asset.rs`, `src-tauri/src/domain/error.rs`, `src-tauri/src/application/assets.rs`, `src-tauri/src/storage/assets.rs`, `src-tauri/src/commands/assets.rs`, `src/lib/assets.ts`, `src/App.svelte`, `src/styles.css`.
- Change: review the final diff for one aggregate/store/IPC contract, run the complete repository rails, and capture exact exit evidence for the post-implementation report; make only fixes within already declared files and scope.
- Preserved behaviour: the packaged desktop app remains offline-capable, capabilities remain least-privilege, and later ticket boundaries remain absent.
- Forbidden: weakening assertions, ignoring warnings/failures, accepting inconclusive output, committing generated build directories, merging the PR, or beginning LIB-002/UI-001.
- Negative cases: verify source/tests cover invalid input, not-found, conflict, storage failure, and no-mutation outcomes; search the diff for fallback/placeholder/TODO/FTS/search additions.
- Tests: full Rust unit suite, clippy, Svelte check, Vite build, and no-bundle Tauri release build.
- Commands: `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`; `cargo test --manifest-path src-tauri/Cargo.toml`; `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`; `npm run check`; `npm run build`; `npm run tauri build -- --no-bundle`; `git diff --check`.
- Expected output: every command exits 0, the release executable is produced with bundled SQLite linkage, and `git diff --check` reports no defects.
- Done when: exact command evidence plus source/tests prove the production UI → typed IPC → command → service → real SQLite CRUD path and all scoped negative cases.
- Deviation stop: any failure, weakened assertion, undeclared changed path, unrelated lock churn, missing production caller, unbundled SQLite, or required out-of-scope change stops execution for disposition.

## Acceptance checks

- `src/App.svelte` is the named production caller, using only `src/lib/assets.ts`; `src-tauri/src/lib.rs` is the production composition/registration entry.
- All four CRUD commands call one concrete application service, and all persistent mutations use the one SQLite store/table with expected-revision conflict handling.
- The packaged no-bundle release build proves the bundled SQLite runtime compiles into the artifact; no external executable or installation is required.
- Focused tests prove field validation, stable structured errors, unknown valid type/property preservation, file-backed CRUD/reopen behavior, not-found, stale-revision conflict, and no mutation on rejected requests without weakened assertions.
- The final diff contains no FTS/search command, ordered migration framework, full autosave/editor work, bundle behavior, broad capability, compatibility layer, fallback store, second aggregate, or additional dependency.
- Exact commands and exit codes are retained for the post-implementation report; an inconclusive or later-only pass does not erase a failure.

## Commands

Run from the exact ticket worktree recorded by `take_ticket`:

1. `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`
2. `cargo test --manifest-path src-tauri/Cargo.toml`
3. `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`
4. `npm run check`
5. `npm run build`
6. `npm run tauri build -- --no-bundle`
7. `git diff --check`
8. `git status --short`

## Failure and deviation rules

Stop and report a failing or inconclusive check, stale evidence that changes scope, an unknown Tauri/SQLite API, an undeclared file requirement, any package beyond bundled `rusqlite`, lockfile churn unrelated to that dependency, a governing-doc conflict, an unsafe path/command, or a need to enter LIB-002/UI-001 scope. Do not hide an error, weaken a test, improvise compatibility, retain parallel initialization, or substitute in-memory/mock behavior for the production claim. A necessary change outside Expected files requires replanning before modification.

## Stop condition

Stop after the scoped implementation is committed and pushed on LIB-001's recorded branch, its post-implementation report contains exact passing evidence, and the ticket is ready for independent `kanmer-review`. Do not merge the PR, move beyond the execute-owned boundary, begin another ticket, or implement LIB-002/UI-001 work.
