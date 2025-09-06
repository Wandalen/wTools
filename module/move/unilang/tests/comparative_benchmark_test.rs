//! Tests for comparative benchmark structure functionality

#![ cfg( feature = "benchmarks" ) ]
#![allow(clippy::cast_lossless)]
#![allow(clippy::float_cmp)]
#![allow(clippy::never_loop)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::uninlined_format_args)]

use unilang::{ ComparativeBenchmark, BenchmarkDataSize };

#[ cfg( not( feature = "benchmarks" ) ) ]
fn main() {}

#[ test ]
fn test_comparative_benchmark_creation()
{
  let comparison : ComparativeBenchmark< Vec< i32 > > = ComparativeBenchmark::new(
    "Test Sorting Algorithms",
    "Comparison of different sorting implementations"
  );
  
  // Basic structure validation
  assert_eq!( comparison.name(), "Test Sorting Algorithms" );
  assert_eq!( comparison.description(), "Comparison of different sorting implementations" );
  assert_eq!( comparison.algorithm_count(), 0 );
}

#[ test ]
fn test_algorithm_registration()
{
  let mut comparison : ComparativeBenchmark< Vec< i32 > > = ComparativeBenchmark::new(
    "Vector Operations",
    "Testing different vector processing approaches"
  );
  
  // Add test algorithms
  comparison.add_algorithm( "linear_search", | data : &Vec< i32 > | {
    for ( i, &value ) in data.iter().enumerate() {
      if value == 42 {
        let _ = i; // Found
        break;
      }
    }
  } );
  
  comparison.add_algorithm( "binary_search", | data : &Vec< i32 > | {
    let _ = data.binary_search( &42 );
  } );
  
  assert_eq!( comparison.algorithm_count(), 2 );
}

#[ test ]
fn test_comparison_run()
{
  let mut comparison : ComparativeBenchmark< String > = ComparativeBenchmark::new(
    "String Processing",
    "Testing string manipulation performance"
  );
  
  // Add simple string algorithms
  comparison.add_algorithm( "simple_count", | data : &String | {
    let _count = data.chars().count();
  } );
  
  comparison.add_algorithm( "byte_count", | data : &String | {
    let _count = data.len();
  } );
  
  // Set up test data
  let test_string = "Hello, world! ".repeat( 100 );
  comparison.set_test_data( BenchmarkDataSize::Small, test_string );
  
  // Run comparison
  let results = comparison.run_comparison( BenchmarkDataSize::Small, 10 );
  
  // Validate results
  assert_eq!( results.results.len(), 2 );
  assert!( results.baseline_time > 0.0 );
  assert!( !results.fastest_algorithm.is_empty() );
  
  // Check that results are sorted by performance (fastest first)
  if results.results.len() > 1 {
    assert!( results.results[ 0 ].average_time_nanos <= results.results[ 1 ].average_time_nanos );
  }
}

#[ test ]
fn test_comparison_table_generation()
{
  let mut comparison : ComparativeBenchmark< Vec< f64 > > = ComparativeBenchmark::new(
    "Math Operations",
    "Comparison of basic mathematical operations"
  );
  
  comparison.add_algorithm( "addition", | data : &Vec< f64 > | {
    let _sum : f64 = data.iter().sum();
  } );
  
  comparison.add_algorithm( "multiplication", | data : &Vec< f64 > | {
    let _product : f64 = data.iter().product();
  } );
  
  // Set up test data
  let test_data : Vec< f64 > = ( 1..=100 ).map( | i | i as f64 ).collect();
  comparison.set_test_data( BenchmarkDataSize::Medium, test_data );
  
  // Run comparison
  let results = comparison.run_comparison( BenchmarkDataSize::Medium, 20 );
  
  // Generate table
  let table = results.generate_comparison_table();
  
  // Validate table content
  assert!( table.contains( "Math Operations Comparison" ) );
  assert!( table.contains( "| Algorithm | Average Time |" ) );
  assert!( table.contains( "addition" ) );
  assert!( table.contains( "multiplication" ) );
  assert!( table.contains( "1.00x (baseline)" ) );
  assert!( table.contains( "🏆" ) );
}

#[ test ]
fn test_relative_performance_calculation()
{
  let results = vec![
    unilang::BenchmarkResult {
      algorithm_name: "fast_algo".to_string(),
      average_time_nanos: 1000.0,
      std_dev_nanos: 50.0,
      min_time_nanos: 900,
      max_time_nanos: 1100,
      sample_count: 10,
    },
    unilang::BenchmarkResult {
      algorithm_name: "slow_algo".to_string(),
      average_time_nanos: 2000.0,
      std_dev_nanos: 100.0,
      min_time_nanos: 1800,
      max_time_nanos: 2200,
      sample_count: 10,
    },
  ];
  
  // Create comparative results
  let comparative_results = unilang::ComparativeResults::new(
    "Performance Test".to_string(),
    "Testing relative performance calculation".to_string(),
    BenchmarkDataSize::Small,
    results
  );
  
  // Validate baseline and relative performance
  assert_eq!( comparative_results.baseline_time, 1000.0 );
  assert_eq!( comparative_results.fastest_algorithm, "fast_algo" );
  assert_eq!( comparative_results.performance_range(), 2.0 );
  
  // Check relative performance calculations
  let fast_result = &comparative_results.results[ 0 ];
  let slow_result = &comparative_results.results[ 1 ];
  
  assert_eq!( fast_result.relative_performance( comparative_results.baseline_time ), 1.0 );
  assert_eq!( slow_result.relative_performance( comparative_results.baseline_time ), 2.0 );
}

#[ test ]
fn test_multi_size_comparison()
{
  let mut comparison : ComparativeBenchmark< Vec< i32 > > = ComparativeBenchmark::new(
    "Size Scaling Test",
    "Testing how algorithms scale with data size"
  );
  
  comparison.add_algorithm( "algorithm_a", | data : &Vec< i32 > | {
    // O(n) algorithm
    for &value in data {
      let _ = value * 2;
    }
  } );
  
  comparison.add_algorithm( "algorithm_b", | data : &Vec< i32 > | {
    // O(n²) algorithm (intentionally slower)
    for &a in data {
      for &b in data {
        let _ = a + b;
        break; // Early break to keep test fast
      }
    }
  } );
  
  // Set up test data for different sizes
  for size in [ BenchmarkDataSize::Small, BenchmarkDataSize::Medium ] {
    let count = size.value();
    let test_data : Vec< i32 > = ( 1..=count as i32 ).collect();
    comparison.set_test_data( size, test_data );
  }
  
  let mut multi_comparison = unilang::MultiSizeComparison::new( comparison );
  multi_comparison.run_all_sizes( 5 );
  
  let report = multi_comparison.generate_comprehensive_report();
  
  // Validate comprehensive report
  assert!( report.contains( "Size Scaling Test - Comprehensive Size Analysis" ) );
  assert!( report.contains( "Performance Summary" ) );
  assert!( report.contains( "algorithm_a" ) );
  assert!( report.contains( "algorithm_b" ) );
}