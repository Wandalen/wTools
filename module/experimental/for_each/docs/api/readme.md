# API Doc Entity

### Scope

- **Purpose**: Document the public macro interface of the `for_each` crate exposed to external callers.
- **Responsibility**: Master index of all API doc instances; each instance captures one public macro contract with operations, error handling, and compatibility guarantees.
- **In Scope**: Macro names, invocation conventions, parameter semantics, and compatibility policy.
- **Out of Scope**: Feature design rationale (→ `feature/`); internal expansion mechanics.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Macro API](001_macro_api.md) | Public interface: `for_each!`, `braces_unwrap!`, `identity!` | ✅ |
