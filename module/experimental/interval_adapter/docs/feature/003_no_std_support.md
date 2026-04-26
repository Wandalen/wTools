# Feature: no_std Support

### Scope

- **Purpose**: Enable use of `interval_adapter` in embedded, kernel, and other `no_std` environments without modification.
- **Responsibility**: Documents the `#![no_std]` guarantee, the `use_alloc` opt-in, zero production dependency status, and feature flag configuration.
- **In Scope**: `no_std` declaration, `use_alloc` feature, feature flag table, and applicable environments.
- **Out of Scope**: Interval API behavior (→ `api/`); iteration semantics (→ `data_structure/002`).

### Abstract

`interval_adapter` is declared `#![no_std]` and has zero production dependencies. It uses only `core` library types (`core::ops::Bound`, `core::ops::RangeBounds`). The `use_alloc` feature enables allocation support when the environment provides an allocator. This makes the crate suitable for embedded systems, kernels, and WASM environments without any configuration changes.

### Design

#### Feature Flags

| Feature | Default | Enables |
|---------|---------|---------|
| `enabled` | off | All functionality; master switch |
| `no_std` | off | `#![no_std]` — uses only `core` |
| `use_alloc` | off | `extern crate alloc` for heap allocation |
| `full` | off | All features: `enabled` + `no_std` + `use_alloc` |

#### Configuration

**Standard (std) environments:**
```toml
interval_adapter = { workspace = true, features = [ "enabled" ] }
```

**no_std with allocation:**
```toml
interval_adapter = { workspace = true, features = [ "enabled", "use_alloc" ] }
```

**Embedded (no alloc):**
```toml
interval_adapter = { workspace = true, features = [ "enabled", "no_std" ] }
```

### Constraints

- No heap allocation in core functionality — `Interval` and `IntervalIterator` are stack-allocated.
- `RangeBounds` and `Bound` are re-exported from `core::ops`, not `std::ops`.
- The `use_alloc` feature is provided for completeness; the current API does not require allocation.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [data_structure/001_interval.md](../data_structure/001_interval.md) | Stack-allocated canonical type |
| doc | [data_structure/002_interval_iterator.md](../data_structure/002_interval_iterator.md) | Stack-allocated iterator |
