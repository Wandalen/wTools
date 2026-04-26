# Invariant: Struct-Only Input

### Scope

- **Purpose**: Guarantee that the Reflect derive macro only processes struct definitions.
- **Responsibility**: Document the input restriction, enforcement point, and compile-time failure mode.
- **In Scope**: Accepted and rejected item kinds, error site, enforcement location.
- **Out of Scope**: Supported struct varieties (→ `docs/api/001_reflect_derive.md`); generated code (→ `docs/feature/001_reflect_derive.md`).

### Invariant Statement

For every invocation of `#[derive(Reflect)]`, the annotated item must be a struct definition. All other item kinds — enums, unions, functions, trait definitions, impl blocks — are unconditionally rejected.

### Enforcement Mechanism

At the derive entry point, input is parsed using strict struct-only parsing. Any item that is not a struct fails parsing immediately. The failure is propagated as a compile error before any code generation begins.

### Violation Consequences

Applying the macro to a non-struct item produces a compile error at the call site. No code is generated. The error identifies the location of the unsupported derive attribute.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/implementation/reflect.rs` | Enforces struct-only parsing; rejects all other input forms |
| test | `tests/corner_cases_test.rs` | Edge case tests — includes non-struct input rejection |
| doc | `docs/api/001_reflect_derive.md` | API contract — Error Handling section |
| doc | `docs/feature/001_reflect_derive.md` | Feature design — supported input forms |
