#![ cfg( feature = "output" ) ]
//! CLI output processing tests.
//!
//! Ported from `unilang::output` module tests.
//!
//! ## Critical Bug Fix: Width Truncation Boundary Detection (BUG-005)
//!
//! **Bug (pre-migration):** Text exactly matching `max_width` was incorrectly truncated.
//!
//! **Root Cause:** `ansi::truncate()` reserves space for the suffix *within* `max_width`.
//! For example, with `max_width=5` and suffix="→":
//! - Input: "hello" (5 visible chars)
//! - Old behavior: Truncated to "hell→" (incorrectly assumed truncation needed)
//! - Correct behavior: Should return "hello" unchanged (fits exactly)
//!
//! **Why Not Caught:** No regression test existed before migration. The code was ported
//! from `strs_tools::output`, which delegated unconditionally to `truncate_lines()` without
//! testing the exact-width boundary case. Visual inspection of typical usage (long lines)
//! passed; the failure required input where `visible_len(line) == max_width`, a case not
//! covered by any existing test at the time of extraction.
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
//! **Bug Reproducer (BUG-005):**
//! - `width_exact_boundary`: Line exactly at `max_width` (`len == max_width`) not truncated — precise reproducer
//!   See `task/bug/closed/005_width_truncation_boundary.md`
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
//! | width | exact boundary (len == max_width) | ✓ |
//! | ANSI | preserves escape codes | ✓ |
//! | ANSI | adds reset on truncate | ✓ |
//! | integration | combined operations | ✓ |
//! | integration | head lines_omitted via process_output | ✓ |
//! | is_default | stream_filter discriminant | ✓ |
//! | is_default | width_suffix discriminant | ✓ |
//! | is_default | unicode_aware discriminant | ✓ |
//! | boundary | head(0) produces empty | ✓ |
//! | boundary | tail(0) produces empty | ✓ |
//! | boundary | width=1 truncates | ✓ |
//! | unicode | unicode_aware=true code path | ✓ |
//! | merge | stderr trailing newline no double-newline | ✓ |
//! | merge | both streams trailing newline no double-newline | ✓ |
//! | merge | stdout trailing newline with separator | ✓ |
//! | config | OutputConfig::new() matches default | ✓ |
//! | OutputConfig | has_processing for tail | ✓ |
//! | OutputConfig | has_processing for width | ✓ |
//! | StreamFilter | empty stdout in Both mode | ✓ |
//! | StreamFilter | empty stderr in Both mode | ✓ |
//! | StreamFilter | both streams empty | ✓ |
//! | head | exact count returns all | ✓ |
//! | head | on empty input | ✓ |
//! | tail | exact count returns all | ✓ |
//! | tail | on empty input | ✓ |
//! | head+tail | sum equals total | ✓ |
//! | width | custom suffix | ✓ |
//! | integration | combined streams+head+width | ✓ |
//! | is_default | tail discriminant | ✓ |
//! | is_default | width discriminant | ✓ |

use cli_fmt::output::*;
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

// test_kind: bug_reproducer(BUG-006)
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
  assert_eq!( result.content,       "" );
  assert_eq!( result.lines_omitted, 0  );
  assert!(    !result.width_truncated  );
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

// BUG-005 task/bug/closed/005_width_truncation_boundary.md — exact-width line incorrectly truncated
// test_kind: bug_reproducer(BUG-005)
#[ test ]
fn width_exact_boundary()
{
  // Line of exactly max_width visible chars must NOT be truncated.
  // This is the precise edge case of the original boundary detection bug:
  // truncate() was invoked even when visual_len(line) == max_width.
  let config = OutputConfig::default().with_width( 10 );
  let result = process_output( "0123456789", "", &config );
  assert!( result.content.starts_with( "0123456789" ) );
  assert!( !result.width_truncated );
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
  // Verify the LAST two lines were retained (not the first two)
  assert!(
    result.content.contains( "line3" ),
    "tail(2) must retain line3 (3rd line), got:\n{}", result.content
  );
  assert!(
    result.content.contains( "line4" ),
    "tail(2) must retain line4 (4th line), got:\n{}", result.content
  );
  assert!(
    !result.content.contains( "line1" ),
    "tail(2) must drop line1 (1st line), got:\n{}", result.content
  );
  assert!(
    !result.content.contains( "line2" ),
    "tail(2) must drop line2 (2nd line), got:\n{}", result.content
  );
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

/// ## Critical Bug Fix: Stderr Stream Ordering (BUG-006)
///
/// # Root Cause
/// `merge_streams` concatenated stdout before stderr, following alphabetical
/// parameter order rather than CLI convention (errors visible first).
///
/// # Why Not Caught
/// No test asserted the relative order of stderr vs stdout in merged output.
/// Existing tests checked content presence but not stream position.
///
/// # Fix Applied
/// Reversed concatenation order in `merge_streams` `Both` arm — stderr is
/// now appended first, stdout second.
///
/// # Prevention
/// Any function merging stdout and stderr must place stderr before stdout.
/// Add ordering assertions, not just content-presence checks.
///
/// # Pitfall
/// Stream ordering is easy to overlook in tests. Always assert not just
/// "is content present" but "is content in correct order".
// BUG-006 task/bug/closed/006_stderr_stream_ordering.md — stdout was placed before stderr
// test_kind: bug_reproducer(BUG-006)
#[ test ]
fn merge_streams_ordering()
{
  // Test that stderr appears before stdout
  let result = merge_streams( "stdout", "stderr", &StreamFilter::Both );
  assert_eq!( result, "stderr\nstdout" );
}

#[ test ]
fn process_output_head_lines_omitted()
{
  // Verify lines_omitted is computed correctly when process_output applies head-only filtering.
  let config = OutputConfig::default().with_head( 2 );
  let result = process_output( "line1\nline2\nline3\nline4\nline5", "", &config );
  assert_eq!( result.lines_omitted, 3 );
  assert_eq!( result.content, "line1\nline2" );
}

// ============================================================================
// is_default() discriminant tests — one test per non-default field
// ============================================================================

// FT-18 / P10: is_default() checks stream_filter == Both
#[ test ]
fn is_default_stream_filter()
{
  let config = OutputConfig::default().with_stream_filter( StreamFilter::Stdout );
  assert!(
    !config.is_default(),
    "config with stream_filter=Stdout must not satisfy is_default()"
  );
}

// FT-18 / P10: is_default() checks width_suffix == "→"
#[ test ]
fn is_default_width_suffix()
{
  let config = OutputConfig::default().with_suffix( "..." );
  assert!(
    !config.is_default(),
    "config with width_suffix='...' must not satisfy is_default()"
  );
}

// FT-18 / P10: is_default() checks !unicode_aware
#[ test ]
fn is_default_unicode_aware()
{
  let config = OutputConfig::default().with_unicode_aware( true );
  assert!(
    !config.is_default(),
    "config with unicode_aware=true must not satisfy is_default()"
  );
}

// FT-24: is_default() checks tail field
#[ test ]
fn is_default_tail()
{
  let config = OutputConfig::default().with_tail( 2 );
  assert!(
    !config.is_default(),
    "config with tail=2 must not satisfy is_default()"
  );
}

// FT-25: is_default() checks width field
#[ test ]
fn is_default_width()
{
  let config = OutputConfig::default().with_width( 5 );
  assert!(
    !config.is_default(),
    "config with width=5 must not satisfy is_default()"
  );
}

// ============================================================================
// Boundary value tests
// ============================================================================

// FT-15 / P11: head(0) retains nothing, reports all lines omitted
#[ test ]
fn head_zero_produces_empty()
{
  let config = OutputConfig::default().with_head( 0 );
  let result = process_output( "a\nb\nc", "", &config );
  assert!(
    result.content.is_empty(),
    "head(0) must produce empty content, got:\n{:?}", result.content
  );
  assert_eq!(
    result.lines_omitted, 3,
    "head(0) on 3-line input must report 3 lines omitted"
  );
}

// FT-16 / P11: tail(0) retains nothing, reports all lines omitted
#[ test ]
fn tail_zero_produces_empty()
{
  let config = OutputConfig::default().with_tail( 0 );
  let result = process_output( "a\nb\nc", "", &config );
  assert!(
    result.content.is_empty(),
    "tail(0) must produce empty content, got:\n{:?}", result.content
  );
  assert_eq!(
    result.lines_omitted, 3,
    "tail(0) on 3-line input must report 3 lines omitted"
  );
}

// FT-17 / P18: width=1 is not short-circuited — truncation fires
#[ test ]
fn width_one_truncates()
{
  // width=0 short-circuits (no truncation); width=1 must not.
  // Note: at width=1 the suffix "→" (visual width 1) would exceed the budget,
  // so the implementation omits it — only width_truncated matters here.
  let config = OutputConfig::default().with_width( 1 );
  let result = process_output( "hello", "", &config );
  assert!(
    result.width_truncated,
    "width=1 must trigger truncation for 'hello' (5 visible chars), got:\n{:?}", result.content
  );
}

// ============================================================================
// Unicode-aware width measurement test
// ============================================================================

// FT-13 / P02: unicode_aware=true exercises the grapheme-based truncation branch
#[ test ]
fn unicode_aware_truncation()
{
  // "café" is 4 visible chars (c-a-f-é); é is a multi-byte UTF-8 code point.
  // With width=3, visual width (4) > limit (3) — truncation must fire.
  // The unicode_aware=true flag routes through truncate_lines_unicode (ansi_unicode feature).
  let config = OutputConfig::default()
    .with_unicode_aware( true )
    .with_width( 3 );
  let result = process_output( "café", "", &config );
  assert!(
    result.width_truncated,
    "unicode_aware=true must trigger truncation when visual width (4) > max_width (3), got:\n{:?}", result.content
  );
}

// ============================================================================
// merge_streams edge cases
// ============================================================================

// FT-14 / P08: stderr ending with '\n' — no double-newline separator inserted
#[ test ]
fn merge_streams_stderr_trailing_newline()
{
  // When stderr ends with '\n', the newline already serves as a stream boundary.
  // merge_streams must NOT insert an additional '\n', avoiding "err\n\nout".
  let result = merge_streams( "out", "err\n", &StreamFilter::Both );
  assert_eq!(
    result, "err\nout",
    "stderr with trailing newline must not produce double-newline separator, got:\n{result:?}"
  );
  assert!(
    !result.contains( "\n\n" ),
    "result must not contain double-newline sequence, got:\n{result:?}"
  );
}

// ============================================================================
// merge_streams edge cases (continued)
// ============================================================================

// When both streams end with '\n', no double-newline separator should appear.
// The separator logic only fires when stderr does NOT end with '\n'.
#[ test ]
fn merge_streams_both_trailing_newlines_no_double_newline()
{
  let result = merge_streams( "out\n", "err\n", &StreamFilter::Both );
  assert_eq!(
    result, "err\nout\n",
    "both streams with trailing newlines must not produce double-newline, got:\n{result:?}"
  );
  assert!(
    !result.contains( "\n\n" ),
    "result must not contain double-newline sequence, got:\n{result:?}"
  );
}

// When stdout ends with '\n' but stderr does not, exactly one separator is added.
#[ test ]
fn merge_streams_stdout_trailing_newline_separator()
{
  let result = merge_streams( "out\n", "err", &StreamFilter::Both );
  assert_eq!(
    result, "err\nout\n",
    "stderr without trailing newline must get exactly one newline separator, got:\n{result:?}"
  );
}

// ============================================================================
// OutputConfig::new() equivalence
// ============================================================================

// AP-6 / P16: new() is a named alias for default() — all fields must match
#[ test ]
fn output_config_new_matches_default()
{
  let from_new     = OutputConfig::new();
  let from_default = OutputConfig::default();
  assert_eq!( from_new.head,          from_default.head,          "head field must match" );
  assert_eq!( from_new.tail,          from_default.tail,          "tail field must match" );
  assert_eq!( from_new.width,         from_default.width,         "width field must match" );
  assert_eq!( from_new.width_suffix,  from_default.width_suffix,  "width_suffix field must match" );
  assert_eq!( from_new.stream_filter, from_default.stream_filter, "stream_filter field must match" );
  assert_eq!( from_new.unicode_aware, from_default.unicode_aware, "unicode_aware field must match" );
  assert!(
    from_new.is_default(),
    "OutputConfig::new() must satisfy is_default()"
  );
}
