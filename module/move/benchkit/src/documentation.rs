//! Documentation integration and auto-update utilities
//!
//! This module provides tools for automatically updating documentation
//! with benchmark results, maintaining performance metrics in README files,
//! and generating comprehensive reports.

use crate::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Documentation update configuration
#[derive(Debug, Clone)]
pub struct DocumentationConfig
{
  /// Path to the documentation file to update
  pub file_path: PathBuf,
  /// Section marker to find and replace (e.g., "## Performance")
  pub section_marker: String,
  /// Whether to add timestamp
  pub add_timestamp: bool,
  /// Backup original file
  pub create_backup: bool,
}

impl DocumentationConfig
{
  /// Create config for README.md performance section
  pub fn readme_performance(readme_path: impl AsRef<Path>) -> Self
  {
    Self
    {
      file_path: readme_path.as_ref().to_path_buf(),
      section_marker: "## Performance".to_string(),
      add_timestamp: true,
      create_backup: true,
    }
  }
  
  /// Create config for benchmark results section
  pub fn benchmark_results(file_path: impl AsRef<Path>, section: &str) -> Self
  {
    Self
    {
      file_path: file_path.as_ref().to_path_buf(),
      section_marker: section.to_string(),
      add_timestamp: true,
      create_backup: false,
    }
  }
}

/// Documentation updater
#[derive(Debug)]
pub struct DocumentationUpdater
{
  config: DocumentationConfig,
}

impl DocumentationUpdater
{
  /// Create new documentation updater
  pub fn new(config: DocumentationConfig) -> Self
  {
    Self { config }
  }
  
  /// Update documentation section with new content
  pub fn update_section(&self, new_content: &str) -> Result<DocumentationDiff>
  {
    // Read existing file
    let original_content = if self.config.file_path.exists()
    {
      fs::read_to_string(&self.config.file_path)?
    }
    else
    {
      String::new()
    };
    
    // Create backup if requested
    if self.config.create_backup && self.config.file_path.exists()
    {
      let backup_path = self.config.file_path.with_extension("md.backup");
      fs::copy(&self.config.file_path, &backup_path)?;
    }
    
    // Generate new content with timestamp if requested
    let timestamped_content = if self.config.add_timestamp
    {
      let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
      format!("<!-- Last updated: {} -->\n\n{}", timestamp, new_content)
    }
    else
    {
      new_content.to_string()
    };
    
    // Update the content
    let updated_content = self.replace_section(&original_content, &timestamped_content)?;
    
    // Write updated content
    fs::write(&self.config.file_path, &updated_content)?;
    
    Ok(DocumentationDiff
    {
      file_path: self.config.file_path.clone(),
      old_content: original_content,
      new_content: updated_content,
      section_marker: self.config.section_marker.clone(),
    })
  }
  
  /// Replace section in markdown content
  fn replace_section(&self, content: &str, new_section_content: &str) -> Result<String>
  {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut in_target_section = false;
    let mut section_found = false;
    
    // Handle timestamp header if it exists
    let mut start_idx = 0;
    if lines.first().map_or(false, |line| line.starts_with("<!--"))
    {
      // Skip existing timestamp
      start_idx = lines.iter().position(|line| !line.trim().is_empty() && !line.starts_with("<!--"))
        .unwrap_or(1);
    }
    
    for (_i, line) in lines.iter().enumerate().skip(start_idx)
    {
      if line.starts_with(&self.config.section_marker)
      {
        // Found our section - start replacement
        in_target_section = true;
        section_found = true;
        result.push(new_section_content);
      }
      else if in_target_section && line.starts_with("## ")
      {
        // Hit next section - stop replacement
        in_target_section = false;
        result.push(line);
      }
      else if !in_target_section
      {
        result.push(line);
      }
      // Skip lines while in target section (they get replaced)
    }
    
    // If section wasn't found, append it
    if !section_found
    {
      if !result.is_empty()
      {
        result.push("");
      }
      result.push(new_section_content);
    }
    
    Ok(result.join("\n"))
  }
}

/// Documentation update diff result
#[derive(Debug)]
pub struct DocumentationDiff
{
  /// Path to the updated file
  pub file_path: PathBuf,
  /// Original content before update
  pub old_content: String,
  /// New content after update
  pub new_content: String,
  /// Section marker used for update
  pub section_marker: String,
}

impl DocumentationDiff
{
  /// Display the diff
  pub fn display_diff(&self)
  {
    println!("📄 Documentation Updated: {:?}", self.file_path.file_name().unwrap_or_default());
    println!("═══════════════════════════");
    
    let old_lines: Vec<&str> = self.old_content.lines().collect();
    let new_lines: Vec<&str> = self.new_content.lines().collect();
    
    let mut changes_found = false;
    let max_lines = old_lines.len().max(new_lines.len());
    
    for i in 0..max_lines.min(50) // Limit output for readability
    {
      let old_line = old_lines.get(i).unwrap_or(&"");
      let new_line = new_lines.get(i).unwrap_or(&"");
      
      if old_line != new_line
      {
        changes_found = true;
        if !old_line.is_empty()
        {
          println!("- {}", old_line);
        }
        if !new_line.is_empty()
        {
          println!("+ {}", new_line);
        }
      }
    }
    
    if !changes_found
    {
      println!("  (No visible changes - timestamps updated)");
    }
    
    if max_lines > 50
    {
      println!("  ... ({} more lines)", max_lines - 50);
    }
    
    println!("═══════════════════════════");
  }
}

/// Benchmark documentation generator
#[derive(Debug)]
pub struct BenchmarkDocumentationGenerator;

impl BenchmarkDocumentationGenerator
{
  /// Generate performance comparison table from multiple results
  pub fn generate_comparison_table(
    title: &str,
    results: &[(String, BenchmarkResult)],
  ) -> String
  {
    let mut output = String::new();
    
    output.push_str(&format!("### {} Performance\n\n", title));
    output.push_str("| Algorithm | Mean Time | Ops/sec | Min | Max | Relative |\n");
    output.push_str("|-----------|-----------|---------|-----|-----|----------|\n");
    
    // Find fastest for relative comparison
    let fastest_ops = results.iter()
      .map(|(_, result)| result.operations_per_second())
      .fold(0.0f64, f64::max);
    
    for (name, result) in results
    {
      let ops_per_sec = result.operations_per_second();
      let relative = if ops_per_sec > 0.0
      {
        format!("{:.1}x", fastest_ops / ops_per_sec)
      }
      else
      {
        "N/A".to_string()
      };
      
      let relative_marker = if (ops_per_sec - fastest_ops).abs() < 1000.0
      {
        "**Fastest**"
      }
      else
      {
        &relative
      };
      
      output.push_str(&format!(
        "| {} | {:.2?} | {:.0} | {:.2?} | {:.2?} | {} |\n",
        name,
        result.mean_time(),
        ops_per_sec,
        result.times.iter().min().unwrap_or(&std::time::Duration::ZERO),
        result.times.iter().max().unwrap_or(&std::time::Duration::ZERO),
        relative_marker
      ));
    }
    
    output
  }
  
  /// Generate scaling analysis table
  pub fn generate_scaling_table(
    operation_name: &str,
    scaling_results: &[(usize, BenchmarkResult)],
  ) -> String
  {
    let mut output = String::new();
    
    output.push_str(&format!("### {} Scaling Performance\n\n", operation_name));
    output.push_str("| Scale | Mean Time | Ops/sec | Memory Est. | Complexity |\n");
    output.push_str("|-------|-----------|---------|-------------|------------|\n");
    
    for (scale, result) in scaling_results
    {
      let scale_display = if *scale >= 1000
      {
        format!("{}K", scale / 1000)
      }
      else
      {
        scale.to_string()
      };
      
      let memory_est = format!("~{:.1} KB", (*scale as f64) * 0.001);
      let complexity_hint = if scaling_results.len() > 1
      {
        "O(n)"
      }
      else
      {
        "Unknown"
      };
      
      output.push_str(&format!(
        "| {} | {:.2?} | {:.0} | {} | {} |\n",
        scale_display,
        result.mean_time(),
        result.operations_per_second(),
        memory_est,
        complexity_hint
      ));
    }
    
    output
  }
}

/// Auto-update README with benchmark results
pub fn update_readme_with_benchmarks<P: AsRef<Path>>(
  readme_path: P,
  benchmark_results: &[(String, BenchmarkResult)],
) -> Result<()>
{
  let config = DocumentationConfig::readme_performance(readme_path);
  let updater = DocumentationUpdater::new(config);
  
  let content = BenchmarkDocumentationGenerator::generate_comparison_table(
    "Latest Benchmark Results",
    benchmark_results,
  );
  
  let diff = updater.update_section(&content)?;
  diff.display_diff();
  
  Ok(())
}

