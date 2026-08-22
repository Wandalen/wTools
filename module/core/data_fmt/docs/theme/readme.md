# Theme Doc Entity

### Scope

- **Purpose**: Document `ColorTheme`'s predefined and custom color palettes for visual formatters.
- **Responsibility**: Registry and overview of all theme doc instances.
- **In Scope**: All named color themes and the custom theme mechanism; per-theme color role assignments, application targets, feature flag.
- **Out of Scope**: Config types themes apply to (see `../api/003_config_types.md`), theme application behavior (see `../feature/004_color_themes.md`), attribute schema definitions (see `../data_structure/002_theme_attributes.md`).

Required instance sections (in order):

| Section | Heading | Required Content |
|---------|---------|-----------------|
| Scope | `### Scope` | 4 bullets: Purpose / Responsibility / In Scope / Out of Scope |
| Typed References | `### DataStructures`, `### APIs`, `### Features`, `### Sources`, `### Tests` | Per-type `\| File \| Relationship \|` table; `### Sources` and `### Tests` always last |
| Attribute Groups | `### Identity & Classification` ... `### Compatibility` | All 21 attributes per `../data_structure/002_theme_attributes.md`, one bullet list per group |
| Example Output | `### Example Output` | Role → ANSI SGR table (or mechanism description for non-fixed themes) |

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Dark](001_dark.md) | High-contrast palette for dark terminals | ✅ |
| 002 | [Light](002_light.md) | Palette optimized for light terminals | ✅ |
| 003 | [Monokai](003_monokai.md) | Popular code-editor-inspired palette | ✅ |
| 004 | [Solarized](004_solarized.md) | Low-contrast scientific palette | ✅ |
| 005 | [Nord](005_nord.md) | Arctic-inspired cool palette | ✅ |
| 006 | [Dracula](006_dracula.md) | Dark palette with vibrant accent colors | ✅ |
| 007 | [None](007_none.md) | Explicit zero-color theme — disables all coloring | ✅ |
| 008 | [Custom](008_custom.md) | User-assembled theme via `ColorThemeBuilder` | ✅ |

### Organization

Instances are grouped by construction path, not by numeric range:

- **001-006**: Named presets — fixed ANSI values returned by a zero-argument constructor (`ColorTheme::dark()`, etc.).
- **007**: The degenerate zero-color case (`ColorTheme::none()`) — every color field is an empty string.
- **008**: The builder-driven escape hatch (`ColorTheme::custom()` → `ColorThemeBuilder`) — no fixed color values; documents the mechanism instead of a palette.

### Cross-Doc Entity Dependencies

Every instance's `### DataStructures` section links to `../data_structure/002_theme_attributes.md` for the attribute schema. Every instance's `### APIs` section links to `../api/005_theme_types.md` for `ColorTheme`/`ColorThemeBuilder` signatures. Every instance's `### Features` section links to `../feature/004_color_themes.md` for application behavior.
