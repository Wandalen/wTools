# Feature Test: Output Processing

### Scope

- **Purpose**: Verify the behavioral requirements documented in `docs/feature/001_output_processing.md`.
- **Responsibility**: Test spec for stream ordering, head/tail line filtering, width truncation, width=0 disable, truncation suffix, ANSI width exclusion, StreamFilter variants, and lines_omitted accuracy.
- **In Scope**: FT-1..FT-40 — stream selection ordering, head/tail line filtering, width truncation, ANSI code handling, StreamFilter variants, metadata accuracy, boundary conditions (head/tail/width zero, exact, overlap), combined configuration scenarios, stream-filter+head interaction, head+tail+width triple combination, empty width suffix, empty-stdout head filtering, and width=0+head interaction.
- **Out of Scope**: strs_tools internals; architectural boundary enforcement — see `tests/docs/invariant/001_architectural_boundary.md`.

### FT-1: Stderr precedes stdout in Both mode

- **Given:** stdout content `"out"`, stderr content `"err"`, `StreamFilter::Both` (default)
- **When:** `process_output("out", "err", &OutputConfig::default())`
- **Then:** `result.content == "err\nout"` — stderr placed before stdout, joined by newline

### FT-2: head() function retains first N lines

- **Given:** five-line text `"line1\nline2\nline3\nline4\nline5"`, head limit `2`
- **When:** `head(text, 2)` (strs_tools utility called directly, not process_output)
- **Then:** result == `"line1\nline2"` — only the first two lines retained

### FT-3: Line well below max_width is not truncated

- **Given:** single-line input `"short line"` (10 visible chars), `OutputConfig::default().with_width(50)`
- **When:** `process_output("short line", "", &config)`
- **Then:** `result.width_truncated == false`; content begins with `"short line"` intact

### FT-4: ANSI escape codes are excluded from visible width measurement

- **Given:** input `"\x1b[31mred text\x1b[0m"` (8 visible chars), `OutputConfig::default().with_width(50)`
- **When:** `process_output("\x1b[31mred text\x1b[0m", "", &config)`
- **Then:** ANSI codes preserved — content starts with `"\x1b[31mred text\x1b[0m"`; no truncation occurs

### FT-5: Tail filtering retains last N lines and reports omitted count

- **Given:** four-line input, `OutputConfig::default().with_tail(2).with_width(10)`
- **When:** `process_output(input, "", &config)`
- **Then:** `result.content.lines().count() == 2`; `result.lines_omitted == 2` — first two lines dropped

### FT-6: Head and tail combined — union of both windows preserved

- **Given:** five-line input `"line1\nline2\nline3\nline4\nline5"`, `OutputConfig::default().with_head(2).with_tail(2)`
- **When:** `process_output("line1\nline2\nline3\nline4\nline5", "", &config)`
- **Then:** `result.content == "line1\nline2\nline4\nline5"`; `result.lines_omitted == 1` — middle line dropped

### FT-7: Truncated lines receive configured suffix

- **Given:** long input `"this is a very long line that needs truncation"`, `OutputConfig::default().with_width(10)`
- **When:** `process_output(input, "", &config)`
- **Then:** `result.width_truncated == true`; `result.content.contains("→")`

### FT-8: Width=0 disables truncation entirely

- **Given:** long input `"this is a very long line"`, `OutputConfig::default().with_width(0)`
- **When:** `process_output("this is a very long line", "", &config)`
- **Then:** `result.content == "this is a very long line"`; `result.width_truncated == false`

### FT-9: StreamFilter::Stdout selects only stdout content

- **Given:** stdout `"stdout"`, stderr `"stderr"`, `OutputConfig::default().with_stream_filter(StreamFilter::Stdout)`
- **When:** `process_output("stdout", "stderr", &config)`
- **Then:** `result.content == "stdout"` — stderr discarded entirely

### FT-10: StreamFilter::Stderr selects only stderr content

- **Given:** stdout `"stdout"`, stderr `"stderr"`, `OutputConfig::default().with_stream_filter(StreamFilter::Stderr)`
- **When:** `process_output("stdout", "stderr", &config)`
- **Then:** `result.content == "stderr"` — stdout discarded entirely

### FT-11: Line of exact max_width is not truncated

- **Given:** single-line input `"0123456789"` (10 visible chars), `OutputConfig::default().with_width(10)`
- **When:** `process_output("0123456789", "", &config)`
- **Then:** `result.width_truncated == false`; content begins with `"0123456789"` intact

### FT-12: process_output with head limit reports accurate lines_omitted count

- **Given:** five-line input `"line1\nline2\nline3\nline4\nline5"`, `OutputConfig::default().with_head(2)`
- **When:** `process_output("line1\nline2\nline3\nline4\nline5", "", &config)`
- **Then:** `result.content` contains only `"line1"` and `"line2"`; `result.lines_omitted == 3`

### FT-13: unicode_aware=true activates grapheme-based width measurement

- **Given:** input `"café"` (4 visible chars, multi-byte UTF-8), `OutputConfig::default().with_unicode_aware(true).with_width(3)`
- **When:** `process_output("café", "", &config)`
- **Then:** `result.width_truncated == true` — visual width 4 exceeds limit 3; the `unicode_aware=true` code path in `apply_width_filtering` is exercised

### FT-14: merge_streams does not insert extra newline when stderr ends with newline

- **Given:** stderr `"err\n"` (trailing newline), stdout `"out"`, `StreamFilter::Both`
- **When:** `merge_streams("out", "err\n", &StreamFilter::Both)`
- **Then:** result is `"err\nout"` — no double-newline separator; the trailing newline from stderr serves as the stream boundary; result does not contain `"\n\n"`

### FT-15: head(0) produces empty content with all lines reported as omitted

- **Given:** three-line input `"a\nb\nc"`, `OutputConfig::default().with_head(0)`
- **When:** `process_output("a\nb\nc", "", &config)`
- **Then:** `result.content.is_empty() == true`; `result.lines_omitted == 3` — all lines dropped; head limit of zero retains nothing

### FT-16: tail(0) produces empty content with all lines reported as omitted

- **Given:** three-line input `"a\nb\nc"`, `OutputConfig::default().with_tail(0)`
- **When:** `process_output("a\nb\nc", "", &config)`
- **Then:** `result.content.is_empty() == true`; `result.lines_omitted == 3` — all lines dropped; tail limit of zero retains nothing

### FT-17: width=1 truncates content — extreme lower boundary of width limit

- **Given:** single-line input `"hello"` (5 visible chars), `OutputConfig::default().with_width(1)`
- **When:** `process_output("hello", "", &config)`
- **Then:** `result.width_truncated == true`; width=1 is processed as an active limit (not short-circuited like width=0); the suffix `"→"` is absent — at width=1 the suffix itself (1 visible char) would exceed the available budget, so the implementation omits it

### FT-18: is_default() returns false when stream_filter, width_suffix, or unicode_aware deviates from default

- **Given:** three configs derived from `OutputConfig::default()`: (a) `.with_stream_filter(StreamFilter::Stdout)`, (b) `.with_suffix("...")`, (c) `.with_unicode_aware(true)`
- **When:** `is_default()` called on each modified config and on an unmodified default
- **Then:** all three modified configs return `is_default() == false`; the unmodified default returns `is_default() == true`; `OutputConfig::new().is_default() == true`

### FT-19: head+tail overlapping windows — all lines shown, lines_omitted is zero

- **Given:** five-line input `"line1\nline2\nline3\nline4\nline5"`, `OutputConfig::default().with_head(3).with_tail(3)`
- **When:** `process_output("line1\nline2\nline3\nline4\nline5", "", &config)`
- **Then:** `result.content == "line1\nline2\nline3\nline4\nline5"` — all five lines retained because head and tail windows overlap; `result.lines_omitted == 0`

### FT-20: Both mode with empty stdout — result equals stderr

- **Given:** empty stdout `""`, non-empty stderr `"stderr"`, `OutputConfig::default()` (StreamFilter::Both)
- **When:** `process_output("", "stderr", &config)`
- **Then:** `result.content == "stderr"` — empty stdout contributes nothing; no separator inserted

### FT-21: Both mode with empty stderr — result equals stdout

- **Given:** non-empty stdout `"stdout"`, empty stderr `""`, `OutputConfig::default()` (StreamFilter::Both)
- **When:** `process_output("stdout", "", &config)`
- **Then:** `result.content == "stdout"` — empty stderr contributes nothing; no separator inserted

### FT-22: head limit exceeds total line count — all lines returned

- **Given:** three-line input `"line1\nline2\nline3"`, head limit `10`
- **When:** `head(text, 10)` (direct utility call)
- **Then:** result == `"line1\nline2\nline3"` — all three lines retained; no truncation occurs when limit exceeds available lines

### FT-23: ANSI codes preserved when truncation fires

- **Given:** ANSI-colored input `"\x1b[31mred text that is very long\x1b[0m"`, `OutputConfig::default().with_width(8)`
- **When:** `process_output(input, "", &config)`
- **Then:** `result.width_truncated == true`; `result.content.contains("\x1b[31m")` — the opening ANSI code survives truncation

### FT-24: is_default() returns false when tail is set

- **Given:** `OutputConfig::default().with_tail(2)`
- **When:** `is_default()` called
- **Then:** `config.is_default() == false` — tail field deviates from default `None`

### FT-25: is_default() returns false when width is set

- **Given:** `OutputConfig::default().with_width(5)`
- **When:** `is_default()` called
- **Then:** `config.is_default() == false` — width field deviates from default `0`

### FT-26: head limit equals total line count — all lines returned

- **Given:** three-line input `"line1\nline2\nline3"`, head limit `3`
- **When:** `head(text, 3)` (direct utility call)
- **Then:** result == `"line1\nline2\nline3"` — all three lines retained; exact-match limit is not off-by-one

### FT-27: head on empty input — empty result

- **Given:** empty input `""`, head limit `5`
- **When:** `head("", 5)` (direct utility call)
- **Then:** result == `""` — empty input returns empty output without error

### FT-28: tail limit exceeds total line count — all lines returned

- **Given:** three-line input `"line1\nline2\nline3"`, tail limit `10`
- **When:** `tail(text, 10)` (direct utility call)
- **Then:** result == `"line1\nline2\nline3"` — all three lines retained when limit exceeds available lines

### FT-29: tail limit equals total line count — all lines returned

- **Given:** three-line input `"line1\nline2\nline3"`, tail limit `3`
- **When:** `tail(text, 3)` (direct utility call)
- **Then:** result == `"line1\nline2\nline3"` — all three lines retained; exact-match limit is not off-by-one

### FT-30: tail on empty input — empty result

- **Given:** empty input `""`, tail limit `5`
- **When:** `tail("", 5)` (direct utility call)
- **Then:** result == `""` — empty input returns empty output without error

### FT-31: head+tail sum equals total line count — all lines returned, lines_omitted is zero

- **Given:** three-line input `"line1\nline2\nline3"`, `OutputConfig::default().with_head(2).with_tail(1)`
- **When:** `process_output("line1\nline2\nline3", "", &config)`
- **Then:** `result.content == "line1\nline2\nline3"`; `result.lines_omitted == 0` — head(2) and tail(1) together cover all three lines exactly, no gap

### FT-32: truncated line uses configured width suffix

- **Given:** long input `"this is a very long line"`, `OutputConfig::default().with_width(10).with_suffix("...")`
- **When:** `process_output(input, "", &config)`
- **Then:** `result.width_truncated == true`; `result.content.contains("...")` — the custom suffix replaces the default `"→"` in the truncated output

### FT-33: combined both-streams + head + width

- **Given:** stdout `"out1\nout2 is long\nout3"`, stderr `"err1\nerr2 is also long"`, `OutputConfig::default().with_head(3).with_width(15)` (StreamFilter::Both)
- **When:** `process_output(stdout, stderr, &config)`
- **Then:** output starts with `"err1"` (stderr precedes stdout); line count == 3 (head limit applied after stream merge)

### FT-34: Both streams end with newline — no double-newline separator in merged output

- **Given:** stdout `"out\n"`, stderr `"err\n"`, `StreamFilter::Both`
- **When:** `merge_streams("out\n", "err\n", &StreamFilter::Both)`
- **Then:** result equals `"err\nout\n"` — no `"\n\n"` sequence appears; separator logic is not triggered when stderr ends with `'\n'`

### FT-35: stdout ends with newline, stderr does not — exactly one newline separator added

- **Given:** stdout `"out\n"`, stderr `"err"` (no trailing newline), `StreamFilter::Both`
- **When:** `merge_streams("out\n", "err", &StreamFilter::Both)`
- **Then:** result equals `"err\nout\n"` — exactly one `'\n'` added between streams; no content lost

### FT-36: StreamFilter::Stdout combined with head limit — stderr discarded, head applied to stdout-only stream

- **Given:** stdout `"a\nb\nc"` (3 lines), stderr `"err"` (1 line), `OutputConfig::default().with_stream_filter(StreamFilter::Stdout).with_head(2)`
- **When:** `process_output("a\nb\nc", "err", &config)`
- **Then:** `result.content` contains `"a"` and `"b"` but not `"c"`; `"err"` is absent from content entirely; `result.lines_omitted == 1` — stderr discarded before head filtering; head applies to the stdout-only merged stream

### FT-37: head+tail+width all three limits active simultaneously

- **Given:** 6-line input where each line has ≥10 visible chars (e.g., `"abcdefghij\nklmnopqrst\nuvwxyzabcd\nefghijklmn\nopqrstuvwx\nyzabcdefgh"`), `OutputConfig::default().with_head(2).with_tail(2).with_width(8)`
- **When:** `process_output(input, "", &config)`
- **Then:** `result.lines_omitted == 2` — lines 3 and 4 are dropped (head window covers lines 1–2; tail window covers lines 5–6; no overlap); `result.width_truncated == true` — all retained lines exceed 8 visible chars; exactly 4 lines in `result.content`

### FT-38: Empty width suffix — truncated line ends at max_width with no marker appended

- **Given:** single-line input `"01234567890123456789"` (20 visible chars), `OutputConfig::default().with_width(10).with_suffix("")`
- **When:** `process_output("01234567890123456789", "", &config)`
- **Then:** `result.width_truncated == true`; content begins with `"0123456789"` (first 10 chars); no extra marker character appended after position 10 — the empty suffix produces clean truncation with no indicator

### FT-39: Empty stdout with non-empty stderr and active head limit

- **Given:** stdout `""`, stderr `"err1\nerr2\nerr3"` (3 lines), `OutputConfig::default().with_head(2)` (StreamFilter::Both)
- **When:** `process_output("", "err1\nerr2\nerr3", &config)`
- **Then:** `result.content` contains `"err1"` and `"err2"` but not `"err3"`; `result.lines_omitted == 1` — empty stdout contributes nothing; head(2) applies to the stderr-only merged stream

### FT-40: width=0 combined with head — width passthrough when head filtering is active

- **Given:** 3-line input where each line exceeds 8 visible chars (e.g., `"longline1\nlongline2\nlongline3"`), `OutputConfig::default().with_width(0).with_head(2)`
- **When:** `process_output("longline1\nlongline2\nlongline3", "", &config)`
- **Then:** `result.content` contains the first 2 lines intact (untruncated); `result.width_truncated == false` — width=0 disables truncation even when head filtering is active; `result.lines_omitted == 1`

### Features

| File | Relationship |
|------|-------------|
| [`../../../docs/feature/001_output_processing.md`](../../../docs/feature/001_output_processing.md) | Authoritative behavioral requirements for this spec |

### Sources

| File | Relationship |
|------|-------------|
| `../../../src/output.rs` | Implements the three-stage output processing pipeline under test |

### Tests

| File | Relationship |
|------|-------------|
| `../../../tests/output.rs` | FT-1: `select_streams_both`, `merge_streams_ordering`; FT-2: `head_basic`; FT-3: `width_no_truncation_needed`; FT-4: `ansi_preserved_when_no_truncation`; FT-5: `combined_tail_and_width`; FT-6: `head_tail_combined_no_overlap`; FT-7: `width_truncation_with_arrow`; FT-8: `width_zero_disables`; FT-9: `select_streams_stdout_only`; FT-10: `select_streams_stderr_only`; FT-11: `width_exact_boundary`; FT-12: `process_output_head_lines_omitted`; FT-13: `unicode_aware_truncation`; FT-14: `merge_streams_stderr_trailing_newline`; FT-15: `head_zero_produces_empty`; FT-16: `tail_zero_produces_empty`; FT-17: `width_one_truncates`; FT-18: `is_default_stream_filter`, `is_default_width_suffix`, `is_default_unicode_aware`; FT-19: `head_tail_overlap_shows_all`; FT-20: `select_streams_empty_stdout`; FT-21: `select_streams_empty_stderr`; FT-22: `head_exceeds_total`; FT-23: `ansi_preserved_with_truncation`; FT-24: `is_default_tail`; FT-25: `is_default_width`; FT-26: `head_exact`; FT-27: `head_empty`; FT-28: `tail_exceeds_total`; FT-29: `tail_exact`; FT-30: `tail_empty`; FT-31: `head_tail_exact_fit`; FT-32: `width_custom_suffix`; FT-33: `combined_streams_head_width`; FT-34: `merge_streams_both_trailing_newlines_no_double_newline`; FT-35: `merge_streams_stdout_trailing_newline_separator`; FT-36: `stdout_filter_with_head`; FT-37: `head_tail_width_triple_combination`; FT-38: `width_empty_suffix_no_marker`; FT-39: `empty_stdout_stderr_with_head`; FT-40: `width_zero_with_head` |
