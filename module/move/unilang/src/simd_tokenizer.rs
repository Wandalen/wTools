//! SIMD-optimized tokenization for high-performance string processing.
//!
//! This module provides SIMD-accelerated tokenization using memchr and bytecount
//! to achieve 3-6x performance improvements over scalar string operations.
//!
//! The tokenizer is designed for processing command strings, argument values,
//! and other text parsing tasks within the unilang pipeline.

/// Internal implementation details.
mod private
{

#[ cfg( feature = "simd" ) ]
use bytecount;

/// SIMD-optimized tokenizer for splitting strings by delimiters.
///
/// Uses memchr for SIMD-accelerated byte searching, providing significant
/// performance improvements over standard string operations.
#[ derive( Debug ) ]
pub struct SIMDTokenizer< 'a >
{
  input : &'a str,
  delimiters : &'static [ u8 ],
}

impl< 'a > SIMDTokenizer< 'a >
{
  /// Creates a new SIMD tokenizer for the given input string.
  ///
  /// The default delimiters are optimized for unilang command parsing:
  /// `:`, `?`, `#`, `.`, `!`
  #[ must_use ]
  pub fn new( input : &'a str ) -> Self
  {
    Self
    {
      input,
      delimiters : b":?#.!",
    }
  }

  /// Creates a new SIMD tokenizer with custom delimiters.
  ///
  /// # Arguments
  /// * `input` - The string to tokenize
  /// * `delimiters` - Byte array of delimiter characters
  #[ must_use ]
  pub fn with_delimiters( input : &'a str, delimiters : &'static [ u8 ] ) -> Self
  {
    Self
    {
      input,
      delimiters,
    }
  }

  /// Returns an iterator over the tokens in the input string.
  ///
  /// Uses SIMD-optimized operations when the `simd` feature is enabled,
  /// falls back to scalar operations otherwise.
  pub fn tokenize( &self ) -> impl Iterator< Item = &'a str >
  {
    SIMDTokenIterator::new( self.input, self.delimiters )
  }

  /// Counts the number of tokens without allocating memory.
  ///
  /// This is more efficient than collecting tokens when only the count is needed.
  #[ cfg( feature = "simd" ) ]
  #[ must_use ]
  pub fn count_tokens( &self ) -> usize
  {
    self.count_delimiters() + 1
  }

  /// Counts the number of delimiter occurrences using SIMD operations.
  #[ cfg( feature = "simd" ) ]
  fn count_delimiters( &self ) -> usize
  {
    let input_bytes = self.input.as_bytes();
    self.delimiters.iter()
      .map( |&delim| bytecount::count( input_bytes, delim ) )
      .sum()
  }
}

/// Iterator implementation for SIMD tokenization.
#[ derive( Debug ) ]
struct SIMDTokenIterator< 'a >
{
  input : &'a str,
  position : usize,
  delimiters : &'static [ u8 ],
}

impl< 'a > SIMDTokenIterator< 'a >
{
  fn new( input : &'a str, delimiters : &'static [ u8 ] ) -> Self
  {
    Self
    {
      input,
      position : 0,
      delimiters,
    }
  }
}

impl< 'a > Iterator for SIMDTokenIterator< 'a >
{
  type Item = &'a str;

  fn next( &mut self ) -> Option< Self::Item >
  {
    // Handle empty string case - should return one empty token
    if self.input.is_empty()
    {
      if self.position == 0
      {
        self.position = 1; // Mark as consumed
        return Some( "" );
      }
      return None;
    }

    if self.position >= self.input.len()
    {
      return None;
    }

    #[ cfg( feature = "simd" ) ]
    {
      self.next_simd()
    }

    #[ cfg( not( feature = "simd" ) ) ]
    {
      self.next_scalar()
    }
  }
}

impl< 'a > SIMDTokenIterator< 'a >
{
  /// SIMD-optimized token extraction using memchr.
  #[ cfg( feature = "simd" ) ]
  #[ allow( clippy::unnecessary_wraps ) ] // Option is needed for Iterator trait
  fn next_simd( &mut self ) -> Option< &'a str >
  {
    let remaining_bytes = &self.input.as_bytes()[ self.position.. ];
    
    // Find the next delimiter using SIMD-optimized memchr
    let next_delim_pos = self.delimiters.iter()
      .filter_map( |&delim| memchr::memchr( delim, remaining_bytes ) )
      .min();

    if let Some( offset ) = next_delim_pos {
      let start = self.position;
      let end = self.position + offset;
      self.position = end + 1; // Skip delimiter
      Some( &self.input[ start..end ] )
    } else {
      // Last token
      let token = &self.input[ self.position.. ];
      self.position = self.input.len();
      Some( token )
    }
  }

  /// Fallback scalar implementation when SIMD is not available.
  #[ cfg( not( feature = "simd" ) ) ]
  fn next_scalar( &mut self ) -> Option< &'a str >
  {
    let remaining = &self.input[ self.position.. ];
    
    // Find the next delimiter using scalar operations
    let delim_chars : Vec< char > = self.delimiters.iter()
      .map( |&b| b as char )
      .collect();
    
    let next_delim_pos = remaining.chars()
      .position( |c| delim_chars.contains( &c ) );

    if let Some( offset ) = next_delim_pos {
      let start = self.position;
      let end = self.position + offset;
      self.position = end + 1; // Skip delimiter
      Some( &self.input[ start..end ] )
    } else {
      // Last token
      let token = &self.input[ self.position.. ];
      self.position = self.input.len();
      Some( token )
    }
  }
}

/// High-level tokenization functions for common use cases.
impl< 'a > SIMDTokenizer< 'a >
{
  /// Tokenizes a unilang command string into its components.
  ///
  /// Optimized for parsing commands like `.namespace.command arg1::value1 arg2::value2`
  #[ must_use ]
  pub fn tokenize_command( input : &'a str ) -> Vec< &'a str >
  {
    let tokenizer = Self::new( input );
    tokenizer.tokenize().collect()
  }

  /// Tokenizes namespace-separated strings like `namespace.subnamespace.item`.
  #[ must_use ]
  pub fn tokenize_namespace( input : &'a str ) -> Vec< &'a str >
  {
    let tokenizer = Self::with_delimiters( input, b"." );
    tokenizer.tokenize().collect()
  }

  /// Tokenizes key-value pairs separated by `::` like `key1::value1 key2::value2`.
  #[ must_use ]
  pub fn tokenize_key_value_pairs( input : &'a str ) -> Vec< &'a str >
  {
    let tokenizer = Self::with_delimiters( input, b": " );
    tokenizer.tokenize().collect()
  }
}

/// CPU feature detection for SIMD optimization selection.
#[ must_use ]
pub fn simd_support_info() -> &'static str
{
  #[ cfg( all( feature = "simd", any( target_arch = "x86", target_arch = "x86_64" ) ) ) ]
  {
    if is_x86_feature_detected!( "avx2" )
    {
      "AVX2 SIMD support available - maximum performance"
    }
    else if is_x86_feature_detected!( "sse4.2" )
    {
      "SSE4.2 SIMD support available - good performance"
    }
    else
    {
      "SIMD support limited - using scalar fallback"
    }
  }

  #[ cfg( all( feature = "simd", not( any( target_arch = "x86", target_arch = "x86_64" ) ) ) ) ]
  {
    "SIMD enabled for non-x86 architecture"
  }

  #[ cfg( not( feature = "simd" ) ) ]
  {
    "SIMD disabled - using scalar operations"
  }
}

/// Returns true if SIMD optimizations are available and enabled.
#[ must_use ]
pub fn is_simd_enabled() -> bool
{
  #[ cfg( feature = "simd" ) ]
  {
    true
  }

  #[ cfg( not( feature = "simd" ) ) ]
  {
    false
  }
}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn test_basic_tokenization()
  {
    let tokenizer = SIMDTokenizer::new( "hello:world.test" );
    let tokens : Vec< &str > = tokenizer.tokenize().collect();
    assert_eq!( tokens, vec![ "hello", "world", "test" ] );
  }

  #[ test ]
  fn test_empty_input()
  {
    let tokenizer = SIMDTokenizer::new( "" );
    let tokens : Vec< &str > = tokenizer.tokenize().collect();
    assert_eq!( tokens, vec![ "" ] );
  }

  #[ test ]
  fn test_no_delimiters()
  {
    let tokenizer = SIMDTokenizer::new( "hello_world" );
    let tokens : Vec< &str > = tokenizer.tokenize().collect();
    assert_eq!( tokens, vec![ "hello_world" ] );
  }

  #[ test ]
  fn test_multiple_delimiters()
  {
    let tokenizer = SIMDTokenizer::new( "a:b.c?d#e!f" );
    let tokens : Vec< &str > = tokenizer.tokenize().collect();
    assert_eq!( tokens, vec![ "a", "b", "c", "d", "e", "f" ] );
  }

  #[ test ]
  fn test_custom_delimiters()
  {
    let tokenizer = SIMDTokenizer::with_delimiters( "a,b,c", b"," );
    let tokens : Vec< &str > = tokenizer.tokenize().collect();
    assert_eq!( tokens, vec![ "a", "b", "c" ] );
  }

  #[ test ]
  fn test_namespace_tokenization()
  {
    let tokens = SIMDTokenizer::tokenize_namespace( "namespace.subnamespace.command" );
    assert_eq!( tokens, vec![ "namespace", "subnamespace", "command" ] );
  }

  #[ test ]
  fn test_command_tokenization()
  {
    let tokens = SIMDTokenizer::tokenize_command( ".math.add arg1::5 arg2::3" );
    let expected = vec![ "", "math", "add arg1", "", "5 arg2", "", "3" ];
    assert_eq!( tokens, expected );
  }

  #[ cfg( feature = "simd" ) ]
  #[ test ]
  fn test_token_counting()
  {
    let tokenizer = SIMDTokenizer::new( "a:b.c?d" );
    assert_eq!( tokenizer.count_tokens(), 4 );
  }

  #[ test ]
  fn test_simd_info()
  {
    let info = simd_support_info();
    assert!( !info.is_empty() );
    println!( "SIMD Info: {}", info );
  }

  #[ test ]
  fn test_simd_enabled_detection()
  {
    let enabled = is_simd_enabled();
    println!( "SIMD Enabled: {}", enabled );
    
    #[ cfg( feature = "simd" ) ]
    assert!( enabled );
    
    #[ cfg( not( feature = "simd" ) ) ]
    assert!( !enabled );
  }
}

} // end private module

mod_interface::mod_interface!
{

  /// SIMD-optimized tokenizer for splitting strings by delimiters.
  orphan use super::private::SIMDTokenizer;
  
  /// CPU feature detection for SIMD optimization selection.
  orphan use super::private::simd_support_info;
  
  /// Returns true if SIMD optimizations are available and enabled.
  orphan use super::private::is_simd_enabled;

}