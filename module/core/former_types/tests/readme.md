# Tests

### Responsibility Table

| File | Responsibility |
|------|----------------|
| tests.rs | Test aggregator — wires inc/ modules into test binary |
| smoke_test.rs | Smoke tests for local and published crate runs |
| inc/ | Domain test modules — lifetime correctness and bug reproducers |
| inc/mod.rs | Aggregates all test submodules under inc/ |
| inc/lifetime_mre_test.rs | Bug reproducer for E0726 — FormerBegin lifetime parameter fix |
