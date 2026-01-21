# impls_index_meta Tests

Test suite for the impls_index_meta crate, the procedural macro companion providing the impls3 advanced indexing macro.

## Organization

Tests for this proc macro crate are minimal, focusing on basic macro functionality verification.

### Test Files

| File | Responsibility |
|------|----------------|
| smoke_test.rs | Verify impls3 procedural macro compiles and expands correctly |

## Test Coverage

### Core Functionality
- impls3 macro basic expansion with simple functions
- Procedural macro compilation and code generation

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
