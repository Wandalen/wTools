# Test Surface: Algorithm — Single-Char Splitting

### Source

- **Doc Instance:** [algorithm/002_single_char_splitting.md](../../../docs/algorithm/002_single_char_splitting.md)

### Cases

| # | Status | Case |
|---|--------|------|
| AC-1 | ✅ | Single-byte delimiter selects fast path |
| AC-2 | ✅ | Dense delimiters produce many short segments |
| AC-3 | ✅ | Results identical to general path |
| AC-4 | ✅ | Delimiter preservation emits delimiter segments |

### AC-1 — Single-byte delimiter selects fast path

- **Given:** A split configuration with exactly one single-byte delimiter
- **When:** The iterator is constructed
- **Then:** The single-char fast path is selected over the general machinery
- **Test:** `tests/inc/split_test.rs`

### AC-2 — Dense delimiters produce many short segments

- **Given:** A source `"a,b,c,d,e"` with delimiter `,`
- **When:** Split is executed via the single-char path
- **Then:** Five single-character segments are produced efficiently
- **Test:** `tests/inc/split_test.rs`

### AC-3 — Results identical to general path

- **Given:** The same input processed by single-char path and general path
- **When:** Segments are collected from both
- **Then:** The segment sequences are identical (transparent optimization)
- **Test:** `tests/inc/split_test.rs`

### AC-4 — Delimiter preservation emits delimiter segments

- **Given:** A source with a single-byte delimiter and delimiter preservation enabled
- **When:** Split is executed
- **Then:** Delimiter segments appear between content segments in the output
- **Test:** `tests/inc/split_test.rs`
