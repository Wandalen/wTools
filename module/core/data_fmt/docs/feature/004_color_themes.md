# Feature: Color Themes

### Scope

- **Purpose**: Provide predefined, professionally designed color schemes for visual formatters (Table, Expanded, Tree) with consistent styling and no manual ANSI code management.
- **Responsibility**: Document predefined themes, custom theme creation, and color reset behavior.
- **In Scope**: Theme definitions, usage patterns, custom theme builder, terminal compatibility, and feature flag integration.
- **Out of Scope**: Algorithm details (see `../algorithm/`), API signatures (see `../api/`).

### APIs

| File | Relationship |
|------|-------------|
| [003_config_types.md](../api/003_config_types.md) | Config types that themes apply to |

### Sources

| File | Relationship |
|------|-------------|
| [`src/themes.rs`](../../src/themes.rs) | Theme definitions |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/themes.rs`](../../tests/themes.rs) | Theme application tests |
| [`tests/table_rendering_colors.rs`](../../tests/table_rendering_colors.rs) | Color rendering tests |

### Design

**Feature flag**: `themes` (zero dependencies, optional enhancement)

#### Predefined Themes

#### Dark

High contrast for dark terminals.

- Headers: Bright cyan
- Borders: Dim white
- Alternating rows: Default + dark gray background
- Tree branches: Cyan

#### Light

Optimized for light terminals.

- Headers: Dark blue
- Borders: Dark gray
- Alternating rows: White + light gray background
- Tree branches: Blue

#### Monokai

Popular code editor theme.

- Headers: Bright magenta
- Borders: Dark gray
- Alternating rows: Black + dark gray background
- Tree branches: Green

#### Solarized

Low-contrast scientific palette.

- Headers: Yellow
- Borders: Base01
- Alternating rows: Base03 + base02 background
- Tree branches: Cyan

#### Nord

Arctic-inspired cool palette.

- Headers: Frost blue
- Borders: Polar night
- Alternating rows: Default + polar night background
- Tree branches: Frost green

#### Dracula

Dark theme with vibrant colors.

- Headers: Purple
- Borders: Comment gray
- Alternating rows: Background + selection background
- Tree branches: Pink

#### Usage

Themes work with all three visual formatter configs. Applying a theme to a config instance sets all relevant color fields automatically — the caller does not need to configure individual ANSI codes. Each visual formatter config type exposes a method to apply a theme and returns a configured instance ready for use.

#### Theme Application

Themes automatically configure the relevant fields on each config type:

- **Table config**: header color, alternating row colors, border colors
- **Expanded config**: key color, record separator colors
- **Tree config**: branch symbol colors, data colors

#### Custom Theme Creation

Build a custom theme by specifying individual color components: header color, border color, and alternating row colors. The resulting theme is applied to any visual formatter config instance the same way as a predefined theme.

#### Color Reset Behavior

All themes include automatic color reset after every colored element. This prevents color bleeding into subsequent terminal output. Each colored line wraps content with the chosen color and a terminal reset sequence.

#### Terminal Compatibility

- Uses standard ANSI escape codes
- 256-color support
- Gracefully degrades in non-color terminals
- No-color mode disables all colors

#### Feature Flag Integration

The `themes` feature is optional. Guard theme usage with the appropriate feature gate when compiling without the feature enabled.
