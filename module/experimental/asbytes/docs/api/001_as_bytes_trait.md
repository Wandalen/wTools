# API: AsBytes Trait

### Scope

- **Purpose**: Document the complete contract for borrowing data as a byte slice.
- **Responsibility**: Define trait methods, implementations, and their behavioral guarantees.
- **In Scope**: `as_bytes`, `byte_size`, `len`, `is_empty`, `to_bytes_vec` methods; four POD-bound implementations.
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

`AsBytes` provides a zero-copy byte-slice view of POD-safe data without consuming the source. The byte slice lifetime is tied to the borrow of the source. Three convenience methods (`byte_size`, `is_empty`, `to_bytes_vec`) have default implementations; only `as_bytes` and `len` are required.

### Operations

#### Trait Definition

The trait requires two methods: `as_bytes`, which returns a borrowed byte slice of the underlying memory, and `len`, which returns the element count. Three methods have default implementations: `to_bytes_vec` (allocates and returns a copy of the byte slice), `byte_size` (returns total size in bytes), and `is_empty` (returns true when element count is zero).

#### `as_bytes`

Returns a borrowed view of the underlying memory as a contiguous byte slice. Uses single-item reinterpretation for the one-element tuple implementation and multi-element cast for collection implementations. The slice is valid for the lifetime of the borrow.

#### `len`

Returns the number of elements, not bytes. A single-item tuple always returns 1. Collection implementations return the element count of the underlying container.

#### `byte_size`

Returns the total size in bytes. Defaults to the length of the byte slice; overridden in all four implementations for efficiency as element count multiplied by element size.

#### `is_empty`

Returns true when the element count is zero. Defaults to checking whether `len` returns zero.

#### `to_bytes_vec`

Allocates and returns an owned copy of the byte representation. Use `IntoBytes::into_bytes` when the source can be consumed and ownership transfer is the goal.

### Implementations

| Source Type | Byte View Strategy | Element Count |
|-------------|-------------------|---------------|
| Single POD item (one-element tuple) | Reinterpret item memory | Always 1 |
| POD vector | Cast contiguous slice of elements | Vector length |
| POD slice (borrowed) | Cast contiguous slice of elements | Slice length |
| POD fixed-size array | Cast contiguous slice of elements | Array length (compile-time constant) |

All four override `byte_size` for efficiency: element count multiplied by element size, without going through the byte slice length.

### Error Handling

No runtime errors. All conversions are statically guaranteed safe by the POD bound and the underlying byte-cast library's compile-time checks.

### Compatibility Guarantees

The method signatures (`as_bytes`, `len`, `byte_size`, `is_empty`, `to_bytes_vec`) are stable. The four provided implementations cover all supported source types; adding implementations for new types is a non-breaking extension. Removing or changing method signatures is a breaking change.

Byte output for a given value and host architecture is deterministic and repeatable across calls. The byte slice is always contiguous and reflects native memory layout without reordering or normalization.
