# API: Theme Types

### Scope

- **Purpose**: Document the public API surface for color theme types.
- **Responsibility**: Define `ColorTheme`, `ColorThemeBuilder`, preset constructors, and the `apply_to_*` projection methods.
- **In Scope**: Struct fields, preset constructors, builder setters, `apply_to_*` method behavior, feature gating.
- **Out of Scope**: Per-theme color catalog (see `../theme/`), theme application usage patterns (see `../feature/004_color_themes.md`).

### Sources

| File | Relationship |
|------|-------------|
| [`src/themes.rs`](../../src/themes.rs) | `ColorTheme`, `ColorThemeBuilder`, preset constructors, `apply_to_*` methods |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/themes.rs`](../../tests/themes.rs) | Theme application tests |
| [`tests/table_rendering_colors.rs`](../../tests/table_rendering_colors.rs) | Color rendering tests |

### Abstract

One struct (`ColorTheme`), one builder (`ColorThemeBuilder`), eight construction paths (six named presets plus `none()` and `custom()`), and three `apply_to_*` methods that project theme colors onto `TableConfig`, `ExpandedConfig`, and `TreeConfig`. The entire module is gated behind the `themes` feature flag and has zero external dependencies.

### Operations

#### ColorTheme

Five public `String` fields: `header_color`, `border_color`, `row_color1`, `row_color2`, `branch_color` — each a raw ANSI SGR escape sequence, or an empty string for "no color". Eight constructors, all `pub fn ... -> Self` except `custom()`: `dark()`, `light()`, `monokai()`, `solarized()`, `nord()`, `dracula()`, `none()` (all five fields empty), `custom() -> ColorThemeBuilder`. No constructor consults the environment (`NO_COLOR`, TTY) — theme selection is always an explicit caller choice, unlike [`QuantityStyle::resolve`](006_quantity_formatting.md), which is environment-aware.

Three application methods, each `#[ must_use ]` and consuming/returning the target config by value:

- `apply_to_table( &self, config : TableConfig ) -> TableConfig` — unconditionally sets `colorize_header = !self.header_color.is_empty()` and calls `with_header_color( self.header_color.clone() )` (overwriting the input config's header color even when empty), and unconditionally sets `alternating_rows = !self.row_color2.is_empty()` and calls `with_row_colors` from `row_color1`/`row_color2` (same unconditional overwrite); by contrast, calls `with_border_color` only when `self.border_color` is non-empty, otherwise skips it entirely, leaving the input config's own `border_color` untouched.
- `apply_to_expanded( &self, config : ExpandedConfig ) -> ExpandedConfig` — unconditionally sets `colorize_keys = !self.header_color.is_empty()` and calls `with_key_color( self.header_color.clone() )` (overwriting the input config's key color even when empty).
- `apply_to_tree( &self, config : TreeConfig ) -> TreeConfig` — sets `with_branch_color( self.branch_color.clone() )` unconditionally, including when empty.

#### ColorThemeBuilder

Five private `Option< String >` fields mirroring `ColorTheme`'s fields. Fluent `#[ must_use ]` setters, each `fn ...( mut self, value : impl Into< String > ) -> Self`: `header_color()`, `border_color()`, `row_color1()`, `row_color2()`, `branch_color()`. `build( self ) -> ColorTheme` consumes the builder: four fields default to an empty string when unset, but `row_color1` defaults to `"\x1b[0m"` (ANSI reset) when unset — the only field with a non-empty default.

### Error Handling

Theme construction and application never return errors. Builder setters and `build()` perform no validation — any string, including a malformed ANSI escape sequence, is accepted as-is and passed through to rendered output verbatim.

### Compatibility Guarantees

All six named preset constructors are stable — their exact ANSI SGR values do not change across minor versions (see `../theme/` for the frozen per-preset catalog). `none()` always produces all-empty fields. `ColorThemeBuilder`'s `row_color1` default (`"\x1b[0m"`) is a stable, deliberate default distinguishing "unset" from "explicitly empty" for that one field only.
