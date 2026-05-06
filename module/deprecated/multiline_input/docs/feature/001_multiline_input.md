# Feature: Multiline Input

### Scope

- **Purpose**: Terminal multiline text input with interactive editing, ENTER to submit, CTRL+ENTER for newlines.
- **Responsibility**: Navigational hub cross-referencing all source, test, and doc artifacts for this capability.
- **In Scope**: Input collection, key bindings, text buffer editing, validation, visual feedback, three-tier API.
- **Out of Scope**: Terminal widget frameworks (→ external crates), syntax highlighting, clipboard integration, undo/redo.

### Design

The editor enters raw terminal mode, captures key events one at a time, maintains a text buffer, and renders visual feedback on each keystroke. On ENTER the collected text is returned; on ESC or CTRL+C, `None` is returned; on unrecoverable terminal errors, an error is returned.

**Key Bindings:**

| Key | Action |
|-----|--------|
| ENTER | Submit — returns collected text |
| CTRL+ENTER, SHIFT+ENTER | Insert newline |
| ESC, CTRL+C | Cancel — returns None |
| CTRL+D | Submit (alternative) |
| Left/Right | Move cursor left/right |
| Up/Down | Move cursor up/down between lines |
| Home / End | Start/end of current line |
| CTRL+Home / CTRL+End | Start/end of entire text |
| Backspace | Delete character before cursor |
| Delete | Delete character at cursor |

**API Design**: Three tiers scaled to caller complexity — see `pattern/003_progressive_api_disclosure.md`.

**Testability**: All terminal operations abstracted behind a trait — see `pattern/001_trait_based_di.md`.

**Terminal requirement**: Minimum 20 columns × 3 rows — see `invariant/001_terminal_minimum_size.md`.

### Cross-References

| Type   | File                                                    | Responsibility                                                |
|--------|---------------------------------------------------------|---------------------------------------------------------------|
| source | `src/lib.rs`                                            | Public API: `collect`, `collect_validated`, re-exports        |
| source | `src/editor.rs`                                         | Core editor state machine and main loop                       |
| source | `src/buffer.rs`                                         | Text buffer: insertion, deletion, cursor state                |
| source | `src/keys.rs`                                           | Key event parsing and action mapping                          |
| source | `src/render.rs`                                         | Terminal rendering and visual feedback                        |
| source | `src/builder.rs`                                        | Builder pattern: configuration and editor construction        |
| source | `src/terminal.rs`                                       | TerminalOps trait and RealTerminal implementation             |
| test   | `tests/integration_workflows_test.rs`                   | End-to-end workflow coverage (submit, cancel, multiline)      |
| test   | `tests/key_handling_test.rs`                            | Key binding action coverage                                   |
| test   | `tests/buffer_operations_test.rs`                       | Text buffer operation coverage                                |
| test   | `tests/validation_test.rs`                              | Input validation coverage                                     |
| test   | `tests/error_paths_test.rs`                             | Error condition coverage (NoTty, TerminalTooSmall)            |
| test   | `tests/api_surface_test.rs`                             | Public API surface verification                               |
| doc    | `docs/pattern/001_trait_based_di.md`                    | Dependency injection design enabling testability              |
| doc    | `docs/pattern/002_test_double_terminal.md`              | Test double design for deterministic testing                  |
| doc    | `docs/pattern/003_progressive_api_disclosure.md`        | Three-tier API layering design                                |
| doc    | `docs/pattern/004_domain_based_test_organization.md`    | Domain-based test file organization                           |
| doc    | `docs/invariant/001_terminal_minimum_size.md`           | Terminal size requirement enforced at startup                 |
