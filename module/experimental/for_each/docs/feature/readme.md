# Feature Doc Entity

### Scope

- **Purpose**: Document the user-facing capabilities of the `for_each` crate and their behavioral contracts.
- **Responsibility**: Master index of all feature doc instances; each instance captures one user-facing capability with its scope, design, and cross-references.
- **In Scope**: Invocation modes, design decisions, token-tree handling, and cross-references to source and test artifacts.
- **Out of Scope**: Public macro interface (→ `api/`); source code implementation details.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [For-Each Macro Iteration](001_for_each_iteration.md) | Compile-time iteration over a token list via declarative macros | ✅ |
