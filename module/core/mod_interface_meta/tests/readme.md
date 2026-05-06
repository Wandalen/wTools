# Tests

Test suite for `mod_interface_meta`.

#### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Basic compile-time importability check |
| `integration_test.rs` | Four-layer namespace integration tests |
| `propagation_bug_test.rs` | Cascade propagation correctness across all four layers |
| `corner_cases_test.rs` | Corner case coverage for all DSL directive forms |
| `manual/` | Manual testing plan and session records |
| `inc/` | Shared test infrastructure |
| `mm_01_04_micro_modules_all_layers/` | Micro-module filesystem fixtures for directive tests |
| `mm_07_multiple_micro_modules_same_layer/` | Micro-module fixture for same-layer multi-module test |
