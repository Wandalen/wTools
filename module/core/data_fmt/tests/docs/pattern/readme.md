# Pattern Test Spec Doc Entity

### Scope

- **Purpose**: Provide test coverage specifications for all design pattern doc instances.
- **Responsibility**: Registry and overview of all pattern test spec instances.
- **In Scope**: PT-N design pattern verification cases in Given/When/Then format for all 4 pattern elements (`three_layer_architecture`, `design_principles`, `formatter_design`, `config_builder_pattern`); minimum 3 cases per spec.
- **Out of Scope**: Feature behavioral tests (see `../feature/`), API contract tests (see `../api/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Three-Layer Architecture](001_three_layer_architecture.md) | Layer 1/2/3 data types, builders, formatters observability | ⏳ |
| 002 | [Design Principles](002_design_principles.md) | Single data structure, unified format, granular features | ⏳ |
| 003 | [Formatter Design](003_formatter_design.md) | Format trait coverage, TableShapedView decoupling, dual output | ⏳ |
| 004 | [Config Builder Pattern](004_config_builder_pattern.md) | Fluent setters, default values, config-by-value construction | ⏳ |
