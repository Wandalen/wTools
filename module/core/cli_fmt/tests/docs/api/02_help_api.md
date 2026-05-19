# API Test: Help Template API

### Scope

- **Purpose**: Verify the API contract documented in `docs/api/002_help_api.md`.
- **Responsibility**: Test spec for render infallibility, `CliHelpStyle::default()` field values, column padding semantics, conditional section omission, and `ExampleEntry.desc` annotation rendering.
- **In Scope**: Render infallibility (AP-1), `CliHelpStyle::default()` field values (AP-2), column padding as minimum width (AP-3), section omission when vecs empty (AP-4), `ExampleEntry.desc` Some/None rendering (AP-5).
- **Out of Scope**: Behavioral rationale and style customization — see `tests/docs/feature/02_cli_help_template.md` for feature-level behavioral specs.

### AP-1: CliHelpTemplate::render() is infallible — accepts any valid input without panic

- **Given:** a minimal `CliHelpData` with `binary: "app"`, `tagline: "test"`, one group with one command, empty options, empty examples; `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** returns a `String`; binary name appears in output; no panic

### AP-2: CliHelpStyle::default() produces the documented field values

- **Given:** `CliHelpStyle::default()`
- **When:** each field is read directly
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

### Sources

| File | Relationship |
|------|-------------|
| `../../../src/help.rs` | Implements all public API types and `CliHelpTemplate::render()` under contract |

### Tests

| File | Relationship |
|------|-------------|
| `../../../tests/help.rs` | AP-1: `test_single_group_binary_name`; AP-2: `test_style_default_fields`; AP-3: `test_name_not_truncated`; AP-4: `test_no_options_section`, `test_no_examples_section`; AP-5: `test_example_desc_rendered` |

### APIs

| File | Relationship |
|------|-------------|
| `../../../docs/api/002_help_api.md` | Authoritative API contract for this spec |
