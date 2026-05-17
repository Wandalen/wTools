# Feature Test: CLI Help Template

### Scope

- **Purpose**: Verify the behavioral requirements documented in `docs/feature/002_cli_help_template.md`.
- **Responsibility**: Test spec for column alignment, TTY detection, conditional section omission, and ExampleEntry.desc annotation rendering.
- **In Scope**: Column padding to configured widths (FT-1), ANSI suppression via `tty_detect=false` (FT-2), empty-vec section omission (FT-3), `ExampleEntry.desc` Some/None rendering (FT-4), `cmd_name_width` as minimum padding not truncation limit (FT-5).
- **Out of Scope**: Style customization beyond default values; description line wrapping (out of scope for this feature).

### FT-1: Command and option names padded to configured column widths

- **Given:** `CliHelpStyle::default()` (`cmd_name_width=20`, `opt_name_width=18`, `tty_detect=false`), data with command `"cmd-one"` and option `"dry::bool"`
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

### Sources

| File | Relationship |
|------|-------------|
| `../../../src/help.rs` | Implements `CliHelpTemplate`, `CliHelpStyle`, `CliHelpData`, and all `emit_*` rendering helpers |

### Tests

| File | Relationship |
|------|-------------|
| `../../../tests/help.rs` | FT-1: `test_column_alignment`; FT-2: `test_no_ansi_codes`, `test_explicit_tty_detect_false`; FT-3: `test_no_options_section`, `test_no_examples_section`; FT-4: `test_example_desc_rendered`; FT-5: `test_name_not_truncated` |

### Features

| File | Relationship |
|------|-------------|
| `../../../docs/feature/002_cli_help_template.md` | Authoritative behavioral requirements for this spec |
