# API Doc Entity

### Scope

- **Purpose**: Define the public programmatic interface of the former_types crate.
- **Responsibility**: Document public traits that external callers and generated code must satisfy.
- **In Scope**: All public traits — definition, formation-process, storage, and collection interfaces.
- **Out of Scope**: Internal implementation details, macro-generated code structure, runtime behavior.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Former Definition Traits](001_former_definition.md) | Type contracts linking entity to builder, storage, and context | ✅ |
| 002 | [Formation Process Traits](002_formation_process.md) | Traits controlling formation start, mutation, and completion | ✅ |
| 003 | [Storage Traits](003_storage.md) | Intermediate state container interface | ✅ |
| 004 | [Collection Traits](004_collection.md) | Collection integration and subformer interface | ✅ |
