# Add 2 missing test corner cases — empty-suffix+multi-line+ANSI width truncation; exact-fit-at-max_width with head/tail

## Execution State

- **Executor Type:** any
- **filed_by:** ai
- **actor:** null
- **started_at:** null
- **expires_at:** null
- **round:** 1
- **state:** 📝 (Draft)
- **closes:** null
- **unit_type:** module
- **unit:** lib/yrd_core/wtools/dev/module/core/cli_fmt
- **validated_by:** null
- **validation_date:** null
- **blocked_by:** null

## Goal

Two width-truncation corner cases have no covering test in `tests/output.rs`, confirmed by direct read of test bodies (not just names):

1. **Empty suffix + multi-line + ANSI, combined.** `width_empty_suffix_no_marker` (line 840) covers empty-suffix truncation but only on a single plain-text line. `ansi_preserved_with_truncation` (line 405) covers ANSI-code preservation under truncation but with the default suffix, single line. No existing test combines empty suffix, multiple lines, and ANSI codes in one scenario.
2. **Width exactly at max_width (the non-truncation boundary) combined with head/tail line selection.** `width_exact_boundary` (line 365) tests the boundary but without head/tail configured. `head_tail_exact_fit` (line 282) tests head/tail *line-count* exact-fit but without any width constraint. `head_tail_width_triple_combination` (line 815) combines all three knobs but forces truncation on every retained line — it does not test the boundary case where a retained line sits exactly at max_width and must NOT truncate.

## History

- **[2026-07-15]** `CREATED` — Draft filed from a finding carried from an earlier session; confirmed still open this session via direct read of the current `tests/output.rs` test bodies at the cited line numbers.
