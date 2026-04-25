# Pattern Doc Entity

### Scope

- **Purpose**: Document the architectural design pattern governing this crate's API surface.
- **Responsibility**: Define the problem, solution, applicability, and consequences of each pattern applied in this crate's design.
- **In Scope**: The std From/Into/TryFrom/TryInto mirroring design decision and its API shape implications.
- **Out of Scope**: Alternative async trait designs, runtime-specific patterns, and implementation details.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Std Mirror Pattern](001_std_mirror_pattern.md) | Mirrors std conversion traits in the async domain | ✅ |
