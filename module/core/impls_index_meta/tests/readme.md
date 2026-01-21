# impls_index_meta Tests

Test suite for the impls_index_meta crate, the procedural macro companion providing the impls3 advanced indexing macro.

## Organization

Tests for this proc macro crate are minimal, focusing on basic macro functionality verification.

### Test Files

| File | Responsibility |
|------|----------------|
| smoke_test.rs | Verify impls3 procedural macro compiles and expands correctly |
| corner_cases_test.rs | Comprehensive corner case testing for all function variants and edge cases |

## Test Coverage

### Core Functionality (smoke_test.rs)
- impls3 macro basic expansion with simple functions
- Procedural macro compilation and code generation
- Optional functions with `?` prefix
- Multiple functions in single block
- Generic functions

### Advanced Corner Cases (corner_cases_test.rs)
- Function variants (lifetimes, where clauses, async, const, unsafe)
- Attribute handling (inline, doc comments, cfg, multiple attributes)
- Edge cases (empty block, mixed optional/required, complex types, impl Trait, default type params)
- Integration scenarios (multiple mixed features)

## Test Execution

Run tests using standard Rust test commands:

```bash
# All tests
cargo nextest run --all-features

# Doc tests
cargo test --doc --all-features
```

## Notes

- This is a proc macro crate (proc-macro = true in Cargo.toml)
- Comprehensive functional tests are in parent impls_index crate
- This crate focuses on macro implementation, not behavior testing
- Main crate (impls_index) tests the full public API including impls3
