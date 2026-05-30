# components_tests — Per-Derive-Macro Test Implementations

One pair of files per derive macro: a hand-coded `_manual.rs` baseline and a derived `.rs` version using `include!` to share assertions from `only_test/`.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `component_assign.rs` | Assign derive — shared struct + include only_test assertions |
| `component_assign_manual.rs` | Assign derive — hand-coded baseline without derive |
| `component_assign_tuple.rs` | Assign derive for tuple structs — shared struct + assertions |
| `component_assign_tuple_manual.rs` | Assign derive for tuple structs — hand-coded baseline |
| `component_from.rs` | ComponentFrom derive — shared struct + assertions |
| `component_from_manual.rs` | ComponentFrom derive — hand-coded baseline |
| `component_from_tuple.rs` | ComponentFrom derive for tuple structs — shared struct + assertions |
| `component_from_tuple_manual.rs` | ComponentFrom derive for tuple structs — hand-coded baseline |
| `components_assign.rs` | ComponentsAssign derive — shared struct + assertions |
| `components_assign_manual.rs` | ComponentsAssign derive — hand-coded baseline |
| `components_assign_tuple.rs` | ComponentsAssign derive for tuple structs — shared struct + assertions |
| `components_assign_tuple_manual.rs` | ComponentsAssign derive for tuple structs — hand-coded baseline |
| `from_components.rs` | FromComponents derive — shared struct + assertions |
| `from_components_manual.rs` | FromComponents derive — hand-coded baseline |
| `from_components_tuple.rs` | FromComponents derive for tuple structs — shared struct + assertions |
| `from_components_tuple_manual.rs` | FromComponents derive for tuple structs — hand-coded baseline |
| `composite.rs` | All five derives combined on one struct |
| `composite_manual.rs` | All five derives combined — hand-coded baseline |
| `options_fixture.rs` | Shared Options1/Options2 type definitions for manual baseline files |
| [only_test/](only_test/readme.md) | Shared test assertion functions included by both derived and manual files |
| [compiletime/](compiletime/readme.md) | Trybuild compile-time fixtures |
