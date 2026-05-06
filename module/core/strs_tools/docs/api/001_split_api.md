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

The count limit option, when set, caps the number of segments the iterator yields. The remainder of the source string is returned as a single final segment rather than split further.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/string/split.rs` | Split iterator, builder, and SplitType definition |
| source | `src/string/split/split_behavior.rs` | Behavioral flag types |
| doc | `docs/feature/001_string_splitting.md` | Split feature design and capability overview |
| doc | `docs/invariant/001_zero_copy_contract.md` | Zero-copy borrowing guarantee |
