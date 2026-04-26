# API: Executor

The executor dispatches verified commands to their registered routines, handling both user-defined commands and internal inspection commands.

### Scope

- **Purpose**: Provides the execution layer that converts verified commands into routine invocations.
- **Responsibility**: Documents the Executor interface, routine dispatch, context injection, and internal command handling.
- **In Scope**: program(), command(), exec_command(), exec_internal_command(), Context, Routine variants.
- **Out of Scope**: Command verification (see api/003), help content generation (see feature/004).

### Abstract

The Executor owns an optional Context and dispatches VerifiedCommand values to their routines. It supports two routine variants: one that receives only the command arguments, and one that also receives the shared context. The executor also handles internal commands (dot-suffix and question-mark-suffix) by routing them to built-in inspection logic instead of user-defined routines.

### Operations

The program operation iterates all commands in a verified Program and executes each sequentially, stopping on the first error.

The command operation checks whether the command is internal (flagged during verification) and routes accordingly. Internal commands are handled by exec_internal_command, which implements list-all, brief-list, prefix-search, and detail-display. User commands are handled by exec_command, which looks up the Routine from the Dictionary and invokes it.

Routine dispatch checks the variant: WithoutContext invokes the closure with just the VerifiedCommand, WithContext invokes it with both the Context and VerifiedCommand. If no routine is registered, execution fails with an error.

Context is constructed via Context::new wrapping any value meeting the required bounds. Retrieval via Context::get performs downcasting and returns the value in a shared reference-counted wrapper.

### Error Handling

Execution errors arise from: missing routine on a registered command, routine returning an error, or internal command encountering an unexpected state. All are wrapped in Error::Execution.

### Compatibility Guarantees

Executor, Context, and Routine are public types. The Handler generic wrapper provides From implementations for eight closure signatures supporting combinations of with/without context, with/without command arguments, and returning unit/Result/Infallible.

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
