# tests/

Tests for `meta_tools`.

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Verify basic local and published package availability |
| `meta_tools_tests.rs` | Main integration harness; pulls in cross-crate `inc/` suites |
| `corner_cases_comprehensive.rs` | Automated corner-case tests for `for_each!` and `meta_idents_concat!` |
| `inc/` | Test include modules cross-linked from constituent crate test suites |
| `manual/` | Manual testing plan for all macro scenarios and feature combinations |
