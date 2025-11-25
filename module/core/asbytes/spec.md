# asbytes

Traits for viewing data as byte slices or consuming data into byte vectors, built on `bytemuck` for POD safety.

## Overview

`asbytes` provides two primary traits that offer a unified, trait-based approach for working with raw byte representations of data:

1. **`AsBytes`** - View data structures as `&[u8]` without ownership transfer
2. **`IntoBytes`** - Consume data structures into owned `Vec<u8>`

Both traits rely on `bytemuck` for Plain Old Data (POD) safety guarantees, ensuring that only types safe to interpret as raw bytes are supported.

### Scope

#### Responsibility

asbytes is responsible for providing ergonomic, safe conversions between Rust data structures and raw byte representations. It abstracts the common pattern of needing byte views for serialization, hashing, I/O operations, and low-level APIs (graphics, networking).

#### In-Scope

- **`AsBytes` trait**: Borrow data as `&[u8]` with `as_bytes()`, `byte_size()`, `len()` methods
- **`IntoBytes` trait**: Consume data into `Vec<u8>` with `into_bytes()` method
- **POD type support**: Vec, slices, arrays of Pod types
- **Single item support**: Single Pod items via tuple wrapper `(item,)`
- **String/Vec support**: String and Vec<u8> consumption
- **Bytemuck re-exports**: Pod, Zeroable, derives, and utility functions

#### Out-of-Scope

- **Non-POD types**: Types with padding, pointers, or non-trivial representations
- **Endianness conversion**: Uses native byte order only
- **Streaming**: No incremental byte production
- **Parsing**: No byte-to-type conversion (opposite direction)

#### Boundaries

- **Upstream**: Depends on `bytemuck` for POD safety guarantees
- **Downstream**: Used by code needing byte views for serialization, hashing, I/O
- **Safety boundary**: All conversions are safe due to Pod constraints

## Architecture

### Module Structure

```
asbytes/
├── src/
│   ├── lib.rs              # Crate root with namespace organization
│   ├── as_bytes.rs         # AsBytes trait and implementations
│   └── into_bytes.rs       # IntoBytes trait and implementations
├── examples/
├── tests/
├── Cargo.toml
├── readme.md
└── spec.md
```

### Trait Relationships

```
bytemuck::Pod
    │
    ├── AsBytes trait
    │   ├── impl for Vec<T: Pod>
    │   ├── impl for &[T: Pod]
    │   ├── impl for [T: Pod; N]
    │   └── impl for (T: Pod,) (single item)
    │
    └── IntoBytes trait (depends on as_bytes)
        ├── impl for Vec<T: Pod>
        ├── impl for [T: Pod; N]
        ├── impl for (T: Pod,) (single item)
        ├── impl for String
        └── impl for Vec<u8>
```

## Public API

### Traits

#### `AsBytes`

View data as byte slice without consuming.

```rust
pub trait AsBytes
{
  /// Get byte slice view of the data
  fn as_bytes( &self ) -> &[ u8 ];

  /// Get total size in bytes
  fn byte_size( &self ) -> usize;

  /// Get number of elements
  fn len( &self ) -> usize;

  /// Check if empty
  fn is_empty( &self ) -> bool
  {
    self.len() == 0
  }
}
```

#### `IntoBytes`

Consume data into owned byte vector.

```rust
pub trait IntoBytes
{
  /// Consume self and return owned byte vector
  fn into_bytes( self ) -> Vec< u8 >;
}
```

### Re-exports from bytemuck

```rust
// Core traits
pub use bytemuck::{ Pod, Zeroable, NoUninit, AnyBitPattern };

// Derive macros (with `derive` feature)
pub use bytemuck::{ Pod, Zeroable }; // as derive macros

// Cast functions
pub use bytemuck::{
  cast, cast_ref, cast_mut,
  cast_slice, cast_slice_mut,
  bytes_of, bytes_of_mut,
  from_bytes, from_bytes_mut,
  try_cast, try_cast_ref, try_cast_mut,
  try_cast_slice, try_cast_slice_mut,
  try_from_bytes, try_from_bytes_mut,
  // ... and more
};
```

## Usage Patterns

### Viewing Data as Bytes (AsBytes)

```rust
use asbytes::AsBytes;

#[ repr( C ) ]
#[ derive( Clone, Copy, asbytes::Pod, asbytes::Zeroable ) ]
struct Point { x: f32, y: f32 }

// Collections
let points: Vec< Point > = vec![ Point { x: 1.0, y: 2.0 } ];
let bytes: &[ u8 ] = points.as_bytes();
println!( "Size: {} bytes, {} elements", points.byte_size(), points.len() );

// Slices
let slice: &[ Point ] = &points[..];
let bytes: &[ u8 ] = slice.as_bytes();

// Arrays
let array: [ Point; 2 ] = [ Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 } ];
let bytes: &[ u8 ] = array.as_bytes();

// Single item (wrap in tuple)
let point = Point { x: 5.0, y: 6.0 };
let bytes: &[ u8 ] = ( point, ).as_bytes();
```

### Consuming Data into Bytes (IntoBytes)

```rust
use asbytes::IntoBytes;

#[ repr( C ) ]
#[ derive( Clone, Copy, asbytes::Pod, asbytes::Zeroable ) ]
struct Header { id: u64, len: u32, checksum: u16, _pad: [ u8; 2 ] }

// POD struct (via tuple)
let header = Header { id: 1, len: 100, checksum: 0xABCD, _pad: [ 0; 2 ] };
let bytes: Vec< u8 > = ( header, ).into_bytes();

// Vec<T: Pod>
let floats: Vec< f32 > = vec![ 1.0, 2.0, 3.0 ];
let bytes: Vec< u8 > = floats.into_bytes();

// String
let message = String::from( "Hello, bytes!" );
let bytes: Vec< u8 > = message.into_bytes();

// Array
let array: [ u32; 4 ] = [ 1, 2, 3, 4 ];
let bytes: Vec< u8 > = array.into_bytes();
```

### Generic Function with AsBytes

```rust
use asbytes::AsBytes;

fn hash_bytes< T: AsBytes >( data: &T ) -> u64
{
  let bytes = data.as_bytes();
  // Simple hash for demonstration
  bytes.iter().fold( 0u64, | acc, &b | acc.wrapping_add( b as u64 ) )
}

let data: Vec< u32 > = vec![ 1, 2, 3 ];
let hash = hash_bytes( &data );
```

### Generic Function with IntoBytes

```rust
use asbytes::IntoBytes;
use std::io::Write;

fn send_data< T: IntoBytes, W: Write >( data: T, writer: &mut W ) -> std::io::Result<()>
{
  let bytes = data.into_bytes();
  writer.write_all( &bytes )
}

let mut buffer = Vec::new();
send_data( vec![ 1u32, 2, 3 ], &mut buffer )?;
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `enabled` | ✓ | Enable the crate |
| `as_bytes` | ✓ | Enable AsBytes trait |
| `into_bytes` | ✓ | Enable IntoBytes trait |
| `derive` | ✓ | Enable Pod/Zeroable derive macros |
| `must_cast` | ✓ | Enable must_cast functions |
| `extern_crate_alloc` | - | Allocation utilities in no_std |
| `zeroable_maybe_uninit` | - | MaybeUninit support |
| `wasm_simd` | - | WASM SIMD support |
| `aarch64_simd` | - | ARM64 SIMD support |
| `min_const_generics` | - | Const generics support |
| `const_zeroed` | - | Const zeroed values |

## Dependencies and Consumers

### Dependencies

| Dependency | Feature | Purpose |
|------------|---------|---------|
| `bytemuck` | `as_bytes` | POD safety, byte casting |

### Potential Consumers

- Serialization systems needing raw byte access
- Hashing utilities
- Graphics/GPU buffer preparation
- Network packet construction
- Memory-mapped file interfaces

## Design Rationale

### Why Wrap bytemuck?

While `bytemuck` provides core casting functions, it lacks:
1. **Unified trait interface**: `AsBytes` and `IntoBytes` provide consistent API
2. **Convenience methods**: `byte_size()`, `len()` alongside `as_bytes()`
3. **Generic constraints**: Easy `T: AsBytes` bounds for generic code

### Why Tuple Wrapper for Single Items?

Single POD items need special handling because:
- `bytemuck::bytes_of` works on references
- Traits need owned or borrowed context
- `(item,)` provides uniform slice-like treatment

### Why Separate as_bytes and into_bytes Features?

- **Modularity**: Use only what you need
- **Dependency control**: `into_bytes` requires allocation
- **no_std flexibility**: `as_bytes` works without alloc

## Testing Strategy

### Test Categories

1. **POD type coverage**: All standard Pod types work
2. **Collection coverage**: Vec, slice, array implementations
3. **Single item**: Tuple wrapper works correctly
4. **Size calculations**: `byte_size()` and `len()` are accurate

### Running Tests

```bash
# Default features
cargo test

# Full features
cargo test --features full

# With derives
cargo test --features derive
```

## Future Considerations

### Potential Enhancements

1. **Endianness conversion**: Optional byte order normalization
2. **Streaming iterator**: Yield byte chunks incrementally
3. **Bidirectional**: FromBytes trait for parsing

### Known Limitations

1. **POD only**: Cannot represent types with pointers/padding
2. **Native endian**: No cross-platform byte order
3. **No validation**: Assumes Pod safety via bytemuck

## Adoption Guidelines

### When to Use

- Need byte views for serialization/hashing
- Working with binary protocols
- Preparing data for graphics/networking APIs
- Generic code over byteable types

### When Not to Use

- Non-POD types with complex layouts
- Need endianness conversion
- Parsing bytes back to types (use bytemuck directly)

### Migration from bytemuck

```rust
// Before: Direct bytemuck
let bytes = bytemuck::cast_slice( &data );

// After: asbytes trait
use asbytes::AsBytes;
let bytes = data.as_bytes();
```

## Related Crates

| Crate | Relationship |
|-------|--------------|
| `bytemuck` | Upstream POD safety provider |
| `zerocopy` | Alternative approach to zero-copy |
| `bytes` | Byte buffer utilities (different focus) |

## References

- [bytemuck documentation](https://docs.rs/bytemuck)
- [Pod trait explanation](https://docs.rs/bytemuck/latest/bytemuck/trait.Pod.html)
