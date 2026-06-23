# Feature: CLI Help Template

### Scope

- **Purpose**: Provide a typed, configurable template that renders structured CLI help output from two disjoint parameter sets — style and data — producing ANSI-colored, column-aligned text suitable for terminal display.
- **Responsibility**: Document style/data separation, multi-section content model, backward compatibility semantics, non-exhaustive enforcement, dependency architecture, and crate boundary rationale.
- **In Scope**: Style configuration (13 layout and color parameters); structured content model (binary, tagline, groups, options, examples, usage lines, arguments, option groups); named option groups with per-group column padding; TTY-conditional ANSI output; backward-compatible options field; non-exhaustive content structure; dependency architecture (parallel with data_fmt); feature flag.
- **Out of Scope**: Type field details and rendering procedure — see `api/002_help_api.md`; data_fmt table-based help path; per-command help (unilang pipeline concern).

### Design

**Separation principle:** CLI help rendering is domain-specific — it encodes CLI presentation conventions (column alignment, color roles, TTY detection, indent hierarchy). This is not general-purpose data formatting. The data_fmt crate operates on untyped row/column structures and is domain-agnostic. Adding CLI help rendering to data_fmt would impose CLI-domain assumptions on a generic library. cli_fmt already holds the boundary for CLI-specific utilities (see `docs/invariant/001_architectural_boundary.md`). The help template belongs here.

**Dependency architecture:** data_fmt and cli_fmt are parallel crates — neither depends on the other. Both are consumed independently by downstream applications. This prevents coupling between domain-specific CLI rendering and generic data formatting.

**Style/data separation:** The template splits configuration into two independent parameter sets. Style parameters (13 fields) control layout and color — indents, column widths, gaps, ANSI color codes, and TTY detection. Data parameters hold structured content — binary name, tagline, command groups, global options, and usage examples. This separation allows the same content to be rendered with different visual styles without rebuilding the data, and vice versa.

**TTY-conditional output:** Colors are active only when the TTY detection flag is enabled and stdout is a terminal. When output is piped or redirected, all color codes are suppressed. This follows CLI convention for machine-readable output.

**Column padding:** Name columns use minimum-width padding — names shorter than the configured width are padded; names longer are not truncated. This ensures alignment across entries without clipping long names.

**Conditional sections:** Options and Examples sections are omitted entirely when their content lists are empty, producing cleaner output for simple tools.

**Usage line override:** When `usage_lines` is non-empty, each entry is emitted on its own indented line, replacing the default `"Usage: {binary} <command>"` emission. When `usage_lines` is empty the original single-line form is preserved — callers that do not set this field see no change.

**Arguments section:** When `arguments` is non-empty, an `Arguments:` section is emitted after the header block (between the `Commands:` label and command group entries), using content-driven column padding. An empty list omits the section entirely.

**Option groups:** `option_groups` holds named sections rendered independently between Commands and the legacy `options` list. Each group computes column padding from its own entries only — longer entries in one group do not widen narrower groups.

**Backward compatibility:** When `option_groups` is empty and `options` is non-empty, the legacy `Options:` section renders unchanged. When `option_groups` is non-empty, the `options` field is suppressed — callers using named groups replace the flat list entirely. Callers that set only `options` and leave `option_groups` empty are fully unaffected.

**Non-exhaustive data structure:** The content structure is marked non-exhaustive. Callers outside the crate cannot construct it with a struct literal; they must use the default constructor followed by field assignment. This is a compile-time enforcement of API extensibility.

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

### Test Specs

| File | Relationship |
|------|-------------|
| [`../../tests/docs/feature/002_cli_help_template.md`](../../tests/docs/feature/002_cli_help_template.md) | Test specification verifying the behavioral cases defined here |

### Sources

| File | Relationship |
|------|-------------|
| `src/help.rs` | Implementation of the style configuration, content structure, and help template renderer |

### Tests

| File | Relationship |
|------|-------------|
| `tests/help.rs` | Column alignment, TTY detection, conditional section rendering, backward compatibility, option groups, and edge cases |
