# API Test: Output API

### Scope

- **Purpose**: Verify the API contract documented in `docs/api/001_output_api.md`.
- **Responsibility**: Test spec for infallibility, OutputConfig builder correctness, StreamFilter selection semantics, ProcessedOutput metadata accuracy, and merge_streams ordering guarantee.
- **In Scope**: Infallibility on empty inputs (AP-1), builder chain processing state detection (AP-2), StreamFilter variant selection (AP-3), result metadata correctness (AP-4), merge_streams ordering (AP-5), OutputConfig::new() equivalence to default (AP-6), has_processing for tail and width builders (AP-7).
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

### APIs

| File | Relationship |
|------|-------------|
| `../../../docs/api/001_output_api.md` | Authoritative API contract for this spec |

### Sources

| File | Relationship |
|------|-------------|
| `../../../src/output.rs` | Implements all public API types and functions under contract |

### Tests

| File | Relationship |
|------|-------------|
| `../../../tests/output.rs` | AP-1: `select_streams_both_empty`; AP-2: `output_config_default_is_no_processing`, `output_config_with_head_has_processing`; AP-3: `select_streams_stdout_only`, `select_streams_stderr_only`, `select_streams_both`; AP-4: `combined_head_and_width`; AP-5: `merge_streams_ordering`; AP-6: `output_config_new_matches_default`; AP-7: `output_config_with_tail_has_processing`, `output_config_with_width_has_processing` |
