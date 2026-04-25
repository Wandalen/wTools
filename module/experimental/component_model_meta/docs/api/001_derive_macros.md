# API: Derive Macros

### Scope

- **Purpose**: Document the five derive macros that generate component model trait implementations for structs.
- **Responsibility**: Provides the canonical usage reference for each derive macro — what it generates, what inputs it accepts, and which feature gates it.
- **In Scope**: ComponentModel, Assign, ComponentsAssign, ComponentFrom, and FromComponents — their generated output, supported struct shapes, and feature flag requirements.
- **Out of Scope**: Internal code generation logic (→ `algorithm/001_popular_type_generation.md`); trait definitions (→ `component_model_types/docs/api/001_assign_trait.md`).

### Abstract

Five derive macros that eliminate boilerplate for component-based struct patterns. Each macro generates one or more trait implementations — primarily Assign impls and constructor methods — based on the fields declared in the target struct. The macros operate on named structs; tuple and unit struct support varies per macro.

### Operations

**ComponentModel** — unified derive combining all functionality:

| Generated output | Description |
|-----------------|-------------|
| `{field}_set()` method | Mutating setter per unique field type |
| `{field}_with()` method | Consuming setter returning self (builder chaining) |
| Popular type impls | Assign implementations for recognized standard library types appearing as field types |

Deduplicates fields by type — only one setter is generated per distinct field type. This is the recommended derive for most use cases.

**Assign** — targeted Assign trait implementation:

| Supported shapes | Description |
|-----------------|-------------|
| Named structs | One Assign impl per field; deduplicates by type |
| Tuple structs | One Assign impl per distinct positional type |
| Unit structs | No-op (no fields, no impls generated) |

**ComponentsAssign** — bulk assignment from a tuple:

Generates a `components_assign()` method accepting a tuple of values, one per field in declaration order. Enables setting multiple fields in a single call.

**ComponentFrom** — construction from a single value:

Generates a constructor that creates the struct from a single component value. Used for single-field structs or when one field dominates construction.

**FromComponents** — construction from a tuple of values:

Generates a `from_components()` constructor accepting a tuple of values positionally matching the struct's fields.

### Error Handling

All macros fail at compile time with diagnostic messages when applied to unsupported shapes (e.g., enums, types with unsupported generic bounds). No runtime errors are possible — code generation is compile-time only.

### Compatibility Guarantees

- These macros are proc-macros; they should not be used directly. Use the `component_model` crate which re-exports them alongside the necessary trait definitions.
- The generated method names (`{field}_set`, `{field}_with`) are derived from field names; renaming a field is a breaking change for callers using these generated methods.
- Feature flag `derive_component_model` gates ComponentModel; individual flags gate the other four macros.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Proc-macro entry points for all five derives |
| source | `src/component/component_model.rs` | ComponentModel unified derive implementation |
| source | `src/component/component_assign.rs` | Assign derive implementation |
| doc | [algorithm/001_popular_type_generation.md](../algorithm/001_popular_type_generation.md) | How popular type Assign impls are generated |
| doc | `component_model_types/docs/api/001_assign_trait.md` | The Assign trait these macros implement |
