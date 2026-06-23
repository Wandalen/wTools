# Test Surface: Algorithm — Boyer-Moore Splitting

### Source

- **Doc Instance:** [algorithm/003_boyer_moore_splitting.md](../../../docs/algorithm/003_boyer_moore_splitting.md)

### Cases

| # | Status | Case |
|---|--------|------|
| AC-1 | ✅ | Multi-byte delimiter selects Boyer-Moore path |
| AC-2 | ✅ | Skip table skips non-matching bytes |
| AC-3 | ✅ | Right-to-left comparison detects full match |
| AC-4 | ✅ | Results identical to naive scan |

### AC-1 — Multi-byte delimiter selects Boyer-Moore path

- **Given:** A split configuration with exactly one delimiter longer than one byte (e.g., `":::"`)
- **When:** The iterator is constructed
- **Then:** The Boyer-Moore inspired path is selected
- **Test:** `tests/inc/split_test.rs`

### AC-2 — Skip table skips non-matching bytes

- **Given:** A long source with a rare 4-byte delimiter
- **When:** The Boyer-Moore scan encounters bytes not in the delimiter
- **Then:** The scan position advances by the full delimiter length (skip table lookup)
- **Test:** `tests/inc/split_test.rs`

### AC-3 — Right-to-left comparison detects full match

- **Given:** A source containing the delimiter `"<>"` at known positions
- **When:** The algorithm compares right-to-left at each candidate position
- **Then:** Full matches are detected and segment boundaries are correctly closed
- **Test:** `tests/inc/split_test.rs`

### AC-4 — Results identical to naive scan

- **Given:** The same input processed by Boyer-Moore path and a naive scan
- **When:** Segments are collected from both
- **Then:** The segment sequences are identical
- **Test:** `tests/inc/split_test.rs`
