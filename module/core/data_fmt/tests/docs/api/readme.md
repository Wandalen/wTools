# API Test Spec Doc Entity

### Scope

- **Purpose**: Provide test coverage specifications for all API doc instances.
- **Responsibility**: Registry and overview of all API test spec instances.
- **In Scope**: AP-N API contract verification cases in Given/When/Then format for all 4 API elements (`data_types`, `builders`, `config_types`, `formatters`); minimum 4 cases per spec.
- **Out of Scope**: Algorithm correctness cases (see `../algorithm/`), feature behavioral cases (see `../feature/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [data_types](001_data_types.md) | API contracts for `TreeNode`, `TableView`, `TableMetadata`, `ColumnData`, `DataType` | 7 ✅ |
| 002 | [builders](002_builders.md) | API contracts for `RowBuilder`, `TreeBuilder`, `FlattenConfig`, flatten functions | 7 ✅ |
| 003 | [config_types](003_config_types.md) | API contracts for config builder setters, `Heading` type, `with_` prefix convention | 6 ✅ 2 ⏳ |
| 004 | [formatters](004_formatters.md) | API contracts for formatter constructors, `Format` trait, `FormatError`, `visual_len`, `pad_to_width` | 7 ✅ |
