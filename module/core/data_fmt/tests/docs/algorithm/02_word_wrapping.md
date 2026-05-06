# Algorithm Spec: Word Wrapping

## Source
`docs/algorithm/002_word_wrapping.md`

## Test Implementation
`tests/word_wrap.rs`

## Case Index

| ID | Name | Status |
|----|------|--------|
| AC-1 | text shorter than budget — no wrap | ✅ |
| AC-2 | text exceeds budget — break at word boundary | ✅ |
| AC-3 | tab characters removed when tab_width=0 | ✅ |
| AC-4 | single unsplittable word exceeds budget | ✅ |
| AC-5 | ANSI codes excluded from wrap-point calculation | ✅ |

---

### AC-1: text shorter than budget — no wrap

- **Given:** A flex column with a budget of 40 characters; the cell contains
  `"hello world"` (11 characters).
- **When:** The column is rendered with `auto_wrap=true`.
- **Then:** The cell occupies exactly one physical line; output is `"hello world"`
  with no inserted line breaks.

---

### AC-2: text exceeds budget — break at word boundary

- **Given:** A flex column with a budget of 20 characters; the cell contains
  `"the quick brown fox jumps"` (25 characters).
- **When:** The column is rendered with `auto_wrap=true`.
- **Then:** The text is split at the last word boundary that fits within 20
  characters; the first physical sub-line is `"the quick brown fox"` and the
  continuation sub-line contains the overflow word; no word is broken mid-character.

---

### AC-3: tab characters removed when tab_width=0

- **Given:** A cell containing `"hello\tworld"` with `WrapConfig` setting `tab_width(0)`.
- **When:** The cell is rendered.
- **Then:** The literal `\t` byte does not appear in output; `"hello"` and `"world"`
  are both present; `tab_width=0` means "expand tabs to zero spaces" (i.e. delete
  the tab character), not a no-op.
- **Note:** Covered by `expand_tabs_bug_zero_width_keeps_tab` (`bug_reproducer`
  issue-004a) in `tests/word_wrap.rs`.

---

### AC-4: single unsplittable word exceeds budget

- **Given:** A flex column with a budget of 10 characters; the cell contains a
  single word `"superlongword"` (13 characters, no spaces to break on).
- **When:** The column is rendered with `auto_wrap=true`.
- **Then:** The word is emitted without breaking (hard-break is not applied by
  default); no panic occurs; the column renders at its natural width for that cell.
- **Note:** Covered by `t19_break_long_words_false_overflow` in `tests/word_wrap.rs`
  (`width=5`, `BreakStrategy::Word`, `break_long_words(false)`, input `"averylongword"`).

---

### AC-5: ANSI codes excluded from wrap-point calculation

- **Given:** A cell containing `"\x1b[32mgreen text that is long enough to wrap\x1b[0m"`;
  the ANSI escape sequences add byte length but zero visual width.
- **When:** The cell is rendered in a flex column with a budget narrower than the
  visual text length.
- **Then:** The wrap point is determined using visual character width (ANSI bytes
  excluded); the ANSI sequences are preserved intact in the output; color is not
  broken by the wrap insertion.
- **Note:** Covered by `ansi_codes_excluded_from_wrap_point_calculation` in
  `tests/word_wrap.rs` (10 visual chars, 19 bytes, budget=15 → 1 line, ANSI intact).
  Implementation fix: `wrap_words` uses `unicode_visual_len(word)` instead of
  `char_count(word)` at `src/wrap.rs`.
