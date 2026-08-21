# API: Help Template Module

### Scope

- **Purpose**: Document the public interface for the CLI help template renderer in `cli_fmt`.
- **Responsibility**: Reference for all public types, their fields, and the rendering entry point.
- **In Scope**: CliHelpStyle, CliHelpData, OptionGroup, CommandGroup, CommandEntry, OptionEntry, ExampleEntry, DetailSection, DetailPageData, DetailPageTemplate, default constructors, and render operations.
- **Out of Scope**: Behavioral rationale and design decisions — see `feature/002_cli_help_template.md`.

### Abstract

The help template API provides a typed, style-parameterized renderer for CLI help text.
Callers construct a `CliHelpStyle` (visual parameters) and a `CliHelpData` (content), pass
both to `CliHelpTemplate::new()`, and call `render()` to obtain the complete help string.

`render()` is infallible — it performs no I/O beyond a single TTY probe and cannot return
an error or panic.

All items are re-exported through `cli_fmt::prelude::*`, the crate's recommended import
path.

### Types

**`CliHelpStyle`** — visual and layout parameters for rendering. All fields are public.

| Field | Type | Default | Purpose |
|-------|------|---------|---------|
| `cmd_indent` | integer | `4` | Left margin before command names |
| `cmd_name_width` | integer | `20` | Minimum column width for command names |
| `grp_indent` | integer | `2` | Left margin before group headers |
| `opt_indent` | integer | `2` | Left margin before option names |
| `opt_name_width` | integer | `18` | Minimum column width for option names |
| `col_gap` | integer | `2` | Gap between name column and description column |
| `example_indent` | integer | `2` | Left margin before example lines |
| `color_tagline` | ANSI string | ANSI bold | ANSI bold for usage line and section headers |
| `color_group` | ANSI string | ANSI yellow+bold | ANSI yellow+bold for group headers |
| `color_option` | ANSI string | ANSI bold cyan | ANSI bold cyan for command and option names |
| `color_example` | ANSI string | ANSI dim | ANSI dim for example invocation lines |
| `color_reset` | ANSI string | ANSI reset | ANSI reset applied after each colored span |
| `tty_detect` | boolean | `true` | When true, colors active only when stdout is a TTY; when false, always suppress colors |

`CliHelpStyle::default()` produces layout and ANSI defaults matching the standard cli_fmt terminal display.

**`CliHelpData`** — structured content for all rendered sections. All fields are public. Extensibility-sealed — external callers cannot use struct literal expressions; must use the default constructor followed by field assignment.

| Field | Type | Purpose |
|-------|------|---------|
| `binary` | string | Binary name used in the usage line |
| `tagline` | string | One-line description shown below the usage line |
| `groups` | list of command groups | Ordered list of command groups |
| `options` | list of option entries | Global options; section omitted when empty, and suppressed when `option_groups` is non-empty |
| `examples` | list of example entries | Usage examples; section omitted when empty |
| `usage_lines` | list of strings | Custom usage lines; when non-empty replaces default `"Usage: {binary} <command>"` emission; default: empty |
| `arguments` | list of option entries | Positional argument entries rendered in an `Arguments:` section (between `Commands:` label and command group entries); section omitted when empty; default: empty |
| `option_groups` | list of option groups | Named option sections rendered after Commands; when non-empty the `options` field is suppressed; default: empty |

**`OptionGroup`** — a named group of option entries with independent column padding.

| Field | Type | Purpose |
|-------|------|---------|
| `name` | string | Section header displayed as `{name}:` |
| `entries` | list of option entries | Option entries; column padding computed from this group's maximum name length only — other groups do not affect it |

**`CommandGroup`** — a named group of commands.

| Field | Type | Purpose |
|-------|------|---------|
| `name` | string | Display name for the group header |
| `entries` | list of command entries | Ordered list of commands within this group |

**`CommandEntry`** — a single command in a group.

| Field | Type | Purpose |
|-------|------|---------|
| `name` | string | Command name as typed by the user |
| `desc` | string | Short description in the adjacent column |

**`OptionEntry`** — a single global option.

| Field | Type | Purpose |
|-------|------|---------|
| `name` | string | Option name or syntax string |
| `desc` | string | Short description in the adjacent column |

**`ExampleEntry`** — a single usage example.

| Field | Type | Purpose |
|-------|------|---------|
| `invocation` | string | Example invocation string shown to the user |
| `desc` | optional string | Optional annotation; when present, rendered as `  # {text}` after the invocation |

**`DetailSection`** — one named block of a detail page. All fields are public.

| Field | Type | Purpose |
|-------|------|---------|
| `title` | string | Section header displayed as `{title}:`; empty title renders entries without a header line |
| `entries` | list of option entries | Name/description rows; section omitted entirely when empty |

**`DetailPageData`** — structured content for a single-subject detail page (one command or one parameter). All fields are public. Extensibility-sealed — external callers cannot use struct literal expressions; must use the default constructor followed by field assignment.

| Field | Type | Purpose |
|-------|------|---------|
| `label` | string | Subject category shown before the name (e.g. `Parameter`, `Command`); empty label drops the `{label}: ` prefix |
| `name` | string | Subject name; empty name renders `{label}:` alone (no trailing space) |
| `usage` | list of strings | Usage lines rendered indented directly under the header |
| `description` | list of strings | Free-form description lines, preceded by a blank line |
| `sections` | list of detail sections | Ordered named blocks; empty-entry sections skipped |
| `examples` | list of example entries | Usage examples; identical rendering to `CliHelpData.examples` |

### Operations

**`CliHelpData::default()`** — constructs a `CliHelpData` with `binary` and `tagline` as empty strings and all list fields as empty lists. Construct instances via field assignment. Struct literal expressions from outside the crate are blocked at compile time.

**`CliHelpTemplate::new(style, data)`** — constructs a template from style and data. Both parameters are consumed.

**`CliHelpTemplate::render`** — renders the complete help text in this order:
1. ANSI codes are active only when `style.tty_detect = true` and stdout IS a TTY; otherwise all color fields are treated as empty strings.
2. Emits header: when `data.usage_lines` is non-empty, emits each line as `"  {line}"`; otherwise emits `"{bold}Usage:{rst} {binary} <command>"`. In both cases follows with: blank line, tagline text, blank line, `"{bold}Commands:{rst}"`.
3. If `data.arguments` is non-empty: emits `"{bold}Arguments:{rst}"` section; entries padded to the maximum argument name length across all argument entries.
4. Emits each command group from `data.groups` with entries padded to `cmd_name_width`.
5. For each `OptionGroup` in `data.option_groups`: if the group's `entries` list is non-empty, emits `"{name}:"` header then entries padded to that group's own maximum entry name length independently; groups with an empty `entries` list are omitted entirely (header included).
6. If `data.option_groups` is empty and `data.options` is non-empty: emits `"{bold}Options:{rst}"` section with names padded to `opt_name_width` (backward compat).
7. If `data.examples` is non-empty: emits `"{bold}Examples:{rst}"` section; each entry with a present desc appends `  # {text}`; entries without desc emit the invocation bare.

Column padding uses minimum-width alignment. For commands (step 4) and legacy options (step 6), the column width equals the larger of the style-configured field width and the longest entry name in that section — the configured value is a floor — plus the column gap. For arguments (step 3) and option groups (step 5), the column width equals the maximum entry name length in that section or group only, followed by a 2-space separator — the gap is not included in this width. Padding is a minimum — a name longer than the configured floor widens the column rather than being truncated or overflowing.

**`DetailSection::new(title, entries)`** — constructs a section from a title (any `Into<String>`) and its entries, stored as given.

**`DetailPageData::default()`** — constructs a `DetailPageData` with empty strings and empty lists throughout. Construct instances via field assignment; struct literal expressions from outside the crate are blocked at compile time (same enforcement as `CliHelpData`, validated by the T-C14 compile_fail doctest).

**`DetailPageTemplate::new(style, data)`** — constructs a detail-page template from a `CliHelpStyle` and a `DetailPageData`. Both parameters are consumed; the style's `opt_indent`, `example_indent`, color fields, and `tty_detect` govern the rendering.

**`DetailPageTemplate::render`** — renders the detail page in this order:
1. ANSI activation follows the same rule as `CliHelpTemplate::render()` (`tty_detect` and stdout TTY probe).
2. Header line by emptiness of `(label, name)`: both empty → no header; only `name` → bare `{name}`; only `label` → `{label}:` with no trailing space; both → `{label}: {name}`. Rendered in the tagline (bold) color.
3. Each `usage` line indented by `example_indent`, in the example color, directly under the header.
4. If `description` is non-empty: a blank line, then each description line verbatim.
5. Each section from `sections`: skipped entirely when its `entries` is empty; otherwise a blank line, the `{title}:` header in the tagline color (omitted when `title` is empty), then entries indented by `opt_indent` — names padded to that section's own longest name plus a 2-space separator; an entry with an empty description emits the name alone with no trailing whitespace.
6. If `examples` is non-empty: the same `Examples:` section emitter as `CliHelpTemplate` step 7 — byte-identical annotation and indentation behavior.

A fully empty `DetailPageData` renders exactly `""`.

### Error Handling

`CliHelpTemplate::render()` and `DetailPageTemplate::render()` are infallible. They perform no file I/O and accept any valid style and data values. No error type is returned and no panics occur.

### Compatibility Guarantees

All public struct fields and the `new` / `render` signatures are stable across patch and minor versions. New fields may be added to `CliHelpStyle` or `CliHelpData` in minor versions with backward-compatible defaults. Semantic changes to existing fields require a major version bump.

`CliHelpData`, `DetailSection`, and `DetailPageData` are extensible structures — struct literals from outside the crate fail to compile. Callers must use the default constructor followed by field assignment; struct update syntax also fails to compile outside the crate. Validated by the T-A08 (`CliHelpData`) and T-C14 (`DetailPageData`) compile_fail doctests in `src/help.rs`.

### Features

| File | Relationship |
|------|-------------|
| [`../feature/002_cli_help_template.md`](../feature/002_cli_help_template.md) | Behavioral rationale and design decisions for the CLI help template |

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
| [`../../tests/docs/api/002_help_api.md`](../../tests/docs/api/002_help_api.md) | Test specification verifying the API contracts defined here |
| `tests/help.rs` | API contract verification — render infallibility, layout defaults, column padding, section omission, annotation rendering, OptionGroup construction, and the detail-page contract (T-C01..T-C14: golden output, header degradation, per-section padding, empty-data emptiness, prelude re-export) |
