#![ cfg( feature = "output" ) ]
//! CLI output processing tests.
//!
//! Ported from `unilang::output` module tests.
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
//! | merge | stream ordering (stderr before stdout) | ✓ |
//! | integration | tail combined with width | ✓ |
//! | integration | StreamFilter::Stdout with head (FT-36) | ✓ |
//! | integration | head+tail+width triple combination (FT-37) | ✓ |
//! | width | empty suffix — no marker appended (FT-38) | ✓ |
//! | integration | empty stdout with stderr and head (FT-39) | ✓ |
//! | width | zero disables truncation with head active (FT-40) | ✓ |
//! | merge | both empty inputs — infallible (AP-11) | ✓ |
//! | invariant | sole runtime dep is strs_tools (IN-3) | ✓ |
//! | OutputConfig | width=0 activates has_processing, not is_default (AP-12) | ✓ |
//! | StreamFilter | merge_streams Stdout-only direct call (AP-14) | ✓ |
//! | StreamFilter | merge_streams Stderr-only direct call (AP-15) | ✓ |
//! | unicode | unicode_aware=false: char count not byte count (FT-43) | ✓ |
//! | width | line exactly 1 over max_width is truncated (FT-44) | ✓ |
//! | integration | StreamFilter::Stderr combined with head (FT-42) | ✓ |

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

// AP-12: with_width(0) stores Some(0) not None — processing flag is set even though
// truncation is disabled at runtime. Distinguishes "explicitly set to zero" from "not set".
#[ test ]
fn output_config_with_width_zero_has_processing()
{
  let config = OutputConfig::default().with_width( 0 );
  assert!(
    config.has_processing(),
    "AP-12: with_width(0) must set has_processing() == true — Some(0) is not None"
  );
  assert!(
    !config.is_default(),
    "AP-12: with_width(0) must set is_default() == false — width deviates from None default"
  );
  let result = process_output( "this is a very long line indeed", "", &config );
  assert!(
    !result.width_truncated,
    "AP-12: width=0 short-circuits truncation at runtime — width_truncated must be false even with Some(0) stored"
  );
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

/// ## Critical Bug Fix: Width Truncation Boundary Detection (BUG-005)
///
/// # Root Cause
/// `apply_width_filtering` delegated unconditionally to `truncate_lines()`, which reserves
/// suffix space *within* `max_width`. For input where `visual_len(line) == max_width`,
/// truncation fired even though the line fits exactly — `"hello"` at `max_width=5`
/// produced `"hell→"` instead of passing through unchanged.
///
/// # Why Not Caught
/// No regression test existed before migration. The code was ported from `strs_tools::output`
/// which delegated unconditionally to `truncate_lines()` without testing the exact-width
/// boundary case. Visual inspection of typical usage (long lines) passed; failure required
/// input where `visible_len(line) == max_width`, a case not covered before extraction.
///
/// # Fix Applied
/// Boundary guard added in `strs_tools::truncate_lines` — invokes truncation only when
/// `visual_len(line) > max_width`. Lines that fit exactly (`== max_width`) pass through.
///
/// # Prevention
/// Always validate visible width before truncating ANSI-aware text. Never assume
/// `truncate()` handles exact-width detection internally.
///
/// # Pitfall
/// `TruncateOptions` is designed for unconditional truncation (reserves suffix space).
/// Consumers must perform boundary detection before calling truncate functions.
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

// FT-44: Line exactly one char wider than max_width is truncated (contrast FT-11: == max_width is not)
#[ test ]
fn width_one_over_boundary()
{
  // 11-char input at width=10: one char over the limit — truncation must fire.
  let config = OutputConfig::default().with_width( 10 );
  let result = process_output( "01234567890", "", &config );
  assert!(
    result.width_truncated,
    "FT-44: 11 chars at width=10 exceeds limit by 1 — truncation fires at > max_width (contrast FT-11: == max_width does not)"
  );
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
  assert!(
    result.width_truncated,
    "FT-33: retained line 'err2 is also long' (18 chars) exceeds width=15 — width_truncated must be true"
  );
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

// AP-14: merge_streams with Stdout filter returns only stdout — stderr is discarded entirely
#[ test ]
fn merge_streams_stdout_only()
{
  let result = merge_streams( "hello", "world", &StreamFilter::Stdout );
  assert_eq!( result, "hello", "AP-14: Stdout filter returns only stdout; stderr discarded" );
}

// AP-15: merge_streams with Stderr filter returns only stderr — stdout is discarded entirely
#[ test ]
fn merge_streams_stderr_only()
{
  let result = merge_streams( "hello", "world", &StreamFilter::Stderr );
  assert_eq!( result, "world", "AP-15: Stderr filter returns only stderr; stdout discarded" );
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
  assert!(
    !result.content.contains( '→' ),
    "FT-17: at width=1 the '→' suffix (1 char) exceeds the 1-char budget and must be omitted, got:\n{:?}", result.content
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

// FT-43: unicode_aware=false uses char count (1), not byte count (2), for "é" at width=1
#[ test ]
fn unicode_aware_false_char_not_byte()
{
  // "é" is U+00E9: 1 char, 2 bytes. unicode_aware=false uses char count (1), not byte count (2).
  // At width=1 the char count equals the limit — no truncation should fire.
  let config = OutputConfig::default().with_width( 1 );
  let result = process_output( "é", "", &config );
  assert!(
    !result.width_truncated,
    "FT-43: unicode_aware=false counts chars (1), not bytes (2); width=1 matches char count — no truncation"
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

// ============================================================================
// FT-36..FT-40 — combined and novel scenario tests
// ============================================================================

// FT-36: StreamFilter::Stdout combined with head — stderr discarded before head filtering
#[ test ]
fn stdout_filter_with_head()
{
  let config = OutputConfig::default()
    .with_stream_filter( StreamFilter::Stdout )
    .with_head( 2 );
  let result = process_output( "a\nb\nc", "err", &config );
  assert!(
    result.content.contains( "a" ) && result.content.contains( "b" ),
    "first 2 stdout lines must be retained, got:\n{:?}", result.content
  );
  assert!(
    !result.content.contains( "c" ),
    "3rd stdout line must be omitted by head(2), got:\n{:?}", result.content
  );
  assert!(
    !result.content.contains( "err" ),
    "stderr must be discarded entirely by StreamFilter::Stdout, got:\n{:?}", result.content
  );
  assert_eq!(
    result.lines_omitted, 1,
    "head(2) on 3-line stdout-only stream must omit 1 line"
  );
}

// FT-42: StreamFilter::Stderr combined with head — stdout discarded, head applied to stderr-only stream
#[ test ]
fn stderr_filter_with_head()
{
  let config = OutputConfig::default()
    .with_stream_filter( StreamFilter::Stderr )
    .with_head( 2 );
  let result = process_output( "x", "err1\nerr2\nerr3", &config );
  assert!( !result.content.contains( "x" ),    "FT-42: stdout discarded by Stderr filter" );
  assert!(  result.content.contains( "err1" ), "FT-42: err1 retained by head(2)" );
  assert!(  result.content.contains( "err2" ), "FT-42: err2 retained by head(2)" );
  assert!( !result.content.contains( "err3" ), "FT-42: err3 dropped by head(2)" );
  assert_eq!(
    result.lines_omitted, 1,
    "FT-42: one line omitted by head(2) on 3-line stderr-only stream"
  );
}

// FT-37: head+tail+width all three limits active simultaneously
#[ test ]
fn head_tail_width_triple_combination()
{
  let input = "abcdefghij\nklmnopqrst\nuvwxyzabcd\nefghijklmn\nopqrstuvwx\nyzabcdefgh";
  let config = OutputConfig::default()
    .with_head( 2 )
    .with_tail( 2 )
    .with_width( 8 );
  let result = process_output( input, "", &config );
  assert_eq!(
    result.lines_omitted, 2,
    "head(2)+tail(2) on 6-line input must omit 2 middle lines"
  );
  assert!(
    result.width_truncated,
    "all retained lines exceed 8 chars — width_truncated must be true"
  );
  let lines : Vec< &str > = result.content.lines().collect();
  assert_eq!(
    lines.len(), 4,
    "head(2)+tail(2) must retain exactly 4 lines, got {} lines:\n{:?}", lines.len(), result.content
  );
}

// FT-38: Empty suffix — truncated line ends at max_width with no marker appended
#[ test ]
fn width_empty_suffix_no_marker()
{
  let config = OutputConfig::default()
    .with_width( 10 )
    .with_suffix( "" );
  let result = process_output( "01234567890123456789", "", &config );
  assert!(
    result.width_truncated,
    "20-char line with width=10 must trigger truncation"
  );
  assert!(
    result.content.starts_with( "0123456789" ),
    "truncated content must start with first 10 visible chars, got:\n{:?}", result.content
  );
  assert!(
    !result.content.contains( '→' ),
    "empty suffix must produce no truncation marker — '→' must not appear, got:\n{:?}", result.content
  );
}

// FT-39: Empty stdout with non-empty stderr and active head limit
#[ test ]
fn empty_stdout_stderr_with_head()
{
  let config = OutputConfig::default().with_head( 2 );
  let result = process_output( "", "err1\nerr2\nerr3", &config );
  assert!(
    result.content.contains( "err1" ) && result.content.contains( "err2" ),
    "first 2 stderr lines must be retained, got:\n{:?}", result.content
  );
  assert!(
    !result.content.contains( "err3" ),
    "3rd stderr line must be omitted by head(2), got:\n{:?}", result.content
  );
  assert_eq!(
    result.lines_omitted, 1,
    "head(2) on 3-line stderr-only merged stream must omit 1 line"
  );
}

// FT-40: width=0 disables truncation even when head filtering is active
#[ test ]
fn width_zero_with_head()
{
  let config = OutputConfig::default()
    .with_width( 0 )
    .with_head( 2 );
  let result = process_output( "longline1\nlongline2\nlongline3", "", &config );
  let lines : Vec< &str > = result.content.lines().collect();
  assert_eq!(
    lines.len(), 2,
    "head(2) must retain first 2 lines, got {} lines:\n{:?}", lines.len(), result.content
  );
  assert_eq!(
    lines[ 0 ], "longline1",
    "first line must be intact (no truncation), got:\n{:?}", lines[ 0 ]
  );
  assert_eq!(
    lines[ 1 ], "longline2",
    "second line must be intact (no truncation), got:\n{:?}", lines[ 1 ]
  );
  assert!(
    !result.width_truncated,
    "width=0 must disable truncation even when head is active"
  );
  assert_eq!(
    result.lines_omitted, 1,
    "head(2) on 3-line input must omit 1 line"
  );
}

// ============================================================================
// API contract tests (AP-11, IN-3)
// ============================================================================

// AP-11: merge_streams is infallible when both inputs are empty
#[ test ]
fn merge_streams_both_empty_infallible()
{
  let result = merge_streams( "", "", &StreamFilter::Both );
  assert_eq!(
    result, "",
    "merge_streams with two empty inputs must return empty string, got:\n{:?}", result
  );
}

// IN-3: strs_tools is the sole runtime dependency of cli_fmt
#[ test ]
fn test_strs_tools_sole_runtime_dependency()
{
  let cargo = include_str!( "../Cargo.toml" );
  // strs_tools must be present
  assert!(
    cargo.contains( "strs_tools" ),
    "cli_fmt Cargo.toml must declare strs_tools as a dependency"
  );
  // No other crate may appear in [dependencies]
  // Extract the [dependencies] section and confirm only strs_tools is listed
  let deps_section = cargo
    .split( "[dependencies]" )
    .nth( 1 )
    .unwrap_or( "" )
    .split( "\n[" )
    .next()
    .unwrap_or( "" );
  let dep_lines : Vec< &str > = deps_section
    .lines()
    .filter( | l | !l.trim().is_empty() && !l.trim_start().starts_with( '#' ) )
    .collect();
  assert_eq!(
    dep_lines.len(), 1,
    "cli_fmt must have exactly one runtime dependency (strs_tools), found {} dep lines:\n{:?}",
    dep_lines.len(),
    dep_lines
  );
  assert!(
    dep_lines[ 0 ].starts_with( "strs_tools" ),
    "the sole runtime dependency must be strs_tools, got:\n{:?}", dep_lines[ 0 ]
  );
}
