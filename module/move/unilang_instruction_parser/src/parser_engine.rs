//! Parser for Unilang instructions.
//!
//! This module provides the core logic for parsing Unilang instructions from a string input.
//! It handles tokenization, command path parsing, argument parsing, and error reporting.

use crate::
{
  config::UnilangParserOptions,
  error::{ ErrorKind, ParseError, SourceLocation },
  item_adapter::{ RichItem, UnilangTokenKind },
};
use std::collections::HashMap;
use strs_tools::string::split::{ SplitType, Split };

/// Represents the parsed instruction, including its command path, arguments, and named arguments.
#[ derive( Debug, PartialEq, Eq, Clone ) ]
pub struct GenericInstruction
{
  /// The command path, e.g., `.` or `cmd.subcmd`.
  pub command_path : Vec< String >,
  /// Positional arguments.
  pub arguments : Vec< String >,
  /// Named arguments, mapping name to value.
  pub named_arguments : HashMap< String, String >,
  /// The source location of the instruction in the original input string.
  pub source_location : SourceLocation,
}

/// The main parser struct.
#[ derive( Debug ) ]
pub struct Parser
{
  options : UnilangParserOptions,
  // item_adapter : ItemAdapter, // Removed as classify_split is a standalone function
}

impl Parser
{
  /// Creates a new `Parser` instance with the given options.
  pub fn new( options : UnilangParserOptions ) -> Self
  {
    Self
    {
      options,
      // item_adapter : ItemAdapter::new(), // Removed
    }
  }

  /// Parses a single Unilang instruction from the input string.
  ///
  /// This function handles the full parsing process for a single instruction,
  /// including tokenization, command path extraction, argument parsing,
  /// and error handling.
  pub fn parse_single_instruction( &self, input : &str ) -> Result< GenericInstruction, ParseError >
  {
    let splits_iter = strs_tools::split() // Changed to splits_iter
    .src( input )
    .delimeter( vec![ "!", ":", "::", "?", "#" ] ) // Changed from .delimiters to .delimeter and added .to_vec()
    .preserving_delimeters( true ) // Changed from .preserving_delimiters
    .form()
    .iter(); // Changed from .into_iter() to .iter()

    let rich_items : Vec< RichItem<'_> > = splits_iter // Used splits_iter
    .filter( |s| !s.string.trim().is_empty() ) // Filter out whitespace-only splits
    .map( |s| crate::item_adapter::classify_split( s ) )
    .collect::< Result< Vec< RichItem<'_> >, ParseError > >()?; // Added lifetime

    self.parse_single_instruction_from_rich_items( rich_items )
  }

  /// Parses multiple Unilang instructions from the input string, separated by `;;`.
  ///
  /// This function splits the input string by `;;` delimiters and parses each segment
  /// as a separate instruction. It handles empty instruction segments and trailing delimiters.
  pub fn parse_multiple_instructions
  (
    &self,
    input : &str,
  )
  ->
  Result< Vec< GenericInstruction >, ParseError >
  {
    let splits : Vec< Split<'_> > = strs_tools::split() // Added lifetime
    .src( input )
    .delimeter( vec![ ";;" ] ) // Changed from .delimiters to .delimeter and added .to_vec()
    .preserving_delimeters( true ) // Changed from .preserving_delimiters
    .preserving_empty( true )
    .form()
    .iter() // Changed from .into_iter() to .iter()
    .collect();

    let mut result = Vec::new();
    let mut current_instruction_items = Vec::new();

    for i in 0 .. splits.len()
    {
      let split = &splits[ i ];
      // println!( "DEBUG: parse_multiple_instructions - split: {:?}", split ); // Keep for debugging if needed

      if split.typ == SplitType::Delimiter
      {
        // If we encounter a delimiter, it means the current_instruction_items form an instruction.
        // If current_instruction_items is empty, it's an empty instruction segment.
        if current_instruction_items.is_empty()
        {
          // This handles cases like `;;cmd`, `cmd1;;;;cmd2` (for the second `;;` onwards)
          let source_location = SourceLocation::StrSpan { start : split.start, end : split.end }; // Changed constructor
          return Err( ParseError::new( ErrorKind::EmptyInstructionSegment, source_location ) );
        }
        else
        {
          // Parse the accumulated items as a single instruction
          let instruction = self.parse_single_instruction_from_rich_items( current_instruction_items.drain( .. ).collect() )?;
          result.push( instruction );
        }
      }
      else if split.string.is_empty() && split.typ == SplitType::Delimeted
      {
        // This handles empty strings returned by preserving_empty, e.g., for "a;;b" or "cmd1;;"
        // An empty delimited split implies an empty instruction segment.
        // The location should be the delimiter that caused this empty segment.
        // If it's the first split and empty, it's a leading empty segment.
        if i == 0
        {
          let source_location = SourceLocation::StrSpan { start : split.start, end : split.end }; // Changed constructor
          return Err( ParseError::new( ErrorKind::EmptyInstructionSegment, source_location ) );
        }
        else
        {
          // If it's an empty delimited split and not the first, the previous split *must* have been a delimiter.
          // The error location should be that previous delimiter.
          let prev_split = &splits[ i - 1 ];
          if prev_split.typ == SplitType::Delimiter
          {
            let source_location = SourceLocation::StrSpan { start : prev_split.start, end : prev_split.end }; // Changed constructor
            return Err( ParseError::new( ErrorKind::EmptyInstructionSegment, source_location ) );
          }
          // If it's an empty delimited split and the previous was NOT a delimiter, it's an unexpected empty string.
          // This should not happen if strs_tools behaves as expected.
          // For now, let's ignore it, as it's likely a whitespace-only split that was trimmed.
          // We should not push it to current_instruction_items.
        }
      }
      else if split.string.trim().is_empty() // Filter out whitespace-only splits
      {
        // Ignore whitespace-only splits as they are not tokens.
        // This handles cases like "cmd1 ;; " where the trailing space is a Delimeted split.
      }
      else
      {
        // This is a regular instruction item
        let (kind, adjusted_source_location) = crate::item_adapter::classify_split( split )?; // Destructure tuple
        current_instruction_items.push( RichItem::new( split.clone(), kind, adjusted_source_location ) ); // Construct RichItem
      }
    }

    // After the loop, if there are remaining items, they form the last instruction.
    if !current_instruction_items.is_empty()
    {
      let instruction = self.parse_single_instruction_from_rich_items( current_instruction_items.drain( .. ).collect() )?;
      result.push( instruction );
    }
    else
    {
      // Check for trailing delimiter that implies an empty instruction.
      // This occurs if the last *meaningful* split was a delimiter, and no instruction was formed after it.
      // Iterate backwards to find the last non-whitespace split.
      let mut last_meaningful_split_idx = None;
      for i in (0..splits.len()).rev()
      {
        let split = &splits[i];
        if !(split.string.is_empty() && split.typ == SplitType::Delimeted) && !split.string.trim().is_empty()
        {
          last_meaningful_split_idx = Some(i);
          break;
        }
      }

      if let Some(idx) = last_meaningful_split_idx
      {
        let last_meaningful_split = &splits[idx];
        if last_meaningful_split.typ == SplitType::Delimiter
        {
          let source_location = SourceLocation::StrSpan { start : last_meaningful_split.start, end : last_meaningful_split.end }; // Changed constructor
          return Err( ParseError::new( ErrorKind::TrailingDelimiter, source_location ) );
        }
      }
      // If no meaningful splits, or last meaningful was not a delimiter, then it's not a TrailingDelimiter error.
    }

    Ok( result )
  }

  /// Parses a single Unilang instruction from a list of rich items.
  ///
  /// This function implements the state machine for parsing a single instruction,
  /// handling command paths, arguments, named arguments, and the help operator.
  fn parse_single_instruction_from_rich_items
  (
    &self,
    rich_items : Vec< RichItem<'_> >, // Added lifetime
  )
  ->
  Result< GenericInstruction, ParseError >
  {
    let mut command_path = Vec::new();
    let mut arguments = Vec::new();
    let mut named_arguments = HashMap::new();
    let mut help_operator_found = false;
    let mut parsing_command_path = true;
    let mut parsing_named_arg_name = false;
    let mut current_named_arg_name = String::new();
    let mut current_instruction_start_location = None;

    let mut rich_items_iter = rich_items.into_iter().peekable(); // rich_items.clone().into_iter()

    while let Some( rich_item ) = rich_items_iter.next()
    {
      // Set the start location of the instruction if it's the first item
      if current_instruction_start_location.is_none()
      {
        if let SourceLocation::StrSpan { start, .. } = rich_item.adjusted_source_location
        {
          current_instruction_start_location = Some( start );
        }
      }

      // Handle full-line comments
      // TODO: Proper comment parsing to extract the full comment text, not just "#"
      if let UnilangTokenKind::Delimiter( "#" ) = &rich_item.kind // Changed from UnilangTokenKind::Comment
      {
        // If it's a full-line comment, and nothing else has been parsed for this instruction,
        // then this instruction is just the comment.
        if command_path.is_empty() && arguments.is_empty() && named_arguments.is_empty() && !help_operator_found
        {
          // This is a full-line comment, it forms a complete instruction.
          // We should not process further items for this instruction.
          return Ok( GenericInstruction
          {
            command_path : vec![ rich_item.inner.string.to_string() ], // Temporary: stores "#" as command path
            arguments : Vec::new(),
            named_arguments : HashMap::new(),
            source_location : rich_item.adjusted_source_location,
          });
        }
        else
        {
          // Inline comments are not allowed as per spec.md
          return Err( ParseError::new( ErrorKind::Syntax( "Inline comments are not allowed".to_string() ), rich_item.adjusted_source_location ) );
        }
      }

      if parsing_command_path
      {
        match rich_item.kind
        {
          UnilangTokenKind::Identifier( s ) =>
          {
            command_path.push( s );
          },
          UnilangTokenKind::Operator( op ) if op == "." =>
          {
            // Allow multiple dots in command path, e.g., `cmd.sub.sub`
            // Do nothing, just consume the dot. Next item should be identifier.
            let next_item_is_identifier = rich_items_iter.peek()
            .map_or( false, |item| matches!( item.kind, UnilangTokenKind::Identifier( _ ) ) );
            if !next_item_is_identifier
            {
              return Err( ParseError::new( ErrorKind::Syntax( "Expected identifier after '.' in command path".to_string() ), rich_item.adjusted_source_location ) );
            }
          },
          UnilangTokenKind::Operator( op ) if op == "?" =>
          {
            help_operator_found = true;
            parsing_command_path = false; // Command path ends with '?'
          },
          UnilangTokenKind::Operator( op ) if op == ":" =>
          {
            parsing_command_path = false;
            parsing_named_arg_name = true;
          },
          UnilangTokenKind::Operator( op ) if op == "::" =>
          {
            return Err( ParseError::new( ErrorKind::Syntax( "Unexpected '::' operator in command path".to_string() ), rich_item.adjusted_source_location ) );
          },
          _ =>
          {
            // Any other token type terminates the command path and starts argument parsing.
            // Push the current item to arguments.
            parsing_command_path = false;
            arguments.push( rich_item.kind.to_string() );
          },
        }
      }
      else if parsing_named_arg_name
      {
        match rich_item.kind
        {
          UnilangTokenKind::Identifier( s ) =>
          {
            current_named_arg_name = s;
            parsing_named_arg_name = false; // Expecting '::' next
          },
          _ =>
          {
            return Err( ParseError::new( ErrorKind::Syntax( "Expected identifier for named argument name".to_string() ), rich_item.adjusted_source_location ) );
          },
        }
      }
      else
      {
        // Parsing arguments or named argument values
        match rich_item.kind
        {
          UnilangTokenKind::Operator( op ) if op == ":" =>
          {
            // This means we are starting a new named argument after a positional or named argument value.
            // This is an error if we just parsed a named argument value.
            if !current_named_arg_name.is_empty()
            {
              return Err( ParseError::new( ErrorKind::Syntax( "Unexpected ':' operator after named argument value".to_string() ), rich_item.adjusted_source_location ) );
            }
            parsing_named_arg_name = true;
          },
          UnilangTokenKind::Operator( op ) if op == "::" =>
          {
            // This is the delimiter for named argument name:value
            if current_named_arg_name.is_empty()
            {
              return Err( ParseError::new( ErrorKind::Syntax( "Unexpected '::' operator without a named argument name".to_string() ), rich_item.adjusted_source_location ) );
            }
            // Expecting value next, so do nothing here.
          },
          UnilangTokenKind::Operator( op ) if op == "?" =>
          {
            if !rich_items_iter.peek().is_none()
            {
              return Err( ParseError::new( ErrorKind::Syntax( "Help operator '?' must be the last token".to_string() ), rich_item.adjusted_source_location ) );
            }
            help_operator_found = true;
          },
          _ =>
          {
            // This is either a positional argument or a named argument value
            if !current_named_arg_name.is_empty()
            {
              // This is a named argument value
              if named_arguments.contains_key( &current_named_arg_name ) && self.options.error_on_duplicate_named_arguments
              {
                return Err( ParseError::new( ErrorKind::Syntax( format!( "Duplicate named argument '{}'", current_named_arg_name ) ), rich_item.adjusted_source_location ) );
              }
              named_arguments.insert( current_named_arg_name.drain( .. ).collect(), rich_item.kind.to_string() );
            }
            else
            {
              // This is a positional argument
              if !named_arguments.is_empty() && self.options.error_on_positional_after_named
              {
                return Err( ParseError::new( ErrorKind::Syntax( "Positional argument after named argument".to_string() ), rich_item.adjusted_source_location ) );
              }
              arguments.push( rich_item.kind.to_string() );
            }
          },
        }
      }
    }

    // Final check for help operator placement
    if help_operator_found && (!arguments.is_empty() || !named_arguments.is_empty())
    {
      // This case should ideally be caught by the `?` operator check inside the loop,
      // but this acts as a safeguard.
      return Err( ParseError::new( ErrorKind::Syntax( "Help operator '?' must be the last token".to_string() ), rich_items.last().unwrap().adjusted_source_location.clone() ) ); // Added .clone()
    }

    // If command_path is empty, it's an error unless it's a comment-only instruction.
    if command_path.is_empty() && !help_operator_found && arguments.is_empty() && named_arguments.is_empty()
    {
      return Err( ParseError::new( ErrorKind::Syntax( "Empty instruction".to_string() ), SourceLocation::StrSpan { start : 0, end : 0 } ) ); // Changed SourceLocation constructor
    }

    let instruction_end_location = rich_items.last()
    .map_or( current_instruction_start_location.unwrap_or( 0 ), |item| {
      if let SourceLocation::StrSpan { end, .. } = item.adjusted_source_location { end } else { 0 } // Fixed access to end
    });
    let instruction_start_location = current_instruction_start_location.unwrap_or( 0 );

    Ok( GenericInstruction
    {
      command_path,
      arguments,
      named_arguments,
      source_location : SourceLocation::StrSpan { start : instruction_start_location, end : instruction_end_location }, // Changed constructor
    })
  }
}