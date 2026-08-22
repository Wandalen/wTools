# Theme: Solarized

### Scope

- **Purpose**: Drive test coverage for the solarized color theme preset.
- **Responsibility**: Documents test cases for the solarized theme in `docs/theme/004_solarized.md`.
- **In Scope**: Exact color field values, the 256-color background requirement for `row_color2`, `apply_to_table` projection onto `TableConfig`, and non-awareness of `NO_COLOR`.
- **Out of Scope**: Theme application mechanics shared across all config types (see `../feature/004_color_themes.md`), attribute schema definitions (see `../data_structure/002_theme_attributes.md`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| TH-1 | header_color is yellow | ⏳ |
| TH-2 | row_color2 is a 256-color background (Solarized base02) | ⏳ |
| TH-3 | apply_to_table projects the full field set | ⏳ |
| TH-4 | NO_COLOR is not consulted during construction | ⏳ |

---

### TH-1: header_color is yellow

- **Given:** A `ColorTheme` constructed via `ColorTheme::solarized()`.
- **When:** Its `header_color` field is inspected.
- **Then:** It equals `\x1b[33m` (yellow) exactly.

---

### TH-2: row_color2 is a 256-color background (Solarized base02)

- **Given:** A `ColorTheme` constructed via `ColorTheme::solarized()`.
- **When:** Its `row_color2` field is inspected.
- **Then:** It equals `\x1b[48;5;235m`, a 256-color background SGR sequence (`48;5;N` form) representing the Solarized base02 tone.

---

### TH-3: apply_to_table projects the full field set

- **Given:** A `ColorTheme` constructed via `ColorTheme::solarized()` and a `TableConfig`.
- **When:** `apply_to_table` is called with the theme.
- **Then:** The returned `TableConfig` has `colorize_header = true`, `alternating_rows = true`, and its header/border/row colors equal `solarized()`'s exact values (`\x1b[33m`, `\x1b[38;5;240m`, `\x1b[38;5;234m`, `\x1b[48;5;235m`).

---

### TH-4: NO_COLOR is not consulted during construction

- **Given:** The `NO_COLOR` environment variable is set.
- **When:** `ColorTheme::solarized()` is constructed.
- **Then:** All five color fields remain populated with their documented non-empty values — `solarized()` never reads `NO_COLOR`; a caller wanting an opt-out must call `ColorTheme::none()` explicitly.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/theme/004_solarized.md`](../../../docs/theme/004_solarized.md) | Source theme doc — solarized preset attribute descriptor and color role assignments |

### Tests

| File | Relationship |
|------|-------------|
| `tests/theme_004_solarized_test.rs` (to create) | Spec tests for TH-1..TH-4 — solarized theme |
