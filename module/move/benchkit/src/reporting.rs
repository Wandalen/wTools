//! Report generation and markdown integration
//!
//! This module provides tools for generating reports from benchmark results,
//! with special focus on markdown integration for documentation updates.

use crate::measurement::BenchmarkResult;
use std::collections::HashMap;
use std::path::Path;

/// Markdown section updater for integrating benchmark results into documentation
#[derive(Debug)]
pub struct MarkdownUpdater {
  file_path: std::path::PathBuf,
  section_marker: String,
}

impl MarkdownUpdater {
  /// Create new markdown updater for specific file and section
  pub fn new(file_path: impl AsRef<Path>, section_name: &str) -> Self {
    Self {
      file_path: file_path.as_ref().to_path_buf(),
      section_marker: format!("## {}", section_name),
    }
  }

  /// Update the section with new content
  pub fn update_section(&self, content: &str) -> error_tools::Result<()> {
    // Read existing file or create empty content
    let existing_content = if self.file_path.exists() {
      std::fs::read_to_string(&self.file_path)?
    } else {
      String::new()
    };

    let updated_content = self.replace_section_content(&existing_content, content);
    std::fs::write(&self.file_path, updated_content)?;
    
    Ok(())
  }

  /// Replace content between section marker and next section (or end)
  fn replace_section_content(&self, existing: &str, new_content: &str) -> String {
    let lines: Vec<&str> = existing.lines().collect();
    let mut result = Vec::new();
    let mut in_target_section = false;
    let mut found_section = false;

    for line in lines {
      if line.trim_start().starts_with("## ") {
        if line.contains(&self.section_marker.trim_start_matches("## ")) {
          // Found our target section
          result.push(line);
          result.push("");
          result.push(new_content);
          result.push("");
          in_target_section = true;
          found_section = true;
        } else if in_target_section {
          // Found next section, stop replacing
          in_target_section = false;
          result.push(line);
        } else {
          // Other section, keep as is
          result.push(line);
        }
      } else if !in_target_section {
        // Not in target section, keep line
        result.push(line);
      }
      // If in_target_section is true, we skip lines (they're being replaced)
    }

    // If section wasn't found, append it at the end
    if !found_section {
      if !existing.is_empty() && !result.is_empty() {
        result.push("");
      }
      result.push(&self.section_marker);
      result.push("");
      result.push(new_content);
    }

    result.join("\n")
  }
}

/// Performance report generator with multiple output formats
#[derive(Debug)]
pub struct ReportGenerator {
  results: HashMap<String, BenchmarkResult>,
  title: String,
}

impl ReportGenerator {
  /// Create new report generator
  pub fn new(title: impl Into<String>, results: HashMap<String, BenchmarkResult>) -> Self {
    Self {
      title: title.into(),
      results,
    }
  }

  /// Generate markdown table format
  pub fn generate_markdown_table(&self) -> String {
    let mut output = String::new();
    
    if self.results.is_empty() {
      return "No benchmark results available.\n".to_string();
    }

    // Table header
    output.push_str("| Operation | Mean Time | Ops/sec | Min | Max | Std Dev |\n");
    output.push_str("|-----------|-----------|---------|-----|-----|----------|\n");

    // Sort results by performance (fastest first)
    let mut sorted_results: Vec<_> = self.results.iter().collect();
    sorted_results.sort_by(|a, b| a.1.mean_time().cmp(&b.1.mean_time()));

    // Table rows
    for (name, result) in sorted_results {
      output.push_str(&format!(
        "| {} | {:.2?} | {:.0} | {:.2?} | {:.2?} | {:.2?} |\n",
        name,
        result.mean_time(),
        result.operations_per_second(),
        result.min_time(),
        result.max_time(),
        result.std_deviation()
      ));
    }

    output
  }

  /// Generate comprehensive markdown report
  pub fn generate_comprehensive_report(&self) -> String {
    let mut output = String::new();
    
    output.push_str(&format!("# {}\n\n", self.title));
    
    if self.results.is_empty() {
      output.push_str("No benchmark results available.\n");
      return output;
    }

    // Executive summary
    output.push_str("## Executive Summary\n\n");
    let sorted_results: Vec<_> = {
      let mut results: Vec<_> = self.results.iter().collect();
      results.sort_by(|a, b| a.1.mean_time().cmp(&b.1.mean_time()));
      results
    };
    
    if let Some((fastest_name, fastest_result)) = sorted_results.first() {
      output.push_str(&format!("**Fastest operation**: {} ({:.2?})\n", 
                               fastest_name, fastest_result.mean_time()));
      
      if sorted_results.len() > 1 {
        let slowest = sorted_results.last().unwrap();
        let ratio = slowest.1.mean_time().as_secs_f64() / fastest_result.mean_time().as_secs_f64();
        output.push_str(&format!("**Performance range**: {:.1}x difference between fastest and slowest\n", ratio));
      }
    }
    output.push('\n');

    // Detailed results
    output.push_str("## Detailed Results\n\n");
    output.push_str(&self.generate_markdown_table());
    output.push('\n');

    // Performance insights
    output.push_str("## Performance Insights\n\n");
    self.add_performance_insights(&mut output);

    output
  }

  /// Add performance insights section
  fn add_performance_insights(&self, output: &mut String) {
    let sorted_results: Vec<_> = {
      let mut results: Vec<_> = self.results.iter().collect();
      results.sort_by(|a, b| a.1.mean_time().cmp(&b.1.mean_time()));
      results
    };

    if sorted_results.len() < 2 {
      output.push_str("Not enough results for comparative analysis.\n");
      return;
    }

    // Performance tiers
    let fastest = sorted_results.first().unwrap().1;
    let _slowest = sorted_results.last().unwrap().1;
    let median_idx = sorted_results.len() / 2;
    let median = sorted_results[median_idx].1;

    // Categorize operations by performance
    let mut fast_ops = Vec::new();
    let mut medium_ops = Vec::new();  
    let mut slow_ops = Vec::new();

    let fast_threshold = fastest.mean_time().as_secs_f64() * 2.0;
    let slow_threshold = median.mean_time().as_secs_f64() * 2.0;

    for (name, result) in &sorted_results {
      let time = result.mean_time().as_secs_f64();
      if time <= fast_threshold {
        fast_ops.push(*name);
      } else if time <= slow_threshold {
        medium_ops.push(*name);
      } else {
        slow_ops.push(*name);
      }
    }

    // Generate insights
    if !fast_ops.is_empty() {
      let fast_list: Vec<String> = fast_ops.iter().map(|s| s.to_string()).collect();
      output.push_str(&format!("**High-performance operations**: {}\n", fast_list.join(", ")));
    }
    if !slow_ops.is_empty() {
      let slow_list: Vec<String> = slow_ops.iter().map(|s| s.to_string()).collect();
      output.push_str(&format!("**Optimization candidates**: {}\n", slow_list.join(", ")));
    }

    // Statistical insights
    let total_variance = self.calculate_performance_variance();
    if total_variance > 0.5 {
      output.push_str("**High performance variance detected** - consider investigating outliers.\n");
    }

    output.push('\n');
  }

  /// Calculate overall performance variance across results
  pub fn calculate_performance_variance(&self) -> f64 {
    if self.results.len() < 2 {
      return 0.0;
    }

    let times: Vec<f64> = self.results.values()
      .map(|r| r.mean_time().as_secs_f64())
      .collect();
    
    let mean = times.iter().sum::<f64>() / times.len() as f64;
    let variance = times.iter()
      .map(|&t| (t - mean).powi(2))
      .sum::<f64>() / times.len() as f64;
    
    variance.sqrt() / mean // Coefficient of variation
  }

  /// Update markdown file section with report
  pub fn update_markdown_file(&self, file_path: impl AsRef<Path>, section_name: &str) -> error_tools::Result<()> {
    let updater = MarkdownUpdater::new(file_path, section_name);
    let content = self.generate_comprehensive_report();
    updater.update_section(&content)
  }

  /// Generate JSON format report  
  #[cfg(feature = "json_reports")]
  pub fn generate_json(&self) -> Result<String, serde_json::Error> {
    use serde_json::json;
    
    let results_json: serde_json::Value = self.results.iter()
      .map(|(name, result)| {
        (name.clone(), json!({
          "mean_time_ms": result.mean_time().as_millis(),
          "mean_time_ns": result.mean_time().as_nanos(),
          "operations_per_second": result.operations_per_second(),
          "min_time_ns": result.min_time().as_nanos(),
          "max_time_ns": result.max_time().as_nanos(),
          "std_deviation_ns": result.std_deviation().as_nanos(),
          "sample_count": result.times.len()
        }))
      })
      .collect();

    let report = json!({
      "title": self.title,
      "timestamp": chrono::Utc::now().to_rfc3339(),
      "results": results_json,
      "summary": {
        "total_benchmarks": self.results.len(),
        "performance_variance": self.calculate_performance_variance()
      }
    });

    serde_json::to_string_pretty(&report)
  }
}

/// Convenience functions for quick report generation
pub mod quick {
  use super::*;

  /// Quickly update a markdown section with benchmark results
  pub fn update_markdown_section(
    results: &HashMap<String, BenchmarkResult>,
    file_path: impl AsRef<Path>,
    section_name: &str,
    title: &str
  ) -> error_tools::Result<()> {
    let generator = ReportGenerator::new(title, results.clone());
    generator.update_markdown_file(file_path, section_name)
  }

  /// Generate a simple markdown table from results
  pub fn results_to_markdown_table(results: &HashMap<String, BenchmarkResult>) -> String {
    let generator = ReportGenerator::new("Benchmark Results", results.clone());
    generator.generate_markdown_table()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::measurement::bench_once;
  use std::time::Duration;

  #[test]
  fn test_markdown_section_replacement() {
    let updater = MarkdownUpdater::new("test.md", "Performance");
    
    let existing = r#"# My Project

## Introduction
Some intro text.

## Performance
Old performance data here.
More old data.

## Conclusion
End text.
"#;

    let new_content = "New performance data!";
    let result = updater.replace_section_content(existing, new_content);
    
    assert!(result.contains("New performance data!"));
    assert!(!result.contains("Old performance data"));
    assert!(result.contains("## Introduction"));
    assert!(result.contains("## Conclusion"));
  }

  #[test]
  fn test_report_generation() {
    let mut results = HashMap::new();
    
    // Create some mock results
    results.insert("fast_op".to_string(), bench_once(|| {}));
    results.insert("slow_op".to_string(), bench_once(|| {
      std::thread::sleep(Duration::from_millis(1));
    }));

    let generator = ReportGenerator::new("Test Report", results);
    let markdown = generator.generate_markdown_table();
    
    assert!(markdown.contains("| Operation |"));
    assert!(markdown.contains("fast_op"));
    assert!(markdown.contains("slow_op"));
  }

  #[test]
  fn test_performance_insights() {
    let mut results = HashMap::new();
    results.insert("op1".to_string(), bench_once(|| {}));
    results.insert("op2".to_string(), bench_once(|| {}));
    
    let generator = ReportGenerator::new("Insights Test", results);
    let report = generator.generate_comprehensive_report();
    
    assert!(report.contains("## Performance Insights"));
  }
}