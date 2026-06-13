# Pattern: Three-Layer Architecture

### Scope

- **Purpose**: Document the three-layer data/builders/formatters structure that organizes all library components.
- **Responsibility**: Complete description of the layer decomposition and module file structure.
- **In Scope**: Layer definitions (Data, Builders & Traits, Formatters), module file layout, inter-layer relationships.
- **Out of Scope**: Per-formatter details (see `feature/`), API signatures (see `api/`), formatter design patterns (see `003_formatter_design.md`).

### APIs

| File | Relationship |
|------|-------------|
| [004_formatters.md](../api/004_formatters.md) | Formatter API surface in Layer 3 |
| [001_data_types.md](../api/001_data_types.md) | Data types in Layer 1 |
| [002_builders.md](../api/002_builders.md) | Builders in Layer 2 |

### Features

| File | Relationship |
|------|-------------|
| [001_table_formatting.md](../feature/001_table_formatting.md) | Table formatting feature consuming Layer 3 |

### Problem

The library must serve as a generic multi-format data visualization tool — the same data must appear as horizontal tables, vertical records, hierarchical trees, JSON, YAML, and more. Without architectural separation, each formatter would need direct knowledge of how to traverse and interpret data, creating tight coupling between data representation and output format. A caller switching formatters would need to change how it constructs or passes data.

### Solution

A strict three-layer architecture separates concerns: data representation, ergonomic construction, and format-specific rendering. All data flows downward from Layer 1 through Layer 2 into Layer 3. Formatters never reach back into data; data is agnostic to formatting.

#### Layer 1: Data (TreeNode)

`TreeNode` is the single data structure serving both hierarchical and tabular use cases. Hierarchical trees use absent data for directory nodes and present data for leaf nodes. Table-shaped trees encode rows as children of root, with each row's children named after columns.

#### Layer 2: Builders and Traits

Ergonomic construction and generic extraction for table-shaped trees:

- `RowBuilder` — fluent and mutable APIs for building table-shaped trees
- `TableShapedView` trait — generic extraction of headers and rows from any tree node whose data supports display formatting
- `TableView` — canonical interchange struct holding `headers` and `rows` for format-agnostic code

#### Layer 3: Formatters

Format-specific renderers that consume `TreeNode` or `TableView`:

- `TableFormatter` — horizontal tabular display (9 style presets)
- `ExpandedFormatter` — vertical record display (PostgreSQL and property styles)
- `TreeFormatter` — hierarchical box-drawing display (with aligned column mode)
- `LogfmtFormatter` — structured logging key=value pairs
- `HtmlFormatter` — semantic HTML tables with CSS themes
- `SqlFormatter` — SQL INSERT statement generation (4 dialects)
- `JsonFormatter`, `YamlFormatter`, `TomlFormatter` — serialization formats
- `TextFormatter` — plain text output (6 styles)

Nine of ten formatters implement the `Format` trait for a unified API; `TreeFormatter` uses direct method dispatch without the trait.

#### Module File Structure

| Module | Responsibility |
|--------|----------------|
| `lib.rs` | Re-exports public API |
| `data.rs` | Core data types and view trait |
| `builder.rs` | TreeBuilder for hierarchical trees |
| `table_tree.rs` | RowBuilder for table-shaped trees |
| `config.rs` | All configuration types |
| `conversions.rs` | Tree-to-table conversion utilities |
| `ansi_str.rs` | ANSI-aware string width utilities |
| `wrap.rs` | Word-wrap config and formatter |
| `themes.rs` | Color theme definitions |
| `formatters/` | Per-formatter modules, one per formatter type |

### Applicability

Apply this pattern when organizing a library that renders the same data in multiple output formats. The three-layer separation is appropriate when: (1) multiple output formats must share a common data representation; (2) formatters must be interchangeable without caller code changes; (3) new formatters must be addable without modifying existing data or builder code.

### Consequences

The three-layer separation ensures formatters remain interchangeable: the same `TreeNode` or `TableView` can be passed to any formatter without modification. Layer 2's `TableShapedView` trait decouples formatter logic from tree internals, so table-shaped formatters operate on flat vectors of strings rather than traversing tree structure directly. This enables the mutual replaceability design principle. The cost is that the tree encoding for tabular data is non-obvious — callers must use the builders rather than constructing trees directly.
