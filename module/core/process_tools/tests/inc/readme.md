# Test Modules

Automated test modules imported via the inc aggregator pattern.

## File Responsibility Table

| Entry | Responsibility |
|-------|---------------|
| mod.rs | Aggregator re-exporting all test submodules |
| basic.rs | Crate public API smoke test |
| process_run.rs | Stream joining and binary execution tests |
| environment_is_cicd.rs | CI/CD environment detection tests |
