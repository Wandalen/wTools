//! Tests for profiling module
//!
//! These tests verify memory allocation tracking and profiling functionality.

#![ cfg( feature = "integration" ) ]

use benchkit::prelude::*;

#[test]
fn test_allocation_tracking()
{
  let result = bench_with_allocation_tracking(
    "test_allocs",
    ||
    {
      let _vec : Vec< i32 > = ( 0..100 ).collect();
    },
    1, // One allocation per call
  );
  
  assert!( result.allocation_rate > 0.0 );
}

#[test]
fn test_string_operations_comparison()
{
  let test_data = vec![ vec![ "perf", "cmd_1" ], vec![ "perf", "cmd_2" ] ];
  let test_slices : Vec< &[ &str ] > = test_data.iter().map( | v | v.as_slice() ).collect();
  
  let comparison = bench_string_operations(
    "format_join",
    "cached_lookup",
    | slices | format!( ".{}", slices.join( "." ) ),
    | slices | format!( ".{}", slices.join( "." ) ), // Same for test
    &test_slices,
  );
  
  println!( "Comparison: {:?}", comparison );
}