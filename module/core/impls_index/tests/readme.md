# Tests

Test suite for the `impls_index` crate.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Verify basic crate functionality for local and published builds |
| `experiment.rs` | Explore new features and macro variants experimentally |
| `tests.rs` | Aggregate all `inc/` module tests as the main entry point |
| `inc/mod.rs` | Aggregate and re-export all domain-specific test modules |
| `inc/impls1_test.rs` | Test basic indexing with the `impls1!` declarative macro |
| `inc/impls2_test.rs` | Test alternative indexing with `impls2!` and callback-based expansion |
| `inc/impls3_test.rs` | Test advanced indexing with the `impls3!` proc macro |
| `inc/impls_basic_test.rs` | Test fundamental indexing behavior across all macro variants |
| `inc/impls_optional_test.rs` | Test optional function indexing variants |
| `inc/index_test.rs` | Test the `index!` invocation macro with all syntax variants |
| `inc/tests_index_test.rs` | Test automatic test attribute injection via `tests_impls!` and `tests_index!` |
| `inc/func_test.rs` | Test function manipulation utilities — `fn_name!`, `fn_rename!`, `fns!`, `fns2!` |
