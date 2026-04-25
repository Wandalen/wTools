# API: IntoBytes Trait

### Scope

- **Purpose**: Document the complete contract for consuming data into an owned byte vector.
- **Responsibility**: Define the trait method, all eleven implementations, and their behavioral guarantees.
- **In Scope**: `into_bytes` method; eleven implementations covering tuples, references, owned collections, string types, and null-terminated C strings.
- **Out of Scope**: Zero-copy borrowing (see `001_as_bytes_trait.md`); non-POD types; endianness conversion.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/into_bytes.rs` | Trait definition and all implementations |
| test | `tests/inc/into_bytes_test.rs` | Coverage tests for all implementations |
| doc | `../feature/001_byte_conversion.md` | Feature guide with usage patterns |
| doc | `001_as_bytes_trait.md` | Borrowing counterpart |
| doc | `../invariant/001_pod_safety.md` | POD safety constraint on generic `T` |
| doc | `../invariant/002_native_endian.md` | Native byte order guarantee |

### Abstract

`IntoBytes` consumes `self` and produces an owned byte vector. It covers eleven distinct source types, including owned and borrowed collections, string types, boxed values, double-ended queues, and null-terminated C strings. For reference types the bytes are copied; for owned types the conversion may avoid allocation where the source already holds contiguous memory.

### Operations

#### Trait Definition

The trait requires one method: `into_bytes`, which consumes the value and returns an owned byte vector.

#### `into_bytes`

Consumes the source value and returns all bytes as an owned vector. For non-copy owned types, the source is no longer accessible after the call. For reference types, bytes are copied into the returned vector.

### Implementations

| Source Type | Strategy | Allocates |
|-------------|----------|-----------|
| Single POD item (one-element tuple) | Reinterpret item memory; copy to owned vector | yes |
| Borrowed POD reference | Reinterpret item memory; copy to owned vector | yes |
| Owned UTF-8 string | Move internal buffer; no reallocation | no |
| Borrowed string slice | Copy UTF-8 bytes to owned vector | yes |
| POD fixed-size array | Cast array as byte slice; copy to owned vector | yes |
| POD vector | Cast element slice as bytes; copy to owned vector | yes |
| Boxed POD item | Reinterpret item memory; copy to owned vector | yes |
| Borrowed POD slice | Cast slice as bytes; copy to owned vector | yes |
| Boxed POD slice | Cast boxed slice as bytes; copy to owned vector | yes |
| POD double-ended queue | Iterate elements in logical order; append bytes per element | yes |
| Null-terminated C string | Move internal buffer; NUL terminator stripped | no |

#### Double-Ended Queue Behavior

A double-ended queue may hold its data in a non-contiguous ring buffer (head wrapped past the end). The implementation iterates all elements in logical order (front to back), appending each element's bytes individually. This produces a contiguous byte sequence matching the logical element order regardless of internal layout.

#### C String Behavior

The standard library `CString::into_bytes()` strips the trailing NUL terminator. The returned byte vector contains only the payload bytes. A C string constructed from `"hello"` produces 5 bytes, not 6.

### Error Handling

No runtime errors. Conversions using generic POD-bounded types are statically guaranteed safe. String and C string conversions delegate to standard library methods with no failure paths.

### Compatibility Guarantees

The `into_bytes` method signature is stable. The eleven provided implementations cover all supported source types; adding implementations for new types is a non-breaking extension. The allocation behavior documented above (which conversions avoid allocation) is stable — callers may rely on it.

For string types, byte content exactly matches the standard library's `into_bytes()` output. For POD types, byte content matches native memory layout as described in `../invariant/002_native_endian.md`.
