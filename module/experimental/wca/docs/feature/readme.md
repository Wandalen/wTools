# Feature Doc Entity

### Scope

- **Purpose**: Navigational hubs collecting all artifacts for each user-facing capability of the wca CLI framework.
- **Responsibility**: Indexes source files, test files, and related documentation per feature without duplicating content.
- **In Scope**: Command pipeline, builder API, type system, help, routing, context, fuzzy suggest.
- **Out of Scope**: Public interface contracts (see api/), correctness properties (see invariant/).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Command Pipeline](001_command_pipeline.md) | Three-stage parse-verify-execute architecture | ✅ |
| 002 | [Fluent Builder](002_fluent_builder.md) | CommandsAggregator builder pattern for command registration | ✅ |
| 003 | [Type System](003_type_system.md) | Type-checked command arguments and properties | ✅ |
| 004 | [Help System](004_help_system.md) | Help generation with configurable variants | ✅ |
| 005 | [Command Routing](005_command_routing.md) | Internal dot commands for inspection and listing | ✅ |
| 006 | [Context Sharing](006_context_sharing.md) | Shared execution state across command routines | ✅ |
| 007 | [Fuzzy Suggest](007_fuzzy_suggest.md) | Feature-gated typo correction for unknown commands | ✅ |
