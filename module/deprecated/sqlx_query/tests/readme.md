# Tests

### Scope

Test suite for `sqlx_query` crate.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Verify crate imports and compilation in all configurations |
| `macro_expansion_test.rs` | Verify `query!` and `query_as!` macro syntax expansion |
| `feature_flag_test.rs` | Verify feature flag detection and dispatch mode selection |
| `readme_example_test.rs` | Verify readme example syntax compiles as expected |
