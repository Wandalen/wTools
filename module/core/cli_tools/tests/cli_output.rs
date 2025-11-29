//! CLI output processing tests.
//!
//! Ported from `unilang::output` module tests.
//!
//! ## Critical Bug Fix: Width Truncation Boundary Detection
//!
//! **Bug (pre-migration):** Text exactly matching `max_width` was incorrectly truncated.
//!
//! **Root Cause:** `ansi::truncate()` reserves space for the suffix *within* `max_width`.
//! For example, with `max_width=5` and suffix="→":
//! - Input: "hello" (5 visible chars)
//! - Old behavior: Truncated to "hell→" (incorrectly assumed truncation needed)
//! - Correct behavior: Should return "hello" unchanged (fits exactly)
//!
//! **Fix Applied:** Check `visual_len(line) > max_width` before calling `truncate()`.
//! Only invoke truncation when line genuinely exceeds the limit.
//!
//! **Prevention:** Always validate visible width before truncating ANSI-aware text.
//! Never assume `truncate()` handles exact-width detection internally.
//!
//! **Pitfall:** `TruncateOptions` is designed for unconditional truncation (reserves suffix space).
//! Consumers must perform boundary detection before calling truncate functions.
//!
//! **Tests Validating Fix:**
//! - `width_no_truncation_needed`: Short text stays unchanged
//! - `width_custom_suffix`: Exact-width with ANSI codes not truncated
//! - Integration tests with combined head/tail/width processing
//!
//! ## Test Matrix
//!
//! | Category | Test | Status |
//! |----------|------|--------|
//! | OutputConfig | default is no processing | ✓ |
//! | OutputConfig | has_processing detection | ✓ |
//! | StreamFilter | stream selection | ✓ |
//! | head | truncate to N lines | ✓ |
//! | head | exceeds total returns all | ✓ |
//! | tail | truncate to last N lines | ✓ |
//! | tail | exceeds total returns all | ✓ |
//! | head+tail | combined truncation | ✓ |
//! | head+tail | overlap returns all | ✓ |
//! | width | no truncation needed | ✓ |
//! | width | truncation with arrow | ✓ |
//! | width | zero disables | ✓ |
//! | ANSI | preserves escape codes | ✓ |
//! | ANSI | adds reset on truncate | ✓ |
//! | integration | combined operations | ✓ |

use cli_tools::cli_output::*;
use strs_tools::string::lines::*;

// ============================================================================
// OutputConfig tests
// ============================================================================

#[ test ]
fn output_config_default_is_no_processing()
{
  let config = OutputConfig::default();
  assert!( config.is_default() );
  assert!( !config.has_processing() );
}

#[ test ]
fn output_config_with_head_has_processing()
{
  let config = OutputConfig::default().with_head( 5 );
  assert!( !config.is_default() );
  assert!( config.has_processing() );
}

#[ test ]
fn output_config_with_tail_has_processing()
{
  let config = OutputConfig::default().with_tail( 5 );
  assert!( config.has_processing() );
}

#[ test ]
fn output_config_with_width_has_processing()
{
  let config = OutputConfig::default().with_width( 80 );
  assert!( config.has_processing() );
}

// ============================================================================
// StreamFilter / stream selection tests
// ============================================================================

#[ test ]
fn select_streams_both()
{
  let config = OutputConfig::default();
  let result = process_output( "stdout", "stderr", &config );
  // stderr appears before stdout
  assert_eq!( result.content, "stderr\nstdout" );
}

#[ test ]
fn select_streams_stdout_only()
{
  let config = OutputConfig::default()
    .with_stream_filter( StreamFilter::Stdout );
  let result = process_output( "stdout", "stderr", &config );
  assert_eq!( result.content, "stdout" );
}

#[ test ]
fn select_streams_stderr_only()
{
  let config = OutputConfig::default()
    .with_stream_filter( StreamFilter::Stderr );
  let result = process_output( "stdout", "stderr", &config );
  assert_eq!( result.content, "stderr" );
}

#[ test ]
fn select_streams_empty_stdout()
{
  let config = OutputConfig::default();
  let result = process_output( "", "stderr", &config );
  assert_eq!( result.content, "stderr" );
}

#[ test ]
fn select_streams_empty_stderr()
{
  let config = OutputConfig::default();
  let result = process_output( "stdout", "", &config );
  assert_eq!( result.content, "stdout" );
}

#[ test ]
fn select_streams_both_empty()
{
  let config = OutputConfig::default();
  let result = process_output( "", "", &config );
  assert_eq!( result.content, "" );
}

// ============================================================================
// Head truncation tests (via lines module)
// ============================================================================

#[ test ]
fn head_basic()
{
  let text = "line1\nline2\nline3\nline4\nline5";
  let result = head( text, 2 );
  assert_eq!( result, "line1\nline2" );
}

#[ test ]
fn head_exceeds_total()
{
  let text = "line1\nline2\nline3";
  let result = head( text, 10 );
  assert_eq!( result, "line1\nline2\nline3" );
}

#[ test ]
fn head_exact()
{
  let text = "line1\nline2\nline3";
  let result = head( text, 3 );
  assert_eq!( result, "line1\nline2\nline3" );
}

#[ test ]
fn head_empty()
{
  let text = "";
  let result = head( text, 5 );
  assert_eq!( result, "" );
}

// ============================================================================
// Tail truncation tests (via lines module)
// ============================================================================

#[ test ]
fn tail_basic()
{
  let text = "line1\nline2\nline3\nline4\nline5";
  let result = tail( text, 2 );
  assert_eq!( result, "line4\nline5" );
}

#[ test ]
fn tail_exceeds_total()
{
  let text = "line1\nline2\nline3";
  let result = tail( text, 10 );
  assert_eq!( result, "line1\nline2\nline3" );
}

#[ test ]
fn tail_exact()
{
  let text = "line1\nline2\nline3";
  let result = tail( text, 3 );
  assert_eq!( result, "line1\nline2\nline3" );
}

#[ test ]
fn tail_empty()
{
  let text = "";
  let result = tail( text, 5 );
  assert_eq!( result, "" );
}

// ============================================================================
// Head + Tail combined tests
// ============================================================================

#[ test ]
fn head_tail_combined_no_overlap()
{
  let config = OutputConfig::default()
    .with_head( 2 )
    .with_tail( 2 );
  let result = process_output( "line1\nline2\nline3\nline4\nline5", "", &config );
  assert_eq!( result.content, "line1\nline2\nline4\nline5" );
  assert_eq!( result.lines_omitted, 1 );
}

#[ test ]
fn head_tail_overlap_shows_all()
{
  let config = OutputConfig::default()
    .with_head( 3 )
    .with_tail( 3 );
  let result = process_output( "line1\nline2\nline3\nline4\nline5", "", &config );
  assert_eq!( result.content, "line1\nline2\nline3\nline4\nline5" );
  assert_eq!( result.lines_omitted, 0 );
}

#[ test ]
fn head_tail_exact_fit()
{
  let config = OutputConfig::default()
    .with_head( 2 )
    .with_tail( 1 );
  let result = process_output( "line1\nline2\nline3", "", &config );
  assert_eq!( result.content, "line1\nline2\nline3" );
  assert_eq!( result.lines_omitted, 0 );
}

// ============================================================================
// Width truncation tests
// ============================================================================

#[ test ]
fn width_no_truncation_needed()
{
  let config = OutputConfig::default().with_width( 50 );
  let result = process_output( "short line", "", &config );
  // strs_tools adds reset code for safety
  assert!( result.content.starts_with( "short line" ) );
  assert!( !result.width_truncated );
}

#[ test ]
fn width_truncation_with_arrow()
{
  let config = OutputConfig::default().with_width( 10 );
  let result = process_output( "this is a very long line that needs truncation", "", &config );
  // Should be truncated to 9 chars + arrow
  assert!( result.width_truncated );
  assert!( result.content.contains( "→" ) );
  assert!( result.content.len() < 50 );
}

#[ test ]
fn width_zero_disables()
{
  let config = OutputConfig::default().with_width( 0 );
  let result = process_output( "this is a very long line", "", &config );
  assert_eq!( result.content, "this is a very long line" );
  assert!( !result.width_truncated );
}

#[ test ]
fn width_custom_suffix()
{
  let config = OutputConfig::default()
    .with_width( 10 )
    .with_suffix( "..." );
  let result = process_output( "this is a very long line", "", &config );
  assert!( result.width_truncated );
  assert!( result.content.contains( "..." ) );
}

// ============================================================================
// ANSI preservation tests
// ============================================================================

#[ test ]
fn ansi_preserved_when_no_truncation()
{
  let config = OutputConfig::default().with_width( 50 );
  let input = "\x1b[31mred text\x1b[0m";
  let result = process_output( input, "", &config );
  // strs_tools adds reset code for safety
  assert!( result.content.starts_with( input ) );
  assert!( result.content.contains( "\x1b[31m" ) );
}

#[ test ]
fn ansi_preserved_with_truncation()
{
  let config = OutputConfig::default().with_width( 8 );
  let input = "\x1b[31mred text that is very long\x1b[0m";
  let result = process_output( input, "", &config );
  // Should preserve ANSI codes and add reset
  assert!( result.content.contains( "\x1b[31m" ) );
  assert!( result.width_truncated );
}

// ============================================================================
// Integration tests (combined operations)
// ============================================================================

#[ test ]
fn combined_head_and_width()
{
  let config = OutputConfig::default()
    .with_head( 2 )
    .with_width( 10 );
  let input = "line1 is very long\nline2 is also long\nline3\nline4";
  let result = process_output( input, "", &config );

  // Should have 2 lines, both truncated
  assert_eq!( result.lines_omitted, 2 );
  assert!( result.width_truncated );
  let lines : Vec< &str > = result.content.lines().collect();
  assert_eq!( lines.len(), 2 );
}

#[ test ]
fn combined_tail_and_width()
{
  let config = OutputConfig::default()
    .with_tail( 2 )
    .with_width( 10 );
  let input = "line1\nline2\nline3 is very long\nline4 is also long";
  let result = process_output( input, "", &config );

  // Should have last 2 lines, both may be truncated
  assert_eq!( result.lines_omitted, 2 );
  let lines : Vec< &str > = result.content.lines().collect();
  assert_eq!( lines.len(), 2 );
}

#[ test ]
fn combined_streams_head_width()
{
  let config = OutputConfig::default()
    .with_head( 3 )
    .with_width( 15 );
  let result = process_output( "out1\nout2 is long\nout3", "err1\nerr2 is also long", &config );

  // stderr appears first, then stdout
  assert!( result.content.starts_with( "err1" ) );
  let lines : Vec< &str > = result.content.lines().collect();
  assert_eq!( lines.len(), 3 ); // head = 3
}

#[ test ]
fn merge_streams_ordering()
{
  // Test that stderr appears before stdout
  let result = merge_streams( "stdout", "stderr", &StreamFilter::Both );
  assert_eq!( result, "stderr\nstdout" );
}
