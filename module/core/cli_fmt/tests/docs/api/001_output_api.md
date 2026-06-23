# API Test: Output API

### Scope

- **Purpose**: Verify the API contract documented in `docs/api/001_output_api.md`.
- **Responsibility**: Test spec for infallibility, OutputConfig builder correctness, StreamFilter selection semantics, ProcessedOutput metadata accuracy, and merge_streams ordering guarantee.
- **In Scope**: Infallibility on empty inputs (AP-1), builder chain processing state detection (AP-2), StreamFilter variant selection (AP-3), result metadata correctness (AP-4), merge_streams ordering (AP-5), OutputConfig::new() equivalence to default (AP-6), has_processing for tail and width builders (AP-7), with_stream_filter builder routing (AP-8), with_suffix builder customization (AP-9), with_unicode_aware builder activation (AP-10), merge_streams infallibility on empty inputs (AP-11), with_width(0) active processing state despite runtime no-op (AP-12), with_suffix("") empty suffix produces clean truncation with no marker (AP-13), merge_streams with Stdout filter (AP-14), merge_streams with Stderr filter (AP-15).
- **Out of Scope**: Internal processing logic — see `tests/docs/feature/001_output_processing.md` for feature-level behavioral specs.

### AP-1: process_output is infallible — accepts empty inputs without error or panic

- **Given:** empty stdout `""`, empty stderr `""`, `OutputConfig::default()`
- **When:** `process_output("", "", &OutputConfig::default())`
- **Then:** returns `ProcessedOutput` with `content == ""`; `lines_omitted == 0`; `width_truncated == false`; no panic

### AP-2: OutputConfig builder chain enables processing state detection

- **Given:** `OutputConfig::default()` — reports `is_default() == true` and `has_processing() == false`
- **When:** `.with_head(5)` applied
- **Then:** `config.has_processing() == true`; `config.is_default() == false`

### AP-3: StreamFilter variants select exactly their designated stream content

- **Given:** stdout `"stdout"`, stderr `"stderr"`
- **When:** `process_output` called three times — once with each `StreamFilter` variant
- **Then:** `Stdout` variant → content is `"stdout"` only; `Stderr` variant → content is `"stderr"` only; `Both` variant → content is `"stderr\nstdout"` — stderr precedes stdout

### AP-4: ProcessedOutput metadata accurately reflects applied operations

- **Given:** four-line input `"line1 is very long\nline2 is also long\nline3\nline4"`, `OutputConfig::default().with_head(2).with_width(10)`
- **When:** `process_output("line1 is very long\nline2 is also long\nline3\nline4", "", &config)`
- **Then:** `result.lines_omitted == 2` (two lines dropped by head); `result.width_truncated == true` (retained lines exceed width 10)

### AP-5: merge_streams places stderr before stdout

- **Given:** stdout `"stdout"`, stderr `"stderr"`, `StreamFilter::Both`
- **When:** `merge_streams("stdout", "stderr", &StreamFilter::Both)`
- **Then:** result is `"stderr\nstdout"` — stderr precedes stdout, joined by newline

### AP-6: OutputConfig::new() is equivalent to OutputConfig::default()

- **Given:** `OutputConfig::new()` and `OutputConfig::default()` constructed independently
- **When:** each corresponding field is compared: `head`, `tail`, `width`, `width_suffix`, `stream_filter`, `unicode_aware`
- **Then:** all fields are equal; `OutputConfig::new().is_default() == true` — `new()` is a named constructor alias for `default()` per API contract

### AP-7: has_processing returns true when tail or width is configured

- **Given:** `OutputConfig::default().with_tail(5)` and separately `OutputConfig::default().with_width(80)`
- **When:** `has_processing()` called on each
- **Then:** both return `true` — tail and width configurations activate processing state just as head does

### AP-8: with_stream_filter() builder routes output to the configured stream

- **Given:** stdout `"hello"`, stderr `"world"`, `OutputConfig::default().with_stream_filter(StreamFilter::Stdout)`
- **When:** `process_output("hello", "world", &config)`
- **Then:** `result.content == "hello"` — only stdout content returned; stderr discarded; the builder wires the filter correctly into the processing pipeline

### AP-9: with_suffix() builder customizes the truncation marker

- **Given:** long input `"this is a very long line"`, `OutputConfig::default().with_width(10).with_suffix("...")`
- **When:** `process_output(input, "", &config)`
- **Then:** `result.width_truncated == true`; `result.content.contains("...")` — the custom suffix replaces the default `"→"` marker; the builder stores the value and it is applied during width filtering

### AP-10: with_unicode_aware() builder activates grapheme-based width measurement

- **Given:** multi-byte UTF-8 input `"café"` (4 visible graphemes), `OutputConfig::default().with_unicode_aware(true).with_width(3)`
- **When:** `process_output("café", "", &config)`
- **Then:** `result.width_truncated == true` — the builder activates Unicode grapheme counting; visual width 4 exceeds limit 3; the correct code path is exercised

### AP-11: merge_streams is infallible on two empty inputs

- **Given:** empty stdout `""`, empty stderr `""`, `StreamFilter::Both`
- **When:** `merge_streams("", "", &StreamFilter::Both)`
- **Then:** returns `""` without panic — the function accepts empty strings for both arguments; no separator is inserted; result length is zero

### AP-12: with_width(0) activates processing state despite disabling runtime truncation

- **Given:** `OutputConfig::default().with_width(0)`
- **When:** `has_processing()` and `is_default()` called on the configured value; then `process_output("this is long", "", &config)` called
- **Then:** `config.has_processing() == true` — width=0 is stored as `Some(0)` (not `None`), so the processing flag is set; `config.is_default() == false` — the width field deviates from its unset default (`None`); `result.width_truncated == false` — width=0 short-circuits the truncation stage at runtime, so no line is actually truncated

### AP-13: with_suffix("") produces clean truncation with no marker appended

- **Given:** single-line input `"01234567890123456789"` (20 visible chars), `OutputConfig::default().with_width(10).with_suffix("")`
- **When:** `process_output("01234567890123456789", "", &config)`
- **Then:** `result.width_truncated == true`; content begins with `"0123456789"` (first 10 chars); no extra character appended after position 10 — the empty suffix produces clean truncation with no visible indicator

### AP-14: merge_streams with Stdout filter returns only stdout content ⏳

- **Given:** stdout `"hello"`, stderr `"world"`, `StreamFilter::Stdout`
- **When:** `merge_streams("hello", "world", &StreamFilter::Stdout)`
- **Then:** returns `"hello"` — only stdout content is returned; stderr is discarded entirely; no newline or separator is inserted

### AP-15: merge_streams with Stderr filter returns only stderr content ⏳

- **Given:** stdout `"hello"`, stderr `"world"`, `StreamFilter::Stderr`
- **When:** `merge_streams("hello", "world", &StreamFilter::Stderr)`
- **Then:** returns `"world"` — only stderr content is returned; stdout is discarded entirely; no newline or separator is inserted

### APIs

| File | Relationship |
|------|-------------|
| [`../../../docs/api/001_output_api.md`](../../../docs/api/001_output_api.md) | Authoritative API contract for this spec |

### Sources

| File | Relationship |
|------|-------------|
| `../../../src/output.rs` | Implements all public API types and functions under contract |

### Tests

| File | Relationship |
|------|-------------|
| `../../../tests/output.rs` | AP-1: `select_streams_both_empty`; AP-2: `output_config_default_is_no_processing`, `output_config_with_head_has_processing`; AP-3: `select_streams_stdout_only`, `select_streams_stderr_only`, `select_streams_both`; AP-4: `combined_head_and_width`; AP-5: `merge_streams_ordering`; AP-6: `output_config_new_matches_default`; AP-7: `output_config_with_tail_has_processing`, `output_config_with_width_has_processing`; AP-8: `select_streams_stdout_only`, `select_streams_stderr_only`, `stdout_filter_with_head`; AP-9: `width_custom_suffix`; AP-10: `unicode_aware_truncation`; AP-11: `merge_streams_both_empty_infallible`; AP-12: `output_config_with_width_zero_has_processing`; AP-13: `width_empty_suffix_no_marker`; AP-14: `merge_streams_stdout_only`; AP-15: `merge_streams_stderr_only` |
