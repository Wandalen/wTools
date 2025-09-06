//! Comparative benchmark between owned and zero-copy token parsing.

use std::time::{ Instant };
use std::borrow::Cow;
use unilang_parser::{ item_adapter::{ classify_split, classify_split_zero_copy, Split, SplitType } };

fn create_test_split( content: &str ) -> Split< '_ >
{
  Split {
    string: Cow::Borrowed( content ),
    bounds: ( 0, content.len() ),
    start: 0,
    end: content.len(),
    typ: SplitType::Delimiter,
    was_quoted: false,
  }
}

fn benchmark_owned_classification( iterations: u32, test_content: &str ) -> core::time::Duration
{
  let test_split = create_test_split( test_content );
  
  let start = Instant::now();
  for _ in 0..iterations
  {
    let result = classify_split( &test_split );
    assert!( result.is_ok() );
  }
  start.elapsed()
}

fn benchmark_zero_copy_classification( iterations: u32, test_content: &str ) -> core::time::Duration
{
  let test_split = create_test_split( test_content );
  
  let start = Instant::now();
  for _ in 0..iterations
  {
    let result = classify_split_zero_copy( &test_split );
    assert!( result.is_ok() );
    // Convert to owned to match the API
    let _owned = result.unwrap().0.to_owned();
  }
  start.elapsed()
}

fn main()
{
  println!( "=== Token Classification Benchmark Comparison ===" );
  
  let test_cases = vec![
    ( "identifier", "hello_world" ),
    ( "number", "12345" ),
    ( "complex", "complex_identifier_with_underscores" ),
    ( "short", "a" ),
  ];
  
  let iterations = 100_000;
  
  println!( "Iterations per test: {iterations}" );
  println!();
  
  for ( name, test_content ) in test_cases
  {
    println!( "Testing '{name}' (content: '{test_content}')" );
    
    // Warmup
    benchmark_owned_classification( 1000, test_content );
    benchmark_zero_copy_classification( 1000, test_content );
    
    // Benchmark owned approach
    let owned_time = benchmark_owned_classification( iterations, test_content );
    let owned_avg = owned_time / iterations;
    let owned_rate = 1_000_000_000.0 / owned_avg.as_nanos() as f64;
    
    // Benchmark zero-copy approach
    let zero_copy_time = benchmark_zero_copy_classification( iterations, test_content );
    let zero_copy_avg = zero_copy_time / iterations;
    let zero_copy_rate = 1_000_000_000.0 / zero_copy_avg.as_nanos() as f64;
    
    let improvement = owned_avg.as_nanos() as f64 / zero_copy_avg.as_nanos() as f64;
    
    println!( "  Owned approach:" );
    println!( "    Time: {owned_time:?}" );
    println!( "    Average: {owned_avg:?}" );
    println!( "    Rate: {owned_rate:.0} classifications/sec" );
    
    println!( "  Zero-copy approach:" );
    println!( "    Time: {zero_copy_time:?}" );
    println!( "    Average: {zero_copy_avg:?}" );
    println!( "    Rate: {zero_copy_rate:.0} classifications/sec" );
    
    println!( "  Improvement: {improvement:.1}x faster" );
    println!();
  }
  
  // Test that both approaches produce the same results
  println!( "=== Correctness Validation ===" );
  let test_split = create_test_split( "test_identifier" );
  
  let owned_result = classify_split( &test_split ).unwrap();
  let zero_copy_result = classify_split_zero_copy( &test_split ).unwrap();
  let zero_copy_owned = zero_copy_result.0.to_owned();
  
  println!( "Owned result: {:?}", owned_result.0 );
  println!( "Zero-copy result: {:?}", zero_copy_result.0 );
  println!( "Zero-copy to owned: {zero_copy_owned:?}" );
  println!( "Results match: {}", format!( "{:?}", owned_result.0 ) == format!( "{zero_copy_owned:?}" ) );
}