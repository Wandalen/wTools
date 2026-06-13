# Wire `ColorTheme::border_color` — apply theme border color when rendering separators

## Execution State

- **Executor Type:** any
- **Actor:** dev
- **Claimed At:** null
- **Status:** ❓ (Unverified)

## Goal

Connect the `border_color` field defined in every `ColorTheme` preset to the rendering path so that separator characters (column dividers, horizontal rules, corners, junctions) are decorated with the configured ANSI style, eliminating the silent dead-field defect where theming a table leaves borders uncolored.
(Motivated: all 6 `ColorTheme` presets define `border_color: DecoratedText` but `apply_to_table()` never forwards it to `TableConfig` and rendering never reads it — `dark_theme().apply_to_table(formatter)` silently discards the border-color intent, making themed borders visually identical to unthemed output despite documented API intent; Observable: a two-column `bordered` table rendered through `ColorTheme::dark_theme()` has every `|` and `+` character wrapped in the theme's border ANSI escape sequence, confirmed by asserting ANSI codes appear specifically on horizontal-rule lines; Scoped: changes limited to `src/config.rs` (add `border_color: Option<DecoratedText>` to `TableConfig`), `src/themes.rs` (forward `self.border_color` in `apply_to_table()`), and `src/formatters/table/rendering.rs` (apply color on all separator/junction/corner chars); no public API additions beyond the new builder method; Testable: `w3 .test level::3` passes and a new `test_dark_theme_border_color` in `tests/theme_test.rs` confirms that horizontal-rule output lines contain ANSI escape sequences when rendered through `dark_theme()`)

## In Scope

All paths relative to the crate root (`module/core/data_fmt/`).

**Source:**
- `src/config.rs` — add `border_color: Option<DecoratedText>` field to `TableConfig`; add `pub fn border_color(mut self, color: DecoratedText) -> Self` builder; all existing preset constructors default this field to `None`
- `src/themes.rs` — in `apply_to_table()`, add forwarding: clone `self.border_color` and call `.border_color(clone)` on the config before building the formatter
- `src/formatters/table/rendering.rs` — add `fn apply_border_color(&self, s: &str) -> String` helper; in all sites that push a border character string (column separator `|` / `│`, junction `+` / `┼` / `├` / `┤` / `┬` / `┴`, corner `┌` / `┐` / `└` / `┘`, horizontal fill `-` / `─`): wrap with `self.apply_border_color(&char)` when `self.config.border_color` is `Some`

**Tests:**
- `tests/theme_test.rs` — add `test_dark_theme_border_color`: render a `bordered` two-column table through `dark_theme()`; assert all horizontal-rule output lines (those whose ANSI-stripped form starts with `+`) contain `\x1b` escape codes

## Out of Scope

- `plain`, `minimal`, `compact`, `csv`, `tsv` — `border_color = None` by default; no behavior change
- `ColorTheme` struct shape — `border_color` field already exists; no new fields
- All non-table formatters (tree, JSON, SQL, etc.)
- Existing test fixtures — update only those that now have ANSI-decorated border chars in expected strings

## Work Procedure

1. Read `src/config.rs`. Add `border_color: Option<DecoratedText>` to `TableConfig`. Default to `None` in all preset constructors. Add builder method.
2. Read `src/themes.rs`. Identify the `apply_to_table()` call chain. Forward `self.border_color.clone()` by calling the new builder method on the config.
3. Read `src/formatters/table/rendering.rs`. Add `fn apply_border_color(&self, s: &str) -> String`. Locate all border-char emission sites (column separators, horizontal rules, corners, junctions) in: `format_single_line_row`, `format_multiline_row`, `format_ascii_horizontal_rule`, `format_unicode_horizontal_rule`, `format_header_separator`, `format_top_border_if_needed`, `format_bottom_border_if_needed`. Wrap each site.
4. Run `w3 .test level::3`. Update any exact-string test assertions that fail because ANSI codes now appear on border chars when a theme is active.
5. Add `test_dark_theme_border_color` to `tests/theme_test.rs` as described in In Scope.
6. Run `w3 .test level::3` again to confirm clean pass.

## Test Matrix

| Scenario | Assertion |
|----------|-----------|
| `dark_theme()` + `bordered`, horizontal-rule lines | each `+---+` line contains `\x1b` |
| `dark_theme()` + `bordered`, header/data lines | `\x1b` codes present on border chars; row content color unchanged |
| `TableConfig::bordered()` alone (no theme, `border_color = None`) | no ANSI codes on border chars |

## Closes

null

## Verification Findings

**Finding — Implementation Readiness (T gap):**
Original proposed test assertion `assert!(l.contains('\x1b') && l.contains('+'))` does not confirm that ANSI codes specifically decorate border chars — it would pass even if only cell content were colored. The test must isolate horizontal-rule lines and assert ANSI codes on those lines specifically.

Resolution: Test Matrix above uses `output.lines().filter(|l| strip_ansi(l).starts_with('+'))` to isolate rule lines, then asserts `\x1b` on each such line. This is specific to border chars and cannot be satisfied by cell coloring alone. Executor should implement using this pattern directly (strip ANSI for the filter predicate; assert on the original unstripped line). No rework of scope or goal required.
