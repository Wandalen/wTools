# API: Derive API

### Scope

- Documents the derive macros that form the public interface of this crate.
- Covers the name, purpose, and applicability conditions of each macro.
- Audience: contributors implementing or extending the macro collection.
- Does not cover the `derive_tools` facade or consumer usage patterns.

### Abstract

The crate exposes a set of derive procedural macros as its public interface.
Each macro is conditionally compiled based on the corresponding feature flag.
When the feature is disabled the macro is not available.

All macros are intended to be applied to struct definitions. Enum support is
outside the scope of this crate.

### Operations

**Delegation macros** — apply to newtype structs to generate transparent
access to the inner value. Two variants exist: one for immutable access and
one that additionally supports mutable access.

**Conversion macros** — apply to newtype structs to generate value conversions.
Two directions are covered: wrapping an inner value into the struct, and
extracting the inner value from the struct.

**Constructor macro** — applies to any struct with named or positional fields
to generate a constructor function accepting all field values as positional
arguments.

**Indexing macros** — apply to structs wrapping a collection to generate
indexed read access and optionally indexed write access.

**Reference conversion macros** — apply to newtype structs to generate shared
reference conversions and optionally exclusive reference conversions.

**Variadic conversion macro** — applies to structs to generate a family of
conversion implementations for different numbers of source values.

**Logical negation macro** — applies to structs wrapping a boolean-like value
to generate the logical negation operation.

**Phantom data macro** — applies to generic structs to generate phantom type
parameter handling.

### Error Handling

Macro expansion errors are reported as compile-time diagnostics at the call
site. Errors are produced when the macro is applied to an incompatible struct
shape (wrong field count, unsupported field type, etc.).

### Compatibility Guarantees

This crate is not intended for direct use. The compatibility contract is
provided by `derive_tools`, which re-exports these macros. Internal
implementation details are not stable.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| feature | docs/feature/001_derive_macros.md | Behavioral description of the macro collection |
| invariant | docs/invariant/001_proc_macro_separation.md | Why this crate does not provide a public API directly |
| integration | docs/integration/001_macro_tools.md | macro_tools dependency used for syntax parsing |
| integration | docs/integration/002_derive_tools.md | derive_tools as the stable public API layer |

### Sources

- [../../spec.md](../../spec.md) — Public API and Feature Flags sections
