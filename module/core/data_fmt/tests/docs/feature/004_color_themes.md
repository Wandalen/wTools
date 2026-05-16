# Feature: Color Themes

### Scope

- **Purpose**: Drive test coverage for the color themes feature.
- **Responsibility**: Documents test cases for the predefined and custom color theme system in `docs/feature/004_color_themes.md`.
- **In Scope**: Six predefined themes, custom theme construction from color components, theme field application to `TableConfig`, per-line ANSI reset enforcement, `themes` feature flag gating.
- **Out of Scope**: ANSI/unicode invariants (see `invariant/002`); alternating row colors without themes (see `feature/001`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| FT-1 | predefined theme applies color fields to table config | ✅ |
| FT-2 | all six predefined themes produce non-empty output with color codes | ✅ |
| FT-3 | custom theme built from color components applies correctly | ✅ |
| FT-4 | every colored line ends with ANSI reset sequence | ✅ |
| FT-5 | themes feature flag gates theme API at compile time | ✅ |

---

### FT-1: predefined theme applies color fields to table config

- **Given:** A `TableConfig` with `theme(Theme::Dark)` applied.
- **When:** The config is inspected.
- **Then:** `colorize_header` is `true`; `header_color` contains a non-empty ANSI code; `alternating_rows` is `true`; `row_colors` contains two non-empty ANSI codes.

---

### FT-2: all six predefined themes produce non-empty output with color codes

- **Given:** The six predefined themes: `Dark`, `Light`, `Monokai`, `Solarized`, `Nord`, `Dracula`.
- **When:** Each is applied to a `TableConfig` and used to render a two-row table.
- **Then:** Each output is non-empty and contains at least one ANSI escape sequence (`\x1b[`); the header and data rows appear in each output; no theme panics.

---

### FT-3: custom theme built from color components applies correctly

- **Given:** A custom theme built by specifying a header color, border color, and two alternating row colors.
- **When:** The custom theme is applied to a `TableConfig` and used to render a table.
- **Then:** The rendered output contains the specified header color code on the header row; the two row colors alternate across data rows; each colored line ends with `\x1b[0m`.

---

### FT-4: every colored line ends with ANSI reset sequence

- **Given:** Any predefined theme applied to a `TableConfig`.
- **When:** A multi-row table is rendered.
- **Then:** Every line containing an ANSI color code also contains `\x1b[0m` (reset) before the trailing newline; no ANSI codes extend beyond the end of a line.

---

### FT-5: themes feature flag gates theme API at compile time

- **Given:** A build configuration without the `themes` feature enabled.
- **When:** The crate is compiled.
- **Then:** `Theme` enum and `theme()` builder method are not accessible; compilation
  succeeds without reference to theme symbols; the crate compiles cleanly with only
  the `enabled` feature active.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/feature/004_color_themes.md`](../../../docs/feature/004_color_themes.md) | Source feature spec — six predefined themes, custom theme, feature flag |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/themes.rs`](../../themes.rs) | Theme API and feature-flag test cases |
| [`tests/table_rendering_colors.rs`](../../table_rendering_colors.rs) | Color rendering test cases |
