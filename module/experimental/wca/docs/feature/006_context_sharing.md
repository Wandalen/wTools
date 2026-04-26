# Feature: Context Sharing

A type-erased shared state container allows routines to access and mutate application state across multiple command executions.

### Scope

- **Purpose**: Enables stateful CLI applications where commands share data between invocations.
- **Responsibility**: Documents the Context type, its construction, retrieval, and thread safety bounds.
- **In Scope**: Context wrapping, type-safe retrieval via downcasting, Routine::WithContext variant.
- **Out of Scope**: Routine closure signatures (see api/004), builder context registration (see feature/002).

### Design

Context wraps a value as a type-erased container with thread-safety bounds. Construction accepts any concrete type meeting the required trait bounds. Retrieval uses downcasting to recover the original type, returning the value wrapped in a shared reference-counted pointer.

Two routine variants exist: WithoutContext receives only the VerifiedCommand, while WithContext receives both a Context and VerifiedCommand. The executor checks which variant is registered and passes the context accordingly.

Context is set on the Executor at build time via the builder. CommandsAggregator provides a with_context convenience method that wraps the value and passes it to the executor builder.

Multiple commands in a program share the same Context instance, enabling accumulation patterns like the counter example in wca_fluent.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/ca/executor/context.rs` | Context struct, new(), get() |
| source | `src/ca/executor/routine.rs` | Routine::WithContext variant |
| test | `tests/inc/executor/command.rs` | with_context test verifying state access |
| doc | [api/004_executor.md](../api/004_executor.md) | Executor API with context dispatch |
| doc | [invariant/005_routine_required.md](../invariant/005_routine_required.md) | Routine presence requirement |
