# API: instance_of

### Scope

- **Purpose**: Provide a semantic alias for implements — allowing callers to express "does this value belong to this trait" using "instance of" phrasing rather than "implements" phrasing.
- **Responsibility**: Documents the instance_of macro — its relationship to implements and its identical contract.
- **In Scope**: Single-expression trait bound evaluation returning bool, using instance_of naming.
- **Out of Scope**: Any behaviour differing from implements (there is none).

### Abstract

A macro alias for `implements`. Accepts the same input form, applies the same autoref specialization mechanism, and returns the same bool result. The sole difference is the name. Provided for callers whose domain language uses "instance of" rather than "implements" as the canonical phrasing for trait membership queries.

### Operations

All operations are identical to `implements`. See api/001_implements.md for the full contract. The macro delegates entirely to the same internal mechanism — no separate implementation path exists.

### Error Handling

Identical to `implements`. See api/001_implements.md.

### Compatibility Guarantees

Identical to `implements`. The alias is unconditional — wherever `implements` is available, `instance_of` is available.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Public macro export and prelude re-export |
| source | `src/implements_impl.rs` | Shared internal mechanism |
| test | `tests/inc/test_cases.rs` | instance_of_basic test case |
| doc | `docs/feature/001_trait_implementation_check.md` | End-to-end feature context |
| doc | `docs/api/001_implements.md` | Primary macro this alias wraps |
| doc | `docs/invariant/001_value_not_consumed.md` | Non-consuming evaluation guarantee |
