# API: Collection Traits

### Scope

- **Purpose**: Enable standard collection types to participate in the builder pattern as subformers.
- **Responsibility**: Document the collection abstraction layer, entry and value operations, the generic collection builder, and per-collection extension traits.
- **In Scope**: Collection, CollectionAdd, CollectionAssign, entry-value conversion traits, CollectionFormer, per-collection definition types and extension traits.
- **Out of Scope**: Non-collection storage types, definition trait hierarchy, formation lifecycle.

### Abstract

Four traits establish the collection abstraction. The base trait declares the entry type — what callers pass in to add operations — and the value type — what the collection stores internally — plus the conversion from entry to value. The add trait provides single-entry insertion with a boolean success indicator. The assign trait provides bulk replacement by consuming an iterator of entries and returning the count added.

Three conversion traits handle the entry-value boundary from all directions: converting an entry to a value from the entry's perspective; converting a value to an entry from the collection's perspective; and converting a value to an entry from the value's perspective.

CollectionFormer is a generic builder for any collection that accepts single-entry insertion. It holds storage, optional context, and a completion handler in the same way a regular former does, but its mutation methods are specialized for collection building: adding one entry at a time with method chaining, or replacing the entire storage in one operation. This single generic type powers all eight standard collection builders.

Each standard collection type — Vec, HashMap, HashSet, BTreeMap, BTreeSet, LinkedList, VecDeque, BinaryHeap — receives a definition type set, a fully-parameterized builder type alias, and an extension trait that adds a builder entry point directly to the standard collection type.

### Operations

Collection abstraction:
- Declare entry type, value type, and entry-to-value conversion for a collection
- Add a single entry; indicate whether addition succeeded
- Replace all entries by consuming an iterator; return count added

Entry-value conversion:
- Convert entry to value from the entry's perspective
- Convert value to entry from the collection's perspective
- Convert value to entry from the value's perspective

Generic collection building:
- Initialize a collection builder with optional storage, context, and completion handler
- Add entries one at a time using method chaining
- Replace the entire storage contents in one operation
- Complete and return the formed result

Per-collection entry points:
- Obtain a collection-specific builder directly from the collection type via the former() method

### Error Handling

Single-entry insertion returns a boolean success indicator for insertions that can fail, such as duplicate entries in a set. Bulk assignment returns an insertion count. Compile-time errors arise when collection types do not satisfy the required bounds for use with the generic collection builder.

### Compatibility Guarantees

The four core collection traits are stable within a major version. Per-collection definition and builder type aliases are stable; their type parameter order is fixed. Extension traits adding new methods are additive and non-breaking.

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Combined source; collection traits section extracted here; remaining content migrated to api/001, api/002, api/003, algorithm/001, invariant/001, feature/001, feature/002 |

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | ../../src/collection.rs | Collection traits and CollectionFormer entry point |
| doc | api/002_formation_process.md | FormingEnd and FormerBegin used by CollectionFormer |
| doc | feature/002_collection_subformer.md | End-to-end collection subformer capability |
