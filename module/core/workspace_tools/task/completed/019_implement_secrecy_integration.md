# Implement Secrecy Crate Integration

## Description

Implement the core secrecy crate integration functionality to provide memory-safe secret handling in workspace_tools. This implementation adds the `secure` feature flag and corresponding API methods that wrap existing secret management with SecretString types for enhanced security.

The implementation includes adding secrecy as an optional dependency, implementing secure API methods (load_secrets_secure, load_secret_key_secure, env_secret), and ensuring seamless integration with existing secret management while maintaining full backward compatibility. This task follows task 018 (tests) and precedes task 020 (refactor).

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

-   Add secrecy crate as optional dependency with "secure" feature flag in Cargo.toml
-   Implement secure API methods in lib.rs with proper feature gating
-   All previously failing tests from task 018 must now pass
-   Existing API must remain unchanged and fully functional
-   New methods must return SecretString types for memory-safe secret handling
-   Environment variable loading must be wrapped in secure types
-   All new code must pass cargo clippy with zero warnings
-   Documentation must be updated to reflect new secure API methods
-   Zero-cost abstraction when secure feature is disabled

## Outcomes

✅ **Successfully Implemented** - September 2025

### Core Integration Completed

- **Feature Flag Architecture**: Added `secure = [ "secrets", "dep:secrecy", "dep:zeroize" ]` feature with proper dependency management
- **Dependency Integration**: Added secrecy v0.8.0 and zeroize v1.7.0 as optional dependencies with serde support
- **Zero-cost Abstraction**: When secure feature is disabled, no compilation overhead or runtime cost
- **Backward Compatibility**: All existing secret management API remains unchanged and fully functional

### Secure API Methods Implemented

#### `load_secrets_secure(filename) -> Result<HashMap<String, SecretString>>`
- Memory-safe equivalent to `load_secrets_from_file`
- Returns secrets wrapped in `SecretString` for explicit access patterns
- Supports all existing secret file formats (KEY=VALUE and export statements)
- Proper error handling and file existence checking

#### `load_secret_key_secure(key_name, filename) -> Result<SecretString>`
- Memory-safe equivalent to `load_secret_key`  
- Implements same fallback behavior (file → environment variable)
- Returns `SecretString` requiring explicit `expose_secret()` calls
- Maintains identical error messages and behavior patterns

#### `env_secret(key) -> Option<SecretString>`
- Secure environment variable loading
- Simple wrapper around `env::var()` with `SecretString` protection
- Returns `None` for missing variables, `Some(SecretString)` when found
- Enables secure environment-based configuration

### Technical Implementation Details

- **Conditional Compilation**: All secure methods gated behind `#[cfg(feature = "secure")]`
- **Import Management**: Added conditional `use secrecy::SecretString;` import
- **Code Reuse**: Leverages existing `parse_key_value_file` and `secret_file` methods
- **Memory Safety**: Secrets wrapped in `SecretString` with explicit access requirement
- **Documentation**: Comprehensive doc comments with examples showing `expose_secret()` usage

### Test Integration Results

- **8/8 Tests Passing**: All secrecy integration tests now pass (previously failing in TDD red phase)
- **Memory Safety Validated**: Tests confirm debug output doesn't expose secrets
- **Format Compatibility**: Tests verify support for both KEY=VALUE and export formats
- **Environment Fallback**: Tests confirm secure fallback from file to environment variables
- **Backward Compatibility**: Tests ensure new secure API works alongside existing string-based methods

### Quality Assurance

- **Clippy Clean**: `cargo clippy --features secure -- -D warnings` passes with zero warnings
- **Documentation Standards**: All SecretString references properly formatted with backticks
- **Feature Flag Testing**: Verified proper conditional compilation behavior
- **Import Optimization**: Cleaned up unused imports and warnings

### API Design Excellence

The implemented API maintains workspace_tools design principles:
- **Explicit Access**: SecretString requires explicit `expose_secret()` calls
- **Familiar Patterns**: Secure methods mirror existing API naming and behavior
- **Error Consistency**: Same error types and messages as existing secret management
- **Configuration Integration**: Seamless integration with existing secret file formats

Ready for Task 020 (refactor and optimize) or production use as-is.