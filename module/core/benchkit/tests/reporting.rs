//! Tests for the reporting module
//!
//! These tests verify that markdown report generation and section updating work correctly,
//! with special focus on preventing section duplication bugs.

#![ cfg( feature = "integration" ) ]

use benchkit::prelude::*;
use core::time::Duration;
use std::collections::HashMap;

#[test]
fn test_no_section_duplication_with_overlapping_names()
{
  let initial = r"# Test Document

## Performance Benchmarks
Old performance data

## Language Operations Performance
Old language data

## Realistic Scenarios Performance
Old scenarios data";

  let updater = MarkdownUpdater::new_unchecked("test.md", "Performance Benchmarks");
  let result = updater.replace_section_content(initial, "New performance data");

  // Should have exactly 3 sections, not more due to duplication
  assert_eq!(result.matches("## ").count(), 3);
  
  // Should have exactly one "Performance Benchmarks" section
  assert_eq!(result.matches("## Performance Benchmarks").count(), 1);
  
  // Should contain the new data exactly once
  assert_eq!(result.matches("New performance data").count(), 1);
  
  // Other sections should remain unchanged
  assert!(result.contains("Old language data"));
  assert!(result.contains("Old scenarios data"));
}

#[test]
fn test_exact_section_matching_prevents_substring_conflicts()
{
  let initial = r"## API
Old API data

## API Documentation  
Old docs data";

  let updater = MarkdownUpdater::new_unchecked("test.md", "API");
  let result = updater.replace_section_content(initial, "New API data");

  // Should update only the exact "API" section, not "API Documentation"
  assert!(result.contains("New API data"));
  assert!(result.contains("Old docs data"));
  assert!(!result.contains("Old API data"));
  
  // Should have exactly 2 sections
  assert_eq!(result.matches("## ").count(), 2);
}

#[test]
fn test_section_matching_with_whitespace_variations()
{
  let initial = r"##   Performance Benchmarks   
Old data with spaces

## Performance Benchmarks
Old data exact";

  let updater = MarkdownUpdater::new_unchecked("test.md", "Performance Benchmarks");  
  let result = updater.replace_section_content(initial, "New data");

  // Should match only the exactly formatted section after trimming
  // This is the correct behavior - prevents fuzzy matching that caused the bug
  let new_data_count = result.matches("New data").count();
  assert_eq!(new_data_count, 1, "Only exact match should be updated");
  
  // The clean section gets updated, the poorly formatted one stays
  assert!(result.contains("Old data with spaces")); // This section is NOT updated
  assert!(!result.contains("Old data exact"));       // This section IS updated
}

#[test]
fn test_markdown_updater_creates_section_if_missing()
{
  let initial = r"# Test Document

## Existing Section
Some data";

  let updater = MarkdownUpdater::new_unchecked("test.md", "New Section");
  let result = updater.replace_section_content(initial, "New section content");

  // Should have the original section plus the new one
  assert!(result.contains("## Existing Section"));
  assert!(result.contains("## New Section"));
  assert!(result.contains("New section content"));
  
  // Should have 2 sections now
  assert_eq!(result.matches("## ").count(), 2);
}

#[test]
fn test_multiple_updates_dont_create_duplicates()
{
  let initial = r"## Performance Benchmarks
Original data";

  let updater = MarkdownUpdater::new_unchecked("test.md", "Performance Benchmarks");
  
  // First update
  let result1 = updater.replace_section_content(initial, "First update");
  assert_eq!(result1.matches("## Performance Benchmarks").count(), 1);
  
  // Second update on the result of first
  let result2 = updater.replace_section_content(&result1, "Second update");
  assert_eq!(result2.matches("## Performance Benchmarks").count(), 1);
  assert!(result2.contains("Second update"));
  assert!(!result2.contains("First update"));
}

#[test]
fn test_real_world_scenario_with_multiple_performance_sections()
{
  // This test simulates the exact scenario that caused the bug in wflow project
  let initial = r"# Benchmark Results

## Performance Benchmarks
Line counting results

## Language Operations Performance
Language detection results

## Realistic Scenarios Performance
End-to-end scenario results";

  // Update each section independently
  let updater1 = MarkdownUpdater::new_unchecked("test.md", "Performance Benchmarks");
  let result1 = updater1.replace_section_content(initial, "Updated line counting");
  
  let updater2 = MarkdownUpdater::new_unchecked("test.md", "Language Operations Performance");
  let result2 = updater2.replace_section_content(&result1, "Updated language detection");
  
  let updater3 = MarkdownUpdater::new_unchecked("test.md", "Realistic Scenarios Performance");
  let result3 = updater3.replace_section_content(&result2, "Updated scenarios");

  // Should still have exactly 3 sections
  assert_eq!(result3.matches("## ").count(), 3);
  
  // Each section should appear exactly once
  assert_eq!(result3.matches("## Performance Benchmarks").count(), 1);
  assert_eq!(result3.matches("## Language Operations Performance").count(), 1);
  assert_eq!(result3.matches("## Realistic Scenarios Performance").count(), 1);
  
  // Should have the correct updated content
  assert!(result3.contains("Updated line counting"));
  assert!(result3.contains("Updated language detection"));
  assert!(result3.contains("Updated scenarios"));
}

#[test]
fn test_report_generator_with_markdown_updater_integration()
{
  let mut results = HashMap::new();
  let test_result = BenchmarkResult::new("test_operation", vec![Duration::from_millis(5)]);
  results.insert("test_operation".to_string(), test_result);

  let generator = ReportGenerator::new("Integration Test", results);
  let report_content = generator.generate_comprehensive_report();

  // Verify report contains expected sections
  assert!(report_content.contains("# Integration Test"));
  assert!(report_content.contains("## Executive Summary"));
  assert!(report_content.contains("## Detailed Results"));
  
  // Test that MarkdownUpdater can handle this content
  let updater = MarkdownUpdater::new_unchecked("test.md", "Integration Test");
  let result = updater.replace_section_content("", &report_content);
  
  assert!(result.contains("## Integration Test"));
  assert_eq!(result.matches("## Integration Test").count(), 1);
}

#[test]
fn test_section_name_validation()
{
  // Valid section names should work
  assert!(MarkdownUpdater::new("test.md", "Valid Section").is_ok());
  assert!(MarkdownUpdater::new("test.md", "Performance Benchmarks").is_ok());
  
  // Empty section name should fail
  assert!(MarkdownUpdater::new("test.md", "").is_err());
  assert!(MarkdownUpdater::new("test.md", "   ").is_err());
  
  // Too long section name should fail
  let long_name = "a".repeat(101);
  assert!(MarkdownUpdater::new("test.md", &long_name).is_err());
  
  // Section names with newlines should fail
  assert!(MarkdownUpdater::new("test.md", "Line\nBreak").is_err());
  assert!(MarkdownUpdater::new("test.md", "Carriage\rReturn").is_err());
}

#[test]
fn test_conflict_detection()
{
  // Create a temporary file with existing sections
  let content = r"# Test Document

## Performance Benchmarks
Old performance data

## Language Operations Performance
Old language data";
  
  std::fs::write("conflict_test.md", content).unwrap();
  
  // Test conflict detection for overlapping names
  let updater = MarkdownUpdater::new("conflict_test.md", "Performance Results").unwrap();
  let conflicts = updater.check_conflicts().unwrap();
  
  // Should detect conflicts with both existing sections that contain "Performance"
  assert!(!conflicts.is_empty());
  assert!(conflicts.iter().any(|c| c.contains("Performance Benchmarks")));
  assert!(conflicts.iter().any(|c| c.contains("Language Operations Performance")));
  
  // Clean up
  std::fs::remove_file("conflict_test.md").ok();
}

#[test] 
fn test_no_conflict_with_distinct_names()
{
  let content = r"## API Documentation
Content

## Database Schema  
Content";
  
  std::fs::write("no_conflict_test.md", content).unwrap();
  
  let updater = MarkdownUpdater::new("no_conflict_test.md", "Testing Results").unwrap();
  let conflicts = updater.check_conflicts().unwrap();
  
  // Should not detect conflicts with completely different section names
  assert!(conflicts.is_empty());
  
  // Clean up
  std::fs::remove_file("no_conflict_test.md").ok();
}

#[test]
fn test_safe_vs_unchecked_api()
{
  // Safe API should reject problematic names
  assert!(MarkdownUpdater::new("test.md", "").is_err());
  
  // Unchecked API should allow them (for backwards compatibility)
  let updater = MarkdownUpdater::new_unchecked("test.md", "");
  assert_eq!(updater.section_marker(), "## ");
  
  // Both should work for valid names
  assert!(MarkdownUpdater::new("test.md", "Valid").is_ok());
  let updater2 = MarkdownUpdater::new_unchecked("test.md", "Valid");
  assert_eq!(updater2.section_marker(), "## Valid");
}