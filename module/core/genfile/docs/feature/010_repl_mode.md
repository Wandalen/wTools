# Feature: REPL Mode

### Scope

- **Purpose**: Provides an interactive session where multiple commands share archive state across invocations.
- **Responsibility**: Documents REPL startup, command processing loop, and session state behavior.
- **In Scope**: Interactive prompt, multi-command workflows, archive state persistence within session, graceful exit.
- **Out of Scope**: Single-command CLI mode, archive persistence to disk (→ 001).

### Design

When invoked with no arguments, genfile starts a REPL that reads commands line-by-line from stdin. Each command is processed via the same pipeline as CLI mode. Archive state persists across commands within the session using thread-local storage (a workaround for `ExecutionContext` not yet supporting custom state). The session ends on `quit`, `exit`, or EOF. History navigation is supported when the `enhanced_repl` feature is enabled.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/repl.rs` | REPL loop implementation |
| source | `src/handlers/shared_state.rs` | Thread-local archive state backing the session |
| source | `src/main.rs` | Entry point that dispatches to REPL or CLI mode |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | FR10 in original spec; combined source migrated to feature/ |
