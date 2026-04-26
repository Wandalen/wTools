# Invariant: No Generic Enums

### Scope

- **Purpose**: Prevents applying the builder macro to enum types that carry generic type parameters, as the macro parser cannot process generic syntax in enum declarations.
- **Responsibility**: Documents the generic enum constraint — its precise statement, the enforcement point, and the consequence of violating it.
- **In Scope**: The restriction on generic type parameters in enum declarations, the parser-level reason it exists.
- **Out of Scope**: Generic parameters on struct types (allowed), the multi-variant trait conflict (→ invariant/003_single_variant_enum.md).

### Invariant Statement

Enum types with generic type parameters cannot derive a builder. The macro uses a token-based parser that does not support generic syntax in enum declarations. Any enum annotated for builder derivation must use only concrete types in its variants. Static references are permitted; type parameters are not.

### Enforcement Mechanism

The macro parser fails at compile time upon encountering generic syntax in an enum declaration. The failure occurs before any code generation. The parser produces an error at the generic parameter position in the source.

### Violation Consequences

Compilation fails with a parser error at the generic parameter site. The error message indicates an unexpected token where the parser expected the variant body. The workaround is to replace generic type parameters with concrete types — for example, substituting the generic parameter with a specific owned type that satisfies the use case.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/002_enum_former.md](../feature/002_enum_former.md) | Enum builder feature — subject to this constraint |

### Sources

| File | Notes |
|------|-------|
| [../../limitations.md](../../limitations.md) | Primary source — Generic Enum Parsing Limitation section (Limitation 1): verified error, workaround, and trade-off analysis; also produced 3 sibling instances: feature/002, invariant/001, invariant/003 |
