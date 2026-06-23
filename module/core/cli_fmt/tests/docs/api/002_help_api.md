# API Test: Help Template API

### Scope

- **Purpose**: Verify the API contract documented in `docs/api/002_help_api.md`.
- **Responsibility**: Test spec for render infallibility, `CliHelpStyle::default()` field values, column padding semantics, conditional section omission, and `ExampleEntry.desc` annotation rendering.
- **In Scope**: Render infallibility (AP-1), `CliHelpStyle::default()` layout field values (AP-2), column padding as minimum width (AP-3), section omission when vecs empty (AP-4), `ExampleEntry.desc` Some/None rendering (AP-5), `CliHelpStyle::default()` color field values and `tty_detect` (AP-6), `OptionGroup` struct construction (AP-7), `CliHelpData::default()` constructs with empty Vecs (AP-8).
- **Out of Scope**: Behavioral rationale and style customization — see `tests/docs/feature/002_cli_help_template.md` for feature-level behavioral specs.

### AP-1: CliHelpTemplate::render() is infallible — accepts any valid input without panic

- **Given:** a minimal `CliHelpData` with `binary: "app"`, `tagline: "test"`, one group with one command, empty options, empty examples; `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** returns a `String`; binary name appears in output; no panic

### AP-2: CliHelpStyle::default() produces the documented layout field values

- **Given:** `CliHelpStyle::default()`
- **When:** each layout field is read directly
- **Then:** `cmd_indent == 4`; `cmd_name_width == 20`; `grp_indent == 2`; `opt_indent == 2`; `opt_name_width == 18`; `col_gap == 2`; `example_indent == 2`

### AP-3: Column padding is a minimum — names wider than configured width are not truncated

- **Given:** `CliHelpStyle { cmd_name_width: 10, tty_detect: false, ..CliHelpStyle::default() }`, a command entry with an 11-character name `"eleven-char"`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** rendered output contains `"eleven-char"` intact (not truncated to 10 characters)

### AP-4: Options and Examples sections are omitted when their vecs are empty

- **Given:** `CliHelpData` with `options: vec![]` and `examples: vec![]`; `tty_detect: false`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** rendered output does not contain `"Options:"`; rendered output does not contain `"Examples:"`

### AP-5: ExampleEntry.desc = Some renders inline annotation; None renders bare invocation

- **Given:** `ExampleEntry { invocation: "myapp cmd-one", desc: Some("run one") }` and `ExampleEntry { invocation: "myapp cmd-two", desc: None }`; `tty_detect: false`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** line containing `"myapp cmd-one"` includes `"# run one"`; line containing `"myapp cmd-two"` contains no `'#'` character

### AP-6: CliHelpStyle::default() produces the documented color field values and tty_detect

- **Given:** `CliHelpStyle::default()`
- **When:** each color field and `tty_detect` are read directly
- **Then:** `color_tagline == "\x1b[1m"`; `color_group == "\x1b[33m\x1b[1m"`; `color_option == "\x1b[1;36m"`; `color_example == "\x1b[2m"`; `color_reset == "\x1b[0m"`; `tty_detect == true`

### AP-7: OptionGroup can be constructed with name and entries

- **Given:** `OptionGroup { name: "MY GROUP".into(), entries: vec![OptionEntry { name: "--flag".into(), desc: "A flag".into() }] }`
- **When:** the struct is constructed and placed in a `CliHelpData::default()` with `option_groups` set to `vec![group]` via field assignment
- **Then:** `CliHelpTemplate::new(style, data).render()` returns a String containing `"MY GROUP:"`; no panic

### AP-8: CliHelpData::default() constructs with all Vec fields empty

- **Given:** `CliHelpData::default()`
- **When:** each field is inspected
- **Then:** `usage_lines.is_empty()`; `arguments.is_empty()`; `option_groups.is_empty()`; `groups.is_empty()`; `options.is_empty()`; `examples.is_empty()`; `binary` is an empty string; `tagline` is an empty string; no panic

### APIs

| File | Relationship |
|------|-------------|
| `../../../docs/api/002_help_api.md` | Authoritative API contract for this spec |

### Sources

| File | Relationship |
|------|-------------|
| `../../../src/help.rs` | Implements all public API types and `CliHelpTemplate::render()` under contract |

### Tests

| File | Relationship |
|------|-------------|
| `../../../tests/help.rs` | AP-1: `test_single_group_binary_name`; AP-2: `test_style_default_fields`; AP-3: `test_name_not_truncated`; AP-4: `test_no_options_section`, `test_no_examples_section`; AP-5: `test_example_desc_rendered`; AP-6: `test_style_color_defaults`; AP-7: `test_option_groups_render` (T-A03); AP-8: `test_cli_help_data_default` (T-A07) |
| `../../../src/help.rs` | AC-10 (compile_fail doctest — T-A08): exhaustive external `CliHelpData` literal rejected by `#[non_exhaustive]` |
