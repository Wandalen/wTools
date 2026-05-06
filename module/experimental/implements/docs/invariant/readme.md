# Invariant Doc Entity

### Scope

- **Purpose**: Document the correctness properties that implements must maintain across all versions and callers.
- **Responsibility**: Index of invariant doc instances for the implements crate.
- **In Scope**: Properties that, if violated, would break callers or undermine the crate's design guarantees.
- **Out of Scope**: Feature documentation (→ feature/), API contracts (→ api/), implementation details.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Value Not Consumed](001_value_not_consumed.md) | The inspected expression is never moved or dropped by the macro | ✅ |
| 002 | [Zero Runtime Dependencies](002_zero_runtime_dependencies.md) | No transitive runtime deps introduced into caller projects      | ✅ |
| 003 | [Fn Trait Limitation](003_fn_trait_limitation.md)             | Callable trait check on named function items yields compile error | ✅ |
