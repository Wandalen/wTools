# API: Attribute Component

### Scope
- **Purpose**: Define the contract for types that parse from a single named attribute.
- **Responsibility**: Document the AttributeComponent and AttributePropertyComponent trait obligations.
- **In Scope**: Keyword declaration, parsing contract from a syn attribute node, property marker contract.
- **Out of Scope**: Concrete property implementations → feature/001; assignment contract → api/002.

### Abstract
Two complementary traits govern structured attribute parsing. The attribute component trait
is implemented by any type that represents a complete attribute — it declares the keyword
that triggers its parsing and provides a constructor from a syn attribute node, returning
a typed result with span-attached errors on failure. The attribute property component trait
is a marker implemented by each property type within the attribute struct — it declares the
keyword that identifies the property within the attribute token list, enabling type-directed
dispatch during parsing.

### Operations
- Declare keyword: the implementor names the exact attribute identifier it handles as a static string constant.
- Construct from syn attribute: given a syn attribute node, parse the contained tokens into the implementor type.
- Mark property keyword: each property type declares its own keyword constant for disambiguation within the attribute list.

### Error Handling
All parse failures return a span-attached error pointing to the offending attribute token.
Callers propagate errors back to the compiler by converting the error into a token stream
at the proc-macro boundary. No error is produced without a span.

### Compatibility Guarantees
Trait signatures are stable within a major version. Breaking changes require a major version
bump of macro_tools. All 12 consumer crates implement these traits; changes affect the
entire ecosystem and must be coordinated.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/attr.rs` | AttributeComponent and AttributePropertyComponent definitions |
| doc | `docs/feature/001_attribute_parsing.md` | End-to-end attribute parsing capability |
| doc | `docs/api/002_assign_api.md` | Assign trait used alongside this API |
| doc | `docs/pattern/002_property_based_attributes.md` | Design rationale for this trait system |
| doc | `docs/pattern/001_abstraction_layer.md` | Pattern providing the re-export foundation for this API |
