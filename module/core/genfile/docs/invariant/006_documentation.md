# Invariant: Documentation

### Scope

- **Purpose**: Ensures the CLI is well-documented for users and contributors.
- **Responsibility**: Documents the documentation completeness requirements.
- **In Scope**: README quick start, all public items documented, working examples, `cargo doc` clean build.
- **Out of Scope**: Architectural design docs (→ `docs/`), test documentation.

### Invariant Statement

The `readme.md` must include a quick start guide and at least one end-to-end workflow example. All public items must have doc comments. All documentation examples must compile and run. `RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features` must produce zero warnings.

### Enforcement Mechanism

`#![deny(missing_docs)]` in `src/lib.rs` enforces doc comment coverage at compile time. `cargo test --doc` validates all doc examples. README is manually reviewed for completeness during release.

### Violation Consequences

Missing documentation is the primary barrier to adoption. Users cannot discover commands or understand parameters without a working quick start. Broken examples erode confidence in the library.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | `#![deny(missing_docs)]` enforcement |
| doc | `docs/cli/readme.md` | CLI design documentation for contributors |
