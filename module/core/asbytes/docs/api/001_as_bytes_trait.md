# API: AsBytes Trait

### Scope

- **Purpose**: Document the complete contract for borrowing data as a `&[u8]` byte slice.
- **Responsibility**: Define trait methods, implementations, and their behavioral guarantees.
- **In Scope**: `as_bytes`, `byte_size`, `len`, `is_empty`, `to_bytes_vec` methods; four `Pod`-bound implementations.
- **Out of Scope**: Ownership transfer (see `002_into_bytes_trait.md`); non-POD types; endianness conversion.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/as_bytes.rs` | Trait definition and all implementations |
| test | `tests/inc/as_bytes_test.rs` | Coverage tests for all implementations |
| doc | `../feature/001_byte_conversion.md` | Feature guide with usage patterns |
| doc | `002_into_bytes_trait.md` | Consuming counterpart |
| doc | `../invariant/001_pod_safety.md` | POD safety constraint on `T` |
| doc | `../invariant/002_native_endian.md` | Native byte order guarantee |

### Abstract

`AsBytes` provides a zero-copy `&[u8]` view of POD-safe data without consuming the source. The byte slice lifetime is tied to `&self`. Three convenience methods (`byte_size`, `is_empty`, `to_bytes_vec`) have default implementations; only `as_bytes` and `len` are required.

### Operations

#### Trait Definition

```rust
pub trait AsBytes
{
  fn as_bytes( &self ) -> &[ u8 ];
  fn len( &self ) -> usize;

  fn to_bytes_vec( &self ) -> Vec< u8 > { self.as_bytes().to_vec() }
  fn byte_size( &self ) -> usize { self.as_bytes().len() }
  fn is_empty( &self ) -> bool { self.len() == 0 }
}
```

#### `as_bytes`

```rust
fn as_bytes( &self ) -> &[ u8 ]
```

Returns a byte slice reinterpreting the underlying memory. Uses `bytemuck::bytes_of` for single items (tuple impl) and `bytemuck::cast_slice` for collections. The slice is valid for `'self`.

#### `len`

```rust
fn len( &self ) -> usize
```

Returns the number of **elements** (not bytes). For `(T,)` always returns `1`. For `Vec<T>`, `&[T]`, `[T; N]` returns the element count.

#### `byte_size`

```rust
fn byte_size( &self ) -> usize  // default: self.as_bytes().len()
```

Returns the total size in bytes. Equivalent to `size_of::<T>() * len()`.

#### `is_empty`

```rust
fn is_empty( &self ) -> bool  // default: self.len() == 0
```

Returns `true` when element count is zero.

#### `to_bytes_vec`

```rust
fn to_bytes_vec( &self ) -> Vec< u8 >  // default: self.as_bytes().to_vec()
```

Allocates and returns a copy of the byte representation. Use `IntoBytes::into_bytes` when ownership transfer is the goal and the source can be consumed.

### Implementations

| Type | `as_bytes` | `len` |
|------|-----------|-------|
| `( T : Pod, )` | `bytemuck::bytes_of( &self.0 )` | always `1` |
| `Vec< T : Pod >` | `bytemuck::cast_slice( self )` | `Vec::len()` |
| `[ T : Pod ]` | `bytemuck::cast_slice( self )` | `slice::len()` |
| `[ T : Pod ; N ]` | `bytemuck::cast_slice( self )` | const `N` |

All four override `byte_size` for efficiency (`size_of::<T>() * count` without calling `as_bytes().len()`).

### Error Handling

No runtime errors. All conversions are statically guaranteed safe by the `T: Pod` bound and `bytemuck`'s compile-time checks.
