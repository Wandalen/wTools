# API: AsyncTryInto

### Scope

- **Purpose**: Provide the complementary fallible async conversion trait derived from AsyncTryFrom, enabling ergonomic call-site conversions.
- **Responsibility**: Defines the AsyncTryInto blanket implementation — how any type whose target implements AsyncTryFrom automatically acquires AsyncTryInto conversion capability.
- **In Scope**: AsyncTryInto trait, the async fallible conversion method, associated error type propagation, and thread-safety requirement.
- **Out of Scope**: Direct implementation instructions (blanket impl only) and infallible conversions.

### Abstract

AsyncTryInto parallels the standard TryInto trait: it is never implemented manually. The blanket implementation provides the fallible conversion method on any type whose target implements AsyncTryFrom. The associated error type is forwarded from the target's AsyncTryFrom implementation.

### Operations

`async_try_into` — Attempts to convert the receiver into the target type by delegating to the target's AsyncTryFrom implementation. Requires both the receiver type and the target type to be thread-safe. The associated error type is forwarded from the target's AsyncTryFrom implementation.

### Error Handling

Returns either the converted target value or the error from the target's AsyncTryFrom implementation. Callers handle the result using standard combinators after awaiting.

### Compatibility Guarantees

Blanket implementation — never implemented manually. Gated behind the `async_try_from` feature flag. Both the receiver type and the target type must be thread-safe.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | AsyncTryInto trait definition and blanket impl |
| test | `tests/inc/basic_test.rs` | Integration tests for AsyncTryInto |
| test | `tests/manual_corner_cases_test.rs` | AsyncTryInto via AsyncTryFrom edge cases |
| test | `tests/additional_corner_cases_test.rs` | AsyncTryInto corner cases via blanket impl |
| test | `tests/send_bounds_validation_test.rs` | Thread-safety bounds enforced by this blanket impl |
| doc | `docs/feature/002_fallible_async_conversion.md` | Feature scope and design rationale |
| doc | `docs/api/003_async_try_from.md` | AsyncTryFrom that this blanket derives from |
| doc | `docs/invariant/001_send_bounds.md` | Thread-safety bounds enforced by this blanket impl |
| doc | `docs/invariant/002_blanket_impl_chain.md` | Non-conflict invariant for blanket impls |
| doc | `docs/pattern/001_std_mirror_pattern.md` | Std mirror pattern governing this blanket's derivation model |
