# API: Split API

### Scope

- **Purpose**: Define the public operations, builder steps, and return types for string splitting and segment iteration.
- **Responsibility**: Contracts the observable behaviour callers depend on for the split feature.
- **In Scope**: Split builder entry point, delimiter and flag setters, iterator output, SplitType variants, segment lifetime relationship to source.
- **Out of Scope**: Algorithm selection internals (`algorithm/002`, `algorithm/003`); SIMD dispatch (`feature/007`); split configuration flags semantics (`feature/001`).

### Operations

The split API exposes a builder entry point that accepts a string source and produces a configured iterator.

The caller supplies a source string slice, one or more delimiter patterns, and any combination of behavioral flags. The builder validates the configuration on `perform` and returns an iterator over typed segments.

Each segment yielded by the iterator is classified as either a delimited segment or a delimiter segment. Delimited segments carry string content; delimiter segments carry the delimiter text as matched. Both variants borrow from the original source where possible, satisfying the zero-copy invariant.

A count limit option that caps the number of segments the iterator yields is a planned extension not yet exposed in the builder API.

### Sources

- [src/string/split/mod.rs](../../src/string/split/mod.rs) — Split iterator, builder, and SplitType definition
- [src/string/split/split_behavior.rs](../../src/string/split/split_behavior.rs) — Behavioral flag types

### Features

- [001_string_splitting.md](../feature/001_string_splitting.md) — Split feature design and capability overview

### Invariants

- [001_zero_copy_contract.md](../invariant/001_zero_copy_contract.md) — Zero-copy borrowing guarantee
