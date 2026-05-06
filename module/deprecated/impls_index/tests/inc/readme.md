# inc

Domain-specific test modules for the `impls_index` crate.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `mod.rs` | Aggregate and re-export all domain-specific test modules |
| `impls1_test.rs` | Test basic indexing with the `impls1!` declarative macro |
| `impls2_test.rs` | Test alternative indexing with `impls2!` and callback-based expansion |
| `impls3_test.rs` | Test advanced indexing with the `impls3!` proc macro |
| `impls_basic_test.rs` | Test fundamental indexing behavior across all macro variants |
| `impls_optional_test.rs` | Test optional function indexing variants |
| `index_test.rs` | Test the `index!` invocation macro with all syntax variants |
| `tests_index_test.rs` | Test automatic test attribute injection via `tests_impls!` and `tests_index!` |
| `func_test.rs` | Test function manipulation utilities — `fn_name!`, `fn_rename!`, `fns!`, `fns2!` |
