//! Analysis functionality tests
//!
//! Tests for comparative analysis and regression analysis

#[ cfg( feature = "integration" ) ]
use benchkit::prelude::*;
use std::thread;
use core::time::Duration;
use std::collections::HashMap;

#[test]
fn test_comparative_analysis() {
  let comparison = ComparativeAnalysis::new("test_comparison")
    .algorithm("fast", || {})
    .algorithm("slow", || thread::sleep(Duration::from_millis(1)));
  
  let report = comparison.run();
  assert_eq!(report.results.len(), 2);
  
  let fastest = report.fastest();
  assert!(fastest.is_some());
  assert_eq!(fastest.unwrap().0, "fast");
}

#[test]
fn test_regression_analysis() {
  let fast_result = bench_once(|| {});
  let slow_result = bench_once(|| thread::sleep(Duration::from_millis(1)));
  
  let mut baseline = HashMap::new();
  baseline.insert("test".to_string(), fast_result);
  
  let mut current = HashMap::new();
  current.insert("test".to_string(), slow_result);
  
  let analysis = RegressionAnalysis::new(baseline, current);
  let regressions = analysis.detect_regressions(1.0);
  
  assert!(!regressions.is_empty());
  assert!(analysis.worst_regression_percentage() > 0.0);
}