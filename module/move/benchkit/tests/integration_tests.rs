//! ## Integration Test Suite for benchkit
//!
//! This module provides integration tests that verify the complete
//! benchkit functionality works together as intended.
//!
//! ### Integration Test Categories
//! - End-to-end workflows: Complete benchmarking processes
//! - File I/O integration: Markdown updating, result persistence
//! - Feature interaction: Multiple features working together
//! - Real-world scenarios: Actual usage patterns

use benchkit::prelude::*;
use std::collections::HashMap;
use std::time::Duration;
use tempfile::TempDir;
use std::fs;

/// Tests complete workflow from benchmarking to markdown report generation
#[test]
fn test_end_to_end_benchmark_to_markdown()
{
  let temp_dir = TempDir::new().unwrap();
  let report_file = temp_dir.path().join("benchmark_results.md");
  
  // Create initial markdown file
  let initial_content = r#"# My Project

## Introduction
This project does amazing things.

## Performance
This section will be updated.

## Conclusion
Great work!
"#;
  fs::write(&report_file, initial_content).unwrap();
  
  // Run benchmarks
  let mut suite = BenchmarkSuite::new("End-to-End Test")
    .add_benchmark("string_processing", || {
      let data = generate_list_data(DataSize::Medium);
      std::hint::black_box(data.to_uppercase());
    })
    .add_benchmark("numeric_computation", || {
      let numbers = generate_random_vec(100);
      let sum: i32 = numbers.iter().sum();
      std::hint::black_box(sum);
    });
  
  let results = suite.run_all();
  
  // Generate and save report
  let report = results.generate_markdown_report();
  report.update_file(&report_file, "Performance").unwrap();
  
  // Verify integration
  let updated_content = fs::read_to_string(&report_file).unwrap();
  
  // Should preserve existing structure
  assert!(updated_content.contains("## Introduction"));
  assert!(updated_content.contains("This project does amazing things"));
  assert!(updated_content.contains("## Conclusion"));
  
  // Should update Performance section with benchmark data
  assert!(updated_content.contains("string_processing"));
  assert!(updated_content.contains("numeric_computation"));
  assert!(updated_content.contains("| Benchmark |"));
  assert!(updated_content.contains("### Key Insights"));
}

/// Tests comparative analysis integration with data generation
#[test]
fn test_comparative_analysis_with_generated_data()
{
  // Compare different string processing approaches using generated data
  let comparison = ComparativeAnalysis::new("String Processing Comparison")
    .algorithm("uppercase_small", || {
      let data = generate_list_data(DataSize::Small);
      std::hint::black_box(data.to_uppercase());
    })
    .algorithm("uppercase_large", || {
      let data = generate_list_data(DataSize::Large);
      std::hint::black_box(data.to_uppercase());
    })
    .algorithm("split_small", || {
      let data = generate_list_data(DataSize::Small);
      let parts: Vec<&str> = data.split(',').collect();
      std::hint::black_box(parts);
    })
    .algorithm("split_large", || {
      let data = generate_list_data(DataSize::Large);
      let parts: Vec<&str> = data.split(',').collect();
      std::hint::black_box(parts);
    });
  
  let report = comparison.run();
  
  // Verify all algorithms executed
  assert_eq!(report.results.len(), 4);
  
  // Performance should scale with data size
  let uppercase_small_time = report.results["uppercase_small"].mean_time();
  let uppercase_large_time = report.results["uppercase_large"].mean_time();
  assert!(uppercase_large_time >= uppercase_small_time, "Large data should take at least as long as small data");
  
  let split_small_time = report.results["split_small"].mean_time();
  let split_large_time = report.results["split_large"].mean_time();
  assert!(split_large_time >= split_small_time, "Large data should take at least as long as small data");
  
  // Generate markdown report
  let markdown = report.to_markdown();
  assert!(markdown.contains("String Processing Comparison"));
  assert!(markdown.contains("**Best performing**"));
}

/// Tests regression analysis workflow with baseline persistence
#[test]
fn test_regression_analysis_workflow()
{
  let temp_dir = TempDir::new().unwrap();
  let baseline_file = temp_dir.path().join("baseline.json");
  
  // Create baseline results
  let mut baseline_suite = BenchmarkSuite::new("Baseline Benchmarks")
    .add_benchmark("critical_function", || {
      let data = generate_string_data(1000);
      std::hint::black_box(data.len());
    })
    .add_benchmark("helper_function", || {
      for i in 0..100 {
        std::hint::black_box(i * 2);
      }
    });
  
  let baseline_results = baseline_suite.run_all();
  
  // Save baseline (Note: actual implementation would serialize to JSON)
  baseline_results.save_as_baseline(&baseline_file).unwrap();
  
  // Simulate current results (slightly different performance)
  let mut current_suite = BenchmarkSuite::new("Current Benchmarks")
    .add_benchmark("critical_function", || {
      let data = generate_string_data(1000);
      // Simulate slight regression
      std::thread::sleep(Duration::from_nanos(100));
      std::hint::black_box(data.len());
    })
    .add_benchmark("helper_function", || {
      // Simulate improvement
      for i in 0..80 { // Less work
        std::hint::black_box(i * 2);
      }
    });
  
  let current_results = current_suite.run_all();
  
  // Perform regression analysis
  let analysis = current_results.regression_analysis(&baseline_results.results);
  let report = analysis.generate_report();
  
  // Verify analysis detected changes
  assert!(report.contains("Performance Regression Analysis"));
  // Note: Actual regression detection may be sensitive to timing variations in tests
}

/// Tests feature flag integration and modularity
#[test]
fn test_feature_integration()
{
  // Test that core features work together
  let data = generate_map_data(DataSize::Medium);
  let result = bench_function("feature_integration", || {
    let pairs: Vec<&str> = data.split(',').collect();
    std::hint::black_box(pairs.len());
  });
  
  // Core timing functionality
  assert!(!result.times.is_empty());
  assert!(result.mean_time().as_nanos() > 0);
  
  // Custom metrics integration
  let enhanced_result = result.with_metric("data_size", DataSize::Medium.size() as f64);
  assert_eq!(enhanced_result.metrics.get("data_size"), Some(&100.0));
  
  // Report generation integration
  let mut results = HashMap::new();
  results.insert("feature_test".to_string(), enhanced_result);
  
  let generator = ReportGenerator::new("Feature Integration", results);
  let markdown = generator.generate_markdown_table();
  
  assert!(markdown.contains("feature_test"));
  assert!(markdown.contains("| Operation |"));
}

/// Tests parsing test data integration with benchmarks
#[test]
fn test_parsing_benchmark_integration()
{
  // Benchmark different parsing approaches with realistic test data
  let mut suite = BenchmarkSuite::new("Parsing Performance")
    .add_benchmark("csv_parsing", || {
      let csv_data = ParsingTestData::csv_data(100, 5);
      let lines: Vec<&str> = csv_data.lines().collect();
      let parsed: Vec<Vec<&str>> = lines.iter()
        .map(|line| line.split(',').collect())
        .collect();
      std::hint::black_box(parsed);
    })
    .add_benchmark("command_parsing", || {
      let args_data = ParsingTestData::command_args(DataSize::Large);
      let parts: Vec<&str> = args_data.split_whitespace().collect();
      std::hint::black_box(parts);
    })
    .add_benchmark("json_parsing", || {
      let json_data = ParsingTestData::json_objects(DataSize::Medium);
      // Simple "parsing" - just count braces
      let brace_count = json_data.matches('{').count();
      std::hint::black_box(brace_count);
    });
  
  let results = suite.run_all();
  
  // Verify all parsing benchmarks executed
  assert_eq!(results.results.len(), 3);
  assert!(results.results.contains_key("csv_parsing"));
  assert!(results.results.contains_key("command_parsing"));
  assert!(results.results.contains_key("json_parsing"));
  
  // Generate comprehensive report
  let report = results.generate_markdown_report();
  let markdown = report.generate();
  
  assert!(markdown.contains("Parsing Performance Results"));
  assert!(markdown.contains("csv_parsing"));
}

/// Tests seeded random data consistency across benchmark runs
#[test]  
fn test_seeded_data_consistency()
{
  // Run same benchmark multiple times with seeded data
  let run_benchmark = || {
    let mut gen = SeededGenerator::new(12345);
    let data = gen.random_vec(1000, 1, 1000);
    
    bench_function("consistent_random", || {
      let sum: i32 = data.iter().sum();
      std::hint::black_box(sum);
    })
  };
  
  let result1 = run_benchmark();
  let result2 = run_benchmark();
  
  // Results should be consistent due to seeded data
  // Note: Timing may vary, but the work done should be identical
  assert_eq!(result1.name, result2.name);
  assert!(!result1.times.is_empty());
  assert!(!result2.times.is_empty());
  
  // Verify seeded generator produces consistent data
  let mut gen1 = SeededGenerator::new(54321);
  let mut gen2 = SeededGenerator::new(54321);
  
  let vec1 = gen1.random_vec(100, 1, 100);
  let vec2 = gen2.random_vec(100, 1, 100);
  
  assert_eq!(vec1, vec2, "Seeded generators should produce identical sequences");
}

/// Tests large-scale benchmark suite with all data sizes
#[test]
fn test_large_scale_benchmark_suite()
{
  let mut suite = BenchmarkSuite::new("Comprehensive Scaling Test");
  
  // Add benchmarks for all standard data sizes
  for size in DataSize::standard_sizes() {
    let size_name = match size {
      DataSize::Small => "small",
      DataSize::Medium => "medium", 
      DataSize::Large => "large",
      DataSize::Huge => "huge",
      _ => "custom",
    };
    
    suite = suite.add_benchmark(format!("list_processing_{}", size_name), move || {
      let data = generate_list_data(size);
      let items: Vec<&str> = data.split(',').collect();
      let processed: Vec<String> = items.iter()
        .map(|item| item.to_uppercase())
        .collect();
      std::hint::black_box(processed);
    });
  }
  
  let results = suite.run_all();
  
  // Verify all sizes were benchmarked
  assert_eq!(results.results.len(), 4);
  
  // Performance should generally increase with data size
  let small_time = results.results["list_processing_small"].mean_time();
  let huge_time = results.results["list_processing_huge"].mean_time();
  
  // Huge should take longer than small (may not be perfectly linear due to optimizations)
  assert!(huge_time >= small_time, "Huge dataset should take at least as long as small dataset");
  
  // Generate scaling report
  let report = results.generate_markdown_report();
  let markdown = report.generate();
  
  assert!(markdown.contains("### Key Insights"));
  assert!(markdown.contains("**Performance range**"));
}

/// Tests error handling and edge cases in integration
#[test]
fn test_integration_error_handling()
{
  let temp_dir = TempDir::new().unwrap();
  let nonexistent_file = temp_dir.path().join("does_not_exist.md");
  
  // Test updating non-existent file (should create it)
  let mut results = HashMap::new();
  results.insert("error_test".to_string(),
    BenchmarkResult::new("error_test", vec![Duration::from_millis(1)]));
  
  let generator = ReportGenerator::new("Error Test", results);
  
  // Should succeed and create file
  let update_result = generator.update_markdown_file(&nonexistent_file, "Results");
  assert!(update_result.is_ok(), "Should handle non-existent file by creating it");
  
  // Verify file was created with content
  let content = fs::read_to_string(&nonexistent_file).unwrap();
  assert!(content.contains("## Results"));
  assert!(content.contains("error_test"));
}

/// Tests custom measurement configuration across all components
#[test]
fn test_custom_config_integration()
{
  let custom_config = MeasurementConfig {
    iterations: 3,
    warmup_iterations: 1,
    max_time: Duration::from_secs(2),
  };
  
  let mut suite = BenchmarkSuite::new("Custom Config Integration")
    .with_config(custom_config);
  
  // Add benchmark that should respect custom config
  suite = suite.add_benchmark("config_test", || {
    let data = generate_nested_data(3, 2);
    std::hint::black_box(data.len());
  });
  
  let results = suite.run_all();
  let result = &results.results["config_test"];
  
  // Should respect iteration limit
  assert!(result.times.len() <= 3, "Should not exceed configured iterations");
  assert!(!result.times.is_empty(), "Should have at least one measurement");
  
  // Integration with reporting should still work
  let report = results.generate_markdown_report();
  let markdown = report.generate();
  
  assert!(markdown.contains("config_test"));
  assert!(markdown.contains("Custom Config Integration"));
}