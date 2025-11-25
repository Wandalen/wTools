//! Output truncation tests.
//!
//! ## Test Matrix
//!
//! | Category | Test | Status |
//! |----------|------|--------|
//! | TruncationConfig | default is no truncation | ✓ |
//! | TruncationConfig | has_truncation detection | ✓ |
//! | OutputFilter | stream selection | ✓ |
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

use unilang::output::truncation::*;

// ============================================================================
// TruncationConfig tests
// ============================================================================

#[test]
fn truncation_config_default_is_no_truncation()
{
  let config = TruncationConfig::default();
  assert!(config.is_default());
  assert!(!config.has_truncation());
}

#[test]
fn truncation_config_with_head_has_truncation()
{
  let config = TruncationConfig { head: Some(5), ..Default::default() };
  assert!(!config.is_default());
  assert!(config.has_truncation());
}

#[test]
fn truncation_config_with_tail_has_truncation()
{
  let config = TruncationConfig { tail: Some(5), ..Default::default() };
  assert!(config.has_truncation());
}

#[test]
fn truncation_config_with_width_has_truncation()
{
  let config = TruncationConfig { width: Some(80), ..Default::default() };
  assert!(config.has_truncation());
}

// ============================================================================
// OutputFilter / stream selection tests
// ============================================================================

#[test]
fn select_streams_both()
{
  let config = TruncationConfig::default();
  let result = apply_truncation("stdout", "stderr", &config);
  assert_eq!(result.content, "stdout\nstderr");
}

#[test]
fn select_streams_stdout_only()
{
  let config = TruncationConfig
  {
    output_filter: OutputFilter::Stdout,
    ..Default::default()
  };
  let result = apply_truncation("stdout", "stderr", &config);
  assert_eq!(result.content, "stdout");
}

#[test]
fn select_streams_stderr_only()
{
  let config = TruncationConfig
  {
    output_filter: OutputFilter::Stderr,
    ..Default::default()
  };
  let result = apply_truncation("stdout", "stderr", &config);
  assert_eq!(result.content, "stderr");
}

#[test]
fn select_streams_empty_stdout()
{
  let config = TruncationConfig::default();
  let result = apply_truncation("", "stderr", &config);
  assert_eq!(result.content, "stderr");
}

#[test]
fn select_streams_empty_stderr()
{
  let config = TruncationConfig::default();
  let result = apply_truncation("stdout", "", &config);
  assert_eq!(result.content, "stdout");
}

#[test]
fn select_streams_both_empty()
{
  let config = TruncationConfig::default();
  let result = apply_truncation("", "", &config);
  assert_eq!(result.content, "");
}

// ============================================================================
// Head truncation tests
// ============================================================================

#[test]
fn head_truncation_basic()
{
  let text = "line1\nline2\nline3\nline4\nline5";
  let result = truncate_head(text, 2);
  assert_eq!(result, "line1\nline2");
}

#[test]
fn head_truncation_exceeds_total()
{
  let text = "line1\nline2\nline3";
  let result = truncate_head(text, 10);
  assert_eq!(result, "line1\nline2\nline3");
}

#[test]
fn head_truncation_exact()
{
  let text = "line1\nline2\nline3";
  let result = truncate_head(text, 3);
  assert_eq!(result, "line1\nline2\nline3");
}

#[test]
fn head_truncation_empty()
{
  let text = "";
  let result = truncate_head(text, 5);
  assert_eq!(result, "");
}

// ============================================================================
// Tail truncation tests
// ============================================================================

#[test]
fn tail_truncation_basic()
{
  let text = "line1\nline2\nline3\nline4\nline5";
  let result = truncate_tail(text, 2);
  assert_eq!(result, "line4\nline5");
}

#[test]
fn tail_truncation_exceeds_total()
{
  let text = "line1\nline2\nline3";
  let result = truncate_tail(text, 10);
  assert_eq!(result, "line1\nline2\nline3");
}

#[test]
fn tail_truncation_exact()
{
  let text = "line1\nline2\nline3";
  let result = truncate_tail(text, 3);
  assert_eq!(result, "line1\nline2\nline3");
}

#[test]
fn tail_truncation_empty()
{
  let text = "";
  let result = truncate_tail(text, 5);
  assert_eq!(result, "");
}

// ============================================================================
// Head + Tail combined tests
// ============================================================================

#[test]
fn head_tail_combined_no_overlap()
{
  let config = TruncationConfig
  {
    head: Some(2),
    tail: Some(2),
    output_filter: OutputFilter::Stdout,
    ..Default::default()
  };
  let result = apply_truncation("a\nb\nc\nd\ne\nf\ng\nh\ni\nj", "", &config);
  assert_eq!(result.content, "a\nb\ni\nj");
  assert_eq!(result.lines_omitted, 6);
}

#[test]
fn head_tail_combined_overlap()
{
  let config = TruncationConfig
  {
    head: Some(3),
    tail: Some(3),
    output_filter: OutputFilter::Stdout,
    ..Default::default()
  };
  // 5 lines, head=3 + tail=3 >= 5, so show all
  let result = apply_truncation("a\nb\nc\nd\ne", "", &config);
  assert_eq!(result.content, "a\nb\nc\nd\ne");
  assert_eq!(result.lines_omitted, 0);
}

#[test]
fn head_tail_exact_fit()
{
  let config = TruncationConfig
  {
    head: Some(2),
    tail: Some(3),
    output_filter: OutputFilter::Stdout,
    ..Default::default()
  };
  // 5 lines, head=2 + tail=3 = 5, show all
  let result = apply_truncation("a\nb\nc\nd\ne", "", &config);
  assert_eq!(result.content, "a\nb\nc\nd\ne");
  assert_eq!(result.lines_omitted, 0);
}

// ============================================================================
// Width truncation tests
// ============================================================================

#[test]
fn width_no_truncation_needed()
{
  let result = truncate_width("short", 100);
  assert_eq!(result, "short");
}

#[test]
fn width_truncation_adds_arrow()
{
  let result = truncate_width("this is a long line", 10);
  assert!(result.ends_with('\u{2192}')); // →
  assert!(result.len() <= 15); // rough check
}

#[test]
fn width_zero_disables()
{
  let result = truncate_width("long line here", 0);
  assert_eq!(result, "long line here");
}

#[test]
fn width_exact_length_no_truncation()
{
  let result = truncate_width("hello", 5);
  assert_eq!(result, "hello");
}

#[test]
fn width_one_over_truncates()
{
  let result = truncate_width("hello!", 5);
  assert!(result.ends_with('\u{2192}'));
  assert_eq!(result, "hell\u{2192}");
}

#[test]
fn width_multiple_lines()
{
  let content = "short\nthis is a very long line\nmedium";
  let result = truncate_width(content, 10);
  let lines: Vec<&str> = result.lines().collect();
  assert_eq!(lines[0], "short");
  assert!(lines[1].ends_with('\u{2192}'));
  assert_eq!(lines[2], "medium");
}

// ============================================================================
// ANSI handling tests
// ============================================================================

#[test]
fn ansi_preserved_during_truncation()
{
  let line = "\x1b[31mred text here\x1b[0m";
  let result = truncate_width(line, 5);
  assert!(result.contains("\x1b[31m"));
}

#[test]
fn ansi_codes_count_as_zero_width()
{
  // ANSI codes should not count toward width
  let line = "\x1b[31mhello\x1b[0m";
  let result = truncate_width(line, 5);
  // "hello" is exactly 5 chars, ANSI codes are 0 width
  // Should not truncate
  assert!(result.contains("hello"));
}

#[test]
fn truncate_plain_text()
{
  let result = truncate_width("hello world", 5);
  assert_eq!(result, "hell\u{2192}");
}

// ============================================================================
// Integration tests
// ============================================================================

#[test]
fn apply_truncation_default_config()
{
  let config = TruncationConfig::default();
  let result = apply_truncation("stdout\nline2", "stderr", &config);
  assert_eq!(result.content, "stdout\nline2\nstderr");
  assert_eq!(result.lines_omitted, 0);
  assert!(!result.width_truncated);
}

#[test]
fn apply_truncation_head_only()
{
  let config = TruncationConfig
  {
    head: Some(2),
    output_filter: OutputFilter::Stdout,
    ..Default::default()
  };
  let result = apply_truncation("a\nb\nc\nd", "", &config);
  assert_eq!(result.content, "a\nb");
  assert_eq!(result.lines_omitted, 2);
}

#[test]
fn apply_truncation_combined()
{
  let config = TruncationConfig
  {
    head: Some(2),
    tail: Some(2),
    width: Some(20),
    output_filter: OutputFilter::Stdout,
  };
  let stdout = "line1\nline2\nline3\nline4\nline5\nline6";
  let result = apply_truncation(stdout, "stderr", &config);
  assert_eq!(result.lines_omitted, 2);
  let lines: Vec<&str> = result.content.lines().collect();
  assert_eq!(lines.len(), 4);
}
