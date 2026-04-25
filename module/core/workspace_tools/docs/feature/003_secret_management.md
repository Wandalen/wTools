# Feature: Secret Management

### Scope

**Purpose**: Load secret credentials from workspace-relative files while preventing accidental disclosure through logging or debug output.
**Responsibility**: Parse KEY=VALUE secret files, provide plaintext and memory-safe (SecretString) access, and search a three-directory fallback chain so installed applications find secrets regardless of invocation context.
**In Scope**: `load_secrets_from_file()`, `load_secret_key()`, `env_secret()`, `load_secrets_with_fallback()`, `load_secret_key_with_fallback()` (secrets feature); `load_secrets_secure()`, `load_secret_key_secure()`, `validate_secret()`, `load_config_with_secrets()`, `load_config_with_secret_injection()` (secure feature).
**Out of Scope**: Encryption at rest, cloud secret stores (AWS/GCP/Azure), audit logging of secret accesses, key rotation.

### Design

Secret files use a shell-compatible KEY=VALUE format. Both bare `KEY=VALUE` and `export KEY=VALUE` (dotenv format) are supported. Values may be quoted. The parser handles edge cases documented in `corner_cases_parsing.rs`.

The fallback chain searches three locations in priority order: (1) `secret/` inside the active workspace root, (2) `$PRO/secret/` for shared project credentials, (3) `$HOME/secret/` for machine-wide defaults. Locations are canonicalized before comparison so the same physical directory reachable via different paths is only searched once.

The `secure` feature wraps secrets in `SecretString` from the `secrecy` crate. `SecretString` requires an explicit `.expose_secret()` call to access the value, preventing accidental logging. The `Debug` implementation redacts the value with `[REDACTED]`. Memory is zeroed on drop via `zeroize`.

`load_config_with_secret_injection()` performs template-based injection — it reads a config file, replaces `${KEY}` placeholders with values from the secrets file, and returns the rendered string. This avoids embedding secrets in config files on disk.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| Source | `src/lib.rs` | Secrets impl block, `AsSecure` trait, secure impl block |
| Test | `tests/centralized_secrets_test.rs` | Centralized secret management integration |
| Test | `tests/corner_cases_critical.rs` | Critical corner cases in secret fallback |
| Test | `tests/corner_cases_parsing.rs` | Content parsing edge cases (format variations, quoting) |
| Test | `tests/enhanced_secret_parsing_tests.rs` | Export statements and dotenv format parsing |
| Test | `tests/secret_directory_verification_test.rs` | Secret directory usage verification |
| Test | `tests/secrecy_integration_tests.rs` | Memory-safe secret handling (secrecy crate) |
| Test | `tests/secrecy_optimization_tests.rs` | Advanced secrecy features (SecretInjectable, validation) |
| Test | `tests/test_fallback_integration.rs` | Secret fallback chain integration |
| Test | `tests/test_new_secrets_api_methods.rs` | Path-aware secret API methods |
| Test | `tests/reproduce_secrets_api_ux_issue.rs` | Reproduction of reported API UX issues |
| Task | `task/completed/017_enhanced_secret_parsing.md` | Enhanced parsing implementation |
| Task | `task/completed/018_write_tests_for_secrecy_integration.md` | Secrecy integration tests |
| Task | `task/completed/019_implement_secrecy_integration.md` | SecretString integration |
| Task | `task/completed/020_refactor_and_optimize_secrecy_implementation.md` | Secrecy refactor |
| Task | `task/completed/023_extend_workspace_resolution_for_installed_applications.md` | Secrets fallback chain |
| Doc | `docs/api/001_workspace.md` | Secret management method signatures |
