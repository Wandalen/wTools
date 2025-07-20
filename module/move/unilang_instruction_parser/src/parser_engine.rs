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
use crate::instruction::{ Argument, GenericInstruction };
use std::collections::HashMap;
use strs_tools::string::split::{ SplitType, Split };



/// The main parser struct.
#[ derive( Debug ) ]
pub struct Parser
{
  options : UnilangParserOptions,
}

impl Parser
{
  /// Creates a new `Parser` instance with the given options.
  pub fn new( options : UnilangParserOptions ) -> Self
  {
    Self { options }
  }

  /// Parses a single Unilang instruction from the input string.
  pub fn parse_single_instruction( &self, input : &str ) -> Result< crate::instruction::GenericInstruction, ParseError >
  {
    let splits_iter = strs_tools::split()
    .src( input )
    .delimeter( vec![ " ", "\n", "\t", "\r", "::", "?", "#", "." ] )
    .preserving_delimeters( true )
    .quoting( true )
    .preserving_quoting( false )
    .perform();

    let rich_items : Vec< RichItem<'_> > = splits_iter
    .map( |s| {
        let (kind, adjusted_source_location) = crate::item_adapter::classify_split(&s)?;
        Ok(RichItem::new(s, kind, adjusted_source_location))
    })
    .collect::<Result<Vec<RichItem<'_>>, ParseError>>()?;

    let rich_items : Vec<RichItem<'_>> = rich_items
      .into_iter()
      .filter( |item| !matches!( item.kind, UnilangTokenKind::Delimiter( " " | "\n" ) ) )
      .collect();


    self.parse_single_instruction_from_rich_items( rich_items )
  }

  /// Parses multiple Unilang instructions from the input string, separated by `;;`.
  pub fn parse_multiple_instructions
  (
    &self,
    input : &str,
  )
  ->
  Result< Vec< crate::instruction::GenericInstruction >, ParseError >
  {
    let segments : Vec< Split<'_> > = strs_tools::split()
    .src( input )
    .delimeter( vec![ ";;" ] )
    .preserving_delimeters( true )
    .preserving_empty( false ) // Do not preserve empty segments for whitespace
    .stripping( true ) // Strip leading/trailing whitespace from delimited segments
    .form()
    .split()
    .collect();

    let mut instructions = Vec::new();
    let mut last_was_delimiter = true; // Tracks if the previous segment was a delimiter

    // Handle cases where input is empty or consists only of delimiters/whitespace
    if segments.is_empty() {
        return Ok(Vec::new()); // Empty input, no instructions
    }

    // Check if the first segment is an empty delimited segment (e.g., " ;; cmd")
    // or if the input starts with a delimiter (e.g., ";; cmd")
    // This handles "EmptyInstructionSegment" for leading " ;;" or "  ;;"
    if segments[0].typ == SplitType::Delimiter || (segments[0].typ == SplitType::Delimeted && segments[0].string.trim().is_empty()) {
        if segments[0].start == 0 { // It's a leading delimiter or empty segment at start
            return Err( ParseError::new( ErrorKind::EmptyInstructionSegment, SourceLocation::StrSpan { start : segments[0].start, end : segments[0].end } ) );
        }
    }

    for segment in &segments
    {
        // Filter out empty delimited segments that are not actual content
        if segment.typ == SplitType::Delimeted && segment.string.trim().is_empty() {
            continue; // Skip this segment, it's just whitespace or an empty token from stripping
        }

        if segment.typ == SplitType::Delimiter
        {
            if last_was_delimiter // Consecutive delimiters (e.g., "cmd ;;;; cmd")
            {
                return Err( ParseError::new( ErrorKind::EmptyInstructionSegment, SourceLocation::StrSpan { start : segment.start, end : segment.end } ) );
            }
            last_was_delimiter = true;
        }
        else // Delimited content
        {
            let instruction = self.parse_single_instruction( segment.string.as_ref() )?;
            instructions.push( instruction );
            last_was_delimiter = false;
        }
    }

    // After the loop, check for a trailing delimiter
    // This handles "TrailingDelimiter" for "cmd ;;" or "cmd ;;   "
    if last_was_delimiter && !instructions.is_empty() // If the last token was a delimiter and we parsed at least one instruction
    {
        let last_delimiter_segment = segments.iter().rev().find(|s| s.typ == SplitType::Delimiter).unwrap();
        return Err( ParseError::new( ErrorKind::TrailingDelimiter, SourceLocation::StrSpan { start : last_delimiter_segment.start, end : last_delimiter_segment.end } ) );
    }

    Ok( instructions )
  }

  /// Parses a single Unilang instruction from a list of rich items.
  fn parse_single_instruction_from_rich_items
  (
    &self,
    rich_items : Vec< RichItem<'_> >,
  )
  ->
  Result< crate::instruction::GenericInstruction, ParseError >
  {
    let mut command_path_slices = Vec::new();
    let mut positional_arguments = Vec::new();
    let mut named_arguments = HashMap::new();
    let mut help_operator_found = false;
    let mut current_instruction_start_location = None;
    let mut last_token_was_dot = false;

    let mut items_iter = rich_items.clone().into_iter().peekable();

    // Handle optional leading dot as per spec.md Rule 3.1
    if let Some(first_item) = items_iter.peek() {
        if let UnilangTokenKind::Delimiter(".") = &first_item.kind {
            if let SourceLocation::StrSpan { start, end: _ } = first_item.adjusted_source_location.clone() {
                if start == 0 { // Ensure it's truly a leading dot at the beginning of the input
                    items_iter.next(); // Consume the leading dot
                }
            }
        }
    }

    // Phase 1: Parse Command Path
    while let Some( item ) = items_iter.peek()
    {
      if current_instruction_start_location.is_none()
      {
        if let SourceLocation::StrSpan { start, .. } = item.adjusted_source_location.clone()
        {
          current_instruction_start_location = Some( start );
        }
      }

      match &item.kind
      {
        UnilangTokenKind::Identifier( ref s ) =>
        {
          if command_path_slices.is_empty() || last_token_was_dot
          {
            command_path_slices.push( s.clone() );
            last_token_was_dot = false;
            items_iter.next(); // Consume item
          }
          else
          {
            break; // End of command path
          }
        },
        UnilangTokenKind::Delimiter( "." ) =>
        {
          if last_token_was_dot // Consecutive dots, e.g., "cmd..sub"
          {
            return Err( ParseError::new( ErrorKind::Syntax( "Unexpected consecutive '.' operator".to_string() ), item.adjusted_source_location.clone() ) );
          }
          last_token_was_dot = true;
          items_iter.next(); // Consume item
        },
        _ =>
        {
          break; // End of command path
        }
      }
    }

    if last_token_was_dot
    {
      return Err(ParseError::new(ErrorKind::Syntax("Command path cannot end with a '.'".to_string()), SourceLocation::StrSpan { start: 0, end: 0 })); // Location needs fix
    }

    // Phase 2: Parse Arguments
    while let Some( item ) = items_iter.next()
    {
      match item.kind
      {
        UnilangTokenKind::Identifier( ref s ) =>
        {
          if let Some( next_item ) = items_iter.peek()
          {
            if let UnilangTokenKind::Operator( "::" ) = &next_item.kind
            {
              // Named argument
              items_iter.next(); // Consume '::'
              let arg_name = s;

              if let Some( value_item ) = items_iter.next()
              {
                match value_item.kind
                {
                  UnilangTokenKind::Identifier( ref val ) =>
                  {
                    if named_arguments.contains_key( arg_name ) && self.options.error_on_duplicate_named_arguments
                    {
                      return Err( ParseError::new( ErrorKind::Syntax( format!( "Duplicate named argument '{}'", arg_name ) ), value_item.adjusted_source_location.clone() ) );
                    }
                    named_arguments.insert( arg_name.clone(), Argument
                    {
                      name : Some( arg_name.clone() ),
                      value : val.clone(),
                      name_location : Some( item.source_location() ),
                      value_location : value_item.source_location(),
                    });
                  },
                  _ => return Err( ParseError::new( ErrorKind::Syntax( format!( "Expected value for named argument '{}'", arg_name ) ), value_item.adjusted_source_location.clone() ) )
                }
              }
              else
              {
                return Err( ParseError::new( ErrorKind::Syntax( format!( "Expected value for named argument '{}' but found end of instruction", arg_name ) ), item.adjusted_source_location.clone() ) );
              }
            }
            else
            {
              // Positional argument
              if !named_arguments.is_empty() && self.options.error_on_positional_after_named
              {
                return Err( ParseError::new( ErrorKind::Syntax( "Positional argument after named argument".to_string() ), item.adjusted_source_location.clone() ) );
              }
              positional_arguments.push( Argument
              {
                name : None,
                value : s.clone(),
                name_location : None,
                value_location : item.source_location(),
              });
            }
          }
          else
          {
            // Last token, must be positional
            if !named_arguments.is_empty() && self.options.error_on_positional_after_named
            {
              return Err( ParseError::new( ErrorKind::Syntax( "Positional argument after named argument".to_string() ), item.adjusted_source_location.clone() ) );
            }
            positional_arguments.push( Argument
            {
              name : None,
              value : s.clone(),
              name_location : None,
              value_location : item.source_location(),
            });
          }
        },
        // Quoted values are now handled as Identifiers by strs_tools
        // UnilangTokenKind::QuotedValue( ref s ) =>
        // {
        //   if !named_arguments.is_empty() && self.options.error_on_positional_after_named
        //   {
        //     return Err( ParseError::new( ErrorKind::Syntax( "Positional argument after named argument".to_string() ), item.adjusted_source_location.clone() ) );
        //   }
        //   positional_arguments.push( Argument
        //   {
        //     name : None,
        //     value : s.clone(),
        //     name_location : None,
        //     value_location : item.source_location(),
        //   });
        // },
        UnilangTokenKind::Operator( "?" ) =>
        {
          if items_iter.peek().is_some()
          {
            return Err( ParseError::new( ErrorKind::Syntax( "Help operator '?' must be the last token".to_string() ), item.adjusted_source_location.clone() ) );
          }
          help_operator_found = true;
        },
        _ => return Err( ParseError::new( ErrorKind::Syntax( format!( "Unexpected token '{}' in arguments", item.inner.string ) ), item.adjusted_source_location.clone() ) ),
      }
    }

    if help_operator_found && ( !positional_arguments.is_empty() || !named_arguments.is_empty() )
    {
      return Err( ParseError::new( ErrorKind::Syntax( "Help operator '?' must be the last token".to_string() ), SourceLocation::StrSpan { start : 0, end : 0 } ) );
    }

    // If after parsing, no command path, arguments, or named arguments were found,
    // and no help operator was found, then it's an empty instruction.
    // This handles cases like empty string or just whitespace.
    if command_path_slices.is_empty() && !help_operator_found && positional_arguments.is_empty() && named_arguments.is_empty()
    {
      // If rich_items is empty, it means the input was empty or only whitespace.
      // This should result in an empty instruction, not an error.
      if rich_items.is_empty() {
          // This case is handled by the overall_location calculation below.
      } else if rich_items.len() == 1 && matches!(rich_items[0].kind, UnilangTokenKind::Delimiter(".")) {
          // Special case: if the original input was just a leading dot, it's not an error.
          // It results in an an empty command path.
          // This case is handled by the overall_location calculation below.
      } else {
          return Err( ParseError::new( ErrorKind::Syntax( "Empty instruction".to_string() ), SourceLocation::StrSpan { start : 0, end : 0 } ) );
      }
    }

    let instruction_start_location = current_instruction_start_location.unwrap_or( 0 );
    let instruction_end_location = if let Some(last_item) = rich_items.last() {
        last_item.inner.end
    } else {
        instruction_start_location // Fallback if no items
    };

    Ok( GenericInstruction
    {
      command_path_slices,
      positional_arguments,
      named_arguments,
      help_requested : help_operator_found,
      overall_location : SourceLocation::StrSpan { start : instruction_start_location, end : instruction_end_location },
    })
  }
}