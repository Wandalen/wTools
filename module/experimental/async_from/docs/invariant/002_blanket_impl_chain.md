# Invariant: Blanket Impl Non-Conflict

### Scope

- **Purpose**: Ensure that the AsyncInto and AsyncTryInto blanket implementations do not conflict with each other or with user-defined implementations.
- **Responsibility**: States the non-overlap invariant for the two blanket impl chains and explains why coherence is preserved.
- **In Scope**: AsyncInto blanket derived from AsyncFrom, AsyncTryInto blanket derived from AsyncTryFrom, and compile-time coherence guarantees.
- **Out of Scope**: Orphan rules beyond these two chains and upstream standard library blanket impl interactions.

### Invariant Statement

The AsyncInto blanket impl and the AsyncTryInto blanket impl target distinct trait surfaces — AsyncInto is derived from AsyncFrom while AsyncTryInto is derived from AsyncTryFrom. Neither set is a subset of the other, so the two blanket impls never overlap for any concrete type pair, and no coherence conflict arises.

### Enforcement Mechanism

Rust's coherence rules prevent two blanket impls for the same trait and type pair from coexisting. Because AsyncInto and AsyncTryInto are distinct traits with distinct where clauses, no coherence conflict arises. A type implementing both AsyncFrom and AsyncTryFrom simultaneously acquires both AsyncInto and AsyncTryInto without conflict, because the target traits are different.

### Violation Consequences

A coherence conflict would produce a compile error for any type simultaneously acquiring both AsyncInto and AsyncTryInto. No such conflict exists because the blanket impls derive from different source traits. The invariant is structural — it cannot be violated without altering the trait definitions themselves.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Blanket impl for AsyncInto and AsyncTryInto |
| doc | `docs/api/002_async_into.md` | AsyncInto blanket impl |
| doc | `docs/api/004_async_try_into.md` | AsyncTryInto blanket impl |
