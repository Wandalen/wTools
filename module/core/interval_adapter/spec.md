# Specification: interval_adapter

## Overview

**interval_adapter** is a zero-dependency no_std crate providing a unified interface for Rust's diverse range types. It abstracts over Range, RangeInclusive, and all standard library range variants with traits for both iterable and non-iterable intervals, solving the problem of writing generic functions that accept any interval type while preserving type safety and zero-cost abstractions.

**Version:** 0.38.0
**Status:** Production
**Category:** Type Utilities (Interval/Range Abstraction)
**Dependents:** 3 workspace crates (likely range-handling utilities)

### Scope

#### Responsibility

Provide a unified trait-based interface for all Rust range types (Range, RangeInclusive, RangeFull, etc.), enabling generic code to accept and manipulate intervals without caring about the specific range implementation while supporting both bounded and unbounded intervals.

#### In-Scope

1. **Trait Hierarchy**
   - `NonIterableInterval<T>` - Interface for all intervals (including unbounded)
   - `IterableInterval<T>` - Interface for bounded intervals (implements IntoIterator)
   - Trait inheritance: `IterableInterval` extends `NonIterableInterval`

2. **Supported Range Types (All from std::ops)**
   - `Range<T>` (0..4) - Half-open interval
   - `RangeInclusive<T>` (0..=3) - Closed interval
   - `RangeTo<T>` (..4) - Unbounded left, excluded right
   - `RangeToInclusive<T>` (..=3) - Unbounded left, included right
   - `RangeFrom<T>` (0..) - Included left, unbounded right
   - `RangeFull` (..) - Fully unbounded (-∞..+∞)

3. **Additional Interval Representations**
   - Tuple `(T, T)` - Both endpoints included
   - Bound tuple `(Bound<T>, Bound<T>)` - Custom bound specification
   - Array `[T; 2]` - Both endpoints included
   - Bound array `[Bound<T>; 2]` - Custom bound specification

4. **Canonical Interval Type**
   - `Interval<T>` - Canonical representation storing `(Bound<T>, Bound<T>)`
   - Convertible from all supported range types
   - Implements `IntoIterator` for iteration
   - Implements `NonIterableInterval` trait

5. **Bound Extension Trait**
   - `BoundExt<T>` - Extends `core::ops::Bound` with conversion methods
   - `into_left_closed()` - Convert to left endpoint of closed interval
   - `into_right_closed()` - Convert to right endpoint of closed interval
   - Handles Included, Excluded, Unbounded variants

6. **Endpoint Trait**
   - `EndPointTrait<T>` - Constraint bundle for interval endpoints
   - Requires: PartialOrd, Sub, Add, Clone, Copy, Sized
   - Blanket implementation for all qualifying types

7. **Interval Operations**
   - `left()` / `right()` - Get bounds as-is
   - `closed_left()` / `closed_right()` - Get endpoints as closed interval
   - `closed()` - Get both endpoints as closed tuple
   - `closed_len()` - Calculate length of interval
   - `canonical()` - Convert to canonical `Interval<T>` type
   - `into_interval()` - Trait method for conversion

8. **Iterator Support**
   - `IntervalIterator<T>` - Iterator over interval values
   - Automatically generated for all `IterableInterval` types
   - Closed-interval iteration (inclusive on both ends)

9. **Generic Conversions**
   - `IntoInterval<T>` trait - Convert any interval type to canonical
   - `From` implementations for all std range types
   - Blanket implementation for all convertible types

10. **no_std Compatibility**
    - `#![no_std]` with optional `use_alloc` feature
    - Zero production dependencies
    - Uses only core library types

11. **Traditional Module Organization**
    - Standard namespaces: own, orphan, exposed, prelude
    - Not using mod_interface! (utility crate)
    - Re-exports `core::ops::Bound` and `core::ops::RangeBounds`

#### Out-of-Scope

1. **NOT Floating-Point Intervals**
   - Does not support f32/f64 intervals
   - Iteration requires integer-like types
   - **Rationale:** Iteration over floats is ambiguous and error-prone

2. **NOT Continuous Mathematics**
   - Does not provide set operations (union, intersection)
   - Does not check containment
   - **Rationale:** Simple abstraction, not computational geometry

3. **NOT Step Customization**
   - Iterator always steps by 1
   - No custom step sizes or non-uniform steps
   - **Rationale:** Simplicity and performance

4. **NOT Multi-Dimensional Intervals**
   - Only one-dimensional intervals
   - No rectangle/box types
   - **Rationale:** Focused scope

5. **NOT Infinite Iteration**
   - Unbounded intervals are `NonIterableInterval`, not `IterableInterval`
   - Cannot iterate over `RangeFrom`, `RangeFull`, etc.
   - **Rationale:** Infinite iteration would never terminate

6. **NOT Reverse Intervals**
   - Assumes left ≤ right
   - No reverse iteration support
   - **Rationale:** Simplicity (use `.rev()` on iterator if needed)

7. **NOT Validation**
   - Does not validate left ≤ right
   - Does not prevent invalid intervals
   - **Rationale:** Trust users, avoid runtime overhead

8. **NOT Arithmetic on Intervals**
   - No interval addition, multiplication, etc.
   - **Rationale:** Beyond scope of abstraction

#### Boundaries

- **interval_adapter vs std::ops ranges**: interval_adapter provides unified interface; std ranges are underlying types
- **interval_adapter vs RangeBounds**: interval_adapter is higher-level with iteration support; RangeBounds is lower-level trait
- **interval_adapter vs itertools**: interval_adapter provides interval types; itertools provides iterator combinators

## Architecture

### Dependency Structure

```
interval_adapter (type utilities, zero dependencies)
├── Internal Dependencies
│   └── (none - foundational utility)
└── Dev Dependencies
    └── test_tools (workspace, testing)
```

**Note:** Zero production dependencies for maximum compatibility.

### Module Organization

```
interval_adapter
├── lib.rs (traditional namespaces)
├── private module (inline implementation)
│   ├── Traits
│   │   ├── BoundExt - Bound extension methods
│   │   ├── EndPointTrait - Endpoint constraints
│   │   ├── NonIterableInterval - All intervals
│   │   ├── IterableInterval - Bounded intervals
│   │   └── IntoInterval - Conversion trait
│   ├── Types
│   │   ├── Interval<T> - Canonical interval
│   │   └── IntervalIterator<T> - Interval iterator
│   └── Implementations
│       ├── NonIterableInterval for all range types
│       ├── From<Range> for Interval
│       └── IntoIterator for Interval
└── Standard namespaces: own, orphan, exposed, prelude
```

**Pattern:** Traditional namespace organization, not mod_interface! (utility crate convention)

### Feature Architecture

```
no_std (default, embedded support)
├── use_alloc (allocation support)
│
enabled (master switch, default)
└── full (all features: enabled + no_std)
```

**Default Features:** `enabled`, `no_std`

### Type Relationships

```
All std range types
    ↓ implements
NonIterableInterval<T>
    ↑ extends
IterableInterval<T>
    ↓ used by
Generic functions accepting intervals
    ↓ converts to
Interval<T> (canonical)
    ↓ implements
IntoIterator
    ↓ produces
IntervalIterator<T>
```

### Iteration Flow

```
0..=3 (RangeInclusive)
  ↓ implements NonIterableInterval
left() → Included(0)
right() → Included(3)
  ↓ converts to canonical
Interval { _left: Included(0), _right: Included(3) }
  ↓ into_iter()
IntervalIterator { current: 0, right: 3 }
  ↓ iteration
yields: 0, 1, 2, 3
```

### Bound Conversion

```
Bound::Included(5)
  ├─ into_left_closed() → 5
  └─ into_right_closed() → 5

Bound::Excluded(5)
  ├─ into_left_closed() → 6 (add 1)
  └─ into_right_closed() → 4 (subtract 1)

Bound::Unbounded
  ├─ into_left_closed() → 0 (or isize::MIN conceptually)
  └─ into_right_closed() → isize::MAX
```

## Public API

### Core Traits

```rust
/// Extend Bound with conversion methods
pub trait BoundExt<T>
where
  T: EndPointTrait<T>,
  isize: Into<T>,
{
  /// Convert bound to left endpoint of closed interval
  fn into_left_closed(&self) -> T;
  /// Convert bound to right endpoint of closed interval
  fn into_right_closed(&self) -> T;
}

/// Endpoint constraint bundle
pub trait EndPointTrait<T>
where
  Self: PartialOrd + Sub<Output = T> + Add<Output = T> + Clone + Copy + Sized,
{
}

/// Non-iterable interval interface
pub trait NonIterableInterval<T = isize>
where
  T: EndPointTrait<T>,
  isize: Into<T>,
{
  /// Left endpoint as-is
  fn left(&self) -> Bound<T>;
  /// Right endpoint as-is
  fn right(&self) -> Bound<T>;
  /// Both bounds as tuple
  fn bounds(&self) -> (Bound<T>, Bound<T>);
  /// Left endpoint as closed interval value
  fn closed_left(&self) -> T;
  /// Right endpoint as closed interval value
  fn closed_right(&self) -> T;
  /// Length of interval (closed)
  fn closed_len(&self) -> T;
  /// Both endpoints as closed tuple
  fn closed(&self) -> (T, T);
  /// Convert to canonical Interval type
  fn canonical(&self) -> Interval<T>;
}

/// Iterable interval interface
pub trait IterableInterval<T = isize>
where
  Self: IntoIterator<Item = T> + NonIterableInterval<T>,
  T: EndPointTrait<T>,
  isize: Into<T>,
{
}

/// Convert to canonical interval
pub trait IntoInterval<T>
where
  T: EndPointTrait<T>,
  isize: Into<T>,
{
  fn into_interval(self) -> Interval<T>;
}
```

### Canonical Interval Type

```rust
/// Canonical interval representation
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Interval<T = isize>
where
  T: EndPointTrait<T>,
  isize: Into<T>,
{
  _left: Bound<T>,
  _right: Bound<T>,
}

impl<T> Interval<T> {
  /// Constructor
  pub fn new(left: Bound<T>, right: Bound<T>) -> Self;
  /// Get iterator
  pub fn iter(&self) -> impl Iterator<Item = T>;
}

impl<T> IntoIterator for Interval<T> {
  type Item = T;
  type IntoIter = IntervalIterator<T>;
  fn into_iter(self) -> Self::IntoIter;
}
```

### Iterator

```rust
/// Iterator over interval values
pub struct IntervalIterator<T>
where
  T: EndPointTrait<T>,
  isize: Into<T>,
{
  current: T,
  right: T,
}

impl<T> Iterator for IntervalIterator<T> {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item>;
}
```

### Re-exports

```rust
pub use core::ops::Bound;
pub use core::ops::RangeBounds;
```

## Usage Patterns

### Pattern 1: Generic Function Accepting Any Interval

```rust
use interval_adapter::IterableInterval;

fn print_range(interval: impl IterableInterval) {
  for i in interval {
    println!("{i}");
  }
}

// All these work identically:
print_range(0..4);              // Range
print_range(0..=3);             // RangeInclusive
print_range((0, 3));            // Tuple
```

### Pattern 2: Custom Interval from Tuple

```rust
use interval_adapter::{IterableInterval, IntoInterval, Bound};

fn process(interval: impl IterableInterval) {
  for i in interval {
    println!("{i}");
  }
}

// Different ways to create same interval:
process(0..=3);
process((0, 3).into_interval());
process((Bound::Included(0), Bound::Included(3)).into_interval());
```

### Pattern 3: Non-Iterable Intervals

```rust
use interval_adapter::{NonIterableInterval, Bound};

fn describe(interval: impl NonIterableInterval) {
  println!("Interval from {:?} to {:?}", interval.left(), interval.right());
  println!("Closed form: {} to {}", interval.closed_left(), interval.closed_right());
}

// Bounded intervals:
describe((Bound::Included(0), Bound::Included(3)).into_interval());

// Unbounded intervals:
describe(0..);                  // RangeFrom: [0, ∞)
describe(..);                   // RangeFull: (-∞, ∞)
describe(..10);                 // RangeTo: (-∞, 10)
```

### Pattern 4: Converting Between Interval Types

```rust
use interval_adapter::{IntoInterval, Interval};

// Convert Range to canonical Interval
let range = 0..10;
let interval: Interval<i32> = range.into_interval();

// Access bounds
let (left, right) = interval.bounds();
println!("Bounds: {:?} to {:?}", left, right);

// Get as closed interval
let (start, end) = interval.closed();
println!("Closed: {} to {}", start, end);  // "Closed: 0 to 9"
```

### Pattern 5: Calculating Interval Properties

```rust
use interval_adapter::NonIterableInterval;

fn analyze(interval: impl NonIterableInterval<i32>) {
  println!("Left: {}", interval.closed_left());
  println!("Right: {}", interval.closed_right());
  println!("Length: {}", interval.closed_len());
}

analyze(0..10);     // Left: 0, Right: 9, Length: 10
analyze(0..=9);     // Left: 0, Right: 9, Length: 10
analyze((5, 15));   // Left: 5, Right: 15, Length: 11
```

### Pattern 6: Bound Conversion

```rust
use interval_adapter::{Bound, BoundExt};

let bound = Bound::Excluded(5);

// As left endpoint of closed interval
let left = bound.into_left_closed();  // 6 (excluded 5 means included 6)

// As right endpoint of closed interval
let right = bound.into_right_closed(); // 4 (excluded 5 means included 4)
```

### Pattern 7: Array and Tuple Intervals

```rust
use interval_adapter::IterableInterval;

fn iterate(interval: impl IterableInterval<i32>) {
  let values: Vec<i32> = interval.into_iter().collect();
  println!("{:?}", values);
}

iterate([0, 5]);                          // Array: [0, 1, 2, 3, 4, 5]
iterate((10, 12));                        // Tuple: [10, 11, 12]
iterate([Bound::Included(0), Bound::Excluded(3)]); // [0, 1]
```

### Pattern 8: Working with Different Numeric Types

```rust
use interval_adapter::{IterableInterval, NonIterableInterval};

fn sum_range<T>(interval: impl IterableInterval<T>) -> T
where
  T: EndPointTrait<T> + std::ops::Add<Output = T> + Default,
  isize: Into<T>,
{
  interval.into_iter().fold(T::default(), |acc, x| acc + x)
}

// Works with different integer types
let sum_i32: i32 = sum_range(1i32..=5);    // 15
let sum_i64: i64 = sum_range(1i64..10);    // 45
let sum_u32: u32 = sum_range(0u32..=10);   // 55
```

## Dependencies and Consumers

### Direct Dependencies

**Production:** (none - zero dependencies)

**Dev:**
- `test_tools` (workspace) - Testing utilities

### Consumers (3 workspace crates)

**Identified:** Likely used by:
- Range-handling utilities
- Iterator tools
- Generic algorithms working with ranges

**Usage Pattern:** Workspace utilities use interval_adapter to write generic code that accepts any Rust range type without caring about the specific implementation.

## Design Rationale

### Why Two Trait Hierarchy?

**Problem:** Some ranges are unbounded (RangeFull, RangeFrom) and cannot be iterated.

**Solution:** Separate traits for iterable and non-iterable intervals:

```rust
// All intervals
trait NonIterableInterval { ... }

// Only bounded intervals
trait IterableInterval: NonIterableInterval + IntoIterator { ... }
```

**Benefits:**
1. **Type Safety**: Compiler prevents iteration over infinite ranges
2. **Flexibility**: Can still query bounds of unbounded intervals
3. **Correctness**: Cannot accidentally create infinite loops

**Tradeoff:** Slightly more complex API for safety

### Why Canonical Interval Type?

The `Interval<T>` type provides:

1. **Uniformity**: All interval types convert to same representation
2. **Storage**: Can store any interval in a single type
3. **Iteration**: Single iterator implementation for all types

**Alternative:** Could use trait objects, but that requires allocation and dynamic dispatch.

**Tradeoff:** Conversion overhead for performance and simplicity

### Why Extend Bound?

`BoundExt` extends `core::ops::Bound` because:

1. **Closed Conversion**: Need to convert Excluded → Included consistently
2. **Orphan Rule**: Can't implement methods directly on foreign type (Bound)
3. **Context**: Left vs right endpoints need different conversion logic

**Example:**
```rust
Bound::Excluded(5).into_left_closed()  → 6  (next value)
Bound::Excluded(5).into_right_closed() → 4  (previous value)
```

### Why EndPointTrait?

`EndPointTrait` bundles common constraints:

```rust
T: PartialOrd + Sub<Output = T> + Add<Output = T> + Clone + Copy
```

**Benefits:**
1. **DRY**: Avoid repeating constraints everywhere
2. **Clarity**: Single bound instead of five
3. **Extensibility**: Easy to add more requirements

**Tradeoff:** Less flexible (all-or-nothing constraints)

### Why Support So Many Types?

interval_adapter implements `NonIterableInterval` for 10+ types:

1. **Standard Ranges**: Range, RangeInclusive, RangeFull, etc. (6 types)
2. **Tuples**: (T, T), (Bound<T>, Bound<T>)
3. **Arrays**: [T; 2], [Bound<T>; 2]

**Rationale:**
- **Convenience**: Users can use natural representations
- **Interop**: Works with existing std range types
- **Flexibility**: Tuples for quick construction, arrays for patterns

**Tradeoff:** More implementations to maintain for better UX

### Why Closed Interval Conversion?

Converting to closed intervals (`closed_left()`, `closed_right()`) because:

1. **Uniformity**: Half-open and closed ranges become equivalent
2. **Arithmetic**: Easy to calculate length (right - left + 1)
3. **Display**: Natural to show "from X to Y inclusive"

**Alternative:** Could keep open/closed distinction, but more complex

### Why Default to isize?

Traits default to `T = isize`:

```rust
trait NonIterableInterval<T = isize> { ... }
```

**Rationale:**
- **Common Case**: Most intervals use signed integers
- **Ergonomics**: Don't need to specify type parameter usually
- **Unbounded**: isize::MAX represents unbounded better than usize

**Tradeoff:** Less universal than generic, but more convenient

### Why no_std?

interval_adapter is `no_std` because:

1. **Embedded**: Works in embedded/kernel environments
2. **Core Abstraction**: Fundamental types shouldn't require std
3. **Zero Dependencies**: Already has no deps, no_std extends that

**Tradeoff:** No benefit for std users, but enables embedded use

## Testing Strategy

### Test Coverage

**test_tools Available:**
- Unlike some other crates, can use test_tools for testing
- No circular dependency issues

### Test Files

```
tests/
├── smoke_test.rs - Basic functionality
└── tests.rs - Comprehensive tests
```

### Test Focus

1. **Range Type Support**: Verify all std range types work
2. **Bound Conversion**: Test Excluded/Included/Unbounded conversion
3. **Iteration**: Verify iteration produces correct values
4. **Edge Cases**: Empty intervals, single-element intervals
5. **Type Safety**: Compile-time verification of trait bounds

### Test Quality Standards

1. **No Mocking**: Use real interval types
2. **Loud Failures**: Clear assertion messages
3. **Comprehensive**: Test all supported range types
4. **Edge Cases**: Boundary conditions

## Future Considerations

### Potential Enhancements

1. **Step Support**: Custom step sizes via Step trait when stabilized
2. **Reverse Iteration**: DoubleEndedIterator support
3. **Set Operations**: Intersection, union, contains
4. **Floating Point**: Discrete floating-point intervals
5. **Multi-Dimensional**: Rectangle/box types for 2D/3D intervals

### Breaking Changes to Consider

1. **Seal Traits**: Make NonIterableInterval sealed to prevent external impls
2. **Default to i32**: Change from isize to i32 as default
3. **Validation**: Add runtime checks for invalid intervals

### Known Limitations

1. **No Custom Steps**: Always iterates by +1
2. **No Reverse Check**: Doesn't detect or handle reversed intervals (left > right)
3. **No Validation**: Trusts intervals are valid
4. **Integer Only**: No floating-point interval support
5. **No Set Ops**: No contains(), overlaps(), etc.

## Adoption Guidelines

### When to Use interval_adapter

**Good Candidates:**
- Generic functions accepting any range type
- APIs that work with intervals
- Converting between range representations
- Calculating interval properties (length, bounds)
- Unified range handling across codebase

**Poor Candidates:**
- Simple iteration (use Range directly)
- Performance-critical tight loops (abstraction overhead)
- Floating-point ranges (not supported)
- Set operations (use external crate)

### Migration from Concrete Range Types

```rust
// Before: Concrete range type
fn process_range(range: std::ops::Range<i32>) {
  for i in range {
    println!("{i}");
  }
}

// After: Generic interval
use interval_adapter::IterableInterval;

fn process_range(range: impl IterableInterval<i32>) {
  for i in range {
    println!("{i}");
  }
}

// Now accepts Range, RangeInclusive, tuples, arrays, etc.!
```

### Best Practices

1. **Use IterableInterval**: For functions that iterate over ranges
2. **Use NonIterableInterval**: For functions that query bounds but don't iterate
3. **Prefer IntoInterval**: Use `.into_interval()` for explicit conversion
4. **Document Bounds**: Specify whether function expects closed/open intervals
5. **Validate Input**: Check that left ≤ right if required

## Related Crates

- **std::ops**: Standard library range types (Range, RangeInclusive, etc.)
- **itertools**: Iterator combinators (complements interval_adapter)
- **num-traits**: Numeric trait abstractions

## References

- [API Documentation](https://docs.rs/interval_adapter)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/interval_adapter)
- [Example](./examples/interval_adapter_trivial.rs)
- [readme.md](./readme.md)
- [std::ops::Range](https://doc.rust-lang.org/std/ops/struct.Range.html)
- [std::ops::Bound](https://doc.rust-lang.org/std/ops/enum.Bound.html)
