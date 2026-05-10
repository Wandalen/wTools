# CLI Test Specs

### Scope

- **Purpose:** Validate behavioral contracts defined in `docs/cli/` through executable test specifications
- **Responsibility:** Enumeration and organization of the CLI test surface across parameters, parameter groups, and commands
- **In Scope:** Parameter invariants, parameter group composition contracts, command behavioral specs, exit code contracts, and format output contracts
- **Out of Scope:** Implementation testing (belongs in `tests/`), manual testing procedures (see `tests/manual/`), type validation (covered by `docs/cli/type.md` invariants)

### Test Architecture

Three tiers of test specs, each covering a distinct layer of the CLI contract:

| Tier | Directory | Prefix | Covers |
|------|-----------|--------|--------|
| 1 | `param/` | `EC-` | Individual parameter invariants (default values, constraints, source priority, edge cases) |
| 2 | `param_group/` | `CC-` | Parameter group composition contracts (co-membership, semantic coherence, cross-command consistency) |
| 3 | `command/` | `IT-` | Command behavioral specs (per-command contracts, exit codes, output format, integration flows) |

### Responsibility Table

| Directory | Responsibility |
|-----------|----------------|
| [command/](command/readme.md) | Spec files for each CLI command namespace (IT- prefix) |
| [param/](param/readme.md) | Spec files for CLI parameters (EC- prefix) |
| [param_group/](param_group/readme.md) | Spec files for CLI parameter groups (GRP- prefix) |
