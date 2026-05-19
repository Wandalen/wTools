# Feature: Word Wrap

### Scope

- **Purpose**: Drive test coverage for the word wrap feature.
- **Responsibility**: Documents test cases for the word wrap configuration feature in `docs/feature/002_word_wrap.md`.
- **In Scope**: Break strategies (WordThenHard, Hard, Word), max_lines with Truncate and Ellipsis overflow policies, preserve_newlines, initial/subsequent indent, break_long_words flag.
- **Out of Scope**: Word wrapping algorithm internals (see `algorithm/002`); auto-wrap budget allocation (see `feature/005`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| FT-1 | WordThenHard strategy wraps at word boundary | ✅ |
| FT-2 | Hard strategy splits at exactly width characters | ✅ |
| FT-3 | max_lines with Truncate discards excess lines | ✅ |
| FT-4 | max_lines with Ellipsis appends marker to last kept line | ✅ |
| FT-5 | preserve_newlines=true treats embedded newlines as hard breaks | ✅ |
| FT-6 | initial_indent and subsequent_indent prepended correctly | ✅ |
| FT-7 | BreakStrategy::Word wraps at word boundary only (no hard fallthrough) | ✅ |
| FT-8 | break_long_words=true enables hard-break for words exceeding budget | ✅ |

---

### FT-1: WordThenHard strategy wraps at word boundary

- **Given:** A `WrapFormatter` with `width(20)` and `break_strategy(WordThenHard)` (default).
- **When:** `wrap("hello world how are you doing today")` is called.
- **Then:** Lines are broken at space characters; no line exceeds 20 characters; no word is split mid-character unless a single token exceeds 20 characters.

---

### FT-2: Hard strategy splits at exactly width characters

- **Given:** A `WrapFormatter` with `width(10)` and `break_strategy(Hard)`.
- **When:** `wrap("abcdefghijklmnop")` is called.
- **Then:** The output is `["abcdefghij", "klmnop"]`; the break occurs at exactly character 10 regardless of word boundaries.

---

### FT-3: max_lines with Truncate discards excess lines

- **Given:** A `WrapFormatter` with `width(10)`, `max_lines(Some(2))`, and `overflow(Truncate)`.
- **When:** `wrap("one two three four five six")` produces more than 2 lines.
- **Then:** The output contains exactly 2 lines; lines beyond index 1 are discarded; no truncation marker is appended.

---

### FT-4: max_lines with Ellipsis appends marker to last kept line

- **Given:** A `WrapFormatter` with `width(20)`, `max_lines(Some(2))`, and `overflow(Ellipsis("...".to_string()))`.
- **When:** `wrap(long_text)` produces more than 2 lines.
- **Then:** The output contains exactly 2 lines; line 1 ends with `"..."`; the total length of line 1 does not exceed 20 characters.

---

### FT-5: preserve_newlines=true treats embedded newlines as hard breaks

- **Given:** A `WrapFormatter` with `preserve_newlines(true)` (default) and `width(80)`.
- **When:** `wrap("first line\nsecond line")` is called.
- **Then:** The output contains exactly 2 lines: `"first line"` and `"second line"`; the `\n` in the input acts as a hard break.

---

### FT-6: initial_indent and subsequent_indent prepended correctly

- **Given:** A `WrapFormatter` with `width(20)`, `initial_indent("  ")`, and `subsequent_indent("    ")`.
- **When:** `wrap("hello world how are you doing today")` is called.
- **Then:** Line 0 is prefixed with 2 spaces; lines 1+ are prefixed with 4 spaces; both indent strings count toward the 20-character width limit.

---

### FT-7: BreakStrategy::Word wraps at word boundary only (no hard fallthrough)

- **Given:** A `WrapFormatter` with `break_strategy(BreakStrategy::Word)` and
  `width(5)`; input containing a single token `"abcdefgh"` (8 chars, exceeds budget)
  with no surrounding spaces.
- **When:** `wrap("abcdefgh")` is called.
- **Then:** The token is emitted as-is without hard-breaking (Word strategy does not
  fall through to character splitting); the output is `["abcdefgh"]` — one line
  overflowing the budget; no panic occurs.

---

### FT-8: break_long_words=true enables hard-break for words exceeding budget

- **Given:** A `WrapFormatter` with `break_long_words(true)` and `width(5)`; input
  `"abcdefghij"` (10 chars, no spaces).
- **When:** `wrap("abcdefghij")` is called.
- **Then:** The word is hard-broken at the width boundary; output is `["abcde", "fghij"]`
  (or similar split at 5); `break_long_words=true` forces character-level splitting
  for tokens that exceed the budget regardless of strategy.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/feature/002_word_wrap.md`](../../../docs/feature/002_word_wrap.md) | Source feature spec — break strategies, overflow policies, indent |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/word_wrap.rs`](../../word_wrap.rs) | Feature test implementation |
