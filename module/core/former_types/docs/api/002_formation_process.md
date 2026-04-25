# API: Formation Process Traits

### Scope

- **Purpose**: Control the start, pre-completion mutation, and completion phases of builder formation.
- **Responsibility**: Define the three traits that govern how formation begins, how storage is transformed before completion, and how storage is converted to the final result.
- **In Scope**: FormingEnd, FormerMutator, FormerBegin, and the built-in completion handler implementations.
- **Out of Scope**: Storage structure, entity type definitions, collection-specific behavior.

### Abstract

Three traits manage the temporal flow of formation. FormingEnd is invoked at formation completion; it receives the populated storage and optional context and returns the final formed entity. Four built-in implementations cover the common cases: returning the preformed entity, returning the storage directly, raising a panic if called unexpectedly, and wrapping a closure for dynamic dispatch.

FormerMutator is a hook invoked immediately before FormingEnd. It receives mutable access to both storage and context, enabling last-moment transformation, default-filling, or validation before the completion handler runs. The default implementation is a no-op; each definition type overrides it as needed.

FormerBegin initiates a sub-former — a builder operating on behalf of a parent builder. It receives optional initial storage, optional context, and a completion handler, returning a fully initialized builder ready to accumulate state. FormerBegin is the integration point for nested struct formation and collection subformers.

### Operations

Completing formation:
- Receive populated storage and optional context; produce the final formed entity
- Common completion patterns: return preformed entity, return storage directly, delegate to closure

Pre-completion mutation:
- Receive mutable access to storage and context immediately before completion
- Apply defaults, validate, or transform storage values before FormingEnd runs

Starting a sub-former:
- Accept optional initial storage, optional context, and a completion handler
- Return an initialized builder ready to accumulate state

### Error Handling

All formation contracts are compile-time. FormingEnd implementations that accept the wrong storage or context types fail at instantiation. The NoEnd built-in panics at runtime if called — it is intended only as a placeholder for builders that must never complete through that path.

### Compatibility Guarantees

The three trait signatures are stable within a major version. Adding parameters to FormingEnd's call method or changing the FormerBegin parameter set constitutes a breaking change.

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Combined source; formation process section extracted here; remaining content migrated to api/001, api/003, api/004, algorithm/001, invariant/001, feature/001, feature/002 |

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | ../../src/forming.rs | Formation process trait implementations |
| doc | api/001_former_definition.md | Definition layer these traits operate within |
| doc | algorithm/001_formation_lifecycle.md | Lifecycle sequence that orders these three phases |
| doc | feature/001_builder_trait_infrastructure.md | End-to-end capability using these traits |
| doc | feature/002_collection_subformer.md | FormerBegin usage in collection subformers |
