# Trait Test Spec Doc Entity

### Scope

- **Purpose**: Provide test coverage specifications for all trait doc instances.
- **Responsibility**: Registry and overview of all trait test spec instances.
- **In Scope**: TR-N trait contract cases in Given/When/Then format for all 3 trait elements (`format`, `table_shaped_formatter`, `table_shaped_view`); minimum 4 cases per spec.
- **Out of Scope**: API-level contracts (see `../api/`), feature behavioral cases (see `../feature/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Format](001_format.md) | Trait method compliance, error variants, trait object dispatch | ✅ |
| 002 | [TableShapedFormatter](002_table_shaped_formatter.md) | Removed trait verification, migration path, Format trait replacement | ✅ |
| 003 | [TableShapedView](003_table_shaped_view.md) | Header extraction, structural check, row matrix extraction, edge cases | ✅ |
