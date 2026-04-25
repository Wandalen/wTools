# Pattern Doc Entity

### Scope
- **Purpose**: Document architectural patterns that guide macro_tools design and consumer usage.
- **Responsibility**: List reusable solutions to recurring proc-macro development problems.
- **In Scope**: Structural decisions and design patterns with cross-ecosystem applicability.
- **Out of Scope**: Implementation details; feature-specific usage guides → feature/.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Abstraction Layer](001_abstraction_layer.md) | Wrapping syn/quote/proc-macro2 with a stable higher-level surface | ✅ |
| 002 | [Property-Based Attributes](002_property_based_attributes.md) | Type-safe composable attribute parsing via typed properties | ✅ |
