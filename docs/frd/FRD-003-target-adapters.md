# FRD-003: Explicit target adapters

## Behavior

After the core library is complete, Peng must render one canonical asset graph for external harnesses through previewable, explicit target adapters.

## Acceptance criteria

1. A generic folder adapter can validate an asset graph and produce a no-write plan with exact destination operations.
2. An approved plan applies only its displayed paths and records destination hashes.
3. Local destination drift blocks overwrite and presents the difference.
4. Adding one adapter does not alter canonical asset schemas or another adapter's output.
5. Editing a canonical asset marks all affected deployments outdated without creating duplicate canonical records.

