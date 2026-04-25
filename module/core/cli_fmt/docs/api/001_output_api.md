# Output Module API

### Purpose

Unified CLI output processing: head/tail filtering, ANSI-aware width truncation, and stream
merging via a builder-pattern configuration API. Returns processed content and metadata
(lines omitted, truncation flag) for caller transparency.

### Signature

```rust
pub fn process_output( stdout : &str, stderr : &str, config : &OutputConfig ) -> ProcessedOutput
pub fn merge_streams( stdout : &str, stderr : &str, filter : &StreamFilter ) -> String
```

### Parameters

**`OutputConfig`** — builder-pattern configuration:

| Method | Type | Default | Effect |
|--------|------|---------|--------|
| `.with_head( n )` | `usize` | none | Retain first N lines |
| `.with_tail( n )` | `usize` | none | Retain last N lines |
| `.with_width( n )` | `usize` | none | Max visible chars per line; 0 = no limit |
| `.with_suffix( s )` | `impl Into<String>` | `"→"` | Suffix appended to truncated lines |
| `.with_stream_filter( f )` | `StreamFilter` | `Both` | Which streams to include |
| `.with_unicode_aware( b )` | `bool` | `false` | Grapheme-based width (requires `ansi_unicode` feature) |

**`StreamFilter`** variants:

| Variant | Behavior |
|---------|----------|
| `Both` | Stderr before stdout (CLI convention) |
| `Stdout` | Stdout only |
| `Stderr` | Stderr only |

### Returns

**`ProcessedOutput`**:

| Field | Type | Meaning |
|-------|------|---------|
| `content` | `String` | Processed output text, ready to display |
| `lines_omitted` | `usize` | Count of lines removed by head/tail filtering |
| `width_truncated` | `bool` | Whether any line was truncated by the width limit |

### Errors

None — `process_output()` and `merge_streams()` are infallible.

### Examples

```rust
use cli_fmt::output::*;

let config = OutputConfig::default()
  .with_head( 10 )
  .with_tail( 5 )
  .with_width( 80 )
  .with_stream_filter( StreamFilter::Both );

let result = process_output( stdout_str, stderr_str, &config );
println!( "{}", result.content );
if result.lines_omitted > 0
{
  eprintln!( "({} lines omitted)", result.lines_omitted );
}
```

**Migration from `strs_tools::output` / `unilang::output`:**

| Old | New |
|-----|-----|
| `TruncationConfig { head: Some( 10 ), .. }` | `OutputConfig::default().with_head( 10 )` |
| `apply_truncation( stdout, stderr, &config )` | `process_output( stdout, stderr, &config )` |
| `TruncatedOutput` | `ProcessedOutput` |
| `OutputFilter::Both` | `StreamFilter::Both` |
| `truncate_head( text, 10 )` | `strs_tools::string::lines::head( text, 10 )` |
| `truncate_tail( text, 10 )` | `strs_tools::string::lines::tail( text, 10 )` |

**Performance:**

| Operation | Complexity | Notes |
|-----------|------------|-------|
| `process_output()` | O(n) | Single pass through text |
| Stream merging | O(n) | Simple string concatenation |
| Line filtering | O(n) | `strs_tools::string::lines` |
| Width truncation | O(n × m) | n = text length, m = average line count |
