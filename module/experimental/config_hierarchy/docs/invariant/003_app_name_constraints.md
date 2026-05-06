# Invariant: App Name Constraints

### Scope

- **Purpose**: Define the validation rules applied to the application name before any path construction.
- **Responsibility**: Document what characters are forbidden and why, and what happens on violation.
- **In Scope**: Every call that uses the app name to construct a path or env var prefix.
- **Out of Scope**: Path formulas (→ api/001); env var naming (→ invariant/001).

### Invariant Statement

The application name (the value returned by `app_name()`) must satisfy all of the following at the time it is used in path construction:

| Rule | Forbidden | Reason |
|------|-----------|--------|
| Non-empty | `""` | Produces path fragments like `.//config.yaml` |
| No forward slash | `/` | Enables directory traversal in constructed paths |
| No backslash | `\` | Enables directory traversal on Windows |
| No double-dot | `..` | Enables path traversal attacks |

No other characters are prohibited — alphanumeric, hyphens, underscores, and Unicode are all valid. Whitespace is valid but inadvisable (causes shell quoting issues).

### Enforcement Mechanism

A validation function checks these rules before any path is constructed. Path discovery functions return an error on violation. The local config discovery loop silently skips invalid names rather than propagating errors that would abort the entire walk.

### Violation Consequences

An invalid app name causes all path construction and discovery to fail. No config files are read or written. The env var prefix derivation may still succeed (it does not pass through path validation) but will produce an env var name that is unusable from a shell.

### APIs

| File | Relationship |
|------|--------------|
| [api/001_config_paths_trait.md](../api/001_config_paths_trait.md) | app_name() method that must satisfy these constraints |
| [api/004_config_manager.md](../api/004_config_manager.md) | Manager operations that trigger validation |

### Features

| File | Relationship |
|------|--------------|
| [feature/001_config_hierarchy.md](../feature/001_config_hierarchy.md) | Security section that describes these rules |

### Sources

| File | Relationship |
|------|--------------|
| [src/path_discovery.rs](../../src/path_discovery.rs) | Validation function implementation |

### Tests

| File | Relationship |
|------|--------------|
| [tests/path_standards_tests.rs](../../tests/path_standards_tests.rs) | App name validation tests |
| [tests/edge_cases_tests.rs](../../tests/edge_cases_tests.rs) | Boundary and invalid name tests |
