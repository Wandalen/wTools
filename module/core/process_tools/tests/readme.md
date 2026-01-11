# Tests Directory

Test suite for process_tools crate providing comprehensive coverage of subprocess execution and CI/CD environment detection.

## File Responsibility Table

| File | Responsibility |
|------|---------------|
| `tests.rs` | Test suite entry point with feature gating |
| `smoke_test.rs` | Package health verification (local and published) |
| `inc/mod.rs` | Test module organization and feature gating |
| `inc/basic.rs` | Basic crate compilation and import test |
| `inc/environment_is_cicd.rs` | CI/CD environment detection tests |
| `inc/process_run.rs` | Process execution with stream joining tests |
| `tool/asset.rs` | Test asset path resolution and helper utilities |
| `asset/err_out_test/err_out_err.rs` | Test program for stderr→stdout→stderr output |
| `asset/err_out_test/out_err_out.rs` | Test program for stdout→stderr→stdout output |

## Test Organization

- **Smoke Tests:** `smoke_test.rs` - Basic package health checks
- **Integration Tests:** `inc/` directory - Domain-based test organization
- **Test Utilities:** `tool/` directory - Helper functions for test execution
- **Test Assets:** `asset/` directory - External programs for integration testing

## Known Technical Debt

- `environment_is_cicd.rs:3` - Need to rewrite tests with external application execution
- `process_run.rs:13-14` - Function should use test_tools/process_tools
- `tool/asset.rs:1` - Helper function should be incorporated into tool
- `tool/asset.rs:59-60` - Former interface improvements needed
- `environment_is_cicd.rs:11-85` - Commented-out environment variable tests (need rewriting per line 3 note)
