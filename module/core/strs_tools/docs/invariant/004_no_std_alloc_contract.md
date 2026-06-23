# Invariant: No-Std Alloc Contract

### Scope

- **Purpose**: Guarantee that core functionality compiles and operates correctly in `no_std` environments that provide an allocator, without requiring the standard library.
- **Responsibility**: Defines the no_std compatibility invariant and identifies which features require allocator access versus which require the standard library.
- **In Scope**: Core splitting, isolation, indentation, and number parsing in `no_std + alloc`, features excluded from no_std targets, allocator requirement boundary.
- **Out of Scope**: Feature gating mechanism (`invariant/002`); SIMD fallback correctness (`invariant/003`).

### Invariant

Core string operations — splitting, isolation, indentation, and number parsing — are available in environments that disable the standard library but provide the `alloc` crate. These operations do not call any standard library I/O or threading primitives.

Features that depend on platform services unavailable in embedded targets — including ANSI terminal utilities and parser integration — are not guaranteed to compile in `no_std` environments. Callers targeting embedded platforms should not enable these features.

Operations that return owned strings require allocator access. Operations that return slices do not require an allocator.

### Sources

- [src/lib.rs](../../src/lib.rs) — Crate-level no_std configuration

### Features

- [001_string_splitting.md](../feature/001_string_splitting.md) — Split feature — core, no_std compatible
- [002_text_indentation.md](../feature/002_text_indentation.md) — Indentation feature — core, no_std compatible
- [003_string_isolation.md](../feature/003_string_isolation.md) — Isolation feature — core, no_std compatible
- [004_number_parsing.md](../feature/004_number_parsing.md) — Number parsing feature — core, no_std compatible

### Invariants

- [002_feature_gating_contract.md](../invariant/002_feature_gating_contract.md) — Feature activation mechanism
