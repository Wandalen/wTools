# Invariant: Terminal Minimum Size

### Scope

- **Purpose**: Ensures the editor always has sufficient terminal space to render without visual corruption.
- **Responsibility**: States the minimum terminal dimensions, the enforcement point, and error behavior.
- **In Scope**: Minimum width and height values, enforcement timing, error type and fields.
- **Out of Scope**: Render layout calculations (→ source comments), dynamic resize handling (→ identified test gap).

### Invariant Statement

The terminal must be at least 20 columns wide and 3 rows tall for the editor to operate. These minimum dimensions are required to render the prompt line, at least one input line, and the status line without overlap or truncation.

### Enforcement Mechanism

Checked during editor startup after entering raw mode. If either dimension is below the minimum, the editor returns immediately with a terminal-too-small error carrying the actual and minimum dimensions, before any rendering occurs.

Known limitation: the check currently occurs inside the render path rather than before raw mode is entered, causing a brief cursor-hide artifact on under-sized terminals. The invariant outcome (error returned, no input accepted) is correct regardless.

### Violation Consequences

The editor returns an error immediately without accepting any input. No text is written to the terminal except the cursor hide/show sequence. Callers receive actual dimensions alongside minimum dimensions (`min_width: 20`, `min_height: 3`) to construct a helpful user-facing error message.

### Cross-References

| Type   | File                                        | Responsibility                                         |
|--------|---------------------------------------------|--------------------------------------------------------|
| source | `src/render.rs`                             | Enforcement logic and minimum size constants           |
| source | `src/error.rs`                              | TerminalTooSmall error type definition                 |
| test   | `tests/error_paths_test.rs`                 | Boundary tests: below minimum, at minimum (20×3)       |
| doc    | `docs/feature/001_multiline_input.md`       | Feature governed by this invariant                     |
