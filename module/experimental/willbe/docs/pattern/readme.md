# Pattern Doc Entity

### Scope

- **Purpose**: Document the architectural patterns and design decisions that shape willbe's internal structure.
- **Responsibility**: Registry and overview of all pattern doc instances.
- **In Scope**: Layer decomposition, module organization, design rationale.
- **Out of Scope**: API signatures (see `../api/`), feature behavior (see `../feature/`), implementation details.

### Files

| File | Responsibility |
|------|----------------|
| `procedure.md` | Instance creation and deprecation procedure |

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Layer Architecture](001_layer_architecture.md) | Five-layer CLI→Command→Action→Entity→Tool decomposition | ✅ |
