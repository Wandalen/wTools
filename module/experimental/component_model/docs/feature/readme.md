# Feature Doc Entity

### Scope

- **Purpose**: Document the user-facing capabilities provided by the `component_model` crate.
- **Responsibility**: Collect one navigational hub per capability; each instance cross-references all relevant source, test, and documentation artifacts across the three-crate ecosystem.
- **In Scope**: Implemented, committed features accessible through this crate's public API.
- **Out of Scope**: Internal implementation details in absorbed crates (→ their own `docs/`); architectural design decisions (→ `pattern/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Component Model](001_component_model.md) | End-to-end type-safe component assignment via derive macros | ✅ |
