# Feature Test: Output Processing

### Scope

- **Purpose**: Verify the behavioral requirements documented in `docs/feature/001_output_processing.md`.
- **Responsibility**: Test spec for stream ordering, head/tail line filtering, width truncation, width=0 disable, truncation suffix, ANSI width exclusion, StreamFilter variants, and lines_omitted accuracy.
- **In Scope**: Stream Both ordering (FT-1), head() direct (FT-2), well-below-width non-truncation (FT-3), ANSI width exclusion (FT-4), tail via process_output (FT-5), head+tail union (FT-6), truncation suffix (FT-7), width=0 disable (FT-8), Stdout-only filter (FT-9), Stderr-only filter (FT-10), exact-width boundary (FT-11), process_output lines_omitted accuracy (FT-12).
- **Out of Scope**: strs_tools internals; architectural boundary enforcement — see `tests/docs/invariant/01_architectural_boundary.md`.

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

### Sources

| File | Relationship |
|------|-------------|
| `../../../src/output.rs` | Implements the three-stage output processing pipeline under test |

### Tests

| File | Relationship |
|------|-------------|
| `../../../tests/output.rs` | FT-1: `select_streams_both`, `merge_streams_ordering`; FT-2: `head_basic`; FT-3: `width_no_truncation_needed`; FT-4: `ansi_preserved_when_no_truncation`; FT-5: `combined_tail_and_width`; FT-6: `head_tail_combined_no_overlap`; FT-7: `width_truncation_with_arrow`; FT-8: `width_zero_disables`; FT-9: `select_streams_stdout_only`; FT-10: `select_streams_stderr_only`; FT-11: `width_exact_boundary`; FT-12: `process_output_head_lines_omitted` |

### Features

| File | Relationship |
|------|-------------|
| `../../../docs/feature/001_output_processing.md` | Authoritative behavioral requirements for this spec |
