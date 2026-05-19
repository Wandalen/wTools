# Feature: REPL Mode

### Scope

- **Purpose**: Provides an interactive session where multiple commands share archive state across invocations.
- **Responsibility**: Documents REPL startup, command processing loop, and session state behavior.
- **In Scope**: Interactive prompt, multi-command workflows, archive state persistence within session, graceful exit.
- **Out of Scope**: Single-command CLI mode, archive persistence to disk (→ 001).

### Design

When invoked with no arguments, genfile starts a REPL that reads commands line-by-line from stdin. Each command is processed via the same pipeline as CLI mode. Archive state persists across commands within the session via session-scoped storage. The session ends on `quit`, `exit`, or EOF. History navigation is available in interactive sessions.

### Invariants

| File | Relationship |
|------|--------------|
| [`invariant/001_performance.md`](../invariant/001_performance.md) | REPL startup latency target that session mode must respect |

### Sources

| File | Relationship |
|------|--------------|
| [`src/repl.rs`](../../src/repl.rs) | REPL loop implementation |
| [`src/handlers/shared_state.rs`](../../src/handlers/shared_state.rs) | Thread-local archive state backing the session |
| [`src/main.rs`](../../src/main.rs) | Entry point that dispatches to REPL or CLI mode |

### Tests

| File | Relationship |
|------|--------------|
| [`tests/repl_exit_code_bug_test.rs`](../../tests/repl_exit_code_bug_test.rs) | REPL exit code and session behavior tests |
