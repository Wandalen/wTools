# API: IntoBytes Trait

### Scope

- **Purpose**: Document the complete contract for consuming data into an owned `Vec<u8>`.
- **Responsibility**: Define the trait method, all eleven implementations, and their behavioral guarantees.
- **In Scope**: `into_bytes` method; eleven implementations covering tuples, references, owned collections, string types, and `CString`.
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

`IntoBytes` consumes `self` and produces an owned `Vec<u8>`. It covers eleven distinct source types, including owned and borrowed collections, string types, boxed values, `VecDeque`, and `CString`. For reference types (`&T`, `&str`, `&[T]`) the bytes are copied; for owned types the conversion may avoid allocation where the source already holds contiguous memory.

### Operations

#### Trait Definition

```rust
pub trait IntoBytes
{
  fn into_bytes( self ) -> Vec< u8 >;
}
```

#### `into_bytes`

```rust
fn into_bytes( self ) -> Vec< u8 >
```

Consumes `self`. Returns an owned byte vector containing the raw byte representation. Move semantics apply: non-`Copy` values are no longer accessible after this call.

### Implementations

| Type | Strategy | Allocates |
|------|----------|-----------|
| `( T : Pod, )` | `bytemuck::bytes_of( &self.0 ).to_vec()` | yes |
| `&T where T : Pod` | `bytemuck::bytes_of( self ).to_vec()` | yes |
| `String` | `String::into_bytes()` — moves internal buffer | no |
| `&str` | `self.as_bytes().to_vec()` | yes |
| `[ T : Pod ; N ]` | `bytemuck::cast_slice( &self ).to_vec()` | yes |
| `Vec< T : Pod >` | `bytemuck::cast_slice( self.as_slice() ).to_vec()` | yes |
| `Box< T : Pod >` | `bytemuck::bytes_of( &*self ).to_vec()` | yes |
| `&[ T : Pod ]` | `bytemuck::cast_slice( self ).to_vec()` | yes |
| `Box< [ T : Pod ] >` | `bytemuck::cast_slice( &self ).to_vec()` | yes |
| `VecDeque< T : Pod >` | iterate elements, extend byte vec per element | yes |
| `std::ffi::CString` | `CString::into_bytes()` — strips trailing NUL | no |

#### VecDeque Behavior

`VecDeque` may hold its data in a non-contiguous ring buffer (head wrapped past the end). The implementation iterates all elements in logical order (front to back), appending each element's bytes via `bytemuck::bytes_of`. This produces a contiguous byte sequence matching the logical element order regardless of internal layout.

#### CString Behavior

`CString::into_bytes()` strips the trailing NUL terminator. The returned `Vec<u8>` contains only the payload bytes. A `CString` constructed from `"hello"` produces 5 bytes, not 6.

### Error Handling

No runtime errors. Conversions using generic `T: Pod` are statically guaranteed safe. `String` and `CString` conversions delegate to standard library methods with no failure paths.
