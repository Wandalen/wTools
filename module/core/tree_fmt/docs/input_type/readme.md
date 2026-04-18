# Input Type Doc Entity

### Scope

- **Purpose**: Document Rust types that carry data into formatters.
- **Responsibility**: Registry and overview of all input type doc instances.
- **In Scope**: `TableView`, `TreeNode<T>` specializations, their fields and trait bounds.
- **Out of Scope**: Conceptual shapes (see `input_model/`), formatting output (see `variant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [TableView](001_table_view.md) | Canonical tabular input for the `Format` trait | ✅ |
| 002 | [TreeNode](002_tree_node.md) | Generic hierarchical input with 3 specializations | ✅ |
