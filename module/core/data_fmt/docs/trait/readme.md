# Trait Doc Entity

### Scope

- **Purpose**: Document interface contracts connecting input types to formatters.
- **Responsibility**: Registry and overview of all trait doc instances.
- **In Scope**: `Format`, `TableShapedView` — signatures, implementors, coverage gaps.
- **Out of Scope**: Callable method surfaces and return types (see `api/`), formatter implementation details (see `feature/`), variant output (see `variant/`).

Required instance sections (in order):

| Section | Heading | Required Content |
|---------|---------|-----------------|
| Scope | `### Scope` | 4 bullets: Purpose / Responsibility / In Scope / Out of Scope |
| Typed References | `### APIs`, `### Sources`, `### Tests` | Per-type `| File | Relationship |` table; `### Sources` and `### Tests` always last |
| Signature | `### Signature` | Full trait method signatures with parameter types and return types |
| Implementors | `### Implementors` | All known types that implement this trait, with the relevant type bounds |
| Coverage Gaps | `### Coverage Gaps` | Known missing implementations or types that should implement but don't |

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Format](001_format.md) | Modern unified trait accepting `&TableView` | ✅ |
| 002 | [TableShapedFormatter](002_table_shaped_formatter.md) | Legacy trait accepting a table-encoded tree — removed in v0.3.0 | ❌ |
| 003 | [TableShapedView](003_table_shaped_view.md) | Input-side trait for extracting tabular data from trees | ✅ |
