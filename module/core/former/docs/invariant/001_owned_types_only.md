# Invariant: Owned Types Only

### Scope

- **Purpose**: Enforces that every field in a type deriving a builder holds owned data, preventing the generated storage from violating memory safety rules around lifetime erasure.
- **Responsibility**: Documents the owned-types constraint — its precise statement, the mechanism that enforces it, and the consequence of violating it.
- **In Scope**: The constraint on field types, the language-level reason it exists, and what triggers the violation.
- **Out of Scope**: The generic enum restriction (→ invariant/002_no_generic_enums.md), workaround guidance (→ sources below).

### Invariant Statement

For all types deriving a builder: every field must hold owned data. Fields carrying borrowed data with non-static lifetimes are not permitted. The builder's internal storage uses trait object erasure, which requires all contained values to satisfy the static lifetime bound. Static references are permitted; non-static borrowed references are not.

### Enforcement Mechanism

The Rust type system enforces this invariant at compile time. When the generated storage implementation attempts to store a value with a non-static borrowed type, the compiler rejects it with a lifetime error indicating that borrowed data escapes the method boundary. No runtime check is needed — the constraint is structural and detected during the macro expansion phase.

### Violation Consequences

Compilation fails with a lifetime error. The error is non-recoverable by configuration — the field type must be changed. Available owned alternatives include: owned strings, owned byte buffers, counted references, static string references, or data-owning wrappers. The specific error message is: "borrowed data escapes outside of method" with a note that the lifetime must outlive the static bound.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_struct_former.md](../feature/001_struct_former.md) | Struct builder feature — subject to this constraint |
| doc | [feature/002_enum_former.md](../feature/002_enum_former.md) | Enum builder feature — subject to this constraint |

### Sources

| File | Notes |
|------|-------|
| [../../limitations.md](../../limitations.md) | Primary source — Lifetime Constraint Limitation section (Limitation 2): verified examples and workaround guidance |
| [../../spec.md](../../spec.md) | Secondary source — section 3.1 Setter Method Implementation: move semantics rationale explaining why owned types are required |
