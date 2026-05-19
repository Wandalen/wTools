# Pattern Doc Entity

### Scope

- **Purpose**: Document architectural patterns and design decisions that govern the library.
- **Responsibility**: Registry and overview of all pattern doc instances.
- **In Scope**: Three-layer architecture, design principles, formatter design, config builder pattern.
- **Out of Scope**: API signatures (see `api/`), behavioral contracts (see `invariant/`), algorithm pseudocode (see `algorithm/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Three-Layer Architecture](001_three_layer_architecture.md) | Data / Builders / Formatters layering with module structure | ✅ |
| 002 | [Design Principles](002_design_principles.md) | Eleven guiding principles for library design decisions | ✅ |
| 003 | [Formatter Design](003_formatter_design.md) | Formatter trait hierarchy and TableShapedView decoupling | ✅ |
| 004 | [Config Builder Pattern](004_config_builder_pattern.md) | Fluent config construction across all formatter config types | ✅ |

### Organization

- **Architecture**: 001
- **Principles**: 002
- **Design**: 003–004
