# Feature: Fluent Builder

The builder pattern provides a method-chaining interface for registering commands, subjects, properties, and routines before forming an immutable CommandsAggregator.

### Scope

- **Purpose**: Enables ergonomic command registration without manual struct construction.
- **Responsibility**: Documents the builder chain, subformers, and form-time side effects.
- **In Scope**: Aggregator builder entry point, command registration chain, form-time mutator.
- **Out of Scope**: Pipeline execution (see feature/001), type definitions (see feature/003).

### Design

CommandsAggregator uses the Former derive macro to generate a builder. Callers open a command builder by name, then chain declarations for subjects, properties, and routines before closing the subformer. After all commands are registered, a final call forms the aggregator and returns it immutably.

A custom mutator runs at form-time to auto-generate help content for all registered commands, injecting help entries into the Dictionary before the aggregator is sealed.

The builder accepts help_variants to control which help modes are available, order to set command listing sort, context for shared state, and callback for post-execution hooks.

willbe uses this pattern to register 13 commands in a single chain. unitore bypasses CommandsAggregator and builds a Dictionary directly using the Dictionary builder.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/ca/aggregator.rs` | Builder implementation with custom mutator |
| source | `src/ca/grammar/command.rs` | Command subformer (subject, property, routine) |
| test | `tests/inc/commands_aggregator/basic.rs` | Builder integration tests |
| doc | [feature/001_command_pipeline.md](001_command_pipeline.md) | Pipeline the builder configures |
| doc | [api/001_commands_aggregator.md](../api/001_commands_aggregator.md) | API surface of built aggregator |
| doc | [invariant/005_routine_required.md](../invariant/005_routine_required.md) | Routine must be attached before execution |
