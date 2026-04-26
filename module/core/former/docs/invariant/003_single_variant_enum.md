# Invariant: Single Variant Enum

### Scope

- **Purpose**: Prevents applying the builder macro to enum types with multiple variants that each trigger generation of the same trait implementation, which the compiler rejects as a duplicate.
- **Responsibility**: Documents the multi-variant trait conflict constraint — its precise statement, the enforcement point, and the consequence of violating it.
- **In Scope**: The trait duplication problem specific to multi-variant enums, the scope of affected variant combinations.
- **Out of Scope**: The generic enum parser restriction (→ invariant/002_no_generic_enums.md), the owned-types constraint (→ invariant/001_owned_types_only.md).

### Invariant Statement

An enum type annotated for builder derivation must not have multiple variants that each require the same trait implementation to be generated. The macro generates one trait implementation per variant using a uniform strategy; when multiple variants require the same trait, the compiler detects duplicate implementations. In practice, enums with a single active variant are safe; enums with multiple variants of similar structural complexity are likely to conflict.

### Enforcement Mechanism

The Rust compiler detects duplicate trait implementations after macro expansion and fails with a conflict error. The error identifies the conflicting implementations by trait and type. The macro itself does not detect the conflict before generating code.

### Violation Consequences

Compilation fails with a trait conflict error indicating conflicting implementations of a generated trait. The workaround is to split the multi-variant enum into separate single-variant enums, one per variant. Alternatively, marking individual variants with the scalar attribute changes the constructor strategy and may avoid the conflicting implementation.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/002_enum_former.md](../feature/002_enum_former.md) | Enum builder feature — subject to this constraint |

### Sources

| File | Notes |
|------|-------|
| [../../limitations.md](../../limitations.md) | Primary source — Trait Conflict Limitation section (Limitation 3): verified error, workaround, and engineering trade-off analysis; also produced 3 sibling instances: feature/002, invariant/001–002 |
