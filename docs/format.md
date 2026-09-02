# Peng portable format

The normative v1 design is `Peng_Design_Pack/Peng_Axiomatic_Design_Spec.md` section 11, with machine-readable validation in `Peng_Design_Pack/Peng_Bundle_1.0.schema.json` and a complete reference bundle in `Peng_Design_Pack/Peng_Format_v1_Example.peng`.

Implementations must preserve deterministic ordering, semantic hashes, file checksums, safe paths, bounded archive expansion, staged import, transactional commit, unknown-field preservation, and references-only secret handling. A bundle is always data during import and preview; it is never executed.

