# Invariant: Value Not Consumed

### Scope

- **Purpose**: Guarantee that the expression passed to `implements` and `instance_of` is never moved, dropped, or otherwise consumed by the macro — the caller retains full ownership after the call.
- **Responsibility**: Documents the non-consuming evaluation invariant — its precise statement, how it is enforced, and what breaks if it is violated.
- **In Scope**: The ownership status of the expression passed to both macros.
- **Out of Scope**: Borrow checker implications for the caller's subsequent use of the value (governed by Rust's ordinary borrow rules), values passed to the macro by move from a temporary.

### Invariant Statement

For all expressions passed to `implements` or `instance_of`: the macro takes a shared reference to the value internally, constructs a phantom type from that reference, and discards both without touching the original value. The caller's binding remains valid and fully owned after the macro call completes.

### Enforcement Mechanism

The internal mechanism constructs a `PhantomData` value by calling a helper function that accepts a shared reference — not the value itself. The `PhantomData` is a zero-sized type that carries no data and is immediately discarded after method resolution. The original expression is therefore not touched beyond the initial borrow, which ends within the macro expansion. The Rust borrow checker enforces that the temporary borrow is well-formed and does not outlive the expression.

### Violation Consequences

If the macro were to consume the expression, callers that pass non-Copy values would lose ownership silently at the check site — a breaking semantic change with no compile-time warning. Patterns like checking a value and then using it immediately after would become impossible without cloning, defeating the purpose of a lightweight diagnostic utility.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/implements_impl.rs` | Internal mechanism — shared reference capture via does() helper |
| doc | `docs/feature/001_trait_implementation_check.md` | Feature whose usability this invariant protects |
| doc | `docs/api/001_implements.md` | Primary macro this invariant constrains |
| doc | `docs/api/002_instance_of.md` | Alias macro equally constrained |
