# API: Multiline Input

### Scope

- **Purpose**: Document the public API surface for collecting multiline text input from a terminal.
- **Responsibility**: Specify the signatures, return types, and behavioral contracts of `collect()`, `Builder`, and `TerminalOps`.
- **In Scope**: `collect()` free function; `Builder` with all configuration methods; `TerminalOps` trait and `MockTerminal` for testing.
- **Out of Scope**: Internal rendering, buffer management, and key dispatch internals.

### Public Interface

`collect(prompt: impl Into<String>) -> Result<Option<String>, Error>` — Collect multiline input with the given prompt. Returns `Ok(Some(text))` on submit, `Ok(None)` on cancel (ESC/CTRL+C), or `Err` on terminal error.

`Builder::new() -> Builder` — Create a new builder with default configuration.
`Builder::prompt(impl Into<String>) -> Builder` — Set the prompt string.
`Builder::min_length(usize) -> Builder` — Require a minimum number of characters before submit.
`Builder::show_line_numbers(bool) -> Builder` — Toggle line number display.
`Builder::show_status(bool) -> Builder` — Toggle status bar display.
`Builder::build() -> Editor` — Build the configured editor.
`Editor::collect() -> Result<Option<String>, Error>` — Run the editor and collect input.

### Sources

| File | Relationship |
|------|-------------|
| [src/lib.rs](../../src/lib.rs) | `collect()` function and re-exports |
| [src/builder.rs](../../src/builder.rs) | `Builder` type implementation |
| [src/terminal.rs](../../src/terminal.rs) | `TerminalOps` trait and `MockTerminal` |
| [src/error.rs](../../src/error.rs) | `Error` type definition |

### Features

| File | Relationship |
|------|-------------|
| [../feature/001_multiline_text_collection.md](../feature/001_multiline_text_collection.md) | Feature implementing this API |
| [../feature/002_terminal_abstraction.md](../feature/002_terminal_abstraction.md) | Terminal abstraction underlying `TerminalOps` |

### Invariants

| File | Relationship |
|------|-------------|
| [../invariant/001_raw_mode_cleanup.md](../invariant/001_raw_mode_cleanup.md) | Raw mode restoration guarantee for all `collect()` return paths |
