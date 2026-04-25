# Feature: Byte Conversion

### Scope

- **Purpose**: Enable ergonomic, safe access to raw byte representations of POD types and standard collections.
- **Responsibility**: Document the byte viewing and consuming capability and its usage patterns.
- **In Scope**: `AsBytes` trait (borrow as a byte slice); `IntoBytes` trait (consume to owned byte vector); implementations for vectors, slices, arrays, tuples, strings, double-ended queues, null-terminated C strings, and boxed values; `bytemuck` re-exports.
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

`AsBytes` provides a zero-copy byte-slice view of POD data. The original value remains owned and accessible after calling `as_bytes`. A vector of POD items, for example, exposes all element bytes as a contiguous slice without copying; the collection remains usable afterward. `byte_size` returns the total byte count; `len` returns the element count.

Single POD items are wrapped in a one-element tuple to obtain a byte view. Wrapping as `(item,)` and calling `as_bytes` reinterprets the single item's memory as bytes without consuming the item.

#### Consuming Data into Bytes (IntoBytes)

`IntoBytes` consumes the value and produces an owned byte vector. Use this when the byte vector must outlive the source, be passed to an I/O sink, or be manipulated independently. A function parameterized over `IntoBytes` can accept any supported type — POD collections, strings, or C strings — and forward the bytes to any byte sink without knowing the source type at the call site.

#### Single Item Pattern

Both traits use a one-element tuple to handle single POD items uniformly. This avoids a blanket implementation over all POD types that would conflict with the collection implementations.

#### Feature Flags

```toml
[features]
default     = []
full        = [ "enabled", "as_bytes", "into_bytes", "derive", "must_cast" ]
enabled     = []
as_bytes    = [ "dep:bytemuck" ]
into_bytes  = [ "as_bytes" ]
derive      = [ "bytemuck?/derive" ]
must_cast   = [ "bytemuck?/must_cast" ]
```

`as_bytes` and `into_bytes` can be enabled independently. `into_bytes` depends on `as_bytes` for its implementations. Use `features = ["full"]` to enable all byte conversion functionality.
