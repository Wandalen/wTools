# API: Theme Types

### Scope

- **Purpose**: Drive test coverage for the theme types API contracts in `docs/api/005_theme_types.md`.
- **Responsibility**: Documents API contract test cases for `ColorTheme`, `ColorThemeBuilder`, preset constructors, and the `apply_to_*` projection methods.
- **In Scope**: Struct field access, preset constructor output, builder setter/`build()` default behavior, `apply_to_table`/`apply_to_expanded`/`apply_to_tree` unconditional-vs-conditional overwrite semantics, feature gating.
- **Out of Scope**: Per-theme color catalog values (see `docs/theme/`), theme application usage patterns (see `tests/docs/feature/004_color_themes.md`), quantity formatting style resolution (see `006_quantity_formatting.md`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| AP-1 | named preset constructors populate all five ANSI SGR fields | ⏳ |
| AP-2 | none() constructor produces all-empty fields | ⏳ |
| AP-3 | custom() builder round-trip produces a theme matching setter inputs | ⏳ |
| AP-4 | build() defaults row_color1 to ANSI reset while other unset fields default empty | ⏳ |
| AP-5 | apply_to_table unconditionally overwrites header/row colors but conditionally skips border color | ⏳ |
| AP-6 | apply_to_expanded and apply_to_tree overwrite unconditionally even when the source color is empty | ⏳ |
| AP-7 | themes feature flag gates ColorTheme out of scope when disabled | ⏳ |

---

### AP-1: named preset constructors populate all five ANSI SGR fields

- **Given:** Each of the six named preset constructors: `ColorTheme::dark()`, `light()`, `monokai()`, `solarized()`, `nord()`, `dracula()`.
- **When:** Each constructor is called with no arguments.
- **Then:** Each returns a `ColorTheme` whose five fields (`header_color`, `border_color`, `row_color1`, `row_color2`, `branch_color`) are populated with a raw ANSI SGR escape string; no constructor consults the environment (`NO_COLOR`, TTY) — calling the same constructor twice under different environment states yields identical output.

---

### AP-2: none() constructor produces all-empty fields

- **Given:** `ColorTheme::none()`.
- **When:** The constructor is called.
- **Then:** All five fields (`header_color`, `border_color`, `row_color1`, `row_color2`, `branch_color`) are empty strings — the explicit "no color" theme, distinct from any named preset.

---

### AP-3: custom() builder round-trip produces a theme matching setter inputs

- **Given:** `ColorTheme::custom()` followed by all five fluent setters (`header_color(..)`, `border_color(..)`, `row_color1(..)`, `row_color2(..)`, `branch_color(..)`), each given a distinct non-empty string.
- **When:** `.build()` is called on the resulting `ColorThemeBuilder`.
- **Then:** The returned `ColorTheme`'s five fields exactly match the five values passed to the setters, in the same field-to-setter correspondence; no value is dropped, reordered, or mutated.

---

### AP-4: build() defaults row_color1 to ANSI reset while other unset fields default empty

- **Given:** `ColorTheme::custom()` with zero setters called (all five builder fields left unset).
- **When:** `.build()` is called.
- **Then:** `header_color`, `border_color`, `row_color2`, and `branch_color` are all empty strings; `row_color1` alone defaults to `"\x1b[0m"` (ANSI reset) — the only field whose unset default is non-empty.

---

### AP-5: apply_to_table unconditionally overwrites header/row colors but conditionally skips border color

- **Given:** A `TableConfig` with pre-existing non-default `border_color`, and a `ColorTheme` whose `border_color` field is empty but whose `header_color`/`row_color1`/`row_color2` are non-empty.
- **When:** `theme.apply_to_table(config)` is called.
- **Then:** `colorize_header` is unconditionally set to `!header_color.is_empty()` and `with_header_color` is called (overwriting any prior header color); `alternating_rows` and the row colors are likewise unconditionally overwritten from `row_color1`/`row_color2`; `with_border_color` is called only because in this scenario it is skipped (empty), so the input config's pre-existing `border_color` remains untouched in the output.

---

### AP-6: apply_to_expanded and apply_to_tree overwrite unconditionally even when the source color is empty

- **Given:** An `ExpandedConfig` with a pre-existing non-default key color and a `TreeConfig` with a pre-existing non-default branch color, both paired with `ColorTheme::none()` (all fields empty).
- **When:** `theme.apply_to_expanded(expanded_config)` and `theme.apply_to_tree(tree_config)` are called.
- **Then:** `apply_to_expanded` sets `colorize_keys = false` and calls `with_key_color("")`, overwriting the prior key color to empty; `apply_to_tree` calls `with_branch_color("")` unconditionally, overwriting the prior branch color to empty — both methods overwrite even when the incoming value is empty, unlike `apply_to_table`'s conditional border-color skip.

---

### AP-7: themes feature flag gates ColorTheme out of scope when disabled

- **Given:** A build with the `themes` feature disabled.
- **When:** The crate is compiled.
- **Then:** `ColorTheme`, `ColorThemeBuilder`, and all preset constructors are not in scope; compilation of code that does not reference theme types succeeds with zero errors (no missing-type errors for the ungated rest of the crate).

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/api/005_theme_types.md`](../../../docs/api/005_theme_types.md) | Source API spec — `ColorTheme`, `ColorThemeBuilder`, preset constructors, `apply_to_*` methods, feature gating |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/themes.rs`](../../themes.rs) (extend) | Spec tests for AP-1..AP-7 — theme construction and application |
| [`tests/table_rendering_colors.rs`](../../table_rendering_colors.rs) (extend) | Color rendering integration cases for theme-applied configs |
