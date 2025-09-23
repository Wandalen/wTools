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