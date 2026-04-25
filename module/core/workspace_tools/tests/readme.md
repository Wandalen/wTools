# workspace_tools Tests

Comprehensive test suite for the workspace_tools crate, organized by functional domain.

## Test Organization

Tests are centralized in this directory following domain-based organization. Each test file has a unique responsibility focusing on specific functionality or scenarios.

### Scope

#### Responsibilities

Validates all workspace_tools functionality including workspace resolution, path operations, secret management, configuration loading, cross-platform compatibility, error handling, and feature combinations. Ensures correctness through comprehensive test matrices covering normal operation, edge cases, corner cases, and boundary conditions.

#### In Scope

- Core workspace resolution and path operations
- Secret management with multiple fallback strategies
- Configuration loading and validation
- Cross-platform compatibility (Windows, Linux, macOS)
- Error handling and recovery
- Feature combination testing
- Backward compatibility validation
- Integration with cargo and serde ecosystems

#### Out of Scope

- Performance benchmarking (belongs in benches/)
- Implementation details testing (black-box testing only)
- External tool testing (git, cargo commands tested only through integration)

### Responsibility Table

### Core Functionality Tests

| File | Responsibility |
|------|----------------|
| `workspace_tests.rs` | Core workspace creation, resolution, and path operations |
| `path_operations_comprehensive_tests.rs` | Path manipulation, normalization, and canonicalization |
| `path_normalization_tests.rs` | Workspace root path normalization (trailing components) |

### Secret Management Tests

| File | Responsibility |
|------|----------------|
| `centralized_secrets_test.rs` | Centralized secret management integration |
| `corner_cases_critical.rs` | Critical corner cases in secret fallback implementation |
| `corner_cases_parsing.rs` | Content parsing edge cases (format variations, quoting, escaping) |
| `enhanced_secret_parsing_tests.rs` | Enhanced parsing supporting export statements and dotenv format |
| `secret_directory_verification_test.rs` | Secret directory usage verification (secret/ not .secrets/) |
| `secrecy_integration_tests.rs` | Memory-safe secret handling using secrecy crate |
| `secrecy_optimization_tests.rs` | Advanced secrecy features (SecretInjectable, validation, performance) |
| `test_fallback_integration.rs` | Secret fallback functionality integration |
| `test_new_secrets_api_methods.rs` | New path-aware secret API methods (task 021) |
| `reproduce_secrets_api_ux_issue.rs` | Reproduction of reported secrets API UX issues |

### Configuration and Validation Tests

| File | Responsibility |
|------|----------------|
| `config_validation_tests.rs` | Schema-based configuration validation preventing runtime errors |
| `validation_boundary_tests.rs` | Input validation and boundary condition handling |

### Error Handling Tests

| File | Responsibility |
|------|----------------|
| `error_handling_comprehensive_tests.rs` | Error variant coverage, creation, display, and trait implementation |
| `edge_case_comprehensive_tests.rs` | Edge case scenarios across all functionality |

### Platform and Integration Tests

| File | Responsibility |
|------|----------------|
| `cross_platform_compatibility_tests.rs` | Platform-specific path handling (Windows, Linux, macOS) |
| `cargo_integration_tests.rs` | Integration with cargo workspace and metadata |
| `serde_integration_tests.rs` | Integration with serde for configuration deserialization |
| `testing_integration_examples.rs` | Testing utilities integration (moved from examples) |

### Feature Combination Tests

| File | Responsibility |
|------|----------------|
| `feature_combination_tests.rs` | Feature flag combination testing ensuring compatibility |

### Comprehensive Test Suites

| File | Responsibility |
|------|----------------|
| `comprehensive_test_suite.rs` | Complete coverage test matrix for all workspace_tools functionality |
| `rulebook_compliance_tests.rs` | Verification of compliance with project rulebook requirements |

### Task-Specific Tests (Feature Documentation)

| File | Responsibility |
|------|----------------|
| `task_021_comprehensive_tests.rs` | Task 021 acceptance criteria (enhanced secrets API and error handling) |
| `task_021_edge_cases.rs` | Task 021 edge cases and boundary conditions |
| `manual_validation_task_021.rs` | Task 021 manual validation in realistic scenarios |
| `task_022_installed_app_resolution.rs` | Task 022 workspace resolution for installed applications |

### Backward Compatibility Tests

| File | Responsibility |
|------|----------------|
| `backward_compatibility_validation.rs` | Ensures existing functionality unchanged after task implementations |

## Test Execution

### Standard Test Levels

Run tests using the `w3` tool with appropriate test levels:

```bash
# Level 1: Nextest only
w3 .test l::1

# Level 2: Nextest + doc tests
w3 .test l::2

# Level 3: Nextest + doc tests + clippy (default)
w3 .test l::3

# Level 4: Full validation with dependency audit
w3 .test l::4

# Level 5: Complete validation with willbe tests
w3 .test l::5
```

### Feature-Specific Testing

Many tests require specific feature flags:

```bash
# Test secret management features
cargo nextest run --features secrets

# Test configuration validation
cargo nextest run --features testing,validation

# Test secure secret handling
cargo nextest run --features secure

# All features
cargo nextest run --all-features
```

## Test Quality Standards

All tests in this directory follow these quality standards (see `$PRO/genai/code/rules/test_organization.rulebook.md`):

- **STATC Quality**: Specific, Technical, Actionable, Traceable, Concise documentation
- **No Mocking**: Real implementations only, no mocks/stubs/fakes
- **Loud Failures**: Clear error messages, explicit assertions
- **Test Matrices**: Comprehensive coverage documentation
- **Bug Reproducers**: 5-section documentation format for bug fix tests
- **Domain Organization**: Tests organized by WHAT is tested, not HOW

## Adding New Tests

Before adding a new test file:

1. **Check this Responsibility Table** - Does a file with this responsibility already exist?
2. **One-Second Test** - Read the responsibility column. Is your test's purpose clearly distinct?
3. **Update this readme** - Add your new test file to the appropriate category with a clear, specific responsibility (3-10 words)
4. **Follow standards** - See `test_organization.rulebook.md` for complete requirements

If no suitable category exists, consult the team before creating a new organizational structure.
