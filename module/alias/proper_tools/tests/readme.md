# Tests

Automated test suite for proper_tools crate.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `examples_exist.rs` | Verify examples directory structure and compilation |
| `metadata_correctness.rs` | Validate Cargo.toml metadata accuracy |
| `manual/` | Document manual testing procedures and test plans |

## Test Organization

- **Integration Tests**: All test files in this directory are integration tests
- **Test Coverage**: Metadata verification, example validation
- **Manual Testing**: Manual testing procedures documented in `manual/readme.md`
- **Documentation**: Each test file contains comprehensive documentation following 5-section bug reproducer format where applicable

## Running Tests

```bash
# Run all tests
cargo test --all-features

# Run specific test file
cargo test --test examples_exist --all-features
cargo test --test metadata_correctness --all-features

# Run with nextest
cargo nextest run --all-features
```
