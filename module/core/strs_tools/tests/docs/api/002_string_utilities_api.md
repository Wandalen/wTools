# String Utilities API

## Edge Case Index

| ID | Short Name | Category | Status |
|----|-----------|----------|--------|
| AP-1 | Indentation returns owned string | Happy path | ✅ |
| AP-2 | Isolation returns optional slice | Happy path | ✅ |
| AP-3 | Number parsing returns result | Happy path | ✅ |
| AP-4 | ANSI detection returns boolean | Happy path | ✅ |
| AP-5 | ANSI stripping returns owned string | Happy path | ✅ |
| AP-6 | Visual width returns display columns | Happy path | ✅ |
| AP-7 | Visual width unicode returns display columns | Happy path | ✅ |

## Cases

### AP-1: Indentation returns owned string

- **Given:** Source string, prefix, and postfix
- **When:** Indentation operation is called
- **Then:** Returns an owned String with each line wrapped

### AP-2: Isolation returns optional slice

- **Given:** Source string and delimiter pattern
- **When:** Left isolation is called
- **Then:** Returns `Option<&str>` — `Some` if delimiter found, `None` otherwise

### AP-3: Number parsing returns result

- **Given:** String slice and target numeric type
- **When:** Number parsing is called
- **Then:** Returns typed `Result` — `Ok(value)` or `Err(error)`

### AP-4: ANSI detection returns boolean

- **Given:** Any string
- **When:** `has_ansi()` is called
- **Then:** Returns `bool` — `true` if ANSI escape sequences present
- **Test:** `tests/inc/ansi_detect_test.rs` — `has_ansi_with_ansi`, `has_ansi_plain_text`

### AP-5: ANSI stripping returns owned string

- **Given:** String with ANSI escape sequences
- **When:** `strip()` is called
- **Then:** Returns owned String with all escape sequences removed
- **Test:** `tests/inc/ansi_strip_test.rs` — `strip_simple_colored_text`, `strip_complex_formatting`

### AP-6: Visual width returns display columns

- **Given:** String potentially containing ANSI escape sequences and wide characters
- **When:** `visual_width()` is called
- **Then:** Returns `usize` — number of terminal display columns (wide chars = 2, combiners = 0, ANSI = 0)
- **Test:** `tests/inc/ansi_visual_test.rs` — `visual_width_pure_ascii`, `visual_width_emoji`, `visual_width_cjk`, `visual_width_ansi_stripped`, `visual_width_empty`, `visual_width_mixed_ascii_emoji`, `visual_width_ansi_emoji_text`

### AP-7: Visual width unicode returns display columns

- **Given:** String potentially containing ANSI escape sequences, wide characters, and grapheme clusters
- **When:** `visual_width_unicode()` is called (requires `ansi_unicode` feature)
- **Then:** Returns `usize` — display columns using grapheme-cluster boundaries for accurate combining-mark handling
- **Test:** `tests/inc/ansi_visual_test.rs` — `visual_width_unicode_combining_accent`
