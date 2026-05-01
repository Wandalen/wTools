# tests

Test suite for the `for_each` crate.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Validate basic package integrity for local and published versions |
| `for_each_tests.rs` | Aggregate all functional test modules using the_module alias pattern |
| `example_output_quality_test.rs` | Validate example output correctness and documentation quality |
| `inc/` | Contain functional test modules for `for_each!`, `braces_unwrap!`, and `identity!` macros |
| `manual/` | Contain manual testing plans for edge cases requiring human verification |
