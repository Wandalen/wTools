# Pattern: Subformer Composition

### Scope

- **Purpose**: Enables hierarchical construction of complex nested types by composing builders, where a parent builder delegates field construction to a child builder and receives the completed value automatically.
- **Responsibility**: Documents the subformer composition pattern — problem, solution structure, delegation mechanism, and consequences.
- **In Scope**: The parent-child builder delegation model, the completion callback mechanism, and the three subformer variants it enables.
- **Out of Scope**: The foundational builder pattern (→ pattern/001_builder_pattern.md), attribute-level activation details (→ api/002_field_attributes.md).

### Problem

The basic builder pattern handles flat types well, but complex nested types require constructing values within values. Ad hoc approaches — constructing the nested value separately before calling the parent setter — break the fluent chain and force the caller to manage both builders explicitly. A principled delegation mechanism is needed so that nested construction remains inline and uninterrupted.

### Solution

When a field's type also supports builder construction, generate a nested builder method on the parent builder that carries a completion callback. The callback knows how to store the finished nested value back into the parent. The caller:

1. Invokes the nested builder method on the parent
2. Builds the nested value using the child builder's interface
3. Calls form on the child, which triggers the completion callback automatically
4. Receives the parent builder back, ready for further chaining

The completion callback is injected at the point the child builder is created, coupling the child to its specific parent field slot. The caller never references the callback directly.

Three specializations of this pattern exist for different field types: scalar subformer (single nested value), collection subformer (aggregate collection), and entry subformer (per-entry construction).

### Applicability

Apply when:
- A field's type itself supports builder construction
- Inline construction of the nested value within the parent chain is desired
- The caller should not need to manage both builders explicitly

### Consequences

- Nested construction remains fluent and self-contained in the calling code
- The parent-child relationship is explicit in the generated API surface
- Adding subformer delegation increases the complexity of the generated type infrastructure (definition types, completion callbacks) but does not increase user-facing call complexity
- The child builder is tightly coupled to a specific parent field slot via the callback, which means it cannot be reused as a standalone builder in the same invocation

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [pattern/001_builder_pattern.md](001_builder_pattern.md) | Foundational pattern that subformer composition extends |
| doc | [feature/003_subform_scalar.md](../feature/003_subform_scalar.md) | Scalar subformer feature using this pattern |
| doc | [feature/004_subform_collection.md](../feature/004_subform_collection.md) | Collection subformer feature using this pattern |
| doc | [feature/005_subform_entry.md](../feature/005_subform_entry.md) | Entry subformer feature using this pattern |

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Primary source — section 1.2 Guiding Principles (Composition over Configuration), section 1.3 Definition/Subformer terminology |
| [../../advanced.md](../../advanced.md) | Primary source; FormingEnd trait and completion callback mechanics described in detail |
