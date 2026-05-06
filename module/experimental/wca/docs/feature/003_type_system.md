# Feature: Type System

Commands declare typed subjects and properties. The verifier casts raw string arguments to typed values at verification time, rejecting mismatches before execution.

### Scope

- **Purpose**: Provides compile-time-like type safety for CLI arguments at verification time.
- **Responsibility**: Documents the type enum, value enum, casting rules, and list support.
- **In Scope**: Type variants, value variants, string-to-value casting, list support with delimiter.
- **Out of Scope**: How types are declared in command registration (see feature/002).

### Design

Five type variants exist: String (passthrough), Number (parsed as a decimal number), Path (as a filesystem path), Bool (only accepts four literals), and List (parameterized by element type and delimiter character).

A type-casting mechanism converts a raw string into the declared target type, returning a typed value on success or an error on failure. Each value variant supports extraction of its inner primitive.

List is recursive: a List parameterized with Number and a semicolon delimiter splits on semicolon and casts each element as Number. This supports willbe's comma-separated path lists and feature lists.

Bool is strict: only "1", "true", "0", and "false" are accepted. Any other string is rejected.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/ca/grammar/types.rs` | Type/Value enums, TryCast, From impls |
| test | `tests/inc/grammar/types.rs` | Type casting and conversion tests |
| doc | [invariant/003_bool_accepted_values.md](../invariant/003_bool_accepted_values.md) | Bool strictness contract |
| doc | [api/002_grammar.md](../api/002_grammar.md) | Grammar API including Type/Value |
