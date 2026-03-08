# Specification: data_type

## Overview

**data_type** is a facade crate aggregating common data types and utilities from multiple workspace crates (interval_adapter, collection_tools) and external dependencies (either). It provides a unified namespace for essential data structures, serving as a convenience layer for workspace consumers to access intervals, collections, and sum types without managing individual dependencies.

**Version:** 0.20.0
**Status:** Production
**Category:** Facade (Data Types Aggregation)
**Dependents:** Unknown (likely various workspace utilities)

### Scope

#### Responsibility

Provide a unified, convenient namespace for common primal data types by aggregating and re-exporting functionality from workspace crates (interval_adapter, collection_tools) and external dependencies (either), eliminating the need for consumers to manage multiple data-type-related dependencies.

#### In-Scope

1. **Interval Types (via interval_adapter)**
   - Feature: `dt_interval` (default)
   - Re-exports: IterableInterval, NonIterableInterval, Interval, Bound
   - Provides unified interface for Range types

2. **Collection Types (via collection_tools)**
   - Feature: `dt_collection` (default)
   - Re-exports: hmap!, hset!, bmap!, bset! macros
   - Provides variadic constructor macros for collections

3. **Sum Types (via either)**
   - Feature: `dt_either` (default)
   - Re-exports: Either<L, R>
   - Provides discriminated union type

4. **Dependency Namespace**
   - `dependency` module exposing underlying crates
   - Allows access to full APIs of dependencies
   - Organized re-exports

5. **Namespace Organization**
   - `dt` sub-module for data type namespace
   - Traditional namespaces: own, orphan, exposed, prelude
   - Layered re-exports for flexibility

6. **Feature Flags**
   - Granular control over included data types
   - Optional no_std support
   - Modular dependencies

7. **no_std Compatibility**
   - `#![no_std]` support via feature flag
   - Optional `use_alloc` for allocation in no_std
   - Portable to embedded environments

8. **Reserved Features**
   - `dt_make` - Reserved for variadic constructors (not implemented)
   - `dt_vectorized_from` - Reserved for vectorized conversions (not implemented)
   - Future expansion capability

#### Out-of-Scope

1. **NOT Original Implementation**
   - Does not provide original data type implementations
   - Only aggregates existing crates
   - **Rationale:** Facade pattern, not library

2. **NOT Type Constructors**
   - `dt_make` and `dt_vectorized_from` features exist but not implemented
   - Reserved for future integration
   - **Rationale:** Functionality not yet ready

3. **NOT Additional Data Structures**
   - Does not add data structures beyond dependencies
   - No custom types in this crate
   - **Rationale:** Pure aggregation layer

4. **NOT Trait Implementations**
   - Does not implement additional traits for re-exported types
   - Traits come from source crates
   - **Rationale:** Avoid orphan rule violations

5. **NOT Algorithm Library**
   - Does not provide algorithms operating on data types
   - Only type definitions and constructors
   - **Rationale:** Focused scope

6. **NOT Collection Prelude**
   - Commented-out std::collections re-exports
   - Not part of current API
   - **Rationale:** Not finalized design

7. **NOT Async Data Types**
   - No async/await types
   - No futures or streams
   - **Rationale:** Not in scope

8. **NOT Specialized Collections**
   - No specialized data structures (graphs, trees, etc.)
   - Only standard collections via collection_tools
   - **Rationale:** Use external crates for specialized needs

#### Boundaries

- **data_type vs interval_adapter**: data_type re-exports; interval_adapter implements
- **data_type vs collection_tools**: data_type aggregates; collection_tools provides
- **data_type vs either**: data_type re-exports; either is external dependency

## Architecture

### Dependency Structure

```
data_type (facade/aggregator)
├── Internal Dependencies
│   ├── interval_adapter (workspace, intervals)
│   └── collection_tools (workspace, collections)
└── External Dependencies
    └── either (~1.6, Either type, optional)
```

### Module Organization

```
data_type
├── lib.rs (top-level aggregation)
├── dt.rs (data type namespace)
│   ├── Re-exports interval_adapter
│   ├── Re-exports collection_tools
│   └── Re-exports either::Either
├── dependency module (dependency exposure)
│   ├── interval_adapter (when dt_interval)
│   ├── collection_tools (when dt_collection)
│   └── either (when dt_either)
└── Standard namespaces: own, orphan, exposed, prelude
```

**Pattern:** Traditional namespace organization with nested `dt` namespace

### Feature Architecture

```
enabled (master switch)
│
default (enabled + dt_either + dt_interval + dt_collection)
│
dt_either (default, Either type from external crate)
├── Re-exports: Either<L, R>
│
dt_interval (default, intervals from interval_adapter)
├── Re-exports: Interval, Bound, IterableInterval, NonIterableInterval
│
dt_collection (default, collections from collection_tools)
├── Re-exports: hmap!, hset!, bmap!, bset! macros
│
full (all features: enabled + all dt_* features)
│
no_std (embedded support)
└── use_alloc (allocation in no_std)
│
dt_make (reserved, not implemented)
dt_vectorized_from (reserved, not implemented)
```

**Default Features:** `enabled`, `dt_either`, `dt_interval`, `dt_collection`

### Re-export Flow

```
External Crates
    ↓
data_type::dependency module
    ↓
data_type::dt::exposed
    ↓
data_type::exposed
    ↓
data_type (top-level, via own)
```

**Example:**
```rust
interval_adapter::Interval
  ↓
data_type::dependency::interval_adapter::Interval
  ↓
data_type::dt::exposed::Interval
  ↓
data_type::exposed::Interval
  ↓
data_type::Interval (via use own::*)
```

## Public API

### Re-exported Types and Macros

```rust
// When dt_either feature enabled (default)
pub use either::Either;

// When dt_interval feature enabled (default)
pub use interval_adapter::{
  Interval,
  Bound,
  IterableInterval,
  NonIterableInterval,
  IntoInterval,
  BoundExt,
};

// When dt_collection feature enabled (default)
pub use collection_tools::{
  hmap,    // HashMap constructor macro
  hset,    // HashSet constructor macro
  bmap,    // BTreeMap constructor macro
  bset,    // BTreeSet constructor macro
};
```

### Dependency Module

```rust
pub mod dependency {
  #[cfg(feature = "dt_either")]
  pub use ::either;

  #[cfg(feature = "dt_interval")]
  pub use ::interval_adapter;

  #[cfg(feature = "dt_collection")]
  pub use ::collection_tools;
}
```

### Namespace Hierarchy

```rust
pub mod dt {
  pub mod own { /* ... */ }
  pub mod orphan { /* ... */ }
  pub mod exposed { /* Re-exports from dependencies */ }
  pub mod prelude { /* Essential items */ }
}

pub mod own { /* Includes dt::orphan */ }
pub mod orphan { /* Includes exposed */ }
pub mod exposed { /* Includes prelude + dt::exposed */ }
pub mod prelude { /* Essential items from all dependencies */ }
```

## Usage Patterns

### Pattern 1: Using Either Type

```rust
use data_type::*;

fn divide(a: i32, b: i32) -> Either<String, i32> {
  if b == 0 {
    Either::Left("Division by zero".to_string())
  } else {
    Either::Right(a / b)
  }
}

match divide(10, 2) {
  Either::Left(err) => println!("Error: {}", err),
  Either::Right(result) => println!("Result: {}", result),
}
```

### Pattern 2: Using Interval Types

```rust
use data_type::*;

fn process_range(interval: impl IterableInterval) {
  for i in interval {
    println!("{}", i);
  }
}

// All these work:
process_range(0..10);              // Range
process_range(0..=9);              // RangeInclusive
process_range((0, 9).into_interval()); // Tuple
```

### Pattern 3: Using Collection Macros

```rust
use data_type::*;

// HashMap constructor
let map = hmap! { "key1" => 1, "key2" => 2 };

// HashSet constructor
let set = hset! { 1, 2, 3, 4, 5 };

// BTreeMap constructor
let sorted_map = bmap! { 3 => "three", 1 => "one", 2 => "two" };

// BTreeSet constructor
let sorted_set = bset! { 5, 2, 8, 1, 9 };
```

### Pattern 4: Combined Usage

```rust
use data_type::*;

fn process_data(data: Either<Vec<i32>, (i32, i32)>) -> Vec<i32> {
  match data {
    Either::Left(vec) => vec,
    Either::Right((start, end)) => {
      (start..=end).into_iter().collect()
    }
  }
}

let result1 = process_data(Either::Left(vec![1, 2, 3]));
let result2 = process_data(Either::Right((1, 5)));
```

### Pattern 5: Accessing Full Dependency APIs

```rust
use data_type::dependency::*;

// Access full interval_adapter API
use interval_adapter::EndPointTrait;

// Access full collection_tools API
use collection_tools::HashMap;

// Access full either API
use either::Either;
```

### Pattern 6: Prelude Import

```rust
use data_type::prelude::*;

// All essential items from all dependencies available
let interval = (0, 10).into_interval();
let map = hmap! { 1 => "one" };
let either: Either<i32, String> = Either::Left(42);
```

### Pattern 7: Selective Features

```toml
# In Cargo.toml, use only specific features
[dependencies]
data_type = { version = "*", default-features = false, features = ["enabled", "dt_interval"] }
```

```rust
// Only interval types available
use data_type::*;

let interval = (0, 10).into_interval();
// hmap! macro not available (dt_collection not enabled)
// Either type not available (dt_either not enabled)
```

## Dependencies and Consumers

### Direct Dependencies

**Internal (workspace):**
- `interval_adapter` - Interval/range abstractions (optional: `dt_interval`)
- `collection_tools` - Collection constructor macros (optional: `dt_collection`)

**External:**
- `either` (~1.6) - Either type for sum types (optional: `dt_either`)

**Dev:**
- `test_tools` (workspace) - Testing utilities

### Consumers (Unknown)

**Likely used by:**
- Workspace tools needing common data types
- Applications wanting unified data type imports
- Generic algorithms working with various data structures

**Usage Pattern:** Workspace consumers use data_type for convenient access to common data structures without managing multiple dependencies.

## Design Rationale

### Why Facade Pattern?

data_type aggregates multiple crates instead of implementing:

**Benefits:**
1. **Single Import**: One crate provides multiple data types
2. **Version Management**: Consumers manage one version, not three
3. **Consistency**: Guarantees compatible versions of dependencies
4. **Discoverability**: Users find all data types in one place

**Tradeoff:** Extra dependency layer for convenience

### Why Feature Flags for Each Type?

Separate features (`dt_either`, `dt_interval`, `dt_collection`):

**Benefits:**
1. **Modularity**: Use only needed data types
2. **Compile Time**: Reduce dependencies if not needed
3. **no_std**: Can exclude std-dependent features
4. **Flexibility**: Fine-grained control

**Tradeoff:** More complex feature matrix

### Why dt Namespace?

Nested `dt` module for data types:

**Rationale:**
1. **Organization**: Separates data types from other utilities
2. **Clarity**: `data_type::dt::Interval` vs `data_type::Interval`
3. **Future Expansion**: Can add non-dt modules later

**Tradeoff:** Slightly more verbose imports

### Why External either Crate?

Uses external `either` crate instead of own implementation:

**Rationale:**
1. **Ecosystem Standard**: either crate is widely used
2. **No Reinvention**: Don't duplicate existing functionality
3. **Compatibility**: Works with other crates expecting either

**Benefit:** Ecosystem integration

### Why Reserved Features?

`dt_make` and `dt_vectorized_from` exist but do nothing:

**Rationale:**
1. **Future Proofing**: Reserve feature names for planned functionality
2. **No Breaking Changes**: Can implement later without name conflicts
3. **Documentation**: Signals intended direction

**Current State:** Not implemented, readme mentions but not functional

### Why Not Implement Collections Prelude?

Commented-out std::collections re-exports:

**Rationale:**
1. **Design Not Finalized**: Still deciding on API
2. **Namespace Pollution**: std collections would conflict
3. **Questionable Value**: std::collections already convenient

**Status:** Reserved for future consideration

## Testing Strategy

### Test Coverage

**test_tools Available:**
- Can use test_tools for integration testing
- Tests verify re-exports work correctly

### Test Focus

1. **Re-export Verification**: Ensure all items re-exported correctly
2. **Feature Combinations**: Test various feature flag combinations
3. **Integration**: Verify dependencies work together
4. **Namespace**: Verify all namespaces expose correct items

### Test Quality Standards

1. **Smoke Tests**: Basic functionality of each re-exported type
2. **Feature Tests**: Each feature flag tested independently
3. **Integration Tests**: Combined usage of multiple dependencies
4. **Doc Tests**: Examples in readme.md verified

## Future Considerations

### Potential Enhancements

1. **Implement dt_make**: Variadic constructor traits (From_0, From1, etc.)
2. **Implement dt_vectorized_from**: Vectorized type conversions
3. **Add std Prelude**: Re-export std::collections with aliases
4. **More Data Types**: Add more workspace data type crates
5. **Derive Macros**: Convenience derives for custom types

### Breaking Changes to Consider

1. **Remove dt Namespace**: Flatten to top-level (simpler imports)
2. **Rename Features**: More descriptive names
3. **Split Crate**: Separate into smaller facade crates

### Known Limitations

1. **No Custom Types**: Only re-exports, no original implementations
2. **Feature Creep Risk**: Easy to add too many dependencies
3. **Documentation Duplication**: Docs in multiple places
4. **Reserved Features Confusing**: dt_make/dt_vectorized_from exist but don't work

## Adoption Guidelines

### When to Use data_type

**Good Candidates:**
- Applications needing multiple data type utilities
- Workspace internal tools
- Projects wanting unified data type imports
- Codebases using intervals + collections + Either together

**Poor Candidates:**
- Libraries (prefer explicit dependencies)
- Projects needing only one data type (use direct dependency)
- no_std projects not needing all features (use specific crates)
- Performance-critical code (facade adds indirection)

### Migration from Direct Dependencies

```rust
// Before: Managing multiple dependencies
use interval_adapter::{Interval, IterableInterval};
use collection_tools::{hmap, hset};
use either::Either;

// After: Single import from data_type
use data_type::*;
```

### Best Practices

1. **Use Specific Features**: Disable unused features for faster compilation
2. **Prefer Prelude**: Use `data_type::prelude::*` for essential items
3. **Document Dependencies**: Note that data_type is facade, not implementation
4. **Avoid in Libraries**: Libraries should depend on specific crates
5. **Check Updates**: Monitor all underlying dependencies for updates

## Related Crates

- **interval_adapter**: Interval/range abstractions (dependency)
- **collection_tools**: Collection macros (dependency)
- **either**: Either type (dependency)
- **std::collections**: Standard library collections

## References

- [API Documentation](https://docs.rs/data_type)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/data_type)
- [readme.md](./readme.md)
- [interval_adapter](https://docs.rs/interval_adapter)
- [collection_tools](https://docs.rs/collection_tools)
- [either](https://docs.rs/either)
