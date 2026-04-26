# Feature: Help System

### Scope

- **Purpose**: Provides universal and per-command help access via auto-generated help commands.
- **Responsibility**: Documents the help system behavior and filtering rules.
- **In Scope**: `.` and `.help` universal help; `.command.help` per-command help; help filtering from listings.
- **Out of Scope**: Command implementation details, verbosity levels on non-help commands.

### Design

Universal help (`.` or `.help`) lists all registered non-help commands with their hints and examples. Per-command help (e.g., `.archive.new.help`) shows the command description, all parameters with kinds and defaults, and usage examples. Help commands are filtered from the command listing to avoid noise. The unilang framework auto-generates help commands for every registered command; genfile does not implement help handlers manually.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/commands/mod.rs` | Registry that enables auto-generated help |
| doc | `docs/cli/readme.md` | CLI documentation including help system overview |
