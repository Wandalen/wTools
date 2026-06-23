# Test Surface: Algorithm — SIMD Delimiter Search

### Source

- **Doc Instance:** [algorithm/001_simd_delimiter_search.md](../../../docs/algorithm/001_simd_delimiter_search.md)

### Cases

| # | Status | Case |
|---|--------|------|
| AC-1 | ✅ | Single-byte delimiter uses byte-search SIMD path |
| AC-2 | ✅ | Multiple delimiters compile Aho-Corasick automaton |
| AC-3 | ✅ | Byte offsets match scalar-computed positions |
| AC-4 | ✅ | Automaton caching reuses compiled state |

### AC-1 — Single-byte delimiter uses byte-search SIMD path

- **Given:** A source string and a single single-byte delimiter with `simd` feature enabled
- **When:** The split iterator is constructed and begins scanning
- **Then:** The byte-search SIMD path is selected (scanning multiple bytes per cycle)
- **Test:** `tests/simd_tests.rs` — `simd_scalar_equivalence_single_delimiter`

### AC-2 — Multiple delimiters compile Aho-Corasick automaton

- **Given:** A source string and a delimiter set with 3 or more patterns with `simd` feature enabled
- **When:** The split iterator is constructed
- **Then:** An Aho-Corasick automaton is compiled from the delimiter set
- **Test:** `tests/simd_tests.rs` — `simd_scalar_equivalence_multi_delimiter`

### AC-3 — Byte offsets match scalar-computed positions

- **Given:** A complex input with mixed Unicode and ASCII content
- **When:** The SIMD search produces byte offsets for each delimiter match
- **Then:** Every offset matches the position found by the scalar comparison loop
- **Test:** `tests/simd_tests.rs` — `simd_scalar_equivalence_single_delimiter`, `simd_scalar_equivalence_multi_delimiter`

### AC-4 — Automaton caching reuses compiled state

- **Given:** The same delimiter set used for two successive split invocations
- **When:** The second invocation begins
- **Then:** The compiled automaton is reused, not rebuilt
- **Test:** `tests/simd_tests.rs` — `simd_pattern_caching_reuse`
