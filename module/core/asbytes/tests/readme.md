# tests

Functional tests for asbytes crate, organized by trait and testing methodology.

## Organization Principles

Tests are organized by domain (what's being tested) rather than methodology (how it's tested). The structure mirrors the crate's trait-based architecture:
- AsBytes trait tests
- IntoBytes trait tests
- Integration tests aggregating all test modules

## Test Execution

Run all tests:
```bash
cargo test --all-features
```

Run specific test module:
```bash
cargo test --test tests --all-features
```

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `tests.rs` | Aggregate all test modules into single test binary |
| `inc/` | Contain domain-specific test modules for trait implementations |
| `manual/` | Document manual testing procedures and verification steps |
