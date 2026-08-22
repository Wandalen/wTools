# Theme: Monokai

### Scope

- **Purpose**: Provide a popular code-editor-inspired dark palette.
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
| [`src/themes.rs`](../../src/themes.rs) | `ColorTheme::monokai()` implementation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/themes.rs`](../../tests/themes.rs) | Theme application tests |
| [`tests/table_rendering_colors.rs`](../../tests/table_rendering_colors.rs) | Color rendering tests |

### Identity & Classification

- **theme**: Monokai
- **is_default**: No
- **category**: Dark-optimized

### Build & Dependencies

- **feature_flag**: themes
- **runtime_deps**: None
- **zero_dependency**: Yes

### Color Role Assignments

- **header_color**: `\x1b[1;35m` (bright magenta, bold)
- **border_color**: `\x1b[38;5;239m` (dark gray)
- **row_color1**: `\x1b[38;5;231m` (white text)
- **row_color2**: `\x1b[48;5;236m` (dark gray background)
- **branch_color**: `\x1b[32m` (green)

### Visual Characteristics

- **color_depth**: 256-color (38;5;N)
- **contrast_profile**: High-contrast
- **background_assumption**: Dark

### Application Targets

- **applies_to_table**: Yes — colorized header, alternating rows, border color
- **applies_to_expanded**: Yes — colorized keys via `header_color`
- **applies_to_tree**: Yes — branch connectors via `branch_color`

### API & Construction

- **constructor**: `ColorTheme::monokai()`
- **builder_pattern**: No

### Compatibility

- **no_color_aware**: No — construction does not consult `NO_COLOR`; call `ColorTheme::none()` to opt out manually
- **works_in_ci**: Yes

### Example Output

| Role | ANSI SGR |
|------|----------|
| Header / key | `\x1b[1;35m` |
| Border / separator | `\x1b[38;5;239m` |
| Row 1 | `\x1b[38;5;231m` |
| Row 2 (background) | `\x1b[48;5;236m` |
| Tree branch | `\x1b[32m` |
