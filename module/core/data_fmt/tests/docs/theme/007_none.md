# Theme: None

### Scope

- **Purpose**: Drive test coverage for the none (zero-color) theme preset.
- **Responsibility**: Documents test cases for the none theme in `docs/theme/007_none.md`.
- **In Scope**: All-empty color fields, forced-off colorize flags on `apply_to_table`/`apply_to_expanded`, the unconditional `apply_to_tree` overwrite, and the `apply_to_table` border-color skip-when-empty asymmetry.
- **Out of Scope**: Theme application mechanics shared across all config types (see `../feature/004_color_themes.md`), attribute schema definitions (see `../data_structure/002_theme_attributes.md`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| TH-1 | all five color fields are empty strings | ⏳ |
| TH-2 | apply_to_table forces colorize_header and alternating_rows off | ⏳ |
| TH-3 | apply_to_expanded forces colorize_keys off | ⏳ |
| TH-4 | apply_to_tree overwrites branch_color to empty unconditionally | ⏳ |
| TH-5 | apply_to_table leaves an existing border_color untouched | ⏳ |

---

### TH-1: all five color fields are empty strings

- **Given:** A `ColorTheme` constructed via `ColorTheme::none()`.
- **When:** Its `header_color`, `border_color`, `row_color1`, `row_color2`, and `branch_color` fields are inspected.
- **Then:** Every field equals the empty string `""`, contrasting with every named preset's non-empty values.

---

### TH-2: apply_to_table forces colorize_header and alternating_rows off

- **Given:** `ColorTheme::none()` and a `TableConfig` with `colorize_header` and `alternating_rows` previously `true`.
- **When:** `apply_to_table` is called with the theme.
- **Then:** The returned `TableConfig` has `colorize_header = false` and `alternating_rows = false` — the prior `true` settings are explicitly overridden.

---

### TH-3: apply_to_expanded forces colorize_keys off

- **Given:** `ColorTheme::none()` and an `ExpandedConfig` with `colorize_keys` previously `true`.
- **When:** `apply_to_expanded` is called with the theme.
- **Then:** The returned `ExpandedConfig` has `colorize_keys = false` — the prior `true` setting is explicitly overridden.

---

### TH-4: apply_to_tree overwrites branch_color to empty unconditionally

- **Given:** `ColorTheme::none()` and a `TreeConfig` with a pre-existing non-empty branch color.
- **When:** `apply_to_tree` is called with the theme.
- **Then:** The returned `TreeConfig`'s branch color is overwritten to the empty string `""` — `apply_to_tree` always calls `with_branch_color` unconditionally, even for an empty value.

---

### TH-5: apply_to_table leaves an existing border_color untouched

- **Given:** `ColorTheme::none()` and a `TableConfig` with a pre-existing non-empty `border_color`.
- **When:** `apply_to_table` is called with the theme.
- **Then:** The returned `TableConfig`'s `border_color` is unchanged from its input value — `apply_to_table` skips `with_border_color` entirely whenever the theme's `border_color` is empty, unlike `header_color`/`row_color1`/`row_color2`, which are always overwritten even when empty.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/theme/007_none.md`](../../../docs/theme/007_none.md) | Source theme doc — zero-color preset attribute descriptor and disabling behavior |

### Tests

| File | Relationship |
|------|-------------|
| `tests/theme_007_none_test.rs` (to create) | Spec tests for TH-1..TH-5 — none theme |
