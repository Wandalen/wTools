# Invariant: ANSI and Unicode

### Scope

- **Purpose**: Define the contracts for ANSI escape sequence handling, Unicode display width calculation, and terminal-safe color output.
- **Responsibility**: Documents visual_len, pad_to_width, ANSI reset, and color wrapping invariants.
- **In Scope**: Character width measurement, display-width-aware padding, ANSI reset guarantees, per-line color wrapping.
- **Out of Scope**: Data model contracts (see `invariant/001_data_model.md`) and formatting algorithms (see `algorithm/` docs).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/ansi_str.rs` | ANSI escape handling implementation |
| test | `tests/unicode_display_width_alignment.rs` | Unicode width and ANSI handling tests |

### Invariant Statement

#### visual_len Behavior

`visual_len` counts visible Unicode codepoints, excluding ANSI escape sequences. Measurement uses character count via `chars().count()`, not display width. ANSI sequences contribute zero to the count. The invariant: ANSI escape sequences are always excluded from width measurement; only printable characters are counted.

#### Character Width Reference

| Character Type | Example | Byte Count | Char Count | Display Width |
|----------------|---------|------------|------------|---------------|
| ASCII | "Hello" | 5 | 5 | 5 |
| Cyrillic | "Привіт" | 12 | 6 | 6 |
| CJK | "日本語" | 9 | 3 | **6** |
| Emoji | "🎉" | 4 | 1 | **2** |
| Combining | "e\u{0301}" (é) | 3 | 2 | 1 |
| ANSI | "\x1b[31mtext\x1b[0m" | 14 | 4 (visible) | 4 |

**Key insight**: multi-byte encoding (byte count > char count) does NOT imply wide display. Cyrillic is multi-byte but 1 display width per character. Only CJK and emoji have display width = 2.

#### pad_to_width Contract

`pad_to_width` uses the East Asian Width property (via the `unicode-width` crate) for terminal column measurement, not raw character count. Wide characters (CJK, emoji) count as 2 display columns. Normal characters (ASCII, Cyrillic, Latin) count as 1. Zero-width characters (combining marks) count as 0. ANSI escape sequences count as 0. When text display width already meets or exceeds the target width, the text is returned unchanged. The `align_right` parameter controls whether padding is prepended (right-align) or appended (left-align).

**Fix(issue-003)**: Previous implementation mixed character-count-based padding with display-width-based formatting, causing misalignment with wide Unicode characters. Always use display width for terminal alignment, not char count.

#### ANSI Reset Invariant

Every colored line must end with `\x1b[0m` (ANSI reset) before the trailing newline character. This prevents terminal background-color bleed into subsequent lines. The pattern is: `color_code + line_content + \x1b[0m + \n`. Enforced in `format_internal()`.

When `colorize_header` or `alternating_rows` is enabled, the formatter wraps each output line individually with its own color and reset pair.

#### Per-Line Color Wrapping for Multiline Cells

For cells containing newlines, each sub-line is wrapped separately. The row buffer is split on newlines and each resulting line gets its own color/reset pair. Single-line cells produce one color/reset pair per row.

#### DecoratedText Detail-Line Rule

When rendering `DecoratedText` detail lines, always iterate over `ct.text.lines()` — never call `ct.render().lines()`. Calling `render()` produces `color_prefix + text + ANSI_RESET` as a single string. Calling `.lines()` on that result splits within the text but places the ANSI_RESET token only after the last sub-line, causing intermediate lines to bleed the terminal's background color across line boundaries. The correct approach iterates the raw text lines and wraps each independently with its own color/reset pair.

### Enforcement Mechanism

`visual_len` filters escape sequences by scanning for `\x1b` start and `m` end characters, counting only characters outside escape sequences. `pad_to_width` uses `unicode_width::UnicodeWidthStr::width()` for measurement before computing padding. The ANSI reset invariant is enforced in `format_internal()` which wraps each output line before returning. The `DecoratedText` rendering rule is enforced by code convention — the correct rendering path is the only one present in `src/formatters/table/mod.rs`.

### Violation Consequences

| Invariant | Consequence of Violation |
|-----------|------------------------|
| visual_len ANSI exclusion | Column widths computed too wide; content appears narrower than allocated space |
| pad_to_width display width | Wide characters (CJK, emoji) misalign column boundaries; table borders shift |
| ANSI reset per-line | Terminal background color bleeds across subsequent output lines |
| Per-line multiline wrapping | Intermediate multiline cell sub-lines inherit prior row's background color |
| DecoratedText line iteration | Detail annotation lines bleed color across line boundaries |
