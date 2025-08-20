# Task 017: Write Tests for Cargo.toml Configuration

## Overview
Write failing tests to verify SmokeModuleTest can configure temporary project dependencies for local/published versions (FR-5).

## Specification Reference
**FR-5:** The smoke testing utility must be able to configure the temporary project's `Cargo.toml` to depend on either a local, path-based version of a crate or a published, version-based version from a registry.

## Acceptance Criteria
- [ ] Write failing test that verifies local path dependency configuration in Cargo.toml
- [ ] Write failing test that verifies published version dependency configuration in Cargo.toml
- [ ] Write failing test that verifies proper Cargo.toml file generation
- [ ] Write failing test that verifies dependency clause formatting for different platforms
- [ ] Write failing test that verifies version string handling
- [ ] Write failing test that verifies path escaping for local dependencies
- [ ] Tests should initially fail to demonstrate TDD Red phase
- [ ] Tests should be organized in tests/cargo_toml_config.rs module

## Test Structure
```rust
#[test]
fn test_local_path_dependency_configuration() {
    // Should fail initially - implementation in task 018
    // Verify local path dependencies are properly configured in Cargo.toml
}

#[test]
fn test_published_version_dependency_configuration() {
    // Should fail initially - implementation in task 018
    // Verify published version dependencies are properly configured
}

#[test]
fn test_cargo_toml_generation() {
    // Should fail initially - implementation in task 018
    // Verify complete Cargo.toml file is properly generated
}

#[test]
fn test_cross_platform_path_handling() {
    // Should fail initially - implementation in task 018
    // Verify path escaping works correctly on Windows and Unix
}
```

## Related Tasks
- **Previous:** Task 016 - Refactor SmokeModuleTest Implementation
- **Next:** Task 018 - Implement Cargo.toml Configuration
- **Context:** Part of implementing specification requirement FR-5