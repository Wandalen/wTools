# Test Organization

## Scope

This directory contains tests for the deterministic_rand crate, organized by test domain.

## Test Domains

The test suite is organized into three domains:

1. **Smoke Tests** (`smoke_test.rs`) - Quick validation tests
2. **Assumption Tests** (`assumption_test.rs`) - RNG behavior assumptions
3. **Integration Tests** (`basic_test.rs`) - End-to-end determinism validation

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Basic smoke tests for quick validation |
| `assumption_test.rs` | Validate RNG behavior assumptions (streams, sampling) |
| `basic_test.rs` | Parallel Monte Carlo tests proving determinism |
| `seed_import_non_deterministic_bug_test.rs` | Reproduce and verify fix for Seed import in non-deterministic mode |

## Test Coverage

All tests verify deterministic behavior when the `determinism` feature is enabled.
