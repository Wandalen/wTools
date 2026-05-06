# Invariant: Documentation

### Scope

- **Purpose**: Ensures the CLI is well-documented for users and contributors.
- **Responsibility**: Documents the documentation completeness requirements.
- **In Scope**: README quick start, all public items documented, working examples, `cargo doc` clean build.
- **Out of Scope**: Architectural design docs (→ `docs/`), test documentation.

### Invariant Statement

The `readme.md` must include a quick start guide and at least one end-to-end workflow example. All public items must have doc comments. All documentation examples must compile and run. Doc-test validation must produce zero warnings.

### Enforcement Mechanism

A compile-time lint enforces doc comment coverage on all public items. Doc-test validation ensures all documentation examples compile and run. README is manually reviewed for completeness during release.

### Violation Consequences

Missing documentation is the primary barrier to adoption. Users cannot discover commands or understand parameters without a working quick start. Broken examples erode confidence in the library.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Compile-time missing-docs lint enforcement |
| doc | `docs/cli/readme.md` | CLI design documentation for contributors |
