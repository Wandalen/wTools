# Feature: Unified Format Interface

### Scope

- **Purpose**: Provide a single `Format` trait and canonical `TableView` data type so callers can write format-agnostic code while each formatter lives behind its own feature flag for zero-cost abstractions.
- **Responsibility**: Document the unified format interface design, formatter registry, and feature flag configuration.
- **In Scope**: Format trait, TableView interchange, formatter registry, feature bundles, and usage patterns.
- **Out of Scope**: Algorithm details (see `../algorithm/`), API signatures (see `../api/`).

### Traits

| File | Relationship |
|------|-------------|
| [001_format.md](../trait/001_format.md) | Format trait contract |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/format_trait.rs`](../../src/formatters/format_trait.rs) | Format trait definition |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/unified_format_trait.rs`](../../tests/unified_format_trait.rs) | Format trait tests |

### Design

#### Design Goals

1. **Unified interface** -- same API for all formatters (table, json, yaml, toml, text, etc.).
2. **Canonical data format** -- `TableView` struct as common interchange format between data producers and formatters.
3. **Granular features** -- each formatter behind an optional feature flag; unused formatters compile to zero code and zero dependencies.
4. **Zero-cost abstractions** -- no runtime overhead from the trait dispatch; unused formats add nothing to the binary.

#### Core Types

#### TableView

The canonical data format consumed by all formatters. It holds a `TableMetadata` header and rows as a two-dimensional collection of string values.

#### Format Trait

The `Format` trait defines a single method that accepts a `TableView` reference and returns a formatted string or an error.

Nine of ten formatters implement `Format`. `TreeFormatter` uses direct method dispatch. Callers build a `TableView` once and pass it to any `Format`-implementing formatter.

#### Formatter Registry

| Formatter | Feature Flag | Dependencies | Use Case |
|-----------|-------------|--------------|----------|
| `TableFormatter` | `format_table` | None | Visual table output |
| `ExpandedFormatter` | `format_expanded` | None | Vertical records |
| `TreeFormatter` | `format_tree` | None | Hierarchical display |
| `LogfmtFormatter` | `format_logfmt` | None | Structured logging |
| `HtmlFormatter` | `format_html` | None | Web tables (HTML) |
| `SqlFormatter` | `format_sql` | None | SQL INSERT statements |
| `JsonFormatter` | `format_json` | serde, serde_json | Data interchange, APIs |
| `YamlFormatter` | `format_yaml` | serde, serde_yaml | Configuration files |
| `TomlFormatter` | `format_toml` | serde, toml | Rust config files |
| `TextFormatter` | `format_text` | None | Human-readable lists |

#### Feature Bundles

| Bundle | Includes |
|--------|----------|
| `format_meta_visual` | `format_table` + `format_expanded` + `format_tree` + `format_logfmt` |
| `format_meta_web` | `format_html` + `format_sql` |
| `format_meta_data` | `format_json` + `format_yaml` + `format_toml` |
| `all_formats` | `format_meta_visual` + `format_meta_web` + `format_meta_data` + `format_text` + `themes` |

#### Feature Configuration

The `default` feature is empty — all formatters are opt-in. The `enabled` feature activates core dependencies and four default formatters: `table_plain`, `expanded_postgres`, `tree_hierarchical`, and `format_logfmt`. The `full` feature enables everything including `all_formats` and `terminal_size`. Each formatter meta-feature aggregates its variant flags (e.g., `format_table` includes all nine table variants). JSON, YAML, and TOML formatters require serde and pull it in via `serde_support` internally.

#### Cargo.toml Usage

Standard workspace integration uses the `enabled` feature for core dependencies and default visual formatters. Adding any formatter meta-feature (e.g., `format_json`) to the features list activates that formatter. Use `full` for all formatters including `terminal_size`. For the smallest binary, specify only the specific variant flag needed (e.g., `table_plain`) without `enabled`.

#### Usage Pattern

Build data once with `RowBuilder::build_view()` to produce a `TableView`, then pass it to any `Format`-implementing formatter. Each formatter is conditionally available based on its feature flag; callers wrap formatter code in the appropriate `cfg(feature = "...")` guard.

