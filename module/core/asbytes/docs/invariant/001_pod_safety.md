# Invariant: POD Safety

### Scope

- **Purpose**: Define the safety constraint that all byte reinterpretation operates only on Plain Old Data types.
- **Responsibility**: Document the `bytemuck::Pod` requirement and its consequences for all generic implementations.
- **In Scope**: `T: Pod` bounds on all generic `AsBytes` and `IntoBytes` implementations; compile-time enforcement via trait bounds.
- **Out of Scope**: Unsafe coercions; manual `Pod` attestation outside `bytemuck`; string types and `CString` (handled via non-generic specialized impls).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/as_bytes.rs` | `impl< T: Pod > AsBytes for ...` bounds |
| source | `src/into_bytes.rs` | `impl< T: Pod > IntoBytes for ...` bounds |
| test | `tests/inc/as_bytes_test.rs` | Tests verifying Pod-typed conversions |
| test | `tests/inc/into_bytes_test.rs` | Tests verifying Pod-typed conversions |
| doc | `../feature/001_byte_conversion.md` | Feature relying on this safety model |
| doc | `../api/001_as_bytes_trait.md` | AsBytes trait enforcing this invariant |
| doc | `../api/002_into_bytes_trait.md` | IntoBytes trait enforcing this invariant |

### Invariant Statement

Every generic type parameter `T` in `AsBytes` and `IntoBytes` implementations must satisfy `T: bytemuck::Pod`. No conversion may reinterpret memory for a type that is not `Pod`-safe.

`bytemuck::Pod` certifies that `T`:
1. Has no uninitialized bytes (no implicit padding)
2. Has no pointer fields
3. Is valid for any bit pattern of its size

Types requiring `#[repr(C)]` with explicit padding (no compiler-inserted gaps) are the standard case. Primitive types (`u8`, `i32`, `f64`, `bool`, `[T; N]` where `T: Pod`, etc.) implement `Pod` unconditionally. User-defined structs must derive or manually implement `Pod` via `bytemuck`.

### Enforcement Mechanism

Enforced statically by the Rust compiler through `T: Pod` trait bounds. No runtime check is required or performed. Attempting to call `as_bytes()` or `into_bytes()` on a non-`Pod` type yields a compile-time error.

Non-generic implementations for `String`, `&str`, `CString` bypass the `Pod` constraint by routing through safe standard library byte representations (`str::as_bytes`, `String::into_bytes`, `CString::into_bytes`) rather than memory reinterpretation.

### Violation Consequences

The `Pod` constraint cannot be violated at runtime — it is a compile-time gate. A struct with implicit padding fields cannot implement `Pod` without `#[repr(C)]` and explicit padding fields. Attempting to derive `Pod` on such a type fails at compile time with a `bytemuck` error.
