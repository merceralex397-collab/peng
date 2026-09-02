# Peng — Axiomatic Product and Technical Design Specification

**Status:** Proposed v1 design  
**Date:** 2 September 2026  
**Product:** Peng  
**Portable package extension:** `.peng`

---

## 1. Executive decision

Peng should be a **small, local-first desktop library for reusable AI building blocks**. It keeps one canonical copy of each prompt, skill, plugin definition, MCP server configuration, and subagent; lets the user search, group, link, annotate, and version those assets; and exports any chosen set as a portable `.peng` bundle.

The recommended implementation is:

- **Desktop shell:** Tauri 2
- **User interface:** Svelte 5 + TypeScript + Vite, as a client-only single-page application
- **Application core:** Rust commands and services
- **Data:** SQLite with FTS5 full-text search; text and metadata in the database, larger attachments in a managed content directory
- **Editor:** lazily loaded CodeMirror 6 for Markdown, plain text, JSON, YAML, and code-like content
- **Portable format:** a deterministic ZIP-compatible container renamed `.peng`, with a versioned JSON manifest, individual asset folders, relationships, checksums, and optional notes/history
- **Secrets:** references only in exported bundles; actual values held outside the ordinary library, using Tauri Stronghold or the operating-system credential store

This gives Peng the broad UI and desktop integration of a web-technology application without bundling a second browser engine. Tauri uses the operating system's webview and places native operations behind a Rust boundary. Its capability system can restrict which native operations the frontend may invoke. [1][2]

Wails 2 is the strongest runner-up for a developer who would rather write Go than Rust, but its next major generation is still in beta; Neutralinojs is smaller at the shell level but would leave Peng to implement more database, update, and security integration itself. [16][17]

### Product rule

> **Store once, project many times.**

A skill or prompt has one canonical record in Peng. Codex, Claude Code, `.agents`, OpenCode, or another harness receives a generated deployment produced by an adapter. Generated copies are not treated as new canonical assets. This avoids maintaining duplicated skill trees merely because different harnesses expect different directories or manifests.

---

## 2. Product definition

### 2.1 What Peng is

Peng is a personal, offline-first catalogue and packaging tool for:

1. **Prompts** — reusable instructions or templates, optionally containing variables.
2. **Skills** — structured instruction packs that teach an agent how to perform a repeatable workflow.
3. **Plugins** — deployable integration packages or descriptors for a particular AI harness. Peng stores and packages them; it does not automatically trust or execute imported plugin code.
4. **MCP servers** — Model Context Protocol server definitions, transports, launch details, environment-variable references, capabilities, and documentation. The current MCP specification defines `stdio` and Streamable HTTP as standard transports. [8]
5. **Subagents** — reusable agent profiles that combine instructions, tools, skills, prompts, MCP servers, model preferences, and execution constraints.

Every item is an **Asset**. Asset types share the same basic organisation and history features, while each type owns its specialist fields.

### 2.2 What Peng is not in v1

Peng v1 is not:

- an AI chat client;
- a cloud collaboration platform;
- a marketplace;
- a Git hosting service;
- a general secrets manager;
- a mandatory background daemon;
- an autonomous plugin or MCP execution sandbox;
- an installer that silently writes into every supported harness.

These exclusions are deliberate. They keep the first release small, understandable, and safe. The architecture leaves clean extension points for target adapters and an opt-in MCP runner later.

---

## 3. Axiomatic design, in plain terms

Axiomatic Design turns a vague wish into a set of independent functions and then chooses a concrete mechanism for each function.

- A **Customer Need (CN)** says what the person needs.
- A **Functional Requirement (FR)** says what the product must do, without prescribing technology.
- A **Design Parameter (DP)** is the part of the solution that fulfils an FR.
- A **Process Variable (PV)** describes how that DP will be built or controlled.

The method applies two rules:

1. **Independence Axiom:** changing one function should not unexpectedly break another. For Peng, changing the MCP editor must not require rewriting prompt search, and adding a new target adapter must not change the canonical data model.
2. **Information Axiom:** among designs that preserve independence, choose the one with the lowest complexity and highest probability of working reliably. [6][7]

A perfectly diagonal FR-to-DP matrix is **uncoupled**. A lower-triangular matrix is **decoupled**: later functions may read earlier outputs through stable interfaces, but there are no circular dependencies. A matrix with dependencies in both directions is coupled and should be redesigned.

---

## 4. Customer needs and design constraints

### 4.1 Customer needs

| ID | Customer need |
|---|---|
| CN1 | Capture a useful prompt or AI component in seconds. |
| CN2 | Find it later without remembering its exact name or storage location. |
| CN3 | Record why it exists, what changed, and what was learned while using it. |
| CN4 | Combine related prompts, skills, servers, plugins, and agents without copying them. |
| CN5 | Move an exact chosen set to another computer in one file. |
| CN6 | Understand an imported bundle before accepting it. |
| CN7 | Keep the program fast, private, offline, and easy to back up. |
| CN8 | Support different AI harness layouts without maintaining duplicate canonical content. |
| CN9 | Retain data across application and schema upgrades. |

### 4.2 Constraints

| ID | Constraint |
|---|---|
| C1 | Windows-first desktop experience, with practical Linux and macOS support. |
| C2 | No account, server, or internet connection required for core use. |
| C3 | Fast cold start, low idle memory, and no bundled Chromium runtime. |
| C4 | Keyboard-complete operation as well as straightforward mouse use. |
| C5 | Imported content is untrusted and is never executed during import or preview. |
| C6 | Secret values are excluded from `.peng` exports by default. |
| C7 | Export bundles are versioned, inspectable, checksummed, and recoverable with ordinary ZIP tooling. |
| C8 | Existing assets remain usable when new asset types or target adapters are introduced. |
| C9 | The normal editing experience must not expose database, file-layout, or schema complexity. |

---

## 5. Functional decomposition

### 5.1 Top-level mapping

| FR | Functional requirement | DP | Design parameter | PV / implementation control | Verification |
|---|---|---|---|---|---|
| FR1 | Create and edit heterogeneous AI assets through one consistent experience. | DP1 | Canonical `Asset` aggregate plus independent type editor adapters. | Rust domain structs; versioned type schemas; Svelte editor components. | A new asset type can be added without altering existing type editors. |
| FR2 | Persist and retrieve assets quickly. | DP2 | Local repository and search subsystem. | SQLite, WAL, foreign keys, FTS5 indexes, migrations. | Search P95 target under 100 ms for 50,000 ordinary text assets. |
| FR3 | Organise assets independently of their type or storage path. | DP3 | Tags, collections, favourites, saved filters, and archive state. | Normalised relation tables and reusable filter queries. | One asset may appear in multiple collections without duplication. |
| FR4 | Preserve notes, activity, and meaningful versions. | DP4 | Separate note stream, append-only activity journal, working draft, and immutable snapshots. | Transactional writes; content hashes; explicit “Create version”. | Editing a draft does not destroy the previous snapshot. |
| FR5 | Describe composition and dependencies. | DP5 | Typed relationship graph between assets. | `asset_relationship` table; cycle and missing-reference checks. | A subagent may link to several skills and MCP servers without embedding copies. |
| FR6 | Export an exact chosen set and its required dependencies. | DP6 | Export basket, dependency resolver, and deterministic `.peng` bundle writer. | Stable bundle order; JSON manifest; SHA-256 checksums; atomic temp-file rename. | Two exports of unchanged input produce equivalent logical contents. |
| FR7 | Import safely and resolve identity conflicts. | DP7 | Staged bundle inspector, validator, conflict resolver, and transactional importer. | Archive limits; path validation; schema validation; ID/hash comparison; rollback. | A malformed bundle changes no library data. |
| FR8 | Deploy canonical assets to different harness layouts without duplicating their source. | DP8 | Optional target adapter interface and deployment records. | Adapter manifest, render/plan/apply contract, output hashes, drift detection. | Adding a Codex adapter does not alter Claude or `.agents` adapters. |
| FR9 | Keep secret and executable operations outside ordinary content handling. | DP9 | Secret-reference model plus an opt-in, capability-scoped runtime boundary. | Stronghold/keyring; redaction policy; allow-listed commands and paths. | Import and preview never execute files or commands. |
| FR10 | Recover from failures and evolve the schema. | DP10 | Automatic local backup, integrity checks, and ordered migrations. | SQLite integrity check; rotation policy; format migration tests. | Restore can rebuild search indexes and preserve asset IDs. |

### 5.2 Detailed decomposition

#### FR1 — Asset authoring

- **FR1.1:** Capture common identity and descriptive metadata.  
  **DP1.1:** Shared header containing name, summary, status, tags, collection membership, source, and compatibility.
- **FR1.2:** Edit type-specific content without showing irrelevant fields.  
  **DP1.2:** One editor adapter per asset type, selected by the asset's discriminator.
- **FR1.3:** Store supporting files.  
  **DP1.3:** Managed attachment store using relative paths and content hashes.
- **FR1.4:** Make rapid capture possible.  
  **DP1.4:** `New` palette with five types, minimal required fields, sensible templates, and keyboard shortcuts.

#### FR2 — Retrieval

- **FR2.1:** Search names, summaries, body text, notes, tags, and selected structured fields.  
  **DP2.1:** FTS5 virtual index with ranked results and highlighted matches. SQLite's FTS5 module provides full-text search inside the same database. [3]
- **FR2.2:** Narrow results predictably.  
  **DP2.2:** Type, status, tag, collection, compatibility, modified-date, and dependency filters.
- **FR2.3:** Reopen working context.  
  **DP2.3:** Recent items and restorable last selection/filter state.

#### FR3 — Organisation

- **FR3.1:** Apply lightweight labels.  
  **DP3.1:** Many-to-many tags.
- **FR3.2:** Build purposeful sets without changing ownership.  
  **DP3.2:** Collections containing references to assets.
- **FR3.3:** Keep unused content without losing it.  
  **DP3.3:** Archive state excluded from default views.
- **FR3.4:** Reuse a query.  
  **DP3.4:** Saved views storing filter state, not copied result lists.

#### FR4 — Notes, history, and versions

- **FR4.1:** Add free-form observations.  
  **DP4.1:** Timestamped notes with pinning and optional links to a version.
- **FR4.2:** Record important actions automatically.  
  **DP4.2:** Append-only events such as created, imported, exported, deployed, versioned, archived, or dependency changed.
- **FR4.3:** Separate ordinary editing from durable checkpoints.  
  **DP4.3:** Autosaved working draft plus explicit immutable snapshots.
- **FR4.4:** Compare revisions.  
  **DP4.4:** Text diff generated on demand; history content is not continuously rendered.

#### FR5 — Composition

- **FR5.1:** Represent “uses”, “requires”, “extends”, and compatibility links.  
  **DP5.1:** Typed directed relationships.
- **FR5.2:** Prevent accidental broken bundles.  
  **DP5.2:** Dependency validation and export closure preview.
- **FR5.3:** Avoid accidental recursive dependency graphs.  
  **DP5.3:** Cycle warnings for relationship types that must be acyclic, while allowing benign links such as `related_to`.

#### FR6 — Export

- **FR6.1:** Select content over time without losing browsing context.  
  **DP6.1:** Persistent in-session export basket.
- **FR6.2:** Include required dependencies deliberately.  
  **DP6.2:** Resolver that distinguishes required, optional, and external dependencies.
- **FR6.3:** Control private/history data.  
  **DP6.3:** Explicit switches for attachments, notes, activity events, and old versions.
- **FR6.4:** Produce a durable portable file.  
  **DP6.4:** `.peng` archive specification described in section 11.

#### FR7 — Import

- **FR7.1:** Preview contents before mutation.  
  **DP7.1:** Read-only bundle summary grouped by type, source, version, and risk.
- **FR7.2:** Detect conflicts.  
  **DP7.2:** Compare stable asset UUID, version, and content hash.
- **FR7.3:** Resolve conflicts per asset or in bulk.  
  **DP7.3:** Add, update as new version, duplicate with new ID, or skip.
- **FR7.4:** Apply all chosen changes atomically.  
  **DP7.4:** One database transaction plus staged attachment move.

#### FR8 — Target adapters

- **FR8.1:** Describe an external target.  
  **DP8.1:** Adapter metadata: target name, supported asset types, platforms, locations, and required fields.
- **FR8.2:** Preview generated changes.  
  **DP8.2:** `plan()` returns files to create/change/delete and warnings; no writes occur.
- **FR8.3:** Apply a deployment explicitly.  
  **DP8.3:** `apply()` writes only after confirmation and records destination hashes.
- **FR8.4:** Detect local drift.  
  **DP8.4:** Compare deployed hashes with current destination content; never silently overwrite drift.

---

## 6. Independence Axiom analysis

### 6.1 Design matrix

`X` is the primary FR-to-DP mapping. `x` is a one-way read through a stable contract.

|  | DP1 Asset model/editors | DP2 Repository/search | DP3 Organisation | DP4 Journal/version | DP5 Relationships | DP6 Export | DP7 Import | DP8 Adapters | DP9 Policy boundary | DP10 Recovery |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| FR1 Author | X |  |  |  |  |  |  |  |  |  |
| FR2 Persist/find | x | X |  |  |  |  |  |  |  |  |
| FR3 Organise | x | x | X |  |  |  |  |  |  |  |
| FR4 Note/version | x | x |  | X |  |  |  |  |  |  |
| FR5 Compose | x | x |  |  | X |  |  |  |  |  |
| FR6 Export | x | x | x | x | x | X |  |  |  |  |
| FR7 Import | x | x | x | x | x | x | X |  | x |  |
| FR8 Deploy | x |  |  |  | x |  |  | X | x |  |
| FR9 Protect |  |  |  |  |  |  |  |  | X |  |
| FR10 Recover/evolve |  | x |  |  |  | x |  |  |  | X |

The matrix is deliberately lower-triangular rather than circular:

- Search knows the canonical asset contract; the asset editor does not know how search is implemented.
- Export reads assets, notes, relationships, and organisation through read interfaces; none of those subsystems know about `.peng` files.
- Import invokes repository interfaces after validation; the repository does not parse archives.
- Adapters consume canonical assets and relationships; canonical assets never contain harness-specific file paths as required core fields.
- The runtime policy boundary can block execution without affecting storage, search, or packaging.

### 6.2 Couplings explicitly rejected

1. **Separate tables and separate screens for every asset type.** This duplicates search, tagging, notes, and export logic and makes a sixth type disproportionately expensive.
2. **Embedding dependencies as copied content inside subagents.** Updating a skill would leave stale copies in every agent.
3. **Making `.peng` equal to a raw database backup.** That would expose internal schema choices, complicate selective export, and make long-term compatibility harder.
4. **Letting the frontend issue arbitrary SQL or shell commands.** This couples UI details to persistence and widens the native attack surface.
5. **Treating generated harness files as authoritative.** This causes the duplication and drift Peng is intended to eliminate.
6. **Executing an imported MCP server to “inspect” it.** Inspection must remain a data operation, not a trust decision.

---

## 7. Information Axiom: selecting the least-complex viable architecture

### 7.1 Qualitative stack assessment

Scores are design judgements from 1 (poor) to 5 (excellent) against Peng's constraints, not laboratory benchmarks.

| Candidate | Runtime footprint | Rich editor/UI productivity | Local OS/file/process access | Cross-platform | Security boundary | Delivery complexity | Weighted fit |
|---|---:|---:|---:|---:|---:|---:|---:|
| **Tauri 2 + Svelte + Rust** | 5 | 4.5 | 5 | 5 | 5 | 4 | **4.75** |
| Wails 2 + Svelte + Go | 5 | 4.5 | 5 | 4.5 | 3.5 | 4.5 | 4.35 |
| Avalonia 12 + .NET | 3 | 4 | 5 | 5 | 4 | 4.5 | 4.05 |
| Neutralinojs + Svelte/TypeScript | 5 | 4 | 3.5 | 4.5 | 3.5 | 3.5 | 4.00 |
| Flutter desktop + Dart | 3 | 4.5 | 4 | 5 | 4 | 4 | 4.00 |
| Pure Rust/Slint UI | 5 | 2.5 | 5 | 4.5 | 5 | 2.5 | 3.90 |
| Electron + TypeScript | 1.5 | 5 | 5 | 5 | 3.5 | 4.5 | 3.75 |

### 7.2 Why Tauri wins

Tauri is designed to use a system webview rather than shipping an entire browser runtime, and its documentation explicitly focuses on small binaries. It supports platform installers and app-store distribution. [1][2][9]

It also provides the native capabilities Peng needs without forcing them into v1:

- file dialogs and filesystem access;
- SQL support and migrations if the official SQL plugin is chosen;
- a shell plugin capable of starting child processes for a future opt-in MCP runner;
- a Stronghold plugin for secrets and keys;
- signed update artefacts;
- per-window/webview capability and permission controls. [2][10][11][12][15]

Svelte compiles components into lean JavaScript and CSS. A desktop client does not need server-side rendering, so Peng should use **Svelte directly with Vite**, not add SvelteKit routing/server features until a real need appears. [5]

### 7.3 Why Wails is the runner-up, not the default

Wails 2 is the closest alternative. It also reuses each platform's native webview, produces a single executable, supports Svelte templates, and gives the frontend typed access to Go methods. Its stable documentation is currently v2.15.0, while Wails 3 is still labelled beta. [16]

For a Go-first developer, Wails 2 is a valid choice. Tauri remains the recommendation because Peng specifically benefits from Tauri's documented capability boundaries and cohesive first-party SQL, shell, secret-store, and signed-updater facilities. Choosing Wails would trade some security-policy structure for a simpler application-core language.

### 7.4 Why not Neutralinojs

Neutralinojs is likely to produce the smallest shell. It uses the operating system's browser library and exposes native operations through a local WebSocket/native API, with additional capabilities supplied through extension IPC. [17]

That is attractive for a tiny utility, but Peng needs a durable database, schema migrations, strict archive parsing, secret references, optional process launching, and safe updates. Building and securing that glue would move complexity from the framework into Peng. The executable might be smaller while the product implementation becomes less certain—the wrong direction under the Information Axiom.

### 7.5 Why not Electron

Electron is extremely capable, but its official process model inherits Chromium's multi-process architecture and includes a Node.js main process plus renderer processes. That is useful for browser-like applications, but unnecessary overhead for a small local catalogue. [13]

### 7.6 Why not Avalonia as the first choice

Avalonia 12 is a credible alternative, especially for a C#-centred team. It offers a rich control library, compiled bindings, accessibility support, and broad desktop/mobile platform support. [14] It loses primarily on the “smallest practical footprint with a polished text-heavy UI” objective and makes web-editor integrations less direct.

### 7.7 Why not a pure Rust UI

A pure Rust toolkit such as Slint can produce an even leaner single-language runtime and supports desktop targets. [18] Peng is editor-heavy, however: syntax highlighting, structured forms, accessible keyboard interaction, drag-and-drop, diff views, and flexible lists matter more than shaving the last few megabytes. Rebuilding or integrating those capabilities would increase information content and reduce delivery probability.

---

## 8. Domain model

### 8.1 Canonical entities

```text
Asset
 ├─ has one working draft
 ├─ has many immutable versions
 ├─ has many notes
 ├─ has many activity events
 ├─ has many attachments
 ├─ has many tags
 ├─ belongs to many collections
 ├─ has outgoing/incoming relationships
 └─ has zero or more deployment records
```

### 8.2 Core data fields

#### `asset`

| Field | Type | Purpose |
|---|---|---|
| `id` | UUID | Stable identity across exports and machines. |
| `type` | enum/string | `prompt`, `skill`, `plugin`, `mcp_server`, `subagent`; unknown future values remain importable as generic assets. |
| `name` | text | Human-readable name. |
| `summary` | text | Short search/list description. |
| `status` | enum | `draft`, `ready`, `deprecated`, `archived`. |
| `source_kind` | enum | `created`, `imported`, `linked`, `generated`. |
| `source_uri` | nullable text | Optional repository, package, or documentation origin. |
| `working_version_id` | UUID | Current editable state. |
| `latest_snapshot_id` | nullable UUID | Most recent immutable checkpoint. |
| `created_at`, `updated_at` | RFC 3339 timestamp | Audit and sorting. |

#### `asset_version`

| Field | Type | Purpose |
|---|---|---|
| `id` | UUID | Version identity. |
| `asset_id` | UUID | Parent asset. |
| `revision` | integer | Monotonic local revision. |
| `semantic_version` | nullable text | Optional external/public version. |
| `is_snapshot` | boolean | Distinguishes working draft from immutable checkpoint. |
| `body` | text | Main Markdown/text/YAML/JSON content. |
| `type_data_json` | JSON text | Versioned type-specific data. SQLite's JSON functions are built in by default in modern SQLite and can query validated JSON while retaining a simple storage model. [4] |
| `schema_version` | text | Type schema version. |
| `content_hash` | SHA-256 | Duplicate detection and integrity. |
| `change_note` | nullable text | Why the snapshot was created. |
| `created_at` | timestamp | History. |

#### Supporting entities

- `tag`, `asset_tag`
- `collection`, `collection_asset`
- `note`
- `activity_event`
- `asset_relationship`
- `attachment`
- `secret_reference`
- `deployment`
- `saved_view`
- `bundle_receipt` for prior imports/exports

### 8.3 Relationship types

| Relationship | Meaning | Included by default when exporting source? |
|---|---|---:|
| `requires` | Source cannot function without target. | Yes |
| `uses` | Source normally invokes or includes target. | Suggested |
| `extends` | Source specialises target. | Yes |
| `optional` | Useful but non-essential target. | No; selectable |
| `compatible_with` | Compatibility declaration, not ownership. | No |
| `supersedes` | Source replaces target. | No |
| `related_to` | Informational link. | No |
| `conflicts_with` | Warn when both are selected/deployed. | No |

---

## 9. Type-specific schemas

### 9.1 Prompt

Required:

- body;
- name.

Optional:

- variables: name, description, required flag, default, example;
- intended task/category;
- compatible models or harnesses;
- expected input and output format;
- temperature/reasoning hints as advisory metadata;
- examples and test cases;
- linked skills/subagents.

### 9.2 Skill

Required:

- primary instructions or entry document;
- name.

Optional:

- trigger/when-to-use description;
- tool requirements;
- supporting files;
- allowed and forbidden actions;
- prerequisites;
- target harness compatibility;
- entry-point path;
- linked prompts, MCP servers, plugins, or subagents.

### 9.3 Plugin

Because “plugin” has no single universal packaging standard, Peng stores a canonical descriptor and files while adapters perform target-specific rendering.

Required:

- name;
- target family or `generic`;
- descriptor/manifest content or source reference.

Optional:

- package files;
- install scope (`global`, `workspace`, `either`);
- source repository/package identifier;
- compatible host versions;
- capabilities/permissions;
- update source;
- linked skills, MCP servers, prompts, and subagents.

### 9.4 MCP server

Required:

- name;
- transport: `stdio`, `streamable_http`, or `custom`;
- connection or launch details appropriate to the transport.

Optional:

- command and argument array for `stdio`;
- URL and headers-as-secret-references for Streamable HTTP;
- working directory;
- environment-variable references;
- expected tools, prompts, and resources;
- package/source repository;
- protocol compatibility;
- setup notes;
- optional health-check definition;
- allowed target adapters.

Peng stores command arguments as an array, never as an opaque shell string, unless an explicit custom transport requires one. Secret values are represented as placeholders such as `${PENG_SECRET:OPENAI_API_KEY}`.

### 9.5 Subagent

Required:

- name;
- system or role instructions.

Optional:

- description and activation criteria;
- preferred model/provider as a non-binding hint;
- reasoning/effort hint;
- allowed and denied tools;
- linked skills, prompts, MCP servers, and plugins;
- input/output contract;
- handoff rules;
- filesystem/network constraints;
- target harness compatibility;
- example tasks and evaluation cases.

---

## 10. User experience and information architecture

### 10.1 Core interaction model

Peng uses one main window with three stable regions:

1. **Navigation rail:** library views, types, collections, settings.
2. **Result list:** search results and compact asset summaries.
3. **Workspace:** selected asset editor, setup, links, notes, and history.

This avoids forcing the user through a deep hierarchy of separate pages. The selected asset stays visible while filters or collections change.

### 10.2 Main window wireframe

```text
┌─────────────────────────────────────────────────────────────────────────────────────────────────────┐
│  Peng            Search everything…  Ctrl+K                         Import    Export (3)    + New    │
├───────────────────┬────────────────────────────────┬────────────────────────────────────────────────┤
│ LIBRARY           │ All items                 128  │ Prompt · Ready                         ☆  •••  │
│  All items     128│ [type ▾] [tag ▾] [sort ▾]     │ PR merge-readiness reviewer                    │
│  Favourites     12│                                │ Finds genuine blockers without review churn.    │
│  Recent           │ □  PR merge-readiness reviewer│                                                │
│  Archived         │    Prompt · ready · 2 links    │ [Editor] [Setup] [Links] [Notes & history]     │
│                   │                                │                                                │
│ TYPES             │ □  Repository research         │ Summary                                        │
│  Prompts        42│    Skill · ready · 6 files     │ ┌────────────────────────────────────────────┐ │
│  Skills         31│                                │ │ Finds genuine blockers…                    │ │
│  Plugins        17│ □  Filesystem server           │ └────────────────────────────────────────────┘ │
│  MCP servers    20│    MCP · stdio · 3 dependants  │                                                │
│  Subagents      18│                                │ Prompt                                         │
│                   │ □  Review coordinator           │ ┌────────────────────────────────────────────┐ │
│ COLLECTIONS       │    Subagent · draft · 4 links   │ │ You are reviewing a pull request…          │ │
│  Pegasus          │                                │ │ {{scope}}                                   │ │
│  Kanmer            │                                │ │ Return only concrete blockers…             │ │
│  Local LLM         │                                │ └────────────────────────────────────────────┘ │
│                   │                                │ Variables  scope · repository · constraints    │
│ + Collection      │                                │                                                │
│                   │                                │ Tags  review  github  coding                   │
│ Settings          │                                │ Saved just now                 Create version │
└───────────────────┴────────────────────────────────┴────────────────────────────────────────────────┘
```

### 10.3 Navigation design

- **All items** is the default view.
- Type entries are ordinary filters, not separate applications.
- Collections are user-created reference sets.
- Tags appear as a filter popover rather than an always-expanded wall of labels.
- Settings stays at the bottom; import/export remain visible in the title bar because they are core actions.

### 10.4 Result list design

Each row shows only:

- selection checkbox for the export basket;
- name;
- type badge;
- status;
- one short summary line;
- modified time or relationship count when useful.

No card should contain instructions explaining what buttons do. Empty states may contain one direct action, such as **Create prompt** or **Import bundle**.

### 10.5 Workspace tabs

| Tab | Content |
|---|---|
| **Editor** | Main body, shared metadata, variables or other frequent type fields. |
| **Setup** | Type-specific configuration: MCP transport/command, plugin target, skill entry point, or subagent execution contract. |
| **Links** | Dependencies, dependants, compatibility, and collection membership. |
| **Notes & history** | Quick note field, pinned notes, activity timeline, versions, and diff action. |

The frequent path remains shallow. Specialist fields are present but do not dominate routine prompt editing.

### 10.6 New asset flow

```text
+ New
 ┌─────────────────────────────────────┐
 │ What are you creating?              │
 │                                     │
 │ [ Prompt ]   [ Skill ]              │
 │ [ Plugin ]   [ MCP server ]         │
 │ [ Subagent ]                        │
 │                                     │
 │ Name  ____________________________   │
 │                                     │
 │                 Cancel   Create     │
 └─────────────────────────────────────┘
```

After creation, focus moves directly to the main body. Only name and type are initially required. The type editor supplies a compact starter template that may be cleared.

### 10.7 Export flow

```text
┌──────────────────────────────────────────────────────────────────────┐
│ Create Peng bundle                                                   │
├──────────────────────────────────────────────────────────────────────┤
│ 3 selected     4 required dependencies     2 optional links          │
│                                                                      │
│ ✓ PR merge-readiness reviewer                         Prompt          │
│ ✓ Review coordinator                                  Subagent        │
│   ├─ ✓ Repository research                            Skill · required│
│   ├─ ✓ GitHub MCP                                      MCP · required  │
│   └─ □ Issue summariser                               Prompt · optional│
│                                                                      │
│ Include                                                               │
│ [✓] Current versions   [✓] Attachments   [ ] Version history          │
│ [✓] Notes              [ ] Activity log                               │
│                                                                      │
│ Secret values will not be included. 3 references remain placeholders.│
│                                                                      │
│ Bundle name  review-pack-2026-09                                     │
│                                           Cancel   Export .peng       │
└──────────────────────────────────────────────────────────────────────┘
```

### 10.8 Import flow

```text
┌──────────────────────────────────────────────────────────────────────┐
│ Import review-pack.peng                                              │
├──────────────────────────────────────────────────────────────────────┤
│ Valid Peng 1.0 bundle        7 assets        11 files        3 refs   │
│                                                                      │
│ New       4     Updates       2     Conflict       1                  │
│                                                                      │
│ PR merge-readiness reviewer      Same ID, newer version              │
│ Action: [Import as new version ▾]                                    │
│                                                                      │
│ Filesystem MCP                   Same ID, different content           │
│ Action: [Review difference ▾]                                        │
│                                                                      │
│ Security                                                             │
│ • Contains executable launch definitions; none will be run.           │
│ • Secret values are absent; 3 references need mapping.                │
│                                                                      │
│ [ ] Remember bulk choices                                             │
│                                      Cancel   Import 6 · Skip 1       │
└──────────────────────────────────────────────────────────────────────┘
```

### 10.9 Keyboard model

| Shortcut | Action |
|---|---|
| `Ctrl/Cmd + K` | Focus global search/command palette. |
| `Ctrl/Cmd + N` | New asset chooser. |
| `Ctrl/Cmd + S` | Flush working draft immediately. |
| `Ctrl/Cmd + Shift + S` | Create immutable version snapshot. |
| `Ctrl/Cmd + E` | Open export basket. |
| `Ctrl/Cmd + I` | Import bundle. |
| `Alt + 1..4` | Switch workspace tab. |
| `Ctrl/Cmd + P` | Quick open by asset name. |

### 10.10 Visual language

- Neutral slate surfaces with one restrained teal accent.
- Colour is never the only status indicator; status always has text and/or icon.
- Type badges are compact and consistent, not large decorative panels.
- Default density is comfortable, with a compact option.
- Minimum interactive target: 32 × 32 px on desktop; primary actions 36–40 px high.
- System font stack; no bundled font dependency.
- Light and dark themes use the same layout and semantics.

---

## 11. `.peng` portable format v1

### 11.1 Design choice

A `.peng` file is a ZIP-compatible archive with a distinct extension and proposed media type:

```text
application/vnd.peng.bundle+zip
```

Using a documented archive rather than serialising Peng's database has four advantages:

1. selective exports are natural;
2. contents can be inspected with ordinary tools;
3. internal database schema can evolve independently;
4. corrupted or future bundles can often be partially recovered.

### 11.2 Archive layout

```text
review-pack.peng
├── mimetype
├── manifest.json
├── schema/
│   └── peng-bundle-1.0.schema.json
├── assets/
│   ├── 018f...a2/
│   │   ├── asset.json
│   │   └── versions/
│   │       └── 0003/
│   │           ├── version.json
│   │           ├── body.md
│   │           └── files/
│   │               └── examples.md
│   └── 018f...f8/
│       └── ...
├── relationships.json
├── collections.json          # optional
├── notes.ndjson              # optional
├── events.ndjson             # optional
├── checksums.sha256
└── signatures/
    └── manifest.ed25519       # optional future-compatible field
```

### 11.3 Manifest

```json
{
  "format": "peng",
  "format_version": "1.0",
  "bundle_id": "01990a5f-dc8d-7c48-88a0-f3f2b11c2860",
  "name": "review-pack-2026-09",
  "created_at": "2026-09-02T15:30:00Z",
  "producer": {
    "name": "Peng",
    "version": "0.1.0"
  },
  "export_policy": {
    "versions": "current",
    "include_attachments": true,
    "include_notes": true,
    "include_events": false,
    "secrets": "references_only"
  },
  "assets": [
    {
      "id": "01990a5f-e1a0-7334-9c70-677f86fd20aa",
      "type": "prompt",
      "path": "assets/01990a5f-e1a0-7334-9c70-677f86fd20aa/asset.json",
      "current_revision": 3,
      "content_hash": "sha256:..."
    }
  ],
  "schema_path": "schema/peng-bundle-1.0.schema.json",
  "relationships_path": "relationships.json",
  "collections_path": "collections.json",
  "notes_path": "notes.ndjson",
  "checksums_path": "checksums.sha256"
}
```

### 11.4 Asset descriptor

```json
{
  "id": "01990a5f-e1a0-7334-9c70-677f86fd20aa",
  "type": "prompt",
  "schema_version": "1.0",
  "name": "PR merge-readiness reviewer",
  "summary": "Finds genuine blockers without review churn.",
  "status": "ready",
  "tags": ["review", "github", "coding"],
  "source": {
    "kind": "created",
    "uri": null
  },
  "versions": [
    {
      "revision": 3,
      "path": "versions/0003/version.json"
    }
  ],
  "secret_references": []
}
```

### 11.5 Format rules

- UTF-8 for JSON, NDJSON, Markdown, YAML, and text files; text bodies use LF line endings when hashed.
- Forward-slash relative paths only.
- No absolute paths, drive letters, `..`, symlinks, hard links, device files, or alternate data streams.
- Stable UUIDs provide identity; a separate semantic `content_hash` supports equality and duplicate detection.
- `mimetype` is the first ZIP entry, contains exactly the proposed media type, and is stored without compression. All remaining entries are ordered lexicographically for deterministic output.
- ZIP entry timestamps are fixed in deterministic mode; meaningful creation timestamps remain inside JSON metadata.
- `checksums.sha256` contains lowercase SHA-256, two spaces, and the relative path for every regular entry except itself and `signatures/` entries.
- Current version only by default; older versions are an explicit export option.
- Unknown asset types and unknown JSON properties are preserved when possible and displayed as generic structured content.
- Secret values are prohibited unless a future encrypted extension is explicitly selected. V1 exporters write references only.
- Import never installs, launches, or loads code.

#### Semantic content hash

`manifest.assets[].content_hash` is not a hash of the ZIP bytes or of the asset UUID. It is SHA-256 over a canonical semantic object containing:

- type and type schema version;
- name, summary, status, sorted tags, and secret-reference descriptors;
- the current body's media type and LF-normalised text;
- current type-specific data;
- attachment relative paths, media types, sizes, and SHA-256 values in path order.

The object excludes identity, source provenance, local revision numbers, and timestamps so that equivalent assets with different UUIDs can still be recognised as probable duplicates. It is serialised with RFC 8785 JSON Canonicalization Scheme before hashing. File-level integrity remains independently covered by `checksums.sha256`. [19]

### 11.6 Conflict policy

Conflict detection order:

1. Same asset ID and same content hash → already present; skip by default.
2. Same asset ID and a higher compatible revision → import as a new version by default.
3. Same asset ID and divergent content → require a choice: compare, duplicate with new ID, replace working draft, or skip.
4. Different asset ID and same content hash → probable duplicate; suggest linking or skipping.
5. Same name only → informational warning, not an identity conflict.

### 11.7 Bundle safety limits

Configurable defaults:

- maximum archive size: 1 GiB;
- maximum expanded size: 4 GiB;
- maximum entry count: 25,000;
- maximum compression ratio: 100:1 per entry;
- maximum JSON/manifest size: 10 MiB;
- maximum individual text asset body: 16 MiB before explicit confirmation.

Limits protect against malformed archives and decompression bombs while remaining well above normal prompt/skill usage.

---

## 12. Technical architecture

```text
┌────────────────────────────────────────────────────────────────┐
│ Svelte 5 + TypeScript UI                                       │
│ library views · editors · search · import/export review        │
└──────────────────────────────┬─────────────────────────────────┘
                               │ typed Tauri commands/events
┌──────────────────────────────▼─────────────────────────────────┐
│ Rust application services                                      │
│ AssetService · SearchService · JournalService · RelationService│
│ BundleService · ImportService · AdapterService · PolicyService │
│ Optional later: RunnerService                                  │
└───────────────┬─────────────────────┬──────────────────────────┘
                │                     │
       ┌────────▼─────────┐  ┌────────▼─────────────────────────┐
       │ SQLite           │  │ Managed content directory        │
       │ metadata · text  │  │ larger/binary attachment files   │
       │ notes · FTS      │  │ addressed by relative path/hash  │
       └────────┬─────────┘  └──────────────────────────────────┘
                │
       ┌────────▼─────────┐
       │ Secret store     │
       │ references in DB │
       │ values encrypted │
       └──────────────────┘
```

### 12.1 Boundary rules

- UI code cannot issue arbitrary SQL.
- UI code cannot spawn arbitrary commands.
- Every native command accepts a typed request and returns a typed result/error.
- Bundle parsing happens in Rust, outside the webview.
- Path and archive validation occurs before any extraction.
- The optional runner uses an allow-list generated from an explicitly approved asset, not an unrestricted shell string. Tauri's shell plugin blocks dangerous commands by default until permissions/scopes are configured. [10]
- The editor lazily loads language support to keep initial startup small.

### 12.2 Persistence strategy

SQLite stores:

- common asset metadata;
- text bodies and structured JSON;
- notes and activity events;
- tags, collections, and relationships;
- attachment metadata and hashes;
- deployment records;
- bundle receipts.

The managed content directory stores:

- binary files;
- large or multi-file skill/plugin attachments;
- optional cached previews.

The database remains authoritative for metadata. Attachment writes use a staging directory, hash verification, then atomic rename before the database transaction commits its reference.

### 12.3 Search strategy

FTS5 indexes a flattened search document containing:

- name and summary;
- main body;
- tags and collection names;
- selected type fields;
- optionally manual notes.

Structured filters remain ordinary indexed SQL columns/tables. Rebuilding FTS from canonical records is supported, so a damaged search index does not endanger source data.

### 12.4 Autosave and version strategy

- Text edits debounce for 500–750 ms.
- A dirty indicator appears immediately.
- Draft save updates one mutable working version inside a transaction.
- **Create version** copies the working state into an immutable snapshot with optional change note.
- Importing an update normally creates a snapshot rather than silently replacing history.
- Activity events are semantic actions, not a noisy record of every keystroke.

### 12.5 Error model

Rust returns structured errors:

```ts
type PengError = {
  code: string;
  message: string;
  field?: string;
  details?: Record<string, unknown>;
  retryable: boolean;
};
```

Stable examples:

- `ASSET_NOT_FOUND`
- `VALIDATION_FAILED`
- `BUNDLE_UNSUPPORTED_VERSION`
- `BUNDLE_CHECKSUM_MISMATCH`
- `BUNDLE_UNSAFE_PATH`
- `IMPORT_CONFLICT`
- `SECRET_REFERENCE_UNRESOLVED`
- `DEPLOYMENT_DRIFT_DETECTED`

UI messages stay direct and actionable; diagnostics may be copied from an expandable detail region.

---

## 13. Target adapter contract

Adapters are optional modules that transform a canonical Peng asset graph into files/configuration for a host.

```rust
trait TargetAdapter {
    fn descriptor(&self) -> AdapterDescriptor;
    fn validate(&self, graph: &ResolvedAssetGraph) -> ValidationReport;
    fn plan(&self, graph: &ResolvedAssetGraph, target: &TargetContext) -> DeploymentPlan;
    fn apply(&self, approved_plan: &DeploymentPlan) -> ApplyReport;
    fn inspect_drift(&self, receipt: &DeploymentReceipt) -> DriftReport;
}
```

A deployment plan contains:

- exact destination paths;
- create/update/delete operations;
- rendered preview or diff;
- required permissions;
- unresolved secret references;
- collisions or drift warnings.

Initial adapter priority after core v1:

1. Generic folder/manifest adapter.
2. `.agents` global and workspace layouts.
3. Codex global plugin/skill projection.
4. Claude Code projection.
5. Generic MCP client JSON projection.

This sequencing proves the abstraction with simple file targets before adding host-specific edge cases.

---

## 14. Security and trust model

### 14.1 Trust levels

| Content | Default trust |
|---|---|
| User-created text | Editable local content. |
| Imported text/config | Untrusted data; view/edit allowed. |
| Imported executable/script | Untrusted attachment; never run automatically. |
| Secret reference | Safe identifier; value resolved only locally. |
| Deployment plan | Preview only until explicit approval. |
| MCP launch | Separate explicit action with target-specific command/path scope. |

### 14.2 Required controls

- strict archive path sanitisation;
- checksum verification before import;
- JSON schema validation;
- size/count/compression limits;
- Content Security Policy with no remote scripts;
- no arbitrary frontend shell/SQL access;
- no secret values in logs, previews, or bundles;
- field-level redaction for values resembling credentials;
- command and argument arrays, not shell interpolation;
- opt-in network access for future online features;
- signed application updates; Tauri can generate platform update bundles and signatures. [12]

Tauri capabilities should grant only the exact filesystem, dialog, updater, and optional runner permissions required by each window. Its capability system can constrain frontend access per window/webview and platform. [2]

---

## 15. Performance, reliability, and accessibility targets

These are acceptance targets, not claims about an unbuilt application.

| Area | Target |
|---|---|
| Cold start | ≤ 1.5 seconds on a typical 2020-or-newer Windows laptop after installation. |
| Idle memory | ≤ 100 MiB working set in the normal single-window library view, excluding OS webview sharing variance. |
| Search | P95 ≤ 100 ms for 50,000 text assets. |
| Editor input | No visible input lag for a 1 MiB text document. |
| Save | Working draft transaction P95 ≤ 150 ms for ordinary assets. |
| Recovery | No committed database reference to an attachment that was not fully written and hashed. |
| Import | Invalid or cancelled import leaves the library unchanged. |
| Accessibility | Keyboard-complete; visible focus; screen-reader names; text alternatives; no colour-only meaning; 200% scaling without loss of function. |
| Offline | All core create/search/note/organise/import/export features work with network disabled. |

---

## 16. Delivery plan

### Phase 0 — Format and vertical slice

Build only:

- canonical asset model;
- prompt editor;
- SQLite persistence/search;
- `.peng` export/import round trip;
- checksum and unsafe-path tests.

Exit criterion: create a prompt, export it, delete it, import it, and recover the same ID/content/metadata.

### Phase 1 — Complete lightweight library

Add:

- all five asset editors;
- tags, collections, favourites, archive;
- notes and activity;
- relationships;
- export basket and dependency closure;
- versions and diffs;
- backup/restore;
- import conflict resolver;
- keyboard and accessibility pass.

This is the first useful release.

### Phase 2 — Projection adapters

Add:

- generic folder/manifest adapter;
- `.agents`, Codex, Claude Code, and generic MCP-client targets;
- plan/diff/apply workflow;
- deployment receipts and drift detection.

### Phase 3 — Optional operational tools

Only after clear demand:

- explicit MCP connection test and runner;
- registry/source refresh;
- Git import/export;
- encrypted secret transfer;
- optional sync provider.

No phase should introduce an account or permanent service merely to support local asset management.

---

## 17. Acceptance scenarios

### A. Rapid prompt capture

1. Press `Ctrl+N`.
2. Choose Prompt and enter a name.
3. Paste content.
4. Close and reopen Peng.
5. Search for a phrase from the body.

**Pass:** the result appears and opens with the saved working draft.

### B. Cross-type composition

1. Create an MCP server, a skill, and a subagent.
2. Link the subagent as requiring the MCP server and using the skill.
3. Change the skill instructions.

**Pass:** the subagent still points to the same skill identity; no embedded stale copy exists.

### C. Selective portable export

1. Add the subagent to the export basket.
2. Review required and optional dependencies.
3. Exclude activity events and include current versions/attachments.
4. Export.

**Pass:** the bundle contains the selected subagent, required dependencies, chosen data categories, checksums, and secret placeholders only.

### D. Safe import

1. Open a bundle containing launchable MCP metadata and scripts.
2. Preview it and cancel.

**Pass:** nothing executes, no destination files are written, and the library remains unchanged.

### E. Divergent conflict

1. Modify the same asset independently on two computers.
2. Export from one and import on the other.

**Pass:** Peng detects the stable-ID divergence and offers compare, import as alternate/new version, duplicate, or skip; it does not silently overwrite.

### F. Canonical-to-many deployment

1. Store one skill in Peng.
2. Plan deployment to a global Codex target and a workspace `.agents` target.
3. Apply both.
4. Edit the canonical skill.

**Pass:** both targets show drift/outdated state, while Peng still contains one canonical asset.

---

## 18. Key product decisions

| Decision | Choice | Reason |
|---|---|---|
| Local vs cloud | Local-first | Core use needs no infrastructure, account, latency, or privacy compromise. |
| One model vs five stores | One canonical Asset aggregate | Shared organisation/history/export logic remains independent of type editors. |
| Database vs loose files | SQLite plus managed attachments | Strong search and transactions without forcing binary trees into relational rows. |
| Working edits vs versions | Autosaved draft plus explicit snapshot | Prevents data loss without producing a version per keystroke/save. |
| Bundle representation | ZIP-compatible `.peng` | Selective, inspectable, recoverable, and independent from DB schema. |
| Secrets | References only | A portable pack should not accidentally become a credential leak. |
| Imported execution | Never automatic | Storage and trust remain separate decisions. |
| Harness support | Adapters/projections | One source can serve incompatible layouts without source duplication. |
| Frontend framework | Svelte 5 SPA with Vite | Rich UI with a small compiled frontend; no unnecessary server framework. |
| Desktop framework | Tauri 2 | Best balance of footprint, native capability, cross-platform packaging, and security control. |

---

## 19. Recommended repository structure

```text
peng/
├── src/                         # Svelte UI
│   ├── lib/
│   │   ├── components/
│   │   ├── editors/
│   │   │   ├── prompt/
│   │   │   ├── skill/
│   │   │   ├── plugin/
│   │   │   ├── mcp-server/
│   │   │   └── subagent/
│   │   ├── stores/
│   │   └── ipc/
│   └── routes/                  # lightweight client routes/views only
├── src-tauri/
│   ├── capabilities/
│   ├── migrations/
│   ├── src/
│   │   ├── domain/
│   │   ├── services/
│   │   ├── repository/
│   │   ├── bundle/
│   │   ├── import/
│   │   ├── adapters/
│   │   ├── policy/
│   │   └── commands/
│   └── tests/
├── schemas/
│   ├── peng-bundle-1.0.schema.json
│   └── asset-types/
├── fixtures/
│   ├── bundles/
│   └── malicious-bundles/
└── docs/
    ├── format.md
    ├── adapter-contract.md
    └── threat-model.md
```

Keep commands thin: validate request, invoke one application service, return a typed response. Do not put database SQL, ZIP parsing, and UI-shaped formatting into the same command handler.

---

## 20. Sources

1. Tauri, “What is Tauri?” — https://v2.tauri.app/start/
2. Tauri, “Capabilities” — https://v2.tauri.app/security/capabilities/
3. SQLite, “FTS5 Extension” — https://www.sqlite.org/fts5.html
4. SQLite, “JSON Functions and Operators” — https://sqlite.org/json1.html
5. Svelte, “Getting started” and “Overview” — https://svelte.dev/docs/svelte/getting-started and https://svelte.dev/docs/svelte
6. MIT DSpace, “Axiomatic Design: 30 Years After” — https://dspace.mit.edu/bitstream/handle/1721.1/107378/Kim_Axiomatic%20design.pdf
7. NASA Technical Reports Server, “Axiomatic Design of Space Life Support Systems” — https://ntrs.nasa.gov/api/citations/20170010336/downloads/20170010336.pdf
8. Model Context Protocol specification, “Transports” — https://modelcontextprotocol.io/specification/2026-07-28/basic/transports
9. Tauri, “App Size” and “Distribute” — https://v2.tauri.app/concept/size/ and https://v2.tauri.app/distribute/
10. Tauri, “Shell” — https://v2.tauri.app/plugin/shell/
11. Tauri, “Stronghold” — https://v2.tauri.app/plugin/stronghold/
12. Tauri, “Updater” — https://v2.tauri.app/plugin/updater/
13. Electron, “Process Model” — https://www.electronjs.org/docs/latest/tutorial/process-model
14. Avalonia 12 documentation — https://docs.avaloniaui.net/docs/welcome
15. Tauri, “SQL” plugin — https://v2.tauri.app/plugin/sql/
16. Wails v2, “Introduction” — https://wails.io/docs/introduction/
17. Neutralinojs, “Introduction” — https://neutralino.js.org/docs/
18. Slint, “Overview” — https://docs.slint.dev/latest/docs/slint/
19. RFC Editor, RFC 8785, “JSON Canonicalization Scheme” — https://www.rfc-editor.org/rfc/rfc8785.html

