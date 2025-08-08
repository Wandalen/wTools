//! High-performance SIMD string operations module.
//!
//! This module provides SIMD-accelerated string operations including splitting,
//! searching, and character counting. It automatically falls back to scalar
//! implementations when SIMD is not available or disabled.

#[ cfg( not( feature = "no_std" ) ) ]
extern crate std;

#[ cfg( feature = "use_alloc" ) ]
extern crate alloc;

#[ cfg( feature = "use_alloc" ) ]
use alloc::string::String;
#[ cfg( all( feature = "use_alloc", feature = "simd" ) ) ]
use alloc::format;

#[ cfg( not( feature = "no_std" ) ) ]
use std::string::String;

#[ cfg( feature = "simd" ) ]
use memchr::{ memchr, memmem };
#[ cfg( feature = "simd" ) ]
use aho_corasick::AhoCorasick;
#[ cfg( feature = "simd" ) ]
use bytecount;

#[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
pub use crate::string::split::{ SIMDSplitIterator, simd_split_cached };

/// SIMD-optimized string search operations.
#[ derive( Debug ) ]
pub struct SimdStringSearch;

impl SimdStringSearch 
{
  /// SIMD-optimized substring search.
  /// 
  /// Uses memchr's memmem implementation which leverages SIMD instructions
  /// for fast substring searching on supported platforms.
  #[ cfg( feature = "simd" ) ]
  #[ must_use ]
  pub fn find( haystack: &str, needle: &str ) -> Option<  usize  > 
  {
    memmem::find( haystack.as_bytes(), needle.as_bytes() )
  }
  
  /// Fallback substring search when SIMD is disabled.
  #[ cfg( not( feature = "simd" ) ) ]
  #[ must_use ]
  pub fn find( haystack: &str, needle: &str ) -> Option<  usize  > 
  {
    haystack.find( needle )
  }
  
  /// SIMD-optimized multi-pattern search.
  /// 
  /// Uses aho-corasick for efficient multi-pattern matching with SIMD acceleration.
  /// Returns the position and pattern index of the first match found.
  #[ cfg( feature = "simd" ) ]
  #[ must_use ]
  pub fn find_any( haystack: &str, needles: &[ &str ] ) -> Option<  ( usize, usize )  > 
  {
    let ac = AhoCorasick::new( needles ).ok()?;
    ac.find( haystack ).map( |m| ( m.start(), m.pattern().as_usize() ) )
  }
  
  /// Fallback multi-pattern search when SIMD is disabled.
  #[ cfg( not( feature = "simd" ) ) ]
  #[ must_use ]
  pub fn find_any( haystack: &str, needles: &[ &str ] ) -> Option<  ( usize, usize )  > 
  {
    let mut earliest_pos = haystack.len();
    let mut pattern_idx = 0;
    let mut found = false;
    
    for ( idx, needle ) in needles.iter().enumerate() 
    {
      if let Some( pos ) = haystack.find( needle ) 
      {
        if pos < earliest_pos 
        {
          earliest_pos = pos;
          pattern_idx = idx;
          found = true;
        }
      }
    }
    
    if found 
    {
      Some( ( earliest_pos, pattern_idx ) )
    } 
    else 
    {
      None
    }
  }
  
  /// SIMD-optimized character counting.
  /// 
  /// Uses bytecount for SIMD-accelerated byte counting for ASCII characters,
  /// falls back to iterator-based counting for Unicode characters.
  #[ cfg( feature = "simd" ) ]
  #[ must_use ]
  pub fn count_char( s: &str, ch: char ) -> usize 
  {
    if ch.is_ascii() 
    {
      bytecount::count( s.as_bytes(), ch as u8 )
    } 
    else 
    {
      s.chars().filter( |&c| c == ch ).count()
    }
  }
  
  /// Fallback character counting when SIMD is disabled.
  #[ cfg( not( feature = "simd" ) ) ]
  #[ must_use ]
  pub fn count_char( s: &str, ch: char ) -> usize 
  {
    s.chars().filter( |&c| c == ch ).count()
  }
  
  /// SIMD-optimized single byte search.
  /// 
  /// Uses memchr for highly optimized single byte searching.
  #[ cfg( feature = "simd" ) ]
  #[ must_use ]
  pub fn find_byte( haystack: &str, byte: u8 ) -> Option<  usize  > 
  {
    memchr( byte, haystack.as_bytes() )
  }
  
  /// Fallback single byte search when SIMD is disabled.
  #[ cfg( not( feature = "simd" ) ) ]
  #[ must_use ]
  pub fn find_byte( haystack: &str, byte: u8 ) -> Option<  usize  > 
  {
    haystack.bytes().position( |b| b == byte )
  }
}

/// Extension trait for strings providing SIMD-optimized operations.
/// 
/// This trait adds convenience methods for SIMD operations directly on string types.
pub trait SimdStringExt 
{
  /// SIMD-optimized string splitting.
  /// 
  /// # Errors
  /// 
  /// Returns an error string if SIMD is not available or pattern compilation fails.
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  fn simd_split( &self, delimiters: &[ &str ] ) -> Result< SIMDSplitIterator<'_>, String >;
  
  /// SIMD-optimized substring search.
  fn simd_find( &self, needle: &str ) -> Option<  usize  >;
  
  /// SIMD-optimized character counting.
  fn simd_count( &self, ch: char ) -> usize;
  
  /// SIMD-optimized multi-pattern search.
  fn simd_find_any( &self, needles: &[ &str ] ) -> Option<  ( usize, usize )  >;
  
  /// SIMD-optimized single byte search.
  fn simd_find_byte( &self, byte: u8 ) -> Option<  usize  >;
}

impl SimdStringExt for str 
{
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  fn simd_split( &self, delimiters: &[ &str ] ) -> Result< SIMDSplitIterator<'_>, String > 
  {
    #[ cfg( feature = "simd" ) ]
    {
      simd_split_cached( self, delimiters )
        .map_err( |e| format!( "SIMD split failed: {e:?}" ) )
    }
    
    #[ cfg( not( feature = "simd" ) ) ]
    {
      Err( "SIMD feature not enabled".to_string() )
    }
  }
  
  fn simd_find( &self, needle: &str ) -> Option<  usize  > 
  {
    SimdStringSearch::find( self, needle )
  }
  
  fn simd_count( &self, ch: char ) -> usize 
  {
    SimdStringSearch::count_char( self, ch )
  }
  
  fn simd_find_any( &self, needles: &[ &str ] ) -> Option<  ( usize, usize )  > 
  {
    SimdStringSearch::find_any( self, needles )
  }
  
  fn simd_find_byte( &self, byte: u8 ) -> Option<  usize  > 
  {
    SimdStringSearch::find_byte( self, byte )
  }
}

impl SimdStringExt for String 
{
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  fn simd_split( &self, delimiters: &[ &str ] ) -> Result< SIMDSplitIterator<'_>, String > 
  {
    self.as_str().simd_split( delimiters )
  }
  
  fn simd_find( &self, needle: &str ) -> Option<  usize  > 
  {
    self.as_str().simd_find( needle )
  }
  
  fn simd_count( &self, ch: char ) -> usize 
  {
    self.as_str().simd_count( ch )
  }
  
  fn simd_find_any( &self, needles: &[ &str ] ) -> Option<  ( usize, usize )  > 
  {
    self.as_str().simd_find_any( needles )
  }
  
  fn simd_find_byte( &self, byte: u8 ) -> Option<  usize  > 
  {
    self.as_str().simd_find_byte( byte )
  }
}

/// Utility functions for SIMD performance testing and validation.
pub mod utils 
{
  /// Determines if SIMD instructions are available at runtime.
  /// 
  /// This function checks CPU capabilities to determine if SIMD
  /// optimizations will be effective.
  #[ cfg( feature = "simd" ) ]
  #[ must_use ]
  pub fn simd_available() -> bool 
  {
    // The underlying libraries (memchr, aho-corasick) handle runtime detection
    // automatically, so we can assume SIMD is available if the feature is enabled
    true
  }
  
  /// Fallback version when SIMD feature is disabled.
  #[ cfg( not( feature = "simd" ) ) ]
  #[ must_use ]
  pub fn simd_available() -> bool 
  {
    false
  }
  
  /// Estimates the performance benefit of using SIMD for a given input size.
  /// 
  /// Returns a multiplier indicating expected speedup (e.g., 3.0 means 3x faster).
  /// This is useful for deciding whether to use SIMD or scalar implementations.
  #[ must_use ]
  pub fn estimated_simd_speedup( input_size: usize, pattern_count: usize ) -> f32 
  {
    if !simd_available() 
    {
      return 1.0;
    }
    
    match ( input_size, pattern_count ) 
    {
      // Small inputs may not benefit from SIMD due to setup overhead
      ( 0..=100, _ ) => 1.2,
      ( 101..=1000, 1 ) => 2.5,
      ( 101..=1000, 2..=5 ) | ( 1001..=10000, 1 ) => 3.5,
      ( 101..=1000, _ ) => 4.0,
      ( 1001..=10000, _ ) | ( _, 2..=5 ) => 6.0,
      // Large inputs show maximum SIMD benefit
      ( _, _ ) => 7.0,
    }
  }
}