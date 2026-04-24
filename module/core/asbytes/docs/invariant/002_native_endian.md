# Invariant: Native Byte Order

### Scope

- **Purpose**: Document the byte order guarantee (and limitation) of all conversions.
- **Responsibility**: Define that all byte outputs reflect the host machine's native endianness.
- **In Scope**: All `AsBytes` and `IntoBytes` outputs for numeric types and structs.
- **Out of Scope**: Big-endian or little-endian normalization; cross-platform byte order interop; endianness detection.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/as_bytes.rs` | `bytemuck::cast_slice` / `bytemuck::bytes_of` — native memory as-is |
| source | `src/into_bytes.rs` | Same underlying cast functions |
| doc | `../feature/001_byte_conversion.md` | Feature context where this applies |
| doc | `../api/001_as_bytes_trait.md` | AsBytes producing native-order bytes |
| doc | `../api/002_into_bytes_trait.md` | IntoBytes producing native-order bytes |

### Invariant Statement

All bytes produced by `as_bytes()`, `to_bytes_vec()`, and `into_bytes()` reflect the host machine's native byte order. Multi-byte scalar values (`u16`, `u32`, `f64`, etc.) and struct fields appear in the byte order dictated by the CPU architecture (little-endian on x86/x64, big-endian on most SPARC/MIPS targets).

No byte-swapping, normalization, or endianness marker is inserted.

### Enforcement Mechanism

Not enforced programmatically — this is an inherent property of memory reinterpretation via `bytemuck`. `bytemuck::bytes_of` and `bytemuck::cast_slice` return the raw bytes of the object as stored in memory, which is always in native order.

### Violation Consequences

Byte sequences produced on a little-endian host are not binary-compatible with sequences produced on a big-endian host for the same value. Callers requiring portable byte order (network protocols, cross-platform binary files) must apply explicit endianness conversion (e.g., `u32::to_be_bytes()`) before or after using these traits. This crate provides no conversion utilities for this purpose.
