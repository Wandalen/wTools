# Theme: Custom

### Scope

- **Purpose**: Drive test coverage for the custom theme construction mechanism via `ColorThemeBuilder`.
- **Responsibility**: Documents test cases for the custom theme in `docs/theme/008_custom.md`.
- **In Scope**: `custom()`'s builder return type, per-field defaults on `build()`, verbatim setter behavior, and the absence of value validation.
- **Out of Scope**: Theme application mechanics shared across all config types (see `../feature/004_color_themes.md`), attribute schema definitions (see `../data_structure/002_theme_attributes.md`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| TH-1 | custom() returns a builder, not a theme | ⏳ |
| TH-2 | unset fields default to empty except row_color1 | ⏳ |
| TH-3 | chained setters populate fields verbatim | ⏳ |
| TH-4 | build() performs no validation on supplied values | ⏳ |

---

### TH-1: custom() returns a builder, not a theme

- **Given:** The `ColorTheme::custom()` associated function.
- **When:** It is called, before any setters or `.build()` are chained.
- **Then:** It yields a `ColorThemeBuilder`, not a `ColorTheme` — distinguishing `custom()` from every other constructor (`dark()`, `light()`, `monokai()`, `solarized()`, `nord()`, `dracula()`, `none()`), which return `ColorTheme` directly.

---

### TH-2: unset fields default to empty except row_color1

- **Given:** A `ColorThemeBuilder` obtained from `ColorTheme::custom()` with no setters called.
- **When:** `.build()` is invoked.
- **Then:** `header_color`, `border_color`, `row_color2`, and `branch_color` are all empty strings, but `row_color1` equals `\x1b[0m` (ANSI reset) — the one field with a non-empty builder default.

---

### TH-3: chained setters populate fields verbatim

- **Given:** A `ColorThemeBuilder` with `.header_color( "\x1b[1;33m" )` and `.border_color( "\x1b[90m" )` chained.
- **When:** `.build()` is invoked.
- **Then:** The resulting `ColorTheme` has `header_color == "\x1b[1;33m"` and `border_color == "\x1b[90m"` exactly as supplied, while unset fields (`row_color2`, `branch_color`) fall back to their defaults.

---

### TH-4: build() performs no validation on supplied values

- **Given:** A `ColorThemeBuilder` with `.header_color(...)` set to an arbitrary non-ANSI string (e.g. `"not-a-valid-escape"`).
- **When:** `.build()` is invoked.
- **Then:** The resulting `ColorTheme`'s `header_color` holds that exact string verbatim — the builder performs no validation and `build()` never returns an error.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/theme/008_custom.md`](../../../docs/theme/008_custom.md) | Source theme doc — `ColorThemeBuilder` construction mechanism and field defaults |

### Tests

| File | Relationship |
|------|-------------|
| `tests/theme_008_custom_test.rs` (to create) | Spec tests for TH-1..TH-4 — custom theme |
