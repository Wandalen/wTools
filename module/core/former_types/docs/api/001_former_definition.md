# API: Former Definition Traits

### Scope

- **Purpose**: Provide the compile-time contracts that link an entity to its builder, storage, and context.
- **Responsibility**: Define the type relationships — storage, formed result, context, and completion handler — required to instantiate a builder.
- **In Scope**: FormerDefinitionTypes, FormerDefinition, and the four entity-to-X mapping traits.
- **Out of Scope**: Formation execution, storage manipulation, collection handling.

### Abstract

The definition traits establish the top layer of the builder type hierarchy. Two core traits declare what types participate in formation: the intermediate storage container, the final formed entity, the contextual information available during building, and the completion handler that converts storage to the result. The first declares the storage, formed, and context types. The second extends that with a mutator constraint and completion handler association.

Four entity-mapping traits project an entity onto specific aspects of its formation: a full-specification mapping that includes the completion handler; a type-set-only mapping that defers completion handler selection; a mapping to the concrete builder implementation; and a mapping to the storage container type. These four projections are the extension points used by generated code to integrate with the framework.

### Operations

Declaring a formation type set:
- Specify the three core associated types — storage container, formed result, and context — for a definition

Declaring a full formation definition:
- Extend a type set with a mutator and a completion handler to produce a buildable specification

Mapping an entity to its formation:
- Associate an entity with its full builder specification including completion handler
- Associate an entity with its type set only, deferring completion handler selection
- Associate an entity with its concrete builder implementation type
- Associate an entity with its storage container type

### Error Handling

All constraints are verified at compile time. Unsatisfied trait bounds or missing associated type declarations produce type errors at the point of use, not at runtime. Error messages identify the specific definition mismatch.

### Compatibility Guarantees

Associated type sets are stable within a major version. Adding new required associated types or strengthening bounds constitutes a breaking change requiring a major version increment.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | ../../src/definition.rs | Definition layer trait implementations |
| doc | algorithm/001_formation_lifecycle.md | Lifecycle that instantiates and sequences these definitions |
| doc | feature/001_builder_trait_infrastructure.md | End-to-end capability built on these traits |
| doc | api/002_formation_process.md | Process traits that operate within this definition layer |
| doc | api/003_storage.md | Storage type declared as associated type here |
| doc | invariant/001_formation_integrity.md | Invariant enforced on the types declared here |
