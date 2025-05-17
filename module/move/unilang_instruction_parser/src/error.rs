//! Error types for the unilang instruction parser.

use std::fmt;
// strs_tools::string::split::SplitIterator does not return Result, so no direct error types to import for From impl.
// Errors like unterminated quotes will be handled by unilang_instruction_parser's analysis phase.

/// Represents the location of a parsing error.
#[derive(Debug, PartialEq, Clone)]
pub enum SourceLocation {
    /// Location within a single string input.
    StrSpan { start: usize, end: usize },
    /// Location within a segment of a slice input.
    SliceSegment {
        segment_index: usize,
        start_in_segment: usize,
        end_in_segment: usize,
    },
}

/// Represents the kind of parsing error.
#[derive(Debug)]
pub enum ErrorKind {
    // /// Error originating from the underlying itemizer. // Removed as SplitIterator doesn't return Result<Item, Error>
    // Itemization(StrsItemizerErrorKind),
    /// General syntax error detected by unilang_instruction_parser.
    Syntax(String),
    /// Unterminated quoted string.
    UnterminatedQuote,
    /// Invalid escape sequence within a string.
    InvalidEscapeSequence,
}

/// Represents a parsing error with its kind and location.
#[derive(Debug)]
pub struct ParseError {
    pub kind: ErrorKind,
    pub location: Option<SourceLocation>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            // ErrorKind::Itemization(kind) => write!(f, "Itemization error: {}", kind), // Removed
            ErrorKind::Syntax(msg) => write!(f, "Syntax error: {}", msg),
            ErrorKind::UnterminatedQuote => write!(f, "Syntax error: Unterminated quote"),
            ErrorKind::InvalidEscapeSequence => write!(f, "Syntax error: Invalid escape sequence"),
        }?;
        if let Some(loc) = &self.location {
            match loc {
                SourceLocation::StrSpan { start, end } => {
                    write!(f, " at bytes {}-{}", start, end)?;
                }
                SourceLocation::SliceSegment { segment_index, start_in_segment, end_in_segment } => {
                    write!(f, " in segment {} at bytes {}-{}", segment_index, start_in_segment, end_in_segment)?;
                }
            }
        }
        Ok(())
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Since ErrorKind variants are simple for now, they don't wrap other errors.
        // If Itemization was wrapping a Box<dyn Error>, this would be relevant.
        None
    }
}

// The From<StrsItemizerParseError> is removed because strs_tools::string::split::SplitIterator
// does not return a Result<_, StrsItemizerParseError>. Errors like unterminated quotes
// will be detected and reported by unilang_instruction_parser's own logic.