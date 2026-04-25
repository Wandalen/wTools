# Pattern Doc Entity

### Scope

- **Purpose**: Document the architectural design patterns applied in the `component_model` ecosystem.
- **Responsibility**: Collect one doc instance per applied pattern; each instance states the problem, solution, and consequences.
- **In Scope**: Patterns that explain why the crate architecture is structured the way it is.
- **Out of Scope**: Feature documentation (→ `feature/`); API reference (→ absorbed crates' `docs/api/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Absorption Pattern](001_absorption_pattern.md) | Three-crate structure that prevents circular dependencies in a macro+types ecosystem | ✅ |
