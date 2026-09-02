# FRD-002: Complete local library

## Behavior

Peng must provide the full local-first library experience across all five asset types after the vertical slice is proven.

## Acceptance criteria

1. Prompts, skills, plugins, MCP servers, and subagents share search, organisation, notes, history, relationships, and export behavior while exposing only relevant specialist fields.
2. Tags, collections, favourites, saved views, and archive state organise references without copying assets.
3. Draft edits autosave and explicit version creation produces an immutable snapshot with an optional change note.
4. Required and extending relationships are included in export closure; optional and informational links require deliberate selection.
5. Backup/restore preserves asset identities and can rebuild derived search indexes.
6. The full core workflow is keyboard-complete, visibly focused, screen-reader named, non-colour-dependent, and usable at 200% scaling.

