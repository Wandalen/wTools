# Theme: None

### Scope

- **Purpose**: Provide an explicit zero-color theme that disables all theme-driven coloring.
- **Responsibility**: Complete attribute descriptor for this theme preset.
- **In Scope**: All 21 theme attributes, disabling behavior, feature flag, compatibility.
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
| [`src/themes.rs`](../../src/themes.rs) | `ColorTheme::none()` implementation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/themes.rs`](../../tests/themes.rs) | Theme application tests |
| [`tests/table_rendering_colors.rs`](../../tests/table_rendering_colors.rs) | Color rendering tests |

### Identity & Classification

- **theme**: None
- **is_default**: No — `TableConfig`/`ExpandedConfig`/`TreeConfig` have no ambient `ColorTheme`; `none()` is an explicit opt-out a caller applies, not an implicit fallback
- **category**: Neutral/Disabled

### Build & Dependencies

- **feature_flag**: themes
- **runtime_deps**: None
- **zero_dependency**: Yes

### Color Role Assignments

- **header_color**: `""` (empty)
- **border_color**: `""` (empty)
- **row_color1**: `""` (empty)
- **row_color2**: `""` (empty)
- **branch_color**: `""` (empty)

### Visual Characteristics

- **color_depth**: None
- **contrast_profile**: None
- **background_assumption**: Any

### Application Targets

- **applies_to_table**: Yes — forces `colorize_header = false` and `alternating_rows = false`; leaves the config's existing `border_color` untouched since `apply_to_table` skips `with_border_color` entirely when the theme's `border_color` is empty
- **applies_to_expanded**: Yes — forces `colorize_keys = false`
- **applies_to_tree**: Yes — sets `branch_color` to an empty string

### API & Construction

- **constructor**: `ColorTheme::none()`
- **builder_pattern**: No

### Compatibility

- **no_color_aware**: No — `none()` is a manual opt-out, not automatic `NO_COLOR`/TTY detection; nothing in `ColorTheme` reads the environment
- **works_in_ci**: Yes

### Example Output

| Role | ANSI SGR |
|------|----------|
| Header / key | (none) |
| Border / separator | (none) |
| Row 1 | (none) |
| Row 2 (background) | (none) |
| Tree branch | (none) |
