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

### Error Handling

All macros in this group fail at compile time; no runtime errors are produced.

**Structural mismatch** — delegation and indexing macros require a struct with exactly one named inner field. Applying them to a type with zero or multiple fields, or to an enum, produces a derive error at the attribute site.

**Feature gate absent** — each macro is compiled only when its dedicated feature flag is active. Using a macro without the corresponding flag causes a missing-item compile error. Activate the required feature in `Cargo.toml` to resolve.

**Trait prerequisite unmet** — reference conversion macros generate trait implementations that depend on traits of the inner type. If those bounds are not satisfied, a trait-bound error appears at the use site.

### Compatibility Guarantees

Macro names and their attribute forms are stable across patch and minor versions. Changes to generated code behavior require a major version bump.

Feature flag names are stable within a major version. A flag valid in one minor version remains valid in all subsequent minor versions of the same major version.

Macros require the Rust 2021 edition. The minimum supported Rust version follows the workspace MSRV.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../feature/001_aggregate_facade.md` | Aggregate facade context for these macros |
| doc | `002_external_derives.md` | External derives available in the same facade |
| doc | `../integration/001_derive_tools_meta.md` | Crate that implements these macros |
