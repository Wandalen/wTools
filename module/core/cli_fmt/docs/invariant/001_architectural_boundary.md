# Invariant: Architectural Boundary

### Scope

- **Purpose**: Define the separation boundary between `cli_fmt` and `strs_tools`.
- **Responsibility**: Document which processing belongs in each crate and why CLI-specific policy must not migrate to general-purpose utilities.
- **In Scope**: Boundary placement rules, enforcement mechanism, and violation consequences.
- **Out of Scope**: Processing logic and behavior — see `feature/` instances.

### Invariant Statement

`cli_fmt` implements CLI-specific policy decisions. `strs_tools` provides general-purpose
text and ANSI manipulation without application-domain assumptions. No CLI-specific logic
belongs in `strs_tools`.

`strs_tools` is designed to be reusable across any application. CLI-specific policy
decisions embedded in `strs_tools` would impose unwanted assumptions on non-CLI consumers.

### Enforcement Mechanism

- `cli_fmt` depends on `strs_tools` — the dependency is one-directional only.
- `strs_tools` carries no stream concepts, head/tail conventions, output-transparency types, or CLI-help-rendering types (`CliHelpTemplate`, `CliHelpStyle`, `CliHelpData`, etc.).
- New CLI-specific utilities belong in `cli_fmt`.
- New general-purpose text or ANSI utilities belong in `strs_tools`.
- Feature flags in `cli_fmt` are independent of `strs_tools` feature flags.

### Violation Consequences

Placing CLI-specific policy in `strs_tools` breaks reusability for non-CLI consumers,
which would inherit CLI assumptions they do not need. Placing general text utilities in
`cli_fmt` prevents their reuse outside CLI applications and increases coupling between
the two crates.

### APIs

| File | Relationship |
|------|-------------|
| [`../api/001_output_api.md`](../api/001_output_api.md) | Output processing interface — types encode this boundary |
| [`../api/002_help_api.md`](../api/002_help_api.md) | Help template interface — types encode this boundary |

### Features

| File | Relationship |
|------|-------------|
| [`../feature/001_output_processing.md`](../feature/001_output_processing.md) | CLI output processing — governed by this boundary |
| [`../feature/002_cli_help_template.md`](../feature/002_cli_help_template.md) | CLI help template rendering — governed by this boundary |

### Test Specs

| File | Relationship |
|------|-------------|
| [`../../tests/docs/invariant/001_architectural_boundary.md`](../../tests/docs/invariant/001_architectural_boundary.md) | Test specification verifying this invariant holds in the codebase |

### Sources

| File | Relationship |
|------|-------------|
| `Cargo.toml` | Dependency declarations enforce one-directional cli_fmt → strs_tools dependency |

### Tests

| File | Relationship |
|------|-------------|
| `tests/output.rs` | Output pipeline tests confirm CLI-specific processing stays within cli_fmt |
| `tests/help.rs` | Help rendering tests confirm cli_fmt carries no data_fmt reverse dependency |
