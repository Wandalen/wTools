# API Doc Entity

### Scope
- **Purpose**: Document public trait and type contracts that consumer crates depend on.
- **Responsibility**: List all stable programmatic interface contracts exposed by macro_tools.
- **In Scope**: Public traits, type aliases, and re-export contracts relied on by 12 consumer crates.
- **Out of Scope**: Implementation details → src/ doc comments; usage patterns → feature/.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Attribute Component API](001_attribute_component_api.md) | AttributeComponent and AttributePropertyComponent trait contract | ✅ |
| 002 | [Assign API](002_assign_api.md) | Assign trait for component-based field assignment | ✅ |
