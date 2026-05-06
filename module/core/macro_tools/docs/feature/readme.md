# Feature Doc Entity

### Scope
- **Purpose**: Navigate all artifacts for each user-facing macro toolkit capability.
- **Responsibility**: Collect source, test, and doc references for each capability in one place.
- **In Scope**: User-facing capabilities of macro_tools that consumers invoke or depend on directly.
- **Out of Scope**: Implementation details → src/ doc comments; architectural patterns → pattern/.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Attribute Parsing](001_attribute_parsing.md) | Structured parsing with property-based system | ✅ |
| 002 | [Type Analysis](002_type_analysis.md) | Type parameter extraction and container detection | ✅ |
| 003 | [Generic Parameters](003_generic_parameters.md) | Generic decomposition, merge, and filter | ✅ |
| 004 | [Syntax Tree Helpers](004_syntax_tree_helpers.md) | Item/struct parsing, identifiers, punctuation | ✅ |
| 005 | [Error Diagnostics](005_error_diagnostics.md) | Span-aware error creation and formatting | ✅ |
| 006 | [Code Generation Support](006_code_generation_support.md) | Token stream utilities and derive helpers | ✅ |
