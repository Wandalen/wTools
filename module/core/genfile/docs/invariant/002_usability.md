# Invariant: Usability

### Scope

- **Purpose**: Ensures the CLI follows consistent conventions from the cli rulebook throughout.
- **Responsibility**: Documents the usability constraints: naming, format, error messages, and dry-run.
- **In Scope**: Dot-prefix naming, `param::value` format, verbosity levels, dry-run support.
- **Out of Scope**: Command functionality, performance targets (→ 001).

### Invariant Statement

All commands must follow dot-prefix naming with snake_case segments. All parameters must use the `param::value` format. Verbosity levels 0-5 must be supported on all commands. Destructive operations must support dry-run (`dry::1`). Error messages must include resolution guidance. CLI rulebook compliance must be 100%.

### Enforcement Mechanism

CLI compliance audit against cli.rulebook.md standards. Automated tests verify naming conventions. Manual review checks error message quality and dry-run behavior. Command definitions in `src/commands/*.rs` serve as the authoritative spec for CLI structure.

### Violation Consequences

Inconsistent CLI conventions frustrate users who internalize patterns from one command and expect them elsewhere. Non-standard error messages leave users unable to diagnose failures.

### Features

| File | Relationship |
|------|--------------|
| [`feature/009_help_system.md`](../feature/009_help_system.md) | Help system that must conform to CLI usability conventions |

### Docs

| File | Relationship |
|------|--------------|
| [`docs/cli/readme.md`](../cli/readme.md) | CLI design documentation |
