# Feature: Help System

### Scope

- **Purpose**: Provides universal and per-command help access via auto-generated help commands.
- **Responsibility**: Documents the help system behavior and filtering rules.
- **In Scope**: `.` and `.help` universal help; `.command.help` per-command help; help filtering from listings.
- **Out of Scope**: Command implementation details, verbosity levels on non-help commands.

### Design

Universal help (`.` or `.help`) lists all registered non-help commands with their hints and examples. Per-command help (e.g., `.archive.new.help`) shows the command description, all parameters with kinds and defaults, and usage examples. Help commands are auto-generated for every registered command and filtered from command listings to avoid noise.

### Invariants

| File | Relationship |
|------|--------------|
| [`invariant/002_usability.md`](../invariant/002_usability.md) | Usability constraint that help system format must satisfy |
| [`invariant/006_documentation.md`](../invariant/006_documentation.md) | Documentation completeness constraint enforced through help |

### Docs

| File | Relationship |
|------|--------------|
| [`docs/cli/readme.md`](../cli/readme.md) | CLI documentation including help system overview |

### Sources

| File | Relationship |
|------|--------------|
| [`src/commands/mod.rs`](../../src/commands/mod.rs) | Registry that enables auto-generated help |
