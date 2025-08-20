# Write Tests for Standalone Build Mode

## Description
Write failing tests to verify standalone_build mode removes circular dependencies for foundational modules (US-4)

## Acceptance Criteria
- [ ] Tests verify standalone_build feature disables normal Cargo dependencies
- [ ] Tests verify #[path] attributes work for direct source inclusion
- [ ] Tests verify circular dependency resolution
- [ ] Tests verify foundational modules can use test_tools
- [ ] Tests verify behavior equivalence between normal and standalone builds
- [ ] Tests initially fail, demonstrating missing standalone build functionality
- [ ] Tests follow TDD red-green-refactor cycle principles

## Status
📋 Ready for implementation

## Effort
4 hours

## Dependencies
None - this is the first step in the TDD cycle for standalone build mode