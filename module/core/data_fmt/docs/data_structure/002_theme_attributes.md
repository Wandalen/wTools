# Data Structure: Theme Attributes

### Scope

- **Purpose**: Define the canonical 21-attribute schema used to describe every `ColorTheme` instance in this library.
- **Responsibility**: Single source of truth for attribute names, types, and example values across all theme doc instances.
- **In Scope**: All 21 per-theme attributes organized by group; attribute name, purpose, and example values for each.
- **Out of Scope**: Per-theme attribute values (see `theme/NNN_*.md` files), theme application behavior (see `feature/004_color_themes.md`).

### Abstract

A 21-attribute schema describing every predefined and custom color theme in the library. Attributes are organized across 7 groups covering identity, build requirements, the five ANSI color role assignments, visual characteristics, formatter application targets, API surface, and compatibility. Every theme doc instance fills out all 21 attributes to enable uniform comparison and selection across themes. Deliberately smaller than the 46-attribute variant schema (`data_structure/001_variant_attributes.md`): `ColorTheme` is a 5-field color overlay, not an output-format encoding, so attribute groups like charset, escaping rules, and MIME type do not apply.

### Structure

The schema defines 21 attributes across 7 groups. Every theme doc instance fills out all 21 attributes.

#### Identity & Classification

| # | Attribute | Purpose | Example Values |
|---|-----------|---------|----------------|
| 1 | `theme` | Theme name | `Dark`, `Monokai`, `Custom` |
| 2 | `is_default` | Whether this theme is applied automatically when no theme is chosen | `Yes`, `No` |
| 3 | `category` | Palette category | `Dark-optimized`, `Light-optimized`, `Neutral/Disabled`, `User-defined` |

#### Build & Dependencies

| # | Attribute | Purpose | Example Values |
|---|-----------|---------|----------------|
| 4 | `feature_flag` | Cargo feature required to enable this theme | `themes` |
| 5 | `runtime_deps` | Runtime crate dependencies | `None` |
| 6 | `zero_dependency` | Whether the theme needs zero external crates | `Yes`, `No` |

#### Color Role Assignments

| # | Attribute | Purpose | Example Values |
|---|-----------|---------|----------------|
| 7 | `header_color` | ANSI SGR applied to table headers / expanded keys | `\x1b[1;36m`, `(empty)` |
| 8 | `border_color` | ANSI SGR applied to border and separator glyphs | `\x1b[2;37m`, `(empty)` |
| 9 | `row_color1` | ANSI SGR applied to the first alternating row | `\x1b[0m`, `(empty)` |
| 10 | `row_color2` | ANSI SGR applied to the second alternating row (background) | `\x1b[48;5;235m`, `(empty)` |
| 11 | `branch_color` | ANSI SGR applied to tree branch connector symbols | `\x1b[36m`, `(empty)` |

#### Visual Characteristics

| # | Attribute | Purpose | Example Values |
|---|-----------|---------|----------------|
| 12 | `color_depth` | ANSI capability tier required to render correctly | `Basic (30-37/90-97)`, `256-color (38;5;N)`, `None` |
| 13 | `contrast_profile` | Design intent for legibility | `High-contrast`, `Low-contrast`, `None` |
| 14 | `background_assumption` | Terminal background the theme targets | `Dark`, `Light`, `Any` |

#### Application Targets

| # | Attribute | Purpose | Example Values |
|---|-----------|---------|----------------|
| 15 | `applies_to_table` | Whether `apply_to_table` meaningfully changes `TableConfig` | `Yes`, `No` |
| 16 | `applies_to_expanded` | Whether `apply_to_expanded` meaningfully changes `ExpandedConfig` | `Yes`, `No` |
| 17 | `applies_to_tree` | Whether `apply_to_tree` meaningfully changes `TreeConfig` | `Yes`, `No` |

#### API & Construction

| # | Attribute | Purpose | Example Values |
|---|-----------|---------|----------------|
| 18 | `constructor` | How to construct this theme | `ColorTheme::dark()`, `ColorTheme::custom()` |
| 19 | `builder_pattern` | Whether the theme is assembled via `ColorThemeBuilder` rather than a fixed preset | `Yes`, `No` |

#### Compatibility

| # | Attribute | Purpose | Example Values |
|---|-----------|---------|----------------|
| 20 | `no_color_aware` | Whether construction itself consults `NO_COLOR`/TTY state | `Yes`, `No` |
| 21 | `works_in_ci` | Whether emitting this theme's raw ANSI escapes in non-TTY/CI output stays safe (no crash or corrupted text) | `Yes`, `No` |

### Operations

Theme doc instances consume this schema by filling out all 21 attributes in an attribute table. Attributes are referenced by name (column 2) — theme docs must use the exact attribute names defined here. `no_color_aware` (#20) is `No` for every predefined theme and for `Custom`: `ColorTheme` never inspects the environment itself — callers that want `NO_COLOR` behavior choose `ColorTheme::none()` explicitly (contrast `QuantityStyle::resolve` in `api/006_quantity_formatting.md`, which does fold `NO_COLOR` automatically). When adding a new theme, the author consults this doc to know which attributes to document and what value vocabulary is expected for each.
