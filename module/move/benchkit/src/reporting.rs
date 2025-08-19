//! Report generation and markdown integration
//!
//! This module provides tools for generating reports from benchmark results,
//! with special focus on markdown integration for documentation updates.

use crate::measurement::BenchmarkResult;
use std::collections::HashMap;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Errors that can occur during markdown processing
#[derive(Debug)]
pub enum MarkdownError {
  /// Section name cannot be empty
  EmptySectionName,
  /// Section name is too long (max 100 characters)
  SectionNameTooLong,
  /// Section name contains invalid characters (newlines, etc.)
  InvalidCharacters,
  /// Potential section name conflicts detected
  SectionConflict { 
    /// List of conflicting section names
    conflicts: Vec<String> 
  },
  /// IO error during file operations
  Io(std::io::Error),
}

impl std::fmt::Display for MarkdownError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      MarkdownError::EmptySectionName => write!(f, "Section name cannot be empty"),
      MarkdownError::SectionNameTooLong => write!(f, "Section name is too long (max 100 characters)"),
      MarkdownError::InvalidCharacters => write!(f, "Section name contains invalid characters"),
      MarkdownError::SectionConflict { conflicts } => {
        write!(f, "Potential section name conflict detected: {:?}", conflicts)
      }
      MarkdownError::Io(err) => write!(f, "IO error: {}", err),
    }
  }
}

impl std::error::Error for MarkdownError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      MarkdownError::Io(err) => Some(err),
      _ => None,
    }
  }
}

impl From<std::io::Error> for MarkdownError {
  fn from(err: std::io::Error) -> Self {
    MarkdownError::Io(err)
  }
}

/// Markdown section updater for integrating benchmark results into documentation
#[derive(Debug)]
pub struct MarkdownUpdater {
  file_path: std::path::PathBuf,
  section_marker: String,
}

impl MarkdownUpdater {
  /// Create new markdown updater for specific file and section
  ///
  /// # Errors
  ///
  /// Returns an error if the section name is invalid or potentially conflicting.
  pub fn new(file_path: impl AsRef<Path>, section_name: &str) -> std::result::Result<Self, MarkdownError> {
    Self::validate_section_name(section_name)?;
    
    Ok(Self {
      file_path: file_path.as_ref().to_path_buf(),
      section_marker: format!("## {section_name}"),
    })
  }

  /// Create new markdown updater without validation (for backwards compatibility)
  ///
  /// # Safety
  ///
  /// This bypasses section name validation and may create conflicts.
  /// Use `new()` instead for safer API.
  pub fn new_unchecked(file_path: impl AsRef<Path>, section_name: &str) -> Self {
    Self {
      file_path: file_path.as_ref().to_path_buf(),
      section_marker: format!("## {section_name}"),
    }
  }

  /// Validate section name for safety
  fn validate_section_name(section_name: &str) -> std::result::Result<(), MarkdownError> {
    if section_name.trim().is_empty() {
      return Err(MarkdownError::EmptySectionName);
    }
    
    if section_name.len() > 100 {
      return Err(MarkdownError::SectionNameTooLong);
    }
    
    if section_name.contains('\n') || section_name.contains('\r') {
      return Err(MarkdownError::InvalidCharacters);
    }
    
    Ok(())
  }

  /// Check if this section name might conflict with existing sections
  ///
  /// # Errors
  ///
  /// Returns an error if the file cannot be read.
  pub fn check_conflicts(&self) -> std::result::Result<Vec<String>, MarkdownError> {
    if !self.file_path.exists() {
      return Ok(vec![]);
    }
    
    let content = std::fs::read_to_string(&self.file_path)?;
    let existing_sections = Self::extract_section_names(&content);
    
    let target_words: std::collections::HashSet<_> = self.section_marker
      .trim_start_matches("## ")
      .split_whitespace()
      .collect();
        
    let conflicts: Vec<String> = existing_sections
      .into_iter()
      .filter(|section| {
        let section_words: std::collections::HashSet<_> = section
          .trim_start_matches("## ")
          .split_whitespace()
          .collect();
        // Check for shared words that could cause substring conflicts
        !target_words.is_disjoint(&section_words) && section != &self.section_marker
      })
      .collect();
      
    Ok(conflicts)
  }
  
  /// Extract section names from markdown content
  fn extract_section_names(content: &str) -> Vec<String> {
    content.lines()
      .filter(|line| line.trim_start().starts_with("## "))
      .map(|line| line.trim().to_string())
      .collect()
  }

  /// Get the section marker (for testing)
  pub fn section_marker(&self) -> &str {
    &self.section_marker
  }

  /// Update the section with new content
  ///
  /// # Errors
  ///
  /// Returns an error if the file cannot be read or written.
  pub fn update_section(&self, content: &str) -> Result<()> {
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
  /// 
  /// This method is public for testing purposes to allow direct verification
  /// of section matching behavior.
  pub fn replace_section_content(&self, existing: &str, new_content: &str) -> String {
    let lines: Vec<&str> = existing.lines().collect();
    let mut result = Vec::new();
    let mut in_target_section = false;
    let mut found_section = false;

    for line in lines {
      if line.trim_start().starts_with("## ") {
        if line.trim() == self.section_marker.trim() {
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

  /// Generate markdown table format with statistical rigor indicators
  #[must_use]
  pub fn generate_markdown_table(&self) -> String {
    let mut output = String::new();
    
    if self.results.is_empty() {
      return "No benchmark results available.\n".to_string();
    }

    // Enhanced table header with statistical information
    output.push_str("| Operation | Mean Time | 95% CI | Ops/sec | CV | Reliability | Samples |\n");
    output.push_str("|-----------|-----------|--------|---------|----|-----------|---------|\n");

    // Sort results by performance (fastest first)
    let mut sorted_results: Vec<_> = self.results.iter().collect();
    sorted_results.sort_by(|a, b| a.1.mean_time().cmp(&b.1.mean_time()));

    // Table rows with statistical rigor information
    for (name, result) in sorted_results {
      let (ci_lower, ci_upper) = result.confidence_interval_95();
      let cv = result.coefficient_of_variation();
      let reliability = if result.is_reliable() { "✅" } else { "⚠️" };
      
      output.push_str(&format!(
        "| {} | {:.2?} | [{:.2?} - {:.2?}] | {:.0} | {:.1}% | {} | {} |\n",
        name,
        result.mean_time(),
        ci_lower,
        ci_upper,
        result.operations_per_second(),
        cv * 100.0,
        reliability,
        result.times.len()
      ));
    }

    output
  }

  /// Generate comprehensive statistical report with research-grade analysis
  #[must_use]
  pub fn generate_statistical_report(&self) -> String {
    let mut output = String::new();
    
    output.push_str(&format!("# {}\n\n", self.title));
    
    if self.results.is_empty() {
      return output + "No benchmark results available.\n";
    }

    // Executive summary
    output.push_str("## Executive Summary\n\n");
    
    let total_tests = self.results.len();
    let reliable_tests = self.results.values().filter(|r| r.is_reliable()).count();
    let reliability_rate = (reliable_tests as f64 / total_tests as f64) * 100.0;
    
    output.push_str(&format!("- **Total benchmarks**: {total_tests}\n"));
    output.push_str(&format!("- **Statistically reliable**: {reliable_tests}/{total_tests} ({reliability_rate:.1}%)\n"));
    
    if let Some((fastest_name, fastest_result)) = self.fastest_result() {
      output.push_str(&format!("- **Best performing**: {fastest_name} ({:.2?} ± {:.2?})\n", 
                               fastest_result.mean_time(),
                               fastest_result.standard_error()));
    }
    
    output.push('\n');

    // Performance results table with statistical information
    output.push_str("## Performance Results\n\n");
    output.push_str(&self.generate_markdown_table());
    output.push('\n');

    // Statistical quality assessment
    output.push_str("## Statistical Quality Assessment\n\n");
    
    let mut quality_issues = Vec::new();
    let mut high_quality_results = Vec::new();
    
    for (name, result) in &self.results {
      if result.is_reliable() {
        high_quality_results.push(name);
      } else {
        let cv = result.coefficient_of_variation();
        let sample_size = result.times.len();
        
        let mut issues = Vec::new();
        if sample_size < 10 {
          issues.push("insufficient sample size");
        }
        if cv > 0.1 {
          issues.push("high variability");
        }
        if result.max_time().as_secs_f64() / result.min_time().as_secs_f64() > 3.0 {
          issues.push("wide performance range");
        }
        
        quality_issues.push((name, issues));
      }
    }
    
    if !high_quality_results.is_empty() {
      output.push_str("### ✅ High Quality Results\n");
      output.push_str("*These results meet research-grade statistical standards*\n\n");
      for name in high_quality_results {
        let result = &self.results[name];
        output.push_str(&format!("- **{}**: {} samples, CV={:.1}%\n", 
                                 name, 
                                 result.times.len(),
                                 result.coefficient_of_variation() * 100.0));
      }
      output.push('\n');
    }
    
    if !quality_issues.is_empty() {
      output.push_str("### ⚠️ Quality Concerns\n");
      output.push_str("*These results may need additional measurement for reliable conclusions*\n\n");
      for (name, issues) in quality_issues {
        output.push_str(&format!("- **{}**: {}\n", name, issues.join(", ")));
      }
      output.push('\n');
    }

    // Methodology note
    output.push_str("## Statistical Methodology\n\n");
    output.push_str("**Reliability Criteria**: Results marked as reliable meet all of the following:\n");
    output.push_str("- Sample size ≥ 10 measurements\n");
    output.push_str("- Coefficient of variation ≤ 10%\n");
    output.push_str("- Max/min time ratio < 3.0x\n\n");
    
    output.push_str("**Confidence Intervals**: 95% confidence intervals calculated using t-distribution\n");
    output.push_str("**CV (Coefficient of Variation)**: Relative standard deviation (σ/μ)\n");
    output.push_str("**Statistical Significance**: Use p < 0.05 for hypothesis testing\n\n");

    output.push_str("---\n");
    output.push_str("*Report generated with benchkit - Research-grade statistical analysis*\n\n");

    output
  }

  /// Get the fastest (best performing) result
  fn fastest_result(&self) -> Option<(&String, &BenchmarkResult)> {
    self.results
      .iter()
      .min_by(|a, b| a.1.mean_time().cmp(&b.1.mean_time()))
  }

  /// Generate comprehensive markdown report
  ///
  /// # Panics
  ///
  /// Panics if the sorted results are empty but last() is called.
  #[must_use]
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
        output.push_str(&format!("**Performance range**: {ratio:.1}x difference between fastest and slowest\n"));
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
      let fast_list: Vec<String> = fast_ops.iter().map(|s| (*s).clone()).collect();
      output.push_str(&format!("**High-performance operations**: {}\n", fast_list.join(", ")));
    }
    if !slow_ops.is_empty() {
      let slow_list: Vec<String> = slow_ops.iter().map(|s| (*s).clone()).collect();
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
  #[must_use]
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
  ///
  /// # Errors
  ///
  /// Returns an error if the file cannot be read or written, or if section name is invalid.
  pub fn update_markdown_file(&self, file_path: impl AsRef<Path>, section_name: &str) -> Result<()> {
    let updater = MarkdownUpdater::new(file_path, section_name)
      .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let content = self.generate_comprehensive_report();
    updater.update_section(&content)
  }

  /// Update markdown file section with report (bypasses validation for compatibility)
  ///
  /// # Errors
  ///
  /// Returns an error if the file cannot be read or written.
  pub fn update_markdown_file_unchecked(&self, file_path: impl AsRef<Path>, section_name: &str) -> Result<()> {
    let updater = MarkdownUpdater::new_unchecked(file_path, section_name);
    let content = self.generate_comprehensive_report();
    updater.update_section(&content)
  }

  /// Generate JSON format report
  ///
  /// # Errors
  ///
  /// Returns an error if JSON serialization fails.
  #[cfg(feature = "json_reports")]
  pub fn generate_json(&self) -> Result<String> {
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

    Ok(serde_json::to_string_pretty(&report)?)
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
  ) -> Result<()> {
    let generator = ReportGenerator::new(title, results.clone());
    generator.update_markdown_file(file_path, section_name)
  }

  /// Quickly update a markdown section with benchmark results (unchecked)
  pub fn update_markdown_section_unchecked(
    results: &HashMap<String, BenchmarkResult>,
    file_path: impl AsRef<Path>,
    section_name: &str,
    title: &str
  ) -> Result<()> {
    let generator = ReportGenerator::new(title, results.clone());
    generator.update_markdown_file_unchecked(file_path, section_name)
  }

  /// Generate a simple markdown table from results
  pub fn results_to_markdown_table(results: &HashMap<String, BenchmarkResult>) -> String {
    let generator = ReportGenerator::new("Benchmark Results", results.clone());
    generator.generate_markdown_table()
  }
}

