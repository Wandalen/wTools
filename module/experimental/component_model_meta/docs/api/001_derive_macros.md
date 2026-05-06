# API: Derive Macros

### Scope

- **Purpose**: Document the five derive macros that generate component model trait implementations for structs.
- **Responsibility**: Provides the canonical usage reference for each derive macro — what it generates, what inputs it accepts, and which feature gates it.
- **In Scope**: ComponentModel, Assign, ComponentsAssign, ComponentFrom, and FromComponents — their generated output, supported struct shapes, and feature flag requirements.
- **Out of Scope**: Internal code generation logic (→ `algorithm/001_popular_type_generation.md`); trait definitions (→ `component_model_types/docs/api/` — to create).

### Abstract

Five derive macros that eliminate boilerplate for component-based struct patterns. Each macro generates one or more trait implementations — primarily Assign impls and constructor methods — based on the fields declared in the target struct. The macros operate on named structs; tuple and unit struct support varies per macro.

### Operations

**ComponentModel** — unified derive combining all functionality:

| Generated output | Description |
|-----------------|-------------|
| Field-type setter method | Mutating setter per unique field type |
| Consuming field-type setter | Consuming setter returning self for builder chaining |
| Popular type impls | Assign implementations for recognized standard library types appearing as field types |

Deduplicates fields by type — only one setter is generated per distinct field type. This is the recommended derive for most use cases.

**Assign** — targeted Assign trait implementation:

| Supported shapes | Description |
|-----------------|-------------|
| Named structs | One Assign impl per field; deduplicates by type |
| Tuple structs | One Assign impl per distinct positional type |
| Unit structs | No-op (no fields, no impls generated) |

**ComponentsAssign** — bulk assignment from a compatible source:

Generates a struct-specific assign trait with a blanket impl requiring that each field type can be assigned from the generic source type. The generated method accepts a single source value and copies all typed components from it in one call.

**ComponentFrom** — extraction into component types:

Generates an extraction conversion for each unique field type — converts a struct reference into a component value by cloning. Deduplicates by field type to avoid conflicting impls.

**FromComponents** — construction from a compatible source:

Generates a construction conversion from any single compatible source type — the source must be convertible into each of the struct's field types.

### Error Handling

All macros fail at compile time with diagnostic messages when applied to unsupported shapes (e.g., enums, types with unsupported generic bounds). No runtime errors are possible — code generation is compile-time only.

### Compatibility Guarantees

- These macros are proc-macros; they should not be used directly. Use the `component_model` crate which re-exports them alongside the necessary trait definitions.
- The generated setter method names are derived from field names; renaming a field is a breaking change for callers using these generated methods.

#### Feature Flags

| Feature flag | Effect |
|-------------|--------|
| `enabled` | Activates the crate and its dependencies |
| `full` | Enables all derive macros |
| `derive_component_model` | Enables the ComponentModel derive |
| `derive_components` | Enables all four targeted derives as a group |
| `derive_component_assign` | Enables the Assign derive |
| `derive_components_assign` | Enables the ComponentsAssign derive (requires Assign) |
| `derive_component_from` | Enables the ComponentFrom derive |
| `derive_from_components` | Enables the FromComponents derive |

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Proc-macro entry points for all five derives |
| source | `src/component/component_model.rs` | ComponentModel unified derive implementation |
| source | `src/component/component_assign.rs` | Assign derive implementation |
| source | `src/component/component_from.rs` | ComponentFrom derive implementation |
| source | `src/component/components_assign.rs` | ComponentsAssign derive implementation |
| source | `src/component/from_components.rs` | FromComponents derive implementation |
| test | `tests/smoke_test.rs` | Integration coverage across all five derive macros |
| test | `tests/manual_examples_comprehensive.rs` | Comprehensive usage examples for all macros |
| test | `tests/assign_duplicate_types_bug.rs` | Duplicate-type deduplication in Assign |
| test | `tests/component_from_duplicate_types_bug.rs` | Duplicate-type deduplication in ComponentFrom |
| test | `tests/component_from_generic_types_bug.rs` | Generic-type handling in ComponentFrom |
| test | `tests/component_model_tuple_struct_limitation.rs` | Tuple struct shape limitation |
| doc | [algorithm/001_popular_type_generation.md](../algorithm/001_popular_type_generation.md) | How popular type Assign impls are generated |
| doc | [algorithm/002_type_deduplication.md](../algorithm/002_type_deduplication.md) | How same-type field deduplication prevents conflicting impls |
| doc | `component_model_types/docs/api/001_assign_trait.md` | The Assign trait these macros implement (to create) |
