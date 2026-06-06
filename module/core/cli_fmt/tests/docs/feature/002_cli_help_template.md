# Feature Test: CLI Help Template

### Scope

- **Purpose**: Verify the behavioral requirements documented in `docs/feature/002_cli_help_template.md`.
- **Responsibility**: Test spec for column alignment, TTY detection, conditional section omission, ExampleEntry.desc annotation rendering, color field defaults, and edge-case inputs.
- **In Scope**: Column padding to configured widths (FT-1), ANSI suppression via `tty_detect=false` (FT-2), empty-vec section omission (FT-3), `ExampleEntry.desc` Some/None rendering (FT-4), `cmd_name_width` as minimum padding not truncation limit (FT-5), color field default values (FT-6), empty groups vec rendering (FT-7), `opt_name_width` as minimum padding not truncation limit (FT-8), header format rendering (FT-9), tty_detect=true in non-TTY suppresses colors (FT-10), data_fmt crate absent from dependencies (FT-11).
- **Out of Scope**: Style customization beyond default values; description line wrapping (out of scope for this feature).

### FT-1: Command and option names padded to configured column widths

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }` (`cmd_name_width=20`, `opt_name_width=18`), data with command `"cmd-one"` and option `"dry::bool"`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"cmd-one             "` (padded to 20 chars); output contains `"dry::bool         "` (padded to 18 chars); no `"\x1b["` sequences present

### FT-2: All ANSI codes suppressed when tty_detect is false

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, any valid `CliHelpData`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** rendered string contains no `"\x1b["` escape sequences anywhere

### FT-3: Options and Examples sections omitted when their vecs are empty

- **Given:** `CliHelpData` with `options: vec![]` and `examples: vec![]`, `tty_detect=false`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output does not contain `"Options:"`; output does not contain `"Examples:"`

### FT-4: ExampleEntry.desc=Some renders inline annotation; desc=None renders no annotation

- **Given:** two `ExampleEntry` values — `{ invocation: "myapp cmd-one", desc: Some("run one") }` and `{ invocation: "myapp cmd-two", desc: None }`; `tty_detect=false`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** line containing `"myapp cmd-one"` includes `"# run one"`; line containing `"myapp cmd-two"` contains no `'#'` character

### FT-5: cmd_name_width is a minimum padding width, not a truncation limit

- **Given:** `CliHelpStyle { cmd_name_width: 10, tty_detect: false, ..CliHelpStyle::default() }`, a command with name `"eleven-char"` (11 visible chars)
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"eleven-char"` intact — the name is not clipped to 10 chars

### FT-6: CliHelpStyle::default() color fields and tty_detect match the API contract

- **Given:** `CliHelpStyle::default()`
- **When:** each color field and `tty_detect` are read directly
- **Then:** `color_tagline == "\x1b[1m"`; `color_group == "\x1b[33m\x1b[1m"`; `color_option == "\x1b[1;36m"`; `color_example == "\x1b[2m"`; `color_reset == "\x1b[0m"`; `tty_detect == true`

### FT-7: Empty groups vec — render succeeds and no group content appears

- **Given:** `CliHelpData` with `groups: vec![]`, one `OptionEntry`, `tty_detect: false`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** render returns a non-empty `String` without panic; output contains binary name and tagline; output contains no group-specific header text

### FT-8: opt_name_width is a minimum padding width, not a truncation limit

- **Given:** `CliHelpStyle { opt_name_width: 10, tty_detect: false, ..CliHelpStyle::default() }`, an `OptionEntry` with `name: "format::json"` (12 visible chars)
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"format::json"` intact — the name is not clipped to 10 characters

### FT-9: Header section renders "Usage: {binary}" and "Commands:" with group and command visible

- **Given:** `CliHelpData` with `binary: "myapp"`, one group `"Cmds"` containing command `"run"`, `tty_detect=false`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"Usage: myapp"`; output contains `"Commands:"`; output contains `"Cmds"`; output contains `"run"`; no `"\x1b["` sequences present

### FT-10: tty_detect=true with non-TTY stdout suppresses ANSI codes

- **Given:** `CliHelpStyle::default()` (`tty_detect=true`), any valid `CliHelpData`; test process stdout is not a TTY
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** rendered string contains no `"\x1b["` escape sequences — colors suppressed because stdout is not a TTY even though `tty_detect=true`

### FT-11: data_fmt crate is not a dependency of cli_fmt

- **Given:** `cli_fmt/Cargo.toml`
- **When:** dependency list is inspected
- **Then:** `data_fmt` does not appear as a dependency — the feature uses only strs_tools primitives, not the higher-level data_fmt formatter

### Sources

| File | Relationship |
|------|-------------|
| `../../../src/help.rs` | Implements `CliHelpTemplate`, `CliHelpStyle`, `CliHelpData`, and all `emit_*` rendering helpers |

### Tests

| File | Relationship |
|------|-------------|
| `../../../tests/help.rs` | FT-1: `test_column_alignment`; FT-2: `test_no_ansi_codes`, `test_explicit_tty_detect_false`; FT-3: `test_no_options_section`, `test_no_examples_section`; FT-4: `test_example_desc_rendered`; FT-5: `test_name_not_truncated`; FT-6: `test_style_color_defaults`; FT-7: `test_empty_groups`; FT-8: `test_opt_name_not_truncated`; FT-9: `test_single_group_binary_name`; FT-10: `test_tty_detect_true_suppresses_ansi_in_non_tty`; FT-11: `test_no_data_fmt_dependency` |

### Features

| File | Relationship |
|------|-------------|
| `../../../docs/feature/002_cli_help_template.md` | Authoritative behavioral requirements for this spec |
