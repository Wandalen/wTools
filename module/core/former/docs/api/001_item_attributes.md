# API: Item Attributes

### Scope

- **Purpose**: Defines the set of attributes that appear on the type declaration and control global aspects of builder generation for the entire type.
- **Responsibility**: Documents item-level attribute names, their effects, valid combinations, and stability guarantees.
- **In Scope**: All attributes applied to the struct or enum declaration itself, their semantics, and interaction rules.
- **Out of Scope**: Field-level and variant-level attributes (→ api/002_field_attributes.md), the generated type structure (→ algorithm/001_variant_constructor_logic.md).

### Abstract

Item-level attributes modify type-wide builder generation behavior: whether extra storage fields are introduced, whether the mutator step is customized, whether a method is called after formation, whether standalone constructor functions are generated, and whether diagnostic output is emitted at compile time. These attributes shape the overall builder generation strategy before any field-level decisions are made.

### Operations

Five item-level attributes are available:

- **Storage fields**: Declares additional fields to include only in the intermediate storage — useful for temporary bookkeeping during a build session that must not appear in the final type.
- **Mutator custom**: Disables the automatically generated mutator implementation, requiring the developer to provide a hand-written one instead.
- **Perform**: Specifies a method name on the completed type that the builder calls automatically after the form step, enabling post-construction initialization without caller involvement.
- **Standalone constructors**: Activates generation of top-level constructor functions as an alternative to the builder chain interface. See feature/006_standalone_constructors.md for behavioral details.
- **Debug**: Emits a structured multi-phase diagnostic to the compiler output, showing generated components and the final token stream. Gated behind a feature flag with zero impact when disabled.

### Error Handling

Conflicting or structurally invalid attribute combinations produce compile-time errors before any code is generated. The macro validates attribute combinations in the parsing phase. Errors are associated with the specific attribute span that caused the issue.

### Compatibility Guarantees

Attribute names and semantics are stable within a major version. New attributes may be introduced in minor releases without breaking existing usage. Attribute removal follows a deprecation cycle of at least one minor release before removal in a subsequent major version.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [api/002_field_attributes.md](002_field_attributes.md) | Complementary field-level attribute reference |
| doc | [feature/006_standalone_constructors.md](../feature/006_standalone_constructors.md) | Feature activated by the standalone constructors attribute |
| doc | [feature/007_debug_attribute.md](../feature/007_debug_attribute.md) | Feature activated by the debug attribute |

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Primary source — section 2.4.1 item-level attribute table: attribute names, effects, and interaction rules; also produced 12 sibling instances — see entities.md |
