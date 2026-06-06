# Feature: Terminal Abstraction

### Scope

- **Purpose**: Abstract all terminal operations behind a trait so that production code and tests use the same interface without coupling to real I/O.
- **Responsibility**: Define `TerminalOps` with TTY detection, size query, raw mode, and write operations; provide `RealTerminal` for production and `MockTerminal` for testing.
- **In Scope**: `TerminalOps` trait definition; `RealTerminal` wrapping crossterm; `MockTerminal` simulating terminal state; error paths for NoTty and TerminalTooSmall.
- **Out of Scope**: Color themes, terminal capability detection beyond size and TTY, alternative backends.

### Sources

| File | Relationship |
|------|-------------|
| [src/terminal.rs](../../src/terminal.rs) | `TerminalOps` trait, `RealTerminal`, and `MockTerminal` implementations |

### Tests

| File | Relationship |
|------|-------------|
| [tests/integration_tests.rs](../../tests/integration_tests.rs) | Uses `MockTerminal` for all error path and workflow tests |

### APIs

| File | Relationship |
|------|-------------|
| [../api/001_multiline_input.md](../api/001_multiline_input.md) | `TerminalOps` trait and `MockTerminal` re-exported for callers |

### Features

| File | Relationship |
|------|-------------|
| [001_multiline_text_collection.md](001_multiline_text_collection.md) | Feature that depends on the terminal abstraction |
