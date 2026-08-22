# Theme: Light

### Scope

- **Purpose**: Provide a color palette optimized for light-background terminals.
- **Responsibility**: Complete attribute descriptor for this theme preset.
- **In Scope**: All 21 theme attributes, example ANSI role assignments, feature flag, compatibility.
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
| [`src/themes.rs`](../../src/themes.rs) | `ColorTheme::light()` implementation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/themes.rs`](../../tests/themes.rs) | Theme application tests |
| [`tests/table_rendering_colors.rs`](../../tests/table_rendering_colors.rs) | Color rendering tests |

### Identity & Classification

- **theme**: Light
- **is_default**: No
- **category**: Light-optimized

### Build & Dependencies

- **feature_flag**: themes
- **runtime_deps**: None
- **zero_dependency**: Yes

### Color Role Assignments

- **header_color**: `\x1b[34m` (dark blue)
- **border_color**: `\x1b[90m` (bright black / dark gray)
- **row_color1**: `\x1b[0m` (default — black on white)
- **row_color2**: `\x1b[48;5;255m` (light gray background)
- **branch_color**: `\x1b[34m` (dark blue)

### Visual Characteristics

- **color_depth**: 256-color (38;5;N) — required by `row_color2`
- **contrast_profile**: High-contrast
- **background_assumption**: Light

### Application Targets

- **applies_to_table**: Yes — colorized header, alternating rows, border color
- **applies_to_expanded**: Yes — colorized keys via `header_color`
- **applies_to_tree**: Yes — branch connectors via `branch_color`

### API & Construction

- **constructor**: `ColorTheme::light()`
- **builder_pattern**: No

### Compatibility

- **no_color_aware**: No — construction does not consult `NO_COLOR`; call `ColorTheme::none()` to opt out manually
- **works_in_ci**: Yes

### Example Output

| Role | ANSI SGR |
|------|----------|
| Header / key | `\x1b[34m` |
| Border / separator | `\x1b[90m` |
| Row 1 | `\x1b[0m` |
| Row 2 (background) | `\x1b[48;5;255m` |
| Tree branch | `\x1b[34m` |
