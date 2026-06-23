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
| FT-9 | Visual width: pure ASCII | Happy path | ✅ |
| FT-10 | Visual width: emoji (2-column) | Happy path | ✅ |
| FT-11 | Visual width: CJK (2-column) | Happy path | ✅ |
| FT-12 | Visual width: ANSI stripped | Happy path | ✅ |
| FT-13 | Visual width: empty string | Boundary | ✅ |
| FT-14 | Visual width: mixed ASCII+emoji | Happy path | ✅ |
| FT-15 | Visual width: combining accent (unicode) | Boundary | ✅ |
| FT-16 | Visual width: ANSI+emoji+text | Boundary | ✅ |

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

- **Given:** Input `"\x1b[31mred\x1b[0m"`
- **When:** `visual_len()` is called
- **Then:** Returns 3 — counts visible characters only
- **Test:** `tests/inc/ansi_visual_test.rs` — `visual_len_ansi_with_text`

### FT-6: Truncation: exact width

- **Given:** String at exactly the width limit
- **When:** `truncate()` is called
- **Then:** No truncation occurs — string returned unchanged
- **Test:** `tests/inc/ansi_truncate_test.rs` — `truncate_no_change_when_fits`

### FT-7: Truncation: exceeds width

- **Given:** String exceeding the width limit
- **When:** `truncate()` is called
- **Then:** String is truncated and suffix marker appended
- **Test:** `tests/inc/ansi_truncate_test.rs` — `truncate_plain_text`, `truncate_with_suffix`

### FT-8: Truncation: preserves ANSI

- **Given:** ANSI-decorated string within width limit
- **When:** `truncate()` is called
- **Then:** ANSI escapes preserved, visible width calculated correctly
- **Test:** `tests/inc/ansi_truncate_test.rs` — `truncate_preserves_ansi`, `truncate_with_reset`

### FT-9: Visual width: pure ASCII

- **Given:** Input `"hello"` (pure ASCII)
- **When:** `visual_width()` is called
- **Then:** Returns 5
- **Test:** `tests/inc/ansi_visual_test.rs` — `visual_width_pure_ascii`

### FT-10: Visual width: emoji (2-column)

- **Given:** Input `"😀😀"` (two emoji)
- **When:** `visual_width()` is called
- **Then:** Returns 4 (each emoji = 2 display columns)
- **Test:** `tests/inc/ansi_visual_test.rs` — `visual_width_emoji`

### FT-11: Visual width: CJK (2-column)

- **Given:** Input `"你好"` (CJK characters)
- **When:** `visual_width()` is called
- **Then:** Returns 4 (each CJK = 2 display columns)
- **Test:** `tests/inc/ansi_visual_test.rs` — `visual_width_cjk`

### FT-12: Visual width: ANSI stripped

- **Given:** Input `"\x1b[31mred\x1b[0m"` (ANSI-wrapped)
- **When:** `visual_width()` is called
- **Then:** Returns 3 (ANSI stripped, ASCII chars = 1 column each)
- **Test:** `tests/inc/ansi_visual_test.rs` — `visual_width_ansi_stripped`

### FT-13: Visual width: empty string

- **Given:** Input `""` (empty)
- **When:** `visual_width()` is called
- **Then:** Returns 0
- **Test:** `tests/inc/ansi_visual_test.rs` — `visual_width_empty`

### FT-14: Visual width: mixed ASCII+emoji

- **Given:** Input `"a😀b"` (mixed ASCII + emoji)
- **When:** `visual_width()` is called
- **Then:** Returns 4 (1+2+1)
- **Test:** `tests/inc/ansi_visual_test.rs` — `visual_width_mixed_ascii_emoji`

### FT-15: Visual width: combining accent (unicode)

- **Given:** Input `"e\u{0301}"` (e + combining accent)
- **When:** `visual_width_unicode()` is called
- **Then:** Returns 1 (single grapheme, 1 display column)
- **Test:** `tests/inc/ansi_visual_test.rs` — `visual_width_unicode_combining_accent`

### FT-16: Visual width: ANSI+emoji+text

- **Given:** Input `"\x1b[1m😀\x1b[0m text"` (ANSI + emoji + space + text)
- **When:** `visual_width()` is called
- **Then:** Returns 7 (2+1+4)
- **Test:** `tests/inc/ansi_visual_test.rs` — `visual_width_ansi_emoji_text`
