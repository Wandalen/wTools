# Feature: Derive Macros

### Scope

- Describes the derive macro collection provided by this crate.
- Covers the categories of boilerplate that the macros eliminate.
- Audience: Rust developers evaluating whether to depend on `derive_tools`.
- Does not specify individual macro behavior — see `api/001_derive_api.md`.

### Design

The crate provides a set of derive macros that auto-generate trait implementations
for structs, eliminating repetitive boilerplate code. Each macro targets a specific
standard trait or newtype pattern.

The collection covers five behavioral areas:

**Delegation** — macros that delegate behavior to an inner field, allowing a
wrapper type to transparently expose the interface of its contents.

**Conversion** — macros that generate type conversion implementations, enabling
ergonomic value transformations between wrapper types and their inner types.

**Construction** — macros that generate constructor functions, providing a
consistent and concise way to instantiate structs with named or positional fields.

**Indexing** — macros that delegate element-access behavior to an inner collection
field, allowing a wrapper to be used where indexed access is expected.

**Logical operations** — macros that generate implementations for logical negation
and related operations on types that wrap boolean-like values.

All macros operate at compile time through procedural macro expansion. They
inspect the struct definition and generate the corresponding trait implementation
source code. No runtime overhead is introduced.

Feature flags allow selective compilation so that only the needed macros are
included in the build, minimizing compile time.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| api | docs/api/001_derive_api.md | Public interface of each derive macro |
| invariant | docs/invariant/001_proc_macro_separation.md | Why this crate is separate from derive_tools |
| invariant | docs/invariant/002_selective_compilation.md | Feature-flag selective compilation contract |
| integration | docs/integration/002_derive_tools.md | derive_tools as the consumer facade |

### Sources

- [../../spec.md](../../spec.md) — Overview, Scope, and Public API sections
