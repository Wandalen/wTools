# API: Help Template Module

### Scope

- **Purpose**: Document the public interface for the CLI help template renderer in `cli_fmt`.
- **Responsibility**: Reference for all public types, their fields, and the rendering entry point.
- **In Scope**: `CliHelpStyle`, `CliHelpData`, `CommandGroup`, `CommandEntry`, `OptionEntry`, `ExampleEntry`, and `CliHelpTemplate::render()`.
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

**`CliHelpData`** — structured content for all rendered sections. All fields are public.

| Field | Type | Purpose |
|-------|------|---------|
| `binary` | `String` | Binary name used in the usage line |
| `tagline` | `String` | One-line description shown below the usage line |
| `groups` | `Vec<CommandGroup>` | Ordered list of command groups |
| `options` | `Vec<OptionEntry>` | Global options; section omitted when empty |
| `examples` | `Vec<ExampleEntry>` | Usage examples; section omitted when empty |

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

**`CliHelpTemplate::new(style, data) -> CliHelpTemplate`** — constructs a template from style and data. Both parameters are moved in.

**`CliHelpTemplate::render(&self) -> String`** — renders the complete help text:
1. ANSI codes are active only when `style.tty_detect = true` and stdout IS a TTY; otherwise all color fields are treated as empty strings.
2. Emits header section: `{color_tagline}Usage:{color_reset} {binary} <command>`, a blank line, the tagline text (no color), a blank line, then `{color_tagline}Commands:{color_reset}`.
3. Emits each command group with its entries, names padded to `cmd_name_width`.
4. If `options` is non-empty: emits `Options:` section with names padded to `opt_name_width`.
5. If `examples` is non-empty: emits `Examples:` section; each `ExampleEntry.desc = Some(text)` appends `  # {text}` to the invocation line; `None` emits the invocation bare.

Column padding uses `{name:<width}` where `width = field_name_width + col_gap`. Padding is a minimum — names longer than the configured width are not truncated.

### Error Handling

`CliHelpTemplate::render()` is infallible. It performs no file I/O and accepts any valid `CliHelpStyle` and `CliHelpData` value. No error type is returned and no panics occur.

### Compatibility Guarantees

All public struct fields and the `new` / `render` signatures are stable across patch and minor versions. New fields may be added to `CliHelpStyle` or `CliHelpData` in minor versions with backward-compatible defaults. Semantic changes to existing fields require a major version bump.

### Features

| File | Relationship |
|------|-------------|
| [`../feature/002_cli_help_template.md`](../feature/002_cli_help_template.md) | Behavioral rationale, design decisions, and acceptance criteria |

### Invariants

| File | Relationship |
|------|-------------|
| [`../invariant/001_architectural_boundary.md`](../invariant/001_architectural_boundary.md) | Boundary principle placing CLI rendering in cli_fmt, not strs_tools |

### Sources

| File | Relationship |
|------|-------------|
| `src/help.rs` | Implements all public types and `CliHelpTemplate::render()` |

### Tests

| File | Relationship |
|------|-------------|
| `tests/help.rs` | T01–T14: column alignment, TTY detection, section omission, desc annotation, color defaults, edge cases, data_fmt absence |
