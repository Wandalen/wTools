# API: Storage Traits

### Scope

- **Purpose**: Define the interface for the intermediate container that holds field values during builder formation.
- **Responsibility**: Document the base container contract and the preformation conversion that produces the formed entity from accumulated state.
- **In Scope**: Storage, StoragePreform.
- **Out of Scope**: Collection storage, definition trait relationships, formation process sequencing.

### Abstract

Storage is the intermediate container that accumulates field values while a builder is active. Each builder has a dedicated storage type that starts in an empty state, allowing formation to begin with no initial values. The base Storage trait establishes this contract: every storage type must support default construction and must declare what preformed entity type it converts to.

StoragePreform extends Storage with the conversion operation itself. When invoked, it consumes the populated storage and produces the preformed entity — the result of applying any field defaults and transformations. This preformed entity is what the completion handler receives for final processing. For entities using the standard return-preformed completion handler, the preformed entity is the final result.

The split between Storage and StoragePreform reflects usage patterns: all storage types satisfy the base contract; only those whose owners use the preform-then-return pattern need the conversion operation.

### Operations

Base storage contract:
- Declare the preformed entity type that this storage converts to
- Construct an empty initial state via default construction

Preformation conversion:
- Consume the populated storage and produce the preformed entity

### Error Handling

Compile-time only. Storage types that fail to satisfy the required constraints produce errors at the point of definition use. Absent default construction prevents builder instantiation. Absent preformation conversion prevents use with the standard return-preformed completion handler.

### Compatibility Guarantees

The Storage associated type declaration is stable within a major version. Changing the preform method signature in StoragePreform constitutes a breaking change.

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Combined source; storage section extracted here; remaining content migrated to api/001, api/002, api/004, algorithm/001, invariant/001, feature/001, feature/002 |

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | ../../src/storage.rs | Storage trait implementations |
| doc | api/001_former_definition.md | Definition traits that reference the storage type |
| doc | algorithm/001_formation_lifecycle.md | Lifecycle step where storage is consumed and converted |
| doc | invariant/001_formation_integrity.md | Default-constructibility invariant enforced on storage |
