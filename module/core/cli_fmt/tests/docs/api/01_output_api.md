# API Test: Output API

### Scope

- **Purpose**: Verify the API contract documented in `docs/api/001_output_api.md`.
- **Responsibility**: Test spec for infallibility, OutputConfig builder correctness, StreamFilter selection semantics, ProcessedOutput metadata accuracy, and merge_streams ordering guarantee.
- **In Scope**: Infallibility on empty inputs (AP-1), builder chain processing state detection (AP-2), StreamFilter variant selection (AP-3), result metadata correctness (AP-4), merge_streams ordering (AP-5).
- **Out of Scope**: Internal processing logic — see `tests/docs/feature/01_output_processing.md` for feature-level behavioral specs.

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

- **Given:** five-line input `"hello\nworld\nfoo\nbar\nbaz"`, `OutputConfig::default().with_head(3).with_width(4)`
- **When:** `process_output("hello\nworld\nfoo\nbar\nbaz", "", &config)`
- **Then:** `result.lines_omitted == 2` (two lines dropped by head); `result.width_truncated == true` (two lines exceed width 4)

### AP-5: merge_streams places stderr before stdout

- **Given:** stdout `"stdout"`, stderr `"stderr"`, `StreamFilter::Both`
- **When:** `merge_streams("stdout", "stderr", &StreamFilter::Both)`
- **Then:** result is `"stderr\nstdout"` — stderr precedes stdout, joined by newline

### Sources

| File | Relationship |
|------|-------------|
| `../../../src/output.rs` | Implements all public API types and functions under contract |

### Tests

| File | Relationship |
|------|-------------|
| `../../../tests/output.rs` | AP-1: `select_streams_both_empty`; AP-2: `output_config_with_head_has_processing`; AP-3: `select_streams_stdout_only`, `select_streams_stderr_only`, `select_streams_both`; AP-4: `combined_head_and_width`; AP-5: `merge_streams_ordering` |

### APIs

| File | Relationship |
|------|-------------|
| `../../../docs/api/001_output_api.md` | Authoritative API contract for this spec |
