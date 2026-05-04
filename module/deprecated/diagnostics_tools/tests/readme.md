# Tests

### Responsibility Table

| File | Responsibility |
|------|----------------|
| [all_tests.rs](all_tests.rs) | Top-level test aggregator; includes all test modules |
| [runtime_assertion_tests.rs](runtime_assertion_tests.rs) | Runtime assertion macro integration tests |
| [smoke_test.rs](smoke_test.rs) | Basic build and feature-flag smoke validation |
| [trybuild.rs](trybuild.rs) | Compile-fail tests for assertion macro error messages |
| [example_002_quality_test.rs](example_002_quality_test.rs) | Quality validation for example 002 output |
| [inc/](inc/readme.md) | Shared test modules included by aggregators |
