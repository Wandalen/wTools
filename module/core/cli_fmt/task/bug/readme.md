# Bug Reports — cli_fmt

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `readme.md` | Bug index and open bugs tracking |
| `closed/` | Resolved bug reports archive |

## Open Bugs

| ID | Title | State | Severity | Component | Filed | Root Cause | Reopen Count |
|----|-------|-------|----------|-----------|-------|------------|--------------|

## Closed Bugs

| ID | Title | Severity | Component | Filed | Root Cause |
|----|-------|----------|-----------|-------|------------|
| BUG-005 | [Width truncation boundary detection](./closed/005_width_truncation_boundary.md) | Minor | `src/output.rs::apply_width_filtering` | 2026-05-17 | `truncate()` called even when `visual_len == max_width` |
| BUG-006 | [stderr stream ordering](./closed/006_stderr_stream_ordering.md) | Medium | `src/output.rs::merge_streams` | 2025-11-29 | stdout placed before stderr instead of after |
| BUG-007 | [ExampleEntry.desc silent drop](./closed/007_example_desc_silent_drop.md) | Medium | `src/help.rs::emit_examples` | 2026-05-17 | `emit_examples()` ignored `desc: Option<String>` field |
