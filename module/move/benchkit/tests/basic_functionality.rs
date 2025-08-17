//! Basic functionality tests for benchkit
//!
//! These tests verify that the core functionality works correctly.

#![ cfg( feature = "integration" ) ]

use benchkit::prelude::*;
use core::time::Duration;

#[test]
fn test_basic_timing()
{
  let result = bench_function( "basic_test", ||
  {
    let mut sum = 0;
    for i in 1..100
    {
      sum += i;
    }
    core::hint::black_box( sum );
  });

  assert!( !result.times.is_empty() );
  assert!( result.mean_time().as_nanos() > 0 );
  assert_eq!( result.name, "basic_test" );
}

#[test]
fn test_data_generation()
{
  let small_data = generate_list_data(DataSize::Small);
  let items: Vec<&str> = small_data.split(',').collect();
  assert_eq!(items.len(), 10);
  
  let medium_data = generate_list_data(DataSize::Medium);
  let medium_items: Vec<&str> = medium_data.split(',').collect();
  assert_eq!(medium_items.len(), 100);
}

#[test]  
fn test_benchmark_suite()
{
  let mut suite = BenchmarkSuite::new("test_suite");
  
  suite.benchmark("operation1", || {
    core::hint::black_box(42 + 42);
  });
  
  suite.benchmark("operation2", || {
    core::hint::black_box("test".len());
  });

  let results = suite.run_all();
  assert_eq!(results.results.len(), 2);
  assert!(results.results.contains_key("operation1"));
  assert!(results.results.contains_key("operation2"));
}

#[test]
fn test_comparative_analysis()
{
  let comparison = ComparativeAnalysis::new("test_comparison")
    .algorithm("fast", || {
      core::hint::black_box(1 + 1);
    })
    .algorithm("slow", || {
      // Simulate a slower operation
      for i in 0..50 {
        core::hint::black_box(i);
      }
    });

  let report = comparison.run();
  assert_eq!(report.results.len(), 2);
  
  let fastest = report.fastest();
  assert!(fastest.is_some());
}

#[test]
fn test_markdown_report_generation()
{
  let mut results = std::collections::HashMap::new();
  let test_result = BenchmarkResult::new("test_op", vec![Duration::from_millis(10)]);
  results.insert("test_op".to_string(), test_result);

  let generator = ReportGenerator::new("Test Report", results);
  let markdown = generator.generate_markdown_table();

  assert!(markdown.contains("| Operation |"));
  assert!(markdown.contains("test_op"));
}