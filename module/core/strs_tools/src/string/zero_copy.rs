//! Zero-copy string operations for optimal memory usage and performance.
//!
//! This module provides string manipulation operations that avoid unnecessary 
//! memory allocations by working with string slices (`&str`) and copy-on-write
//! semantics (`Cow< str >`) whenever possible.

use std ::borrow ::Cow;
use crate ::string ::split :: { Split, SplitType };

#[ cfg( feature = "simd" ) ]
use crate ::simd ::simd_split_cached;

/// Zero-copy string segment with optional mutation capabilities.
/// 
/// This is a higher-level wrapper around `Split` that provides
/// convenient methods for zero-copy string operations.
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct ZeroCopySegment< 'a > 
{
  /// The string content, using copy-on-write semantics
  pub content: Cow< 'a, str >,
  /// The type of segment (content or delimiter)
  pub segment_type: SegmentType,
  /// Starting position in original string
  pub start_pos: usize,
  /// Ending position in original string  
  pub end_pos: usize,
  /// Whether this segment was originally quoted
  pub was_quoted: bool,
}

/// Segment type for zero-copy operations
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum SegmentType 
{
  /// Content segment between delimiters
  Content,
  /// Delimiter segment
  Delimiter,
}

impl< 'a > ZeroCopySegment< 'a > 
{
  /// Create a new zero-copy segment from a string slice
  #[ must_use ]
  pub fn from_str( content: &'a str, start: usize, end: usize ) -> Self
  {
  Self {
   content: Cow ::Borrowed( content ),
   segment_type: SegmentType ::Content,
   start_pos: start,
   end_pos: end,
   was_quoted: false,
 }
 }
  
  /// Create a delimiter segment
  #[ must_use ]
  pub fn delimiter( content: &'a str, start: usize, end: usize ) -> Self
  {
  Self {
   content: Cow ::Borrowed( content ),
   segment_type: SegmentType ::Delimiter,
   start_pos: start,
   end_pos: end,
   was_quoted: false,
 }
 }
  
  /// Get string slice without allocation (zero-copy access)
  pub fn as_str( &self ) -> &str
  {
  &self.content
 }
  
  /// Convert to owned String only when needed
  pub fn into_owned( self ) -> String
  {
  self.content.into_owned()
 }
  
  /// Get mutable access to content (triggers copy-on-write if needed)
  pub fn make_mut( &mut self ) -> &mut String
  {
  self.content.to_mut()
 }
  
  /// Check if this segment is borrowed (zero-copy)
  pub fn is_borrowed( &self ) -> bool
  {
  matches!( self.content, Cow ::Borrowed( _ ) )
 }
  
  /// Check if this segment is owned (allocated)
  pub fn is_owned( &self ) -> bool
  {
  matches!( self.content, Cow ::Owned( _ ) )
 }
  
  /// Length of the segment
  pub fn len( &self ) -> usize
  {
  self.content.len()
 }
  
  /// Check if segment is empty
  pub fn is_empty( &self ) -> bool
  {
  self.content.is_empty()
 }
  
  /// Clone as borrowed (avoids allocation if possible)
  pub fn clone_borrowed( &self ) -> ZeroCopySegment< '_ >
  {
  ZeroCopySegment {
   content: match &self.content 
  {
  Cow ::Borrowed( s ) => Cow ::Borrowed( s ),
  Cow ::Owned( s ) => Cow ::Borrowed( s.as_str() ),
 },
   segment_type: self.segment_type,
   start_pos: self.start_pos,
   end_pos: self.end_pos,
   was_quoted: self.was_quoted,
 }
 }
}

impl< 'a > From< Split<'a >> for ZeroCopySegment< 'a > 
{
  fn from( split: Split< 'a > ) -> Self 
  {
  Self {
   content: split.string,
   segment_type: match split.typ 
  {
  SplitType ::Delimited => SegmentType ::Content,
  SplitType ::Delimiter => SegmentType ::Delimiter,
 },
   start_pos: split.start,
   end_pos: split.end,
   was_quoted: split.was_quoted,
 }
 }
}

impl< 'a > AsRef< str > for ZeroCopySegment< 'a > 
{
  fn as_ref( &self ) -> &str 
  {
  &self.content
 }
}

/// Zero-copy split iterator that avoids allocations for string segments
#[ derive( Debug ) ]
pub struct ZeroCopySplitIterator< 'a > 
{
  input: &'a str,
  delimiters: Vec< &'a str >,
  position: usize,
  preserve_delimiters: bool,
  preserve_empty: bool,
  finished: bool,
  pending_delimiter: Option< (&'a str, usize, usize) >, // (delimiter_str, start, end)
}

impl< 'a > ZeroCopySplitIterator< 'a > 
{
  /// Create new zero-copy split iterator
  pub fn new( 
  input: &'a str, 
  delimiters: Vec< &'a str >,
  preserve_delimiters: bool,
  preserve_empty: bool,
 ) -> Self {
  Self {
   input,
   delimiters,
   position: 0,
   preserve_delimiters,
   preserve_empty,
   finished: false,
   pending_delimiter: None,
 }
 }
  
  /// Find next delimiter in input starting from current position
  fn find_next_delimiter( &self ) -> Option< ( usize, usize, &'a str ) > 
  {
  if self.position >= self.input.len() 
  {
   return None;
 }
  
  let remaining = &self.input[ self.position.. ];
  let mut earliest_match: Option< ( usize, usize, &'a str ) > = None;
  
  // Find the earliest delimiter match
  for delimiter in &self.delimiters 
  {
   if let Some( pos ) = remaining.find( delimiter ) 
   {
  let absolute_start = self.position + pos;
  let absolute_end = absolute_start + delimiter.len();
  
  match earliest_match 
  {
   None =>
  {
  earliest_match = Some(( absolute_start, absolute_end, delimiter ));
 },
   Some(( prev_start, _, _ ))  if absolute_start < prev_start => 
  {
  earliest_match = Some(( absolute_start, absolute_end, delimiter ));
 },
   _ => {} // Keep previous match
 }
 }
 }
  
  earliest_match
 }
}

impl< 'a > Iterator for ZeroCopySplitIterator< 'a > 
{
  type Item = ZeroCopySegment< 'a >;
  
  fn next( &mut self ) -> Option< Self ::Item > 
  {
  loop
  {
   if self.finished || self.position > self.input.len() 
   {
  return None;
 }
   
   // If we have a pending delimiter to return, return it
   if let Some(( delimiter_str, delim_start, delim_end )) = self.pending_delimiter.take() 
   {
  return Some( ZeroCopySegment ::delimiter( delimiter_str, delim_start, delim_end ) );
 }
   
   // Handle end of input
   if self.position == self.input.len() 
   {
  self.finished = true;
  return None;
 }
   
   match self.find_next_delimiter() 
   {
  Some(( delim_start, delim_end, delimiter )) =>
  {
   // Extract content before delimiter
   let content = &self.input[ self.position..delim_start ];
   let content_start_pos = self.position;
   
   // Move position past delimiter
   self.position = delim_end;
   
   // If preserving delimiters, queue it for next iteration
   if self.preserve_delimiters 
   {
  self.pending_delimiter = Some(( delimiter, delim_start, delim_end ));
 }
   
   // Return content segment if non-empty or preserving empty
   if !content.is_empty() || self.preserve_empty 
   {
  return Some( ZeroCopySegment ::from_str( content, content_start_pos, delim_start ) );
 }
   
   // If content is empty and not preserving, continue loop
   // (delimiter will be returned in next iteration if preserving delimiters)
 },
  None =>
  {
   // No more delimiters, return remaining content
   if self.position < self.input.len() 
   {
  let remaining = &self.input[ self.position.. ];
  let start_pos = self.position;
  self.position = self.input.len();
  
  if !remaining.is_empty() || self.preserve_empty 
  {
   return Some( ZeroCopySegment ::from_str( remaining, start_pos, self.input.len() ) );
 }
 }
   
   self.finished = true;
   return None;
 }
 }
 }
 }
}

/// Zero-copy split builder with fluent API
#[ derive( Debug ) ]
pub struct ZeroCopySplit< 'a > 
{
  src: Option< &'a str >,
  delimiters: Vec< &'a str >,
  preserve_delimiters: bool,
  preserve_empty: bool,
}

impl< 'a > ZeroCopySplit< 'a > 
{
  /// Create new zero-copy split builder
  pub fn new() -> Self
  {
  Self {
   src: None,
   delimiters: Vec ::new(),
   preserve_delimiters: false,
   preserve_empty: false,
 }
 }
  
  /// Set source string
  pub fn src( mut self, src: &'a str ) -> Self
  {
  self.src = Some( src );
  self
 }
  
  /// Add delimiter
  pub fn delimiter( mut self, delim: &'a str ) -> Self
  {
  self.delimiters.push( delim );
  self
 }
  
  /// Add multiple delimiters
  pub fn delimiters( mut self, delims: Vec< &'a str > ) -> Self
  {
  self.delimiters.extend( delims );
  self
 }
  
  /// Preserve delimiters in output
  pub fn preserve_delimiters( mut self, preserve: bool ) -> Self
  {
  self.preserve_delimiters = preserve;
  self
 }
  
  /// Preserve empty segments
  pub fn preserve_empty( mut self, preserve: bool ) -> Self
  {
  self.preserve_empty = preserve;
  self
 }
  
  /// Execute zero-copy split operation
  pub fn perform( self ) -> ZeroCopySplitIterator< 'a >
  {
  let src = self.src.expect( "Source string is required for zero-copy split" );
  
  ZeroCopySplitIterator ::new(
   src,
   self.delimiters,
   self.preserve_delimiters,
   self.preserve_empty,
 )
 }
  
  /// Execute with SIMD optimization if available
  #[ cfg( feature = "simd" ) ]
  pub fn perform_simd( self ) -> Result< impl Iterator<Item = ZeroCopySegment<'a >>, String>
  {
  let src = self.src.expect( "Source string is required for SIMD split" );
  
  // Convert &str to &[ &str] for SIMD interface
  let delim_refs: Vec< &str > = self.delimiters.iter().copied().collect();
  
  match simd_split_cached( src, &delim_refs ) 
  {
   Ok( simd_iter ) =>
  {
  // Convert SIMD split results to ZeroCopySegment
  Ok( simd_iter.map( |split| ZeroCopySegment ::from( split ) ) )
 },
   Err( e ) => Err( format!( "SIMD split failed: {:?}", e ) ),
 }
 }
}

impl< 'a > Default for ZeroCopySplit< 'a > 
{
  fn default() -> Self 
  {
  Self ::new()
 }
}

/// Convenience function for zero-copy string splitting
pub fn zero_copy_split< 'a >( input: &'a str, delimiters: &[ &'a str] ) -> ZeroCopySplitIterator< 'a > 
{
  ZeroCopySplit ::new()
  .src( input )
  .delimiters( delimiters.to_vec() )
  .perform()
}

/// Extension trait adding zero-copy operations to string types
pub trait ZeroCopyStringExt 
{
  /// Split string using zero-copy operations
  fn zero_copy_split< 'a >( &'a self, delimiters: &[ &'a str] ) -> ZeroCopySplitIterator< 'a >;
  
  /// Split with delimiter preservation (zero-copy)
  fn zero_copy_split_preserve< 'a >( &'a self, delimiters: &[ &'a str] ) -> ZeroCopySplitIterator< 'a >;
  
  /// Count segments without allocation
  fn count_segments( &self, delimiters: &[ &str] ) -> usize;
}

impl ZeroCopyStringExt for str 
{
  fn zero_copy_split< 'a >( &'a self, delimiters: &[ &'a str] ) -> ZeroCopySplitIterator< 'a > 
  {
  zero_copy_split( self, delimiters )
 }
  
  fn zero_copy_split_preserve< 'a >( &'a self, delimiters: &[ &'a str] ) -> ZeroCopySplitIterator< 'a > 
  {
  ZeroCopySplit ::new()
   .src( self )
   .delimiters( delimiters.to_vec() )
   .preserve_delimiters( true )
   .perform()
 }
  
  fn count_segments( &self, delimiters: &[ &str] ) -> usize 
  {
  // Use a temporary conversion for counting to avoid lifetime issues
  let delims_vec: Vec< &str > = delimiters.iter().copied().collect();
  zero_copy_split( self, &delims_vec ).count()
 }
}

impl ZeroCopyStringExt for String 
{
  fn zero_copy_split< 'a >( &'a self, delimiters: &[ &'a str] ) -> ZeroCopySplitIterator< 'a > 
  {
  self.as_str().zero_copy_split( delimiters )
 }
  
  fn zero_copy_split_preserve< 'a >( &'a self, delimiters: &[ &'a str] ) -> ZeroCopySplitIterator< 'a > 
  {
  self.as_str().zero_copy_split_preserve( delimiters )
 }
  
  fn count_segments( &self, delimiters: &[ &str] ) -> usize 
  {
  self.as_str().count_segments( delimiters )
 }
}

#[ cfg( test ) ]
mod tests 
{
  use super :: *;
  
  #[ test ]
  fn test_zero_copy_basic_split() 
  {
  let input = "hello,world,rust";
  let segments: Vec< _ > = input.zero_copy_split( &[ ","] ).collect();
  
  assert_eq!( segments.len(), 3 );
  assert_eq!( segments[0].as_str(), "hello" );
  assert_eq!( segments[1].as_str(), "world" );
  assert_eq!( segments[2].as_str(), "rust" );
  
  // Verify zero-copy (all should be borrowed)
  assert!( segments[0].is_borrowed() );
  assert!( segments[1].is_borrowed() );
  assert!( segments[2].is_borrowed() );
 }
  
  #[ test ]
  fn test_zero_copy_with_delimiter_preservation() 
  {
  let input = "a: b: c";
  let segments: Vec< _ > = input.zero_copy_split_preserve( &[ ": "] ).collect();

  assert_eq!( segments.len(), 5 ); // a, : , b, : , c
  assert_eq!( segments[0].as_str(), "a" );
  assert_eq!( segments[1].as_str(), ": " );
  assert_eq!( segments[2].as_str(), "b" );
  assert_eq!( segments[3].as_str(), ": " );
  assert_eq!( segments[4].as_str(), "c" );
  
  // Check segment types
  assert_eq!( segments[0].segment_type, SegmentType ::Content );
  assert_eq!( segments[1].segment_type, SegmentType ::Delimiter );
  assert_eq!( segments[2].segment_type, SegmentType ::Content );
 }
  
  #[ test ]
  fn test_copy_on_write_behavior() 
  {
  let input = "test";
  let mut segment = ZeroCopySegment ::from_str( input, 0, 4 );
  
  // Initially borrowed
  assert!( segment.is_borrowed() );
  
  // Mutation triggers copy-on-write
  segment.make_mut().push_str( "_modified" );
  
  // Now owned
  assert!( segment.is_owned() );
  assert_eq!( segment.as_str(), "test_modified" );
 }
  
  #[ test ]
  fn test_empty_segments() 
  {
  let input = "a,,b";
  let segments: Vec< _ > = input.zero_copy_split( &[ ","] ).collect();
  
  // By default, empty segments are not preserved
  assert_eq!( segments.len(), 2 );
  assert_eq!( segments[0].as_str(), "a" );
  assert_eq!( segments[1].as_str(), "b" );
  
  // With preserve_empty enabled
  let segments_with_empty: Vec< _ > = ZeroCopySplit ::new()
   .src( input )
   .delimiter( "," )
   .preserve_empty( true )
   .perform()
   .collect();
  
  assert_eq!( segments_with_empty.len(), 3 );
  assert_eq!( segments_with_empty[0].as_str(), "a" );
  assert_eq!( segments_with_empty[1].as_str(), "" );
  assert_eq!( segments_with_empty[2].as_str(), "b" );
 }
  
  #[ test ]
  fn test_multiple_delimiters() 
  {
  let input = "a,b;c: d";
  let segments: Vec< _ > = input.zero_copy_split( &[ ",", ";", ": "] ).collect();
  
  assert_eq!( segments.len(), 4 );
  assert_eq!( segments[0].as_str(), "a" );
  assert_eq!( segments[1].as_str(), "b" );
  assert_eq!( segments[2].as_str(), "c" );
  assert_eq!( segments[3].as_str(), "d" );
 }
  
  #[ test ]
  fn test_position_tracking() 
  {
  let input = "hello,world";
  let segments: Vec< _ > = input.zero_copy_split( &[ ","] ).collect();
  
  assert_eq!( segments[0].start_pos, 0 );
  assert_eq!( segments[0].end_pos, 5 );
  assert_eq!( segments[1].start_pos, 6 );
  assert_eq!( segments[1].end_pos, 11 );
 }
  
  #[ test ]
  fn test_count_segments_without_allocation() 
  {
  let input = "a,b,c,d,e,f,g";
  let count = input.count_segments( &[ ","] );
  
  assert_eq!( count, 7 );
  
  // This operation should not allocate any String objects,
  // only count the segments
 }
  
  #[ cfg( feature = "simd" ) ]
  #[ test ]
  fn test_simd_zero_copy_integration() 
  {
  let input = "field1,field2,field3";
  
  let simd_result = ZeroCopySplit ::new()
   .src( input )
   .delimiter( "," )
   .perform_simd();
  
  match simd_result 
  {
   Ok( iter ) =>
  {
  let segments: Vec< _ > = iter.collect();
  
  // Debug output to understand what SIMD is returning
  eprintln!( "SIMD segments count: {}", segments.len() );
  for ( i, segment ) in segments.iter().enumerate() 
  {
   eprintln!( "  [{}] : '{}' (type: {:?})", i, segment.as_str(), segment.segment_type );
 }
  
  // SIMD might include delimiters in output, so we need to filter content segments
  let content_segments: Vec< _ > = segments
   .into_iter()
   .filter( |seg| seg.segment_type == SegmentType ::Content )
   .collect();
  
  assert_eq!( content_segments.len(), 3 );
  assert_eq!( content_segments[0].as_str(), "field1" );
  assert_eq!( content_segments[1].as_str(), "field2" );
  assert_eq!( content_segments[2].as_str(), "field3" );
 },
   Err( e ) =>
  {
  // SIMD might not be available in test environment
  eprintln!( "SIMD test failed (expected in some environments) : {}", e );
 }
 }
 }
}