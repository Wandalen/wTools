# Feature: Type System

Commands declare typed subjects and properties. The verifier casts raw string arguments to typed values at verification time, rejecting mismatches before execution.

### Scope

- **Purpose**: Provides compile-time-like type safety for CLI arguments at verification time.
- **Responsibility**: Documents the type enum, value enum, casting rules, and list support.
- **In Scope**: Type variants, Value variants, TryCast trait, From conversions, List with delimiter.
- **Out of Scope**: How types are declared in command registration (see feature/002).

### Design

Five type variants exist: String (passthrough), Number (parsed as f64), Path (as PathBuf), Bool (only accepts four literals), and List (parameterized by element type and delimiter character).

The TryCast trait converts a raw string value into the target Type, returning a typed Value on success or an error on failure. Each Value variant provides From implementations for extracting the inner primitive.

List is recursive: List(Number, ';') splits on semicolon and casts each element as Number. This supports willbe's comma-separated path lists and feature lists.

Bool is strict: only "1", "true", "0", and "false" are accepted. Any other string is rejected.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/ca/grammar/types.rs` | Type/Value enums, TryCast, From impls |
| test | `tests/inc/grammar/types.rs` | Type casting and conversion tests |
| doc | [invariant/003_bool_accepted_values.md](../invariant/003_bool_accepted_values.md) | Bool strictness contract |
| doc | [api/002_grammar.md](../api/002_grammar.md) | Grammar API including Type/Value |
