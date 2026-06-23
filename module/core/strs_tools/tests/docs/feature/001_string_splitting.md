# String Splitting

## Edge Case Index

| ID | Short Name | Category | Status |
|----|-----------|----------|--------|
| FT-1 | Basic comma split | Happy path | ✅ |
| FT-2 | Quote-aware split | Happy path | ✅ |
| FT-3 | Delimiter preservation | Boundary | ✅ |
| FT-4 | Escape unescaping | Happy path | ✅ |
| FT-5 | Count limit stops iterator | Boundary | ⛔ blocked: API not implemented |
| FT-6 | Empty input | Boundary | ✅ |
| FT-7 | Consecutive delimiters | Boundary | ✅ |
| FT-8 | Algorithm selection transparency | Invariant | ✅ |

## Cases

### FT-1: Basic comma split

- **Given:** Input `"a,b,c"` with delimiter `","`
- **When:** Split is performed with default options
- **Then:** Returns 3 segments: `["a", "b", "c"]`

### FT-2: Quote-aware split

- **Given:** Input `'a,"b,c",d'` with delimiter `","` and quoting enabled
- **When:** Split is performed
- **Then:** Returns 3 segments: `["a", "b,c", "d"]` — comma inside quotes is not a split point

### FT-3: Delimiter preservation

- **Given:** Input `"a;b;c"` with delimiter `";"` and preserve_delimiters enabled
- **When:** Split is performed
- **Then:** Returns 5 segments alternating content and delimiter: `["a", ";", "b", ";", "c"]`

### FT-4: Escape unescaping

- **Given:** Input with escaped quotes `'a "b\"c" d'` and unescaping enabled
- **When:** Split is performed
- **Then:** Escaped quotes resolve to literal quotes in segment content

### FT-5: Count limit stops iterator

- **Given:** Input `"a,b,c,d"` with delimiter `","` and count limit `2`
- **When:** Split is performed
- **Then:** Returns 2 segments: first segment plus remainder as final segment
- **Blocked:** `BasicSplitBuilder` has no `limit`, `max_splits`, or `count` field — count-limit is documented but not implemented

### FT-6: Empty input

- **Given:** Empty string `""`
- **When:** Split is performed with any delimiter
- **Then:** Returns single empty segment or empty iterator

### FT-7: Consecutive delimiters

- **Given:** Input `"a,,b"` with delimiter `","`
- **When:** Split is performed with preserve_empty enabled
- **Then:** Returns segments including the empty segment between consecutive delimiters

### FT-8: Algorithm selection transparency

- **Given:** Same input split with single-char delimiter and with multi-char delimiter
- **When:** Both splits complete
- **Then:** Segment content is identical regardless of which algorithm path was selected
- **Test:** `tests/inc/split_test.rs` — covered via single-char and Boyer-Moore algorithm specs (AC-3 in each)
