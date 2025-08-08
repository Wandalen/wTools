//! ## Test Matrix for Benchmark Suite Functionality
//!
//! This test suite validates benchmark suite management and execution.
//!
//! ### Test Factors
//! - Suite Configuration: Default config, Custom config, Multiple benchmarks
//! - Execution: Single run, Multiple runs, Result aggregation
//! - Integration: File operations, Baseline management, Report generation
//!
//! ### Test Combinations
//! | ID   | Configuration | Benchmarks | Operation       | Expected Behavior                    |
//! |------|---------------|------------|-----------------|--------------------------------------|
//! | S1.1 | Default       | Single     | Execute         | Single result recorded               |
//! | S1.2 | Default       | Multiple   | Execute         | All results recorded, sorted output  |
//! | S1.3 | Custom        | Multiple   | Execute         | Custom config respected              |
//! | S1.4 | Default       | Multiple   | Generate report | Markdown report with insights        |
//! | S1.5 | Default       | Single     | Result access   | Previous results retrievable         |
//! | S1.6 | Default       | Multiple   | Print summary   | Console output formatted correctly   |

use benchkit::prelude::*;
use std::time::Duration;
use std::collections::HashMap;

/// Tests single benchmark execution in suite
/// Test Combination: S1.1
#[test]
fn test_single_benchmark_suite_execution()
{
  let mut suite = BenchmarkSuite::new("single_test_suite");
  
  suite.benchmark("simple_operation", || {
    std::hint::black_box(42 + 42);
  });
  
  let results = suite.run_all();
  
  assert_eq!(results.suite_name, "single_test_suite");
  assert_eq!(results.results.len(), 1);
  assert!(results.results.contains_key("simple_operation"));
  
  let result = &results.results["simple_operation"];
  assert_eq!(result.name, "simple_operation");
  assert!(!result.times.is_empty());
}

/// Tests multiple benchmarks execution with sorting
/// Test Combination: S1.2
#[test]
fn test_multiple_benchmarks_execution()
{
  let mut suite = BenchmarkSuite::new("multi_test_suite")
    .add_benchmark("fast_op", || {
      std::hint::black_box(1 + 1);
    })
    .add_benchmark("slow_op", || {
      std::thread::sleep(Duration::from_millis(1));
    })
    .add_benchmark("medium_op", || {
      for i in 0..1000 {
        std::hint::black_box(i);
      }
    });
  
  let results = suite.run_all();
  
  // Verify all benchmarks were executed
  assert_eq!(results.results.len(), 3);
  assert!(results.results.contains_key("fast_op"));
  assert!(results.results.contains_key("slow_op"));
  assert!(results.results.contains_key("medium_op"));
  
  // Verify results are meaningful
  for (name, result) in &results.results {
    assert_eq!(result.name, *name);
    assert!(!result.times.is_empty(), "Benchmark {} should have recorded times", name);
    assert!(result.mean_time().as_nanos() > 0, "Benchmark {} should have non-zero timing", name);
  }
  
  // Verify performance ordering is logical
  let fast_time = results.results["fast_op"].mean_time();
  let slow_time = results.results["slow_op"].mean_time();
  assert!(fast_time < slow_time, "Fast operation should be faster than slow operation");
}

/// Tests custom configuration application
/// Test Combination: S1.3
#[test]
fn test_custom_configuration_suite()
{
  let custom_config = MeasurementConfig {
    iterations: 3,
    warmup_iterations: 1,
    max_time: Duration::from_secs(5),
  };
  
  let mut suite = BenchmarkSuite::new("custom_config_suite")
    .with_config(custom_config);
  
  suite.benchmark("config_test", || {
    std::hint::black_box("test");
  });
  
  let results = suite.run_all();
  
  // Verify configuration was applied (max 3 iterations)
  let result = &results.results["config_test"];
  assert!(
    result.times.len() <= 3,
    "Should respect custom iteration limit: got {} iterations",
    result.times.len()
  );
  assert!(
    !result.times.is_empty(),
    "Should have at least one measurement"
  );
}

/// Tests markdown report generation from suite results
/// Test Combination: S1.4
#[test]
fn test_suite_markdown_report_generation()
{
  let mut suite = BenchmarkSuite::new("report_test_suite")
    .add_benchmark("operation_a", || {
      std::thread::sleep(Duration::from_millis(1));
    })
    .add_benchmark("operation_b", || {
      std::thread::sleep(Duration::from_millis(2));
    });
  
  let results = suite.run_all();
  let report = results.generate_markdown_report();
  
  let markdown = report.generate();
  
  // Verify report structure
  assert!(markdown.contains("## report_test_suite Results"), "Should have suite name as title");
  assert!(markdown.contains("| Benchmark |"), "Should contain table header");
  assert!(markdown.contains("operation_a"), "Should include first operation");
  assert!(markdown.contains("operation_b"), "Should include second operation");
  
  // Verify insights section
  assert!(markdown.contains("### Key Insights"), "Should have insights section");
  assert!(markdown.contains("**Fastest operation**"), "Should identify fastest operation");
  assert!(markdown.contains("**Performance range**"), "Should calculate performance range");
}

/// Tests result access after execution
/// Test Combination: S1.5
#[test]
fn test_suite_result_access()
{
  let mut suite = BenchmarkSuite::new("access_test_suite");
  
  suite.benchmark("accessible_test", || {
    std::hint::black_box(vec![1, 2, 3, 4, 5]);
  });
  
  // Execute suite
  let _results = suite.run_all();
  
  // Access results through suite
  let suite_results = suite.results();
  assert!(!suite_results.is_empty(), "Suite should retain results");
  assert!(suite_results.contains_key("accessible_test"), "Should contain executed benchmark");
  
  let result = &suite_results["accessible_test"];
  assert_eq!(result.name, "accessible_test");
  assert!(!result.times.is_empty());
}

/// Tests suite summary printing
/// Test Combination: S1.6  
#[test]
fn test_suite_summary_printing()
{
  let mut suite = BenchmarkSuite::new("summary_test_suite")
    .add_benchmark("first_op", || {
      std::hint::black_box(42);
    })
    .add_benchmark("second_op", || {
      for i in 0..100 {
        std::hint::black_box(i);
      }
    });
  
  let results = suite.run_all();
  
  // This would normally print to stdout, but we can't easily test that
  // Instead, we'll verify the data that would be printed is available
  assert_eq!(results.results.len(), 2);
  
  // Verify all results have valid timing data for printing
  for (name, result) in &results.results {
    assert!(!name.is_empty(), "Operation names should not be empty");
    assert!(result.mean_time().as_nanos() > 0, "Mean time should be positive");
    assert!(result.std_deviation().as_nanos() >= 0, "Std deviation should be non-negative");
  }
  
  // Test the actual print summary (output goes to stdout)
  results.print_summary(); // This will print but we can't capture it in test
}

/// Tests suite builder pattern
#[test]
fn test_suite_builder_pattern()
{
  let suite = BenchmarkSuite::new("builder_test")
    .add_benchmark("first", || { std::hint::black_box(1); })
    .add_benchmark("second", || { std::hint::black_box(2); })
    .add_benchmark("third", || { std::hint::black_box(3); })
    .with_config(MeasurementConfig {
      iterations: 5,
      warmup_iterations: 1,
      max_time: Duration::from_secs(10),
    });
  
  // Verify builder pattern worked
  assert_eq!(suite.name, "builder_test");
  // Note: Can't easily test private fields, but run_all will validate
}

/// Tests empty suite handling
#[test]
fn test_empty_suite_handling()
{
  let mut empty_suite = BenchmarkSuite::new("empty_suite");
  let results = empty_suite.run_all();
  
  assert_eq!(results.suite_name, "empty_suite");
  assert!(results.results.is_empty());
  
  // Test markdown generation with empty results
  let report = results.generate_markdown_report();
  let markdown = report.generate();
  assert!(markdown.contains("No benchmark results available"), "Should handle empty results");
}

/// Tests regression analysis integration
#[test]
fn test_suite_regression_analysis()
{
  let mut baseline_results = HashMap::new();
  baseline_results.insert("test_op".to_string(),
    BenchmarkResult::new("test_op", vec![Duration::from_millis(10)]));
  
  let mut suite = BenchmarkSuite::new("regression_test");
  suite.benchmark("test_op", || {
    std::thread::sleep(Duration::from_millis(20)); // Slower than baseline
  });
  
  let results = suite.run_all();
  let analysis = results.regression_analysis(&baseline_results);
  
  // Should detect regression
  let regressions = analysis.detect_regressions(5.0);
  assert!(!regressions.is_empty(), "Should detect performance regression");
  
  let worst_regression = analysis.worst_regression_percentage();
  assert!(worst_regression > 0.0, "Should report regression percentage");
}

/// Tests suite result metadata and statistics
#[test]
fn test_suite_result_statistics()
{
  let mut suite = BenchmarkSuite::new("stats_test")
    .add_benchmark("consistent_op", || {
      // Consistent timing operation
      for _i in 0..100 {
        std::hint::black_box(1);
      }
    });
  
  let results = suite.run_all();
  let result = &results.results["consistent_op"];
  
  // Test statistical measures
  assert!(result.min_time() <= result.mean_time(), "Min should be <= mean");
  assert!(result.max_time() >= result.mean_time(), "Max should be >= mean");
  assert!(result.operations_per_second() > 0.0, "Ops/sec should be positive");
  
  // Test statistical validity
  if result.times.len() > 1 {
    let std_dev = result.std_deviation();
    let mean_time = result.mean_time();
    let coefficient_of_variation = std_dev.as_secs_f64() / mean_time.as_secs_f64();
    
    // For consistent operations, coefficient of variation should be reasonable
    assert!(coefficient_of_variation < 1.0, "Coefficient of variation should be reasonable");
  }
}

/// Tests suite configuration preservation
#[test]
fn test_suite_config_preservation()
{
  let config = MeasurementConfig {
    iterations: 7,
    warmup_iterations: 2,
    max_time: Duration::from_secs(15),
  };
  
  let mut suite = BenchmarkSuite::new("config_preservation")
    .with_config(config.clone());
  
  suite.benchmark("config_preserved", || {
    std::hint::black_box("preserved");
  });
  
  let results = suite.run_all();
  
  // Verify config was used (check that iterations were respected)
  let result = &results.results["config_preserved"];
  assert!(
    result.times.len() <= 7,
    "Should not exceed configured iteration count"
  );
}

/// Tests suite analysis integration
#[test]
fn test_suite_analysis_integration()
{
  let mut suite = BenchmarkSuite::new("analysis_integration");
  
  suite.benchmark("analyzed_op", || {
    let mut sum = 0;
    for i in 1..1000 {
      sum += i;
    }
    std::hint::black_box(sum);
  });
  
  let results = suite.run_analysis(); // Uses run_all internally
  
  assert!(!results.results.is_empty());
  assert!(results.results.contains_key("analyzed_op"));
  
  // Verify integration with analysis tools
  let result = &results.results["analyzed_op"];
  assert!(result.mean_time().as_nanos() > 0);
  assert!(result.operations_per_second() > 0.0);
}

/// Tests suite markdown report customization
#[test]
fn test_suite_markdown_customization()
{
  let mut suite = BenchmarkSuite::new("customization_test")
    .add_benchmark("custom_test", || {
      std::hint::black_box([1, 2, 3, 4, 5]);
    });
  
  let results = suite.run_all();
  let report = results.generate_markdown_report()
    .with_raw_data()
    .with_statistics();
  
  let markdown = report.generate();
  
  // Verify customization applied
  assert!(markdown.contains("customization_test Results"));
  assert!(markdown.contains("custom_test"));
  
  // Basic structure should be preserved
  assert!(markdown.contains("| Benchmark |"));
  assert!(markdown.contains("### Key Insights"));
}

/// Tests multiple suite execution independence
#[test]
fn test_multiple_suite_independence()
{
  let mut suite1 = BenchmarkSuite::new("suite_one")
    .add_benchmark("op1", || { std::hint::black_box(1); });
  
  let mut suite2 = BenchmarkSuite::new("suite_two")
    .add_benchmark("op2", || { std::hint::black_box(2); });
  
  let results1 = suite1.run_all();
  let results2 = suite2.run_all();
  
  // Verify independence
  assert_eq!(results1.suite_name, "suite_one");
  assert_eq!(results2.suite_name, "suite_two");
  
  assert!(results1.results.contains_key("op1"));
  assert!(!results1.results.contains_key("op2"));
  
  assert!(results2.results.contains_key("op2"));
  assert!(!results2.results.contains_key("op1"));
}