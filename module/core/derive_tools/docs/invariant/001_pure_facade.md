# Invariant: Pure Facade

### Scope

- **Purpose**: Define the constraint that `derive_tools` must not implement any derive macros itself.
- **Responsibility**: Document the pure-facade invariant — rationale, enforcement, and violation consequences.
- **In Scope**: Delegation rationale, Cargo.toml enforcement, and violation consequences.
- **Out of Scope**: Which macros are available — see `api/` instances.

### Invariant Statement

`derive_tools` is a facade crate. It must not contain any procedural macro implementations.
All derive macros it exposes must be re-exported from external crates or workspace companions.

This crate is not a proc-macro crate. Adding procedural macro attributes or derive
implementations here would require declaring it as `proc-macro = true` in `Cargo.toml`,
which would prevent it from also exporting regular library items (types, re-exports, modules).

Custom derive implementations must live in `derive_tools_meta` or equivalent proc-macro crates.

### Enforcement Mechanism

The `Cargo.toml` of this crate does not declare `lib.proc-macro = true`. Any attempt to
add procedural macro code will fail compilation with a link error at the macro site.

Code reviews must reject changes that add `#[proc_macro_derive]` or `#[proc_macro_attribute]`
attributes to this crate's source.

### Violation Consequences

Declaring this crate as a proc-macro crate would prevent it from re-exporting regular items
and types. The namespace organization (own/orphan/exposed/prelude modules) and the dependency
namespace would all become unavailable, breaking existing consumers.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../feature/001_aggregate_facade.md` | Behavioral description of the aggregate facade |
| doc | `../integration/001_derive_tools_meta.md` | The correct location for custom derive implementations |

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Design Rationale: Why No Custom Implementations and Out-of-Scope sections; spec.md has been deleted — Sources entry retained as migration record. |
