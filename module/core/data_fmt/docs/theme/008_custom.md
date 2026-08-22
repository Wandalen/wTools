# Theme: Custom

### Scope

- **Purpose**: Document the user-assembled theme mechanism via `ColorThemeBuilder`.
- **Responsibility**: Complete attribute descriptor for the custom-theme construction path — a mechanism description rather than a fixed palette, since no color values are pre-determined.
- **In Scope**: All 21 theme attributes, builder field defaults, feature flag, compatibility.
- **Out of Scope**: Theme application mechanics (see `../feature/004_color_themes.md`), attribute schema (see `../data_structure/002_theme_attributes.md`).

### DataStructures

| File | Relationship |
|------|-------------|
| [002_theme_attributes.md](../data_structure/002_theme_attributes.md) | Attribute definitions for all 21 theme attributes |

### APIs

| File | Relationship |
|------|-------------|
| [005_theme_types.md](../api/005_theme_types.md) | `ColorTheme`/`ColorThemeBuilder` signatures |

### Features

| File | Relationship |
|------|-------------|
| [004_color_themes.md](../feature/004_color_themes.md) | Theme application behavior across config types |

### Sources

| File | Relationship |
|------|-------------|
| [`src/themes.rs`](../../src/themes.rs) | `ColorTheme::custom()` / `ColorThemeBuilder` implementation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/themes.rs`](../../tests/themes.rs) | Theme application tests |
| [`tests/table_rendering_colors.rs`](../../tests/table_rendering_colors.rs) | Color rendering tests |

### Identity & Classification

- **theme**: Custom
- **is_default**: No
- **category**: User-defined

### Build & Dependencies

- **feature_flag**: themes
- **runtime_deps**: None
- **zero_dependency**: Yes

### Color Role Assignments

Not fixed — each field is set via a `ColorThemeBuilder` setter, or left at its default if unset:

- **header_color**: user-supplied via `.header_color(...)`; defaults to `""` (empty) if unset
- **border_color**: user-supplied via `.border_color(...)`; defaults to `""` (empty) if unset
- **row_color1**: user-supplied via `.row_color1(...)`; defaults to `\x1b[0m` if unset — the one field with a non-empty builder default
- **row_color2**: user-supplied via `.row_color2(...)`; defaults to `""` (empty) if unset
- **branch_color**: user-supplied via `.branch_color(...)`; defaults to `""` (empty) if unset

### Visual Characteristics

- **color_depth**: Determined by caller — any ANSI SGR string is accepted verbatim, no validation
- **contrast_profile**: Determined by caller
- **background_assumption**: Determined by caller

### Application Targets

- **applies_to_table**: Yes — same `apply_to_table` mechanics as every other theme, driven by whichever fields the caller set
- **applies_to_expanded**: Yes — same `apply_to_expanded` mechanics
- **applies_to_tree**: Yes — same `apply_to_tree` mechanics

### API & Construction

- **constructor**: `ColorTheme::custom()` returns `ColorThemeBuilder::default()`; chain setters, then `.build()` to produce the `ColorTheme`
- **builder_pattern**: Yes — the only theme instance built via `ColorThemeBuilder` rather than a fixed preset

### Compatibility

- **no_color_aware**: No — the builder performs no validation or environment consultation; any string (including an invalid escape sequence) is accepted as-is
- **works_in_ci**: Yes, provided the caller-supplied values are valid ANSI SGR sequences

### Example Output

No fixed output — this instance documents a mechanism, not a palette (parallel to `variant/023_html_custom.md`'s treatment of `HtmlVariant::Custom`). Example construction:

```rust
let theme = ColorTheme::custom()
  .header_color( "\x1b[1;33m" )
  .border_color( "\x1b[90m" )
  .build();
```
