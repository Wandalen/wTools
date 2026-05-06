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

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Secrets loading, fallback chain, and memory-safe secret impl |
| config | `Cargo.toml` | `secrets` and `secure` feature flags and their optional dependency declarations |
| test | `tests/centralized_secrets_test.rs` | Centralized secret management integration |
| test | `tests/corner_cases_critical.rs` | Critical corner cases in secret fallback |
| test | `tests/corner_cases_parsing.rs` | Content parsing edge cases (format variations, quoting) |
| test | `tests/enhanced_secret_parsing_tests.rs` | Export statements and dotenv format parsing |
| test | `tests/secret_directory_verification_test.rs` | Secret directory usage verification |
| test | `tests/secrecy_integration_tests.rs` | Memory-safe secret handling (secrecy crate) |
| test | `tests/secrecy_optimization_tests.rs` | Advanced secrecy features (SecretInjectable, validation) |
| test | `tests/test_fallback_integration.rs` | Secret fallback chain integration |
| test | `tests/test_new_secrets_api_methods.rs` | Path-aware secret API methods |
| test | `tests/reproduce_secrets_api_ux_issue.rs` | Reproduction of reported API UX issues |
| task | `task/completed/017_enhanced_secret_parsing.md` | Enhanced parsing implementation |
| task | `task/completed/018_write_tests_for_secrecy_integration.md` | Secrecy integration tests |
| task | `task/completed/019_implement_secrecy_integration.md` | SecretString integration |
| task | `task/completed/020_refactor_and_optimize_secrecy_implementation.md` | Secrecy refactor |
| task | `task/completed/023_extend_workspace_resolution_for_installed_applications.md` | Secrets fallback chain |
| doc | `docs/api/001_workspace.md` | Secret management method signatures |
