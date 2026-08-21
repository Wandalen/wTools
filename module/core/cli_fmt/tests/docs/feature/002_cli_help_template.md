# Feature Test: CLI Help Template

### Scope

- **Purpose**: Verify the behavioral requirements documented in `docs/feature/002_cli_help_template.md`.
- **Responsibility**: Test spec for column alignment, TTY detection, conditional section omission, ExampleEntry.desc annotation rendering, color field defaults, and edge-case inputs.
- **In Scope**: FT-1..FT-43 — column padding, ANSI suppression, section omission, example annotation, minimum-width semantics, color defaults, TTY detection, dependency boundary, usage lines override, arguments section, option groups, backward compatibility, infallibility, edge-case inputs, example declaration order, tagline-usage-line separation, col_gap spacing, cmd_indent leading indent, contiguous padding+gap+description for a genuinely short name (both commands and legacy options) at default settings, option_groups within-group differential padding against the group's own longest entry, floor semantics for `cmd_name_width`/`opt_name_width` (the shared name column grows to the longest entry name, so alignment survives any configured width), and the detail-page template (FT-38..FT-43: golden full-page output, empty-data emptiness, empty-section skip, untitled-section bare block, per-section independent padding, empty-description trailing-whitespace freedom).
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

### FT-31: Custom col_gap produces correct spacing between name column and description

- **Given:** `CliHelpStyle { col_gap: 4, cmd_name_width: 7, tty_detect: false, ..CliHelpStyle::default() }`, data with a command group containing `CommandEntry { name: "cmd-one".into(), desc: "do one thing".into() }`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"    cmd-one    do one thing"` — 4-space indent (default cmd_indent), `"cmd-one"` padded to 7 chars (exact match, no extra spaces), 4 spaces col_gap, then description; contrast with default col_gap=2 which would produce `"    cmd-one  do one thing"`

### FT-32: Custom cmd_indent produces correct leading indent for command lines

- **Given:** `CliHelpStyle { cmd_indent: 2, cmd_name_width: 3, tty_detect: false, ..CliHelpStyle::default() }`, data with a command group containing `CommandEntry { name: "run".into(), desc: "run the app".into() }`
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"  run  run the app"` — 2-space indent (custom cmd_indent), `"run"` padded to 3 chars (exact match), 2-space col_gap (default), then description; contrast with default cmd_indent=4 which would produce `"    run  run the app"`

### FT-33: Padded short command name is immediately contiguous with col_gap and description at default settings

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }` (`cmd_name_width=20`, `col_gap=2`), data with a command group containing `CommandEntry { name: "cmd-one".into(), desc: "do one thing".into() }` — a name shorter than `cmd_name_width` (7 of 20 chars), unlike FT-31/FT-32 which use exact-fit names
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"cmd-one               do one thing"` — `"cmd-one"` padded with 13 trailing spaces to reach `cmd_name_width` (20), immediately followed by 2 more spaces for `col_gap`, immediately followed by the description, all in one contiguous run of 15 spaces; closes the gap between FT-1 (proves padding alone, no gap or description in the same assertion) and FT-31/FT-32 (prove gap+description, but only for names that exactly fill `cmd_name_width` with zero padding spaces) — no existing case proves padding, gap, and description are contiguous for a genuinely short name at default settings

### FT-34: Padded short option name is immediately contiguous with col_gap and description at default settings

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }` (`opt_name_width=18`, `col_gap=2`), data with a legacy `options` vec containing `OptionEntry { name: "dry::bool".into(), desc: "Dry run".into() }` — a name shorter than `opt_name_width` (9 of 18 chars)
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"dry::bool           Dry run"` — `"dry::bool"` padded with 9 trailing spaces to reach `opt_name_width` (18), immediately followed by 2 more spaces for `col_gap`, immediately followed by the description, all in one contiguous run of 11 spaces; mirrors FT-33 for the legacy options path (`emit_options`, structurally identical `{indent}{color}{padded}{reset}{gap}{desc}` formatting to `emit_groups`) — `docs/api/002_help_api.md`'s Column padding paragraph documents commands (step 4) and legacy options (step 6) sharing the identical field-width-plus-col_gap formula, but only the commands side has a contiguity proof (FT-33); FT-8 proves `opt_name_width` padding alone, without col_gap or description in the same assertion — option_groups (step 5) and arguments (step 3) use a self-relative width (the maximum entry name length within that section/group, not a fixed style constant) and so do not share this specific gap class; arguments' own differential padding (short name padded to match a longer sibling) is proven separately by FT-22

### FT-35: option_groups within-group differential padding matches the group's own longest entry, not a global constant

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, data with one `OptionGroup` containing two entries of different name lengths: `OptionEntry { name: "-x".into(), desc: "short flag".into() }` (2 chars) and `OptionEntry { name: "--extended".into(), desc: "long flag".into() }` (10 chars, this group's own longest name)
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains both `"  -x          short flag"` (`"-x"` padded with 8 trailing spaces to reach the group's own max length of 10, immediately followed by 2 more fixed spaces, immediately followed by the description — 10 spaces total) and `"  --extended  long flag"` (`"--extended"` already at the group's max length, needs no padding, only the fixed 2-space gap separates it from the description); closes the gap left by T-A06/FT-19 (proves independence ACROSS two different groups, i.e. two different max-lengths, but not differential padding WITHIN one group) and mirrors FT-22's proof of the identical formula for `arguments` — `docs/api/002_help_api.md`'s Column padding paragraph documents `option_groups` (step 5) and `arguments` (step 3) sharing the self-relative-max-length-plus-fixed-2-space formula; FT-22 already proves it for arguments, this closes the analogous gap for option_groups

### FT-36: cmd_name_width is a floor — the command column grows to the longest name across all groups

- **Given:** `CliHelpStyle { cmd_name_width: 10, tty_detect: false, ..CliHelpStyle::default() }`, data with two command groups: `CommandEntry { name: "a-very-long-command".into(), desc: "do long".into() }` (19 chars, above the floor) in one group and `CommandEntry { name: "short".into(), desc: "do short".into() }` (5 chars) in the other
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"a-very-long-command  do long"` (19-char name needs no padding, only the 2-space `col_gap`) and `"short                do short"` (`"short"` padded with 14 trailing spaces to reach the longest name's length of 19 — not the configured 10 — plus 2 more for `col_gap`, 16 spaces total); the configured `cmd_name_width` acts as a floor only — a name longer than it widens the shared column for all groups, so no configuration can produce a misaligned command column (previously such a name overflowed its own line and broke alignment; FT-5 proves only that a long name is not truncated, with a single entry and therefore no sibling alignment at stake)

### FT-37: opt_name_width is a floor — the options column grows to the longest option name

- **Given:** `CliHelpStyle { opt_name_width: 10, tty_detect: false, ..CliHelpStyle::default() }`, data with two legacy options: `OptionEntry { name: "a-really-long-option".into(), desc: "long opt".into() }` (20 chars, above the floor) and `OptionEntry { name: "dry".into(), desc: "dry run".into() }` (3 chars)
- **When:** `CliHelpTemplate::new(style, data).render()`
- **Then:** output contains `"a-really-long-option  long opt"` (20-char name, no padding, 2-space `col_gap` only) and `"dry                   dry run"` (`"dry"` padded with 17 trailing spaces to reach 20, plus 2 more for `col_gap`, 19 spaces total); mirrors FT-36 for the legacy options path — `opt_name_width` is likewise a floor, extending FT-8's single-entry non-truncation proof with the sibling-alignment guarantee

### FT-38: Detail page renders header, usage, description, sections, and examples byte-exactly

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, a full `DetailPageData` — `label: "Parameter"`, `name: "scope"`, one usage line `.projects scope::<value>`, one description line, a `Type`-less titled section pair (`""`-titled Type/Default block and a `"Possible values"` section), and two examples (one annotated, one bare)
- **When:** `DetailPageTemplate::new(style, data).render()`
- **Then:** output equals the golden string exactly — `"Parameter: scope\n"` header, indented usage line, blank line, description, each non-empty section with per-section padding, `"Examples:"` block with `# ` annotation on the first entry only — byte-for-byte

### FT-39: Fully empty DetailPageData renders the empty string

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, `DetailPageData::default()` (all fields empty)
- **When:** `DetailPageTemplate::new(style, data).render()`
- **Then:** output is exactly `""` — no header, no blank lines, no panic; the template emits nothing it wasn't given

### FT-40: Detail section with empty entries is skipped entirely, title included

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, `DetailPageData` with `sections` containing `DetailSection::new("Empty Section", vec![])` and one populated section
- **When:** `DetailPageTemplate::new(style, data).render()`
- **Then:** output does NOT contain `"Empty Section"`; the populated section renders normally

### FT-41: Detail section with empty title renders entries as a bare block without a header line

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, `DetailPageData` with one section `DetailSection::new("", vec![OptionEntry { name: "Kind", desc: "String" }])`
- **When:** `DetailPageTemplate::new(style, data).render()`
- **Then:** output contains the aligned entry line (`"  Kind  String"`) preceded by a blank line but no `":"`-suffixed header line for that section

### FT-42: Each detail section pads names to its OWN longest entry, independently

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, two sections — one whose longest name is `"a-very-long-name"` (16 chars) alongside `"ab"`, another whose longest name is `"cd"` (2 chars)
- **When:** `DetailPageTemplate::new(style, data).render()`
- **Then:** `"ab"` pads to 16 in its own section (`"  ab                ..."`); `"cd"` pads only to 2 in the other (`"  cd  ..."`) — one section's long name never widens another section's column

### FT-43: Detail entry with empty description renders name-only with no trailing whitespace

- **Given:** `CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() }`, a section entry `OptionEntry { name: "bare", desc: "" }`
- **When:** `DetailPageTemplate::new(style, data).render()`
- **Then:** the entry line is exactly `"  bare"` followed by a newline — no padding spaces, no column gap, no trailing whitespace

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
| `../../../tests/help.rs` | FT-1: `test_column_alignment`; FT-2: `test_no_ansi_codes`, `test_explicit_tty_detect_false`; FT-3: `test_no_options_section`, `test_no_examples_section`; FT-4: `test_example_desc_rendered`; FT-5: `test_name_not_truncated`; FT-6: `test_style_color_defaults`; FT-7: `test_empty_groups`; FT-8: `test_opt_name_not_truncated`; FT-9: `test_single_group_binary_name`; FT-10: `test_tty_detect_true_suppresses_ansi_in_non_tty`; FT-11: `test_no_data_fmt_dependency`; FT-12/FT-13: `test_usage_lines` (T-A01); FT-14/FT-15: `test_arguments_section` (T-A02); FT-16: `test_option_groups_render` (T-A03); FT-17: `test_option_groups_empty_backward_compat` (T-A04); FT-18: `test_option_groups_suppresses_options` (T-A05); FT-19: `test_option_groups_independent_padding` (T-A06); FT-20: `test_cli_help_data_default` (T-A07); T-A08: compile_fail doctest in `src/help.rs`; T-A09: `test_examples_compile` (construction pattern under `#[non_exhaustive]`); FT-21: `test_multiple_usage_lines` (T-B01); FT-22: `test_arguments_multi_entry_padding` (T-B02); FT-23: `test_command_group_empty_entries` (T-B03); FT-24: `test_render_empty_data_infallible` (T-B04); FT-25: `test_example_empty_desc_some_renders_marker` (T-B05); FT-26: `test_option_group_empty_entries_skipped` (T-B06); FT-27: `test_empty_option_group_suppresses_legacy_options` (T-B07); FT-28: `test_arguments_before_groups_in_output` (T-B08); FT-29: `test_examples_declaration_order` (T-B09); FT-30: `test_tagline_blank_line_separator` (T-B10); FT-31: `test_col_gap_custom`; FT-32: `test_cmd_indent_custom`; FT-33: `test_padded_name_contiguous_with_gap_and_description` (T-B13); FT-34: `test_padded_opt_name_contiguous_with_gap_and_description` (T-B14); FT-35: `test_option_group_differential_padding_within_group` (T-B16); FT-36: `test_cmd_column_grows_to_longest_name` (T-B17); FT-37: `test_opt_column_grows_to_longest_name` (T-B18); FT-38: `test_detail_page_golden_full_output` (T-C01); FT-39: `test_detail_page_empty_data_renders_empty` (T-C03); FT-40: `test_detail_page_empty_section_skipped` (T-C06); FT-41: `test_detail_page_untitled_section` (T-C07); FT-42: `test_detail_page_per_section_padding` (T-C08); FT-43: `test_detail_page_empty_desc_no_trailing_whitespace` (T-C09); detail-page contract without FT anchors: T-C02 `test_detail_page_no_ansi_codes`, T-C04 `test_detail_page_name_without_label`, T-C05 `test_detail_page_label_without_name`, T-C10 `test_detail_page_example_desc_parity`, T-C11 `test_detail_section_new_constructor`, T-C12 `prelude_reexports_detail_items::test_prelude_reexports_detail_items`, T-C13 `test_detail_page_usage_lines`; T-C14: compile_fail doctest in `src/help.rs` (`DetailPageData` literal rejected) |
| `../../../examples/basic_usage.rs` | T-A09: `cargo test --examples` compiles and runs the example using `CliHelpData::default()` + field assignment |
