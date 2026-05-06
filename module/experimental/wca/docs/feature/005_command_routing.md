# Feature: Command Routing

Internal dot commands provide runtime inspection of the command dictionary without executing user-defined routines.

### Scope

- **Purpose**: Enables command discovery and detailed inspection from within the CLI.
- **Responsibility**: Documents the four internal command patterns and their routing logic.
- **In Scope**: Dot-suffix commands (`.`, `.?`), prefix search, detail display, bypass of normal verification.
- **Out of Scope**: Help content generation (see feature/004), normal command execution (see feature/001).

### Design

Four internal command patterns are recognized by the executor:

The list-all command (a single dot character) prints all registered commands. The brief-list command (a single dot followed by question mark) prints a condensed command listing.

The prefix-search command (a name followed by a dot) lists all commands whose names begin with that prefix. The detail command (a name followed by dot and question mark) shows detailed information for matching commands.

Internal commands bypass normal verification. The executor detects them by checking whether the command name ends with a dot or question mark, and routes them to the internal command handler instead of the user command handler. This means they do not require a registered Command entry in the Dictionary.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/ca/executor/executor.rs` | Internal command detection and dispatch |
| source | `src/ca/grammar/dictionary.rs` | search() prefix matching for prefix-search |
| test | `tests/inc/commands_aggregator/basic.rs` | dot_command test for `.` and `.cmd.` |
| doc | [feature/004_help_system.md](004_help_system.md) | Help generation invoked by internal commands |
| doc | [api/004_executor.md](../api/004_executor.md) | Executor API handling internal routing |
| doc | [invariant/001_dot_prefix_required.md](../invariant/001_dot_prefix_required.md) | Dot prefix convention extended by suffix patterns |
| doc | [invariant/004_help_no_execute.md](../invariant/004_help_no_execute.md) | Internal commands must not trigger routines |
