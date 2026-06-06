# Feature: CLI Help Template

### Scope

- **Purpose**: Provide a typed, configurable template that renders structured CLI help output from two disjoint parameter sets — style and data — producing ANSI-colored, column-aligned text suitable for terminal display.
- **Responsibility**: Document the style/data separation rationale, dependency architecture, and why CLI help rendering belongs in cli_fmt rather than data_fmt.
- **In Scope**: Style configuration (13 layout and color parameters); structured content model (binary name, tagline, command groups, options, examples); TTY-conditional ANSI output; dependency architecture (parallel with data_fmt, no cross-dependency); feature flag.
- **Out of Scope**: Type field details and rendering procedure — see `api/002_help_api.md`; data_fmt table-based help path; per-command help (unilang pipeline concern).

### Design

**Separation principle:** CLI help rendering is domain-specific — it encodes CLI presentation conventions (column alignment, color roles, TTY detection, indent hierarchy). This is not general-purpose data formatting. The data_fmt crate operates on untyped row/column structures and is domain-agnostic. Adding CLI help rendering to data_fmt would impose CLI-domain assumptions on a generic library. cli_fmt already holds the boundary for CLI-specific utilities (see `docs/invariant/001_architectural_boundary.md`). The help template belongs here.

**Dependency architecture:** data_fmt and cli_fmt are parallel crates — neither depends on the other. Both are consumed independently by downstream applications. This prevents coupling between domain-specific CLI rendering and generic data formatting.

**Style/data separation:** The template splits configuration into two independent parameter sets. Style parameters (13 fields) control layout and color — indents, column widths, gaps, ANSI color codes, and TTY detection. Data parameters hold structured content — binary name, tagline, command groups, global options, and usage examples. This separation allows the same content to be rendered with different visual styles without rebuilding the data, and vice versa.

**TTY-conditional output:** Colors are active only when the TTY detection flag is enabled and stdout is a terminal. When output is piped or redirected, all color codes are suppressed. This follows CLI convention for machine-readable output.

**Column padding:** Name columns use minimum-width padding — names shorter than the configured width are padded; names longer are not truncated. This ensures alignment across entries without clipping long names.

**Conditional sections:** Options and Examples sections are omitted entirely when their content lists are empty, producing cleaner output for simple tools.

**Feature flag:** The `cli_help_template` feature flag enables this module. Included in the default feature set when the crate is enabled.

For complete type definitions, field defaults, and the rendering procedure, see [`api/002_help_api.md`](../api/002_help_api.md).

### Acceptance Criteria

- **AC-1**: `CliHelpStyle::default()` produces identical column widths and indents to the hardcoded `print_usage()` in `claude_profile/src/lib.rs` (cmd_indent=4, cmd_name_width=20, grp_indent=2, opt_indent=2, opt_name_width=18, col_gap=2, example_indent=2, same ANSI color codes).
- **AC-2**: `CliHelpTemplate::render()` with `CliHelpData` built from the same data as `print_usage()` produces byte-identical output (TTY mode; colors active).
- **AC-3**: When `tty_detect = true` and stdout is a pipe, `render()` returns plain text with no ANSI escape sequences.
- **AC-4**: `cli_fmt` has no dependency on `data_fmt`; grep for `data_fmt` in `cli_fmt/Cargo.toml` returns empty.
- **AC-5**: After `claude_profile` replaces `print_usage()` with `CliHelpTemplate::render()`, all help output integration tests (IT-1..IT-12 from task 128) pass without modification.
- **AC-6**: `cli_fmt` compiles with `RUSTFLAGS="-D warnings" cargo check` and zero warnings.

### APIs

| File | Relationship |
|------|-------------|
| [`../api/002_help_api.md`](../api/002_help_api.md) | Public interface contract for this feature |

### Invariants

| File | Relationship |
|------|-------------|
| [`../invariant/001_architectural_boundary.md`](../invariant/001_architectural_boundary.md) | Boundary principle placing CLI rendering in cli_fmt, not strs_tools |

### Sources

| File | Relationship |
|------|-------------|
| `src/help.rs` | Implementation of CliHelpStyle, CliHelpData, and CliHelpTemplate |

### Tests

| File | Relationship |
|------|-------------|
| `tests/help.rs` | T01–T14: column alignment, TTY detection, section omission, desc annotation, color defaults, edge cases, data_fmt absence |
