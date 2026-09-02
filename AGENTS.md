<!-- kanmer:instructions:start — managed by kanmer-setup; edits inside will be overwritten -->
# Kanmer operating instructions

This repo's work is tracked on a Kanmer board in `.kanmer/`. In a Git repo set up
through the GUI the board lives in its own worktree, `.worktrees/kanmer`, on the
board branch, and MCP is already rooted there — never create, switch or push that
branch yourself. Your own ticket worktree is a separate thing, recorded by
`take_ticket`.

The board branch convention is the repository variable `KANMER_BOARD_BRANCH`,
falling back to `kanmer-board` when it is unset. A branch rename is an
administrator handoff: retarget branch protection and required checks, update
the repository variable, and only then reconcile the board worktree and remove
old refs. Agents must not mutate protected refs, branch protection, or repository
variables; stop and report when the observed branch and configured convention
disagree.

- Start every session with `get_status`, then `list_board` / `list_items` to find your ticket.
- **Which documents a ticket needs depends on its profile, not on a fixed pipeline.** Call `get_doc_gates <id>` before every move. Not `board.yml` — requirements are injected at resolve time, so its `profiles:` block is not the effective set.
- Stages: backlog → preparing → implementing → review → verifying → done. **A move crosses at most one gated boundary**, so walk the stages one at a time; a jump is refused even when every document exists.
- **Gates constrain `move_item` and nothing else** — creation in any stage is ungated, and `gh pr merge` is outside the engine, so an unmet gate never stops a merge.
- An unticked `- [ ]` in `open-questions/` blocks a move: tick it, or move it below the literal `## Parked (explicitly deferred)` with a reason.
- Read the whole ticket folder before starting — documents are folders (`research/`, `plan/`, …), so there may be several files per type. If the ticket is in a group, read the group's `context.md` too: the constraint binding the batch is written once, there.
- Work each fresh ticket on its own branch and worktree: worktree `.worktrees/<id>`, branch `<id>-<slug>`; `take_ticket` records both and moves the stage. A resumed execution packet is available only in `implementing` and must validate/reuse the exact recorded branch and **worktree root** — never create a second worktree or take the ticket again. It must not name the board, shared source checkout, another active ticket's worktree, or any child of those; its checked-out branch and Git common directory must match the record and source repository. Pause by retaining that taken record; never release a paused ticket while its worktree/branch remains a resume target.
- Write pipeline documents with `set_ticket_doc`. Running notes go to `append_scratch` — scratch is the notepad and is never gated, and neither is anything under `reference/` or `assets/`.
- Proof is written on merged `main`, after review and the merge, not before.
- Archive, don't delete. Reference other items with [[ID]] wiki-links.
- Skills run in this order: kanmer-tickets → -research → -plan → -execute → -review → -verify → -closeout. How far a ticket walks it depends on its profile, so ask `get_doc_gates` rather than assuming every step. Off to the side: -auto (drives that order over many tickets), -docs (governing docs), -groom (fix the board), -report (read-only), -setup (reconcile after a Kanmer update).
- Each skill ends by naming what comes next — read that line before improvising a hand-off.

The local MCP convention is `KANMER_BOARD_BRANCH` in each project-scoped
provider registration or exported local runtime, falling back to the default
board branch when unset. GUI Connect writes the saved board-branch setting into local
registrations. Hosted Actions should mirror the same value in the repository
variable, but Actions variables are not inherited by local processes.
When a native runtime supervisor launches Kanmer through an operator-private
wrapper, that wrapper must export both `KANMER_PROVIDER_CWD` and
`KANMER_BOARD_BRANCH` before invoking the stable launcher. Native
The GUI's OpenAI tunnel controls manage the same long-lived native runtime
alias through `tunnel-client runtimes connect/status/stop/rm`. Application quit
does not stop that runtime; readiness requires structured non-stale status, and
local removal must confirm the alias is stopped before deleting its metadata.

## Agent conduct

**Scope**

1. **Scope is the brief.** “While I’m here” changes are follow-up tickets, not commits.
2. **Never absorb another ticket’s scope.** Link it and let it be worked on its own record.
3. **Release and remediation work ships no new features.**
4. **The ticket precedes the branch.** No board record, no PR.
5. **Stop at the stop condition.** Never merge your own PR or start the next ticket; report deviations instead of redesigning.

**Build**

6. **Greenfield has no legacy.** Unless the brief names users or data, add no fallback, compatibility, or deprecation path; delete what you replace.
7. **Reuse before build.** Name the helper, port, or route you extend; report a genuinely unfit one instead of silently building a parallel copy.
8. **One list per concept.** A second copy in another layer is duplication, even when it is “just strings”.
9. **Paths are relative.** Use repo-root-relative or injected configuration, never machine-specific paths.
10. **Dependencies are approvals.** Add no package unless the brief lists it.
11. **Concurrency results are never discarded.** Retry, defer, or surface them; a swallowed conflict is data loss.
12. **Errors surface.** No catch-all suppression or empty catch.
13. **No fabricated domain data.** Fixtures use the documented estate.

**Prove**

14. **Done means wired.** New code needs a named production caller; registered-but-unreachable or test-only code is not done.
15. **Runtime dependencies ship in the artifact.** Prove the deployed image carries every required browser, font, or package.
16. **A schema change and its permissions ride the same diff.** Include migration, grants, and bootstrap census together.
17. **Recorded commits must be reachable.** Ticket SHAs must exist on the merge target.
18. **Stubs are not done.** Do not present TODOs, placeholders, or mocks as implementation.
19. **Tests prove the claim.** Never weaken or delete an assertion to pass; a failing test stops and is reported.
20. **Verify with exit codes.** Run stated commands and record outputs; INCONCLUSIVE is not PASS, and a later pass does not erase a failure. Done requires PASS; an explicitly disposed terminal non-PASS stays Verifying, is archived, and is released.
21. **No speculative CI or tests.** Delete a gate that gates nothing.

**Conduct**

22. **Review findings get dispositions.** Fix, reject with reason, accept risk, or defer to a ticket; never silence them.
23. **Secrets never appear in code, tickets, or proofs.**
24. **A PR that changes commands or conventions updates AGENTS.md in the same PR.**
<!-- kanmer:instructions:end -->

# Contributor guide

## Product authority

- Treat Peng as actively developed and unreleased. Compatibility is required only when a current brief identifies a real consumer, released interface, or persistent data that must survive.
- For product and technical intent, read `Peng_Design_Pack/Peng_Axiomatic_Design_Spec.md` first. The current task and its acceptance criteria take precedence over the pack when they explicitly differ.
- Use `Peng_Design_Pack/Peng_GUI_Wireframe.html` and its PNG preview as interaction and visual references, not as production implementation.
- Treat `Peng_Bundle_1.0.schema.json` and `Peng_Format_v1_Example.peng` as the v1 portable-format contract and reference fixture. Change them only with an intentional format decision and update all affected documentation, fixtures, readers, writers, and tests together.
- Keep design claims honest: the pack is a proposed design and its performance figures are acceptance targets, not measured product capabilities.

## Product direction

Peng is a small, local-first desktop library for five canonical asset types:

- prompts;
- skills;
- plugins;
- MCP server definitions;
- subagents.

Follow the rule **store once, project many times**. Peng owns one canonical asset graph; target adapters render deployments for external harnesses. Generated target files are never a second canonical store.

Peng v1 is not a chat client, cloud collaboration product, marketplace, Git host, general secrets manager, background daemon, or automatic plugin/MCP execution environment. Do not add these without a new explicit requirement.

## Intended stack and boundaries

- Desktop: Tauri 2.
- Frontend: Svelte 5, TypeScript, and Vite as a client-only SPA. Do not add SvelteKit without a concrete requirement.
- Native core: Rust with typed Tauri commands and typed errors.
- Persistence: SQLite with foreign keys, WAL, ordered migrations, and FTS5; store larger or binary attachments in a managed content directory.
- Editor: lazily loaded CodeMirror 6 for supported text formats.
- Secrets: store references in ordinary records and bundles; resolve values locally through Stronghold or the operating-system credential store.

Keep these boundaries strict:

- The frontend must not issue arbitrary SQL, access unrestricted filesystem paths, or spawn arbitrary commands.
- Tauri commands stay thin: validate a typed request, call one application service, and return a typed result or structured error.
- Parse and validate bundles in Rust, outside the webview.
- Keep database concerns, archive handling, UI formatting, and target rendering in their respective layers.
- Target adapters may consume canonical assets and relationships; canonical assets must not require harness-specific paths.

## Delivery order

Build the smallest coherent phase requested. Do not pull later-phase capabilities forward.

1. Phase 0: canonical asset model, prompt editor, SQLite persistence/search, `.peng` round trip, checksum validation, and unsafe-path tests.
2. Phase 1: all five editors, organisation, notes/activity, relationships, dependency-aware export, versions/diffs, backup/restore, conflict handling, keyboard operation, and accessibility.
3. Phase 2: explicit plan/diff/apply target adapters with deployment receipts and drift detection.
4. Phase 3: operational features such as an MCP runner, registry refresh, Git integration, encrypted transfer, or sync only after demonstrated demand.

The Phase 0 end-to-end proof is: create a prompt, export it, delete it, import it, and recover the same identity, content, and metadata.

## Domain and persistence rules

- Use one shared `Asset` aggregate with type-specific editor/data adapters. Do not create separate organisation, search, history, or export systems per asset type.
- An asset has one autosaved mutable working draft and explicit immutable snapshots. Do not create a version for every keystroke.
- Model tags and collections as references; never duplicate an asset to place it in another collection.
- Model composition as typed relationships. Required dependency closure follows `requires` and `extends`; optional and informational relationships remain explicit choices.
- Preserve concurrent outcomes: retry, surface, or deliberately defer conflicts. Never silently discard them.
- Write attachment data to staging, validate and hash it, atomically move it, then commit its database reference so committed rows never point at incomplete files.
- FTS indexes are derived and rebuildable; canonical asset records remain authoritative.

## Portable bundle and import safety

- A `.peng` file is a deterministic ZIP-compatible archive, not a database backup.
- Use UTF-8 text, LF-normalised hashed bodies, stable forward-slash relative paths, deterministic entry ordering, and SHA-256 checksums as specified by the design pack.
- Reject absolute paths, drive prefixes, backslashes, parent traversal, links, device files, alternate data streams, invalid checksums, excessive sizes/counts, and suspicious compression ratios before extraction or mutation.
- Validate the manifest and documents against the bundled/current schema before import.
- Stage import decisions and apply accepted changes transactionally. Invalid, cancelled, or failed imports must leave the library unchanged.
- Never execute, install, launch, or dynamically load imported content during import or preview.
- Never export secret values. Export reference descriptors/placeholders only, and redact secret-like values from logs and previews.
- Store commands and arguments as structured arrays; do not interpolate an opaque shell string.
- Preserve unknown asset types and unknown JSON properties where the format contract requires forward readability, without inventing execution support for them.

## UX requirements

- Preserve the three-region desktop model: navigation rail, result list, and selected-asset workspace.
- Keep the frequent path shallow. Shared editing belongs in **Editor**; specialist configuration in **Setup**; graph data in **Links**; notes, activity, versions, and diffs in **Notes & history**.
- Keep import/export visible as core actions and preview dependency closure and conflicts before mutation.
- Use the restrained slate-and-teal visual language from the wireframe. Colour must never be the only status signal.
- Maintain keyboard-complete operation, visible focus, accessible names, at least 32 x 32 px desktop targets, and usable 200% scaling.
- Do not add instructional clutter to ordinary asset rows or cards; use concise labels and actionable empty states.

## Implementation discipline

- Prefer direct, readable code. Every new abstraction, dependency, service, configuration option, cache, worker, or extension point needs a present requirement.
- Use existing project conventions and dependencies. Adding a package requires explicit approval from the brief.
- Keep paths repository-relative or injected; never commit machine-specific paths.
- Do not preserve obsolete pre-release behavior through shims, aliases, dual paths, fallback implementations, or speculative migrations.
- Update all affected callers, schemas, migrations, fixtures, tests, and documentation in the same change, then remove superseded code.
- Surface structured, actionable errors. Do not use catch-all suppression or empty catches.
- Use documented Peng assets in fixtures. Do not fabricate domain estates or present stubs, TODOs, or mocks as completed functionality.

## Validation

- Add focused tests for the behavior changed and realistic failure cases. Never weaken an assertion to make a build pass.
- Bundle work must cover deterministic output, schema validation, checksum failures, unsafe paths, archive limits, conflict behavior, secret exclusion, dependency closure, and transactional rollback as applicable.
- Persistence work must cover migrations, foreign keys, transaction boundaries, restart recovery, and search-index rebuilds as applicable.
- UI work must exercise keyboard paths, focus behavior, non-colour status cues, and the relevant acceptance scenario.
- Run repository-provided format, lint, type-check, unit, integration, and build commands once those scripts exist. Report exact commands and exit codes; do not invent a successful check or call an inconclusive result a pass.
- Stop when the current acceptance criteria are satisfied and the affected product path is coherently wired end to end.
