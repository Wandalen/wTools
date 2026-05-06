# impls_index_meta Tests

Test suite for the impls_index_meta crate, the procedural macro companion providing the impls3 advanced indexing macro.

## Organization

Tests for this proc macro crate are minimal, focusing on basic macro functionality verification.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Verify impls3 procedural macro compiles and expands correctly |
| `corner_cases_test.rs` | Comprehensive corner case testing for all function variants and edge cases |
| `manual/` | Manual testing plan for impls3 macro |

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
