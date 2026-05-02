# CLI Test Surface

### Scope

- **Purpose**: Document the complete CLI test surface for `program_tools`.
- **Responsibility**: Houses test spec files for all CLI test surface elements — commands, parameters, and invariants.
- **In Scope**: `run` subcommand; all 10 named flags and the `<TARGET>` positional; 4 behavioral invariants.
- **Out of Scope**: Programmatic API tests (→ `tests/inc/`); data structure tests (→ `tests/inc/corner_cases_test.rs`).

### Responsibility Table

| Directory | Responsibility |
|-----------|----------------|
| `command/` | Test specs for CLI subcommands (TC- cases, min 8 per command) |
| `param/` | Test specs for CLI parameters and flags (EC- cases, min 6 per param) |
| `invariant/` | Test specs for behavioral invariants (IC- cases, min 2 per invariant) |
