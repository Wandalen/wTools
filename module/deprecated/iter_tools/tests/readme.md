# tests

Test suite for iter_tools crate.

## Organization

Tests are organized with smoke tests at root level and functional tests in the `inc/` subdirectory following the standard test aggregator pattern.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Validate basic package integrity for local and published versions |
| `tests.rs` | Aggregate all test modules using the_module alias pattern |
| `inc/` | Contain functional test modules for iterator extension traits |
| `manual/` | Contain manual testing plans and human-executed test cases |
