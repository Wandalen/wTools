# Invariant: Memory Efficiency

### Scope

- **Purpose**: Caps heap allocation during template operations to prevent memory pressure on host tools.
- **Responsibility**: Documents the memory ceiling constraint and its measurement workload.
- **In Scope**: Peak heap allocation for up to 100 files and 1MB total template content.
- **Out of Scope**: Rendering latency (→ 001), filesystem buffering.

### Invariant Statement

In-memory template operations must not allocate more than 10MB of heap memory for typical use cases: up to 100 files with a total of 1MB of template content.

### Enforcement Mechanism

Measured via memory profiling during test suite execution. The in-memory design (no streaming) means all content fits in heap; the constraint bounds aggregate allocation. `MemoryFileSystem` tests are the primary verification workload.

### Violation Consequences

Exceeding 10MB heap for typical workloads makes genfile_core an unsuitable library for memory-constrained environments or tools that process many templates concurrently.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `docs/feature/012_memory_file_system.md` | Primary testing workload for this constraint |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | NFR2 in original spec; combined source migrated to invariant/. spec.md has been deleted — Sources entry retained as migration record. |
