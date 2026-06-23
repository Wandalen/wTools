# Builder Test Spec Doc Entity

### Scope

- **Purpose**: Provide test coverage specifications for all builder doc instances.
- **Responsibility**: Registry and overview of all builder test spec instances.
- **In Scope**: BL-N builder API contract cases in Given/When/Then format for all 2 builder elements (`row_builder`, `tree_builder`); minimum 4 cases per spec.
- **Out of Scope**: Data model invariants (see `../invariant/`), API trait contracts (see `../api/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| [001](001_row_builder.md) | RowBuilder | BL-1..BL-8: row construction, chaining, details, validation | ⏳ |
| [002](002_tree_builder.md) | TreeBuilder | BL-9..BL-16: tree building, paths, batch, siblings | ⏳ |
