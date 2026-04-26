# Pattern Doc Entity

### Scope

- **Purpose**: Document recurring architectural decisions that shape how benchkit is designed and extended.
- **Responsibility**: Collects problem/solution pairs with applicability criteria and trade-off analysis.
- **In Scope**: Architectural patterns that differentiate benchkit from alternatives; composability decisions.
- **Out of Scope**: Implementation algorithms (→ feature/); API surface (→ api/).

### Overview Table

| ID  | Name                                                            | Purpose                                              | Status |
|-----|-----------------------------------------------------------------|------------------------------------------------------|--------|
| 001 | [Toolkit Not Framework](001_toolkit_not_framework.md)           | Composable tools over imposed workflow structure     | ✅ |
| 002 | [Markdown-First Reporting](002_markdown_first_reporting.md)     | Write results into version-controlled markdown files | ✅ |
