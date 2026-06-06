# Feature: Multiline Text Collection

### Scope

- **Purpose**: Collect multiline text input from the terminal with intuitive key bindings for submit, newline insertion, and cancel.
- **Responsibility**: Handle raw terminal mode, render input with optional line numbers and status, process key events, and return the collected text or a cancellation signal.
- **In Scope**: ENTER to submit, CTRL+ENTER/SHIFT+ENTER to insert newline, ESC/CTRL+C to cancel; cursor navigation (arrows, Home/End, CTRL+Home/End); backspace and delete; optional prompt, line numbers, and status bar; min-length validation.
- **Out of Scope**: Syntax highlighting, history recall, undo/redo, copy/paste from clipboard, non-TTY input sources.

### Sources

| File | Relationship |
|------|-------------|
| [src/lib.rs](../../src/lib.rs) | `collect()` function and `Builder` public API |
| [src/editor.rs](../../src/editor.rs) | Core event loop and key dispatch |
| [src/buffer.rs](../../src/buffer.rs) | Text buffer management |
| [src/render.rs](../../src/render.rs) | Terminal rendering logic |
| [src/keys.rs](../../src/keys.rs) | Key binding definitions |
| [src/builder.rs](../../src/builder.rs) | `Builder` configuration API |

### Tests

| File | Relationship |
|------|-------------|
| [tests/integration_tests.rs](../../tests/integration_tests.rs) | Integration tests covering submit, cancel, newline, and error paths |

### APIs

| File | Relationship |
|------|-------------|
| [../api/001_multiline_input.md](../api/001_multiline_input.md) | Public function and type signatures |

### Invariants

| File | Relationship |
|------|-------------|
| [../invariant/001_raw_mode_cleanup.md](../invariant/001_raw_mode_cleanup.md) | Raw mode must be restored on all exit paths |

### Features

| File | Relationship |
|------|-------------|
| [002_terminal_abstraction.md](002_terminal_abstraction.md) | Dependency-injected terminal backend enabling testability |
