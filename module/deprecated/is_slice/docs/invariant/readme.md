# Invariant Doc Entity

### Scope

- **Purpose**: Document the correctness properties that is_slice must maintain across all versions and callers.
- **Responsibility**: Index of invariant doc instances for the is_slice crate.
- **In Scope**: Properties that, if violated, would break callers or undermine the crate's design guarantees.
- **Out of Scope**: Feature documentation (→ feature/), API contracts (→ api/), implementation details.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [No Standard Library Required](001_no_std.md) | Crate operates without the standard library — only core is used | ✅ |
| 002 | [Value Not Consumed](002_value_not_consumed.md) | The inspected expression is never moved or dropped by the macro | ✅ |
