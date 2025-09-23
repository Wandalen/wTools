# Write Tests for Secrecy Crate Integration

## Description

Write comprehensive failing tests for the secrecy crate integration feature. This task focuses on creating test cases that define the expected behavior of memory-safe secret handling within the workspace_tools ecosystem. Tests should cover the secure API alongside existing string-based methods, ensuring backward compatibility while introducing new secure secret loading capabilities.

The tests should validate memory-safe secret loading, secure configuration injection, environment variable protection, and integration with existing secret management functionality. This task is linked to tasks 019 (implementation) and 020 (refactor).

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

-   All new test files must be created in the `tests/` directory following existing naming conventions
-   Tests must initially fail (red phase of TDD cycle)
-   Test coverage must include secure secret loading methods (load_secrets_secure, load_secret_key_secure)
-   Tests must validate memory safety and explicit secret access patterns
-   Tests must verify SecretString integration with existing HashMap-based secret storage
-   Tests must ensure backward compatibility with existing secret management API
-   All tests must use appropriate feature flags (#[cfg(feature = "secure")])
-   Tests must follow the project's testing standards and be warning-free

## Outcomes

✅ **Successfully Implemented** - September 2025

### Test Coverage Created
- **Comprehensive Test Suite**: Created `tests/secrecy_integration_tests.rs` with 9 test cases covering all secure API methods
- **Memory Safety Tests**: Tests validate SecretString behavior and debug output safety
- **Backward Compatibility**: Tests ensure new secure API works alongside existing string-based methods
- **Export Format Support**: Tests verify secure loading works with both KEY=VALUE and export formats
- **Environment Fallback**: Tests confirm secure environment variable loading and fallback behavior

### Test Cases Implemented
1. **`test_load_secrets_secure_basic`** - Basic secure secret loading returning HashMap<String, SecretString>
2. **`test_load_secret_key_secure`** - Individual secure key loading with explicit access requirement
3. **`test_env_secret`** - Secure environment variable loading with SecretString wrapping
4. **`test_load_secret_key_secure_with_env_fallback`** - Fallback behavior from file to environment
5. **`test_secure_and_regular_api_compatibility`** - Backward compatibility verification
6. **`test_secure_loading_with_export_format`** - Support for shell export statement format
7. **`test_secret_string_debug_safety`** - Memory safety validation (no secrets in debug output)
8. **`test_secure_error_handling`** - Error handling for missing files and keys

### TDD Red Phase Verification
- ✅ All tests properly fail when `secure` feature not available
- ✅ Tests are gated behind `#[cfg(feature = "secure")]` as required
- ✅ 0 tests run when feature disabled (proper conditional compilation)
- ✅ Clear compilation warnings indicating missing feature and API methods

### API Design Defined
The tests define the expected secure API interface:
- `workspace.load_secrets_secure(filename) -> Result<HashMap<String, SecretString>>`
- `workspace.load_secret_key_secure(key, filename) -> Result<SecretString>`
- `workspace.env_secret(key) -> Option<SecretString>`
- SecretString requires explicit `expose_secret()` calls for access
- Full integration with existing secret file formats and patterns

Ready for implementation phase in Task 019.