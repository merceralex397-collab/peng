---
kind: review-attestation
pr: "1"
head_sha: "4e28c71d0b47fb44c197eb328ae886101b365d9e"
verdict: needs-changes
reviewer: "lib001-review-1"
independent: true
plan_hash: "f6d5140f568996dc"
ticket_updated: "2026-09-02T22:37:44.678Z"
board_sha: "744b5276bf2f1c90dc2a1547f8e9ecd68de7c117"
expected_reviewers:
  - "lib001-review-1"
threads_snapshot: []
findings:
  - id: F-001
    severity: major
    summary: "Schema-version validation accepts values outside the authoritative major.minor contract."
    disposition: open
---
# Independent review — LIB-001

## Scope and changes reviewed

Reviewed PR #1 at exact head `4e28c71d0b47fb44c197eb328ae886101b365d9e` against the complete LIB-001 packet, revised plan `f6d5140f568996dc`, HZN-001 group context, and the linked PRD, FRD, ADR, design pack, and portable-format schema. The diff stays within the declared files and implements one canonical Asset model, one SQLite-backed service/store, four thin Tauri commands, and the minimal production frontend caller. No FTS/search, migration framework, bundle implementation, expanded capability, compatibility path, fallback store, or additional dependency beyond bundled rusqlite was introduced.

The reviewer is an independent agent role and is not the implementation author. The expected reviewer set contains only `lib001-review-1`; that reviewer posted finding F-001 publicly on this exact head at https://github.com/merceralex397-collab/peng/pull/1#issuecomment-5517442078. GitHub exposed no review threads, reviews, requested changes, or unresolved conversations at gather time.

## Acceptance checks

Independent commands run from `.worktrees/lib-001`:

- Scoped Rust 2024 `rustfmt --check` command from the revised plan — exit 0.
- `cargo test --manifest-path src-tauri/Cargo.toml` — exit 0; 7 passed, 0 failed.
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings` — exit 0.
- `npm run check` — exit 0; 0 errors and 0 warnings.
- `npm run build` — exit 0.
- `npm run tauri build -- --no-bundle` — exit 0; produced `src-tauri/target/release/peng.exe`.
- `git diff --check 3ee6e3549ffbe3deeb2e4ea1b2f5e34329ff82a1..HEAD` — exit 0.

GitHub reported PR #1 OPEN, non-draft, merge state CLEAN, base `main`, head branch `LIB-001-canonical-asset-domain`, exact head SHA above, and no status checks configured. The ticket does not declare a required GitHub check; the repository validation commands above provide the applicable evidence.

## Findings and dispositions

### F-001 — Major — Open

`src-tauri/src/domain/asset.rs` validates `schemaVersion` as one or more dot-separated decimal components. It therefore accepts `1` and `1.2.3`, contrary to the authoritative Peng v1 schema pattern `^\d+\.\d+$` and the plan's requirement that the canonical boundary enforce the schema-version token. Current tests cover `1..0` but do not cover too few or too many components.

Disposition: open. Restrict the existing validator to exactly two non-empty decimal components and add focused negative assertions for both `1` and `1.2.3`. This is a bounded in-scope remediation in the already-authorized domain file and test module.

## Verdict and residual risk

Verdict: `needs-changes`. F-001 is an open major contract finding and blocks merge. All other reviewed scope and validation rails passed; no separate residual risk is accepted at this round. The same branch, worktree, PR, and claim must be retained for the single remediation batch.
