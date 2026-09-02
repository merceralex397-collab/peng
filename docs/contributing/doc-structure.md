# Documentation structure

- `docs/product/` states the product vision and unresolved product decisions.
- `docs/prd/` defines product spans and their outcomes.
- `docs/frd/` defines observable behavior and acceptance criteria.
- `docs/adr/` records cross-cutting technical decisions and consequences.
- `docs/architecture.md` maps production boundaries.
- `docs/format.md` points to the portable-format contract.
- `docs/threat-model.md` records assets, threats, and mandatory controls.
- `Peng_Design_Pack/` retains the supplied source design, schema, fixture, and prototype.

Update the governing document and affected implementation/tests together when behavior changes. New work should link the PRD, FRD, or ADR it implements rather than restating those requirements in ticket prose.
