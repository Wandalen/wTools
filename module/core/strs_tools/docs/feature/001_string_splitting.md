# Feature: String Splitting

### Scope

- **Purpose**: Provide advanced string splitting that handles quoting, escape sequences, delimiter preservation, and configurable token stripping beyond what the standard library offers.
- **Responsibility**: Documents the split capability: its configuration model, behavioral modes, and navigational links to all related artifacts.
- **In Scope**: Quote-aware splitting, multi-delimiter support, delimiter preservation, escape unescaping, strip whitespace, and SplitType classification. Count limit is a planned extension not yet implemented.
- **Out of Scope**: Algorithm internals (`algorithm/002`, `algorithm/003`); API operation signatures (`api/001`); zero-copy memory guarantee (`invariant/001`).

### Design

The split operation is configured through a builder: the caller supplies a source string, one or more delimiter patterns, and a set of behavioral flags. The builder executes lazily and yields an iterator of segments.

Each segment carries a classification: a delimited segment (content between delimiters) or a delimiter segment (the delimiter itself). This classification enables callers to preserve or discard delimiters and reconstruct the original string losslessly.

Quoting mode tracks opening and closing quote characters, treating any delimiters found inside a quoted region as literal content rather than split points. Escaping mode recognizes backslash sequences and treats an escaped quote as part of the content rather than a region boundary.

When stripping is enabled, whitespace is removed from the start and end of each delimited segment before it is yielded. When unescaping is enabled, escape sequences within content segments are resolved to their intended characters.

A count limit that stops the iterator after a configured number of segments is a planned extension not yet implemented in the builder API.

Specialized algorithm selection — single-char fast path, Boyer-Moore, SIMD — is automatic and transparent to the caller.

### Sources

- [src/string/split/mod.rs](../../src/string/split/mod.rs) — Split iterator, builder, and SplitType definition
- [src/string/split/split_behavior.rs](../../src/string/split/split_behavior.rs) — Split configuration flags
- [src/string/split/simd.rs](../../src/string/split/simd.rs) — SIMD-accelerated delimiter search path
- [src/string/specialized/mod.rs](../../src/string/specialized/mod.rs) — Single-char and Boyer-Moore split specializations

### Tests

- [tests/inc/split_test/basic_split_tests.rs](../../tests/inc/split_test/basic_split_tests.rs) — Basic splitting behavior
- [tests/inc/split_test/quoting_options_tests.rs](../../tests/inc/split_test/quoting_options_tests.rs) — Quoting and escape handling
- [tests/inc/split_test/preserving_options_tests.rs](../../tests/inc/split_test/preserving_options_tests.rs) — Delimiter and quoting preservation
- [tests/inc/split_test/stripping_options_tests.rs](../../tests/inc/split_test/stripping_options_tests.rs) — Whitespace stripping behavior
- [tests/inc/split_test/edge_case_tests.rs](../../tests/inc/split_test/edge_case_tests.rs) — Empty input and empty delimiter edge cases
- [tests/strs_tools_tests.rs](../../tests/strs_tools_tests.rs) — Main test entry point

### APIs

- [001_split_api.md](../api/001_split_api.md) — Public split API operations and compatibility contract

### Invariants

- [001_zero_copy_contract.md](../invariant/001_zero_copy_contract.md) — Zero-copy borrowing guarantee
- [004_no_std_alloc_contract.md](../invariant/004_no_std_alloc_contract.md) — No-std compatibility guarantee for core operations

### Algorithms

- [002_single_char_splitting.md](../algorithm/002_single_char_splitting.md) — Single-char fast path
- [003_boyer_moore_splitting.md](../algorithm/003_boyer_moore_splitting.md) — Boyer-Moore multi-char path

### Tasks

- [007_specialized_algorithms.md](../../task/completed/007_specialized_algorithms.md) — Specialized algorithm implementation task
