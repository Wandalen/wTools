# Feature: Secret Management

### Scope

- **Purpose**: Load secret credentials from workspace-relative files while preventing accidental disclosure through logging or debug output.
- **Responsibility**: Parse KEY=VALUE secret files, provide plaintext and memory-safe secret access, and search a three-directory fallback chain so installed applications find secrets regardless of invocation context.
- **In Scope**: Plaintext secret loading from files and environment, directory fallback chain search (requires `secrets` feature); memory-safe secret wrapping, strength validation, and config injection (requires `secure` feature).
- **Out of Scope**: Encryption at rest, cloud secret stores (AWS/GCP/Azure), audit logging of secret accesses, key rotation.

### Design

Secret files use a shell-compatible KEY=VALUE format. Both bare `KEY=VALUE` and `export KEY=VALUE` (dotenv format) are supported. Values may be quoted. The parser handles edge cases documented in `corner_cases_parsing.rs`.

The fallback chain searches three locations in priority order: (1) `secret/` inside the active workspace root, (2) `$PRO/secret/` for shared project credentials, (3) `$HOME/secret/` for machine-wide defaults. Locations are canonicalized before comparison so the same physical directory reachable via different paths is only searched once.

The `secure` feature wraps secrets in a memory-safe handle that requires an explicit exposure call to read the value, preventing accidental inclusion in log output. The debug representation redacts the value entirely. Memory is zeroed when the handle is dropped, preventing secrets from lingering in heap memory after use.

Template-based injection reads a config file, replaces named placeholders with values from the secrets file, and returns the rendered string. This avoids embedding secrets in config files on disk.

### Sources

| File | Relationship |
|------|-------------|
| [src/lib.rs](../../src/lib.rs) | Secrets loading, fallback chain, and memory-safe secret impl |
| [Cargo.toml](../../Cargo.toml) | `secrets` and `secure` feature flags and optional dependency declarations |

### Tests

| File | Relationship |
|------|-------------|
| [tests/centralized_secrets_test.rs](../../tests/centralized_secrets_test.rs) | Centralized secret management integration |
| [tests/corner_cases_critical.rs](../../tests/corner_cases_critical.rs) | Critical corner cases in secret fallback |
| [tests/corner_cases_parsing.rs](../../tests/corner_cases_parsing.rs) | Content parsing edge cases (format variations, quoting) |
| [tests/enhanced_secret_parsing_tests.rs](../../tests/enhanced_secret_parsing_tests.rs) | Export statements and dotenv format parsing |
| [tests/secret_directory_verification_test.rs](../../tests/secret_directory_verification_test.rs) | Secret directory usage verification |
| [tests/secrecy_integration_tests.rs](../../tests/secrecy_integration_tests.rs) | Memory-safe secret handling (secrecy crate) |
| [tests/secrecy_optimization_tests.rs](../../tests/secrecy_optimization_tests.rs) | Advanced secrecy features (SecretInjectable, validation) |
| [tests/test_fallback_integration.rs](../../tests/test_fallback_integration.rs) | Secret fallback chain integration |
| [tests/test_new_secrets_api_methods.rs](../../tests/test_new_secrets_api_methods.rs) | Path-aware secret API methods |
| [tests/reproduce_secrets_api_ux_issue.rs](../../tests/reproduce_secrets_api_ux_issue.rs) | Reproduction of reported API UX issues |

### Tasks

| File | Relationship |
|------|-------------|
| [task/completed/017_enhanced_secret_parsing.md](../../task/completed/017_enhanced_secret_parsing.md) | Enhanced parsing implementation |
| [task/completed/018_write_tests_for_secrecy_integration.md](../../task/completed/018_write_tests_for_secrecy_integration.md) | Secrecy integration tests |
| [task/completed/019_implement_secrecy_integration.md](../../task/completed/019_implement_secrecy_integration.md) | SecretString integration |
| [task/completed/020_refactor_and_optimize_secrecy_implementation.md](../../task/completed/020_refactor_and_optimize_secrecy_implementation.md) | Secrecy refactor |
| [task/completed/023_extend_workspace_resolution_for_installed_applications.md](../../task/completed/023_extend_workspace_resolution_for_installed_applications.md) | Secrets fallback chain |

### APIs

| File | Relationship |
|------|-------------|
| [api/001_workspace.md](../api/001_workspace.md) | Secret management method signatures |
