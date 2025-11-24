# Specification: iter_tools

## Overview

**iter_tools** is a unified iterator utilities crate that combines selective re-exports from the `itertools` ecosystem with wTools-specific iterator abstractions. It provides clonable boxed iterators, iterator extension traits, and standard iterator functions with consistent API design across the wTools workspace.

**Version:** 0.42.0
**Status:** Experimental
**Category:** Iterator Utilities
**Dependents:** 10 crates (macro_tools, former, component_model, reflect_tools, implements, strs_tools, unilang, error_tools, mod_interface, meta_tools)

### Scope

#### Responsibility

Act as the canonical iterator utilities abstraction for the wTools ecosystem, providing:
1. Selective re-exports from `itertools` with consistent versioning
2. Clonable boxed iterator trait objects (`IterTrait`, `BoxedIter`)
3. Iterator extensions for common wTools patterns (`IterExt`)
4. no_std compatible iterator operations

#### In-Scope

1. **Itertools Re-exports**
   - Standard iterator combinators: `chain`, `zip`, `enumerate`, `fold`, `concat`, etc.
   - Advanced combinators: `interleave`, `intersperse`, `kmerge`, `merge_join_by`, etc.
   - Comparison functions: `min`, `max`, `equal`, `diff_with`, `sorted`
   - Specialized iterators: `multipeek`, `peek_nth`, `put_back`, `rciter`, `repeat_n`
   - Core traits and types: `Itertools`, `Either`, `EitherOrBoth`, `Position`, `FoldWhile`, etc.
   - Result processing: `process_results` for error handling in iterator chains

2. **Clonable Boxed Iterators**
   - `_IterTrait`: Base trait for clonable iterator trait objects
   - `IterTrait`: Public trait combining `_IterTrait` with `Clone`
   - `BoxedIter<'a, T>`: Type alias for `Box<dyn _IterTrait<'a, T> + 'a>`
   - Clone implementations for `Box<dyn _IterTrait>` variants:
     - `Box<dyn _IterTrait<T>>`
     - `Box<dyn _IterTrait<T> + Send>`
     - `Box<dyn _IterTrait<T> + Sync>`
     - `Box<dyn _IterTrait<T> + Send + Sync>`

3. **Iterator Extensions**
   - `IterExt` trait with wTools-specific iterator methods
   - `map_result`: Map iterator with fallible function, returning `Result<Vec<T>, E>`
   - Result-oriented iterator processing patterns

4. **No-std Support**
   - Feature flag `no_std` for embedded environments
   - `use_alloc` feature for allocation-dependent functionality
   - Conditional compilation for std-only features

5. **Feature Architecture**
   - `enabled`: Master feature switch
   - `iter_trait`: Enable clonable boxed iterator traits
   - `iter_ext`: Enable iterator extension methods
   - `full`: Enable all functionality

6. **Consistent Versioning**
   - Centralized `itertools` dependency version
   - Workspace-level version management
   - Unified API surface across wTools crates

#### Out-of-Scope

1. **NOT Custom Iterator Implementations**
   - Does not provide custom iterator types beyond trait objects
   - Does not implement specialized iterators (e.g., file iterators, network iterators)
   - **Rationale:** Focus on composition and re-export, not novel iterator types

2. **NOT Iterator Adapters for Specific Domains**
   - Does not provide parsing-specific iterators
   - Does not provide database-specific iterators
   - Does not provide async iterators
   - **Rationale:** Domain-specific iterators belong in domain-specific crates

3. **NOT Parallel Iteration**
   - Does not provide parallel iterator traits
   - Does not integrate with `rayon` or similar parallelism libraries
   - **Rationale:** Parallelism is a separate concern handled by dedicated crates

4. **NOT Stream Abstractions**
   - Does not provide async stream traits
   - Does not provide futures-based iteration
   - **Rationale:** Async streams are handled by async runtime crates

5. **NOT Collection Types**
   - Does not provide custom collection types
   - Does not implement collection-specific iterators
   - **Rationale:** Collections are provided by `collection_tools` crate

6. **NOT Iterator Performance Optimization**
   - Does not provide SIMD-optimized iterators
   - Does not provide compile-time loop unrolling
   - Does not implement zero-cost iterator abstractions beyond Rust's standard library
   - **Rationale:** Performance optimization is the compiler's responsibility

7. **NOT Iterator Validation**
   - Does not validate iterator bounds or constraints
   - Does not provide iterator debugging utilities
   - **Rationale:** Validation and debugging are testing concerns

8. **NOT Complete Itertools Re-export**
   - Selectively re-exports from `itertools`, not all functions
   - `zip` is not re-exported (uses `core::iter::zip` instead)
   - **Rationale:** Avoid namespace pollution, prefer std library where available

#### Boundaries

- **iter_tools vs itertools**: iter_tools re-exports and extends itertools, not replaces it
- **iter_tools vs std::iter**: iter_tools prefers std library functions when available (e.g., `core::iter::zip`)
- **iter_tools vs collection_tools**: iter_tools provides iteration, collection_tools provides collection types
- **iter_tools vs macro_tools**: macro_tools consumes iter_tools for macro implementation

## Architecture

### Dependency Structure

```
iter_tools
├── External Dependencies
│   └── itertools (workspace, optional, features: ["use_std"])
└── Internal Dependencies
    └── clone_dyn_types (workspace, optional)
```

### Module Organization

iter_tools uses the traditional module organization pattern:

```
iter_tools
├── lib.rs (module re-exports)
└── iter.rs (core functionality)
    ├── private (trait definitions, implementations)
    ├── own (all items)
    ├── orphan (exposed + itertools re-exports)
    ├── exposed (public traits and types)
    └── prelude (essential items)
```

### Feature Architecture

```
enabled (master switch)
├── iter_trait (clonable boxed iterators)
└── iter_ext (iterator extensions)

no_std (embedded support)
└── use_alloc (allocation for no_std)

full (all features)
```

**Default Features:** `enabled`, `iter_trait`, `iter_ext`

### Core Abstractions

1. **Clonable Boxed Iterator System**

The clonable boxed iterator system allows trait objects (`Box<dyn _IterTrait>`) to be cloned, which is not possible with standard Rust trait objects.

```rust
pub trait _IterTrait<'a, T>
where
  T: 'a,
  Self: Iterator<Item = T> + ExactSizeIterator<Item = T> + DoubleEndedIterator,
  Self: CloneDyn,
{}

pub trait IterTrait<'a, T>
where
  T: 'a,
  Self: _IterTrait<'a, T> + Clone,
{}

pub type BoxedIter<'a, T> = Box<dyn _IterTrait<'a, T> + 'a>;
```

**Design Rationale:**
- `_IterTrait`: Internal trait with `CloneDyn` requirement
- `IterTrait`: Public trait combining `_IterTrait` with `Clone`
- `BoxedIter`: Ergonomic type alias for common case
- Separate traits allow blanket implementations without conflicts

2. **Iterator Extension System**

```rust
pub trait IterExt
where
  Self: core::iter::Iterator,
{
  fn map_result<F, RE, El>(self, f: F) -> core::result::Result<Vec<El>, RE>
  where
    Self: Sized + Clone,
    F: FnMut(<Self as core::iter::Iterator>::Item) -> core::result::Result<El, RE>,
    RE: core::fmt::Debug;
}
```

**Design Rationale:**
- Blanket implementation for all iterators
- Result-oriented processing pattern common in wTools
- Uses `itertools::process_results` internally for efficiency

## Public API

### Core Traits

```rust
/// Base trait for clonable iterator trait objects
#[cfg(feature = "iter_trait")]
pub trait _IterTrait<'a, T>
where
  T: 'a,
  Self: Iterator<Item = T> + ExactSizeIterator<Item = T> + DoubleEndedIterator,
  Self: CloneDyn,
{}

/// Public trait for clonable iterators
#[cfg(feature = "iter_trait")]
pub trait IterTrait<'a, T>
where
  T: 'a,
  Self: _IterTrait<'a, T> + Clone,
{}

/// Iterator extension trait
#[cfg(feature = "iter_ext")]
pub trait IterExt
where
  Self: core::iter::Iterator,
{
  fn map_result<F, RE, El>(self, f: F) -> core::result::Result<Vec<El>, RE>
  where
    Self: Sized + Clone,
    F: FnMut(<Self as core::iter::Iterator>::Item) -> core::result::Result<El, RE>,
    RE: core::fmt::Debug;
}
```

### Type Aliases

```rust
/// Type alias for boxed iterator trait objects
#[cfg(feature = "iter_trait")]
pub type BoxedIter<'a, T> = Box<dyn _IterTrait<'a, T> + 'a>;
```

### Re-exported Functions

From `itertools`:

**Combinators:**
- `chain` - Chain two iterators
- `enumerate` - Add index to iterator
- `fold` - Fold iterator into single value
- `concat` - Concatenate iterator of iterables
- `interleave` - Interleave two iterators
- `intersperse`, `intersperse_with` - Insert separator between elements
- `kmerge`, `kmerge_by` - Merge k sorted iterators
- `merge`, `merge_join_by` - Merge iterators

**Utilities:**
- `min`, `max` - Find minimum/maximum element
- `sorted` - Collect into sorted vector
- `equal` - Compare two iterators for equality
- `diff_with` - Find differences between iterators
- `join` - Join iterator into string
- `all`, `any` - Logical operations
- `partition` - Split into two collections

**Specialized:**
- `multipeek` - Peek multiple elements ahead
- `peek_nth` - Peek n-th element
- `put_back`, `put_back_n` - Put elements back into iterator
- `rciter` - Reference-counted iterator
- `repeat_n` - Repeat element n times
- `unfold` - Generate iterator from function

**Result Processing:**
- `process_results` - Process iterator of results

**Traits:**
- `Itertools` - Main extension trait
- `PeekingNext` - Trait for peeking iterators

**Types:**
- `Either`, `EitherOrBoth` - Sum types for iterators
- `Position` - Iterator position enum
- `FoldWhile` - Control flow for folding
- `MinMaxResult` - Result of minmax operation
- `Diff` - Difference between iterators

### Re-exported from std

```rust
#[cfg(not(feature = "no_std"))]
pub use core::iter::zip;
```

## Usage Patterns

### Pattern 1: Basic Iteration with Re-exports

```rust
use iter_tools::*;

// Standard functions
let vec = vec![5, 1, -2];
let min = min(&vec);
assert_eq!(*min.unwrap(), -2);

// Zipping iterators
let vec = vec![5, 1, -2];
let added = vec!["a", "b", "c"];
let mut result = vec![];
let zipped = zip(&vec, &added);
for (left, right) in zipped
{
  result.push((*left, *right));
}
assert_eq!(result, vec![(5, "a"), (1, "b"), (-2, "c")]);
```

### Pattern 2: Clonable Boxed Iterators

```rust
use iter_tools::{BoxedIter, IterTrait};

fn get_iterator<'a>(data: &'a [i32]) -> BoxedIter<'a, &'a i32>
{
  Box::new(data.iter())
}

let data = vec![1, 2, 3];
let iter1 = get_iterator(&data);
let iter2 = iter1.clone(); // Clone the boxed iterator

assert_eq!(iter1.collect::<Vec<_>>(), iter2.collect::<Vec<_>>());
```

### Pattern 3: Result-Oriented Iterator Processing

```rust
use iter_tools::IterExt;

let items = vec!["1", "2", "3", "invalid"];
let result = items.iter().map_result(|s| s.parse::<i32>());

assert!(result.is_err()); // Fails on "invalid"

let items = vec!["1", "2", "3"];
let result = items.iter().map_result(|s| s.parse::<i32>());
assert_eq!(result.unwrap(), vec![1, 2, 3]);
```

### Pattern 4: Advanced Combinators

```rust
use iter_tools::*;

// Interleave two iterators
let a = vec![1, 2, 3];
let b = vec![10, 20, 30];
let interleaved: Vec<_> = interleave(&a, &b).cloned().collect();
assert_eq!(interleaved, vec![1, 10, 2, 20, 3, 30]);

// Intersperse separator
let data = vec![1, 2, 3];
let with_sep: Vec<_> = intersperse(data.iter(), &0).cloned().collect();
assert_eq!(with_sep, vec![1, 0, 2, 0, 3]);
```

## Dependencies and Consumers

### Direct Dependencies

**External:**
- `itertools` (workspace) - Core iterator utilities library

**Internal:**
- `clone_dyn_types` (workspace) - Clone trait objects support

### Consumers (10 crates)

1. **macro_tools** - Iterator utilities for procedural macros
2. **former** - Iterator processing in builder patterns
3. **component_model** - Component iteration and processing
4. **reflect_tools** - Reflection over collections
5. **implements** - Implementation detection over types
6. **strs_tools** - String iterator processing
7. **unilang** - Language construct iteration
8. **error_tools** - Error collection and processing
9. **mod_interface** - Module interface iteration
10. **meta_tools** - Meta-programming iterator utilities

## Design Rationale

### Why Wrapper Around itertools?

1. **Unified Versioning**: All wTools crates use the same `itertools` version
2. **Selective API Surface**: Only re-export commonly used functions, reducing namespace pollution
3. **wTools Extensions**: Add wTools-specific patterns (clonable boxed iterators, result processing)
4. **no_std Compatibility**: Centralized no_std feature management
5. **Consistency**: Single import point for all iterator utilities

### Why Clonable Boxed Iterators?

Standard Rust trait objects (`Box<dyn Iterator>`) cannot be cloned because `Clone` is not object-safe. The `IterTrait` system solves this by:

1. **CloneDyn Integration**: Uses `clone_dyn_types` crate for object-safe cloning
2. **Type Safety**: Separate `_IterTrait` and `IterTrait` prevent conflicting implementations
3. **Ergonomics**: `BoxedIter` type alias simplifies common case
4. **Send/Sync Variants**: Four clone implementations cover all marker trait combinations

**Use Case:** Storing iterators in data structures that require `Clone`, such as configuration objects or cached computation results.

### Why IterExt Trait?

The `map_result` method encapsulates a common pattern in wTools:

1. **Error Propagation**: Map iterator with fallible function, stop on first error
2. **Convenience**: Single method call instead of manual error handling
3. **Consistency**: Same pattern across all wTools crates
4. **Efficiency**: Uses `itertools::process_results` for optimal performance

### Why Not Re-export All of itertools?

1. **Namespace Management**: Avoid overwhelming users with 100+ functions
2. **Std Library Preference**: Use `core::iter::zip` instead of `itertools::zip`
3. **Intentional API**: Only export functions used in wTools ecosystem
4. **Maintenance**: Smaller API surface reduces breaking change risk

### Why Traditional Module Pattern (Not mod_interface)?

1. **Simplicity**: iter_tools is a small crate with clear structure
2. **No Dependencies**: Avoids depending on macro_tools (which depends on iter_tools)
3. **Stability**: Traditional pattern is stable and well-understood

## Testing Strategy

### Test Coverage

- **Example Programs**: 1 comprehensive example (`iter_tools_trivial.rs`)
- **Doc Tests**: Embedded in lib.rs and iter.rs
- **Integration Tests**: Uses `test_tools` for integration testing

### Test Focus

1. **Re-export Verification**: Ensure all re-exported functions work correctly
2. **Clonable Iterator Tests**: Verify `BoxedIter` can be cloned and used
3. **IterExt Tests**: Verify `map_result` handles success and error cases
4. **no_std Tests**: Verify functionality in no_std environments

## Future Considerations

### Potential Enhancements

1. **Additional IterExt Methods**: Add more wTools-specific iterator patterns
2. **Async Iterator Support**: Conditional support for `Stream` trait when async is needed
3. **More Send/Sync Variants**: Additional marker trait combinations for boxed iterators
4. **Documentation**: Add specification sections to module docs

### Breaking Changes to Consider

1. **Module Organization**: Potential migration to mod_interface pattern
2. **Feature Structure**: Consolidation of iter_trait and iter_ext into single feature
3. **Re-export Expansion**: Add more itertools functions based on usage patterns

## Related Crates

- **itertools**: External dependency providing core functionality
- **clone_dyn_types**: Internal dependency for clonable trait objects
- **collection_tools**: Provides collection types that iter_tools iterates over
- **macro_tools**: Consumes iter_tools for procedural macro iteration

## References

- [API Documentation](https://docs.rs/iter_tools)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/iter_tools)
- [itertools Documentation](https://docs.rs/itertools)
- [readme.md](./readme.md)
