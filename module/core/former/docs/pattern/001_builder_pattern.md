# Pattern: Builder Pattern

### Scope

- **Purpose**: Separates object construction from its representation by introducing an intermediate builder that accumulates field values before producing the final value.
- **Responsibility**: Documents the builder pattern as applied by the former macro — problem statement, solution structure, applicability, and consequences.
- **In Scope**: The builder pattern structure, participants, and the tradeoffs it addresses in the context of the former macro.
- **Out of Scope**: The derive macro that implements it (→ feature/001_struct_former.md), subformer delegation (→ pattern/002_subformer_composition.md).

### Problem

Constructing values with many optional or interdependent fields leads to verbose, error-prone initialization. Direct struct literals require every field at once, and multiple constructor overloads do not coexist cleanly. Callers need a way to set only the fields relevant to their context while relying on declared defaults for the rest, without sacrificing compile-time completeness guarantees.

### Solution

Introduce a companion builder type that mirrors the fields of the target type. Each field in the builder is optional. The caller sets only the fields it cares about via named setter methods, then calls a finalizing method to produce the target value. Unset fields resolve to declared defaults or produce a compile-time or runtime error if no default exists and the field is required.

The builder is consumed by the finalizing call — it cannot be reused after forming. Move semantics ensure the constructed value transfers cleanly without defensive copying. The setter methods accept any value convertible to the field type, widening the ergonomic surface.

### Applicability

Apply when:
- A type has more than a few fields and not all are required in every construction context
- The order of field assignment is not significant
- Default values make sense for most fields in most contexts
- A fluent, self-documenting construction API is desired

### Consequences

- Construction calls are self-documenting: each setter names the field it sets
- Fields with defaults require no explicit setter call
- The final value is fully initialized when the finalizing call succeeds
- Adding a required field to the target type propagates as a compile error to callers that must now provide it
- The builder type and all setter methods are generated automatically, eliminating repetitive boilerplate

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_struct_former.md](../feature/001_struct_former.md) | Macro feature implementing this pattern for structs |
| doc | [feature/002_enum_former.md](../feature/002_enum_former.md) | Macro feature implementing this pattern for enums |
| doc | [pattern/002_subformer_composition.md](002_subformer_composition.md) | Extension for nested builder delegation |

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Primary source — section 1.2 Guiding Principles and section 1.3 Key Terminology defining Former, Storage, and Definition |
| [../../readme.md](../../readme.md) | Combined source; basic usage and rationale described for end users |
