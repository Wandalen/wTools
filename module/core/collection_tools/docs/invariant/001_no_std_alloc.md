# Invariant: No-std Allocation Selection

### Scope

- **Purpose**: Define the rule governing which `HashMap` and `HashSet` implementation is used based on feature flags.
- **Responsibility**: Document the conditional re-export contract between `std::collections` and `hashbrown`.
- **In Scope**: `HashMap` and `HashSet` source selection; `no_std` + `use_alloc` feature interaction; `hashbrown` dependency activation.
- **Out of Scope**: Other collection types (always from `std` or `alloc`); capacity pre-allocation behavior (see `002_capacity_preallocated.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/collection/hash_map.rs` | Conditional `HashMap` re-export |
| source | `src/collection/hash_set.rs` | Conditional `HashSet` re-export |
| doc | `../api/001_collection_macros.md` | Collection type re-export table |
| doc | `../feature/001_collection_constructors.md` | Feature depending on this invariant |

### Invariant Statement

When both `no_std` and `use_alloc` features are active, `collection_tools::HashMap` and `collection_tools::HashSet` resolve to `hashbrown::HashMap` and `hashbrown::HashSet`. In all other configurations — when `no_std` is inactive — they resolve to `std::collections::HashMap` and `std::collections::HashSet`.

The selection rule is:

```
no_std + use_alloc  →  hashbrown::HashMap, hashbrown::HashSet
otherwise           →  std::collections::HashMap, std::collections::HashSet
```

All other collection types follow a simpler rule: when `use_alloc` is active they come from `alloc::collections::*`; otherwise from `std::collections::*`.

### Enforcement Mechanism

Enforced via `#[cfg(...)]` conditional compilation in each collection source file. The `hashbrown` crate is an optional workspace dependency declared with `optional = true`; it is activated only when the `use_alloc` feature is present. The `cfg` guards are mutually exclusive: exactly one branch is compiled per configuration.

### Violation Consequences

The invariant cannot be violated at runtime — configuration is resolved entirely at compile time. Attempting to use `HashMap` in a `no_std` environment without `use_alloc` will produce a compile error because `std::collections` is unavailable without `std`. Enabling `use_alloc` without `no_std` is benign: the `std` branch takes precedence and `hashbrown` is not activated.

### Sources

Migrated from `../../spec.md`. Sections contributing to this instance: "Architecture → Feature-Based Collection Source", "Design Rationale → Why Conditional hashbrown Dependency", "Overview → In-Scope → No-std Support". Sibling extractions: `../api/001_collection_macros.md`, `../feature/001_collection_constructors.md`, `../feature/002_into_constructors.md`, `002_capacity_preallocated.md`.
