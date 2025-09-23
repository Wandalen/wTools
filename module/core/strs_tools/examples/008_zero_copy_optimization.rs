//! Zero-copy optimization examples demonstrating memory-efficient string operations.
//!
//! This example shows how zero-copy string operations can significantly reduce
//! memory allocations and improve performance for read-only string processing.

#[ allow( unused_imports ) ]
use strs_tools :: *;
#[ allow( unused_imports ) ]
use std ::time ::Instant;

fn main()
{
  println!( "=== Zero-Copy Optimization Examples ===" );
  
  basic_zero_copy_usage();
  performance_comparison();
  memory_efficiency_demonstration();
  copy_on_write_behavior();
}

/// Demonstrates basic zero-copy string splitting
fn basic_zero_copy_usage()
{
  println!( "\n--- Basic Zero-Copy Usage ---" );
  
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  {
  use strs_tools ::string ::zero_copy ::ZeroCopyStringExt;
  
  let input = "field1,field2,field3,field4";
  
  // Zero-copy splitting - no string allocations for segments
  let segments: Vec< _ > = input.zero_copy_split( &[ ","] ).collect();
  
  println!( "Input: '{}'", input );
  println!( "Zero-copy segments: " );
  for ( i, segment ) in segments.iter().enumerate() 
  {
   println!( "  [{}] : '{}' (borrowed: {})", 
   i, segment.as_str(), segment.is_borrowed() );
 }
  
  // All segments should be borrowed (zero-copy)
  assert!( segments.iter().all( |s| s.is_borrowed() ) );
  
  // Count segments without any allocation
  let count = input.count_segments( &[ ","] );
  println!( "Segment count (no allocation) : {}", count );
 }
}

/// Compare performance between traditional and zero-copy approaches
fn performance_comparison()
{
  println!( "\n--- Performance Comparison ---" );
  
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  {
  use strs_tools ::string ::zero_copy ::ZeroCopyStringExt;
  
  // Large test data to show performance differences
  let large_input = "word1,word2,word3,word4,word5,word6,word7,word8,word9,word10"
   .repeat( 1000 ); // ~50KB of data
  
  println!( "Processing {} bytes of data...", large_input.len() );
  
  // Traditional approach (allocates owned strings)
  let start = Instant ::now();
  let mut total_len = 0;
  for _ in 0..100 
  {
   let traditional_result: Vec< String > = string ::split()
  .src( &large_input )
  .delimeter( "," )
  .perform()
  .map( |split| split.string.into_owned() )
  .collect();
   total_len += traditional_result.iter().map( |s| s.len() ).sum :: < usize >();
 }
  let traditional_time = start.elapsed();
  
  // Zero-copy approach (no allocations for segments)
  let start = Instant ::now();
  let mut zero_copy_len = 0;
  for _ in 0..100 
  {
   zero_copy_len += large_input
  .zero_copy_split( &[ ","] )
  .map( |segment| segment.len() )
  .sum :: < usize >();
 }
  let zero_copy_time = start.elapsed();
  
  println!( "Traditional approach: {:?}", traditional_time );
  println!( "Zero-copy approach: {:?}", zero_copy_time );
  println!( "Speedup: {:.2}x", 
  traditional_time.as_secs_f64() / zero_copy_time.as_secs_f64() );
  
  // Verify same results
  assert_eq!( total_len, zero_copy_len );
  println!( "✓ Results verified identical" );
 }
}

/// Demonstrate memory efficiency of zero-copy operations
fn memory_efficiency_demonstration()
{
  println!( "\n--- Memory Efficiency Demonstration ---" );
  
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  {
  use strs_tools ::string ::zero_copy ::ZeroCopyStringExt;
  
  let csv_line = "Name,Age,City,Country,Email,Phone,Address,Occupation";
  
  // Traditional approach: each field becomes an owned String
  let traditional_fields: Vec< String > = string ::split()
   .src( csv_line )
   .delimeter( "," )
   .perform()
   .map( |split| split.string.into_owned() )
   .collect();
  
  // Zero-copy approach: fields are string slices into original
  let zero_copy_fields: Vec< _ > = csv_line
   .zero_copy_split( &[ ","] )
   .collect();
  
  println!( "Original CSV line: '{}'", csv_line );
  println!( "Traditional fields (owned strings) : " );
  for ( i, field ) in traditional_fields.iter().enumerate() 
  {
   println!( "  [{}] : '{}' (allocated {} bytes)", i, field, field.len() );
 }
  
  println!( "Zero-copy fields (borrowed slices) : " );
  for ( i, field ) in zero_copy_fields.iter().enumerate() 
  {
   println!( "  [{}] : '{}' (borrowed, 0 extra bytes)", i, field.as_str() );
 }
  
  // Calculate memory usage
  let traditional_memory: usize = traditional_fields
   .iter()
   .map( |s| s.capacity() )
   .sum();
  let zero_copy_memory = 0; // No extra allocations
  
  println!( "Memory usage comparison: " );
  println!( "  Traditional: {} bytes allocated", traditional_memory );
  println!( "  Zero-copy: {} bytes allocated", zero_copy_memory );
  println!( "  Savings: {} bytes ({:.1}%)", 
  traditional_memory - zero_copy_memory,
  100.0 * ( traditional_memory as f64 ) / ( traditional_memory as f64 ) );
 }
}

/// Demonstrate copy-on-write behavior when modification is needed
fn copy_on_write_behavior()
{
  println!( "\n--- Copy-on-Write Behavior ---" );
  
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  {
  use strs_tools ::string ::zero_copy ::ZeroCopyStringExt;
  
  let input = "hello,world,rust";
  let mut segments: Vec< _ > = input.zero_copy_split( &[ ","] ).collect();
  
  println!( "Initial segments (all borrowed) : " );
  for ( i, segment ) in segments.iter().enumerate() 
  {
   println!( "  [{}] : '{}' (borrowed: {})", 
   i, segment.as_str(), segment.is_borrowed() );
 }
  
  // Modify the second segment - this triggers copy-on-write
  println!( "\nModifying second segment (triggers copy-on-write)..." );
  segments[1].make_mut().push_str( "_modified" );
  
  println!( "After modification: " );
  for ( i, segment ) in segments.iter().enumerate() 
  {
   println!( "  [{}] : '{}' (borrowed: {})", 
   i, segment.as_str(), segment.is_borrowed() );
 }
  
  // Only the modified segment should be owned
  assert!( segments[0].is_borrowed() ); // Still borrowed
  assert!( segments[1].is_owned() );    // Now owned due to modification
  assert!( segments[2].is_borrowed() ); // Still borrowed
  
  println!( "✓ Copy-on-write working correctly" );
 }
}