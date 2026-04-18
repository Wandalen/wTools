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

### visual_len() Behavior

Counts visible Unicode codepoints, excluding ANSI escape sequences.

```rust
pub fn visual_len( text : &str ) -> usize
{
  let mut len = 0;
  let mut in_escape = false;
  for ch in text.chars()
  {
    if ch == '\x1b' { in_escape = true; }
    else if in_escape && ch == 'm' { in_escape = false; }
    else if !in_escape { len += 1; }
  }
  len
}
```

Measurement: character count via `chars().count()`, not display width. ANSI sequences contribute zero to the count.

### pad_to_width() Contract

Display-width-aware padding using the `unicode-width` crate (Fix issue-003).

```rust
pub fn pad_to_width( text : &str, target_width : usize, align_right : bool ) -> String;
```

Uses East Asian Width property for measurement (terminal columns, not character count). Handles:

- Wide characters (CJK, emoji): 2 display columns
- Normal characters (ASCII, Cyrillic, Latin): 1 display column
- Zero-width characters (combining marks): 0 display columns
- ANSI escape sequences: 0 display columns (filtered out)

If text display width already >= target_width, returns text unchanged. The `align_right` parameter controls whether padding is prepended (true) or appended (false).

**Fix(issue-003)**: Previous implementation mixed character-count-based padding with display-width-based formatting, causing misalignment with wide Unicode characters. Always use display width for terminal alignment, not char count.

### Character Width Reference

| Character Type | Example | Byte Count | Char Count | Display Width |
|----------------|---------|------------|------------|---------------|
| ASCII | "Hello" | 5 | 5 | 5 |
| Cyrillic | "Привіт" | 12 | 6 | 6 |
| CJK | "日本語" | 9 | 3 | **6** |
| Emoji | "🎉" | 4 | 1 | **2** |
| Combining | "e\u{0301}" (é) | 3 | 2 | 1 |
| ANSI | "\x1b[31mtext\x1b[0m" | 14 | 4 (visible) | 4 |

**Key insight**: multi-byte encoding (byte count > char count) does NOT imply wide display. Cyrillic is multi-byte but 1 display width per character. Only CJK and emoji have display width = 2.

### ANSI_RESET Invariant

Every colored line MUST end with `\x1b[0m` (ANSI reset) before the trailing `\n` to prevent terminal background-color bleed into subsequent lines. Enforced in `format_internal()`.

When `colorize_header` or `alternating_rows` is enabled, the formatter wraps each output line individually:

```
color_code + line_content + \x1b[0m + \n
```

#### Per-Line Color Wrapping for Multiline Cells

For cells containing `\n`, each sub-line is wrapped separately. The row buffer is split on `\n` and each resulting line gets its own color/RESET pair:

```rust
for line in row_buf.lines()
{
  output.push_str( color );
  output.push_str( line );
  output.push_str( ANSI_RESET );  // \x1b[0m before \n
  output.push( '\n' );
}
```

Single-line cells produce one color/RESET pair per row.

#### DecoratedText Detail-Line Pitfall

When rendering `DecoratedText` detail lines, always iterate `ct.text.lines()` — **never** call `ct.render().lines()`.

**Why it matters:** `ct.render()` produces `color_prefix + text + ANSI_RESET` as one string. Calling `.lines()` on that result splits on `\n` within the text, placing the ANSI_RESET token only after the last sub-line. Intermediate lines have no RESET, so the terminal's background color bleeds across line boundaries.

**Correct pattern** (`src/formatters/table.rs`):

```rust
for line in ct.text.lines()
{
  output.push_str( indent );
  if let Some( ref color ) = ct.color
  {
    output.push_str( color );
    output.push_str( line );
    output.push_str( ANSI_RESET );
  }
  else
  {
    output.push_str( line );
  }
  output.push( '\n' );
}
```

**Wrong pattern** (causes ANSI bleed):

```rust
// ❌ NEVER do this — RESET lands after last sub-line only:
for line in ct.render().lines()
{
  output.push_str( line );
  output.push( '\n' );
}
```

### Performance Characteristics

#### Time Complexity

- Tree construction: O(n * d) where n = nodes, d = depth
- Tree rendering: O(n) single traversal
- Table rendering: O(r * c) where r = rows, c = columns
- Aggregation: O(n) two passes
- Flattening: O(n) DFS traversal

#### Space Complexity

- Tree storage: O(n) for node storage
- Rendering: O(d) recursion depth
- Table output: O(r * c) for output buffer
- Minimal allocations, string concatenation only, no temporary collections

### Non-Functional Requirements

- **Zero unsafe code**: no `unsafe` blocks anywhere in the crate
- **No unwrap in production paths**: all `Option`/`Result` handling is explicit
- **no_std compatible**: works with `alloc` crate, no standard library required
- **Zero core dependencies**: core formatters have no external dependencies
- **Platform-independent**: pure Rust, no platform-specific code
