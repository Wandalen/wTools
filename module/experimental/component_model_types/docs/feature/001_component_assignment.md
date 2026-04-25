# Feature: Component Assignment

### Scope

- **Purpose**: Enable any struct to receive typed values for its fields through a uniform assignment interface, without requiring distinct setter methods per field.
- **Responsibility**: Documents the component assignment capability — its design, the roles of the four traits, and all related artifacts.
- **In Scope**: The Assign trait system, Option-aware assignment, explicit-type disambiguation, and the marker for standard library type support.
- **Out of Scope**: Derive macro implementation (→ `component_model_meta`); runtime aggregation (→ `component_model`).

### Design

Component assignment solves the problem of populating structs with multiple fields of potentially the same type using a single generic interface. Instead of distinct `set_age()`, `set_name()`, and `set_duration()` methods, a single `assign()` call dispatches to the correct field based on the value's type.

The design uses two type parameters to separate the component identity (`T` — which field) from the input flexibility (`IntoT` — what value can be assigned). This allows a single implementation to accept any value convertible to the target type, while the compiler uses the two parameters to route the assignment to the right field.

The feature provides four tightly related building blocks:

| Trait | Role |
|-------|------|
| Assign | Core assignment — mutating and consuming variants |
| OptionExt | Assignment into optional fields — creates or updates |
| AssignWithType | Explicit-type disambiguation when multiple Assign impls overlap |
| PopularType | Marker for standard library types needing macro-generated impls |

Builder chaining is supported through the consuming `impute()` method, which returns the modified receiver. This enables fluent initialization patterns without a separate builder type.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/component.rs` | Assign, OptionExt, AssignWithType trait definitions |
| source | `src/popular_types/std_types.rs` | PopularType marker trait |
| doc | [api/001_assign_trait.md](../api/001_assign_trait.md) | Full API reference for all four traits |
| doc | [invariant/001_orphan_rule.md](../invariant/001_orphan_rule.md) | Why standard library impls are macro-generated |
