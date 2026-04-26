# Feature: Help System

Configurable help generation produces formatted command documentation at runtime, supporting multiple detail levels and output formats.

### Scope

- **Purpose**: Provides users with discoverable command documentation from within the CLI.
- **Responsibility**: Documents help variants, generation pipeline, and format options.
- **In Scope**: HelpVariants enum, unified_help routine, markdown formatter, level of detail.
- **Out of Scope**: Internal dot commands that trigger help (see feature/005).

### Design

Four help variants control which help modes are available: All enables every mode, General shows the command list, SubjectCommand shows detailed help for a specific command, and DotCommand enables dot-prefix inspection commands.

Help content is auto-generated at form-time via the CommandsAggregator mutator. The mutator injects a .help command into the Dictionary with a unified_help routine that handles both general help (no arguments) and command-specific help (command name as subject) in a single code path.

Output passes through a formatter. The markdown generator produces structured output with a table of contents and per-command sections including subjects and properties with types and optionality markers.

The level of detail system has three tiers: None suppresses detail, Simple shows hint only, and Detailed shows long_hint with full parameter descriptions. Property and subject display respects the command ordering setting.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/ca/help.rs` | HelpVariants, unified_help, HelpGeneratorFn |
| source | `src/ca/formatter.rs` | Markdown help output generation |
| test | `tests/inc/commands_aggregator/help.rs` | Help output and ordering tests |
| doc | [feature/005_command_routing.md](005_command_routing.md) | Dot commands that invoke help |
| doc | [invariant/004_help_no_execute.md](../invariant/004_help_no_execute.md) | Help must not execute commands |
