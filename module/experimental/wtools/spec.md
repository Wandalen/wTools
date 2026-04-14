# wtools

Main aggregator crate providing unified access to the wTools ecosystem of general-purpose Rust utilities.

## Overview

`wtools` is the primary entry point for the wTools ecosystem - a comprehensive collection of general-purpose tools that fundamentally extend Rust's capabilities without polluting or spoiling the language. It aggregates multiple specialized crates under a single, feature-gated dependency, allowing users to include exactly the functionality they need.

The crate follows the philosophy of providing clean, composable utilities that work well together or independently. Each aggregated module maintains its own namespace while also exposing commonly-used items through the traditional own/orphan/exposed/prelude hierarchy.

### Scope

#### Responsibility

wtools is responsible for aggregating and re-exporting the wTools ecosystem under a unified interface. It manages feature flag composition, provides consistent namespace organization across all sub-crates, and ensures that users can access the entire toolkit through a single dependency.

#### In-Scope

- **Aggregation**: Re-export all wTools ecosystem crates under feature flags
- **Namespace unification**: Provide `iter`, `meta`, `mem`, `typing`, `time`, `string`, `error`, `derive`, `dt`, `diagnostics` modules
- **Feature composition**: Granular feature flags for fine-grained control
- **Dependency management**: Single Cargo.toml entry for entire ecosystem
- **Prelude organization**: Unified prelude combining all sub-crate preludes

#### Out-of-Scope

- **Implementation logic**: All functionality lives in sub-crates
- **New utilities**: wtools provides access, not new features
- **Breaking encapsulation**: Sub-crates remain independently usable

#### Boundaries

- **Upstream**: Aggregates iter_tools, meta_tools, mem_tools, typing_tools, time_tools, strs_tools, error_tools, derive_tools, data_type, diagnostics_tools
- **Downstream**: End-user applications and libraries
- **Feature boundary**: Each sub-crate has its own feature namespace

## Architecture

### Module Structure

```
wtools/
├── src/
│   └── lib.rs            # Aggregation and namespace organization
├── examples/
│   └── wtools_trivial.rs # Basic usage example
├── Cargo.toml            # Feature-gated dependencies
├── readme.md
└── spec.md
```

### Namespace Organization

```rust
wtools
├── dependency           # Raw crate access
│   ├── meta_tools
│   ├── mem_tools
│   ├── typing_tools
│   ├── time_tools
│   ├── strs_tools
│   ├── error_tools
│   ├── derive_tools
│   ├── diagnostics_tools
│   └── ...
├── own                   # Owned namespace with aliased modules
│   ├── iter          → iter_tools
│   ├── meta          → meta_tools
│   ├── mem           → mem_tools
│   ├── typing        → typing_tools
│   ├── time          → time_tools
│   ├── string        → strs_tools
│   ├── error         → error_tools
│   ├── derive        → derive_tools
│   ├── dt            → data_type
│   └── diagnostics   → diagnostics_tools
├── orphan               # Re-exports exposed
├── exposed              # Combined exposed from all sub-crates
└── prelude              # Combined prelude from all sub-crates
```

### Feature Flag Hierarchy

Each aggregated crate has a corresponding feature namespace:

```
feature_category/
├── feature               # Enable the sub-crate
├── feature_default       # Default features
├── feature_full          # All features
├── feature_no_std        # no_std support
├── feature_use_alloc     # alloc in no_std
└── feature_*             # Specific sub-features
```

## Public API

### Aggregated Modules

When features are enabled, the following modules become available:

| Module | Feature | Source Crate | Description |
|--------|---------|--------------|-------------|
| `iter` | `iter` | `iter_tools` | Iterator utilities and extensions |
| `meta` | `meta` | `meta_tools` | Metaprogramming utilities |
| `mem` | `mem` | `mem_tools` | Memory manipulation utilities |
| `typing` | `typing` | `typing_tools` | Type inspection and manipulation |
| `time` | `time` | `time_tools` | Time and duration utilities |
| `string` | `string` | `strs_tools` | String manipulation utilities |
| `error` | `error` | `error_tools` | Error handling utilities |
| `derive` | `derive` | `derive_tools` | Derive macro collection |
| `dt` | `dt` / `data_type` | `data_type` | Data type utilities |
| `diagnostics` | `diagnostics` | `diagnostics_tools` | Diagnostic utilities |
| `former` | `former` / `meta_former` | via `meta_tools` | Builder pattern |
| `options` | `options` / `meta_options` | via `meta_tools` | Options pattern |

### Dependency Namespace

Raw access to underlying crates:

```rust
// Direct crate access
use wtools::dependency::meta_tools;
use wtools::dependency::typing_tools;
use wtools::dependency::error_tools;
```

### Standard Namespaces

Each module follows the traditional namespace pattern:

```rust
// Full module access
use wtools::iter;
use wtools::meta;
use wtools::error;

// Exposed items (commonly used)
use wtools::exposed::*;

// Prelude (essentials)
use wtools::prelude::*;
```

## Feature Flags

### Category: iter (Iterator Tools)

| Feature | Description |
|---------|-------------|
| `iter` | Enable iter_tools |
| `iter_default` | Default iterator features |
| `iter_full` | All iterator features |
| `iter_no_std` | no_std support |
| `iter_use_alloc` | alloc in no_std |

### Category: meta (Metaprogramming)

| Feature | Description |
|---------|-------------|
| `meta` | Enable meta_tools |
| `meta_default` | Default: for_each, impls_index, mod_interface, idents_concat |
| `meta_full` | All meta features |
| `meta_no_std` | no_std support |
| `meta_for_each` | for_each! macro |
| `meta_impls_index` | impls!/index! macros |
| `meta_mod_interface` | mod_interface! macro |
| `meta_idents_concat` | Identifier concatenation |

### Category: mem (Memory Tools)

| Feature | Description |
|---------|-------------|
| `mem` | Enable mem_tools |
| `mem_default` | Default memory features |
| `mem_full` | All memory features |
| `mem_no_std` | no_std support |
| `mem_use_alloc` | alloc in no_std |

### Category: typing (Type Tools)

| Feature | Description |
|---------|-------------|
| `typing` | Enable typing_tools |
| `typing_default` | Default: inspect_type, is_slice, implements |
| `typing_full` | All typing features |
| `typing_no_std` | no_std support |
| `typing_inspect_type` | Type inspection |
| `typing_is_slice` | Slice detection |
| `typing_implements` | Trait implementation checking |

### Category: time (Time Tools)

| Feature | Description |
|---------|-------------|
| `time` | Enable time_tools |
| `time_default` | Default: now |
| `time_full` | All time features |
| `time_no_std` | no_std support |
| `time_now` | Current time functions |

### Category: string (String Tools)

| Feature | Description |
|---------|-------------|
| `string` | Enable strs_tools |
| `string_default` | Default: indentation, isolate, parse_*, split |
| `string_full` | All string features |
| `string_no_std` | no_std support |
| `string_indentation` | Indentation utilities |
| `string_isolate` | String isolation |
| `string_parse_request` | Request parsing |
| `string_parse_number` | Number parsing |
| `string_split` | String splitting |

### Category: error (Error Tools)

| Feature | Description |
|---------|-------------|
| `error` | Enable error_tools |
| `error_default` | Default: typed, untyped |
| `error_full` | All error features |
| `error_no_std` | no_std support |
| `error_typed` | Typed errors |
| `error_untyped` | Untyped errors |

### Category: derive (Derive Macros)

| Feature | Description |
|---------|-------------|
| `derive` | Enable derive_tools |
| `derive_default` | Common derive macros |
| `derive_full` | All derive macros |
| `derive_no_std` | no_std support |
| `derive_add`, `derive_add_assign` | Arithmetic derives |
| `derive_as_ref`, `derive_as_mut` | Reference derives |
| `derive_deref`, `derive_deref_mut` | Deref derives |
| `derive_from`, `derive_inner_from` | From derives |
| `derive_display`, `derive_from_str` | Display/FromStr |
| `derive_clone_dyn` | Clone for dyn traits |
| `derive_strum`, `derive_strum_phf` | Strum integration |
| `derive_is_variant`, `derive_unwrap` | Enum utilities |

### Category: dt (Data Types)

| Feature | Description |
|---------|-------------|
| `dt` | Enable data_type |
| `dt_default` | Default: either, interval |
| `dt_full` | All data type features |
| `dt_use_alloc` | alloc support |
| `dt_either` | Either type |
| `dt_interval` | Interval type |

### Category: diagnostics

| Feature | Description |
|---------|-------------|
| `diagnostics` | Enable diagnostics_tools |
| `diagnostics_default` | Default: runtime/compiletime assertions |
| `diagnostics_full` | All diagnostic features |
| `diagnostics_no_std` | no_std support |
| `diagnostics_runtime_assertions` | Runtime assertions |
| `diagnostics_compiletime_assertions` | Compile-time assertions |

### Meta Features

| Feature | Description |
|---------|-------------|
| `enabled` | Enable the crate |
| `default` | All default features from all categories |
| `full` | All full features from all categories |
| `no_std` | Global no_std mode |
| `use_alloc` | alloc in global no_std |
| `nightly` | Nightly-only features |

## Usage Patterns

### Minimal Usage

```rust
// Enable just what you need
[dependencies]
wtools = { version = "0.2", default-features = false, features = ["iter", "error"] }
```

```rust
use wtools::prelude::*;

fn main() -> Result<(), wtools::error::untyped::Error>
{
  let sum: i32 = wtools::iter::exposed::Itertools::sum( [ 1, 2, 3 ].into_iter() );
  Ok(())
}
```

### Full Ecosystem Access

```rust
[dependencies]
wtools = { version = "0.2", features = ["full"] }
```

```rust
use wtools::prelude::*;

// Access everything
use wtools::{ iter, meta, mem, typing, time, string, error, derive, dt, diagnostics };
```

### Feature-Specific Import

```rust
use wtools::typing::exposed::*;
use wtools::error::exposed::*;
use wtools::derive::exposed::*;
```

### no_std Usage

```rust
[dependencies]
wtools = { version = "0.2", default-features = false, features = [
  "iter_no_std",
  "meta_no_std",
  "typing_no_std",
  "use_alloc"
] }
```

### Accessing Sub-Crate Directly

```rust
// For when you need the raw crate
use wtools::dependency::typing_tools;
use wtools::dependency::error_tools;
```

## Dependencies and Consumers

### Aggregated Crates

| Crate | Feature | Category |
|-------|---------|----------|
| `iter_tools` | `iter` | Iterator utilities |
| `meta_tools` | `meta` | Metaprogramming |
| `mem_tools` | `mem` | Memory utilities |
| `typing_tools` | `typing` | Type utilities |
| `time_tools` | `time` | Time utilities |
| `strs_tools` | `string` | String utilities |
| `error_tools` | `error` | Error handling |
| `derive_tools` | `derive` | Derive macros |
| `data_type` | `dt` | Data types |
| `diagnostics_tools` | `diagnostics` | Diagnostics |
| `impls_index` | (always) | Function indexing |
| `parse-display` | `derive_display` | Display/FromStr |

### Potential Consumers

- Applications wanting unified wTools access
- Libraries building on multiple wTools crates
- Projects migrating from multiple dependencies to single entry point
- no_std projects needing curated utility set

## Design Rationale

### Why Aggregation?

1. **Simplified dependency management**: One line in Cargo.toml
2. **Version coherence**: All sub-crates tested together
3. **Discoverability**: All tools in one place
4. **Reduced boilerplate**: Single prelude import

### Why Feature Flags?

1. **Compile time**: Only compile what you use
2. **Binary size**: Exclude unused code
3. **Flexibility**: Mix and match features
4. **no_std support**: Granular std/alloc/no_std control

### Why Module Aliases?

The `own` namespace uses short aliases:
- `iter` instead of `iter_tools`
- `string` instead of `strs_tools`
- `dt` instead of `data_type`

This provides cleaner, more intuitive module paths.

### Why Namespace Hierarchy?

Following the traditional own/orphan/exposed/prelude pattern:
- Consistent with all sub-crates
- Predictable import patterns
- Clear visibility boundaries

## Testing Strategy

### Test Categories

1. **Integration tests**: Verify aggregation works correctly
2. **Feature combination tests**: Test various feature combinations
3. **no_std tests**: Verify no_std compatibility

### Running Tests

```bash
# Default features
cargo test

# Full features
cargo test --features full

# Specific feature set
cargo test --no-default-features --features "iter,error"
```

## Future Considerations

### Potential Enhancements

1. **Workspace-level features**: Cargo feature unification
2. **Documentation aggregation**: Unified docs.rs
3. **Version pinning**: Explicit sub-crate version constraints
4. **Profile-based defaults**: Different defaults per build profile

### Known Limitations

1. **Feature explosion**: Many feature combinations possible
2. **Documentation fragmentation**: Docs split across sub-crates
3. **Compile time**: Full features increase compile time

## Adoption Guidelines

### When to Use wtools

- Need multiple wTools crates
- Want single dependency entry point
- Value version coherence
- Building comprehensive Rust applications

### When to Use Individual Crates

- Need only one or two specific tools
- Want minimal dependency footprint
- Finer control over versions
- Library development with minimal deps

### Migration Path

```rust
// Before: multiple dependencies
[dependencies]
iter_tools = "0.18"
error_tools = "0.17"
derive_tools = "0.27"

// After: single dependency
[dependencies]
wtools = { version = "0.2", features = ["iter", "error", "derive"] }
```

## Related Crates

| Crate | Relationship |
|-------|--------------|
| `iter_tools` | Aggregated as `iter` |
| `meta_tools` | Aggregated as `meta` |
| `mem_tools` | Aggregated as `mem` |
| `typing_tools` | Aggregated as `typing` |
| `time_tools` | Aggregated as `time` |
| `strs_tools` | Aggregated as `string` |
| `error_tools` | Aggregated as `error` |
| `derive_tools` | Aggregated as `derive` |
| `data_type` | Aggregated as `dt` |
| `diagnostics_tools` | Aggregated as `diagnostics` |

## References

- [wTools GitHub Repository](https://github.com/Wandalen/wTools)
- [Individual crate documentation on docs.rs](https://docs.rs/wtools)
