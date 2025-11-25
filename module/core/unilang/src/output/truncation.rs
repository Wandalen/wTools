//!
//! Output truncation utilities for CLI applications.
//!
//! Handles head/tail line limiting, width truncation, and stream selection
//! while preserving ANSI escape codes and handling Unicode correctly.
//!
//! # Design Decisions
//!
//! ## ANSI-Aware Width Truncation
//!
//! Terminal output often contains ANSI escape codes for colors. Naive truncation
//! by byte count breaks these codes, causing garbled output. This module:
//! - Parses ANSI escape sequences as invisible (zero-width)
//! - Counts only visible characters toward width limit
//! - Preserves complete escape sequences in output
//! - Adds reset code if truncation occurs mid-formatting
//!
//! ## Unicode Grapheme Clusters
//!
//! Character width is measured in grapheme clusters, not bytes or codepoints.
//! This correctly handles:
//! - Multi-byte UTF-8 (e.g., emojis, CJK characters)
//! - Combining characters (e.g., accents)
//! - Zero-width joiners
//!
//! ## Two-Pass Truncation Algorithm
//!
//! Width truncation uses two passes:
//! 1. First pass: Count total visible characters
//! 2. If within limit: return unchanged (avoid unnecessary work)
//! 3. Second pass: Truncate to (max_width - 1) and add arrow indicator
//!
//! This avoids the off-by-one error of truncating exactly-fitting strings.
//!
//! ## Head + Tail Combination
//!
//! When both head and tail are specified:
//! - If head + tail >= total lines: show all (no overlap handling needed)
//! - Otherwise: show head lines, then tail lines, with omission marker
//!

use unicode_segmentation::UnicodeSegmentation;

/// Configuration for output truncation and filtering.
#[derive(Clone, Debug, Default)]
pub struct TruncationConfig
{
  /// Show only first N lines (None = no limit).
  pub head: Option<usize>,
  /// Show only last N lines (None = no limit).
  pub tail: Option<usize>,
  /// Maximum line width in visible characters (None = no limit).
  pub width: Option<usize>,
  /// Which output stream to show.
  pub output_filter: OutputFilter,
}

/// Which output stream to display.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum OutputFilter
{
  /// Display both stdout and stderr combined.
  #[default]
  Both,
  /// Display only stdout.
  Stdout,
  /// Display only stderr.
  Stderr,
}

/// Result of applying output truncation.
#[derive(Clone, Debug)]
pub struct TruncatedOutput
{
  /// The processed content.
  pub content: String,
  /// Number of lines omitted (for display).
  pub lines_omitted: usize,
  /// Whether output was truncated by width.
  pub width_truncated: bool,
}

impl TruncationConfig
{
  /// Check if any truncation is configured.
  #[inline]
  #[must_use]
  pub fn has_truncation(&self) -> bool
  {
    self.head.is_some() || self.tail.is_some() || self.width.is_some()
  }

  /// Check if default (no modifications).
  #[inline]
  #[must_use]
  pub fn is_default(&self) -> bool
  {
    self.head.is_none()
      && self.tail.is_none()
      && self.width.is_none()
      && self.output_filter == OutputFilter::Both
  }
}

/// Apply output configuration to raw streams.
///
/// # Arguments
///
/// * `stdout` - Standard output content
/// * `stderr` - Standard error content
/// * `config` - Truncation configuration
///
/// # Returns
///
/// Processed output with truncation applied.
#[must_use]
pub fn apply_truncation(
  stdout: &str,
  stderr: &str,
  config: &TruncationConfig
) -> TruncatedOutput
{
  // 1. Select stream(s)
  let content = select_streams(stdout, stderr, &config.output_filter);

  // 2. Apply head/tail
  let (lines_result, omitted) = apply_head_tail(&content, config.head, config.tail);

  // 3. Apply width truncation
  let (final_content, width_truncated) = apply_width_truncation(&lines_result, config.width);

  TruncatedOutput
  {
    content: final_content,
    lines_omitted: omitted,
    width_truncated,
  }
}

/// Truncate to first N lines.
///
/// # Arguments
///
/// * `text` - Input text
/// * `lines` - Number of lines to keep
///
/// # Returns
///
/// Truncated text containing only the first N lines.
#[inline]
#[must_use]
pub fn truncate_head(text: &str, lines: usize) -> String
{
  text.lines()
    .take(lines)
    .collect::<Vec<_>>()
    .join("\n")
}

/// Truncate to last N lines.
///
/// # Arguments
///
/// * `text` - Input text
/// * `lines` - Number of lines to keep
///
/// # Returns
///
/// Truncated text containing only the last N lines.
#[inline]
#[must_use]
pub fn truncate_tail(text: &str, lines: usize) -> String
{
  let all_lines: Vec<_> = text.lines().collect();
  let start = all_lines.len().saturating_sub(lines);
  all_lines[start..].join("\n")
}

/// Truncate each line to max visible width (ANSI and Unicode aware).
///
/// # Arguments
///
/// * `text` - Input text (may be multiline)
/// * `max_width` - Maximum visible characters per line (0 = no limit)
///
/// # Returns
///
/// Text with each line truncated to max_width visible characters.
#[must_use]
pub fn truncate_width(text: &str, max_width: usize) -> String
{
  if max_width == 0
  {
    return text.to_string();
  }

  text.lines()
    .map(|line| truncate_line_ansi_aware(line, max_width).0)
    .collect::<Vec<_>>()
    .join("\n")
}

// ============================================================================
// Internal functions
// ============================================================================

fn select_streams(stdout: &str, stderr: &str, filter: &OutputFilter) -> String
{
  match filter
  {
    OutputFilter::Stdout => stdout.to_string(),
    OutputFilter::Stderr => stderr.to_string(),
    OutputFilter::Both =>
    {
      let mut merged = String::with_capacity(stdout.len() + stderr.len());
      if !stdout.is_empty()
      {
        merged.push_str(stdout);
        // Ensure newline between streams
        if !stdout.ends_with('\n') && !stderr.is_empty()
        {
          merged.push('\n');
        }
      }
      if !stderr.is_empty()
      {
        merged.push_str(stderr);
      }
      merged
    }
  }
}

fn apply_head_tail(content: &str, head: Option<usize>, tail: Option<usize>) -> (String, usize)
{
  if content.is_empty()
  {
    return (String::new(), 0);
  }

  let lines: Vec<&str> = content.lines().collect();
  let total = lines.len();

  let (selected, omitted) = match (head, tail)
  {
    (None, None) => (lines, 0),

    (Some(h), None) =>
    {
      if h >= total
      {
        (lines, 0)
      }
      else
      {
        (lines[..h].to_vec(), total - h)
      }
    }

    (None, Some(t)) =>
    {
      if t >= total
      {
        (lines, 0)
      }
      else
      {
        let skip = total - t;
        (lines[skip..].to_vec(), skip)
      }
    }

    (Some(h), Some(t)) =>
    {
      if h + t >= total
      {
        // Overlap or exact fit - show all
        (lines, 0)
      }
      else
      {
        // Need to omit middle section
        let omit_count = total - h - t;
        let mut result = Vec::with_capacity(h + t);
        result.extend_from_slice(&lines[..h]);
        result.extend_from_slice(&lines[total - t..]);
        (result, omit_count)
      }
    }
  };

  (selected.join("\n"), omitted)
}

fn apply_width_truncation(content: &str, width: Option<usize>) -> (String, bool)
{
  let Some(max_width) = width else
  {
    return (content.to_string(), false);
  };

  if max_width == 0
  {
    return (content.to_string(), false);
  }

  let mut truncated = false;
  let result: Vec<String> = content
    .lines()
    .map(|line|
    {
      let (truncated_line, was_truncated) = truncate_line_ansi_aware(line, max_width);
      if was_truncated
      {
        truncated = true;
      }
      truncated_line
    })
    .collect();

  (result.join("\n"), truncated)
}

/// Truncate a line while preserving ANSI escape codes.
///
/// Returns (truncated_line, was_truncated).
fn truncate_line_ansi_aware(line: &str, max_width: usize) -> (String, bool)
{
  if max_width == 0
  {
    return (line.to_string(), false);
  }

  // Parse line into segments (ANSI codes vs visible text)
  let segments = parse_ansi_segments(line);

  // First pass: count total visible characters
  let mut total_visible = 0;
  for segment in &segments
  {
    if let AnsiSegment::Text(text) = segment
    {
      total_visible += text.graphemes(true).count();
    }
  }

  // No truncation needed if within limit
  if total_visible <= max_width
  {
    return (line.to_string(), false);
  }

  // Second pass: truncate to max_width - 1 and add arrow
  let mut result = String::new();
  let mut visible_count = 0;
  let truncate_at = max_width - 1;

  for segment in segments
  {
    match segment
    {
      AnsiSegment::Code(code) =>
      {
        // Always include ANSI codes (they're invisible)
        result.push_str(code);
      }
      AnsiSegment::Text(text) =>
      {
        // Count grapheme clusters for proper Unicode handling
        for grapheme in text.graphemes(true)
        {
          if visible_count >= truncate_at
          {
            // Add truncation indicator
            result.push('\u{2192}'); // → arrow
            // Add reset code if we were in middle of formatting
            if has_active_formatting(&result)
            {
              result.push_str("\x1b[0m");
            }
            return (result, true);
          }
          result.push_str(grapheme);
          visible_count += 1;
        }
      }
    }
  }

  (result, true)
}

enum AnsiSegment<'a>
{
  Code(&'a str),
  Text(&'a str),
}

fn parse_ansi_segments(line: &str) -> Vec<AnsiSegment<'_>>
{
  let mut segments = Vec::new();
  let mut remaining = line;

  while !remaining.is_empty()
  {
    if remaining.starts_with("\x1b[")
    {
      // Find end of ANSI sequence
      if let Some(end) = remaining[2..].find('m')
      {
        let code_end = end + 3; // Include the 'm'
        segments.push(AnsiSegment::Code(&remaining[..code_end]));
        remaining = &remaining[code_end..];
      }
      else
      {
        // Malformed ANSI code, treat as text
        segments.push(AnsiSegment::Text(&remaining[..1]));
        remaining = &remaining[1..];
      }
    }
    else
    {
      // Find next ANSI code or end
      let next_ansi = remaining.find("\x1b[").unwrap_or(remaining.len());
      if next_ansi > 0
      {
        segments.push(AnsiSegment::Text(&remaining[..next_ansi]));
        remaining = &remaining[next_ansi..];
      }
    }
  }

  segments
}

fn has_active_formatting(s: &str) -> bool
{
  // Simple heuristic: count opens vs resets
  let opens = s.matches("\x1b[").count();
  let resets = s.matches("\x1b[0m").count();
  opens > resets
}
