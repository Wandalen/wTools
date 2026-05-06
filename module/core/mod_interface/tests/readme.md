# Tests

## Scope

Automated test suite for the `mod_interface` crate. Covers macro expansion correctness,
layer propagation semantics, namespace cascade rules, and compile-time error diagnostics.

#### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Basic importability check for the mod_interface! macro |
| `tests.rs` | Core propagation, layer composition, and exposure-level tests |
| `examples_test.rs` | Compile-and-run checks for the example programs |
| `inc/` | Shared test infrastructure and inline test modules |
| `docs/` | Specification coverage tests mapping doc instance claims to test cases |
