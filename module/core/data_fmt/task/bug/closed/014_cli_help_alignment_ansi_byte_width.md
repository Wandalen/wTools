# BUG-014: CLI Help Key Width Uses Byte Count Instead of visual_len

- **Status:** Closed (Fixed)
- **Root Cause:** `format_cli_help()` used `.len()` (UTF-8 byte count) to compute
  `max_key_width` and per-row padding. ANSI escape sequences are 4–7 bytes each with
  zero visual width, so any key containing ANSI codes inflated the alignment column
  by the byte overhead, misaligning all sibling plain-text descriptions rightward.
- **Fix Location:** `src/formatters/text.rs` — `format_cli_help()` first pass (line ~343)
  and second pass (line ~389); both `.len()` replaced with `visual_len()`.
- **Pitfall:** Never use `.len()` on user-visible strings for alignment column
  computation. Use `visual_len()` (strips ANSI, counts chars) or `unicode_visual_len()`
  (EAW-aware) depending on whether East Asian Width matters for the context.
- **Test Reference:** `tests/text_cli_help.rs` —
  `ansi_key_alignment_uses_visual_len_not_byte_count_ac7`
  tagged `bug_reproducer(BUG-014)`; asserts `--help` description starts at column 13
  (indent=2 + max_visual_key=9 + gap=2), which would be 22 under the byte-count bug.
