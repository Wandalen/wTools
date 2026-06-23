# Feature Test: CLI Help Template

### Scope

- **Purpose**: Verify the behavioral requirements documented in `docs/feature/002_cli_help_template.md`.
- **Responsibility**: Test spec for column alignment, TTY detection, conditional section omission, ExampleEntry.desc annotation rendering, color field defaults, and edge-case inputs.
- **In Scope**: FT-1..FT-30 — column padding, ANSI suppression, section omission, example annotation, minimum-width semantics, color defaults, TTY detection, dependency boundary, usage lines override, arguments section, option groups, backward compatibility, infallibility, edge-case inputs, example declaration order, and tagline-usage-line separation.
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

### FT-12: Non-empty usage_lines replaces default "Usage: {binary} <command>" header

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, `CliHelpData::default()` with `usage_lines` set to `vec!["clr <command>".into()]` and `binary` set to `"clr".into()`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"  clr <command>"`; output does NOT contain `"Usage: clr <command>"` (the default form is replaced)

### FT-13: Empty usage_lines preserves default "Usage: {binary} <command>" header

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, `CliHelpData::default()` with `binary` set to `"myapp".into()` (usage_lines defaults to vec![])
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"Usage: myapp"`; default single-line header is present

### FT-14: Non-empty arguments renders Arguments section

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, `CliHelpData::default()` with `arguments` set to `vec![OptionEntry { name: "<MSG>".into(), desc: "Message to send".into() }]`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"Arguments:"`; output contains `"  <MSG>  Message to send"` (single entry, name width=5, 2-space separator between padded name and desc)

### FT-15: Empty arguments omits Arguments section

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, `CliHelpData::default()` (arguments defaults to vec![])
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output does NOT contain `"Arguments:"`

### FT-16: option_groups renders each group as a named section

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, `CliHelpData::default()` with `option_groups` set to `vec![OptionGroup { name: "RUNNER OPTIONS".into(), entries: vec![OptionEntry { name: "--flag".into(), desc: "A flag".into() }] }]`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"RUNNER OPTIONS:"`; output contains `"  --flag  A flag"`

### FT-17: option_groups empty preserves options field as "Options:" section

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, `CliHelpData::default()` with `options` set to `vec![OptionEntry { name: "--opt".into(), desc: "desc".into() }]` (option_groups defaults to vec![])
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"Options:"`; output contains `"  --opt"`

### FT-18: option_groups non-empty suppresses options field

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, `CliHelpData::default()` with `options` set to `vec![OptionEntry { name: "--old".into(), desc: "old".into() }]` and `option_groups` set to `vec![OptionGroup { name: "NEW".into(), entries: vec![OptionEntry { name: "--new".into(), desc: "new".into() }] }]`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"NEW:"`; output contains `"  --new  new"`; output does NOT contain `"--old"`; output does NOT contain `"Options:"`

### FT-19: Each OptionGroup computes column padding from its own entries independently

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, `CliHelpData` with two `OptionGroup` values — group A containing `OptionEntry { name: "--aa", desc: "flag a" }` and `OptionEntry { name: "--bb", desc: "flag b" }`; group B containing `OptionEntry { name: "--longer-name", desc: "a long flag" }`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"  --aa  flag a"` and `"  --bb  flag b"` (group A max_len=4, names are exactly 4 chars, 2-space separator); output contains `"  --longer-name  a long flag"` (group B max_len=13) — group A padding is not inflated by group B's longer name

### FT-20: CliHelpData::default() constructs without panic; all Vec fields are empty

- **Given:** `CliHelpData::default()`
- **When:** each Vec field is inspected
- **Then:** constructs without panic; `usage_lines.is_empty()`; `arguments.is_empty()`; `option_groups.is_empty()`; `groups.is_empty()`; `options.is_empty()`; `examples.is_empty()`

### FT-21: Multiple custom usage_lines all render indented; default header absent

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, `CliHelpData::default()` with `usage_lines` set to 3 entries: `"app <command>"`, `"app --help"`, `"app <command> [options]"`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"  app <command>\n"`, `"  app --help\n"`, `"  app <command> [options]\n"`; output does NOT contain `"Usage: app"` — all three lines rendered individually, default header replaced entirely

### FT-22: Arguments multi-entry padding: column width from longest name

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, `arguments` containing `OptionEntry { name: "<A>", desc: "short arg" }` and `OptionEntry { name: "<LONG-ARGUMENT>", desc: "long arg" }`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"  <A>              short arg"` (3-char name padded to 15 = max); output contains `"  <LONG-ARGUMENT>  long arg"` (15-char name, no extra padding)

### FT-23: CommandGroup with empty entries vec renders group header but no command lines

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, `groups` containing one `CommandGroup { name: "EMPTY GROUP", entries: vec![] }` and one fully populated group
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"EMPTY GROUP"`; output contains commands from the populated group; no panic

### FT-24: Render with entirely empty CliHelpData is infallible

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, `CliHelpData::default()` (all fields at empty defaults)
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** no panic; output is non-empty; output contains `"Usage:"`; output contains `"Commands:"`

### FT-25: ExampleEntry desc=Some("") renders the annotation marker with empty text

- **Given:** `ExampleEntry { invocation: "app cmd", desc: Some("".into()) }`, `tty_detect: false`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** the invocation line contains `"# "` — the annotation marker appears whenever `desc` is `Some`, regardless of whether the inner string is empty

### FT-26: OptionGroup with empty entries vec is silently skipped — no header emitted

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, `option_groups` set to `vec![OptionGroup { name: "EMPTY SECTION", entries: vec![] }]`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output does NOT contain `"EMPTY SECTION:"`; no panic — groups with no entries are silently omitted

### FT-27: OptionGroup vec non-empty suppresses options even if all groups have empty entries

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, `options` containing `OptionEntry { name: "--verbose", ... }`, `option_groups` containing `vec![OptionGroup { name: "EMPTY GROUP", entries: vec![] }]`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output does NOT contain `"Options:"`; output does NOT contain `"--verbose"`; output does NOT contain `"EMPTY GROUP:"`; suppression is based on vec non-emptiness, not whether any group has entries

### FT-28: Arguments section appears before command group entries in output

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, both `arguments` (non-empty) and `groups` (non-empty) present
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** the position of `"Arguments:"` in the output string is less than the position of the first group header — Arguments section renders between the `Commands:` label and the first command group

### FT-29: Multiple examples render in declaration order

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, `CliHelpData::default()` with `examples: vec![ExampleEntry { invocation: "app cmd-a".into(), desc: None }, ExampleEntry { invocation: "app cmd-b".into(), desc: None }]`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** `out.find("app cmd-a").unwrap() < out.find("app cmd-b").unwrap()` — the first declared example appears at a lower byte offset than the second; declaration order is preserved in rendered output

### FT-30: Tagline appears after the usage line, separated by a blank line

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, `CliHelpData::default()` with `binary: "myapp".into()` and `tagline: "My helpful tool".into()`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"My helpful tool"`; a blank line (`"\n\n"`) appears between the usage line (containing `"Usage: myapp"`) and the tagline content — the blank line serves as the structural separator between header and tagline

### Features

| File | Relationship |
|------|-------------|
| [`../../../docs/feature/002_cli_help_template.md`](../../../docs/feature/002_cli_help_template.md) | Authoritative behavioral requirements for this spec |

### Sources

| File | Relationship |
|------|-------------|
| `../../../src/help.rs` | Implements `CliHelpTemplate`, `CliHelpStyle`, `CliHelpData`, and all `emit_*` rendering helpers |

### Tests

| File | Relationship |
|------|-------------|
| `../../../tests/help.rs` | FT-1: `test_column_alignment`; FT-2: `test_no_ansi_codes`, `test_explicit_tty_detect_false`; FT-3: `test_no_options_section`, `test_no_examples_section`; FT-4: `test_example_desc_rendered`; FT-5: `test_name_not_truncated`; FT-6: `test_style_color_defaults`; FT-7: `test_empty_groups`; FT-8: `test_opt_name_not_truncated`; FT-9: `test_single_group_binary_name`; FT-10: `test_tty_detect_true_suppresses_ansi_in_non_tty`; FT-11: `test_no_data_fmt_dependency`; FT-12/FT-13: `test_usage_lines` (T-A01); FT-14/FT-15: `test_arguments_section` (T-A02); FT-16: `test_option_groups_render` (T-A03); FT-17: `test_option_groups_empty_backward_compat` (T-A04); FT-18: `test_option_groups_suppresses_options` (T-A05); FT-19: `test_option_groups_independent_padding` (T-A06); FT-20: `test_cli_help_data_default` (T-A07); T-A08: compile_fail doctest in `src/help.rs`; T-A09: `test_examples_compile` (construction pattern under `#[non_exhaustive]`); FT-21: `test_multiple_usage_lines` (T-B01); FT-22: `test_arguments_multi_entry_padding` (T-B02); FT-23: `test_command_group_empty_entries` (T-B03); FT-24: `test_render_empty_data_infallible` (T-B04); FT-25: `test_example_empty_desc_some_renders_marker` (T-B05); FT-26: `test_option_group_empty_entries_skipped` (T-B06); FT-27: `test_empty_option_group_suppresses_legacy_options` (T-B07); FT-28: `test_arguments_before_groups_in_output` (T-B08); FT-29: `test_examples_declaration_order` (T-B09); FT-30: `test_tagline_blank_line_separator` (T-B10) |
| `../../../examples/basic_usage.rs` | T-A09: `cargo test --examples` compiles and runs the example using `CliHelpData::default()` + field assignment |
