# Algorithm: Boyer-Moore Splitting

### Scope

- **Purpose**: Document the skip-table split algorithm used for multi-character single-delimiter patterns, which reduces the number of comparisons versus a naive scan.
- **Responsibility**: Explains the Boyer-Moore inspired approach, skip table construction, selection criteria, and throughput characteristics.
- **In Scope**: Skip table construction from delimiter bytes, right-to-left pattern matching, mismatch skip calculation, segment emission.
- **Out of Scope**: Single-byte fast path (`algorithm/002`); multi-delimiter SIMD search (`algorithm/001`); public API (`api/001`).

### Design

When the split configuration contains exactly one delimiter that is longer than one byte, the iterator selects the Boyer-Moore inspired path. Selection occurs once at iterator construction time based on delimiter byte length.

A skip table is constructed from the delimiter bytes before scanning begins. The table maps each possible byte value to the number of source bytes that can be safely skipped when that byte is encountered at a mismatch position. Bytes that do not appear in the delimiter map to the full delimiter length, allowing long jumps over unrelated content.

Scanning proceeds from left to right in the source. At each candidate position the delimiter is compared right-to-left against the source. On a full match, the segment boundary is closed and the delimiter segment is emitted if preservation is enabled. On a mismatch the skip table is consulted to advance the scan position by more than one byte.

For long delimiters in inputs where the delimiter is rare, this algorithm reduces the number of byte comparisons substantially compared with a naive left-to-right scan. For short delimiters or dense inputs the benefit is smaller, and the single-char path or SIMD path may be preferable — selection logic accounts for this.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/string/specialized.rs` | Boyer-Moore split specialization and skip table |
| source | `src/string/split.rs` | Fast-path selection at iterator construction |
| doc | `docs/feature/001_string_splitting.md` | Split feature overview including algorithm selection |
| doc | `docs/algorithm/002_single_char_splitting.md` | Single-char fast path counterpart |
