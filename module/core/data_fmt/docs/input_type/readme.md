# Input Type Doc Entity

### Scope

- **Purpose**: Document Rust types that carry data into formatters.
- **Responsibility**: Registry and overview of all input type doc instances.
- **In Scope**: `TableView`, `TreeNode` specializations, and their fields.
- **Out of Scope**: Conceptual shapes (see `input_model/`), formatting output (see `variant/`).

#### Type-Specific Requirements

Every input_type doc instance must contain these sections in order:

| Section | Heading | Required Content |
|---------|---------|-----------------|
| Scope | `### Scope` | 4 bullets: Purpose / Responsibility / In Scope / Out of Scope |
| Cross-References | `### Cross-References` | Table: Type / File / Responsibility |
| Type Definition | `### Type Definition` | Struct fields, generic parameters, trait bounds; source file reference |
| Specializations | `### Specializations` | Concrete instantiations used in the library (required when the type is generic) |

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [TableView](001_table_view.md) | Canonical tabular input for the `Format` trait | ✅ |
| 002 | [TreeNode](002_tree_node.md) | Generic hierarchical input with 3 specializations | ✅ |
