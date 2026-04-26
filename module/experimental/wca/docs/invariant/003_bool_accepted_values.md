# Invariant: Bool Accepted Values

The Bool type accepts exactly four string literals during type casting: `1`, `true`, `0`, and `false`. Any other string value produces a type cast error.

### Scope

- **Purpose**: Defines the closed set of accepted boolean representations.
- **Responsibility**: Documents which strings convert to Bool and what happens for unrecognized values.
- **In Scope**: Accepted literals, case sensitivity, type cast error on rejection.
- **Out of Scope**: Other type conversions (see feature/003), verifier type resolution (see api/003).

### Invariant Statement

When the verifier casts a string argument to the Bool type, only the strings `1`, `true`, `0`, and `false` are accepted. The comparison is case-sensitive: `True`, `TRUE`, `yes`, `on`, and all other variants are rejected. The value `1` and `true` both produce the boolean true representation. The value `0` and `false` both produce the boolean false representation.

### Enforcement Mechanism

The TryCast implementation for Bool in the type system module performs an exact string match against the four accepted literals. On match, it returns the corresponding Value variant. On mismatch, it returns a type cast error containing the expected type name and the actual string that failed conversion. This error propagates through the verifier as a validation error.

### Violation Consequences

Passing a string not in the accepted set to a Bool-typed subject or property produces a verification error. The pipeline halts before execution. The error message identifies the expected type and the rejected value, enabling callers to correct their input.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/ca/grammar/types.rs` | TryCast implementation for Bool |
| test | `tests/inc/grammar/types.rs` | Boolean conversion tests |
| doc | [feature/003_type_system.md](../feature/003_type_system.md) | Type system overview |
| doc | [api/002_grammar.md](../api/002_grammar.md) | Type and Value enum documentation |
