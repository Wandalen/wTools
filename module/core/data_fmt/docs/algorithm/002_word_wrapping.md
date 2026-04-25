# Algorithm: Word Wrapping

### Scope

- **Purpose**: Break text into lines that fit within a configured column width, respecting word boundaries, indent strings, overlong tokens, and hard line breaks.
- **Responsibility**: Documents the word wrapping algorithm including break strategies, overlong word handling, and overflow policies.
- **In Scope**: Hard, Word, and WordThenHard break strategies, indent model, overflow policy, and known fixes.
- **Out of Scope**: Table-level column width calculation (see `algorithm/001_multiline_cell_rendering.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/wrap.rs` | WrapFormatter implementation |
| test | `tests/word_wrap.rs` | Word wrapping test suite |
| doc | `../feature/002_word_wrap.md` | Feature-level word wrap documentation |

### Abstract

A text wrapping algorithm that breaks input strings into lines fitting within a configured width using configurable break strategies. Supports word-boundary breaking, hard character splitting, and a hybrid default mode that word-wraps first and hard-breaks only overlong tokens. Handles indentation, newline preservation, tab expansion, line limits, and overflow policies.

### Source Location

`src/wrap.rs` — `WrapFormatter::wrap()` (lines 213-240) dispatches to `wrap_words()` (lines 391-442) or `hard_break_str()` (lines 298-326). Overlong words handled by `push_overlong_word()` (lines 329-370).

### Pipeline

```
input text
  → expand_tabs (replace \t with spaces)
  → split on \n if preserve_newlines (produces segments)
  → for each segment:
      if Hard strategy → hard_break_str
      else → wrap_words (Word or WordThenHard)
  → enforce max_lines + overflow policy
  → output lines
```

### Break Strategies

#### Hard

Split text at exactly `width` characters per line, ignoring word boundaries.

```
while remaining is not empty:
  indent = indent_for(line_idx)
  avail = width - char_count(indent)
  take first `avail` chars → line
  trim leading spaces from remainder  (Fix issue-004b)
```

#### Word

Greedy word packing: accumulate words until adding the next word would exceed available width. When a single word exceeds the full available width, delegate to `push_overlong_word`.

#### WordThenHard (default)

Same as Word, but `push_overlong_word` hard-breaks overlong tokens character-by-character instead of emitting them as one oversized line.

### Algorithm

```
pending = []      // words accumulated for current line
pending_width = 0 // char count of pending words + spaces

for each word:
  avail = width - char_count(indent_for(current_line))
  if pending is empty:
    if word fits in avail → add to pending
    else → push_overlong_word(word)
  else:
    if pending_width + 1 + word_width <= avail → add to pending
    else:
      flush pending → output line
      recompute avail for new line
      if word fits → start new pending
      else → push_overlong_word(word)

flush any remaining pending
```

### Overlong Word Handling — `push_overlong_word`

When `break_strategy` is `WordThenHard` or `break_long_words` is true, the word is sliced character-by-character across multiple lines. Available width is recomputed per line (Fix issue-004c: `initial_indent` and `subsequent_indent` may differ).

```
while remaining is not empty:
  avail = width - char_count(indent_for(line_idx))
  take first `avail` chars → line
  remaining = rest
```

When neither condition holds, the word is emitted as-is on a single line (may exceed `width`).

### Indent Model

- **Line 0**: prepended with `initial_indent`
- **Lines 1+**: prepended with `subsequent_indent`
- Both indent strings reduce available width: `avail = width - char_count(indent)`
- `indent()` builder sets both to the same value

### Overflow Policy

Applied after all lines are produced, when `max_lines` is set:

| Policy | Behavior |
|--------|----------|
| `Truncate` | Drop lines beyond limit |
| `Ellipsis(s)` | Truncate last kept line content and append `s`, keeping total line width ≤ `width` |

### Known Fix Comments

Three fixes documented in source with `Fix(issue-004x)` format:

| Fix | Root Cause | Location |
|-----|------------|----------|
| `issue-004a` | `tab_width=0` returned raw `\t` instead of deleting it | `expand_tabs` |
| `issue-004b` | Leading space after chunk boundary caused `" worl"` instead of `"world"` | `hard_break_str` |
| `issue-004c` | Overlong word avail computed once, not per-line; continuation lines exceeded width | `push_overlong_word` |

### Complexity

- Time: O(n) where n is input length — each character visited at most twice (once for splitting, once for line assignment).
- Space: O(output_lines) — no copies beyond the output vector.
