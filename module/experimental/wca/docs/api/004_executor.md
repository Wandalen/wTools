# API: Executor

The executor dispatches verified commands to their registered routines, handling both user-defined commands and internal inspection commands.

### Scope

- **Purpose**: Provides the execution layer that converts verified commands into routine invocations.
- **Responsibility**: Documents the Executor interface, routine dispatch, context injection, and internal command handling.
- **In Scope**: Program execution, command dispatch, user command invocation, internal command handling, shared context, routine variants.
- **Out of Scope**: Command verification (see api/003), help content generation (see feature/004).

### Abstract

The Executor owns an optional Context and dispatches verified commands to their routines. It supports two routine variants: one that receives only the command arguments, and one that also receives the shared context. The executor also handles internal commands (dot-suffix and question-mark-suffix) by routing them to built-in inspection logic instead of user-defined routines.

### Operations

The program operation iterates all commands in a verified program and executes each sequentially, stopping on the first error.

The command operation checks whether the command is internal (flagged during verification) and routes accordingly. Internal commands are handled by the internal command handler, which implements list-all, brief-list, prefix-search, and detail-display. User commands are handled by the user command handler, which looks up the routine from the Dictionary and invokes it.

Routine dispatch checks the variant: the context-free variant invokes the closure with just the verified command, the context-aware variant invokes it with both the shared context and the verified command. If no routine is registered, execution fails with an error.

Context is constructed by wrapping any value meeting the required trait bounds. Retrieval performs downcasting and returns the value in a shared reference-counted wrapper.

### Error Handling

Execution errors arise from: missing routine on a registered command, routine returning an error, or internal command encountering an unexpected state. All are wrapped in the execution error category.

### Compatibility Guarantees

Executor, Context, and Routine are public types. The handler wrapper provides conversion from eight closure signatures supporting combinations of with/without context, with/without command arguments, and returning a value or a result type.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/ca/executor/executor.rs` | Executor struct and dispatch logic |
| source | `src/ca/executor/routine.rs` | Routine enum and Handler wrapper |
| source | `src/ca/executor/context.rs` | Context type-erased container |
| test | `tests/inc/executor/command.rs` | Command execution tests including context |
| test | `tests/inc/executor/program.rs` | Program-level execution tests |
| doc | [feature/005_command_routing.md](../feature/005_command_routing.md) | Internal command routing |
| doc | [feature/006_context_sharing.md](../feature/006_context_sharing.md) | Context sharing feature |
| doc | [invariant/004_help_no_execute.md](../invariant/004_help_no_execute.md) | Internal commands must not trigger routines |
| doc | [invariant/005_routine_required.md](../invariant/005_routine_required.md) | Routine presence contract |
