# Feature: CLI Help Template

### Scope

- **Purpose**: Provide a typed, configurable template that renders structured CLI help output from two disjoint parameter sets — style and data — producing ANSI-colored, column-aligned text suitable for terminal display.
- **Responsibility**: Document style/data separation, multi-section content model, backward compatibility semantics, non-exhaustive enforcement, dependency architecture, and crate boundary rationale.
- **In Scope**: Style configuration (13 layout and color parameters); structured content model (binary, tagline, groups, options, examples, usage lines, arguments, option groups); named option groups with per-group column padding; TTY-conditional ANSI output; backward-compatible options field; non-exhaustive content structure; dependency architecture (parallel with data_fmt); feature flag.
- **Out of Scope**: Type field details and rendering procedure — see `api/002_help_api.md`; data_fmt table-based help path; per-command help (unilang pipeline concern).

### Design

**Separation principle:** CLI help rendering is domain-specific — it encodes CLI presentation conventions (column alignment, color roles, TTY detection, indent hierarchy). This is not general-purpose data formatting. The data_fmt crate operates on untyped row/column structures and is domain-agnostic. Adding CLI help rendering to data_fmt would impose CLI-domain assumptions on a generic library. cli_fmt already holds the boundary for CLI-specific utilities (see `docs/invariant/001_architectural_boundary.md`). The help template belongs here.

**Dependency architecture:** data_fmt and cli_fmt are parallel crates — neither depends on the other. Both are consumed independently by downstream applications. This prevents coupling between domain-specific CLI rendering and generic data formatting.

**Style/data separation:** The template splits configuration into two independent parameter sets. Style parameters (13 fields) control layout and color — indents, column widths, gaps, ANSI color codes, and TTY detection. Data parameters hold structured content — binary name, tagline, command groups, global options, and usage examples. This separation allows the same content to be rendered with different visual styles without rebuilding the data, and vice versa.

**TTY-conditional output:** Colors are active only when the TTY detection flag is enabled and stdout is a terminal. When output is piped or redirected, all color codes are suppressed. This follows CLI convention for machine-readable output.

**Column padding:** Name columns use minimum-width padding — names shorter than the configured width are padded; names longer are not truncated. This ensures alignment across entries without clipping long names.

**Conditional sections:** Options and Examples sections are omitted entirely when their content lists are empty, producing cleaner output for simple tools.

**Usage line override:** When `usage_lines` is non-empty, each entry is emitted on its own indented line, replacing the default `"Usage: {binary} <command>"` emission. When `usage_lines` is empty the original single-line form is preserved — callers that do not set this field see no change.

**Arguments section:** When `arguments` is non-empty, an `Arguments:` section is emitted after the header block (between the `Commands:` label and command group entries), using content-driven column padding. An empty list omits the section entirely.

**Option groups:** `option_groups` holds named sections rendered independently between Commands and the legacy `options` list. Each group computes column padding from its own entries only — longer entries in one group do not widen narrower groups.

**Backward compatibility:** When `option_groups` is empty and `options` is non-empty, the legacy `Options:` section renders unchanged. When `option_groups` is non-empty, the `options` field is suppressed — callers using named groups replace the flat list entirely. Callers that set only `options` and leave `option_groups` empty are fully unaffected.

**Non-exhaustive data structure:** The content structure is marked non-exhaustive. Callers outside the crate cannot construct it with a struct literal; they must use the default constructor followed by field assignment. This is a compile-time enforcement of API extensibility.

**Feature flag:** The `cli_help_template` feature flag enables this module (declared as `["std"]` dependency). Included in the default feature set when the crate is enabled.

For complete type definitions, field defaults, and the rendering procedure, see [`api/002_help_api.md`](../api/002_help_api.md).

### Acceptance Criteria

- **AC-1**: `CliHelpStyle::default()` produces identical column widths and indents to the hardcoded `print_usage()` in `claude_profile/src/lib.rs` (cmd_indent=4, cmd_name_width=20, grp_indent=2, opt_indent=2, opt_name_width=18, col_gap=2, example_indent=2, same ANSI color codes).
- **AC-2**: `CliHelpTemplate::render()` with `CliHelpData` built from the same data as `print_usage()` produces byte-identical output (TTY mode; colors active).
- **AC-3**: When `tty_detect = true` and stdout is a pipe, `render()` returns plain text with no ANSI escape sequences.
- **AC-4**: `cli_fmt` has no dependency on `data_fmt`; grep for `data_fmt` in `cli_fmt/Cargo.toml` returns empty.
- **AC-5**: After `claude_profile` replaces `print_usage()` with `CliHelpTemplate::render()`, all help output integration tests (IT-1..IT-12 from task 128) pass without modification.
- **AC-6**: `cli_fmt` compiles with `RUSTFLAGS="-D warnings" cargo check` and zero warnings.
- **AC-7**: `usage_lines: vec!["clr <command>".into()]` → render output contains `"  clr <command>"`; empty `usage_lines` → output contains `"Usage: "` followed by the binary name (default behavior preserved).
- **AC-8**: `arguments: vec![OptionEntry { name: "<MSG>".into(), desc: "Message to send".into() }]` → output contains `"  <MSG>  Message to send"`; empty `arguments` → `"Arguments:"` does not appear in output.
- **AC-9**: `option_groups: vec![OptionGroup { name: "RUNNER OPTIONS".into(), entries: vec![...] }]` → output contains `"RUNNER OPTIONS:"`; two groups with differing name lengths → each group pads names to its own `max(name.len())` independently of the other.
- **AC-10**: An exhaustive external struct literal on `CliHelpData` listing all fields fails to compile (`#[non_exhaustive]` enforced); struct update syntax also fails to compile from outside the crate (E0639); callers construct instances via `CliHelpData::default()` followed by field assignment.

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
| `tests/help.rs` | T01–T14, T-A01–T-A09: column alignment, TTY detection, section omission, desc annotation, color defaults, edge cases, data_fmt absence, usage_lines, arguments, option_groups, backward compat, per-group padding, CliHelpData::default() |
