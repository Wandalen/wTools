//! ## Test Matrix for Analysis Functionality
//!
//! This test suite validates comparative analysis and regression detection.
//!
//! ### Test Factors
//! - Analysis Type: Comparative analysis, Regression detection, Statistical analysis
//! - Data Patterns: Similar performance, Different performance, Gradual changes
//! - Thresholds: Default thresholds, Custom thresholds, Edge cases
//!
//! ### Test Combinations
//! | ID   | Analysis Type | Data Pattern    | Threshold | Expected Behavior                    |
//! |------|---------------|-----------------|-----------|--------------------------------------|
//! | A1.1 | Comparative   | Different perf  | Default   | Clear winner identified              |
//! | A1.2 | Comparative   | Similar perf    | Default   | Close performance reported           |
//! | A1.3 | Regression    | Performance drop| 5%        | Regression detected and quantified   |
//! | A1.4 | Regression    | Performance gain| 5%        | Improvement detected and quantified  |
//! | A1.5 | Regression    | Stable perf     | 5%        | No significant changes detected      |
//! | A1.6 | Comparative   | Multiple algos  | Default   | Full ranking with relative speeds    |

use benchkit::prelude::*;
use std::time::Duration;
use std::collections::HashMap;

/// Tests comparative analysis with different performance characteristics
/// Test Combination: A1.1
#[test]
fn test_comparative_analysis_different_performance()
{
  let comparison = ComparativeAnalysis::new("performance_comparison")
    .algorithm("fast_algo", || {
      std::hint::black_box(42 + 42);
    })
    .algorithm("slow_algo", || {
      std::thread::sleep(Duration::from_millis(1));
      std::hint::black_box("slow");
    });
  
  let report = comparison.run();
  
  // Verify comparison executed both algorithms
  assert_eq!(report.name, "performance_comparison");
  assert_eq!(report.results.len(), 2);
  assert!(report.results.contains_key("fast_algo"));
  assert!(report.results.contains_key("slow_algo"));
  
  // Verify performance analysis
  let (fastest_name, fastest_result) = report.fastest().unwrap();
  let (slowest_name, slowest_result) = report.slowest().unwrap();
  
  assert_eq!(*fastest_name, "fast_algo", "Fast algorithm should be identified as fastest");
  assert_eq!(*slowest_name, "slow_algo", "Slow algorithm should be identified as slowest");
  assert!(fastest_result.mean_time() < slowest_result.mean_time(), "Fastest should actually be faster");
  
  // Test sorted results
  let sorted = report.sorted_by_performance();
  assert_eq!(sorted.len(), 2);
  assert_eq!(*sorted[0].0, "fast_algo", "First in sorted should be fastest");
  assert_eq!(*sorted[1].0, "slow_algo", "Last in sorted should be slowest");
}

/// Tests comparative analysis with similar performance
/// Test Combination: A1.2
#[test]
fn test_comparative_analysis_similar_performance()
{
  let comparison = ComparativeAnalysis::new("similar_performance")
    .algorithm("algo_a", || {
      for i in 0..100 {
        std::hint::black_box(i);
      }
    })
    .algorithm("algo_b", || {
      for i in 0..105 { // Slightly more work
        std::hint::black_box(i);
      }
    });
  
  let report = comparison.run();
  
  assert_eq!(report.results.len(), 2);
  
  let fastest = report.fastest().unwrap();
  let slowest = report.slowest().unwrap();
  
  // Should still identify fastest and slowest
  assert!(fastest.1.mean_time() <= slowest.1.mean_time());
  
  // Performance difference should be relatively small
  let time_ratio = slowest.1.mean_time().as_secs_f64() / fastest.1.mean_time().as_secs_f64();
  assert!(time_ratio < 10.0, "Similar performance should not have huge differences");
}

/// Tests regression detection with performance drop
/// Test Combination: A1.3
#[test]
fn test_regression_detection_performance_drop()
{
  // Create baseline (fast) and current (slow) results
  let mut baseline = HashMap::new();
  baseline.insert("test_function".to_string(),
    BenchmarkResult::new("test_function", vec![Duration::from_millis(10)]));
  
  let mut current = HashMap::new();
  current.insert("test_function".to_string(),
    BenchmarkResult::new("test_function", vec![Duration::from_millis(50)])); // 5x slower
  
  let analysis = RegressionAnalysis::new(baseline, current);
  
  // Test regression detection with 5% threshold
  let regressions = analysis.detect_regressions(5.0);
  assert!(!regressions.is_empty(), "Should detect significant regression");
  
  let regression = &regressions[0];
  assert_eq!(regression.current.name, "test_function");
  assert!(regression.improvement_percentage < -5.0, "Should show significant performance drop");
  assert!(regression.is_regression(), "Should be identified as regression");
  
  // Test worst regression percentage
  let worst = analysis.worst_regression_percentage();
  assert!(worst > 50.0, "Should report large regression percentage");
}

/// Tests improvement detection with performance gain
/// Test Combination: A1.4
#[test]
fn test_improvement_detection_performance_gain()
{
  // Create baseline (slow) and current (fast) results
  let mut baseline = HashMap::new();
  baseline.insert("optimized_function".to_string(),
    BenchmarkResult::new("optimized_function", vec![Duration::from_millis(100)]));
  
  let mut current = HashMap::new();
  current.insert("optimized_function".to_string(),
    BenchmarkResult::new("optimized_function", vec![Duration::from_millis(20)])); // 5x faster
  
  let analysis = RegressionAnalysis::new(baseline, current);
  
  // Test improvement detection
  let improvements = analysis.detect_improvements(5.0);
  assert!(!improvements.is_empty(), "Should detect significant improvement");
  
  let improvement = &improvements[0];
  assert_eq!(improvement.current.name, "optimized_function");
  assert!(improvement.improvement_percentage > 5.0, "Should show significant performance gain");
  assert!(improvement.is_improvement(), "Should be identified as improvement");
  
  // Test no regressions detected
  let regressions = analysis.detect_regressions(5.0);
  assert!(regressions.is_empty(), "Should not detect regressions when performance improved");
}

/// Tests stable performance detection
/// Test Combination: A1.5
#[test]
fn test_stable_performance_detection()
{
  // Create baseline and current with very similar results
  let mut baseline = HashMap::new();
  baseline.insert("stable_function".to_string(),
    BenchmarkResult::new("stable_function", vec![Duration::from_millis(50)]));
  
  let mut current = HashMap::new();
  current.insert("stable_function".to_string(),
    BenchmarkResult::new("stable_function", vec![Duration::from_millis(52)])); // 4% slower (under threshold)
  
  let analysis = RegressionAnalysis::new(baseline, current);
  
  // Test that small changes are not detected as significant
  let regressions = analysis.detect_regressions(5.0);
  let improvements = analysis.detect_improvements(5.0);
  
  assert!(regressions.is_empty(), "Small performance changes should not be flagged as regressions");
  assert!(improvements.is_empty(), "Small performance changes should not be flagged as improvements");
  
  let worst_regression = analysis.worst_regression_percentage();
  assert!(worst_regression < 5.0, "Worst regression should be under threshold");
}

/// Tests multi-algorithm comparative analysis with full ranking
/// Test Combination: A1.6
#[test]
fn test_multi_algorithm_comparative_analysis()
{
  let comparison = ComparativeAnalysis::new("algorithm_tournament")
    .algorithm("ultra_fast", || {
      std::hint::black_box(1);
    })
    .algorithm("fast", || {
      for i in 0..10 {
        std::hint::black_box(i);
      }
    })
    .algorithm("medium", || {
      for i in 0..100 {
        std::hint::black_box(i);
      }
    })
    .algorithm("slow", || {
      std::thread::sleep(Duration::from_millis(1));
    })
    .algorithm("ultra_slow", || {
      std::thread::sleep(Duration::from_millis(5));
    });
  
  let report = comparison.run();
  
  assert_eq!(report.results.len(), 5);
  
  // Test sorted performance ranking
  let sorted = report.sorted_by_performance();
  assert_eq!(sorted.len(), 5);
  
  // Verify ordering is correct (times should increase)
  for i in 1..sorted.len() {
    assert!(
      sorted[i-1].1.mean_time() <= sorted[i].1.mean_time(),
      "Results should be sorted by performance: {} ({:?}) should be <= {} ({:?})",
      sorted[i-1].0, sorted[i-1].1.mean_time(),
      sorted[i].0, sorted[i].1.mean_time()
    );
  }
  
  // Test that fastest and slowest are correctly identified
  assert_eq!(*sorted[0].0, "ultra_fast", "Ultra fast should be first");
  assert_eq!(*sorted[4].0, "ultra_slow", "Ultra slow should be last");
}

/// Tests comparative analysis markdown generation
#[test]
fn test_comparative_analysis_markdown_generation()
{
  let comparison = ComparativeAnalysis::new("markdown_test")
    .algorithm("algorithm_one", || {
      std::hint::black_box(vec![1, 2, 3]);
    })
    .algorithm("algorithm_two", || {
      std::thread::sleep(Duration::from_millis(1));
    });
  
  let report = comparison.run();
  let markdown = report.to_markdown();
  
  // Verify markdown structure
  assert!(markdown.contains("## markdown_test Comparison"), "Should have comparison title");
  assert!(markdown.contains("| Algorithm |"), "Should have table header");
  assert!(markdown.contains("algorithm_one"), "Should include first algorithm");
  assert!(markdown.contains("algorithm_two"), "Should include second algorithm");
  
  // Verify performance indicators
  assert!(markdown.contains("**Fastest**") || markdown.contains("slower"), "Should indicate relative performance");
  assert!(markdown.contains("### Key Insights"), "Should have insights section");
  assert!(markdown.contains("**Best performing**"), "Should identify best performer");
}

/// Tests regression analysis report generation
#[test]
fn test_regression_analysis_report_generation()
{
  // Setup: Create both improvements and regressions
  let mut baseline = HashMap::new();
  baseline.insert("improved_func".to_string(),
    BenchmarkResult::new("improved_func", vec![Duration::from_millis(100)]));
  baseline.insert("regressed_func".to_string(),
    BenchmarkResult::new("regressed_func", vec![Duration::from_millis(10)]));
  baseline.insert("stable_func".to_string(),
    BenchmarkResult::new("stable_func", vec![Duration::from_millis(50)]));
  
  let mut current = HashMap::new();
  current.insert("improved_func".to_string(),
    BenchmarkResult::new("improved_func", vec![Duration::from_millis(20)])); // 5x faster
  current.insert("regressed_func".to_string(),
    BenchmarkResult::new("regressed_func", vec![Duration::from_millis(50)])); // 5x slower
  current.insert("stable_func".to_string(),
    BenchmarkResult::new("stable_func", vec![Duration::from_millis(52)])); // Stable
  
  let analysis = RegressionAnalysis::new(baseline, current);
  let report = analysis.generate_report();
  
  // Verify report structure
  assert!(report.contains("# Performance Regression Analysis"), "Should have main title");
  
  // Should contain regression section
  assert!(report.contains("## 🚨 Performance Regressions"), "Should identify regressions");
  assert!(report.contains("regressed_func"), "Should mention regressed function");
  assert!(report.contains("slower"), "Should indicate performance degradation");
  
  // Should contain improvement section  
  assert!(report.contains("## 🎉 Performance Improvements"), "Should identify improvements");
  assert!(report.contains("improved_func"), "Should mention improved function");
  assert!(report.contains("faster"), "Should indicate performance improvement");
  
  // Should show quantified changes
  assert!(report.contains("%"), "Should show percentage changes");
}

/// Tests stable performance report generation
#[test]
fn test_stable_performance_report()
{
  let mut baseline = HashMap::new();
  baseline.insert("stable_func".to_string(),
    BenchmarkResult::new("stable_func", vec![Duration::from_millis(50)]));
  
  let mut current = HashMap::new();
  current.insert("stable_func".to_string(),
    BenchmarkResult::new("stable_func", vec![Duration::from_millis(51)])); // Minimal change
  
  let analysis = RegressionAnalysis::new(baseline, current);
  let report = analysis.generate_report();
  
  // Should indicate stability
  assert!(report.contains("## ✅ No Significant Changes"), "Should indicate stability");
  assert!(report.contains("Performance appears stable"), "Should mention stability");
}

/// Tests comparative analysis with empty results
#[test]
fn test_comparative_analysis_empty_handling()
{
  let empty_comparison = ComparativeAnalysis::new("empty_test");
  let report = empty_comparison.run();
  
  assert_eq!(report.results.len(), 0);
  assert!(report.fastest().is_none());
  assert!(report.slowest().is_none());
  
  let markdown = report.to_markdown();
  assert!(markdown.contains("No results available"), "Should handle empty results");
}

/// Tests regression analysis with missing baselines
#[test]
fn test_regression_analysis_missing_baselines()
{
  let mut baseline = HashMap::new();
  baseline.insert("old_function".to_string(),
    BenchmarkResult::new("old_function", vec![Duration::from_millis(10)]));
  
  let mut current = HashMap::new();
  current.insert("new_function".to_string(),
    BenchmarkResult::new("new_function", vec![Duration::from_millis(10)]));
  current.insert("old_function".to_string(),
    BenchmarkResult::new("old_function", vec![Duration::from_millis(15)]));
  
  let analysis = RegressionAnalysis::new(baseline, current);
  
  // Should only analyze functions that exist in both baseline and current
  let regressions = analysis.detect_regressions(1.0);
  assert_eq!(regressions.len(), 1); // Only old_function should be analyzed
  assert_eq!(regressions[0].current.name, "old_function");
}

/// Tests comparative analysis summary printing
#[test]
fn test_comparative_analysis_summary_printing()
{
  let comparison = ComparativeAnalysis::new("summary_test")
    .algorithm("first", || { std::hint::black_box(1); })
    .algorithm("second", || {
      for i in 0..100 {
        std::hint::black_box(i);
      }
    });
  
  let report = comparison.run();
  
  // This would print to stdout - we test data availability instead
  assert!(report.fastest().is_some(), "Should have fastest result for summary");
  
  // Verify data for summary is complete
  for (name, result) in &report.results {
    assert!(!name.is_empty(), "Names should be available for summary");
    assert!(result.mean_time().as_nanos() > 0, "Times should be available for summary");
  }
  
  // Test actual summary printing (output to stdout)
  report.print_summary();
}

/// Tests performance comparison edge cases
#[test]
fn test_performance_comparison_edge_cases()
{
  // Test with zero-time operations
  let very_fast_result = BenchmarkResult::new("instant", vec![Duration::from_nanos(1)]);
  let fast_result = BenchmarkResult::new("fast", vec![Duration::from_nanos(10)]);
  
  let comparison = very_fast_result.compare(&fast_result);
  
  // Should handle very small timings correctly
  assert!(comparison.improvement_percentage > 0.0, "Should detect improvement even with tiny timings");
  assert!(comparison.is_improvement(), "Should identify as improvement");
  
  // Test with identical timings
  let identical1 = BenchmarkResult::new("same1", vec![Duration::from_millis(10)]);
  let identical2 = BenchmarkResult::new("same2", vec![Duration::from_millis(10)]);
  
  let same_comparison = identical1.compare(&identical2);
  assert_eq!(same_comparison.improvement_percentage, 0.0, "Identical times should show 0% change");
  assert!(!same_comparison.is_improvement(), "Should not be improvement");
  assert!(!same_comparison.is_regression(), "Should not be regression");
}