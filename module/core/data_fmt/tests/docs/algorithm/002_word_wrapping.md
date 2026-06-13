# Algorithm: Word Wrapping

### Scope

- **Purpose**: Drive test coverage for the word wrapping algorithm.
- **Responsibility**: Documents test cases for the word wrapping algorithm in `docs/algorithm/002_word_wrapping.md`.
- **In Scope**: Break strategies (Word, Hard, WordThenHard), overflow policies (Truncate, Ellipsis), tab expansion, ANSI exclusion from wrap-point calculation, preserve_newlines semantics.
- **Out of Scope**: Budget allocation (see `algorithm/004`); feature-level wrap configuration (see `feature/002`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| AC-1 | text shorter than budget — no wrap | ✅ |
| AC-2 | text exceeds budget — break at word boundary | ✅ |
| AC-3 | tab characters removed when tab_width=0 | ✅ |
| AC-4 | single unsplittable word exceeds budget | ✅ |
| AC-5 | ANSI codes excluded from wrap-point calculation | ✅ |
| AC-6 | BreakStrategy::Hard splits at exact character boundary | ✅ |
| AC-7 | overflow policy Truncate drops lines beyond max_lines | ✅ |
| AC-8 | overflow policy Ellipsis appends indicator to last kept line | ✅ |
| AC-9 | preserve_newlines creates independent wrapping segments per input line | ✅ |
| AC-10 | tab_width > 0 expands tab to N spaces | ✅ |
| AC-11 | WordThenHard falls through to hard-break when word exceeds budget | ✅ |
| AC-12 | no leading space on continuation line after hard break (BUG-002) | ✅ |

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
  BUG-002) in `tests/word_wrap.rs`.

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

---

### AC-6: BreakStrategy::Hard splits at exact character boundary

- **Given:** A `WrapFormatter` configured with `break_strategy(BreakStrategy::Hard)`
  and `width(7)`; input text `"hello world"`.
- **When:** `wrap()` is called.
- **Then:** Line 1 is `"hello w"` (exactly 7 chars) and line 2 is `"orld"`;
  the split occurs at exactly 7 visible characters regardless of word boundaries;
  no leading space appears at the start of line 2 (Fix BUG-002).

---

### AC-7: overflow policy Truncate drops lines beyond max_lines

- **Given:** A `WrapFormatter` with `width(10)`, `max_lines(2)`, and
  `overflow(OverflowPolicy::Truncate)`; input text that produces 4 natural lines.
- **When:** `wrap()` is called.
- **Then:** Exactly 2 lines are returned; lines 3 and 4 are silently discarded;
  the returned lines contain no truncation indicator and no trailing ellipsis.

---

### AC-8: overflow policy Ellipsis appends indicator to last kept line

- **Given:** A `WrapFormatter` with `width(12)`, `max_lines(2)`, and
  `overflow(OverflowPolicy::Ellipsis("..."))`;  input text that produces 4 natural
  lines.
- **When:** `wrap()` is called.
- **Then:** Exactly 2 lines are returned; the second line ends with `"..."`; the
  total visual width of the second line (content + ellipsis) does not exceed
  `width(12)`.

---

### AC-9: preserve_newlines creates independent wrapping segments per input line

- **Given:** A `WrapFormatter` with `width(10)` and `preserve_newlines(true)`;
  input `"short\na much longer line here that must wrap"`.
- **When:** `wrap()` is called.
- **Then:** `"short"` forms its own segment with no trailing words from the second
  line; the second segment wraps independently within 10 characters; the two
  segments appear in order with the original line-break boundary preserved.

---

### AC-10: tab_width > 0 expands tab to N spaces

- **Given:** A cell containing `"a\tb"` with `WrapConfig` setting `tab_width(4)`.
- **When:** `WrapFormatter::wrap()` is called.
- **Then:** The `\t` byte does not appear in the output (it is expanded to spaces
  before word splitting); `wrap_segment()` then calls `split_whitespace()` which
  collapses all whitespace runs to single-space boundaries; the resulting output is
  `"a b"` (single space) — not `"a    b"`; the literal tab is removed and both
  words are present, but the expanded spaces are normalised by the word-split pass.
- **Note:** The tab IS expanded (no literal `\t` survives), but the
  `split_whitespace()`-based word algorithm collapses the resulting space run to a
  single separator. Tests guard against tab preservation (no literal `\t`) and
  against the exact normalised form (`"a b"`).

---

### AC-11: WordThenHard falls through to hard-break when word exceeds budget

- **Given:** A `WrapFormatter` with `break_strategy(BreakStrategy::WordThenHard)`
  and `width(5)`; input containing a single token `"abcdefgh"` (8 chars, exceeds budget).
- **When:** `wrap()` is called.
- **Then:** The token is hard-broken at character 5; line 1 is `"abcde"` and line 2
  is `"fgh"`; the WordThenHard strategy falls through to character-level splitting
  when no word boundary exists within the budget.

---

### AC-12: no leading space on continuation line after hard break (BUG-002)

- **Given:** A `WrapFormatter` with `break_strategy(BreakStrategy::Hard)` and
  `width(7)`; input `"hello world"` (space at position 5).
- **When:** `wrap()` is called.
- **Then:** Line 2 begins with `"orld"` not `" orld"`; no leading space is inserted
  at the start of a continuation line when the break falls mid-word after a space.
- **Note:** Regression guard for BUG-002; the bug was a leading-space artifact
  from the split-at-boundary implementation.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/algorithm/002_word_wrapping.md`](../../../docs/algorithm/002_word_wrapping.md) | Source algorithm spec — break strategies, overflow policies, tab expansion |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/word_wrap.rs`](../../word_wrap.rs) | Algorithm test implementation |
