# tests

This directory contains all functional and integration tests for the willbe crate organized by domain.

## Organization

Tests follow domain-based organization mirroring the source code structure. The `inc/` subdirectory contains modular test files organized by functional area (action tests, command tests, entity tests, tool tests, publishing tests).

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Verify basic binary invocation and minimal functionality |
| `tests.rs` | Aggregate test modules from inc/ directory |
| `inc/` | Organize modular domain-specific test implementations |
| `asset/` | Store test fixtures and sample workspace structures |

## Test Domains

The `inc/` directory organizes tests by functional domain:
- `action_tests/` - Test action layer business logic
- `command/` - Test command layer CLI parsing and execution
- `entity/` - Test entity layer data structures
- `tool/` - Test tool layer utilities
- `publish/` - Test publishing workflows and bug reproducers

## Adding New Tests

Before adding a new test:
1. Determine the functional domain being tested
2. Locate corresponding subdirectory in `inc/`
3. Add test to appropriate domain module
4. Update this table if adding new top-level test files
