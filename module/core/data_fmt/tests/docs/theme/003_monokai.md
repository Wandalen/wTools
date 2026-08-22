# Theme: Monokai

### Scope

- **Purpose**: Drive test coverage for the monokai color theme preset.
- **Responsibility**: Documents test cases for the monokai theme in `docs/theme/003_monokai.md`.
- **In Scope**: Exact color field values, the 256-color background requirement for `row_color2`, `apply_to_table` projection onto `TableConfig`, and non-awareness of `NO_COLOR`.
- **Out of Scope**: Theme application mechanics shared across all config types (see `../feature/004_color_themes.md`), attribute schema definitions (see `../data_structure/002_theme_attributes.md`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| TH-1 | header_color is bright magenta bold | ⏳ |
| TH-2 | row_color2 is a 256-color dark-gray background | ⏳ |
| TH-3 | apply_to_table projects the full field set | ⏳ |
| TH-4 | NO_COLOR is not consulted during construction | ⏳ |

---

### TH-1: header_color is bright magenta bold

- **Given:** A `ColorTheme` constructed via `ColorTheme::monokai()`.
- **When:** Its `header_color` field is inspected.
- **Then:** It equals `\x1b[1;35m` (bright magenta, bold) exactly.

---

### TH-2: row_color2 is a 256-color dark-gray background

- **Given:** A `ColorTheme` constructed via `ColorTheme::monokai()`.
- **When:** Its `row_color2` field is inspected.
- **Then:** It equals `\x1b[48;5;236m`, a 256-color background SGR sequence (`48;5;N` form).

---

### TH-3: apply_to_table projects the full field set

- **Given:** A `ColorTheme` constructed via `ColorTheme::monokai()` and a `TableConfig`.
- **When:** `apply_to_table` is called with the theme.
- **Then:** The returned `TableConfig` has `colorize_header = true`, `alternating_rows = true`, and its header/border/row colors equal `monokai()`'s exact values (`\x1b[1;35m`, `\x1b[38;5;239m`, `\x1b[38;5;231m`, `\x1b[48;5;236m`).

---

### TH-4: NO_COLOR is not consulted during construction

- **Given:** The `NO_COLOR` environment variable is set.
- **When:** `ColorTheme::monokai()` is constructed.
- **Then:** All five color fields remain populated with their documented non-empty values — `monokai()` never reads `NO_COLOR`; a caller wanting an opt-out must call `ColorTheme::none()` explicitly.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/theme/003_monokai.md`](../../../docs/theme/003_monokai.md) | Source theme doc — monokai preset attribute descriptor and color role assignments |

### Tests

| File | Relationship |
|------|-------------|
| `tests/theme_003_monokai_test.rs` (to create) | Spec tests for TH-1..TH-4 — monokai theme |
