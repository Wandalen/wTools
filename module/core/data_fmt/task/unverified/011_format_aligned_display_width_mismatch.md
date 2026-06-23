# 011 — format_aligned display width mismatch

## MOST Goal

Fix `TreeFormatter::format_aligned()` to use display-width measurement consistently in both passes, eliminating column misalignment when cells contain emoji or CJK characters.

- **Motivated:** willbe CLI `.clones layout::tree` renders augmented columns with emoji status indicators; migrating to `data_fmt::format_aligned` is blocked by incorrect width handling.
- **Observable:** `format_aligned` output with emoji/CJK cells has vertically aligned columns.
- **Scoped:** Single file change in `src/formatters/tree/aligned.rs`.
- **Testable:** `cargo nextest run --all-features -E 'test(aligned)'` — new test with emoji columns produces aligned output.

## Acceptance Criteria

- [ ] `aligned.rs` uses `unicode_visual_len` for pass 1 width calculation (not char-count `visual_len`)
- [ ] `aligned.rs` uses `pad_unicode_width` for pass 2 padding (not char-count `pad_to_width`)
- [ ] Both functions use the same `UnicodeWidthChar::width()` metric
- [ ] New test: tree with emoji column cells verifies column alignment correctness
- [ ] Existing aligned tree tests continue to pass
- [ ] `visual_len`/`pad_to_width` from `strs_tools::ansi` are no longer imported in `aligned.rs`
- [ ] Zero clippy warnings

## In Scope

- `src/formatters/tree/aligned.rs` — switch imports from `visual_len`/`pad_to_width` to `unicode_visual_len`/`pad_unicode_width`
- New test in `tests/aligned_tree_edge_cases.rs` or `tests/unicode_display_width_alignment.rs` covering emoji-column alignment
- `src/formatters/tree/mod.rs` if it re-exports or references the changed functions

## Out of Scope

- Changing `strs_tools` upstream to add `visual_width()` — self-contained fix preferred
- Modifying `visual_len`/`pad_to_width` re-exports in `src/ansi_str.rs` — those serve other callers
- Hierarchical (non-aligned) tree rendering — uses a closure, not column widths
- TableFormatter column width logic — separate code path

## Delivery Requirements

- Implementation must not introduce new dependencies
- All `cargo nextest run --all-features` tests pass (Level 1)
- `cargo clippy --all-targets --all-features -- -D warnings` clean (Level 3)

## Execution State

- **State:** ❓ (Unverified)
- **ID:** 011
- **Slug:** format_aligned_display_width_mismatch
- **Executor:** any
- **Priority:** 3
- **Value:** 6
- **Easiness:** 9
- **Safety:** 9
- **Dir:** `module/core/data_fmt`
- **Closes:** null
- **Reopen Count:** 0

## History

- **[2026-06-23]** `CREATED` — Filed from willbe CLI integration need; self-contained fix in aligned.rs
- **[2026-06-23]** `UPDATED` — Normalized to tsk.rulebook.md v5.11 format (was ad-hoc format)
