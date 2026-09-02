# PRD-001: Local asset library

## Goal

Provide one coherent local library for prompts, skills, plugins, MCP server definitions, and subagents.

## Requirements

- Store common identity, descriptive metadata, working content, and type-specific data through one canonical Asset aggregate.
- Persist assets locally in SQLite and retrieve them through full-text search and structured filters.
- Organise without duplication through tags, collections, favourites, saved views, and archive state.
- Preserve an autosaved working draft, explicit immutable versions, notes, and meaningful activity events.
- Represent composition through typed relationships between stable asset identities.

## Out of scope

Cloud accounts, collaboration, marketplaces, chat, automatic execution, and target-harness deployment.

## Success measures

- Search P95 is at most 100 ms for 50,000 ordinary text assets on the reference class of hardware.
- Ordinary draft-save P95 is at most 150 ms.
- Core library operation works with networking disabled.

