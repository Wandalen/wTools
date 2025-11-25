# deterministic_rand

Hierarchical random number generators for concurrent simulations with switchable determinism.

## Overview

`deterministic_rand` provides a hierarchical random number generation (HRNG) system designed for parallel and concurrent applications where reproducibility is critical. The crate solves the fundamental tension between high-performance parallel code and deterministic behavior by providing RNG hierarchies that produce identical results across runs when the same seed is used, regardless of thread scheduling.

The key insight is that in parallel systems, random number generation tied to thread IDs creates non-deterministic behavior because thread scheduling varies between runs. This crate instead ties child generators to **batch indices**, ensuring that task `N` always gets the same random sequence regardless of which thread executes it.

### Scope

#### Responsibility

deterministic_rand is responsible for providing reproducible random number generation in concurrent and parallel applications. It manages hierarchical RNG structures where parent generators spawn deterministic child generators, enabling parallel code to produce identical results across runs.

#### In-Scope

- **Hierarchical RNG structure** (`Hrng`): Parent-child generator relationships with deterministic child creation
- **Seed management** (`Seed`): Flexible seed creation from strings, integers, or random sources
- **Shared generator access** (`SharedGenerator`): Thread-safe access to underlying RNG via `Arc<Mutex<...>>`
- **Feature-gated determinism**: Runtime behavior switching via `determinism` feature flag
- **Iterator determinism extension** (`IfDeterminismIteratorExt`): Conditional sorting for `HashMap`/`HashSet` iteration
- **Re-export of `rand` crate**: Full `rand` API available through this crate

#### Out-of-Scope

- **Distribution implementations**: Uses `rand`'s distributions, does not provide new ones
- **Cryptographic guarantees**: While ChaCha8 is cryptographically secure, this crate is for simulation determinism, not cryptography
- **Statistical testing**: No built-in statistical randomness testing
- **Persistent state serialization**: RNG state cannot be serialized/deserialized
- **Custom RNG algorithms**: Fixed to ChaCha8Rng (deterministic) or ThreadRng (non-deterministic)

#### Boundaries

- **Upstream**: Depends on `rand`, `rand_chacha`, `rand_seeder` for RNG implementation
- **Downstream**: Used by simulation, testing, and ML training code requiring reproducibility
- **Feature boundary**: `determinism` feature is the single control point for mode switching
- **Platform boundary**: `no_std` support available but excludes deterministic mode

## Architecture

### Module Structure

```
deterministic_rand/
├── src/
│   ├── lib.rs                    # Crate root with mod_interface and feature-conditional module selection
│   ├── hrng_deterministic.rs     # Hrng implementation using ChaCha8Rng (when determinism enabled)
│   ├── hrng_non_deterministic.rs # Hrng stub using ThreadRng (when determinism disabled)
│   ├── seed.rs                   # Seed struct for flexible seed creation
│   └── iter.rs                   # IfDeterminismIteratorExt trait for deterministic iteration
├── tests/
│   ├── basic_test.rs             # Parallel RNG tests, reusability, seed tests
│   ├── smoke_test.rs             # Basic smoke tests
│   └── assumption_test.rs        # RNG assumption validation
├── Cargo.toml
├── readme.md
└── spec.md
```

### Feature-Conditional Architecture

The crate uses compile-time feature selection to swap between two implementations:

```rust
// When `determinism` feature is enabled (default)
#[ cfg( feature = "determinism" ) ]
pub use hrng_deterministic as hrng;

// When `determinism` feature is disabled OR `no_std` is enabled
#[ cfg( any( not( feature = "determinism" ), feature = "no_std" ) ) ]
pub use hrng_non_deterministic as hrng;
```

Both modules export the same `Hrng` type with identical public API, enabling seamless switching.

### Core Data Structures

#### Hrng (Deterministic Mode)

```rust
#[ derive( Debug, Clone ) ]
pub struct Hrng
{
  /// List of child generators produced by this HRNG
  children: Arc< RwLock< Vec< Hrng > > >,
  /// Main generator for number generation
  generator: SharedGenerator,
  /// Separate generator for child creation (improves uniformness)
  children_generator: SharedGenerator,
}
```

Key design decisions:
- **Separate generators**: Data generation and child creation use different generators to improve statistical quality
- **Children caching**: Child generators are created once and cached in the `children` vector
- **RwLock for children**: Read-heavy access pattern (most calls retrieve existing children)
- **Mutex for generators**: Protects RNG state from concurrent access

#### Hrng (Non-Deterministic Mode)

```rust
#[ derive( Debug, Clone ) ]
pub struct Hrng;  // Zero-sized type

pub struct SharedGenerator;  // Stub that returns ThreadRng
```

The non-deterministic mode provides API compatibility with zero overhead:
- All methods are `#[inline(always)]`
- `child()` returns `Self` (no-op)
- `rng_ref()` returns a stub that produces `ThreadRng`

### Generator Hierarchy Pattern

```
Master (seed = "experiment_42")
├── Child[0] (batch 0) → deterministic sequence A
├── Child[1] (batch 1) → deterministic sequence B
│   ├── Child[0] (sub-batch 0) → deterministic sequence B.0
│   └── Child[1] (sub-batch 1) → deterministic sequence B.1
├── Child[2] (batch 2) → deterministic sequence C
...
```

Each child's seed is derived from the parent's `children_generator`, ensuring:
- Same index always produces same child
- Children are independent (no sequence overlap)
- Hierarchy can be arbitrarily deep

## Public API

### Types

#### `Hrng`

The hierarchical random number generator.

```rust
impl Hrng
{
  /// Create master HRNG with default seed ("master_seed")
  pub fn master() -> Self;

  /// Create master HRNG with custom seed
  pub fn master_with_seed( seed: Seed ) -> Self;

  /// Get thread-safe reference to underlying RNG
  pub fn rng_ref( &self ) -> SharedGenerator;

  /// Get or create child generator at given index
  pub fn child( &self, index: usize ) -> Self;

  /// Number of children created (diagnostic)
  pub fn _children_len( &self ) -> usize;
}

impl Default for Hrng
{
  fn default() -> Self { Hrng::master() }
}
```

#### `SharedGenerator`

Type alias for thread-safe RNG access.

```rust
// Deterministic mode
pub type SharedGenerator = Arc< Mutex< ChaCha8Rng > >;

// Non-deterministic mode (API-compatible stub)
pub struct SharedGenerator;
impl SharedGenerator
{
  pub fn lock( self ) -> SharedGeneratorLock;
}
```

#### `Seed`

Flexible seed initialization.

```rust
#[ derive( Clone, Debug, PartialEq, Eq ) ]
pub struct Seed( String );

impl Seed
{
  /// Create seed from any string-convertible value
  pub fn new< IntoString: Into< String > >( value: IntoString ) -> Self;

  /// Create seed from integer (formats as "master_seed_{n}")
  pub fn from_integer( src: u64 ) -> Self;

  /// Create seed from 16 random alphanumeric characters
  pub fn random() -> Self;

  /// Extract inner string value
  pub fn into_inner( self ) -> String;
}

impl Default for Seed
{
  fn default() -> Self { Self( "master_seed".to_owned() ) }
}

impl< IntoString: Into< String > > From< IntoString > for Seed
{
  fn from( src: IntoString ) -> Self { Self::new( src ) }
}
```

### Traits

#### `IfDeterminismIteratorExt`

Iterator extension for deterministic collection iteration.

```rust
#[ sealed ]
pub trait IfDeterminismIteratorExt: Iterator
{
  /// Sort iterator items (no-op when determinism disabled)
  fn if_determinism_then_sort( self ) -> impl Iterator< Item = Self::Item >
  where
    Self: Sized,
    Self::Item: Ord;

  /// Sort iterator items with custom comparator (no-op when determinism disabled)
  fn if_determinism_then_sort_by< F >( self, cmp: F ) -> impl Iterator< Item = Self::Item >
  where
    Self: Sized,
    F: FnMut( &Self::Item, &Self::Item ) -> Ordering;
}

// Blanket implementation for all iterators
impl< T: Iterator > IfDeterminismIteratorExt for T {}
```

### Re-exports

The crate re-exports the entire `rand` crate for convenience:

```rust
pub use ::rand::*;  // Rng, distributions, SeedableRng, etc.
```

This allows users to import `Rng`, `Uniform`, etc. directly from `deterministic_rand`.

## Usage Patterns

### Basic Usage

```rust
use deterministic_rand::{ Hrng, Rng };

let hrng = Hrng::master();
let rng_ref = hrng.rng_ref();
let mut rng = rng_ref.lock().unwrap();
let value: u64 = rng.gen();
```

### Parallel Monte Carlo Simulation

```rust
use deterministic_rand::{ Hrng, Rng };
use rand::distributions::Uniform;
use rayon::prelude::*;

let range = Uniform::new( -1.0f64, 1.0 );
let hrng = Hrng::master_with_seed( "pi_estimation".into() );

let count: u64 = ( 0..100 )
  .into_par_iter()
  .map( | batch_id |
  {
    // Each batch gets its own deterministic child
    let child = hrng.child( batch_id );
    let rng_ref = child.rng_ref();
    let mut rng = rng_ref.lock().unwrap();

    let mut hits = 0u64;
    for _ in 0..1000
    {
      let x = rng.sample( range );
      let y = rng.sample( range );
      if x * x + y * y <= 1.0 { hits += 1; }
    }
    hits
  })
  .sum();

let pi = 4.0 * ( count as f64 ) / 100_000.0;
// pi ≈ 3.1438 (deterministic across runs)
```

### Deterministic HashMap Iteration

```rust
use deterministic_rand::IfDeterminismIteratorExt;
use std::collections::HashMap;

let map: HashMap< &str, i32 > = [( "a", 1 ), ( "b", 2 ), ( "c", 3 )].into();

// Non-deterministic: order varies between runs
for ( k, v ) in map.iter() { /* ... */ }

// Deterministic: always same order when `determinism` feature enabled
for ( k, v ) in map.iter().if_determinism_then_sort() { /* ... */ }
```

### Custom Seed Strategies

```rust
use deterministic_rand::{ Hrng, Seed };

// From string (any Into<String>)
let hrng = Hrng::master_with_seed( "experiment_42".into() );
let hrng = Hrng::master_with_seed( Seed::new( "experiment_42" ) );

// From integer
let hrng = Hrng::master_with_seed( Seed::from_integer( 12345 ) );

// Random seed for non-reproducible runs
let hrng = Hrng::master_with_seed( Seed::random() );

// Default seed
let hrng = Hrng::master();  // Uses "master_seed"
```

### Reusing Child Generators

```rust
use deterministic_rand::{ Hrng, Rng };

let hrng = Hrng::master();

// First access creates the child
let child = hrng.child( 0 );
let v1: u64 = child.rng_ref().lock().unwrap().gen();

// Second access returns the SAME child (cached)
let child = hrng.child( 0 );
let v2: u64 = child.rng_ref().lock().unwrap().gen();

// v1 and v2 are DIFFERENT (sequence continues from same RNG)
// But if we create a new master with same seed, we get v1 again:
let hrng2 = Hrng::master();
let child2 = hrng2.child( 0 );
let v3: u64 = child2.rng_ref().lock().unwrap().gen();
assert_eq!( v1, v3 );  // Same seed → same sequence
```

### Nested Hierarchies

```rust
use deterministic_rand::{ Hrng, Rng };

let master = Hrng::master_with_seed( "simulation".into() );

// Level 1: worker threads
let worker_0 = master.child( 0 );
let worker_1 = master.child( 1 );

// Level 2: sub-tasks within workers
let subtask_0_0 = worker_0.child( 0 );
let subtask_0_1 = worker_0.child( 1 );
let subtask_1_0 = worker_1.child( 0 );

// Each subtask has a unique, deterministic sequence
```

### Switching Modes at Compile Time

```toml
# Cargo.toml

# Production: fast non-deterministic mode
[dependencies]
deterministic_rand = { version = "0.6", default-features = false }

# Testing: deterministic mode (default)
[dev-dependencies]
deterministic_rand = { version = "0.6" }
```

## Dependencies and Consumers

### Dependencies

| Dependency | Feature | Purpose |
|------------|---------|---------|
| `rand` | always | Core RNG traits and re-exports |
| `rand_chacha` | `determinism` | ChaCha8Rng for deterministic generation |
| `rand_seeder` | `determinism` | Seed any RNG from string seeds |
| `iter_tools` | `determinism` | `sorted()` method for iterator extension |
| `sealed` | always | Seal the `IfDeterminismIteratorExt` trait |
| `mod_interface` | always | Traditional namespace organization |

### Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `enabled` | ✓ | Enables the crate |
| `determinism` | ✓ | Use ChaCha8Rng for reproducible generation |
| `no_std` | - | Disable std; also disables determinism |

### Potential Consumers

- **Simulation frameworks**: Monte Carlo, agent-based, physics simulations
- **Machine learning training**: Reproducible initialization and data augmentation
- **Testing infrastructure**: Deterministic fuzzing and property-based testing
- **Game development**: Reproducible procedural generation
- **Scientific computing**: Reproducible numerical experiments

## Design Rationale

### Why Hierarchical Structure?

Traditional approaches to parallel RNG have problems:

1. **Single shared RNG with mutex**: Serializes all random access, destroying parallelism
2. **Thread-local RNG**: Non-deterministic because thread scheduling varies
3. **Pre-generated arrays**: Memory-intensive and inflexible

The hierarchical approach solves these:
- **Parallelism preserved**: Each task gets its own mutex-protected RNG
- **Determinism guaranteed**: Child N always produces sequence N regardless of thread
- **Memory efficient**: Children created on-demand and cached

### Why ChaCha8?

ChaCha8 provides:
- **Speed**: Faster than ChaCha20 while still cryptographically secure
- **Quality**: Passes all statistical randomness tests
- **Seedability**: Can be seeded from arbitrary strings via `rand_seeder`
- **Determinism**: Same seed always produces same sequence

### Why Separate children_generator?

Using a separate RNG for child creation:
- **Independence**: Calling `child()` doesn't affect the main sequence
- **Quality**: Different RNG stream for seeding vs data generation
- **Predictability**: Child N is always seeded the same way

### Why Sealed Trait for Iterator Extension?

`IfDeterminismIteratorExt` is sealed because:
- It's a behavioral contract, not an extension point
- Users should not need to implement it
- Allows internal implementation changes without breaking changes

## Testing Strategy

### Test Categories

1. **Determinism tests**: Verify same seed produces same results
2. **Parallel tests**: Verify correctness under concurrent access (rayon)
3. **Reusability tests**: Verify child caching works correctly
4. **Seed tests**: Verify various seed creation methods

### Key Test Cases

```rust
// Monte Carlo pi estimation with deterministic result
#[ test ]
fn test_rng_manager()
{
  let hrng = Hrng::master();
  // ... parallel computation ...
  assert_eq!( _got_pi, 3.1438 );  // Exact match!
}

// Child reuse across multiple accesses
#[ test ]
fn test_reusability()
{
  let hrng = Hrng::master();
  let c1 = hrng.child( 0 );
  let v1 = c1.rng_ref().lock().unwrap().gen::< u64 >();

  let hrng2 = Hrng::master();  // New master, same seed
  let c2 = hrng2.child( 0 );
  let v2 = c2.rng_ref().lock().unwrap().gen::< u64 >();

  assert_eq!( v1, v2 );  // Same child produces same first value
}

// Parallel access produces deterministic results
#[ test ]
fn test_par()
{
  // Run twice with same seed, verify same results
}
```

### Running Tests

```bash
# Default (deterministic mode)
cargo test

# Non-deterministic mode
cargo test --no-default-features

# With no_std
cargo test --features no_std --no-default-features
```

## Future Considerations

### Potential Enhancements

1. **Custom RNG backends**: Allow users to specify RNG algorithm beyond ChaCha8
2. **State serialization**: Save/restore RNG state for checkpointing
3. **Async support**: Async-friendly child creation
4. **Distribution helpers**: Pre-configured common distributions
5. **Statistics tracking**: Optional tracking of RNG usage patterns

### Known Limitations

1. **no_std excludes determinism**: The `no_std` feature disables deterministic mode
2. **No state persistence**: Cannot serialize/deserialize RNG state
3. **Fixed algorithm**: ChaCha8 cannot be changed at runtime
4. **Mutex overhead**: Each generator access requires mutex lock

## Adoption Guidelines

### When to Use

- **Parallel simulations** requiring reproducibility
- **Testing** where deterministic random behavior simplifies debugging
- **Scientific computing** where reproducibility is mandatory
- **ML training** where reproducible initialization is needed

### When Not to Use

- **Cryptography**: Use dedicated crypto RNG crates
- **Single-threaded code**: Standard `rand::thread_rng()` is simpler
- **Maximum performance**: Mutex overhead may be unacceptable
- **no_std with determinism**: Not supported

### Migration Path

```rust
// Before: non-deterministic
use rand::{ thread_rng, Rng };
let mut rng = thread_rng();
let value: u64 = rng.gen();

// After: deterministic
use deterministic_rand::{ Hrng, Rng };
let hrng = Hrng::master_with_seed( "my_seed".into() );
let rng_ref = hrng.rng_ref();
let value: u64 = rng_ref.lock().unwrap().gen();
```

## Related Crates

| Crate | Relationship |
|-------|--------------|
| `rand` | Upstream dependency, fully re-exported |
| `rand_chacha` | Provides ChaCha8Rng implementation |
| `rand_seeder` | Enables string-based seeding |
| `iter_tools` | Provides `sorted()` for iterator extension |

## References

- [rand crate documentation](https://docs.rs/rand)
- [ChaCha cipher family](https://cr.yp.to/chacha.html)
- [Parallel random number generation strategies](https://www.pcg-random.org/posts/critiquing-pcg-streams.html)
