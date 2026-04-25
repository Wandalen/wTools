# API Doc Entity

### Scope

- **Purpose**: Document the public derive macro interface of `component_model_meta`.
- **Responsibility**: Collect one doc instance per derive macro group; each instance documents what the macro generates and how to use it.
- **In Scope**: Derive macro names, generated behavior, supported input shapes, and feature gating.
- **Out of Scope**: Internal macro implementation details (→ `algorithm/`); trait definitions (→ `component_model_types/docs/api/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Derive Macros](001_derive_macros.md) | Five derive macros for component model patterns: ComponentModel, Assign, ComponentsAssign, ComponentFrom, FromComponents | ✅ |
