# Invariant: Raw Mode Cleanup

### Scope

- **Purpose**: Guarantee that the terminal is never left in raw mode after a `collect()` call returns, regardless of whether the call succeeded, was cancelled, or returned an error.
- **Responsibility**: Ensure every exit path from the editor loop — submit, cancel, error — restores the terminal to its original cooked mode before returning.
- **In Scope**: All `collect()` return paths; `disable_raw_mode` call in error and cancel branches.
- **Out of Scope**: Recovery from process kill signals; behavior when `disable_raw_mode` itself fails.

### Enforcement

The editor loop calls `terminal.disable_raw_mode()` in every return branch. A `RealTerminal` maps this directly to `crossterm::terminal::disable_raw_mode()`. `MockTerminal` tracks raw mode state for test assertion.

### Sources

| File | Relationship |
|------|-------------|
| [src/editor.rs](../../src/editor.rs) | Raw mode enable/disable call sites in the event loop |
| [src/terminal.rs](../../src/terminal.rs) | `TerminalOps::disable_raw_mode` implementation for real and mock terminal |

### Features

| File | Relationship |
|------|-------------|
| [../feature/001_multiline_text_collection.md](../feature/001_multiline_text_collection.md) | Feature that requires this invariant |

### APIs

| File | Relationship |
|------|-------------|
| [../api/001_multiline_input.md](../api/001_multiline_input.md) | `collect()` is the API surface where this invariant applies |
