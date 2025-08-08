//! ## Test Matrix for Report Generation Functionality
//!
//! This test suite validates markdown report generation and file updating.
//!
//! ### Test Factors
//! - Report Format: Markdown table, Comprehensive report, JSON output
//! - Content Type: Empty results, Single result, Multiple results, With metrics
//! - File Operations: Section updating, File creation, Content preservation
//!
//! ### Test Combinations
//! | ID   | Format       | Content       | Operation      | Expected Behavior                    |
//! |------|--------------|---------------|----------------|--------------------------------------|
//! | R1.1 | Markdown     | Single        | Generate       | Valid markdown table                 |
//! | R1.2 | Markdown     | Multiple      | Generate       | Sorted by performance, insights      |
//! | R1.3 | Comprehensive| Multiple      | Generate       | Executive summary + detailed table   |
//! | R1.4 | Markdown     | Empty         | Generate       | "No results" message                 |
//! | R1.5 | File update  | Single        | Section replace| Preserve other sections              |
//! | R1.6 | File update  | Multiple      | New section    | Append section if not found          |
//! | R1.7 | JSON         | Multiple      | Generate       | Valid JSON with all metrics          |

use super::*;
use std::fs;
use tempfile::TempDir;

/// Tests basic markdown table generation with single result
/// Test Combination: R1.1
#[test]
fn test_single_result_markdown_generation()
{
  let mut results = HashMap::new();
  let test_result = BenchmarkResult::new("test_operation", vec![Duration::from_millis(10)]);
  results.insert("test_operation".to_string(), test_result);
  
  let generator = ReportGenerator::new("Single Test", results);
  let markdown = generator.generate_markdown_table();
  
  assert!(markdown.contains("| Operation |"), "Should contain table header");
  assert!(markdown.contains("test_operation"), "Should contain operation name");
  assert!(markdown.contains("10.00ms"), "Should contain formatted time");
  assert!(markdown.contains("100"), "Should contain ops/sec calculation");
}

/// Tests multiple results with performance sorting and insights
/// Test Combination: R1.2  
#[test]
fn test_multiple_results_markdown_with_sorting()
{
  let mut results = HashMap::new();
  
  // Add results with different performance characteristics
  results.insert("fast_op".to_string(), 
    BenchmarkResult::new("fast_op", vec![Duration::from_millis(5)]));
  results.insert("slow_op".to_string(),
    BenchmarkResult::new("slow_op", vec![Duration::from_millis(50)]));
  results.insert("medium_op".to_string(),
    BenchmarkResult::new("medium_op", vec![Duration::from_millis(25)]));
  
  let generator = ReportGenerator::new("Performance Test", results);
  let markdown = generator.generate_markdown_table();
  
  // Verify table structure
  assert!(markdown.contains("| Operation |"), "Should have table header");
  assert!(markdown.contains("fast_op"), "Should include fast operation");
  assert!(markdown.contains("slow_op"), "Should include slow operation");
  assert!(markdown.contains("medium_op"), "Should include medium operation");
  
  // Verify performance sorting (fastest first)
  let fast_pos = markdown.find("fast_op").unwrap();
  let medium_pos = markdown.find("medium_op").unwrap();  
  let slow_pos = markdown.find("slow_op").unwrap();
  
  assert!(fast_pos < medium_pos, "Fast operation should appear before medium");
  assert!(medium_pos < slow_pos, "Medium operation should appear before slow");
}

/// Tests comprehensive report generation with executive summary
/// Test Combination: R1.3
#[test]
fn test_comprehensive_report_generation()
{
  let mut results = HashMap::new();
  results.insert("operation_a".to_string(),
    BenchmarkResult::new("operation_a", vec![Duration::from_millis(10)]));
  results.insert("operation_b".to_string(), 
    BenchmarkResult::new("operation_b", vec![Duration::from_millis(30)]));
    
  let generator = ReportGenerator::new("Comprehensive Test", results);
  let report = generator.generate_comprehensive_report();
  
  // Should contain all major sections
  assert!(report.contains("# Comprehensive Test"), "Should have main title");
  assert!(report.contains("## Executive Summary"), "Should have executive summary");
  assert!(report.contains("**Fastest operation**"), "Should identify fastest operation");
  assert!(report.contains("**Performance range**"), "Should calculate performance range");
  assert!(report.contains("## Detailed Results"), "Should have detailed results section");
  assert!(report.contains("## Performance Insights"), "Should have insights section");
  
  // Verify performance analysis
  assert!(report.contains("operation_a"), "Should mention fastest operation");
  assert!(report.contains("3.0x difference"), "Should calculate correct performance ratio");
}

/// Tests empty results handling
/// Test Combination: R1.4
#[test]
fn test_empty_results_handling()
{
  let empty_results = HashMap::new();
  let generator = ReportGenerator::new("Empty Test", empty_results);
  
  let markdown = generator.generate_markdown_table();
  assert!(markdown.contains("No benchmark results available"), "Should handle empty results gracefully");
  
  let comprehensive = generator.generate_comprehensive_report();
  assert!(comprehensive.contains("# Empty Test"), "Should still have title");
  assert!(comprehensive.contains("No benchmark results available"), "Should indicate no results");
}

/// Tests markdown section replacement in existing files
/// Test Combination: R1.5
#[test]
fn test_markdown_section_replacement()
{
  let temp_dir = TempDir::new().unwrap();
  let file_path = temp_dir.path().join("test.md");
  
  // Create initial file with existing content
  let initial_content = r#"# My Project

## Introduction
This is the introduction.

## Performance
Old performance data here.
This will be replaced.

## Conclusion  
This is the conclusion.
"#;
  
  fs::write(&file_path, initial_content).unwrap();
  
  // Test section replacement
  let updater = MarkdownUpdater::new(&file_path, "Performance");
  updater.update_section("New performance data!").unwrap();
  
  let updated_content = fs::read_to_string(&file_path).unwrap();
  
  // Verify replacement
  assert!(updated_content.contains("New performance data!"), "Should contain new content");
  assert!(!updated_content.contains("Old performance data"), "Should not contain old content");
  
  // Verify preservation of other sections
  assert!(updated_content.contains("## Introduction"), "Should preserve Introduction section");
  assert!(updated_content.contains("This is the introduction"), "Should preserve Introduction content");
  assert!(updated_content.contains("## Conclusion"), "Should preserve Conclusion section");
  assert!(updated_content.contains("This is the conclusion"), "Should preserve Conclusion content");
}

/// Tests new section appending when section doesn't exist
/// Test Combination: R1.6
#[test]
fn test_new_section_appending()
{
  let temp_dir = TempDir::new().unwrap();
  let file_path = temp_dir.path().join("append_test.md");
  
  // Create file without Performance section
  let initial_content = r#"# My Project

## Introduction
Existing content here.
"#;
  
  fs::write(&file_path, initial_content).unwrap();
  
  // Add new section
  let updater = MarkdownUpdater::new(&file_path, "Performance");
  updater.update_section("This is new performance data.").unwrap();
  
  let updated_content = fs::read_to_string(&file_path).unwrap();
  
  // Verify section was appended
  assert!(updated_content.contains("## Performance"), "Should add new section");
  assert!(updated_content.contains("This is new performance data"), "Should add new content");
  
  // Verify existing content preserved
  assert!(updated_content.contains("## Introduction"), "Should preserve existing sections");
  assert!(updated_content.contains("Existing content here"), "Should preserve existing content");
}

/// Tests JSON report generation
/// Test Combination: R1.7
#[cfg(feature = "json_reports")]
#[test]
fn test_json_report_generation()
{
  let mut results = HashMap::new();
  let mut test_result = BenchmarkResult::new("json_test", vec![
    Duration::from_millis(10),
    Duration::from_millis(20),
  ]);
  test_result = test_result.with_metric("custom_metric", 42.0);
  results.insert("json_test".to_string(), test_result);
  
  let generator = ReportGenerator::new("JSON Test", results);
  let json_str = generator.generate_json().unwrap();
  
  // Parse JSON to verify structure
  let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();
  
  // Verify top-level structure
  assert_eq!(json["title"], "JSON Test", "Should contain correct title");
  assert!(json["timestamp"].is_string(), "Should contain timestamp");
  assert!(json["results"].is_object(), "Should contain results object");
  assert!(json["summary"].is_object(), "Should contain summary object");
  
  // Verify result details
  let result = &json["results"]["json_test"];
  assert!(result["mean_time_ms"].is_u64(), "Should contain mean time in milliseconds");
  assert!(result["mean_time_ns"].is_u64(), "Should contain mean time in nanoseconds");
  assert!(result["operations_per_second"].is_f64(), "Should contain ops/sec");
  assert_eq!(result["sample_count"], 2, "Should contain correct sample count");
  
  // Verify summary
  assert_eq!(json["summary"]["total_benchmarks"], 1, "Should count benchmarks");
  assert!(json["summary"]["performance_variance"].is_f64(), "Should calculate variance");
}

/// Tests performance insights generation
#[test]
fn test_performance_insights_generation()
{
  let mut results = HashMap::new();
  
  // Create results with diverse performance characteristics
  results.insert("very_fast".to_string(),
    BenchmarkResult::new("very_fast", vec![Duration::from_millis(1)]));
  results.insert("fast".to_string(),
    BenchmarkResult::new("fast", vec![Duration::from_millis(2)]));
  results.insert("medium".to_string(),
    BenchmarkResult::new("medium", vec![Duration::from_millis(10)]));
  results.insert("slow".to_string(),
    BenchmarkResult::new("slow", vec![Duration::from_millis(50)]));
  results.insert("very_slow".to_string(),
    BenchmarkResult::new("very_slow", vec![Duration::from_millis(100)]));
  
  let generator = ReportGenerator::new("Insights Test", results);
  let report = generator.generate_comprehensive_report();
  
  // Should categorize operations
  assert!(report.contains("**High-performance operations**"), "Should identify fast operations");
  assert!(report.contains("**Optimization candidates**"), "Should identify slow operations");
  
  // Should contain very_fast and fast in high-performance
  assert!(report.contains("very_fast"), "Should mention very fast operation");
  
  // Should contain performance variance analysis
  assert!(report.contains("variance"), "Should analyze performance variance");
}

/// Tests report generation with custom metrics
#[test] 
fn test_report_with_custom_metrics()
{
  let mut results = HashMap::new();
  let mut result_with_metrics = BenchmarkResult::new("metrics_test", vec![Duration::from_millis(15)]);
  result_with_metrics = result_with_metrics
    .with_metric("memory_usage_mb", 256.0)
    .with_metric("cache_hit_ratio", 0.95)
    .with_metric("allocations", 1000.0);
    
  results.insert("metrics_test".to_string(), result_with_metrics);
  
  let generator = ReportGenerator::new("Metrics Test", results);
  let markdown = generator.generate_markdown_table();
  
  // Basic table should still work with custom metrics
  assert!(markdown.contains("metrics_test"), "Should contain operation name");
  assert!(markdown.contains("15.00ms"), "Should contain timing data");
  
  // Custom metrics are stored but not displayed in basic table
  // (They would be available for JSON export or custom formatters)
}

/// Tests quick utility functions
#[test]
fn test_quick_utility_functions()
{
  let mut results = HashMap::new();
  results.insert("quick_test".to_string(),
    BenchmarkResult::new("quick_test", vec![Duration::from_millis(5)]));
  
  // Test quick markdown table generation
  let table = quick::results_to_markdown_table(&results);
  assert!(table.contains("| Operation |"), "Should generate table header");
  assert!(table.contains("quick_test"), "Should include operation");
  
  // Test quick file updating
  let temp_dir = TempDir::new().unwrap();
  let file_path = temp_dir.path().join("quick_test.md");
  
  // Create minimal file
  fs::write(&file_path, "# Test\n\n## Other Section\nContent.").unwrap();
  
  // Update using quick function
  quick::update_markdown_section(&results, &file_path, "Performance", "Quick Test Results")
    .unwrap();
    
  let content = fs::read_to_string(&file_path).unwrap();
  assert!(content.contains("## Performance"), "Should add Performance section");
  assert!(content.contains("quick_test"), "Should include benchmark data");
  assert!(content.contains("## Other Section"), "Should preserve existing sections");
}

/// Tests edge cases in markdown section replacement
#[test]
fn test_markdown_replacement_edge_cases()
{
  let temp_dir = TempDir::new().unwrap();
  let file_path = temp_dir.path().join("edge_test.md");
  
  // Test with file that doesn't exist
  let updater = MarkdownUpdater::new(&file_path, "New Section");
  updater.update_section("New content").unwrap();
  
  let content = fs::read_to_string(&file_path).unwrap();
  assert!(content.contains("## New Section"), "Should create new file with section");
  assert!(content.contains("New content"), "Should include new content");
  
  // Test with empty file
  fs::write(&file_path, "").unwrap();
  updater.update_section("Content in empty file").unwrap();
  
  let content = fs::read_to_string(&file_path).unwrap();
  assert!(content.contains("## New Section"), "Should handle empty file");
  assert!(content.contains("Content in empty file"), "Should add content to empty file");
}

/// Tests performance variance calculation
#[test]
fn test_performance_variance_calculation()
{
  let mut results = HashMap::new();
  
  // Low variance scenario (similar times)
  results.insert("consistent".to_string(),
    BenchmarkResult::new("consistent", vec![Duration::from_millis(10)]));
  results.insert("also_consistent".to_string(),
    BenchmarkResult::new("also_consistent", vec![Duration::from_millis(12)]));
  
  let low_variance_gen = ReportGenerator::new("Low Variance", results);
  let low_variance = low_variance_gen.calculate_performance_variance();
  
  // High variance scenario (very different times)
  let mut high_var_results = HashMap::new();
  high_var_results.insert("very_fast".to_string(),
    BenchmarkResult::new("very_fast", vec![Duration::from_millis(1)]));
  high_var_results.insert("very_slow".to_string(),
    BenchmarkResult::new("very_slow", vec![Duration::from_millis(1000)]));
  
  let high_variance_gen = ReportGenerator::new("High Variance", high_var_results);
  let high_variance = high_variance_gen.calculate_performance_variance();
  
  assert!(high_variance > low_variance, "High variance case should have higher variance value");
  assert!(high_variance > 0.5, "High variance should exceed threshold");
}