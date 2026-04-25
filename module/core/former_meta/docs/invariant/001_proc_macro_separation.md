# Invariant: Proc-Macro Crate Separation

### Scope

- **Purpose**: Enforce that macro implementations reside in a dedicated proc-macro crate.
- **Responsibility**: Document the two-crate split constraint and its structural enforcement mechanism.
- **In Scope**: The proc-macro crate declaration invariant, enforcement, and violation consequences.
- **Out of Scope**: The consumer-facing facade — see `integration/003_former.md`.

### Invariant Statement

This crate must always be declared as a proc-macro crate (`[lib] proc-macro = true` in
`Cargo.toml`). It must not export regular library items such as types, traits, or
functions intended for runtime use. All runtime contracts belong in `former_types` or
`former`.

### Enforcement Mechanism

The Rust compiler enforces this implicitly: proc-macro crates cannot be used as regular
library dependencies. Any attempt to import types from a proc-macro crate at compile time
produces a linker or type-resolution error. The two-crate split is therefore structurally
enforced by the toolchain.

### Violation Consequences

Placing runtime types in this crate would make them inaccessible to downstream consumers
who depend on `former` (not `former_meta` directly). Generated macro code that references
types defined here would fail to compile for all consumers. The split must be maintained
to preserve usability.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../feature/001_former_derive.md` | The macro capability constrained by this invariant |
| doc | `../integration/003_former.md` | Consumer facade enabled by this separation |
| doc | `../integration/002_former_types.md` | Runtime type crate separate from this proc-macro crate |
