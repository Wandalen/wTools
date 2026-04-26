# Trait Doc Entity

### Scope

- **Purpose**: Document interface contracts connecting input types to formatters.
- **Responsibility**: Registry and overview of all trait doc instances.
- **In Scope**: `Format`, `TableShapedFormatter`, `TableShapedView` — signatures, implementors, coverage gaps.
- **Out of Scope**: Formatter implementation details (see `feature/`), variant output (see `variant/`).

### Type-Specific Requirements

Every trait doc instance must contain these sections in order:

| Section | Heading | Required Content |
|---------|---------|-----------------|
| Scope | `### Scope` | 4 bullets: Purpose / Responsibility / In Scope / Out of Scope |
| Cross-References | `### Cross-References` | Table: Type / File / Responsibility |
| Signature | `### Signature` | Full trait method signatures with parameter types and return types |
| Implementors | `### Implementors` | All known types that implement this trait, with the relevant type bounds |
| Coverage Gaps | `### Coverage Gaps` | Known missing implementations or types that should implement but don't |

### Infrastructure

| File | Responsibility |
|------|----------------|
| `procedure.md` | Operational procedure for creating and updating trait doc instances |

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Format](001_format.md) | Modern unified trait accepting `&TableView` | ✅ |
| 002 | [TableShapedFormatter](002_table_shaped_formatter.md) | Legacy trait accepting a table-encoded tree | ✅ |
| 003 | [TableShapedView](003_table_shaped_view.md) | Input-side trait for extracting tabular data from trees | ✅ |
