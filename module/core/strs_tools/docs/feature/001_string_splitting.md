# Feature: String Splitting

### Scope

- **Purpose**: Provide advanced string splitting that handles quoting, escape sequences, delimiter preservation, and configurable token stripping beyond what the standard library offers.
- **Responsibility**: Documents the split capability: its configuration model, behavioral modes, and navigational links to all related artifacts.
- **In Scope**: Quote-aware splitting, multi-delimiter support, delimiter preservation, escape unescaping, strip whitespace, limit, and SplitType classification.
- **Out of Scope**: Algorithm internals (`algorithm/002`, `algorithm/003`); API operation signatures (`api/001`); zero-copy memory guarantee (`invariant/001`).

### Design

The split operation is configured through a builder: the caller supplies a source string, one or more delimiter patterns, and a set of behavioral flags. The builder executes lazily and yields an iterator of segments.

Each segment carries a classification: a delimited segment (content between delimiters) or a delimiter segment (the delimiter itself). This classification enables callers to preserve or discard delimiters and reconstruct the original string losslessly.

Quoting mode tracks opening and closing quote characters, treating any delimiters found inside a quoted region as literal content rather than split points. Escaping mode recognizes backslash sequences and treats an escaped quote as part of the content rather than a region boundary.

When stripping is enabled, whitespace is removed from the start and end of each delimited segment before it is yielded. When unescaping is enabled, escape sequences within content segments are resolved to their intended characters.

An optional count limit stops the iterator after a configured number of segments, leaving the remainder of the source as the final segment.

Specialized algorithm selection — single-char fast path, Boyer-Moore, SIMD — is automatic and transparent to the caller.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/string/split.rs` | Split iterator, builder, and SplitType definition |
| source | `src/string/split/split_behavior.rs` | Split configuration flags |
| source | `src/string/split/simd.rs` | SIMD-accelerated delimiter search path |
| source | `src/string/specialized.rs` | Single-char and Boyer-Moore split specializations |
| test | `tests/inc/split_test/basic_split_tests.rs` | Basic splitting behavior |
| test | `tests/inc/split_test/quoting_options_tests.rs` | Quoting and escape handling |
| test | `tests/inc/split_test/preserving_options_tests.rs` | Delimiter and quoting preservation |
| test | `tests/inc/split_test/stripping_options_tests.rs` | Whitespace stripping behavior |
| test | `tests/inc/split_test/edge_case_tests.rs` | Empty input, consecutive delimiters, limits |
| test | `tests/strs_tools_tests.rs` | Main test entry point |
| doc | `docs/api/001_split_api.md` | Public split API operations and compatibility contract |
| doc | `docs/invariant/001_zero_copy_contract.md` | Zero-copy borrowing guarantee |
| doc | `docs/algorithm/002_single_char_splitting.md` | Single-char fast path |
| doc | `docs/algorithm/003_boyer_moore_splitting.md` | Boyer-Moore multi-char path |
| doc | `docs/invariant/004_no_std_alloc_contract.md` | No-std compatibility guarantee for core operations |
| task | `task/completed/007_specialized_algorithms.md` | Specialized algorithm implementation task |
