# ANSI Utilities

## Edge Case Index

| ID | Short Name | Category | Status |
|----|-----------|----------|--------|
| FT-1 | Detection: has escapes | Happy path | ✅ |
| FT-2 | Detection: no escapes | Happy path | ✅ |
| FT-3 | Parsing: yields segments | Happy path | ✅ |
| FT-4 | Stripping: removes escapes | Happy path | ✅ |
| FT-5 | Visual length: ANSI excluded | Happy path | ✅ |
| FT-6 | Truncation: exact width | Boundary | ✅ |
| FT-7 | Truncation: exceeds width | Happy path | ✅ |
| FT-8 | Truncation: preserves ANSI | Boundary | ✅ |

## Cases

### FT-1: Detection: has escapes

- **Given:** Input `"\x1b[31mred\x1b[0m"`
- **When:** `has_ansi()` is called
- **Then:** Returns `true`
- **Test:** `tests/inc/ansi_detect_test.rs` — `has_ansi_with_ansi`

### FT-2: Detection: no escapes

- **Given:** Input `"plain text"`
- **When:** `has_ansi()` is called
- **Then:** Returns `false`
- **Test:** `tests/inc/ansi_detect_test.rs` — `has_ansi_plain_text`

### FT-3: Parsing: yields segments

- **Given:** Input `"\x1b[1mbold\x1b[0m text"`
- **When:** `parse_segments()` is called
- **Then:** Yields alternating Escape and Text segments in order
- **Test:** `tests/inc/ansi_parse_test.rs` — `ansi_with_text`, `complex_formatting`

### FT-4: Stripping: removes escapes

- **Given:** Input `"\x1b[31mred\x1b[0m text"`
- **When:** `strip()` is called
- **Then:** Returns `"red text"` — all escape sequences removed
- **Test:** `tests/inc/ansi_strip_test.rs` — `strip_simple_colored_text`

### FT-5: Visual length: ANSI excluded

- **Given:** Input `"\x1b[31mhello\x1b[0m"`
- **When:** `visual_len()` is called
- **Then:** Returns 5 — counts visible characters only
- **Test:** `tests/inc/ansi_visual_test.rs` — `visual_len_ansi_with_text`

### FT-6: Truncation: exact width

- **Given:** String at exactly the width limit
- **When:** `truncate_if_needed()` is called
- **Then:** No truncation occurs — string returned unchanged

### FT-7: Truncation: exceeds width

- **Given:** String exceeding the width limit
- **When:** `truncate_if_needed()` is called
- **Then:** String is truncated and suffix marker appended

### FT-8: Truncation: preserves ANSI

- **Given:** ANSI-decorated string within width limit
- **When:** `truncate_if_needed()` is called
- **Then:** ANSI escapes preserved, visible width calculated correctly
