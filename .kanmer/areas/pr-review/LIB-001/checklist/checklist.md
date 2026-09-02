# Checklist — LIB-001

- [ ] [pre-review] Step 1 — Define and test the one canonical `Asset` aggregate, validation rules, CRUD DTOs, forward-readable unknown fields, and exact structured `PengError` contract in the declared domain files.
- [ ] [pre-review] Step 2 — Add only bundled `rusqlite` and prove the concrete service performs revision-safe CRUD against an isolated real on-disk SQLite database, including reopen, not-found, conflict, malformed-data, and no-mutation failures.
- [ ] [pre-review] Step 3 — Register exactly four thin typed Tauri CRUD commands over one managed application service and app-data `peng.sqlite3`, with unchanged capabilities and no SQL/filesystem logic in handlers.
- [ ] [pre-review] Step 4 — Wire the production `src/App.svelte` caller through the single typed IPC module so a user can create, reload/read, update, and delete one prompt with visible pending/result/error states and no UI-001 features.
- [ ] [pre-review] Step 5 — Run and record formatting, full Rust tests, clippy, Svelte check, Vite build, no-bundle Tauri release build, diff check, and scope audit; stop on any non-pass or undeclared change and do not merge or start another ticket.

## Progress notes

Append implementation evidence here; do not rewrite completed history.
