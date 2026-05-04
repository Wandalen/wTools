# Invariant: Value Not Consumed

### Scope

- **Purpose**: Guarantee that the expression passed to `is_slice` is never moved, dropped, or otherwise consumed by the macro — the caller retains full ownership after the call.
- **Responsibility**: Documents the non-consuming evaluation invariant — its precise statement, how it is enforced, and what breaks if it is violated.
- **In Scope**: The ownership status of the expression passed to is_slice.
- **Out of Scope**: The caller's subsequent use of the value under ordinary ownership rules.

### Invariant Statement

For all expressions passed to `is_slice`: the macro takes a shared reference to the value internally, constructs a temporary type marker from that reference, and discards both without touching the original value. The caller's binding remains valid and fully owned after the macro call completes.

### Enforcement Mechanism

The internal mechanism calls a helper that accepts a shared reference to the value — not the value itself. The temporary type marker constructed from that reference is discarded immediately after compile-time type dispatch concludes. The original expression is therefore not touched beyond the initial borrow, which ends within the macro expansion. The language's ownership system enforces that the temporary borrow is well-formed and does not outlive the expression.

### Violation Consequences

If the macro were to consume the expression, callers that pass non-Copy values would lose ownership silently at the detection site. Patterns like checking whether a value is a slice and then iterating over it immediately after would become impossible without cloning, making the macro impractical for real use.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Internal mechanism — shared reference capture via helper |
| doc | `docs/feature/001_slice_detection.md` | Feature whose usability this invariant protects |
| doc | `docs/api/001_is_slice.md` | API this invariant constrains |
