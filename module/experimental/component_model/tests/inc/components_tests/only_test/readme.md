# only_test — Shared Assertion Functions

Shared test assertion functions included via `include!()` by both the derived and manual test files in the parent directory. Each file contains one or more `#[test]` functions that operate on structs defined in the including file.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `component_assign.rs` | Assertions for Assign derive: assign and impute |
| `component_assign_tuple.rs` | Assertions for Assign derive on tuple structs |
| `component_from.rs` | Assertions for ComponentFrom derive |
| `component_from_tuple.rs` | Assertions for ComponentFrom derive on tuple structs |
| `components_assign.rs` | Assertions for ComponentsAssign derive |
| `components_assign_tuple.rs` | Assertions for ComponentsAssign derive on tuple structs |
| `from_components.rs` | Assertions for FromComponents derive |
| `from_components_tuple.rs` | Assertions for FromComponents derive on tuple structs |
| `composite.rs` | Assertions combining all five derives on one struct |
