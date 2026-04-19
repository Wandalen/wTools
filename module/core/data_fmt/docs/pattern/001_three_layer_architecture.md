# Pattern: Three-Layer Architecture

### Scope

- **Purpose**: Document the three-layer data/builders/formatters structure that organizes all library components.
- **Responsibility**: Complete description of the layer decomposition and module file structure.
- **In Scope**: Layer definitions (Data, Builders & Traits, Formatters), module file layout, inter-layer relationships.
- **Out of Scope**: Per-formatter details (see `feature/`), API signatures (see `api/`), formatter design patterns (see `003_formatter_design.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../architecture.md` | Original combined architecture document (retained per migration rules) |
| doc | `../feature/001_table_formatting.md` | Table formatting feature consuming Layer 3 |
| doc | `../api/004_formatters.md` | Formatter API surface in Layer 3 |
| doc | `../api/001_data_types.md` | Data types in Layer 1 |
| doc | `../api/002_builders.md` | Builders in Layer 2 |

### Problem

The library must serve as a generic multi-format data visualization tool — the same data must appear as horizontal tables, vertical records, hierarchical trees, JSON, YAML, and more. Without architectural separation, each formatter would need direct knowledge of how to traverse and interpret data, creating tight coupling between data representation and output format. A caller switching formatters would need to change how it constructs or passes data.

### Solution

A strict three-layer architecture separates concerns: data representation, ergonomic construction, and format-specific rendering. All data flows downward from Layer 1 through Layer 2 into Layer 3. Formatters never reach back into data; data is agnostic to formatting.

#### Layer 1: Data (TreeNode)

`TreeNode< T >` is the single data structure serving both hierarchical and tabular use cases. Hierarchical trees use `data = None` for directories and `data = Some(T)` for files. Table-shaped trees encode rows as children of root, with each row's children named after columns.

#### Layer 2: Builders and Traits

Ergonomic construction and generic extraction for table-shaped trees:

- `RowBuilder` — fluent and mutable APIs for building table-shaped trees
- `TableShapedView` trait — generic extraction of headers and rows from any `TreeNode< T >` where `T : Display`
- `TableView` — canonical interchange struct holding `headers` and `rows` for format-agnostic code

#### Layer 3: Formatters

Format-specific renderers that consume `TreeNode< T >` or `TableView`:

- `TableFormatter` — horizontal tabular display (9 style presets)
- `ExpandedFormatter` — vertical record display (PostgreSQL and property styles)
- `TreeFormatter` — hierarchical box-drawing display (with aligned column mode)
- `LogfmtFormatter` — structured logging key=value pairs
- `HtmlFormatter` — semantic HTML tables with CSS themes
- `SqlFormatter` — SQL INSERT statement generation (4 dialects)
- `JsonFormatter`, `YamlFormatter`, `TomlFormatter` — serialization formats
- `TextFormatter` — plain text output (6 styles)

All formatters implement the `Format` trait for a unified API.

#### Module File Structure

```
src/
  lib.rs                     # Re-exports public API
  data.rs                    # TreeNode, TableView struct, TableShapedView trait
  builder.rs                 # TreeBuilder (hierarchical)
  table_tree.rs              # RowBuilder (table-shaped)
  config.rs                  # TreeConfig, TableConfig, ExpandedConfig
  conversions.rs             # Tree<->Table conversions, FlattenConfig
  ansi_str.rs                # visual_len, pad_to_width, truncate_cell
  wrap.rs                    # WrapConfig, WrapFormatter, BreakStrategy, Overflow
  themes.rs                  # ColorTheme predefined and custom themes
  formatters/
    mod.rs                   # TableShapedFormatter trait (deprecated), Format trait re-export
    format_trait.rs          # Format trait, FormatError
    tree.rs                  # TreeFormatter with format() and format_aligned()
    table/                   # TableFormatter (split into directory)
    expanded.rs              # ExpandedFormatter
    logfmt.rs                # LogfmtFormatter
    html.rs                  # HtmlFormatter
    sql.rs                   # SqlFormatter
    json.rs                  # JsonFormatter
    yaml.rs                  # YamlFormatter
    toml_fmt.rs              # TomlFormatter
    text.rs                  # TextFormatter
```

### Applicability

Apply this pattern when organizing a library that renders the same data in multiple output formats. The three-layer separation is appropriate when: (1) multiple output formats must share a common data representation; (2) formatters must be interchangeable without caller code changes; (3) new formatters must be addable without modifying existing data or builder code.

### Consequences

The three-layer separation ensures formatters remain interchangeable: the same `TreeNode< T >` or `TableView` can be passed to any formatter without modification. Layer 2's `TableShapedView` trait decouples formatter logic from tree internals, so table-shaped formatters operate on flat vectors of strings rather than traversing tree structure directly. This enables the mutual replaceability design principle. The cost is that the tree encoding for tabular data is non-obvious — callers must use the builders rather than constructing trees directly.

### Sources

| File | Notes |
|------|-------|
| [../architecture.md](../architecture.md) | Original source; sections "Three-Layer Architecture" and "Module File Structure" extracted into this instance |
