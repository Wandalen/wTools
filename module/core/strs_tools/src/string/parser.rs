//! Parser integration for single-pass string processing operations.
//!
//! This module provides integrated parsing operations that combine tokenization,
//! validation, and transformation in single passes for optimal performance.

use std::marker::PhantomData;
use crate::string::zero_copy::ZeroCopyStringExt;

/// Error types for parsing operations
#[ derive( Debug, Clone ) ]
pub enum ParseError
{
  /// Invalid token encountered during parsing
  InvalidToken
  {
    /// The token that failed to parse
    token: String,
    /// Position in the input where the token was found
    position: usize,
    /// Description of what was expected
    expected: String,
  },
  /// Validation failed for a token
  ValidationFailed
  {
    /// The token that failed validation
    token: String,
    /// Position in the input where the token was found
    position: usize,
    /// Reason why validation failed
    reason: String,
  },
  /// Unexpected end of input
  UnexpectedEof
  {
    /// Position where end of input was encountered
    position: usize,
    /// Description of what was expected
    expected: String,
  },
  /// Invalid key-value pair format
  InvalidKeyValuePair( String ),
  /// Unknown key in parsing context
  UnknownKey( String ),
  /// I/O error during streaming operations (not cloneable, stored as string)
  IoError( String ),
}

impl std::fmt::Display for ParseError
{
  fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    match self
    {
      ParseError::InvalidToken { token, position, expected } =>
        write!( f, "Invalid token '{}' at position {}, expected: {}", token, position, expected ),
      ParseError::ValidationFailed { token, position, reason } =>
        write!( f, "Validation failed for '{}' at position {}: {}", token, position, reason ),
      ParseError::UnexpectedEof { position, expected } =>
        write!( f, "Unexpected end of input at position {}, expected: {}", position, expected ),
      ParseError::InvalidKeyValuePair( pair ) =>
        write!( f, "Invalid key-value pair format: '{}'", pair ),
      ParseError::UnknownKey( key ) =>
        write!( f, "Unknown key: '{}'", key ),
      ParseError::IoError( e ) =>
        write!( f, "I/O error: {}", e ),
    }
  }
}

impl std::error::Error for ParseError {}

impl ParseError
{
  /// Add position information to error
  pub fn with_position( mut self, pos: usize ) -> Self
  {
    match &mut self
    {
      ParseError::InvalidToken { position, .. } => *position = pos,
      ParseError::ValidationFailed { position, .. } => *position = pos,
      ParseError::UnexpectedEof { position, .. } => *position = pos,
      _ => {},
    }
    self
  }
}

/// Single-pass token parsing iterator that combines splitting and parsing
pub struct TokenParsingIterator< 'a, F, T >
{
  input: &'a str,
  delimiters: Vec< &'a str >,
  parser_func: F,
  position: usize,
  _phantom: PhantomData< T >,
}

impl< 'a, F, T > std::fmt::Debug for TokenParsingIterator< 'a, F, T >
{
  fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    f.debug_struct( "TokenParsingIterator" )
      .field( "input", &self.input )
      .field( "delimiters", &self.delimiters )
      .field( "position", &self.position )
      .field( "parser_func", &"<function>" )
      .finish()
  }
}

impl< 'a, F, T > TokenParsingIterator< 'a, F, T >
where
  F: Fn( &str ) -> Result< T, ParseError >,
{
  /// Create new token parsing iterator
  pub fn new( input: &'a str, delimiters: Vec< &'a str >, parser: F ) -> Self
  {
    Self
    {
      input,
      delimiters,
      parser_func: parser,
      position: 0,
      _phantom: PhantomData,
    }
  }

  /// Find next token using simple string operations
  fn find_next_token( &mut self ) -> Option< &'a str >
  {
    loop
    {
      if self.position >= self.input.len()
      {
        return None;
      }

      let remaining = &self.input[ self.position.. ];
      
      // Find the earliest delimiter match
      let mut earliest_delim_pos = None;
      let mut earliest_delim_len = 0;
      
      for delim in &self.delimiters
      {
        if let Some( pos ) = remaining.find( delim )
        {
          match earliest_delim_pos
          {
            None => 
            {
              earliest_delim_pos = Some( pos );
              earliest_delim_len = delim.len();
            },
            Some( current_pos ) if pos < current_pos =>
            {
              earliest_delim_pos = Some( pos );
              earliest_delim_len = delim.len();
            },
            _ => {} // Keep current earliest
          }
        }
      }
      
      let token = if let Some( delim_pos ) = earliest_delim_pos
      {
        // Token is everything before the delimiter
        let token = &remaining[ ..delim_pos ];
        self.position += delim_pos + earliest_delim_len;
        token
      }
      else
      {
        // No delimiter found, rest of input is the token
        let token = remaining;
        self.position = self.input.len();
        token
      };
      
      if !token.is_empty()
      {
        return Some( token );
      }
      
      // If token is empty, continue loop to find next non-empty token
    }
  }
}

impl< 'a, F, T > Iterator for TokenParsingIterator< 'a, F, T >
where
  F: Fn( &str ) -> Result< T, ParseError >,
{
  type Item = Result< T, ParseError >;

  fn next( &mut self ) -> Option< Self::Item >
  {
    let token = self.find_next_token()?;
    Some( ( self.parser_func )( token ) )
  }
}

/// Parse and split in single operation
pub fn parse_and_split< 'a, T, F >(
  input: &'a str,
  delimiters: &'a [ &'a str ],
  parser: F,
) -> TokenParsingIterator< 'a, F, T >
where
  F: Fn( &str ) -> Result< T, ParseError >,
{
  TokenParsingIterator::new( input, delimiters.to_vec(), parser )
}

/// Parsed token types for structured command-line parsing
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum ParsedToken< 'a >
{
  /// Command name
  Command( &'a str ),
  /// Key-value pair argument
  KeyValue
  {
    /// The key part of the pair
    key: &'a str,
    /// The value part of the pair
    value: &'a str,
  },
  /// Flag argument (starts with --)
  Flag( &'a str ),
  /// Positional argument
  Positional( &'a str ),
}

impl< 'a > ParsedToken< 'a >
{
  /// Get the string content of the token
  pub fn as_str( &self ) -> &'a str
  {
    match self
    {
      ParsedToken::Command( s ) => s,
      ParsedToken::KeyValue { key, .. } => key, // Return key by default
      ParsedToken::Flag( s ) => s,
      ParsedToken::Positional( s ) => s,
    }
  }

  /// Check if this token is a specific type
  pub fn is_command( &self ) -> bool
  {
    matches!( self, ParsedToken::Command( _ ) )
  }

  /// Check if this token is a flag
  pub fn is_flag( &self ) -> bool
  {
    matches!( self, ParsedToken::Flag( _ ) )
  }

  /// Check if this token is a key-value pair
  pub fn is_key_value( &self ) -> bool
  {
    matches!( self, ParsedToken::KeyValue { .. } )
  }

  /// Check if this token is a positional argument
  pub fn is_positional( &self ) -> bool
  {
    matches!( self, ParsedToken::Positional( _ ) )
  }
}

/// Parser context for state-aware parsing
#[ derive( Debug, Clone, Copy ) ]
enum ParsingContext
{
  /// Expecting command name
  Command,
  /// Expecting arguments or flags
  Arguments,
  /// Expecting value after key (reserved for future use)
  #[ allow( dead_code ) ]
  Value,
}

/// Structured command-line parser with context awareness
#[ derive( Debug, Clone ) ]
pub struct CommandParser< 'a >
{
  input: &'a str,
  token_delimiters: Vec< &'a str >,
  kv_separator: &'a str,
  flag_prefix: &'a str,
}

impl< 'a > CommandParser< 'a >
{
  /// Create new command parser with default settings
  pub fn new( input: &'a str ) -> Self
  {
    Self
    {
      input,
      token_delimiters: vec![ " ", "\t" ],
      kv_separator: ":",
      flag_prefix: "--",
    }
  }

  /// Set custom token delimiters
  pub fn with_token_delimiters( mut self, delimiters: Vec< &'a str > ) -> Self
  {
    self.token_delimiters = delimiters;
    self
  }

  /// Set custom key-value separator
  pub fn with_kv_separator( mut self, separator: &'a str ) -> Self
  {
    self.kv_separator = separator;
    self
  }

  /// Set custom flag prefix
  pub fn with_flag_prefix( mut self, prefix: &'a str ) -> Self
  {
    self.flag_prefix = prefix;
    self
  }

  /// Parse command line in single pass with context awareness
  pub fn parse_structured( self ) -> impl Iterator< Item = Result< ParsedToken< 'a >, ParseError > > + 'a
  {
    StructuredParsingIterator
    {
      parser: self,
      position: 0,
      current_context: ParsingContext::Command,
    }
  }
}

/// Internal iterator for structured parsing
struct StructuredParsingIterator< 'a >
{
  parser: CommandParser< 'a >,
  position: usize,
  current_context: ParsingContext,
}

impl< 'a > StructuredParsingIterator< 'a >
{
  /// Find next token boundary using position-based slicing
  fn find_next_token( &mut self ) -> Option< &'a str >
  {
    loop
    {
      if self.position >= self.parser.input.len()
      {
        return None;
      }

      let remaining = &self.parser.input[ self.position.. ];
      
      // Find the earliest delimiter match
      let mut earliest_delim_pos = None;
      let mut earliest_delim_len = 0;
      
      for delim in &self.parser.token_delimiters
      {
        if let Some( pos ) = remaining.find( delim )
        {
          match earliest_delim_pos
          {
            None => 
            {
              earliest_delim_pos = Some( pos );
              earliest_delim_len = delim.len();
            },
            Some( current_pos ) if pos < current_pos =>
            {
              earliest_delim_pos = Some( pos );
              earliest_delim_len = delim.len();
            },
            _ => {} // Keep current earliest
          }
        }
      }
      
      let (token_start, token_end) = if let Some( delim_pos ) = earliest_delim_pos
      {
        // Token is everything before the delimiter
        let token_start = self.position;
        let token_end = self.position + delim_pos;
        self.position += delim_pos + earliest_delim_len;
        (token_start, token_end)
      }
      else
      {
        // No delimiter found, rest of input is the token
        let token_start = self.position;
        let token_end = self.parser.input.len();
        self.position = self.parser.input.len();
        (token_start, token_end)
      };
      
      if token_start < token_end
      {
        let token = &self.parser.input[ token_start..token_end ];
        if !token.is_empty()
        {
          return Some( token );
        }
      }
      
      // If token is empty, continue loop to find next non-empty token
    }
  }

  /// Parse argument token based on context and characteristics
  fn parse_argument_token( &mut self, token: &'a str ) -> Result< ParsedToken< 'a >, ParseError >
  {
    // Check for key-value pairs first (can start with flag prefix)
    if token.contains( self.parser.kv_separator )
    {
      let separator_pos = token.find( self.parser.kv_separator ).unwrap();
      let key_part = &token[ ..separator_pos ];
      let value = &token[ separator_pos + self.parser.kv_separator.len().. ];

      // Extract key from potential flag prefix
      let key = if key_part.starts_with( self.parser.flag_prefix )
      {
        &key_part[ self.parser.flag_prefix.len().. ]
      }
      else
      {
        key_part
      };

      if key.is_empty() || value.is_empty()
      {
        Err( ParseError::InvalidKeyValuePair( token.to_string() ) )
      }
      else
      {
        Ok( ParsedToken::KeyValue { key, value } )
      }
    }
    else if token.starts_with( self.parser.flag_prefix )
    {
      // Flag argument
      let flag_name = &token[ self.parser.flag_prefix.len().. ];
      Ok( ParsedToken::Flag( flag_name ) )
    }
    else
    {
      // Positional argument
      Ok( ParsedToken::Positional( token ) )
    }
  }
}

impl< 'a > Iterator for StructuredParsingIterator< 'a >
{
  type Item = Result< ParsedToken< 'a >, ParseError >;

  fn next( &mut self ) -> Option< Self::Item >
  {
    let token = self.find_next_token()?;

    // Parse based on current context and token characteristics
    let result = match self.current_context
    {
      ParsingContext::Command =>
      {
        self.current_context = ParsingContext::Arguments;
        Ok( ParsedToken::Command( token ) )
      },
      ParsingContext::Arguments =>
      {
        self.parse_argument_token( token )
      },
      ParsingContext::Value =>
      {
        self.current_context = ParsingContext::Arguments;
        Ok( ParsedToken::Positional( token ) ) // Previous token was expecting this value
      },
    };

    Some( result )
  }
}

/// Manual split iterator for validation that preserves lifetime references
pub struct ManualSplitIterator< 'a, F >
{
  /// Input string to split
  input: &'a str,
  /// Delimiters to split on
  delimiters: Vec< &'a str >,
  /// Validation function for each token
  validator: F,
  /// Current position in input string
  position: usize,
}

impl< 'a, F > std::fmt::Debug for ManualSplitIterator< 'a, F >
{
  fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    f.debug_struct( "ManualSplitIterator" )
      .field( "input", &self.input )
      .field( "delimiters", &self.delimiters )
      .field( "position", &self.position )
      .field( "validator", &"<function>" )
      .finish()
  }
}

impl< 'a, F > ManualSplitIterator< 'a, F >
where
  F: Fn( &str ) -> bool,
{
  /// Create a new manual split iterator with validation
  pub fn new( input: &'a str, delimiters: &'a [ &'a str ], validator: F ) -> Self
  {
    Self
    {
      input,
      delimiters: delimiters.to_vec(),
      validator,
      position: 0,
    }
  }

  fn find_next_token( &mut self ) -> Option< &'a str >
  {
    loop
    {
      if self.position >= self.input.len()
      {
        return None;
      }

      let remaining = &self.input[ self.position.. ];
      
      // Find the earliest delimiter match
      let mut earliest_delim_pos = None;
      let mut earliest_delim_len = 0;
      
      for delim in &self.delimiters
      {
        if let Some( pos ) = remaining.find( delim )
        {
          match earliest_delim_pos
          {
            None => 
            {
              earliest_delim_pos = Some( pos );
              earliest_delim_len = delim.len();
            },
            Some( current_pos ) if pos < current_pos =>
            {
              earliest_delim_pos = Some( pos );
              earliest_delim_len = delim.len();
            },
            _ => {} // Keep current earliest
          }
        }
      }
      
      let (token_start, token_end) = if let Some( delim_pos ) = earliest_delim_pos
      {
        // Token is everything before the delimiter
        let token_start = self.position;
        let token_end = self.position + delim_pos;
        self.position += delim_pos + earliest_delim_len;
        (token_start, token_end)
      }
      else
      {
        // No delimiter found, rest of input is the token
        let token_start = self.position;
        let token_end = self.input.len();
        self.position = self.input.len();
        (token_start, token_end)
      };
      
      if token_start < token_end
      {
        return Some( &self.input[ token_start..token_end ] );
      }
      // If token is empty, continue loop to find next non-empty token
    }
  }
}

impl< 'a, F > Iterator for ManualSplitIterator< 'a, F >
where
  F: Fn( &str ) -> bool,
{
  type Item = Result< &'a str, ParseError >;

  fn next( &mut self ) -> Option< Self::Item >
  {
    let token = self.find_next_token()?;
    
    if ( self.validator )( token )
    {
      Some( Ok( token ) )
    }
    else
    {
      Some( Err( ParseError::ValidationFailed
      {
        token: token.to_string(),
        position: self.position,
        reason: "Validation failed".to_string(),
      } ) )
    }
  }
}

/// Extension trait adding parser integration to string types
pub trait ParserIntegrationExt
{
  /// Parse tokens while splitting in single pass
  fn split_and_parse< 'a, T: 'a, F >(
    &'a self,
    delimiters: &'a [ &'a str ],
    parser: F,
  ) -> impl Iterator< Item = Result< T, ParseError > > + 'a
  where
    F: Fn( &str ) -> Result< T, ParseError > + 'a;

  /// Split with validation using zero-copy operations
  fn split_with_validation< 'a, F >(
    &'a self,
    delimiters: &'a [ &'a str ],
    validator: F,
  ) -> impl Iterator< Item = Result< &'a str, ParseError > > + 'a
  where
    F: Fn( &str ) -> bool + 'a;

  /// Parse structured command line arguments
  fn parse_command_line< 'a >( &'a self ) -> impl Iterator< Item = Result< ParsedToken< 'a >, ParseError > > + 'a;

  /// Count tokens that pass validation without allocation
  fn count_valid_tokens< F >( &self, delimiters: &[ &str ], validator: F ) -> usize
  where
    F: Fn( &str ) -> bool;
}

impl ParserIntegrationExt for str
{
  fn split_and_parse< 'a, T: 'a, F >(
    &'a self,
    delimiters: &'a [ &'a str ],
    parser: F,
  ) -> impl Iterator< Item = Result< T, ParseError > > + 'a
  where
    F: Fn( &str ) -> Result< T, ParseError > + 'a,
  {
    parse_and_split( self, delimiters, parser )
  }

  fn split_with_validation< 'a, F >(
    &'a self,
    delimiters: &'a [ &'a str ],
    validator: F,
  ) -> impl Iterator< Item = Result< &'a str, ParseError > > + 'a
  where
    F: Fn( &str ) -> bool + 'a,
  {
    // Use manual splitting that can return references to original string
    ManualSplitIterator::new( self, delimiters, validator )
  }

  fn parse_command_line< 'a >( &'a self ) -> impl Iterator< Item = Result< ParsedToken< 'a >, ParseError > > + 'a
  {
    CommandParser::new( self ).parse_structured()
  }

  fn count_valid_tokens< F >( &self, delimiters: &[ &str ], validator: F ) -> usize
  where
    F: Fn( &str ) -> bool,
  {
    self.zero_copy_split( delimiters )
      .filter( |segment| validator( segment.as_str() ) )
      .count()
  }
}

impl ParserIntegrationExt for String
{
  fn split_and_parse< 'a, T: 'a, F >(
    &'a self,
    delimiters: &'a [ &'a str ],
    parser: F,
  ) -> impl Iterator< Item = Result< T, ParseError > > + 'a
  where
    F: Fn( &str ) -> Result< T, ParseError > + 'a,
  {
    self.as_str().split_and_parse( delimiters, parser )
  }

  fn split_with_validation< 'a, F >(
    &'a self,
    delimiters: &'a [ &'a str ],
    validator: F,
  ) -> impl Iterator< Item = Result< &'a str, ParseError > > + 'a
  where
    F: Fn( &str ) -> bool + 'a,
  {
    self.as_str().split_with_validation( delimiters, validator )
  }

  fn parse_command_line< 'a >( &'a self ) -> impl Iterator< Item = Result< ParsedToken< 'a >, ParseError > > + 'a
  {
    self.as_str().parse_command_line()
  }

  fn count_valid_tokens< F >( &self, delimiters: &[ &str ], validator: F ) -> usize
  where
    F: Fn( &str ) -> bool,
  {
    self.as_str().count_valid_tokens( delimiters, validator )
  }
}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn test_parse_and_split_integers()
  {
    let input = "1,2,3,4,5";
    let result: Result< Vec< i32 >, _ > = input
      .split_and_parse( &[ "," ], |token| {
        token.parse().map_err( |_| ParseError::InvalidToken {
          token: token.to_string(),
          position: 0,
          expected: "integer".to_string(),
        } )
      } )
      .collect();

    assert!( result.is_ok() );
    let numbers = result.unwrap();
    assert_eq!( numbers, vec![ 1, 2, 3, 4, 5 ] );
  }

  #[ test ]
  fn test_command_line_parsing()
  {
    let input = "myapp --verbose input.txt output.txt";
    let result: Result< Vec< _ >, _ > = input.parse_command_line().collect();

    assert!( result.is_ok() );
    let tokens = result.unwrap();
    
    assert_eq!( tokens.len(), 4 );
    assert!( matches!( tokens[ 0 ], ParsedToken::Command( "myapp" ) ) );
    assert!( matches!( tokens[ 1 ], ParsedToken::Flag( "verbose" ) ) );
    assert!( matches!( tokens[ 2 ], ParsedToken::Positional( "input.txt" ) ) );
    assert!( matches!( tokens[ 3 ], ParsedToken::Positional( "output.txt" ) ) );
  }

  #[ test ]
  fn test_key_value_parsing()
  {
    let input = "config timeout:30 retries:5";
    let result: Result< Vec< _ >, _ > = input.parse_command_line().collect();

    assert!( result.is_ok() );
    let tokens = result.unwrap();
    
    assert_eq!( tokens.len(), 3 );
    assert!( matches!( tokens[ 0 ], ParsedToken::Command( "config" ) ) );
    
    if let ParsedToken::KeyValue { key, value } = &tokens[ 1 ]
    {
      assert_eq!( *key, "timeout" );
      assert_eq!( *value, "30" );
    }
    else
    {
      panic!( "Expected KeyValue token" );
    }
    
    if let ParsedToken::KeyValue { key, value } = &tokens[ 2 ]
    {
      assert_eq!( *key, "retries" );
      assert_eq!( *value, "5" );
    }
    else
    {
      panic!( "Expected KeyValue token" );
    }
  }

  #[ test ]
  fn test_validation_during_split()
  {
    let input = "apple,123,banana,456,cherry";
    
    // Count only alphabetic tokens
    let alpha_count = input.count_valid_tokens( &[ "," ], |token| {
      token.chars().all( |c| c.is_alphabetic() )
    } );
    
    assert_eq!( alpha_count, 3 ); // apple, banana, cherry
  }

  #[ test ]
  fn test_empty_and_invalid_tokens()
  {
    let input = "valid,123,banana";
    let results: Vec< _ > = input
      .split_with_validation( &[ "," ], |token| token.chars().all( |c| c.is_alphabetic() ) )
      .collect();

    // Should have validation errors for "123" token (not alphabetic)
    assert!( results.iter().any( |r| r.is_err() ) );
    
    // Should have successful results for "valid" and "banana"
    assert!( results.iter().any( |r| r.is_ok() ) );
  }
}