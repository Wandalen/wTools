# API: Workspace Derives

### Scope

- **Purpose**: Document the derive macros re-exported from workspace-internal crates.
- **Responsibility**: Reference for macros from `derive_tools_meta`, `variadic_from`, and `clone_dyn`.
- **In Scope**: Delegation, conversion, construction, indexing, logical negation, variadic, and trait-object-cloning macros.
- **Out of Scope**: External package derives — see `api/002_external_derives.md`.

### Abstract

The workspace-internal derive macros cover patterns specific to the workspace's design
conventions: newtype wrappers, delegation, conversion, and construction. Each macro is
activated by a dedicated feature flag and is not compiled when that flag is absent.

### Operations

**Delegation macros** — generate transparent access to an inner field. Two variants:
immutable access only, and immutable plus mutable access.

**Reference conversion macros** — generate shared reference conversions from a newtype
to its inner type. Two variants: shared reference only, and shared plus exclusive.

**Value conversion macros** — generate value-level conversions. One variant wraps an
inner value into the newtype; another extracts the inner value from the newtype.

**Constructor macro** — generates a constructor function accepting all fields as
positional arguments, for both named-field and positional-field structs.

**Indexing macros** — generate element-access delegation to an inner collection field.
Two variants: read-only indexed access, and read-write indexed access.

**Logical negation macro** — generates the logical negation operation for types
that wrap a boolean-like value.

**Variadic conversion macro** — generates a family of conversion implementations for
different argument counts, enabling multi-argument construction patterns.

**Trait object cloning macro** — generates the support necessary to clone a value
through a trait object, compatible with no-std environments.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../feature/001_aggregate_facade.md` | Aggregate facade context for these macros |
| doc | `002_external_derives.md` | External derives available in the same facade |
| doc | `../integration/001_derive_tools_meta.md` | Crate that implements these macros |

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Public API: Workspace Derives section; spec.md has been deleted — Sources entry retained as migration record. |
