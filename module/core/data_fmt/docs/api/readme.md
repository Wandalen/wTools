# API Doc Entity

### Scope

- **Purpose**: Document the public API surface — type signatures, method contracts, configuration options.
- **Responsibility**: Registry and overview of all API doc instances.
- **In Scope**: Data types, builders, config types, formatter APIs.
- **Out of Scope**: Behavioral invariants (see `invariant/`), feature guides (see `feature/`).

### Infrastructure

| File | Responsibility |
|------|----------------|
| `procedure.md` | Operational procedure for creating and updating API doc instances |

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Data Types](001_data_types.md) | Core data structures: TreeNode, TableView, ColumnData, TreeSymbols | ✅ |
| 002 | [Builders](002_builders.md) | Construction APIs: RowBuilder, TreeBuilder | ✅ |
| 003 | [Config Types](003_config_types.md) | Configuration types: TableConfig, ExpandedConfig, TreeConfig and enums | ✅ |
| 004 | [Formatters](004_formatters.md) | Formatter APIs: all formatters, Format trait, TableShapedFormatter trait | ✅ |
