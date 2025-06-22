//!
//! Input abstraction for the command aggregator parser.
//!

///
/// Represents a location within the input, handling both single strings and slices.
///
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum Location
{
  /// Location within a single string input (byte offset).
  ByteOffset( usize ),
  /// Location within a slice of string segments (segment index, offset within segment).
  SegmentOffset
  (
    usize,
    usize,
  ),
}

///
/// Represents the current state of the input being parsed.
///
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum InputState< 'a >
{
  /// State for a single string input.
  SingleString
  {
    /// The input string.
    input : &'a str,
    /// The current byte offset.
    offset : usize,
  },
  /// State for a slice of string segments input.
  SegmentSlice
  {
    /// The slice of string segments.
    segments : &'a [&'a str],
    /// The current segment index.
    segment_index : usize,
    /// The current byte offset within the segment.
    offset_in_segment : usize,
  },
}

///
/// Provides a unified interface to process input from either a single string or a slice of strings.
///
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct InputAbstraction< 'a >
{
  state : InputState< 'a >,
}

impl< 'a > InputAbstraction< 'a >
{
  ///
  /// Creates a new `InputAbstraction` from a single string.
  ///
  pub fn from_str( input : &'a str ) -> Self
  {
    Self
    {
      state : InputState::SingleString { input, offset : 0 },
    }
  }

  ///
  /// Creates a new `InputAbstraction` from a slice of string segments.
  ///
  pub fn from_segments( segments : &'a [&'a str] ) -> Self
  {
    Self
    {
      state : InputState::SegmentSlice { segments, segment_index : 0, offset_in_segment : 0 },
    }
  }

  // Placeholder methods based on the revised conceptual design.
  // Implementation will be done in a future increment.

  ///
  /// Peeks at the next character without consuming it.
  ///
  pub fn peek_next_char( &self ) -> Option< char >
  {
    // TODO: Implement based on InputState
    // aaa: Placeholder added.
    None
  }

  ///
  /// Consumes and returns the next character.
  ///
  pub fn next_char( &mut self ) -> Option< char >
  {
    // TODO: Implement based on InputState
    // aaa: Placeholder added.
    None
  }

  ///
  /// Peeks at the next full segment (relevant for `&[&str]` input).
  ///
  pub fn peek_next_segment( &self ) -> Option< &'a str >
  {
    // TODO: Implement based on InputState
    // aaa: Placeholder added.
    None
  }

  ///
  /// Consumes and returns the next full segment (relevant for `&[&str]` input).
  ///
  pub fn next_segment( &mut self ) -> Option< &'a str >
  {
    // TODO: Implement based on InputState
    // aaa: Placeholder added.
    None
  }

  ///
  /// Searches for the next occurrence of any of the provided string patterns.
  /// Returns the matched pattern and its location.
  ///
  pub fn find_next_occurrence( &self, _patterns : &'a [&'a str] ) -> Option< ( &'a str, Location ) >
  {
    // TODO: Implement based on InputState and patterns
    // aaa: Placeholder added.
    None
  }

  ///
  /// Consumes the input up to a specified location and returns the consumed slice.
  ///
  pub fn consume_until( &mut self, _location : Location ) -> &'a str
  {
    // TODO: Implement based on InputState and target location
    // aaa: Placeholder added.
    ""
  }

  ///
  /// Consumes a specified number of characters/bytes.
  ///
  pub fn consume_len( &mut self, _len : usize ) -> &'a str
  {
    // TODO: Implement based on InputState and length
    // aaa: Placeholder added.
    ""
  }

  ///
  /// Returns the current parsing location.
  ///
  pub fn current_location( &self ) -> Location
  {
    match &self.state
    {
      InputState::SingleString { offset, .. } => Location::ByteOffset( *offset ),
      InputState::SegmentSlice { segment_index, offset_in_segment, .. } => Location::SegmentOffset( *segment_index, *offset_in_segment ),
    }
  }

  ///
  /// Checks if there is any remaining input.
  ///
  pub fn is_empty( &self ) -> bool
  {
    match &self.state
    {
      InputState::SingleString { input, offset } => *offset >= input.len(),
      InputState::SegmentSlice { segments, segment_index, offset_in_segment } =>
      {
        if *segment_index >= segments.len()
        {
          true
        }
        else
        {
          *offset_in_segment >= segments[ *segment_index ].len()
        }
      }
    }
  }
}

///
/// Represents the type of delimiter found during parsing.
///
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum DelimiterType
{
  /// `::` separator.
  ColonColon,
  /// `;;` separator.
  SemiColonSemiColon,
  /// `?` help operator.
  QuestionMark,
  /// Single quote `'`.
  SingleQuote,
  /// Double quote `"`.
  DoubleQuote,
  /// Whitespace character.
  Whitespace,
}

///
/// Represents a part of the input after splitting by a delimiter.
///
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum InputPart< 'a >
{
  /// A regular string segment.
  Segment( &'a str ),
  /// A recognized delimiter.
  Delimiter( DelimiterType ),
}