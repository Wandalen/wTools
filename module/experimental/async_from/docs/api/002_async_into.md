# API: AsyncInto

### Scope

- **Purpose**: Provide the complementary async conversion trait derived from AsyncFrom, enabling ergonomic call-site conversions.
- **Responsibility**: Defines the AsyncInto blanket implementation — how any type whose target implements AsyncFrom automatically acquires AsyncInto conversion capability.
- **In Scope**: AsyncInto trait, the async conversion method, and the thread-safety requirement for the blanket impl.
- **Out of Scope**: Direct implementation instructions (blanket impl only) and fallible conversions.

### Abstract

AsyncInto parallels the standard Into trait: it is never implemented manually. The blanket implementation provides the conversion method on any type whose target type implements AsyncFrom. Callers await the result to complete the conversion; no explicit trait selection required.

### Operations

`async_into` — Consumes the receiver and produces a value of the target type by delegating to the target type's AsyncFrom implementation. Requires both the receiver type and the target type to be thread-safe.

### Error Handling

No error handling — this trait is infallible. It is derived from AsyncFrom whose contract is also infallible.

### Compatibility Guarantees

Blanket implementation — never implemented manually. Gated behind the `async_from` feature flag. Both the receiver type and the target type must be thread-safe, required by the async execution model for futures that may cross thread boundaries.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | AsyncInto trait definition and blanket impl |
| test | `tests/inc/basic_test.rs` | Integration tests for AsyncInto |
| test | `tests/manual_corner_cases_test.rs` | AsyncInto via AsyncFrom edge cases |
| test | `tests/additional_corner_cases_test.rs` | AsyncInto via AsyncFrom corner cases |
| test | `tests/send_bounds_validation_test.rs` | Thread-safety bounds enforced by this blanket impl |
| doc | `docs/feature/001_infallible_async_conversion.md` | Feature scope and design rationale |
| doc | `docs/api/001_async_from.md` | AsyncFrom that this blanket derives from |
| doc | `docs/invariant/001_send_bounds.md` | Thread-safety bounds enforced by this blanket impl |
| doc | `docs/invariant/002_blanket_impl_chain.md` | Non-conflict invariant for blanket impls |
