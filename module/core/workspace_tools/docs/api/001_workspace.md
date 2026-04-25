# API: Workspace

### Scope

**Purpose**: Expose a workspace root handle and resolution utilities as the sole public interface of `workspace_tools`.
**Responsibility**: Define the `Workspace` struct, the `workspace()` free function, and all feature-gated method groups for path operations, configuration loading, secret management, resource discovery, and validation.
**In Scope**: `Workspace` struct, `workspace()` free function, all `impl Workspace` method groups, `WorkspaceError` enum, `Result<T>` alias, `SecretInjectable` trait, `AsSecure` trait, `testing` module.
**Out of Scope**: Internal helpers (`cleanup_path`, `detect_format`, `parse_content`), serde trait implementations, derive macro internals.

### Abstract

`workspace_tools` exposes a single library root with no public sub-modules except the `testing` feature. The central type is `Workspace` — a cloneable, hashable struct wrapping one normalized absolute path (the workspace root).

The API is feature-gated in layers: the core path API is always available; `serde`, `secrets`, `secure`, `glob`, and `validation` each add method groups. Enabling `secure` automatically enables `secrets`; enabling `validation` automatically enables `serde`. The `full` feature enables everything.

### Operations

#### Free Functions

`workspace()` — auto-detects the workspace root using the full six-strategy fallback chain; the top-level entry point for most callers; equivalent to `Workspace::resolve_with_extended_fallbacks()`.

#### Creation Methods (always available)

`Workspace::new(root)` — constructs a workspace from an explicit path; normalizes trailing `/.` components automatically; never fails.

`Workspace::resolve()` — tries cargo workspace metadata detection first; falls back to the `WORKSPACE_PATH` environment variable.

`Workspace::resolve_with_extended_fallbacks()` — tries all six strategies in priority order: cargo metadata, `WORKSPACE_PATH`, git root, `$PRO`, `$HOME`, current directory.

`Workspace::from_current_dir()` — resolves using the process current directory as the starting point.

`Workspace::from_git_root()` — walks ancestor directories to find a `.git` directory alongside a `Cargo.toml`.

`Workspace::from_cwd()` — uses the raw process current working directory as the workspace root without further detection.

`Workspace::from_pro_env()` — reads the `$PRO` environment variable as the workspace root path.

`Workspace::from_home_dir()` — uses the user home directory as the workspace root.

`Workspace::from_cargo_workspace()` — invokes cargo metadata to detect the workspace root from the nearest `Cargo.toml`.

`Workspace::from_cargo_manifest(manifest_path)` — uses a specific manifest path to determine the workspace root via cargo metadata.

`Workspace::cargo_metadata()` — returns the raw cargo metadata struct for the workspace.

#### Path Methods (always available)

`Workspace::root()` — returns the normalized absolute workspace root path.

`Workspace::join(path)` — appends a relative path to the workspace root; returns an absolute path.

`Workspace::config_dir()` — returns `root/config/`.

`Workspace::data_dir()` — returns `root/data/`.

`Workspace::logs_dir()` — returns `root/logs/`.

`Workspace::docs_dir()` — returns `root/docs/`.

`Workspace::tests_dir()` — returns `root/tests/`.

#### Configuration Loading (`serde` feature)

`Workspace::load_config(name)` — loads a typed configuration by name; searches for the file with `.toml`, then `.json`, then `.yaml`/`.yml` extension.

`Workspace::load_config_from(path)` — loads a typed configuration from an explicit workspace-relative path.

`Workspace::load_config_layered(names)` — loads and deep-merges multiple named configurations; later entries override earlier ones.

`Workspace::load_config_with_merge(base, overlay)` — loads one configuration and merges a second on top of it.

`Workspace::find_config(name)` — returns the path of the first matching configuration file using the priority search order.

`Workspace::merge_config(base, overlay)` — merges two configuration value trees; overlay wins on any conflict.

`Workspace::save_config(name, config)` — serializes a typed value and writes it as TOML to a workspace-relative path.

#### Secret Management (`secrets` feature)

`Workspace::load_secrets_from_file(file)` — parses all KEY=VALUE pairs from a workspace-relative secret file; supports bare and `export KEY=VALUE` formats.

`Workspace::load_secret_key(key, file)` — loads a single named secret from a workspace-relative secret file.

`Workspace::env_secret(var)` — reads a secret from an environment variable; returns absent when the variable is unset.

`Workspace::load_secrets_with_fallback(file)` — loads secrets searching three locations in order: workspace `secret/`, `$PRO/secret/`, `$HOME/secret/`; deduplicates by canonicalized path.

`Workspace::load_secret_key_with_fallback(key, file)` — loads a single named secret via the three-directory fallback chain.

#### Memory-Safe Secrets (`secure` feature, implies `secrets`)

`Workspace::load_secrets_secure(file)` — loads secrets wrapped in `SecretString`; value requires explicit exposure call to read; memory is zeroed on drop.

`Workspace::load_secret_key_secure(key, file)` — loads a single secret as `SecretString`.

`Workspace::validate_secret(secret)` — checks a secret string against minimum strength requirements.

`Workspace::load_config_with_secrets(config, secrets_file)` — injects secrets from a file into a typed config struct via the `SecretInjectable` trait.

`Workspace::load_config_with_secret_injection(config_file, secrets_file)` — renders a config file by replacing `${KEY}` placeholders with values from the secrets file; returns the rendered string.

`SecretInjectable` trait — implement to allow secret injection into a typed configuration struct; requires `inject_secret(key, value)` and `validate_secrets()`.

`AsSecure` trait — converts a plaintext type to its memory-safe equivalent.

#### Resource Discovery (`glob` feature)

`Workspace::find_resources(pattern)` — returns all paths matching a glob pattern anchored at the workspace root; supports standard glob syntax (`**/*.rs`, `config/*.toml`).

#### Configuration Validation (`validation` feature, implies `serde`)

`Workspace::load_config_with_validation(name)` — loads a named configuration and validates it against a JSON Schema derived from the target type at runtime; returns all validation errors, not just the first.

#### Testing Utilities (`testing` feature)

`testing::create_test_workspace_with_structure()` — creates a temporary directory-backed workspace with all standard sub-directories pre-populated; returns a `(TempDir, Workspace)` pair; dropping `TempDir` removes the directory.

### Error Handling

All fallible operations return `Result<T>` where the error type is `WorkspaceError`. `WorkspaceError` is non-exhaustive — match arms must include a catch-all branch to remain compatible with future additions.

Error categories:

- `ConfigurationError` — configuration file content is invalid or inconsistent with the expected structure.
- `EnvironmentVariableMissing` — a required environment variable is not set or is empty.
- `IoError` — a file read, write, or permission operation failed.
- `PathNotFound` — a referenced path does not exist on the filesystem.
- `PathOutsideWorkspace` — a resolved path would escape the workspace root boundary.
- `CargoError` — the cargo metadata command failed or returned unusable output.
- `TomlError` — TOML content could not be parsed.
- `GlobError` (`glob` feature) — a glob pattern is invalid or filesystem traversal failed.
- `SerdeError` (`serde` feature) — deserialization of configuration content into the target type failed.
- `ValidationError` (`validation` feature) — configuration content did not satisfy the JSON Schema.
- `SecretValidationError` (`secure` feature) — a secret value did not meet strength requirements.
- `SecretInjectionError` (`secure` feature) — injection of a named secret into a config struct failed.

Display messages include actionable context: the offending path, key name, or error detail.

### Compatibility Guarantees

The public API follows semantic versioning. `WorkspaceError` is non-exhaustive — new variants may be added in minor releases without a major version bump. Feature flags are strictly additive — enabling additional features never removes or changes existing methods. Methods present without any feature gate are stable across all minor releases.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| Source | `src/lib.rs` | Entire public API implementation |
| Test | `tests/workspace_tests.rs` | Core creation and path methods |
| Test | `tests/error_handling_comprehensive_tests.rs` | WorkspaceError variants and Display |
| Test | `tests/serde_integration_tests.rs` | Configuration loading methods |
| Test | `tests/secrecy_integration_tests.rs` | Memory-safe secret methods |
| Test | `tests/comprehensive_test_suite.rs` | Full API coverage matrix |
| Test | `tests/backward_compatibility_validation.rs` | API stability across versions |
| Doc | `docs/feature/001_workspace_root_resolution.md` | Root resolution feature scope |
| Doc | `docs/feature/002_configuration_loading.md` | Configuration loading feature scope |
| Doc | `docs/feature/003_secret_management.md` | Secret management feature scope |
| Doc | `docs/feature/004_resource_discovery.md` | Resource discovery feature scope |
| Doc | `docs/feature/005_configuration_validation.md` | Configuration validation feature scope |
| Doc | `docs/feature/006_testing_support.md` | Testing support feature scope |
| Doc | `docs/pattern/001_workspace_resolution_fallback.md` | Resolution strategy design |
