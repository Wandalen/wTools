# Feature: Byte Conversion

### Scope

- **Purpose**: Enable ergonomic, safe access to raw byte representations of POD types and standard collections.
- **Responsibility**: Document the byte viewing and consuming capability and its usage patterns.
- **In Scope**: `AsBytes` trait (borrow as `&[u8]`); `IntoBytes` trait (consume to `Vec<u8>`); implementations for `Vec`, slices, arrays, tuples, `String`, `&str`, `VecDeque`, `CString`, `Box`; `bytemuck` re-exports.
- **Out of Scope**: Endianness conversion; streaming byte production; parsing bytes back to types; non-POD types.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/as_bytes.rs` | AsBytes trait definition and implementations |
| source | `src/into_bytes.rs` | IntoBytes trait definition and implementations |
| test | `tests/inc/as_bytes_test.rs` | AsBytes coverage tests |
| test | `tests/inc/into_bytes_test.rs` | IntoBytes coverage tests |
| doc | `../api/001_as_bytes_trait.md` | AsBytes trait API contract |
| doc | `../api/002_into_bytes_trait.md` | IntoBytes trait API contract |
| doc | `../invariant/001_pod_safety.md` | POD safety constraint |
| doc | `../invariant/002_native_endian.md` | Native byte order constraint |

### Design

#### Viewing Data as Bytes (AsBytes)

`AsBytes` provides a zero-copy `&[u8]` view of POD data. The original value remains owned and accessible after calling `as_bytes()`.

```rust
use asbytes::AsBytes;

#[ repr( C ) ]
#[ derive( Clone, Copy, asbytes::Pod, asbytes::Zeroable ) ]
struct Point { x : f32, y : f32 }

let points : Vec< Point > = vec![ Point { x : 1.0, y : 2.0 } ];
let bytes : &[ u8 ] = points.as_bytes();      // 8 bytes, no copy
println!( "Size: {} bytes, {} elements", points.byte_size(), points.len() );
// points is still accessible here
```

Single POD items are wrapped in a one-element tuple to obtain a byte view:

```rust
let point = Point { x : 5.0, y : 6.0 };
let bytes : &[ u8 ] = ( point, ).as_bytes();  // 8 bytes
```

#### Consuming Data into Bytes (IntoBytes)

`IntoBytes` consumes the value and produces an owned `Vec<u8>`. Use this when the byte vector must outlive the source, be passed to an I/O sink, or be manipulated independently.

```rust
use asbytes::IntoBytes;
use std::io::Write;

fn send_data< T : IntoBytes, W : Write >( data : T, writer : &mut W ) -> std::io::Result< () >
{
  writer.write_all( &data.into_bytes() )
}

let mut buf = Vec::new();
send_data( vec![ 1u32, 2, 3 ], &mut buf ).unwrap();  // 12 bytes
send_data( String::from( "hello" ), &mut buf ).unwrap();
```

#### Single Item Pattern

Both traits use a one-element tuple `(item,)` to handle single POD items uniformly. This avoids a separate blanket impl for `T: Pod` which would conflict with collection impls.

#### Feature Flags

```toml
[features]
default     = [ "enabled", "as_bytes", "into_bytes", "derive", "must_cast" ]
enabled     = []
as_bytes    = [ "dep:bytemuck" ]
into_bytes  = [ "as_bytes" ]
derive      = [ "bytemuck/derive" ]
must_cast   = [ "bytemuck/must_cast" ]
```

`as_bytes` and `into_bytes` can be used independently. `into_bytes` depends on `as_bytes` for its implementations. Both require the `enabled` flag to be active (on by default).
