# tests/

This directory contains all functional tests for the `claude_runner_core` crate, which provides builder pattern API for executing Claude Code commands programmatically with zero process execution logic.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `builder_edge_cases_test.rs` | Test builder pattern edge cases |
| `builder_methods_test.rs` | Test builder method existence and chainability |
| `default_values_test.rs` | Test default value correctness |
| `environment_variables_test.rs` | Test environment variable setting |
| `migration_validation_test.rs` | Test migration completeness |
| `old_way_impossible_verification.rs` | Test deprecated pattern prevention |
| `responsibility_single_execution_point_test.rs` | Test single execution point principle |
| `types_test.rs` | Test type definitions and conversions |
| `verification_impossibility_test.rs` | Test impossibility layer verification (VER-3) |
| `verification_migration_metrics_test.rs` | Test migration metrics verification (VER-2) |
| `verification_negative_criteria_test.rs` | Test negative criteria verification (VER-6) |
| `verification_rollback_test.rs` | Test rollback impossibility verification (VER-4) |
| `verification_shortcuts_test.rs` | Test shortcuts detection verification (VER-5) |
| `manual_edge_case_check.rs` | Test edge case manual verification |
| `manual_execution_test.rs` | Manual tests with real Claude binary |
| `float_edge_cases_test.rs` | Test float parameter edge cases (NaN, infinity, negative) |
| `verbose_and_path_edge_cases_test.rs` | Test verbose flag and path parameter edge cases |
| `describe_test.rs` | Test describe() and describe_env() command inspection methods |
| `execution_output_test.rs` | Test ExecutionOutput struct fields and Display formatting |
| `skip_permissions_test.rs` | Test skip_permissions flag and --dangerously-skip-permissions arg |

## Organization (20 test files)

Tests organized by functional domain and architectural principles (see Responsibility Table above).

### Scope

This test suite covers the claude_runner_core crate's builder pattern API for Claude Code command construction and comprehensive verification framework (20 test files):

**In Scope:**
- Builder pattern API (4 test files):
  - Edge cases: token limits (0, 1, 200K, u32::MAX), method overrides (last wins), argument accumulation
  - Methods: 22+ with_*() methods, chainability, order independence
  - Defaults: tier 1 (bash_timeout=3.6M, auto_continue=true, telemetry=false, max_output_tokens=200K), tier 2/3 (None)
  - Environment variables: each parameter sets correct env var, tier 1 defaults set vars
- Type definitions (1 test file):
  - ActionMode enum (Ask/Allow/Deny) with string conversions
  - LogLevel enum (Error/Warn/Info/Debug/Trace) with ordering and conversions
- Migration validation (2 test files):
  - Factory pattern completely removed, builder pattern universally adopted
  - Old patterns impossible (from_message/create/generate dont exist)
  - Single execution point enforcement
- Verification framework - 6-layer pyramid (5 test files, 231 validations total):
  - Layer 1 - Migration metrics (42 checks): 0%→100% shift in 8 metrics
  - Layer 2 - Rollback detection (27 checks): migration irreversible
  - Layer 3 - Impossibility (34 checks): old API wont compile
  - Layer 4 - Shortcuts detection (48 checks): no mocks/fakes/disabled tests
  - Layer 5 - Negative criteria (15 checks): forbidden patterns = 0
  - Layer 6 - Positive tests (65 tests): new pattern works
- Bug reproducers with 5-section documentation:
  - issue-token-limit-default: Migration from factory pattern didnt preserve 200K default
  - Root Cause, Why Not Caught Initially, Fix Applied, Prevention, Pitfall documented
- Test organization: Test Matrix cataloging, Lessons Learned documentation, Cross-references in Out of Scope

**Out of Scope:**
- Session lifecycle management (→ claude_session crate)
- Context injection from wplan (→ dream_agent crate)
- Actual process execution (→ dream_agent crate)
- Interactive terminal UI (→ terminal-based tools)
- Configuration hierarchy (→ config_hierarchy crate)

**Test Quality**: Uses process::Command inspection via test-only `build_command_for_test()` method. All tests validate builder output without executing Claude binary. Token limit tests validate environment variables. Override tests validate last-wins semantics. Accumulation tests validate args collection. Bug reproducer tests include complete 5-section documentation following test_organization.rulebook.md standards.

## Navigation

Test files follow domain-based naming that reflects the functionality being tested. Bug reproducer tests use `_bug_` suffix with issue ID. Edge case tests use descriptive suffixes (`_edge_case`, `_minimum`, `_boundary`, `_stress`).
