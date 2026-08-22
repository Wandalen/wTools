# Theme: Dracula

### Scope

- **Purpose**: Drive test coverage for the dracula color theme preset.
- **Responsibility**: Documents test cases for the dracula theme in `docs/theme/006_dracula.md`.
- **In Scope**: Exact color field values, the 256-color background requirement for `row_color2`, `apply_to_table` projection onto `TableConfig`, and non-awareness of `NO_COLOR`.
- **Out of Scope**: Theme application mechanics shared across all config types (see `../feature/004_color_themes.md`), attribute schema definitions (see `../data_structure/002_theme_attributes.md`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| TH-1 | header_color is purple | ⏳ |
| TH-2 | row_color2 is a 256-color selection-background value | ⏳ |
| TH-3 | apply_to_table projects the full field set | ⏳ |
| TH-4 | NO_COLOR is not consulted during construction | ⏳ |

---

### TH-1: header_color is purple

- **Given:** A `ColorTheme` constructed via `ColorTheme::dracula()`.
- **When:** Its `header_color` field is inspected.
- **Then:** It equals `\x1b[38;5;141m` (purple) exactly.

---

### TH-2: row_color2 is a 256-color selection-background value

- **Given:** A `ColorTheme` constructed via `ColorTheme::dracula()`.
- **When:** Its `row_color2` field is inspected.
- **Then:** It equals `\x1b[48;5;236m`, a 256-color background SGR sequence (`48;5;N` form) representing the selection-background tone.

---

### TH-3: apply_to_table projects the full field set

- **Given:** A `ColorTheme` constructed via `ColorTheme::dracula()` and a `TableConfig`.
- **When:** `apply_to_table` is called with the theme.
- **Then:** The returned `TableConfig` has `colorize_header = true`, `alternating_rows = true`, and its header/border/row colors equal `dracula()`'s exact values (`\x1b[38;5;141m`, `\x1b[38;5;61m`, `\x1b[38;5;231m`, `\x1b[48;5;236m`).

---

### TH-4: NO_COLOR is not consulted during construction

- **Given:** The `NO_COLOR` environment variable is set.
- **When:** `ColorTheme::dracula()` is constructed.
- **Then:** All five color fields remain populated with their documented non-empty values — `dracula()` never reads `NO_COLOR`; a caller wanting an opt-out must call `ColorTheme::none()` explicitly.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/theme/006_dracula.md`](../../../docs/theme/006_dracula.md) | Source theme doc — dracula preset attribute descriptor and color role assignments |

### Tests

| File | Relationship |
|------|-------------|
| `tests/theme_006_dracula_test.rs` (to create) | Spec tests for TH-1..TH-4 — dracula theme |
