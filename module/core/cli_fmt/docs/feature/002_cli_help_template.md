# Feature: CLI Help Template

### Scope

- **Purpose**: Provide a typed, configurable template that renders structured CLI help output from two disjoint parameter sets — style and data — producing ANSI-colored, column-aligned text suitable for terminal display.
- **Responsibility**: Document style/data separation, multi-section content model, backward compatibility semantics, non-exhaustive enforcement, dependency architecture, and crate boundary rationale.
- **In Scope**: Rendering structured CLI help from typed style and data parameters, covering section layout, ANSI color, TTY detection, column alignment, and section omission rules; the detail-page template for single-subject help (one command or one parameter).
- **Out of Scope**: Type field details and rendering procedure — see `api/002_help_api.md`; data_fmt table-based help path; per-command help (unilang pipeline concern).

### Design

**Separation principle:** CLI help rendering is domain-specific — it encodes CLI presentation conventions (column alignment, color roles, TTY detection, indent hierarchy). This is not general-purpose data formatting. The data_fmt crate operates on untyped row/column structures and is domain-agnostic. Adding CLI help rendering to data_fmt would impose CLI-domain assumptions on a generic library. cli_fmt already holds the boundary for CLI-specific utilities (see `docs/invariant/001_architectural_boundary.md`). The help template belongs here.

**Dependency architecture:** `data_fmt` and `cli_fmt` are parallel crates — neither depends on the other (see `docs/invariant/001_architectural_boundary.md`, which enforces this boundary alongside the `strs_tools` boundary). Both are consumed independently by downstream applications. This prevents coupling between domain-specific CLI rendering and generic data formatting.

**Style/data separation:** The template splits configuration into two independent parameter sets. Style parameters (13 fields) control layout and color — indents, column widths, gaps, ANSI color codes, and TTY detection. Data parameters hold structured content — binary name, tagline, command groups, global options, and usage examples. This separation allows the same content to be rendered with different visual styles without rebuilding the data, and vice versa.

**TTY-conditional output:** Colors are active only when the TTY detection flag is enabled and stdout is a terminal. When output is piped or redirected, all color codes are suppressed. This follows CLI convention for machine-readable output.

**Column padding:** Configured name-column widths are floors, not fixed widths — the effective column width is the larger of the configured value and the longest name in the section. Names shorter than the column are padded; a name longer than the configured width widens the whole column instead of overflowing its own line. Alignment therefore holds for every configuration, and long names are never truncated.

**Conditional sections:** Options and Examples sections are omitted entirely when their content lists are empty, producing cleaner output for simple tools.

**Usage line override:** When `usage_lines` is non-empty, each entry is emitted on its own indented line, replacing the default `"Usage: {binary} <command>"` emission. When `usage_lines` is empty the original single-line form is preserved — callers that do not set this field see no change.

**Arguments section:** When `arguments` is non-empty, an `Arguments:` section is emitted after the header block (between the `Commands:` label and command group entries), using content-driven column padding. An empty list omits the section entirely.

**Option groups:** `option_groups` holds named sections rendered independently between Commands and the legacy `options` list. Each group computes column padding from its own entries only — longer entries in one group do not widen narrower groups.

**Backward compatibility:** When `option_groups` is empty and `options` is non-empty, the legacy `Options:` section renders unchanged. When `option_groups` is non-empty, the `options` field is suppressed — callers using named groups replace the flat list entirely. Callers that set only `options` and leave `option_groups` empty are fully unaffected.

**Non-exhaustive data structure:** The content structure is marked non-exhaustive. Callers outside the crate cannot construct it with a struct literal; they must use the default constructor followed by field assignment. This is a compile-time enforcement of API extensibility.

**Detail page template:** Alongside the overview template (`CliHelpTemplate`, which renders a whole binary's help), a second template renders a detail page about a single subject — one command or one parameter. Its data model is generic and domain-free: a labeled header (`{label}: {name}`, degrading gracefully when either half is empty), usage lines, description lines, an ordered list of named sections each holding option entries, and examples. Frameworks (not this crate) decide what the sections mean — a parameter page might carry "Type"/"Possible values" sections; a command page might carry "Parameters". This keeps cli_fmt free of any command-framework vocabulary while giving frameworks a shared, styled rendering target.

**Detail page section rules:** A section with no entries is skipped entirely, title included. A section with entries but an empty title renders its entries as a bare block without a header line. Entry name columns pad to each section's own longest name — per-section independent, content-driven, with no configured floor (unlike the overview template's `cmd_name_width`/`opt_name_width` floors, a detail page's sections are small and self-describing). An entry with an empty description renders the name alone with no trailing whitespace. A fully empty data structure renders to the empty string — the detail template emits nothing it wasn't given, so callers can compose its output without defensive trimming.

**Detail page shared emitters:** The Examples section reuses the same emitter as the overview template, so annotation rendering (`# text` when a description is present) and indentation behave identically across both templates — one convention, learned once.

**Feature flag:** The `cli_help_template` feature flag enables this module (declared as `["std"]` dependency). Included in the default feature set when the crate is enabled.

For complete type definitions, field defaults, and the rendering procedure, see [`api/002_help_api.md`](../api/002_help_api.md).

### APIs

| File | Relationship |
|------|-------------|
| [`../api/002_help_api.md`](../api/002_help_api.md) | Public interface contract for this feature |

### Invariants

| File | Relationship |
|------|-------------|
| [`../invariant/001_architectural_boundary.md`](../invariant/001_architectural_boundary.md) | Boundary principle placing CLI rendering in cli_fmt, not strs_tools |

### Pitfalls

| File | Relationship |
|------|-------------|
| [`../pitfall/003_option_field_silent_drop.md`](../pitfall/003_option_field_silent_drop.md) | Option-field silent-drop trap this feature's example rendering must avoid |

### Sources

| File | Relationship |
|------|-------------|
| `src/help.rs` | Implementation of the style configuration, content structure, and help template renderer |

### Tests

| File | Relationship |
|------|-------------|
| [`../../tests/docs/feature/002_cli_help_template.md`](../../tests/docs/feature/002_cli_help_template.md) | Test specification verifying the behavioral cases defined here |
| `tests/help.rs` | Column alignment, TTY detection, conditional section rendering, backward compatibility, option groups, detail page rendering (T-C01..T-C14), and edge cases |
