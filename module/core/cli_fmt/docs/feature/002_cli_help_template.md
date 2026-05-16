# Feature: CLI Help Template

### Scope

- **Purpose**: Provide a typed, configurable template that renders structured CLI help output from two disjoint parameter sets — style and data — producing ANSI-colored, column-aligned text suitable for terminal display.
- **Responsibility**: Document the `CliHelpStyle`, `CliHelpData`, and `CliHelpTemplate` types; the style/data separation rationale; the rendering algorithm; and why this belongs in `cli_fmt` rather than `data_fmt`.
- **In Scope**: `CliHelpStyle` (13 style parameters with defaults); `CliHelpData` (typed sections: binary, tagline, groups, options, examples); `CliHelpTemplate::render() -> String`; ANSI color roles with TTY-conditional output; dependency architecture (parallel with `data_fmt`, no cross-dependency); feature flag `cli_help_template`.
- **Out of Scope**: `data_fmt` `TextFormatter::CliHelp` path (separate consumer, `TableView`-based); unilang `HelpGenerator` (registry-coupled, separate concern); per-command help (unilang pipeline concern); API contract — see `api/` when added.

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

| Field | Default | Type | Purpose |
|-------|---------|------|---------|
| `cmd_indent` | `4` | `usize` | Left margin for command names |
| `cmd_name_width` | `20` | `usize` | Column width reserved for command name |
| `grp_indent` | `2` | `usize` | Left margin for group headers |
| `opt_indent` | `2` | `usize` | Left margin for option names |
| `opt_name_width` | `18` | `usize` | Column width reserved for option name |
| `col_gap` | `2` | `usize` | Gap between name column and description column |
| `example_indent` | `2` | `usize` | Left margin for example lines |
| `color_tagline` | `"\x1B[1m"` | `&str` | ANSI code for tagline (bold) |
| `color_group` | `"\x1b[33m\x1b[1m"` | `&str` | ANSI codes for group headers (yellow+bold) |
| `color_option` | `"\x1b[1;36m"` | `&str` | ANSI code for option names (bold cyan) |
| `color_example` | `"\x1B[2m"` | `&str` | ANSI code for example lines (dim) |
| `color_reset` | `"\x1B[0m"` | `&str` | ANSI reset sequence |
| `tty_detect` | `true` | `bool` | When true, suppress colors if stdout is not a terminal |

`CliHelpStyle::default()` produces the same visual result as the hardcoded `print_usage()` in `claude_profile/src/lib.rs`. When `tty_detect = true` and stdout is not a TTY (piped, redirected), all color fields are treated as empty strings.

**Value parameters (`CliHelpData`):**

```rust
pub struct CliHelpData
{
  pub binary   : String,             // e.g. "clp"
  pub tagline  : String,             // one-line description
  pub groups   : Vec<CommandGroup>,  // command groups with entries
  pub options  : Vec<OptionEntry>,   // global option list
  pub examples : Vec<ExampleEntry>,  // usage examples
}

pub struct CommandGroup
{
  pub name    : String,
  pub entries : Vec<CommandEntry>,
}

pub struct CommandEntry
{
  pub name : String,
  pub desc : String,
}

pub struct OptionEntry
{
  pub name : String,
  pub desc : String,
}

pub struct ExampleEntry
{
  pub invocation : String,
  pub desc       : Option<String>,
}
```

**Template and rendering:**

```rust
pub struct CliHelpTemplate
{
  style : CliHelpStyle,
  data  : CliHelpData,
}

impl CliHelpTemplate
{
  pub fn new( style : CliHelpStyle, data : CliHelpData ) -> Self;
  pub fn render( &self ) -> String;
}
```

`render()` produces the full help text as a single `String`. The rendering algorithm:

1. Apply TTY detection: if `tty_detect = true` and stdout is not a TTY, zero all color fields.
2. Emit tagline: `{color_tagline}{binary} — {tagline}{color_reset}`.
3. For each group: emit group header with `grp_indent` and `color_group`, then each entry with `cmd_indent` and left-padded to `cmd_name_width + col_gap`.
4. If options present: emit `Options:` header, each option with `opt_indent` padded to `opt_name_width + col_gap`, colored with `color_option`.
5. If examples present: emit `Examples:` header, each example with `example_indent`, colored with `color_example`.

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

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [`../invariant/001_architectural_boundary.md`](../invariant/001_architectural_boundary.md) | Boundary principle placing CLI-specific logic in cli_fmt, not strs_tools |
| source | `src/help.rs` (to create) | CliHelpStyle, CliHelpData, CliHelpTemplate implementation |
| test | `tests/help.rs` (to create) | Rendering correctness, TTY detection, column alignment |
| consumer | `claude_profile/src/lib.rs` | print_usage() replacement using this template |
| task | `claude_tools/task/workspace/140_cli_fmt_cli_help_template.md` | Implementation task for this feature |
| task | `claude_tools/task/claude_profile/141_claude_profile_use_cli_help_template.md` | Integration task: replace print_usage() |
