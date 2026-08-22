# API Doc Entity

### Scope

- **Purpose**: Document the public API surface — type signatures, method contracts, configuration options.
- **Responsibility**: Registry and overview of all API doc instances.
- **In Scope**: Data types, builders, config types, formatter APIs, theme types, quantity formatting.
- **Out of Scope**: Behavioral invariants (see `invariant/`), feature guides (see `feature/`), trait interface contracts (see `trait/`), struct field semantics and type specializations (see `input_type/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Data Types](001_data_types.md) | Core data structures: TreeNode, TableView, ColumnData, TreeSymbols | ✅ |
| 002 | [Builders](002_builders.md) | Construction APIs: RowBuilder, TreeBuilder | ✅ |
| 003 | [Config Types](003_config_types.md) | Configuration types: TableConfig, ExpandedConfig, TreeConfig and enums | ✅ |
| 004 | [Formatters](004_formatters.md) | Formatter APIs: all formatters, Format trait | ✅ |
| 005 | [Theme Types](005_theme_types.md) | ColorTheme, ColorThemeBuilder, apply_to_* methods | ✅ |
| 006 | [Quantity Formatting](006_quantity_formatting.md) | QuantityStyle, DurationError, ten duration/number/byte formatters | ✅ |
