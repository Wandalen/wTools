//! Test suite functionality

#[cfg(feature = "integration")]
use benchkit::prelude::*;
use std::thread;
use core::time::Duration;

#[test]
fn test_benchmark_suite()
{
  let mut suite = BenchmarkSuite::new("test_suite")
    .add_benchmark("fast_op", || {})
    .add_benchmark("slow_op", || thread::sleep(Duration::from_millis(1)));

  let results = suite.run_all();
  assert_eq!(results.results.len(), 2);
  assert!(results.results.contains_key("fast_op"));
  assert!(results.results.contains_key("slow_op"));
}

#[test] 
fn test_markdown_report()
{
  let mut suite = BenchmarkSuite::new("test_report");
  suite.benchmark("test_op", || {});
  
  let results = suite.run_all();
  let report = results.generate_markdown_report();
  
  let markdown = report.generate();
  assert!(markdown.contains("## test_report Results"));
  assert!(markdown.contains("| Benchmark |"));
}