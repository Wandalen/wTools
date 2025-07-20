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
use alloc::vec::IntoIter;
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
  #[ must_use ]
  pub fn new( options : UnilangParserOptions ) -> Self
  {
    Self { options }
  }

  /// Parses a single Unilang instruction from the input string.
  /// Parses a single Unilang instruction from the input string.
  ///
  /// # Errors
  /// Returns a `ParseError` if the input string cannot be parsed into a valid instruction.
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
      .filter( |item| !matches!( item.kind, UnilangTokenKind::Delimiter( " " | "\n" | "\t" | "\r" ) ) )
      .collect();


    self.parse_single_instruction_from_rich_items( rich_items )
  }

  /// Parses multiple Unilang instructions from the input string, separated by `;;`.
  /// Parses multiple Unilang instructions from the input string, separated by `;;`.
  ///
  /// # Errors
  /// Returns a `ParseError` if any segment cannot be parsed into a valid instruction,
  /// or if there are empty instruction segments (e.g., `;;;;`) or trailing delimiters (`cmd;;`).
  ///
  /// # Panics
  /// Panics if `segments.iter().rev().find(|s| s.typ == SplitType::Delimiter).unwrap()` fails,
  /// which indicates a logic error where a trailing delimiter was expected but not found.
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
    if (segments[0].typ == SplitType::Delimiter || (segments[0].typ == SplitType::Delimeted && segments[0].string.trim().is_empty()))
        && segments[0].start == 0
    {
        return Err( ParseError::new( ErrorKind::EmptyInstructionSegment, SourceLocation::StrSpan { start : segments[0].start, end : segments[0].end } ) );
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
    // Handle empty input (after filtering whitespace)
    if rich_items.is_empty() {
        return Ok(GenericInstruction {
            command_path_slices: Vec::new(),
            positional_arguments: Vec::new(),
            named_arguments: HashMap::new(),
            help_requested: false,
            overall_location: SourceLocation::None, // No specific location for empty input
        });
    }

    let instruction_start_location = rich_items.first().map_or(0, |item| item.inner.start);
    let instruction_end_location = rich_items.last().map_or(instruction_start_location, |item| item.inner.end);

    let mut items_iter = rich_items.into_iter().peekable();

    // Handle optional leading dot as per spec.md Rule 3.1
    if let Some(first_item) = items_iter.peek() {
        if let UnilangTokenKind::Delimiter(".") = &first_item.kind {
            if first_item.inner.start == 0 { // Ensure it's truly a leading dot at the beginning of the input
                items_iter.next(); // Consume the leading dot
            }
        }
    }

    let command_path_slices = Self::parse_command_path( &mut items_iter, instruction_end_location )?;
    let ( positional_arguments, named_arguments, help_operator_found ) = self.parse_arguments( &mut items_iter )?;

    Ok( GenericInstruction
    {
      command_path_slices,
      positional_arguments,
      named_arguments,
      help_requested : help_operator_found,
      overall_location : SourceLocation::StrSpan { start : instruction_start_location, end : instruction_end_location },
    })
  }

  /// Parses the command path from a peekable iterator of rich items.
  fn parse_command_path
  (
    items_iter : &mut core::iter::Peekable<IntoIter<RichItem<'_>>>,
    instruction_end_location : usize,
  )
  ->
  Result< Vec< String >, ParseError >
  {
    let mut command_path_slices = Vec::new();
    let mut last_token_was_dot = false;

    while let Some( item ) = items_iter.peek()
    {
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
      // Capture the location of the trailing dot for the error message
      let last_dot_location = if let Some(last_item) = items_iter.peek() { // Peek at the last item if available
          SourceLocation::StrSpan { start: last_item.inner.start, end: last_item.inner.end }
      } else {
          // Fallback if items_iter is empty after consuming the dot.
          // This might happen if the input was just "cmd."
          SourceLocation::StrSpan { start: instruction_end_location - 1, end: instruction_end_location } // Approximate, using overall end
      };
      return Err(ParseError::new(ErrorKind::Syntax("Command path cannot end with a '.'".to_string()), last_dot_location));
    }

    Ok( command_path_slices )
  }

  /// Parses arguments from a peekable iterator of rich items.
  #[ allow( clippy::type_complexity ) ]
  fn parse_arguments
  (
    &self,
    items_iter : &mut core::iter::Peekable<IntoIter<RichItem<'_>>>,
  )
  ->
  Result< ( Vec< Argument >, HashMap< String, Argument >, bool ), ParseError >
  {
    let mut positional_arguments = Vec::new();
    let mut named_arguments = HashMap::new();
    let mut help_operator_found = false;

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
                      return Err( ParseError::new( ErrorKind::Syntax( format!( "Duplicate named argument '{arg_name}'" ) ), value_item.adjusted_source_location.clone() ) );
                    }
                    named_arguments.insert( arg_name.clone(), Argument
                    {
                      name : Some( arg_name.clone() ),
                      value : val.clone(),
                      name_location : Some( item.source_location() ),
                      value_location : value_item.source_location(),
                    });
                  },
                  _ => return Err( ParseError::new( ErrorKind::Syntax( format!( "Expected value for named argument '{arg_name}'" ) ), value_item.adjusted_source_location.clone() ) )
                }
              }
              else
              {
                return Err( ParseError::new( ErrorKind::Syntax( format!( "Expected value for named argument '{arg_name}' but found end of instruction" ) ), item.adjusted_source_location.clone() ) );
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

    Ok( ( positional_arguments, named_arguments, help_operator_found ) )
  }
}