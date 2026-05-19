# Invariant: Documentation Coverage

### Scope

- **Purpose**: Requires all public API items to carry doc comments for complete generated documentation.
- **Responsibility**: Documents the documentation coverage requirement and its verification.
- **In Scope**: All public traits, structs, enums, functions, and methods in `src/`.
- **Out of Scope**: Private items, internal helpers, test utilities.

### Invariant Statement

Every public API item — trait, struct, enum, function, and method — must have a doc comment explaining its purpose, parameters, return values, and errors. The documentation build must produce zero missing-doc warnings.

### Enforcement Mechanism

Run `RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features`. The missing-docs lint in `src/lib.rs` enforces this at compile time for the crate itself.

### Violation Consequences

Missing documentation makes the library harder to adopt. Users cannot understand behavior from the generated documentation alone and must read source code, increasing integration friction.

### Sources

| File | Relationship |
|------|--------------|
| [`src/lib.rs`](../../src/lib.rs) | Missing documentation attribute enforcement |
