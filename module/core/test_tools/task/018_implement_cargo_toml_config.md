# Task 018: Implement Cargo.toml Configuration

## Overview
Implement ability for SmokeModuleTest to configure temporary project Cargo.toml for local/published dependencies (FR-5).

## Specification Reference
**FR-5:** The smoke testing utility must be able to configure the temporary project's `Cargo.toml` to depend on either a local, path-based version of a crate or a published, version-based version from a registry.

## Acceptance Criteria
- [ ] Implement local path dependency configuration in Cargo.toml generation
- [ ] Implement published version dependency configuration in Cargo.toml generation
- [ ] Enhance Cargo.toml file generation with proper formatting
- [ ] Implement cross-platform path handling (Windows vs Unix)
- [ ] Add proper version string validation and handling
- [ ] Implement path escaping for local dependencies
- [ ] All Cargo.toml configuration tests from task 017 must pass
- [ ] Maintain backward compatibility with existing functionality

## Implementation Notes
- This task implements the GREEN phase of TDD - making the failing tests from task 017 pass
- Build upon existing Cargo.toml generation in form() method (lines 145-162 in current implementation)
- Enhance platform-specific path handling (lines 133-144)
- Focus on improving configuration flexibility and reliability

## Technical Approach
1. **Enhance Dependency Configuration**
   - Improve local_path_clause handling for better cross-platform support
   - Add validation for version strings and path formats
   - Implement proper dependency clause formatting

2. **Improve Cargo.toml Generation**
   - Enhance template generation for better compatibility
   - Add proper metadata fields (edition, name, version)
   - Implement configurable dependency sections

3. **Cross-Platform Support**
   - Improve Windows path escaping (lines 134-138)
   - Ensure Unix path handling works correctly
   - Add platform-specific validation

## Success Metrics
- All Cargo.toml configuration tests pass
- Local and published dependencies are properly configured
- Cross-platform path handling works correctly
- Generated Cargo.toml files are valid and functional
- Integration with existing smoke test workflow is seamless

## Related Tasks
- **Previous:** Task 017 - Write Tests for Cargo.toml Configuration
- **Next:** Task 019 - Refactor Cargo.toml Configuration Logic
- **Context:** Core implementation of specification requirement FR-5