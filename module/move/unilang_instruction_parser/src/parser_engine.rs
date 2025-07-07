//! Parser for unilang instructions.
//!
//! This module provides the `Parser` struct, which is responsible for parsing
//! instruction strings into a structured `GenericInstruction` format. It handles
//! command paths, positional and named arguments, and error reporting.
//!
//! The parsing process involves tokenization, classification of tokens, and
//! a state machine to build the `GenericInstruction`.

use crate::
{
  Argument,
  GenericInstruction,
  UnilangParserOptions,
};
use crate::error::
{
  ErrorKind,
  ParseError,
  SourceLocation,
};
use crate::item_adapter::
{
  classify_split,
  RichItem,
  UnilangTokenKind,
  unescape_string_with_errors,
};
use std::collections::HashMap;
use strs_tools::string::split::{ Split, SplitType };

/// Represents the current state of the parser's state machine.
#[ derive( Debug, PartialEq, Eq ) ]
enum ParserState
{
  /// Initial state, or parsing the command path.
  ParsingCommandPath,
  /// Parsing arguments (either positional or named).
  ParsingArguments,
  /// Parsing a named argument's value after seeing `::`.
  ParsingNamedArgumentValue { name : String, name_location : SourceLocation },
  /// Parsing a help request.
  ParsingHelp,
}

/// Main parser struct.
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

  /// Parses a single instruction string into a vector of `GenericInstruction`s.
  ///
  /// Currently, only one instruction per string is supported.
  pub fn parse_single_str( &self, input : &str ) -> Result< Vec< GenericInstruction >, ParseError >
  {
    let mut all_instructions = Vec::new();
    let mut last_segment_end_byte = 0;

    // Find all occurrences of ";;"
    let semicolon_delimiters: Vec<(usize, &str)> = input.match_indices(";;").collect();

    for (idx, (delimiter_start_byte, delimiter_str)) in semicolon_delimiters.into_iter().enumerate()
    {
      // Extract the segment before the current delimiter
      let segment_raw = &input[last_segment_end_byte..delimiter_start_byte];
      let segment_trimmed = segment_raw.trim();

      if segment_trimmed.is_empty()
      {
        // This segment is empty. Check if it's a leading or consecutive delimiter.
        let is_leading_or_consecutive_empty = idx == 0 || last_segment_end_byte == delimiter_start_byte;

        if is_leading_or_consecutive_empty
        {
          return Err(ParseError {
            kind: ErrorKind::EmptyInstructionSegment,
            location: Some(SourceLocation::StrSpan {
              start: delimiter_start_byte,
              end: delimiter_start_byte + delimiter_str.len(),
            }),
          });
        }
        // If it's an empty segment between valid instructions, just skip it.
      }
      else
      {
        // Process the non-empty segment
        let rich_items = self.tokenize_input( segment_trimmed, None )?;
        let instruction = self.parse_single_instruction_from_rich_items( rich_items, segment_trimmed )?;
        all_instructions.push(instruction);
      }

      // Update last_segment_end_byte to after the current delimiter
      last_segment_end_byte = delimiter_start_byte + delimiter_str.len();
    }

    // Process the last segment after the last delimiter (or the only segment if no delimiters)
    let last_segment_raw = &input[last_segment_end_byte..];
    let last_segment_trimmed = last_segment_raw.trim();

    if !last_segment_trimmed.is_empty()
    {
      let rich_items = self.tokenize_input( last_segment_trimmed, None )?;
      let instruction = self.parse_single_instruction_from_rich_items( rich_items, last_segment_trimmed )?;
      all_instructions.push(instruction);
    }
    else if input.ends_with(";;") // Check for trailing delimiter
    {
      // This means the last segment was empty because the input ended with ";;"
      return Err(ParseError {
        kind: ErrorKind::TrailingDelimiter,
        location: Some(SourceLocation::StrSpan {
          start: input.len() - 2,
          end: input.len(),
        }),
      });
    }

    Ok(all_instructions)
  }

  /// Parses a slice of instruction strings into a vector of `GenericInstruction`s.
  pub fn parse_slice<'a>( &self, input_slice : &'a [&'a str] ) -> Result< Vec< GenericInstruction >, ParseError >
  {
    let mut all_instructions = Vec::new();

    for (segment_index, &input_str) in input_slice.iter().enumerate()
    {
      if input_str == ";;"
      {
        // If we encounter a ";;" and the previous instruction was empty, it's an error.
        // This handles ";; cmd" and "cmd ;; ;;"
        if all_instructions.is_empty() && segment_index == 0 // Leading ";;"
           || (segment_index > 0 && input_slice[segment_index - 1] == ";;") // Consecutive ";;"
        {
          return Err(ParseError {
            kind: ErrorKind::EmptyInstructionSegment,
            location: Some(SourceLocation::SliceSegment {
              segment_index,
              start_in_segment: 0,
              end_in_segment: input_str.len(),
            }),
          });
        }
        // If it's just a separator between valid instructions, do nothing,
        // as the loop will naturally process the next segment.
      }
      else
      {
        // Each non-";;" segment is a single instruction
        let rich_items = self.tokenize_input( input_str, Some(segment_index) )?;
        let instruction = self.parse_single_instruction_from_rich_items( rich_items, input_str )?;
        all_instructions.push(instruction);
      }
    }

    // Handle trailing delimiter: if the last segment was ";;" and it didn't lead to an EmptyInstructionSegment error
    if input_slice.len() > 0 && input_slice.last().unwrap() == &";;"
    {
      // If the last element was ";;" and it was not the first element,
      // and it didn't cause an EmptyInstructionSegment error (meaning it followed a valid instruction),
      // then it's a trailing delimiter.
      // The `EmptyInstructionSegment` check above handles `;;` at index 0 or `;; ;;`.
      // So if we reach here and the last element is `;;`, it must be a trailing one.
      return Err(ParseError {
        kind: ErrorKind::TrailingDelimiter,
        location: Some(SourceLocation::SliceSegment {
          segment_index: input_slice.len() - 1,
          start_in_segment: 0,
          end_in_segment: 2, // Length of ";;"
        }),
      });
    }

    Ok(all_instructions)
  }

  /// Tokenizes the input string into `RichItem`s using a custom state machine.
  fn tokenize_input<'a>( &self, input : &'a str, segment_idx : Option<usize> ) -> Result< Vec< RichItem<'a> >, ParseError >
  {
    let mut rich_items = Vec::new();
    let mut current_token_start_byte = 0;
    let mut chars = input.chars().enumerate().peekable();

    // Sort delimiters and operators by length in descending order for longest match first
    let mut sorted_delimiters: Vec<&str> = self.options.main_delimiters.iter().map(|&s| s).collect();
    sorted_delimiters.sort_by(|a, b| b.len().cmp(&a.len()));

    let mut sorted_operators: Vec<&str> = self.options.operators.iter().map(|&s| s).collect();
    sorted_operators.sort_by(|a, b| b.len().cmp(&a.len()));

    // Helper to push accumulated token - moved outside the loop
    let push_accumulated = | rich_items : &mut Vec<RichItem<'a>>, end_byte : usize, current_token_start_byte : &mut usize, segment_idx : Option<usize>, options : &UnilangParserOptions, input : &'a str |
    {
      if end_byte > *current_token_start_byte
      {
        let raw_slice = &input[ *current_token_start_byte..end_byte ];
        let kind = classify_split( &Split { string : raw_slice, start : *current_token_start_byte, end : end_byte, typ : SplitType::Delimeted }, options );
        rich_items.push( RichItem
        {
          inner : Split { string : raw_slice, start : *current_token_start_byte, end : end_byte, typ : SplitType::Delimeted },
          segment_idx,
          kind,
        });
      }
      *current_token_start_byte = end_byte;
    };

    while let Some((char_byte_idx, c)) = chars.next()
    {
      // 1. Check for start of a quoted string
      if self.options.quote_pairs.iter().any(|(open, _)| *open == c)
      {
        push_accumulated( &mut rich_items, char_byte_idx, &mut current_token_start_byte, segment_idx, &self.options, input ); // Push any accumulated token before the quote

        // Parse the quoted string
        let quote_char = c;
        let quote_start_byte = char_byte_idx;
        let mut in_escape = false;
        let mut quote_end_byte = 0;

        while let Some((inner_char_byte_idx, inner_c)) = chars.next()
        {
          if in_escape
          {
            in_escape = false;
          }
          else if inner_c == '\\'
          {
            in_escape = true;
          }
          else if self.options.quote_pairs.iter().any(|(_, close)| *close == inner_c) && inner_c == quote_char
          {
            quote_end_byte = inner_char_byte_idx + inner_c.len_utf8();
            break;
          }
        }

        if quote_end_byte == 0
        {
          return Err( ParseError
          {
            kind : ErrorKind::Syntax( format!( "Unclosed quote: Expected '{}'", quote_char ) ),
            location : Some( SourceLocation::StrSpan { start : quote_start_byte, end : input.len() } ),
          });
        }

        let raw_slice = &input[ quote_start_byte..quote_end_byte ];
        rich_items.push( RichItem
        {
          inner : Split { string : raw_slice, start : quote_start_byte, end : quote_end_byte, typ : SplitType::Delimeted }, // Type is Delimeted because it's a single block
          segment_idx,
          kind : UnilangTokenKind::QuotedValue( raw_slice.to_string() ), // Store the raw quoted value
        });
        current_token_start_byte = quote_end_byte;
        continue; // Continue outer loop from after the quoted string
      }

      // 2. Check for multi-character operators (e.g., "::")
      let mut matched_special_token = false;
      for operator_str in &sorted_operators
      {
        if input[char_byte_idx..].starts_with(operator_str)
        {
          push_accumulated( &mut rich_items, char_byte_idx, &mut current_token_start_byte, segment_idx, &self.options, input ); // Push any accumulated token before the operator

          // Push the operator itself
          let raw_slice = &input[ char_byte_idx..char_byte_idx + operator_str.len() ];
          let kind = classify_split( &Split { string : raw_slice, start : char_byte_idx, end : char_byte_idx + operator_str.len(), typ : SplitType::Delimiter }, &self.options );
          rich_items.push( RichItem
          {
            inner : Split { string : raw_slice, start : char_byte_idx, end : char_byte_idx + operator_str.len(), typ : SplitType::Delimiter },
            segment_idx,
            kind,
          });
          current_token_start_byte = char_byte_idx + operator_str.len();
          // Advance chars iterator past the operator
          for _ in 0..(operator_str.len() - c.len_utf8()) { chars.next(); }
          matched_special_token = true;
          break;
        }
      }
      if matched_special_token { continue; }

      // 3. Check for multi-character delimiters (e.g., ";;")
      for delimiter_str in &sorted_delimiters
      {
        if delimiter_str.len() > 1 && input[char_byte_idx..].starts_with(delimiter_str)
        {
          push_accumulated( &mut rich_items, char_byte_idx, &mut current_token_start_byte, segment_idx, &self.options, input ); // Push any accumulated token before the delimiter

          // Push the multi-character delimiter itself
          let raw_slice = &input[ char_byte_idx..char_byte_idx + delimiter_str.len() ];
          let kind = classify_split( &Split { string : raw_slice, start : char_byte_idx, end : char_byte_idx + delimiter_str.len(), typ : SplitType::Delimiter }, &self.options );
          rich_items.push( RichItem
          {
            inner : Split { string : raw_slice, start : char_byte_idx, end : char_byte_idx + delimiter_str.len(), typ : SplitType::Delimiter },
            segment_idx,
            kind,
          });
          current_token_start_byte = char_byte_idx + delimiter_str.len();
          // Advance chars iterator past the delimiter
          for _ in 0..(delimiter_str.len() - c.len_utf8()) { chars.next(); }
          matched_special_token = true;
          break;
        }
      }
      if matched_special_token { continue; }


      // 4. Check for single-character delimiters or whitespace
      if self.options.main_delimiters.iter().any(|delimiter| *delimiter == c.to_string().as_str()) || ( self.options.whitespace_is_separator && c.is_whitespace() )
      {
        push_accumulated( &mut rich_items, char_byte_idx, &mut current_token_start_byte, segment_idx, &self.options, input ); // Push any accumulated token before the delimiter

        // Push the delimiter itself
        let raw_slice = &input[ char_byte_idx..char_byte_idx + c.len_utf8() ];
        let kind = classify_split( &Split { string : raw_slice, start : char_byte_idx, end : char_byte_idx + c.len_utf8(), typ : SplitType::Delimiter }, &self.options );
        rich_items.push( RichItem
        {
          inner : Split { string : raw_slice, start : char_byte_idx, end : char_byte_idx + c.len_utf8(), typ : SplitType::Delimiter },
          segment_idx,
          kind,
        });
        current_token_start_byte = char_byte_idx + c.len_utf8();
      }
      // Else, it's part of an identifier, continue accumulating
    }

    // Push any remaining token after loop
    push_accumulated( &mut rich_items, input.len(), &mut current_token_start_byte, segment_idx, &self.options, input );

    Ok( rich_items )
  }

  /// Parses a single instruction from a list of `RichItem`s.
  fn parse_single_instruction_from_rich_items<'a>( &self, rich_items : Vec< RichItem<'a> >, input : &str ) -> Result< GenericInstruction, ParseError >
  {
    let mut command_path_slices = Vec::new();
    let mut positional_arguments = Vec::new();
    let mut named_arguments = HashMap::new();
    let mut help_requested = false;
    let mut state = ParserState::ParsingCommandPath;
    let mut rich_items_iter = rich_items.into_iter().peekable();

    while let Some( item ) = rich_items_iter.next()
    {
      match state
      {
        ParserState::ParsingCommandPath =>
        {
          match item.kind
          {
            UnilangTokenKind::Identifier(_) =>
            {
              // Check if the next item is '::'. If so, this identifier is a named argument name.
              if let Some( next_item ) = rich_items_iter.peek()
              {
                if matches!(next_item.kind, UnilangTokenKind::Operator(_)) && next_item.inner.string == "::"
                {
                  // This is a named argument, so command path parsing is done.
                  state = ParserState::ParsingArguments;
                  // Re-process the current item as an argument.
                  self.parse_argument_item(item, &mut rich_items_iter, &mut command_path_slices, &mut positional_arguments, &mut named_arguments, &mut help_requested, &mut state)?;
                  continue;
                }
              }

              // Add the current identifier to command path slices.
              command_path_slices.push( item.inner.string.to_string() );

              // If the next item is NOT a space or a dot, then the command path is finished.
              if let Some( next_item ) = rich_items_iter.peek()
              {
                if !(matches!(next_item.kind, UnilangTokenKind::Delimiter(_)) && (next_item.inner.string == "." || next_item.inner.string == " "))
                {
                  state = ParserState::ParsingArguments;
                }
              }
              else
              {
                // End of input, command path is done.
                state = ParserState::ParsingArguments;
              }
            },
            UnilangTokenKind::Delimiter(_) if item.inner.string == "." =>
            {
              // Dot between command path segments.
              // If command_path_slices is empty, it's a leading dot, which is ignored.
              // If command_path_slices is not empty, it's a separator.
              // If arguments have already started, it's an error.
              if command_path_slices.is_empty()
              {
                // Leading dot, ignore.
                continue;
              }
              else if positional_arguments.is_empty() && named_arguments.is_empty()
              {
                // Dot between command path segments, continue.
              }
              else
              {
                // Dot after arguments have started is an error.
                return Err( ParseError
                {
                  kind : ErrorKind::Syntax( "Unexpected '.' after arguments begin.".to_string() ),
                  location : Some( item.source_location() ),
                });
              }
            },
            UnilangTokenKind::Delimiter(_) if item.inner.string == " " =>
            {
              // Ignore spaces within command path.
              continue;
            },
            UnilangTokenKind::Operator(_) if item.inner.string == "?" =>
            {
              // Help operator. Only valid if no arguments have started.
              if !positional_arguments.is_empty() || !named_arguments.is_empty()
              {
                return Err( ParseError
                {
                  kind : ErrorKind::Syntax( "Unexpected help operator '?' amidst arguments.".to_string() ),
                  location : Some( item.source_location() ),
                });
              }
              help_requested = true;
              state = ParserState::ParsingHelp;
            },
            
            
            _ =>
            {
              // Any other token type means command path is done, and this token is an argument.
              state = ParserState::ParsingArguments;
              // Re-process the current item as an argument.
              self.parse_argument_item(item, &mut rich_items_iter, &mut command_path_slices, &mut positional_arguments, &mut named_arguments, &mut help_requested, &mut state)?;
            },
          }
        },
        ParserState::ParsingArguments =>
        {
          self.parse_argument_item(item, &mut rich_items_iter, &mut command_path_slices, &mut positional_arguments, &mut named_arguments, &mut help_requested, &mut state)?;
        },
        ParserState::ParsingNamedArgumentValue { ref name, ref name_location } =>
        {
          match item.kind
          {
            UnilangTokenKind::Identifier(_) | UnilangTokenKind::QuotedValue(_) =>
            {
              let value = if matches!(item.kind, UnilangTokenKind::QuotedValue(_))
              {
                let val_s = item.inner.string;
                unescape_string_with_errors( &val_s[1..val_s.len() - 1], &item.source_location() )?
              }
              else
              {
                item.inner.string.to_string()
              };

              if named_arguments.contains_key( name ) && self.options.error_on_duplicate_named_arguments
              {
                return Err( ParseError
                {
                  kind : ErrorKind::Syntax( format!( "Duplicate named argument: {}", name ) ),
                  location : Some( name_location.clone() ),
                });
              }
              named_arguments.insert( name.clone(), Argument
              {
                name : Some( name.clone() ),
                value,
                name_location : Some( name_location.clone() ),
                value_location : item.source_location(),
              });
              state = ParserState::ParsingArguments;
            },
            
            UnilangTokenKind::Delimiter(_) if item.inner.string == " " =>
            {
              // Ignore spaces after ::, but before value
            },
            _ =>
            {
              return Err( ParseError
              {
                kind : ErrorKind::Syntax( format!( "Expected value for named argument '{}' but found {:?}{}", name, item.kind, if item.inner.string.is_empty() { "".to_string() } else { format!( "(\"{}\")", item.inner.string ) } ) ),
                location : Some( name_location.clone() ),
              });
            },
          }
        },
        ParserState::ParsingHelp =>
        {
          // After '?', any further tokens are unexpected.
          return Err( ParseError
          {
            kind : ErrorKind::Syntax( format!( "Unexpected token after help operator: '{}' ({:?})", item.inner.string, item.kind ) ),
            location : Some( item.source_location() ),
          });
        },
      }
    }

    // Handle case where named argument value was expected but not found (e.g., "cmd name::")
    if let ParserState::ParsingNamedArgumentValue { ref name, ref name_location } = state
    {
      return Err( ParseError
      {
        kind : ErrorKind::Syntax( format!( "Expected value for named argument '{}' but found end of instruction", name ) ),
        location : Some( name_location.clone() ),
      });
    }

    Ok( GenericInstruction
    {
      command_path_slices,
      positional_arguments,
      named_arguments,
      help_requested,
      overall_location : SourceLocation::StrSpan { start: 0, end: input.len() },
    })
  }

  /// Helper function to parse an item as an argument.
  fn parse_argument_item<'a, I>(
    &self,
    item: RichItem<'a>,
    items_iter: &mut std::iter::Peekable<I>,
    command_path_slices: &mut Vec<String>, // Added command_path_slices
    positional_arguments: &mut Vec<Argument>,
    named_arguments: &mut HashMap<String, Argument>,
    help_requested: &mut bool,
    state: &mut ParserState,
  ) -> Result<(), ParseError>
  where
    I: Iterator<Item = RichItem<'a>>,
  {
    // If we were expecting a named arg value, the first token we see is it.
    if let ParserState::ParsingNamedArgumentValue { name, name_location } = std::mem::replace(state, ParserState::ParsingArguments)
    {
      return self.finalize_named_argument(item, name, name_location, named_arguments, state);
    }

    match item.kind
    {
      UnilangTokenKind::Identifier(_) =>
      {
        // Check for named argument delimiter
        if let Some( next_item ) = items_iter.peek()
        {
          if matches!(next_item.kind, UnilangTokenKind::Operator(_)) && next_item.inner.string == "::"
          {
            // Consume "::"
            let _ = items_iter.next();
            *state = ParserState::ParsingNamedArgumentValue
            {
              name : item.inner.string.to_string(),
              name_location : item.source_location(),
            };
            return Ok(());
          }
        }
        // Positional argument
        if !named_arguments.is_empty() && self.options.error_on_positional_after_named
        {
          return Err( ParseError
          {
            kind : ErrorKind::Syntax( "Positional argument encountered after a named argument.".to_string() ),
            location : Some( item.source_location() ),
          });
        }
        positional_arguments.push( Argument
        {
          name : None,
          value : item.inner.string.to_string(),
          name_location : None,
          value_location : item.source_location(),
        });
      },
      UnilangTokenKind::QuotedValue(_) =>
      {
        // Positional argument
        if !named_arguments.is_empty() && self.options.error_on_positional_after_named
        {
          return Err( ParseError
          {
            kind : ErrorKind::Syntax( "Positional argument encountered after a named argument.".to_string() ),
            location : Some( item.source_location() ),
          });
        }
        // Strip outer quotes before unescaping
        let val_s = item.inner.string;
        let unescaped_value = unescape_string_with_errors( &val_s[1..val_s.len() - 1], &item.source_location() )?;
        positional_arguments.push( Argument
        {
          name : None,
          value : unescaped_value,
          name_location : None,
          value_location : item.source_location(),
        });
      },
      UnilangTokenKind::Delimiter(_) if item.inner.string == " " =>
      {
        // Ignore spaces between arguments
      },
      UnilangTokenKind::Operator(_) if item.inner.string == "?" =>
      {
        // The '?' operator is only valid as a help request immediately after the command path.
        // If it's encountered while parsing arguments, it's an error.
        return Err( ParseError
        {
          kind : ErrorKind::Syntax( "Unexpected help operator '?' amidst arguments.".to_string() ),
          location : Some( item.source_location() ),
        });
      },
      UnilangTokenKind::Operator(_) if item.inner.string == "::" =>
      {
        return Err( ParseError
        {
          kind : ErrorKind::Syntax( "Unexpected '::' without preceding argument name".to_string() ),
          location : Some( item.source_location() ),
        });
      },
      _ =>
      {
        return Err( ParseError
        {
          kind : ErrorKind::Syntax( format!( "Unexpected token in arguments: '{}' ({:?})", item.inner.string, item.kind ) ),
          location : Some( item.source_location() ),
        });
      },
    }
    Ok(())
  }

  /// Helper to finalize a named argument.
  fn finalize_named_argument(
    &self,
    value_item: RichItem<'_>,
    name: String,
    name_location: SourceLocation,
    named_arguments: &mut HashMap<String, Argument>,
    state: &mut ParserState,
  ) -> Result<(), ParseError>
  {
    let value = match value_item.kind
    {
      UnilangTokenKind::Identifier(_) | UnilangTokenKind::QuotedValue(_) =>
      {
        if matches!(value_item.kind, UnilangTokenKind::QuotedValue(_))
        {
          let val_s = value_item.inner.string;
          unescape_string_with_errors( &val_s[1..val_s.len() - 1], &value_item.source_location() )?
        }
        else
        {
          value_item.inner.string.to_string()
        }
      }
      _ =>
      {
        return Err( ParseError
        {
          kind : ErrorKind::Syntax( format!( "Expected value for named argument '{}' but found {:?}{}", name, value_item.kind, if value_item.inner.string.is_empty() { "".to_string() } else { format!( "(\"{}\")", item.inner.string ) } ) ),
          location : Some( name_location.clone() ),
        });
      }
    };

    if named_arguments.contains_key( &name ) && self.options.error_on_duplicate_named_arguments
    {
      return Err( ParseError
      {
        kind : ErrorKind::Syntax( format!( "Duplicate named argument: {}", name ) ),
        location : Some( name_location.clone() ),
      });
    }

    named_arguments.insert( name.clone(), Argument
    {
      name : Some( name.clone() ),
      value,
      name_location : Some( name_location.clone() ),
      value_location : value_item.source_location(),
    });
    *state = ParserState::ParsingArguments;
    Ok(())
  }
}