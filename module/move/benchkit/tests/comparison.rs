//! Test comparison functionality

#[cfg(feature = "integration")]
use benchkit::prelude::*;
use std::collections::HashMap;
use core::time::Duration;

#[test]
fn test_framework_comparison()
{
  let config = ComparisonConfig
  {
    study_name: "Test Comparison".to_string(),
    scale_factors: vec![10, 100],
    ..Default::default()
  };
  
  let mut comparison = FrameworkComparison::new(config);
  
  // Add mock results
  let mut fast_framework_results = HashMap::new();
  fast_framework_results.insert(10, BenchmarkResult::new("fast_10", vec![Duration::from_micros(10)]));
  fast_framework_results.insert(100, BenchmarkResult::new("fast_100", vec![Duration::from_micros(100)]));
  
  let mut slow_framework_results = HashMap::new();
  slow_framework_results.insert(10, BenchmarkResult::new("slow_10", vec![Duration::from_millis(1)]));
  slow_framework_results.insert(100, BenchmarkResult::new("slow_100", vec![Duration::from_millis(10)]));
  
  comparison.add_framework_results("FastFramework", fast_framework_results);
  comparison.add_framework_results("SlowFramework", slow_framework_results);
  
  let report = comparison.generate_report();
  assert!(report.contains("FastFramework"));
  assert!(report.contains("SlowFramework"));
  assert!(report.contains("Executive Summary"));
}