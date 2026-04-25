# inc

Test implementations organized by functional domain.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `mod.rs` | Declares all test submodules with feature-gated cfg attributes |
| `basic.rs` | Tests `#[clone_dyn]` on non-generic traits and all Send/Sync variants |
| `parametrized.rs` | Tests `#[clone_dyn]` on generic traits with type parameters and where clauses |
| `basic_manual.rs` | Tests manual `Clone` implementation for `Box<dyn Trait>` without macro |
| `example_corner_cases_test.rs` | Tests macro expansion edge cases from real-world examples |
| `only_test/` | Shared test helpers: `clone_into_box` and `clone` function tests |
