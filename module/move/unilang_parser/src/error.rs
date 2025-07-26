//! Defines error types for the unilang instruction parser.

#![allow(clippy::std_instead_of_alloc)]
#![allow(clippy::std_instead_of_core)]

use core::fmt;

/// Represents a span of characters in the source string.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StrSpan {
  /// Starting byte index of the span.
  pub start: usize,
  /// Ending byte index of the span (exclusive).
  pub end: usize,
}

/// Represents a location in the source string.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SourceLocation {
  /// A span of characters.
  /// Represents a span within a string, defined by start and end byte indices.
  StrSpan {
    /// The starting byte index of the span.
    start: usize,
    /// The ending byte index of the span.
    end: usize,
  },
  /// No specific location.
  None,
}

impl SourceLocation {
  /// Returns the start index of the source location.
  #[must_use]
  pub fn start(&self) -> usize {
    match self {
      SourceLocation::StrSpan { start, .. } => *start,
      SourceLocation::None => 0,
    }
  }

  /// Returns the end index of the source location.
  #[must_use]
  pub fn end(&self) -> usize {
    match self {
      SourceLocation::StrSpan { end, .. } => *end,
      SourceLocation::None => 0,
    }
  }
}
impl fmt::Display for SourceLocation {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      SourceLocation::StrSpan { start, end } => write!(f, "StrSpan {{ start: {start}, end: {end} }}"),
      SourceLocation::None => write!(f, "None"),
    }
  }
}

/// Kinds of parsing errors.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorKind {
  /// Syntax error.
  Syntax(String),
  /// Invalid escape sequence in a string.
  InvalidEscapeSequence(String),
  /// An instruction segment is empty (e.g., `;;` with nothing between).
  EmptyInstructionSegment,
  /// Trailing delimiter error.
  TrailingDelimiter,
  /// Unknown error.
  Unknown,
}

/// Represents a parsing error with its kind and location.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParseError {
  /// The kind of error.
  pub kind: ErrorKind,
  /// The location in the source string where the error occurred.
  pub location: Option<SourceLocation>,
}

impl ParseError {
  /// Creates a new `ParseError`.
  #[must_use]
  pub fn new(kind: ErrorKind, location: SourceLocation) -> Self {
    Self {
      kind,
      location: Some(location),
    }
  }
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self.kind {
      ErrorKind::InvalidEscapeSequence(s) => write!(f, "Invalid escape sequence: {s}")?,
      ErrorKind::EmptyInstructionSegment => write!(f, "Empty instruction segment")?,
      ErrorKind::TrailingDelimiter => write!(f, "Trailing delimiter")?,
      _ => write!(f, "{:?}", self.kind)?,
    }
    if let Some(location) = &self.location {
      write!(f, " at {location}")?;
    }
    Ok(())
  }
}

impl std::error::Error for ParseError {}
