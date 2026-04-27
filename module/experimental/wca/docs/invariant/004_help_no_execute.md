# Invariant: Help No Execute

Requesting help for a command must never trigger that command's routine. Help display is a read-only inspection operation with no side effects beyond console output.

### Scope

- **Purpose**: Guarantees that help queries are safe and side-effect-free.
- **Responsibility**: Documents the separation between help display and command execution, and the regression test protecting it.
- **In Scope**: Help variants, internal command routing, bug reproducer test.
- **Out of Scope**: Help content formatting (see feature/004), executor dispatch (see api/004).

### Invariant Statement

When a user requests help — whether via the global help variant, a dot-suffix command, or a question-mark-suffix command — the executor must route the request to the help display logic and must not invoke the target command's registered routine. The routine closure must not execute, partially execute, or observe any of its side effects during a help request.

### Enforcement Mechanism

The verifier marks internal commands (dot-suffix, question-mark-suffix) with an internal flag. The executor checks this flag before dispatch: internal commands are routed to the help display handler, while user commands are routed to the routine invocation handler. These are mutually exclusive paths. A dedicated regression test (marked as bug_reproducer) verifies that requesting help for a command with a side-effecting routine does not trigger that routine.

### Violation Consequences

If help triggered routine execution, users could not safely explore available commands without risking unintended mutations, network calls, or other side effects. This would make the help system unreliable and potentially destructive, violating the principle of least surprise.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/ca/executor/executor.rs` | Internal command routing bypass |
| source | `src/ca/help.rs` | Help variant definitions |
| test | `tests/inc/commands_aggregator/help.rs` | Bug reproducer: help must not execute |
| doc | [feature/004_help_system.md](../feature/004_help_system.md) | Help system feature |
| doc | [feature/005_command_routing.md](../feature/005_command_routing.md) | Internal command routing |
| doc | [api/004_executor.md](../api/004_executor.md) | Executor dispatch logic |
