# Invariant: Documentation Coverage

### Scope

- **Purpose**: Requires all public API items to carry doc comments for `cargo doc` completeness.
- **Responsibility**: Documents the documentation coverage requirement and its verification.
- **In Scope**: All `pub` traits, structs, enums, functions, and methods in `src/`.
- **Out of Scope**: Private items, internal helpers, test utilities.

### Invariant Statement

Every public API item — trait, struct, enum, function, and method — must have a doc comment explaining its purpose, parameters, return values, and errors. `cargo doc --all-features` must produce zero missing-doc warnings.

### Enforcement Mechanism

Run `RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features`. The `#![deny(missing_docs)]` attribute in `src/lib.rs` enforces this at compile time for the crate itself.

### Violation Consequences

Missing documentation makes the library harder to adopt. Users cannot understand behavior from `cargo doc` alone and must read source code, increasing integration friction.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | `#![deny(missing_docs)]` enforcement |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | NFR5 in original spec; combined source migrated to invariant/. spec.md has been deleted — Sources entry retained as migration record. |
