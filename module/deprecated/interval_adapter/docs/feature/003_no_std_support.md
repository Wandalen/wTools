# Feature: no_std Support

### Scope

- **Purpose**: Enable use of `interval_adapter` in embedded, kernel, and other no-standard-library environments without modification.
- **Responsibility**: Documents the no-standard-library guarantee, the allocation opt-in, zero production dependency status, and feature flag configuration.
- **In Scope**: The no-standard-library declaration, the `use_alloc` feature, feature flag table, and applicable environments.
- **Out of Scope**: Interval API behavior (→ `api/`); iteration semantics (→ `data_structure/002`).

### Abstract

`interval_adapter` is declared for no-standard-library use and has zero production dependencies. It uses only core library types for bounds and range queries. The `use_alloc` feature enables allocation support when the environment provides an allocator. This makes the crate suitable for embedded systems, kernels, and WASM environments without any configuration changes.

### Design

#### Feature Flags

| Feature | Default | Enables |
|---------|---------|---------|
| `enabled` | off | All functionality; master switch |
| `no_std` | off | No-standard-library mode — uses only core library |
| `use_alloc` | off | Heap allocation when an allocator is available |
| `full` | off | All features: `enabled` + `no_std` + `use_alloc` |

#### Configuration

**Standard environments:** add `interval_adapter` to the workspace dependencies with the `enabled` feature.

**No-standard-library with allocation:** enable both `enabled` and `use_alloc` features.

**Embedded (no allocation):** enable both `enabled` and `no_std` features.

### Constraints

- No heap allocation in core functionality — the canonical interval type and its iterator are stack-allocated.
- Bound and range-query types are sourced from the core library, not the standard library.
- The `use_alloc` feature is provided for completeness; the current API does not require allocation.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [data_structure/001_interval.md](../data_structure/001_interval.md) | Stack-allocated canonical type |
| doc | [data_structure/002_interval_iterator.md](../data_structure/002_interval_iterator.md) | Stack-allocated iterator |
