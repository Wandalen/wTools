# Output Processing

### Purpose

CLI output processing for command-line applications. Provides head/tail line filtering,
ANSI-aware width truncation, and stream merging. Distinct from general-purpose string
manipulation in `strs_tools`: this module encodes CLI-specific policy decisions (stream
ordering, Unix head/tail semantics, transparency metadata) that strs_tools must not contain.

### Behavior

`process_output()` applies three sequential stages:

1. **Stream selection** — merge stdout/stderr according to `StreamFilter`; `Both` places
   stderr before stdout (CLI convention: errors visible without scrolling past normal output)
2. **Line filtering** — apply head/tail limits via `strs_tools::string::lines::{head, tail, head_and_tail}`
3. **Width truncation** — truncate lines exceeding `width` using `strs_tools::ansi::truncate_lines()`;
   ANSI codes are preserved; a configurable suffix (default `"→"`) marks truncated lines

Result metadata is always accurate:
- `lines_omitted` counts lines removed by head/tail; zero when head+tail windows overlap
- `width_truncated` is true if any line was truncated by the width limit

### Constraints

- ANSI escape sequences are excluded from visible width measurement
- Text with visible length exactly equal to `max_width` is NOT truncated; boundary detection
  (`visual_len > max_width`) is required before calling `strs_tools::ansi::truncate()`
- Width of zero disables truncation entirely (treated as no limit)
- `lines_omitted` is accurate even when head+tail windows overlap (overlap yields 0 omitted)
- Unicode grapheme support is opt-in via `ansi_unicode` feature; default uses char-based width

### Examples

```rust
use cli_fmt::output::*;

// Show first 5 and last 5 lines, max 80 chars wide
let config = OutputConfig::default()
  .with_head( 5 )
  .with_tail( 5 )
  .with_width( 80 );

let result = process_output( stdout_str, stderr_str, &config );
println!( "{}", result.content );
println!( "Lines omitted: {}", result.lines_omitted );
println!( "Width truncated: {}", result.width_truncated );
```
