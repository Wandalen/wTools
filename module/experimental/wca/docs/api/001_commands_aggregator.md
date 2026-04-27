# API: Commands Aggregator

Top-level facade that owns the full pipeline and exposes a single entry point for processing CLI input from raw strings to executed routines.

### Scope

- **Purpose**: Provides the primary public interface for building and running a CLI application.
- **Responsibility**: Documents the facade operations, builder options, and callback mechanism.
- **In Scope**: Pipeline entry point, fluent builder, help variant configuration, command ordering, post-execution callback, error types.
- **Out of Scope**: Internal pipeline stages (see api/003, api/004), type definitions (see api/002).

### Abstract

CommandsAggregator is the main entry point for wca consumers. It owns a Parser, Verifier, Executor, and Dictionary, orchestrating them through the pipeline entry point. Callers build an aggregator using the fluent builder pattern, register commands with their types and routines, then invoke the entry point with input to execute the full pipeline.

### Operations

The pipeline entry point accepts any input implementing the input conversion interface and processes it through all three pipeline stages. It returns either success or a pipeline error.

The builder supports command registration by name, help variant configuration, command ordering, shared context injection, and post-execution hooks via callback.

The callback operation receives the raw input string and the verified program after each successful pipeline run, enabling logging, history tracking, or external integrations.

### Error Handling

Two top-level error categories exist: Validation errors occur before execution when parsing or verification fails, and Execution errors occur when a routine returns an error. Validation errors subdivide into Parser errors (malformed input) and Verifier errors (unknown command, type mismatch, missing required argument).

### Compatibility Guarantees

The crate uses semantic versioning. The CommandsAggregator builder API and pipeline entry point are the stable public surface. Intermediate representation types are exposed but considered semi-stable.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/ca/aggregator.rs` | CommandsAggregator struct and builder |
| source | `src/lib.rs` | Public re-exports via mod_interface |
| test | `tests/inc/commands_aggregator/basic.rs` | Core aggregator integration tests |
| test | `tests/inc/commands_aggregator/callback.rs` | Callback mechanism tests |
| doc | [feature/001_command_pipeline.md](../feature/001_command_pipeline.md) | Pipeline architecture |
| doc | [feature/002_fluent_builder.md](../feature/002_fluent_builder.md) | Builder pattern details |
| doc | [api/005_input.md](005_input.md) | Input conversion interface accepted by the pipeline |
