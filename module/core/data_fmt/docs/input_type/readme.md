# Input Type Doc Entity

### Scope

- **Purpose**: Document Rust types that carry data into formatters.
- **Responsibility**: Registry and overview of all input type doc instances.
- **In Scope**: Concrete Rust struct/enum type definitions — `TableView`, `TreeNode` specializations, their fields and generic parameters.
- **Out of Scope**: Abstract conceptual data shape models (see `input_model/`), formatting output (see `variant/`).

Required instance sections (in order):

| Section | Heading | Required Content |
|---------|---------|-----------------|
| Scope | `### Scope` | 4 bullets: Purpose / Responsibility / In Scope / Out of Scope |
| Typed References | `### InputModels`, `### APIs`, `### Sources`, `### Tests` | Per-type `| File | Relationship |` table; `### Sources` and `### Tests` always last |
| Type Definition | `### Type Definition` | Struct fields, generic parameters, trait bounds; source file reference |
| Specializations | `### Specializations` | Concrete instantiations used in the library (required when the type is generic) |

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [TableView](001_table_view.md) | Canonical tabular input for the `Format` trait | ✅ |
| 002 | [TreeNode](002_tree_node.md) | Generic hierarchical input with 3 specializations | ✅ |
