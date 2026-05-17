# Feature: CLI Help Template

### Scope

- **Purpose**: Provide a typed, configurable template that renders structured CLI help output from two disjoint parameter sets — style and data — producing ANSI-colored, column-aligned text suitable for terminal display.
- **Responsibility**: Document the `CliHelpStyle`, `CliHelpData`, and `CliHelpTemplate` types; the style/data separation rationale; the rendering algorithm; and why this belongs in `cli_fmt` rather than `data_fmt`.
- **In Scope**: `CliHelpStyle` (13 style parameters with defaults); `CliHelpData` (typed sections: binary, tagline, groups, options, examples); `CliHelpTemplate::render() -> String`; ANSI color roles with TTY-conditional output; dependency architecture (parallel with `data_fmt`, no cross-dependency); feature flag `cli_help_template`.
- **Out of Scope**: `data_fmt` `TextFormatter::CliHelp` path (separate consumer, `TableView`-based); unilang `HelpGenerator` (registry-coupled, separate concern); per-command help (unilang pipeline concern); API contract — see `api/002_help_api.md`.

### Design

**Separation principle:** CLI help rendering is domain-specific — it encodes CLI presentation conventions (column alignment, color roles, TTY detection, indent hierarchy). This is not general-purpose data formatting. `data_fmt` operates on `TableView` (untyped row/column structures) and is domain-agnostic. Adding CLI help rendering to `data_fmt` would impose CLI-domain assumptions on a generic library. `cli_fmt` already holds the boundary for CLI-specific utilities (see `docs/invariant/001_architectural_boundary.md`). `CliHelpTemplate` belongs here.

**Dependency architecture:**

```
data_fmt                              cli_fmt
(TextFormatter::CliHelp               (CliHelpTemplate, CliHelpStyle,
 for TableView consumers)              CliHelpData — this feature)
        │                                       │
        └─────── independent ──────────────────┘
                 both used by
              claude_profile
           (imports cli_fmt directly;
            print_usage() → CliHelpTemplate::render())
```

`data_fmt` and `cli_fmt` are parallel crates. Neither depends on the other.

**Style parameters (`CliHelpStyle`):**

| Field | Default | Purpose |
|-------|---------|---------|
| `cmd_indent` | `4` | Left margin for command names |
| `cmd_name_width` | `20` | Minimum column width for command names |
| `grp_indent` | `2` | Left margin for group headers |
| `opt_indent` | `2` | Left margin for option names |
| `opt_name_width` | `18` | Minimum column width for option names |
| `col_gap` | `2` | Gap between name column and description column |
| `example_indent` | `2` | Left margin for example lines |
| `color_tagline` | `"\x1b[1m"` | ANSI code for section headers and the usage line (bold) |
| `color_group` | `"\x1b[33m\x1b[1m"` | ANSI codes for group headers (yellow+bold) |
| `color_option` | `"\x1b[1;36m"` | ANSI code for option names (bold cyan) |
| `color_example` | `"\x1b[2m"` | ANSI code for example lines (dim) |
| `color_reset` | `"\x1b[0m"` | ANSI reset sequence |
| `tty_detect` | `true` | When true, colors active only when stdout is a terminal; when false, always suppress colors |

`CliHelpStyle::default()` produces the same visual result as the hardcoded `print_usage()` in `claude_profile/src/lib.rs`. ANSI codes are active only when `tty_detect = true` and stdout IS a TTY; when `tty_detect = false` or stdout is not a TTY (piped, redirected), all color fields are treated as empty strings.

**Value parameters (`CliHelpData`):**

`CliHelpData` holds five sections: the binary name (e.g. `"clp"`), a one-line tagline, a list of command groups, a list of global option entries, and a list of example entries. Each command group has a name and an ordered list of command entries (name + description). Each option entry has a name and description. Each example entry has an invocation string and an optional inline annotation.

**Template and rendering:**

`CliHelpTemplate` holds one style value and one data value. Constructed via `new(style, data)`; rendered via `render() -> String`, which produces the complete help text. The rendering algorithm:

1. Apply TTY detection: ANSI codes active only when `tty_detect = true` and stdout IS a TTY; zero all color fields otherwise.
2. Emit tagline: `{color_tagline}{binary} — {tagline}{color_reset}`.
3. For each group: emit group header with `grp_indent` and `color_group`, then each entry with `cmd_indent` and left-padded to `cmd_name_width + col_gap`.
4. If options present: emit `Options:` header, each option with `opt_indent` padded to `opt_name_width + col_gap`, colored with `color_option`.
5. If examples present: emit `Examples:` header, each example with `example_indent`, colored with `color_example`. When an example entry has an annotation, append `  # {annotation}` after the invocation.

Column padding: `{name:<width}` where `width = field_name_width + col_gap`. Descriptions that exceed terminal width are not wrapped (out of scope for this feature; wrapping is a separate concern).

**Feature flag:**

`cli_help_template` in `cli_fmt/Cargo.toml` enables this module. Default feature set includes it when `enabled` is active.

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
| `tests/help.rs` | T01–T09: column alignment, TTY detection, section omission, desc annotation |
