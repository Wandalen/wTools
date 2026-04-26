# Feature: Collection Subformer

### Scope

- **Purpose**: Enable standard collection types to serve as builder-pattern subformers with element-at-a-time construction.
- **Responsibility**: Navigate all artifacts for the collection subformer capability — source files, api docs, and the core builder infrastructure it extends.
- **In Scope**: Collection abstraction traits, CollectionFormer generic builder, per-collection definition types, and extension traits for all eight supported standard collections.
- **Out of Scope**: Non-collection storage types, custom user-defined collections not implementing the collection traits.

### Design

Collection subformer support extends the core builder infrastructure to standard collection types. All collection building shares the same pattern: initialize empty, add entries one at a time, complete and return. This pattern is factored into a single generic builder parameterized by entry type and formation definition, eliminating per-collection boilerplate.

The collection trait layer adds three concerns to this generic builder: the entry type passed in by callers, the value type stored internally, and the entry-to-value conversion. The single-entry add trait restricts to collections that can accept one entry at a time; the assign trait covers bulk replacement.

Eight standard collections receive wiring: each gets a definition type set, a fully-parameterized builder type alias, and an extension trait that adds a builder entry point directly to the collection type. This makes the subformer accessible with minimal syntax from any call site, whether inside a generated builder or in direct user code.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | ../../src/collection.rs | Collection traits and CollectionFormer entry point |
| source | ../../src/collection/vector.rs | Vec collection implementation |
| source | ../../src/collection/hash_map.rs | HashMap collection implementation |
| source | ../../src/collection/hash_set.rs | HashSet collection implementation |
| source | ../../src/collection/btree_map.rs | BTreeMap collection implementation |
| source | ../../src/collection/btree_set.rs | BTreeSet collection implementation |
| source | ../../src/collection/linked_list.rs | LinkedList collection implementation |
| source | ../../src/collection/vector_deque.rs | VecDeque collection implementation |
| source | ../../src/collection/binary_heap.rs | BinaryHeap collection implementation |
| doc | api/004_collection.md | Collection traits API contract |
| doc | api/002_formation_process.md | FormingEnd and FormerBegin used by CollectionFormer |
| doc | feature/001_builder_trait_infrastructure.md | Core builder infrastructure this capability extends |
