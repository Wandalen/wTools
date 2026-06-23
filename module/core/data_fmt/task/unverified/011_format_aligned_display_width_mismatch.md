# 011 — format_aligned uses char count instead of display width

- **Status:** Open
- **Crate:** data_fmt
- **Severity:** Medium
- **Requested by:** wip (willbe CLI)
- **Blocked by:** None (self-contained fix possible)

## Problem

`TreeFormatter::format_aligned()` in `src/formatters/tree/aligned.rs:6` imports:

```rust
use crate::{ TreeNode, ColumnData, ansi_str::{ visual_len, pad_to_width } };
```

- `visual_len` (re-exported from `strs_tools::ansi`) uses `chars().count()` — character count
- `pad_to_width` (re-exported from `strs_tools::ansi`) uses `UnicodeWidthStr::width()` — display columns

Pass 1 computes max column widths with `visual_len` (char-based). Pass 2 pads with `pad_to_width` (display-width-based). The measurement mismatch causes incorrect column alignment when cells contain emoji or CJK characters.

Example: cell "master" has visual_len=9, display_width=9 (match). Cell with emoji status indicator has visual_len=N but display_width=N+1 per emoji (mismatch — each emoji is 1 char but 2 display columns).

## Requested Change

Switch `aligned.rs` to use `unicode_visual_len` and `pad_unicode_width` which already exist as `pub(crate)` in `src/ansi_str.rs:89-117` and correctly use `UnicodeWidthChar::width()`.

```rust
// aligned.rs line 6 — change from:
use crate::{ TreeNode, ColumnData, ansi_str::{ visual_len, pad_to_width } };
// to:
use crate::{ TreeNode, ColumnData, ansi_str::{ unicode_visual_len as visual_len, pad_unicode_width as pad_to_width } };
```

Both measurement and padding would then use the same `unicode_width` metric.

## Alternative

If `strs_tools` adds `visual_width()` (see strs_tools task 011), the re-export in `ansi_str.rs` could switch to that, and no aligned.rs change would be needed. But the self-contained fix above works without waiting for the upstream change.

## Motivation

`.clones layout::tree` in willbe CLI renders a tree with augmented columns (track, current status, tags, purpose). The current column contains emoji status indicators. Migrating from manual string concatenation to `data_fmt::format_aligned` requires correct emoji width handling.
