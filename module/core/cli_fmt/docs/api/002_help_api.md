# API: Help Template Module

### Scope

- **Purpose**: Document the public interface for the CLI help template renderer in `cli_fmt`.
- **Responsibility**: Reference for all public types, their fields, and the rendering entry point.
- **In Scope**: `CliHelpStyle`, `CliHelpData`, `OptionGroup`, `CommandGroup`, `CommandEntry`, `OptionEntry`, `ExampleEntry`, `CliHelpData::default()`, and `CliHelpTemplate::render()`.
- **Out of Scope**: Behavioral rationale and design decisions — see `feature/002_cli_help_template.md`.

### Abstract

The help template API provides a typed, style-parameterized renderer for CLI help text.
Callers construct a `CliHelpStyle` (visual parameters) and a `CliHelpData` (content), pass
both to `CliHelpTemplate::new()`, and call `render()` to obtain the complete help string.

`render()` is infallible — it performs no I/O beyond a single TTY probe and cannot return
an error or panic.

### Types

**`CliHelpStyle`** — visual and layout parameters for rendering. All fields are public.

| Field | Type | Default | Purpose |
|-------|------|---------|---------|
| `cmd_indent` | `usize` | `4` | Left margin before command names |
| `cmd_name_width` | `usize` | `20` | Minimum column width for command names |
| `grp_indent` | `usize` | `2` | Left margin before group headers |
| `opt_indent` | `usize` | `2` | Left margin before option names |
| `opt_name_width` | `usize` | `18` | Minimum column width for option names |
| `col_gap` | `usize` | `2` | Gap between name column and description column |
| `example_indent` | `usize` | `2` | Left margin before example lines |
| `color_tagline` | `&'static str` | `"\x1b[1m"` | ANSI bold for usage line and section headers |
| `color_group` | `&'static str` | `"\x1b[33m\x1b[1m"` | ANSI yellow+bold for group headers |
| `color_option` | `&'static str` | `"\x1b[1;36m"` | ANSI bold cyan for command and option names |
| `color_example` | `&'static str` | `"\x1b[2m"` | ANSI dim for example invocation lines |
| `color_reset` | `&'static str` | `"\x1b[0m"` | ANSI reset applied after each colored span |
| `tty_detect` | `bool` | `true` | When true, colors active only when stdout is a TTY; when false, always suppress colors |

`CliHelpStyle::default()` reproduces the layout and ANSI codes of the hardcoded `claude_profile::print_usage()`.

**`CliHelpData`** — structured content for all rendered sections. All fields are public. Carries `#[non_exhaustive]` — external callers cannot use struct expressions (including struct update syntax); must use `CliHelpData::default()` followed by field assignment. Derives `Default`, `Debug`, `Clone`.

| Field | Type | Purpose |
|-------|------|---------|
| `binary` | `String` | Binary name used in the usage line |
| `tagline` | `String` | One-line description shown below the usage line |
| `groups` | `Vec<CommandGroup>` | Ordered list of command groups |
| `options` | `Vec<OptionEntry>` | Global options; section omitted when empty, and suppressed when `option_groups` is non-empty |
| `examples` | `Vec<ExampleEntry>` | Usage examples; section omitted when empty |
| `usage_lines` | `Vec<String>` | Custom usage lines; when non-empty replaces default `"Usage: {binary} <command>"` emission; default: `vec![]` |
| `arguments` | `Vec<OptionEntry>` | Positional argument entries rendered in an `Arguments:` section (between `Commands:` label and command group entries); section omitted when empty; default: `vec![]` |
| `option_groups` | `Vec<OptionGroup>` | Named option sections rendered after Commands; when non-empty the `options` field is suppressed; default: `vec![]` |

**`OptionGroup`** — a named group of option entries with independent column padding. Derives `Debug`, `Clone`.

| Field | Type | Purpose |
|-------|------|---------|
| `name` | `String` | Section header displayed as `{name}:` |
| `entries` | `Vec<OptionEntry>` | Option entries; column padding computed from this group's `max(name.len())` only — other groups do not affect it |

**`CommandGroup`** — a named group of commands.

| Field | Type | Purpose |
|-------|------|---------|
| `name` | `String` | Display name for the group header |
| `entries` | `Vec<CommandEntry>` | Ordered list of commands within this group |

**`CommandEntry`** — a single command in a group.

| Field | Type | Purpose |
|-------|------|---------|
| `name` | `String` | Command name as typed by the user |
| `desc` | `String` | Short description in the adjacent column |

**`OptionEntry`** — a single global option.

| Field | Type | Purpose |
|-------|------|---------|
| `name` | `String` | Option name or syntax string |
| `desc` | `String` | Short description in the adjacent column |

**`ExampleEntry`** — a single usage example.

| Field | Type | Purpose |
|-------|------|---------|
| `invocation` | `String` | Example invocation string shown to the user |
| `desc` | `Option<String>` | Optional annotation; when `Some(text)`, rendered as `  # {text}` after the invocation |

### Operations

**`CliHelpData::default() -> CliHelpData`** — constructs a `CliHelpData` with `binary` and `tagline` as empty strings and all `Vec` fields as `vec![]`. Construct instances via field assignment: `let mut d = CliHelpData::default(); d.binary = "myapp".into();`. Struct expressions (including struct update syntax) are blocked from outside the crate by `#[non_exhaustive]`.

**`CliHelpTemplate::new(style, data) -> CliHelpTemplate`** — constructs a template from style and data. Both parameters are moved in.

**`CliHelpTemplate::render(&self) -> String`** — renders the complete help text in this order:
1. ANSI codes are active only when `style.tty_detect = true` and stdout IS a TTY; otherwise all color fields are treated as empty strings.
2. Emits header: when `data.usage_lines` is non-empty, emits each line as `"  {line}"`; otherwise emits `"{bold}Usage:{rst} {binary} <command>"`. In both cases follows with: blank line, tagline text, blank line, `"{bold}Commands:{rst}"`.
3. If `data.arguments` is non-empty: emits `"{bold}Arguments:{rst}"` section; entries padded to `max(name.len())` across all argument entries.
4. Emits each command group from `data.groups` with entries padded to `cmd_name_width`.
5. For each `OptionGroup` in `data.option_groups`: emits `"{name}:"` header then entries padded to that group's own `max(name.len())` independently.
6. If `data.option_groups` is empty and `data.options` is non-empty: emits `"{bold}Options:{rst}"` section with names padded to `opt_name_width` (backward compat).
7. If `data.examples` is non-empty: emits `"{bold}Examples:{rst}"` section; each `ExampleEntry.desc = Some(text)` appends `  # {text}`; `None` emits the invocation bare.

Column padding uses `{name:<width}` formatting. For commands (step 4) and legacy options (step 6), `width = field_name_width + col_gap` where `field_name_width` is the style-configured `cmd_name_width` or `opt_name_width` (the gap is included in the format specifier width). For arguments (step 3) and option_groups (step 5), `width = max(name.len())` across entries in that section/group only, followed by a hardcoded 2-space literal separator — the gap is NOT included in the format specifier width. Padding is a minimum — names longer than the computed width are not truncated.

### Error Handling

`CliHelpTemplate::render()` is infallible. It performs no file I/O and accepts any valid `CliHelpStyle` and `CliHelpData` value. No error type is returned and no panics occur.

### Compatibility Guarantees

All public struct fields and the `new` / `render` signatures are stable across patch and minor versions. New fields may be added to `CliHelpStyle` or `CliHelpData` in minor versions with backward-compatible defaults. Semantic changes to existing fields require a major version bump.

`CliHelpData` carries `#[non_exhaustive]` — exhaustive struct literals from outside the crate fail to compile. Callers must use `CliHelpData::default()` followed by field assignment; struct update syntax (`..CliHelpData::default()`) also fails to compile outside the crate (E0639). Validated by the T-A08 compile_fail doctest in `src/help.rs`.

### Features

| File | Relationship |
|------|-------------|
| [`../feature/002_cli_help_template.md`](../feature/002_cli_help_template.md) | Behavioral rationale and design decisions for the CLI help template |

### Invariants

| File | Relationship |
|------|-------------|
| [`../invariant/001_architectural_boundary.md`](../invariant/001_architectural_boundary.md) | Boundary principle placing CLI rendering in cli_fmt, not strs_tools |

### Test Specs

| File | Relationship |
|------|-------------|
| [`../../tests/docs/api/002_help_api.md`](../../tests/docs/api/002_help_api.md) | Test specification verifying the API contracts defined here |

### Sources

| File | Relationship |
|------|-------------|
| `src/help.rs` | Implements all public types and `CliHelpTemplate::render()` |

### Tests

| File | Relationship |
|------|-------------|
| `tests/help.rs` | API contract verification — render infallibility, layout defaults, column padding, section omission, annotation rendering, and OptionGroup construction |
