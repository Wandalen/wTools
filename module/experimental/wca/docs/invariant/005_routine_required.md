# Invariant: Routine Required

Every registered command must have a routine attached before execution. Attempting to execute a command with no routine is a terminal error, not a silent no-op.

### Scope

- **Purpose**: Prevents silent failures when a command definition lacks an execution handler.
- **Responsibility**: Documents the routine presence contract and its enforcement point.
- **In Scope**: Routine lookup in executor, error on missing routine, Routine enum variants.
- **Out of Scope**: Routine registration via builder (see feature/002), Handler closure signatures (see api/004).

### Invariant Statement

When the executor dispatches a verified command to its routine, it must find a Routine value in the Dictionary entry for that command. If the routine is absent (the command was registered with type information but no execution closure), the executor must return an execution error. It must not silently skip the command, return success, or fall through to a default behavior.

### Enforcement Mechanism

The executor looks up the command in the Dictionary by phrase name, then accesses the routine field. If the routine is not present, the executor constructs an execution error and returns it immediately. The error propagates to the caller as an Execution variant of the top-level Error enum. A dedicated test verifies this behavior by registering a command without a routine and asserting that execution produces the expected error.

### Violation Consequences

If a missing routine were treated as success, callers would silently lose command functionality without any diagnostic signal. Debugging would be difficult because the command would appear to work (no error) while producing no effect. The terminal error ensures that routine absence is caught immediately during development or integration testing.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/ca/executor/executor.rs` | Routine lookup and missing-routine error |
| source | `src/ca/executor/routine.rs` | Routine enum definition |
| test | `tests/inc/executor/command.rs` | Missing routine error test |
| doc | [api/004_executor.md](../api/004_executor.md) | Executor dispatch documentation |
| doc | [api/002_grammar.md](../api/002_grammar.md) | Command and Dictionary structure |
| doc | [feature/002_fluent_builder.md](../feature/002_fluent_builder.md) | Routine attachment via builder |
| doc | [feature/006_context_sharing.md](../feature/006_context_sharing.md) | Context routines also require presence |
