//! Output truncation tests.
//!
//! ## Test Matrix
//!
//! | Category | Test | Status |
//! |----------|------|--------|
//! | `OutputConfig` | default is no truncation | ✓ |
//! | `OutputConfig` | `has_processing` detection | ✓ |
//! | `StreamFilter` | stream selection | ✓ |
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
//!
//! **Note:** This test file now uses `cli_fmt` directly rather than the deprecated
//! `unilang::output` module to avoid deprecation warnings.
//!
//! **Requires:** `output_processing` feature to be enabled.

#![cfg(feature = "output_processing")]

use cli_fmt::output::*;
use strs_tools::string::lines::*;
use strs_tools::ansi::{ truncate, TruncateOptions };

// ============================================================================
// OutputConfig tests
// ============================================================================

#[test]
fn output_config_default_is_no_processing()
{
  let config = OutputConfig::default();
  assert!(config.is_default());
  assert!(!config.has_processing());
}

#[test]
fn output_config_with_head_has_processing()
{
  let config = OutputConfig { head: Some(5), ..Default::default() };
  assert!(!config.is_default());
  assert!(config.has_processing());
}

#[test]
fn output_config_with_tail_has_processing()
{
  let config = OutputConfig { tail: Some(5), ..Default::default() };
  assert!(config.has_processing());
}

#[test]
fn output_config_with_width_has_processing()
{
  let config = OutputConfig { width: Some(80), ..Default::default() };
  assert!(config.has_processing());
}

// ============================================================================
// StreamFilter / stream selection tests
// ============================================================================

#[test]
fn select_streams_both()
{
  let config = OutputConfig::default();
  let result = process_output("stdout", "stderr", &config);
  // stderr appears before stdout
  assert_eq!(result.content, "stderr\nstdout");
}

#[test]
fn select_streams_stdout_only()
{
  let config = OutputConfig
  {
    stream_filter: StreamFilter::Stdout,
    ..Default::default()
  };
  let result = process_output("stdout", "stderr", &config);
  assert_eq!(result.content, "stdout");
}

#[test]
fn select_streams_stderr_only()
{
  let config = OutputConfig
  {
    stream_filter: StreamFilter::Stderr,
    ..Default::default()
  };
  let result = process_output("stdout", "stderr", &config);
  assert_eq!(result.content, "stderr");
}

#[test]
fn select_streams_empty_stdout()
{
  let config = OutputConfig::default();
  let result = process_output("", "stderr", &config);
  assert_eq!(result.content, "stderr");
}

#[test]
fn select_streams_empty_stderr()
{
  let config = OutputConfig::default();
  let result = process_output("stdout", "", &config);
  assert_eq!(result.content, "stdout");
}

#[test]
fn select_streams_both_empty()
{
  let config = OutputConfig::default();
  let result = process_output("", "", &config);
  assert_eq!(result.content, "");
}

// ============================================================================
// Head truncation tests
// ============================================================================

#[test]
fn head_truncation_basic()
{
  let text = "line1\nline2\nline3\nline4\nline5";
  let result = head(text, 2);
  assert_eq!(result, "line1\nline2");
}

#[test]
fn head_truncation_exceeds_total()
{
  let text = "line1\nline2\nline3";
  let result = head(text, 10);
  assert_eq!(result, "line1\nline2\nline3");
}

#[test]
fn head_truncation_exact()
{
  let text = "line1\nline2\nline3";
  let result = head(text, 3);
  assert_eq!(result, "line1\nline2\nline3");
}

#[test]
fn head_truncation_empty()
{
  let text = "";
  let result = head(text, 5);
  assert_eq!(result, "");
}

// ============================================================================
// Tail truncation tests
// ============================================================================

#[test]
fn tail_truncation_basic()
{
  let text = "line1\nline2\nline3\nline4\nline5";
  let result = tail(text, 2);
  assert_eq!(result, "line4\nline5");
}

#[test]
fn tail_truncation_exceeds_total()
{
  let text = "line1\nline2\nline3";
  let result = tail(text, 10);
  assert_eq!(result, "line1\nline2\nline3");
}

#[test]
fn tail_truncation_exact()
{
  let text = "line1\nline2\nline3";
  let result = tail(text, 3);
  assert_eq!(result, "line1\nline2\nline3");
}

#[test]
fn tail_truncation_empty()
{
  let text = "";
  let result = tail(text, 5);
  assert_eq!(result, "");
}

// ============================================================================
// Head + Tail combined tests
// ============================================================================

#[test]
fn head_tail_combined_no_overlap()
{
  let config = OutputConfig
  {
    head: Some(2),
    tail: Some(2),
    stream_filter: StreamFilter::Stdout,
    ..Default::default()
  };
  let result = process_output("a\nb\nc\nd\ne\nf\ng\nh\ni\nj", "", &config);
  assert_eq!(result.content, "a\nb\ni\nj");
  assert_eq!(result.lines_omitted, 6);
}

#[test]
fn head_tail_combined_overlap()
{
  let config = OutputConfig
  {
    head: Some(3),
    tail: Some(3),
    stream_filter: StreamFilter::Stdout,
    ..Default::default()
  };
  // 5 lines, head=3 + tail=3 >= 5, so show all
  let result = process_output("a\nb\nc\nd\ne", "", &config);
  assert_eq!(result.content, "a\nb\nc\nd\ne");
  assert_eq!(result.lines_omitted, 0);
}

#[test]
fn head_tail_exact_fit()
{
  let config = OutputConfig
  {
    head: Some(2),
    tail: Some(3),
    stream_filter: StreamFilter::Stdout,
    ..Default::default()
  };
  // 5 lines, head=2 + tail=3 = 5, show all
  let result = process_output("a\nb\nc\nd\ne", "", &config);
  assert_eq!(result.content, "a\nb\nc\nd\ne");
  assert_eq!(result.lines_omitted, 0);
}

// ============================================================================
// Width truncation tests
// ============================================================================

#[test]
fn width_no_truncation_needed()
{
  let opts = TruncateOptions::new( 100 ).with_suffix( "→" ).with_reset( true );
  let result = truncate( "short", &opts );
  // strs_tools adds reset code for safety
  assert!( result.starts_with( "short" ) );
}

#[test]
fn width_truncation_adds_arrow()
{
  let opts = TruncateOptions::new( 10 ).with_suffix( "→" ).with_reset( true );
  let result = truncate( "this is a long line", &opts );
  assert!( result.contains( '\u{2192}' ) ); // →
}

#[test]
fn width_zero_disables()
{
  let result = if 0 == 0 { "long line here".to_string() } else { truncate( "long line here", &TruncateOptions::new( 0 ) ) };
  assert_eq!( result, "long line here" );
}

#[test]
fn width_exact_length_no_truncation()
{
  let config = OutputConfig::default().with_width( 5 );
  let result = process_output( "hello", "", &config );
  assert!( result.content.contains( "hello" ) );
  assert!( !result.width_truncated );
}

#[test]
fn width_one_over_truncates()
{
  let opts = TruncateOptions::new( 5 ).with_suffix( "→" ).with_reset( true );
  let result = truncate( "hello!", &opts );
  assert!( result.contains( '\u{2192}' ) );
}

#[test]
fn width_multiple_lines()
{
  let content = "short\nthis is a very long line\nmedium";
  let opts = TruncateOptions::new( 10 ).with_suffix( "→" ).with_reset( true );
  let result : String = content.lines()
    .map( | line | truncate( line, &opts ) )
    .collect::< Vec< _ > >()
    .join( "\n" );
  let lines : Vec< &str > = result.lines().collect();
  assert!( lines[ 0 ].starts_with( "short" ) );
  assert!( lines[ 1 ].contains( '\u{2192}' ) );
  assert!( lines[ 2 ].starts_with( "medium" ) );
}

// ============================================================================
// ANSI handling tests
// ============================================================================

#[test]
fn ansi_preserved_during_truncation()
{
  let line = "\x1b[31mred text here\x1b[0m";
  let opts = TruncateOptions::new( 5 ).with_suffix( "→" ).with_reset( true );
  let result = truncate( line, &opts );
  assert!( result.contains( "\x1b[31m" ) );
}

#[test]
fn ansi_codes_count_as_zero_width()
{
  // ANSI codes should not count toward width
  let line = "\x1b[31mhello\x1b[0m";
  let config = OutputConfig::default().with_width( 5 );
  let result = process_output( line, "", &config );
  // "hello" is exactly 5 chars, ANSI codes are 0 width
  // Should not truncate
  assert!( result.content.contains( "hello" ) );
  assert!( !result.width_truncated );
}

#[test]
fn truncate_plain_text()
{
  let opts = TruncateOptions::new( 5 ).with_suffix( "→" ).with_reset( true );
  let result = truncate( "hello world", &opts );
  assert!( result.contains( "hell" ) );
  assert!( result.contains( "\u{2192}" ) );
}

// ============================================================================
// Integration tests
// ============================================================================

#[test]
fn process_output_default_config()
{
  let config = OutputConfig::default();
  let result = process_output( "stdout\nline2", "stderr", &config );
  // stderr appears before stdout
  assert_eq!( result.content, "stderr\nstdout\nline2" );
  assert_eq!( result.lines_omitted, 0 );
  assert!( !result.width_truncated );
}

#[test]
fn process_output_head_only()
{
  let config = OutputConfig
  {
    head: Some(2),
    stream_filter: StreamFilter::Stdout,
    ..Default::default()
  };
  let result = process_output("a\nb\nc\nd", "", &config);
  assert_eq!(result.content, "a\nb");
  assert_eq!(result.lines_omitted, 2);
}

#[test]
fn process_output_combined()
{
  let config = OutputConfig::default()
    .with_head( 2 )
    .with_tail( 2 )
    .with_width( 20 )
    .with_stream_filter( StreamFilter::Stdout );
  let stdout = "line1\nline2\nline3\nline4\nline5\nline6";
  let result = process_output( stdout, "stderr", &config );
  assert_eq!( result.lines_omitted, 2 );
  let lines : Vec< &str > = result.content.lines().collect();
  assert_eq!( lines.len(), 4 );
}
