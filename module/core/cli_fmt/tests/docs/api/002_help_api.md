# API Test: Help Template API

### Scope

- **Purpose**: Verify the API contract documented in `docs/api/002_help_api.md`.
- **Responsibility**: Test spec for render infallibility, `CliHelpStyle::default()` field values, column padding semantics, conditional section omission, and `ExampleEntry.desc` annotation rendering.
- **In Scope**: Render infallibility (AP-1), `CliHelpStyle::default()` layout field values (AP-2), column padding as minimum width (AP-3), section omission when vecs empty (AP-4), `ExampleEntry.desc` Some/None rendering (AP-5), `CliHelpStyle::default()` typed color/style field values and `tty_detect` (AP-6), `OptionGroup` struct construction (AP-7), `CliHelpData::default()` constructs with empty Vecs (AP-8), `CliHelpData` struct-literal rejection under `#[non_exhaustive]` (AP-9), prelude re-export surface (AP-10), OptionGroup with empty entries list omitted entirely including header (AP-11).
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

### AP-6: CliHelpStyle::default() produces the documented color/style values and tty_detect

- **Given:** `CliHelpStyle::default()`
- **When:** each color-role field (a `color_tools::DecoratedText` style descriptor) is rendered via `.clone().render()` — exercising the same code path used at real render time, since the field's own `text` is empty — and `tty_detect` is read directly
- **Then:** `color_tagline.clone().render() == "\x1b[1m\x1b[0m"` (bold only); `color_group.clone().render() == "\x1b[1m\x1b[33m\x1b[0m"` (bold + yellow); `color_option.clone().render() == "\x1b[1m\x1b[36m\x1b[0m"` (bold + cyan); `color_example.clone().render() == "\x1b[2m\x1b[0m"` (dim only); `tty_detect == true`. Bold/dim always precede color in a combined role's rendered sequence — see `color_tools::DecoratedText::render()`'s field order. There is no separate `color_reset` field to assert on; the reset is appended automatically by `render()`.

### AP-7: OptionGroup can be constructed with name and entries

- **Given:** `OptionGroup { name: "MY GROUP".into(), entries: vec![OptionEntry { name: "--flag".into(), desc: "A flag".into() }] }`
- **When:** the struct is constructed and placed in a `CliHelpData::default()` with `option_groups` set to `vec![group]` via field assignment
- **Then:** `CliHelpTemplate::new(style, data).render()` returns a String containing `"MY GROUP:"`; no panic

### AP-8: CliHelpData::default() constructs with all Vec fields empty

- **Given:** `CliHelpData::default()`
- **When:** each field is inspected
- **Then:** `usage_lines.is_empty()`; `arguments.is_empty()`; `option_groups.is_empty()`; `groups.is_empty()`; `options.is_empty()`; `examples.is_empty()`; `binary` is an empty string; `tagline` is an empty string; no panic

### AP-9: CliHelpData struct literals from outside the crate fail to compile under #[non_exhaustive]

- **Given:** external crate code attempting `CliHelpData { binary: String::new(), tagline: String::new(), groups: vec![], options: vec![], examples: vec![], usage_lines: vec![], arguments: vec![], option_groups: vec![] }` — a fully-exhaustive struct literal naming every field
- **When:** the crate is compiled
- **Then:** compilation fails with E0639 (`#[non_exhaustive]` blocks struct expressions, including struct update syntax, from outside the defining crate); callers must use `CliHelpData::default()` followed by field assignment instead

### AP-10: cli_fmt::prelude::* re-exports all help-template API items

- **Given:** a consumer crate importing `use cli_fmt::prelude::*;`
- **When:** `CliHelpTemplate`, `CliHelpStyle`, `CliHelpData`, `OptionGroup`, `CommandGroup`, `CommandEntry`, `OptionEntry`, `ExampleEntry` are referenced by their bare names
- **Then:** all resolve without an explicit `cli_fmt::help::` path — the prelude re-export makes the help-template API directly accessible

### AP-11: OptionGroup with an empty entries list is omitted entirely, header included

- **Given:** a `CliHelpData` with `option_groups` containing an `OptionGroup` whose `entries` list is empty
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** the rendered output contains neither the group's `"{name}:"` header nor any entry line for that group — omission is unconditional on entries being empty, not partial (header-only) omission

### APIs

| File | Relationship |
|------|-------------|
| [`../../../docs/api/002_help_api.md`](../../../docs/api/002_help_api.md) | Authoritative API contract for this spec |

### Sources

| File | Relationship |
|------|-------------|
| `../../../src/help.rs` | Implements all public API types and `CliHelpTemplate::render()` under contract |

### Tests

| File | Relationship |
|------|-------------|
| `../../../tests/help.rs` | AP-1: `test_single_group_binary_name`; AP-2: `test_style_default_fields`; AP-3: `test_name_not_truncated`; AP-4: `test_no_options_section`, `test_no_examples_section`; AP-5: `test_example_desc_rendered`; AP-6: `test_style_color_defaults`; AP-7: `test_option_groups_render` (T-A03); AP-8: `test_cli_help_data_default` (T-A07); AP-10: `test_prelude_reexports_help_items` (T-B15); AP-11: `test_option_group_empty_entries_skipped` (T-B06) |
| `../../../src/help.rs` | AP-9: T-A08 (compile_fail doctest): exhaustive external `CliHelpData` literal rejected by `#[non_exhaustive]` |
