# API Doc Entity

### Scope

- **Purpose**: Document all public macros exported by implements — their operations, error handling, and compatibility contracts.
- **Responsibility**: Index of API doc instances for the implements crate.
- **In Scope**: Publicly exported macros and their behavioural contracts.
- **Out of Scope**: Internal implementation (→ src/), feature-level context (→ feature/), invariants (→ invariant/).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [implements](001_implements.md) | Check whether a type satisfies trait bounds, return bool | ✅ |
| 002 | [instance_of](002_instance_of.md) | Alias for implements — semantic variant for "instance of" phrasing | ✅ |
