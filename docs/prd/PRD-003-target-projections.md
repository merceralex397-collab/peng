# PRD-003: Target projections

## Goal

Project canonical Peng assets into supported AI-harness layouts without duplicating their source.

## Requirements

- Adapters declare supported assets, platforms, destinations, and required fields.
- Planning produces exact creates, updates, deletes, diffs, warnings, permissions, and unresolved references without writes.
- Applying requires explicit approval and records destination hashes.
- Later edits detect deployed outputs as stale; destination drift is never silently overwritten.

## Delivery position

This is Phase 2 work. It must not complicate the Phase 0 bundle vertical slice or Phase 1 local library.

