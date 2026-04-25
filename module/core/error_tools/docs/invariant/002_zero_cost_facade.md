# Invariant: Zero-Cost Facade

### Scope

- **Purpose**: Guarantee that the error facade introduces no runtime overhead compared to using upstream libraries directly.
- **Responsibility**: Documents the zero-cost facade invariant — its statement, enforcement points, and violation consequences.
- **In Scope**: The mandate that all re-exports are direct, introducing no wrapper types, adapter functions, or intermediate allocations.
- **Out of Scope**: Performance of the underlying upstream libraries themselves — those are upstream concerns.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_error_facade.md](../feature/001_error_facade.md) | Facade subject to this invariant |
| doc | [feature/002_typed_errors.md](../feature/002_typed_errors.md) | Typed component — re-export only |
| doc | [feature/003_untyped_errors.md](../feature/003_untyped_errors.md) | Untyped component — re-export only |

### Invariant Statement

All items exported by `error_tools` are pure re-exports of upstream items. No wrapper types, no forwarding functions, and no additional allocations are introduced at any layer of the facade.

### Enforcement Mechanism

- Source inspection: the typed and untyped modules contain only re-export declarations
- The error-with-report trait is a generic trait with blanket implementation; it adds no allocation
- The paired-result alias is a type alias, not a newtype struct
- No intermediate modules perform data transformation

### Violation Consequences

Any wrapper type or forwarding function would impose binary-size and possibly runtime costs on every consumer, violating the design contract that `error_tools` is cost-equivalent to direct upstream imports.
