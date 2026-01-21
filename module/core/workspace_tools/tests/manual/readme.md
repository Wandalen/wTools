# Manual Testing Plan for workspace_tools

This directory contains manual validation tests that verify workspace_tools functionality in realistic scenarios beyond automated test coverage.

## Purpose

Manual tests complement automated tests by:

- Validating end-to-end workflows in real development environments
- Testing integration with external tools (git, cargo) in actual workspaces
- Verifying error messages are clear and actionable for human users
- Catching UX issues that automated tests might miss
- Validating cross-platform behavior in actual OS environments

## Manual Test Files

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `manual_testing_task_021.rs` | Manual validation of Task 021 enhanced secrets API in realistic scenarios |

## Manual Testing Checklist

### Prerequisites

Before running manual tests:

- [ ] Clean git workspace (no uncommitted changes)
- [ ] All feature flags enabled (`--all-features`)
- [ ] Run on target platforms (Linux, macOS, Windows if available)
- [ ] Environment variables cleared (`unset WORKSPACE_PATH CARGO_WORKSPACE_DIR`)

### Core Functionality Manual Validation

**M1: Workspace Resolution in Real Projects**

1. Navigate to an actual cargo workspace (e.g., wTools)
2. Create test executable that calls `workspace()`
3. Verify workspace root correctly identified
4. Expected: Resolves to git root containing Cargo.toml

**M2: Secret Loading from Real Directories**

1. Create `secret/` directory in workspace root
2. Add real secret file `-test_secret.sh` with `TEST_KEY=test_value`
3. Use `load_secret("test_secret")` API
4. Verify secret loads correctly
5. Expected: Loads without file extension, finds hyphen-prefixed file

**M3: Error Messages for Missing Secrets**

1. Attempt to load non-existent secret `load_secret("nonexistent")`
2. Read error message output
3. Verify it shows:
   - All paths searched (workspace/secret/, workspace/.secrets/, etc.)
   - Available secret files in those directories
   - Actionable guidance on fixing the issue
4. Expected: Clear, helpful error message guiding user to solution

**M4: Cross-Platform Path Handling**

*Run on Windows if available*

1. Create workspace on Windows with paths using backslashes
2. Call workspace path operations
3. Verify paths normalized correctly
4. Expected: Windows paths work seamlessly, converted to canonical form

**M5: Installed Application Resolution (Task 022)**

1. Install a cargo binary with `cargo install --path .`
2. Run installed binary from different directory
3. Binary should use workspace resolution (via fallback strategies)
4. Expected: Resolves workspace even without WORKSPACE_PATH env var

### API Usability Manual Validation

**M6: Developer Experience - Secret API**

Scenario: New developer trying to load secrets

1. Read API documentation
2. Attempt to load secret using docs as only guide
3. Note any confusion points or unclear instructions
4. Expected: API intuitive enough to use without reading source code

**M7: Error Recovery - Configuration Validation**

1. Create intentionally malformed config file (invalid TOML)
2. Attempt to load with `load_config()`
3. Verify error message pinpoints issue location
4. Fix error based on error message alone
5. Expected: Error message sufficient to fix problem without debugging

**M8: Feature Combinations**

Test feature flag combinations in real usage:

1. Build with `--features secrets`
2. Build with `--features validation`
3. Build with `--features secrets,validation`
4. Build with `--all-features`
5. Expected: All combinations compile and work correctly

### Performance and Scale Manual Validation

**M9: Large Workspace Performance**

1. Test in large workspace (e.g., wTools with 100+ crates)
2. Measure workspace resolution time
3. Verify performance acceptable (<100ms for first call)
4. Expected: No noticeable delay in CLI tool startup

**M10: Secret Directory Scan Performance**

1. Create secret/ directory with 100+ secret files
2. Request non-existent secret (triggers directory scan for error message)
3. Verify error response time reasonable (<500ms)
4. Expected: Directory scanning doesn't cause UI lag

### Edge Case Manual Validation

**M11: Symlink Handling**

*If platform supports symlinks*

1. Create symlink to workspace directory
2. Navigate into workspace via symlink
3. Call workspace resolution functions
4. Expected: Resolves correctly, canonicalizes paths

**M12: Nested Git Repositories**

1. Create git repository inside another git repository (submodule scenario)
2. Test workspace resolution from inner repository
3. Verify correct workspace root selected
4. Expected: Finds outer workspace or handles gracefully with clear error

**M13: Permission Denied Scenarios**

1. Create secret file with no read permissions (`chmod 000 secret/-test.sh`)
2. Attempt to load secret
3. Verify error message indicates permission problem
4. Expected: Clear permission error, not generic "file not found"

## Manual Testing Execution

### Running Manual Tests

```bash
# Navigate to workspace_tools crate
cd module/core/workspace_tools

# Run all manual tests with all features
cargo test --test manual_testing_task_021 --all-features -- --nocapture

# Run specific manual test
cargo test --test manual_testing_task_021 test_manual_enhanced_error_handling --all-features -- --nocapture
```

### Recording Manual Test Results

After each manual testing session, document results:

```markdown
## Manual Test Session - [DATE]

**Tester**: [Name]
**Platform**: [OS + Version]
**Rust Version**: [rustc --version]

### Results

| Test ID | Status | Notes |
|---------|--------|-------|
| M1      | ✅ PASS | Workspace resolved correctly in wTools |
| M2      | ✅ PASS | Secret loaded from hyphen-prefixed file |
| M3      | ⚠️ ISSUE | Error message could be more concise |
| ...     | ...    | ... |

### Issues Found

1. **Issue**: [Description]
   - **Severity**: Critical/Major/Minor
   - **Reproduction**: [Steps]
   - **Expected**: [What should happen]
   - **Actual**: [What actually happened]
```

## Reporting Manual Test Failures

If manual testing reveals issues not caught by automated tests:

1. **Create Bug Reproducer Test**: Add automated test in `tests/` directory reproducing the issue
2. **File Issue**: Create issue with `manual-test-failure` label
3. **Update Manual Test**: Mark manual test as failing until fix verified
4. **Document Fix**: Once fixed, update manual test notes with resolution

## Maintenance

This manual testing plan should be updated when:

- New major features added
- API changes affect user workflows
- Bug reports reveal gaps in manual coverage
- Platform-specific issues discovered

**Last Updated**: 2026-01-10
**Next Review**: On next major feature release
