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
use alloc::collections::BTreeMap;
use alloc::vec::{ Vec, IntoIter };
use alloc::string::{ String, ToString };
use alloc::format;

/// Handle quoted string parsing with escape sequence support
fn handle_quoted_string< 'a >( input : &'a str, pos : &mut usize, result : &mut Vec< crate::item_adapter::Split< 'a > > )
{
  use alloc::string::String;
  
  let quote_start = *pos;
  let ch = input.chars().nth( *pos ).unwrap();
  *pos += ch.len_utf8(); // Skip opening quote
  let content_start = *pos;
  
  let mut unescaped_content = String::new();
  let mut has_escapes = false;
  
  // Process content character by character to handle escapes
  while *pos < input.len()
  {
    let current_ch = input.chars().nth( *pos ).unwrap();
    
    if current_ch == '"'
    {
      // Found closing quote
      let content_end = *pos;
      *pos += current_ch.len_utf8(); // Skip closing quote
      
      // Create split with either the original content or unescaped content
      let final_content = if has_escapes {
        alloc::borrow::Cow::Owned( unescaped_content )
      } else {
        alloc::borrow::Cow::Borrowed( &input[ content_start..content_end ] )
      };
      
      result.push( crate::item_adapter::Split {
        string : final_content,
        bounds : ( quote_start, *pos ),
        start : quote_start,
        end : *pos,
        typ : crate::item_adapter::SplitType::Delimiter,
        was_quoted : true, // Mark as quoted
      });
      return;
    }
    else if current_ch == '\\'
    {
      // Handle escape sequences
      // If this is the first escape, copy all previous content
      if !has_escapes {
        unescaped_content.push_str( &input[ content_start..*pos ] );
        has_escapes = true;
      }
      
      *pos += current_ch.len_utf8();
      if *pos < input.len()
      {
        let escaped_ch = input.chars().nth( *pos ).unwrap();
        
        match escaped_ch
        {
          '"' => unescaped_content.push( '"' ),
          '\\' => unescaped_content.push( '\\' ),
          'n' => unescaped_content.push( '\n' ),
          't' => unescaped_content.push( '\t' ),
          'r' => unescaped_content.push( '\r' ),
          _ => {
            // For unknown escapes, include the backslash and the character
            unescaped_content.push( '\\' );
            unescaped_content.push( escaped_ch );
          }
        }
        *pos += escaped_ch.len_utf8();
      }
      else
      {
        // Trailing backslash at end - just add it
        unescaped_content.push( '\\' );
      }
    }
    else
    {
      // Regular character
      if has_escapes {
        unescaped_content.push( current_ch );
      }
      *pos += current_ch.len_utf8();
    }
  }
  
  // If we reached end without finding closing quote
  if *pos >= input.len()
  {
    // Unterminated quote - include what we have
    let final_content = if has_escapes {
      alloc::borrow::Cow::Owned( unescaped_content )
    } else {
      alloc::borrow::Cow::Borrowed( &input[ content_start.. ] )
    };
    
    result.push( crate::item_adapter::Split {
      string : final_content,
      bounds : ( quote_start, input.len() ),
      start : quote_start,
      end : input.len(),
      typ : crate::item_adapter::SplitType::Delimiter,
      was_quoted : true,
    });
  }
}

/// Check for multi-character delimiters
fn try_multi_char_delimiter< 'a >( input : &'a str, pos : &mut usize, delimiters : &[ &str ], result : &mut Vec< crate::item_adapter::Split< 'a > > ) -> bool
{
  for delimiter in delimiters
  {
    if delimiter.len() > 1 && input[ *pos.. ].starts_with( delimiter )
    {
      result.push( crate::item_adapter::Split {
        string : alloc::borrow::Cow::Borrowed( &input[ *pos..*pos + delimiter.len() ] ),
        bounds : ( *pos, *pos + delimiter.len() ),
        start : *pos,
        end : *pos + delimiter.len(),
        typ : crate::item_adapter::SplitType::Delimiter,
        was_quoted : false,
      });
      *pos += delimiter.len();
      return true;
    }
  }
  false
}

/// Handle non-delimiter segment
fn handle_non_delimiter_segment< 'a >( input : &'a str, pos : &mut usize, delimiters : &[ &str ], result : &mut Vec< crate::item_adapter::Split< 'a > > )
{
  let start_pos = *pos;
  while *pos < input.len()
  {
    let current_ch = input.chars().nth( *pos ).unwrap();
    let current_ch_str = &input[ *pos..*pos + current_ch.len_utf8() ];
    
    // Check if we hit a delimiter or quote
    let is_delimiter = current_ch == '"' || current_ch.is_whitespace() || 
      delimiters.iter().any( | d | d.len() == 1 && *d == current_ch_str ) ||
      delimiters.iter().any( | d | d.len() > 1 && input[ *pos.. ].starts_with( d ) );
    
    if is_delimiter
    {
      break;
    }
    
    *pos += current_ch.len_utf8();
  }
  
  if start_pos < *pos
  {
    result.push( crate::item_adapter::Split {
      string : alloc::borrow::Cow::Borrowed( &input[ start_pos..*pos ] ),
      bounds : ( start_pos, *pos ),
      start : start_pos,
      end : *pos,
      typ : crate::item_adapter::SplitType::Delimiter, // Mark as delimiter so it gets classified as Identifier
      was_quoted : false,
    });
  }
}

/// Simple split function to replace `strs_tools` functionality
fn simple_split< 'a >( input : &'a str, delimiters : &[ &str ] ) -> Vec< crate::item_adapter::Split< 'a > >
{
  let mut result = Vec::new();
  let mut pos = 0;
  
  while pos < input.len()
  {
    let ch = input.chars().nth( pos ).unwrap();
    
    // Check if we're starting a quoted string
    if ch == '"'
    {
      handle_quoted_string( input, &mut pos, &mut result );
      continue;
    }
    
    // First check for multi-character delimiters
    if try_multi_char_delimiter( input, &mut pos, delimiters, &mut result )
    {
      continue;
    }
    
    // Check for single-character delimiters or whitespace
    let ch_str = &input[ pos..pos + ch.len_utf8() ];
    
    if ch.is_whitespace() || delimiters.iter().any( | d | d.len() == 1 && *d == ch_str )
    {
      result.push( crate::item_adapter::Split {
        string : alloc::borrow::Cow::Borrowed( ch_str ),
        bounds : ( pos, pos + ch.len_utf8() ),
        start : pos,
        end : pos + ch.len_utf8(),
        typ : crate::item_adapter::SplitType::Delimiter,
        was_quoted : false,
      });
      pos += ch.len_utf8();
    }
    else
    {
      handle_non_delimiter_segment( input, &mut pos, delimiters, &mut result );
    }
  }
  
  result
}

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
    // Simple replacement for strs_tools split since the feature is not available
    let splits_iter = simple_split( input, &[ " ", "\n", "\t", "\r", "::", "?", "#", ".", "!" ] );

    let rich_items : Vec< RichItem< '_ > > = splits_iter
    .into_iter()
    .map( | s |
    {
      let ( kind, adjusted_source_location ) = crate::item_adapter::classify_split( &s )?;
      Ok( RichItem::new( s, kind, adjusted_source_location ) )
    })
    .collect::< Result< Vec< RichItem< '_ > >, ParseError > >()?;

    let rich_items : Vec< RichItem< '_ > > = rich_items
    .into_iter()
    .filter( | item | !matches!( item.kind, UnilangTokenKind::Delimiter( " " | "\n" | "\t" | "\r" ) ) )
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
  pub fn parse_multiple_instructions( &self, input : &str ) -> Result< Vec< crate::instruction::GenericInstruction >, ParseError >
  {
    // Use standard string split instead of simple_split to avoid interference with :: operator
    let parts: Vec<&str> = input.split(";;").collect();
    let mut instructions = Vec::new();

    // Handle empty input
    if parts.is_empty() || (parts.len() == 1 && parts[0].trim().is_empty())
    {
      return Ok( Vec::new() );
    }

    // Check for invalid patterns
    if input.starts_with(";;")
    {
      return Err( ParseError::new
      (
        ErrorKind::EmptyInstructionSegment,
        SourceLocation::StrSpan { start: 0, end: 2 },
      ));
    }
    

    // Check for consecutive delimiters
    if input.contains(";;;;")
    {
      let pos = input.find(";;;;").unwrap();
      return Err( ParseError::new
      (
        ErrorKind::EmptyInstructionSegment,
        SourceLocation::StrSpan { start: pos, end: pos + 4 },
      ));
    }

    // Parse each part as an instruction
    for (i, part) in parts.iter().enumerate()
    {
      let trimmed = part.trim();
      if trimmed.is_empty()
      {
        // Empty part - need to determine if this is trailing delimiter or empty segment
        if i == parts.len() - 1 && input.contains(";;")
        {
          // This is the last part and it's empty, which means we have a trailing delimiter
          let semicolon_pos = input.rfind(";;").unwrap();
          return Err( ParseError::new
          (
            ErrorKind::TrailingDelimiter,
            SourceLocation::StrSpan 
            { 
              start: semicolon_pos, 
              end: semicolon_pos + 2
            },
          ));
        }
        // Empty part between delimiters  
        let part_start = input.find(part).unwrap_or(0);
        return Err( ParseError::new
        (
          ErrorKind::EmptyInstructionSegment,
          SourceLocation::StrSpan 
          { 
            start: part_start, 
            end: part_start + part.len().max(1)
          },
        ));
      }
      let instruction = self.parse_single_instruction( trimmed )?;
      instructions.push( instruction );
    }

    Ok( instructions )
  }

  /// Parses a single Unilang instruction from a list of rich items.
  fn parse_single_instruction_from_rich_items
  (
    &self,
    rich_items : Vec< RichItem< '_ > >,
  )
  -> Result< crate::instruction::GenericInstruction, ParseError >
  {
    // Handle empty input (after filtering whitespace)

    if rich_items.is_empty()
    {
      return Ok( GenericInstruction
      {
        command_path_slices : Vec::new(),
        positional_arguments : Vec::new(),
        named_arguments : BTreeMap::new(),
        help_requested : false,
        overall_location : SourceLocation::None, // No specific location for empty input
      });
    }

    let instruction_start_location = rich_items.first().map_or( 0, | item | item.inner.start );
    let instruction_end_location = rich_items.last().map_or( instruction_start_location, | item | item.inner.end );

    let mut items_iter = rich_items.into_iter().peekable();

    // Handle optional leading dot as per spec.md Rule 3.1
    if let Some( first_item ) = items_iter.peek()
    {
      if let UnilangTokenKind::Delimiter( "." ) = &first_item.kind
      {
        if first_item.inner.start == 0
        {
          // Ensure it's truly a leading dot at the beginning of the input
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
      overall_location : SourceLocation::StrSpan
      {
        start : instruction_start_location,
        end : instruction_end_location,
      },
    })
  }

  /// Parses the command path from a peekable iterator of rich items.
  fn parse_command_path
  (
    items_iter : &mut core::iter::Peekable< IntoIter< RichItem< '_ > > >,
    instruction_end_location : usize,
  )
  -> Result< Vec< String >, ParseError >
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
            if s.contains( '-' )
            {
              return Err( ParseError::new
              (
                ErrorKind::Syntax( format!( "Invalid character '-' in command path segment '{s}'" ) ),
                item.adjusted_source_location.clone(),
              ));
            }
            command_path_slices.push( s.clone() );
            last_token_was_dot = false;
            items_iter.next(); // Consume item
          }
          else
          {
            break; // End of command path
          }
        }
        UnilangTokenKind::Delimiter( "." ) =>
        {
          if last_token_was_dot
          // Consecutive dots, e.g., "cmd..sub"
          {
            return Err( ParseError::new
            (
              ErrorKind::Syntax( "Consecutive dots in command path".to_string() ),
              item.adjusted_source_location.clone(),
            ));
          }
          last_token_was_dot = true;
          items_iter.next(); // Consume item
        }
        UnilangTokenKind::Unrecognized( ref s ) | UnilangTokenKind::Number( ref s ) =>
        {
          if last_token_was_dot
          {
            return Err( ParseError::new
            (
              ErrorKind::Syntax( format!( "Invalid identifier '{s}' in command path" ) ),
              item.adjusted_source_location.clone(),
            ));
          }
          break; // End of command path
        }
        _ =>
        {
          break; // End of command path
        }
      }
    }

    if last_token_was_dot
    {
      // If the last token was a dot, and we are at the end of the command path,
      // it's a trailing dot error. The location should be the end of the instruction.
      return Err( ParseError::new
      (
        ErrorKind::Syntax( "Command path cannot end with a '.'".to_string() ),
        SourceLocation::StrSpan
        {
          start : instruction_end_location - 1,
          end : instruction_end_location,
        },
      ));
    }

    Ok( command_path_slices )
  }

  /// Parses arguments from a peekable iterator of rich items.
  #[ allow( clippy::type_complexity ) ]
  #[ allow( clippy::too_many_lines ) ]
  fn parse_arguments
  (
    &self,
    items_iter : &mut core::iter::Peekable< IntoIter< RichItem< '_ > > >,
  )
  -> Result< ( Vec< Argument >, BTreeMap< String, Argument >, bool ), ParseError >
  {
    let mut positional_arguments = Vec::new();
    let mut named_arguments = BTreeMap::new();
    let mut help_operator_found = false;

    while let Some( item ) = items_iter.next()
    {
      match item.kind
      {
        UnilangTokenKind::Unrecognized( ref s ) =>
        {
          return Err( ParseError::new
          (
            ErrorKind::Syntax( format!( "Unexpected token '{s}' in arguments" ) ),
            item.adjusted_source_location.clone(),
          ));
        }

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
                  UnilangTokenKind::Identifier( ref val )
                  | UnilangTokenKind::Unrecognized( ref val )
                  | UnilangTokenKind::Number( ref val ) =>
                  {
                    let mut current_value = val.clone();
                    let mut current_value_end_location = match value_item.source_location()
                    {
                      SourceLocation::StrSpan { end, .. } => end,
                      SourceLocation::None => 0, // Default or handle error appropriately
                    };

                    // Loop to consume subsequent path segments
                    loop
                    {
                      let Some( peeked_dot ) = items_iter.peek() else
                      {
                        break;
                      };
                      if let UnilangTokenKind::Delimiter( "." ) = &peeked_dot.kind
                      {
                        let _dot_item = items_iter.next().unwrap(); // Consume the dot
                        let Some( peeked_segment ) = items_iter.peek() else
                        {
                          break;
                        };
                        if let UnilangTokenKind::Identifier( ref s ) = &peeked_segment.kind
                        {
                          current_value.push( '.' );
                          current_value.push_str( s );
                          current_value_end_location = match peeked_segment.source_location()
                          {
                            SourceLocation::StrSpan { end, .. } => end,
                            SourceLocation::None => current_value_end_location, // Keep previous if None
                          };
                          items_iter.next(); // Consume the segment
                        }
                        else if let UnilangTokenKind::Unrecognized( ref s ) = &peeked_segment.kind
                        {
                          current_value.push( '.' );
                          current_value.push_str( s );
                          current_value_end_location = match peeked_segment.source_location()
                          {
                            SourceLocation::StrSpan { end, .. } => end,
                            SourceLocation::None => current_value_end_location, // Keep previous if None
                          };
                          items_iter.next(); // Consume the segment
                        }
                        else if let UnilangTokenKind::Number( ref s ) = &peeked_segment.kind
                        {
                          current_value.push( '.' );
                          current_value.push_str( s );
                          current_value_end_location = match peeked_segment.source_location()
                          {
                            SourceLocation::StrSpan { end, .. } => end,
                            SourceLocation::None => current_value_end_location, // Keep previous if None
                          };
                          items_iter.next(); // Consume the segment
                        }
                        else
                        {
                          // Not a valid path segment after dot, break
                          break;
                        }
                      }
                      else
                      {
                        break; // Next item is not a dot, end of path segments
                      }
                    }

                    if named_arguments.contains_key( arg_name )
                      && self.options.error_on_duplicate_named_arguments
                      {
                        return Err( ParseError::new
                        (
                          ErrorKind::Syntax( format!( "Duplicate named argument '{arg_name}'" ) ),
                          value_item.source_location(),
                        ));
                      }
                      // If not erroring on duplicates, the new value will overwrite the old one
                    named_arguments.insert
                    (
                      arg_name.clone(),
                      Argument
                      {
                        name : Some( arg_name.clone() ),
                        value : current_value,
                        name_location : Some( item.source_location() ),
                        value_location : SourceLocation::StrSpan
                        {
                          start : match value_item.source_location()
                          {
                            SourceLocation::StrSpan { start, .. } => start,
                            SourceLocation::None => 0,
                          },
                          end : current_value_end_location,
                        },
                      },
                    );
                  }
                  UnilangTokenKind::Delimiter( "." ) =>
                  {
                    // Handle file paths that start with "./" or "../"
                    let mut current_value = ".".to_string();
                    let mut current_value_end_location = match value_item.source_location()
                    {
                      SourceLocation::StrSpan { end, .. } => end,
                      SourceLocation::None => 0,
                    };

                    // Continue building the path starting with "."
                    // Look for the next token after "."
                    if let Some( next_item ) = items_iter.peek() {
                      match &next_item.kind {
                        UnilangTokenKind::Unrecognized( ref s ) => {
                          // This handles cases like "./examples" where "/examples" is unrecognized
                          current_value.push_str( s );
                          current_value_end_location = match next_item.source_location() {
                            SourceLocation::StrSpan { end, .. } => end,
                            SourceLocation::None => current_value_end_location,
                          };
                          items_iter.next(); // Consume the unrecognized token
                        }
                        UnilangTokenKind::Delimiter( "." ) => {
                          // This handles "../" patterns
                          current_value.push( '.' );
                          current_value_end_location = match next_item.source_location() {
                            SourceLocation::StrSpan { end, .. } => end,
                            SourceLocation::None => current_value_end_location,
                          };
                          items_iter.next(); // Consume the second dot
                          
                          // Look for the next token after ".."
                          if let Some( third_item ) = items_iter.peek() {
                            if let UnilangTokenKind::Unrecognized( ref s ) = &third_item.kind {
                              current_value.push_str( s );
                              current_value_end_location = match third_item.source_location() {
                                SourceLocation::StrSpan { end, .. } => end,
                                SourceLocation::None => current_value_end_location,
                              };
                              items_iter.next(); // Consume the unrecognized token
                            }
                          }
                        }
                        _ => {
                          // Other cases - not a file path, just leave as is
                        }
                      }

                      // Continue with the normal path-building loop for any additional dots
                      loop
                      {
                        let Some( peeked_dot ) = items_iter.peek() else
                        {
                          break;
                        };
                        if let UnilangTokenKind::Delimiter( "." ) = &peeked_dot.kind
                        {
                          let _dot_item = items_iter.next().unwrap(); // Consume the dot
                          let Some( peeked_segment ) = items_iter.peek() else
                          {
                            break;
                          };
                          if let UnilangTokenKind::Identifier( ref s ) = &peeked_segment.kind
                          {
                            current_value.push( '.' );
                            current_value.push_str( s );
                            current_value_end_location = match peeked_segment.source_location()
                            {
                              SourceLocation::StrSpan { end, .. } => end,
                              SourceLocation::None => current_value_end_location,
                            };
                            items_iter.next(); // Consume the segment
                          }
                          else if let UnilangTokenKind::Unrecognized( ref s ) = &peeked_segment.kind
                          {
                            current_value.push( '.' );
                            current_value.push_str( s );
                            current_value_end_location = match peeked_segment.source_location()
                            {
                              SourceLocation::StrSpan { end, .. } => end,
                              SourceLocation::None => current_value_end_location,
                            };
                            items_iter.next(); // Consume the segment
                          }
                          else if let UnilangTokenKind::Number( ref s ) = &peeked_segment.kind
                          {
                            current_value.push( '.' );
                            current_value.push_str( s );
                            current_value_end_location = match peeked_segment.source_location()
                            {
                              SourceLocation::StrSpan { end, .. } => end,
                              SourceLocation::None => current_value_end_location,
                            };
                            items_iter.next(); // Consume the segment
                          }
                          else
                          {
                            break;
                          }
                        }
                        else
                        {
                          break;
                        }
                      }
                    }

                    if named_arguments.contains_key( arg_name )
                      && self.options.error_on_duplicate_named_arguments
                      {
                        return Err( ParseError::new
                        (
                          ErrorKind::Syntax( format!( "Duplicate named argument '{arg_name}'" ) ),
                          value_item.source_location(),
                        ));
                      }
                      // If not erroring on duplicates, the new value will overwrite the old one
                    named_arguments.insert
                    (
                      arg_name.clone(),
                      Argument
                      {
                        name : Some( arg_name.clone() ),
                        value : current_value,
                        name_location : Some( item.source_location() ),
                        value_location : SourceLocation::StrSpan
                        {
                          start : match value_item.source_location()
                          {
                            SourceLocation::StrSpan { start, .. } => start,
                            SourceLocation::None => 0,
                          },
                          end : current_value_end_location,
                        },
                      },
                    );
                  }
                  _ =>
                  {
                    return Err( ParseError::new
                    (
                      ErrorKind::Syntax( format!( "Expected value for named argument '{arg_name}'" ) ),
                      value_item.source_location(),
                    ))
                  }
                }
              }
              else
              {
                return Err( ParseError::new
                (
                  ErrorKind::Syntax( format!(
                    "Expected value for named argument '{arg_name}' but found end of instruction"
                  ) ),
                  item.adjusted_source_location.clone(),
                ));
              }
            }
            else
            {
              // Positional argument
              if !named_arguments.is_empty() && self.options.error_on_positional_after_named
              {
                return Err( ParseError::new
                (
                  ErrorKind::Syntax( "Positional argument after named argument".to_string() ),
                  item.adjusted_source_location.clone(),
                ));
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
              return Err( ParseError::new
              (
                ErrorKind::Syntax( "Positional argument after named argument".to_string() ),
                item.adjusted_source_location.clone(),
              ));
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
        UnilangTokenKind::Number( ref s ) =>
        {
          // Positional argument
          if !named_arguments.is_empty() && self.options.error_on_positional_after_named
          {
            return Err( ParseError::new
            (
              ErrorKind::Syntax( "Positional argument after named argument".to_string() ),
              item.adjusted_source_location.clone(),
            ));
          }
          positional_arguments.push( Argument
          {
            name : None,
            value : s.clone(),
            name_location : None,
            value_location : item.source_location(),
          });
        }
        UnilangTokenKind::Operator( "?" ) =>
        {
          if items_iter.peek().is_some()
          {
            return Err( ParseError::new
            (
              ErrorKind::Syntax( "Help operator '?' must be the last token".to_string() ),
              item.adjusted_source_location.clone(),
            ));
          }
          help_operator_found = true;
        }
        _ =>
        {
          return Err( ParseError::new
          (
            ErrorKind::Syntax( format!( "Unexpected token '{}' in arguments", item.inner.string ) ),
            item.adjusted_source_location.clone(),
          ));
        }
      }
    }

    Ok( ( positional_arguments, named_arguments, help_operator_found ) )
  }
}
