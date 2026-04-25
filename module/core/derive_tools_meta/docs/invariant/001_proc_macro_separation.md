# Invariant: Proc-Macro Crate Separation

### Scope

- Defines the structural constraint that keeps macro implementations in a dedicated crate.
- Covers the reason this crate exists separately from its consumer facade.
- Audience: contributors who might consider merging this crate into another.
- Does not cover the public API contract — see `api/001_derive_api.md`.

### Invariant Statement

Procedural macro implementations must reside in a crate that is itself a
procedural macro crate. A procedural macro crate cannot also expose regular
library items. Therefore, the macro implementations and the consumer-facing
API must be in separate crates.

This crate is the procedural macro crate. It must not expose utility types,
re-exports, or documentation intended for end users.

### Enforcement Mechanism

The Rust compiler enforces this at the crate level: a crate declared as
a procedural macro crate cannot simultaneously act as a library. Any attempt
to use this crate directly as a library dependency will fail at the point
where the macro attributes are applied. The `derive_tools` facade crate is the
correct direct dependency for consumers.

### Violation Consequences

Merging this crate into `derive_tools` would break compilation because
`derive_tools` is not a procedural macro crate. Attempting to make it one
would prevent it from exporting regular items.

Any feature that requires regular library items (types, functions, traits)
alongside the macros must be split: library items in `derive_tools`, macro
implementations here.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| feature | docs/feature/001_derive_macros.md | Behavioral description of the macros |
| api | docs/api/001_derive_api.md | Public interface declared by this crate |
| integration | docs/integration/002_derive_tools.md | derive_tools as the consumer-facing facade |

### Sources

- [../../spec.md](../../spec.md) — Design Rationale: Why Separate Crate section
