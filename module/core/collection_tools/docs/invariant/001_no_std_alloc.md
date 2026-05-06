# Invariant: No-std Allocation Selection

### Scope

- **Purpose**: Define the rule governing which `HashMap` and `HashSet` implementation is used based on feature flags.
- **Responsibility**: Document the conditional re-export contract between `std::collections` and `hashbrown`.
- **In Scope**: `HashMap` and `HashSet` source selection; `no_std` + `use_alloc` feature interaction; `hashbrown` dependency activation.
- **Out of Scope**: Other collection types (always from `std` or `alloc`); capacity pre-allocation behavior (see `002_capacity_preallocated.md`).

### Sources

| File | Relationship |
|------|-------------|
| `src/collection/hash_map.rs` | Conditional `HashMap` re-export (std vs hashbrown) |
| `src/collection/hash_set.rs` | Conditional `HashSet` re-export (std vs hashbrown) |

### Tests

| File | Relationship |
|------|-------------|
| `tests/no_std_alloc_test.rs` | Type-identity tests for hashbrown path (cfg-gated on use_alloc) |
| `tests/docs/invariant/01_no_std_alloc.md` | Test spec for this invariant |

### APIs

| File | Relationship |
|------|-------------|
| `../api/001_collection_macros.md` | Collection type re-export table |

### Features

| File | Relationship |
|------|-------------|
| `../feature/001_collection_constructors.md` | Strict macros whose HashMap/HashSet depend on this invariant |
| `../feature/002_into_constructors.md` | Into-based macros whose HashMap/HashSet depend on this invariant |

### Invariants

| File | Relationship |
|------|-------------|
| `002_capacity_preallocated.md` | Sibling invariant governing capacity pre-allocation |

### Invariant Statement

When both `no_std` and `use_alloc` features are active, `collection_tools::HashMap` and `collection_tools::HashSet` resolve to `hashbrown::HashMap` and `hashbrown::HashSet`. In all other configurations — when `no_std` is inactive — they resolve to `std::collections::HashMap` and `std::collections::HashSet`.

The selection rule is:

| Feature Configuration | `HashMap` / `HashSet` source |
|-----------------------|------------------------------|
| `no_std` + `use_alloc` both active | `hashbrown` crate |
| all other configurations (std enabled) | standard library |

All other collection types follow a simpler rule: when `use_alloc` is active they come from `alloc::collections::*`; otherwise from `std::collections::*`.

### Enforcement Mechanism

Enforced via `#[cfg(...)]` conditional compilation in each collection source file. The `hashbrown` crate is an optional workspace dependency declared with `optional = true`; it is activated only when the `use_alloc` feature is present. The `cfg` guards are mutually exclusive: exactly one branch is compiled per configuration.

### Violation Consequences

The invariant cannot be violated at runtime — configuration is resolved entirely at compile time. Attempting to use `HashMap` in a `no_std` environment without `use_alloc` will produce a compile error because `std::collections` is unavailable without `std`. Enabling `use_alloc` without `no_std` is benign: the `std` branch takes precedence and `hashbrown` is not activated.
