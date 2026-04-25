# Invariant: No Color Styling

### Scope

- **Purpose**: Ensure all format_tools output is plain ASCII text with no embedded control sequences.
- **Responsibility**: States the plain-text output constraint, how it is enforced, and what breaks if violated.
- **In Scope**: Absence of ANSI escape codes, color attributes, terminal cursor control, and bold/italic markers in all formatted output.
- **Out of Scope**: Output written by callers after receiving a formatted string (callers may post-process freely).

### Invariant Statement

No byte sequence produced by format_tools formatting operations contains ANSI escape sequences, terminal color codes, or any control character intended for terminal interpretation. All separators, borders, and labels are plain printable ASCII. Verified by: no color crate dependencies in Cargo.toml; no escape character literals (`\x1b`, `\033`) in source files.

### Enforcement Mechanism

No color crate is listed as a dependency. Absence of color dependencies verified via `cargo tree`. Source files contain no escape character constants. Code review enforces this at contribution time.

### Violation Consequences

Color codes embedded in formatted output corrupt plain-text consumers such as log files, CI output buffers, and file redirections. They also create a hidden dependency on terminal capability detection, which belongs in a separate terminal-handling layer.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| config | `Cargo.toml` | Dependency manifest — absence of color crates enforces this invariant |
| doc | `docs/feature/002_table_formatting.md` | Table formatting constrained by this invariant |
