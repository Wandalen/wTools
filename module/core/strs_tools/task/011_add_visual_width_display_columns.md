# 011 — Add `visual_width` function returning display columns

- **Status:** Open
- **Crate:** strs_tools
- **Severity:** Medium
- **Requested by:** wip (willbe CLI)

## Problem

`visual_len()` in `src/ansi/visual.rs:48` uses `t.chars().count()` — returns character count, not terminal display columns. `visual_len_unicode()` (line 81) uses `t.graphemes(true).count()` — returns grapheme count, still not display columns.

Neither function returns the number of terminal columns a string occupies. Emoji (2 display columns, 1 char) and CJK characters (2 display columns, 1 char) are undercounted.

Meanwhile `pad_to_width()` (line 148) already uses `UnicodeWidthStr::width()` for correct display-column measurement. Any code that computes a target width via `visual_len` then pads via `pad_to_width` has a measurement mismatch — the target is too narrow.

## Requested Change

Add a `visual_width()` function that strips ANSI escapes then measures display columns using `UnicodeWidthChar::width()` (already a dependency via `unicode-width`).

```rust
pub fn visual_width( text : &str ) -> usize
{
  use unicode_width::UnicodeWidthChar;
  parse_segments( text )
    .iter()
    .filter_map( | seg | match seg
    {
      Segment::Text( t ) => Some(
        t.chars().map( | c | c.width().unwrap_or( 1 ) ).sum::< usize >()
      ),
      Segment::Ansi( _ ) => None,
    })
    .sum()
}
```

## Motivation

`data_fmt::TreeFormatter::format_aligned()` uses `visual_len` for pass-1 column-width calculation and `pad_to_width` for pass-2 padding. The char-count vs display-width mismatch causes misaligned columns when tree cells contain emoji status indicators (e.g., clone status output with emoji).

## Impact

- All consumers of `visual_len` that feed results to `pad_to_width` have a latent alignment bug
- `visual_width` provides a drop-in replacement with correct terminal semantics
- No breaking change — additive API
