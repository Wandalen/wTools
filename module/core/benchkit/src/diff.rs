//! Git-style diff functionality for benchmark results
//!
//! This module provides utilities for comparing benchmark results across
//! different runs, implementations, or time periods, similar to git diff
//! but specialized for performance metrics.

use crate::prelude::*;
use std::collections::HashMap;

/// Represents a diff between two benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkDiff
{
  /// Name of the benchmark being compared
  pub benchmark_name: String,
  /// Baseline (old) result
  pub baseline: BenchmarkResult,
  /// Current (new) result
  pub current: BenchmarkResult,
  /// Performance change analysis
  pub analysis: PerformanceChange,
}

/// Analysis of performance change between two results
#[derive(Debug, Clone)]
pub struct PerformanceChange
{
  /// Percentage change in operations per second (positive = improvement)
  pub ops_per_sec_change: f64,
  /// Percentage change in mean execution time (negative = improvement)
  pub mean_time_change: f64,
  /// Change classification
  pub change_type: ChangeType,
  /// Statistical significance (if determinable)
  pub significance: ChangeSignificanceLevel,
  /// Human-readable summary
  pub summary: String,
}

/// Classification of performance change
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeType
{
  /// Significant improvement
  Improvement,
  /// Significant regression
  Regression,
  /// Minor improvement (within noise threshold)
  MinorImprovement,
  /// Minor regression (within noise threshold)
  MinorRegression,
  /// No meaningful change
  NoChange,
}

/// Statistical significance level
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeSignificanceLevel
{
  /// High confidence change (>20% difference)
  High,
  /// Medium confidence change (5-20% difference)
  Medium,
  /// Low confidence change (1-5% difference)
  Low,
  /// Not significant (<1% difference)
  NotSignificant,
}

impl BenchmarkDiff
{
  /// Create a new benchmark diff
  pub fn new(
    benchmark_name: &str,
    baseline: BenchmarkResult,
    current: BenchmarkResult,
  ) -> Self
  {
    let analysis = Self::analyze_change(&baseline, &current);
    
    Self
    {
      benchmark_name: benchmark_name.to_string(),
      baseline,
      current,
      analysis,
    }
  }
  
  /// Analyze the performance change between two results
  fn analyze_change(baseline: &BenchmarkResult, current: &BenchmarkResult) -> PerformanceChange
  {
    let baseline_ops = baseline.operations_per_second();
    let current_ops = current.operations_per_second();
    
    let baseline_mean = baseline.mean_time().as_secs_f64();
    let current_mean = current.mean_time().as_secs_f64();
    
    // Calculate percentage changes
    let ops_change = if baseline_ops > 0.0
    {
      ((current_ops - baseline_ops) / baseline_ops) * 100.0
    }
    else
    {
      0.0
    };
    
    let time_change = if baseline_mean > 0.0
    {
      ((current_mean - baseline_mean) / baseline_mean) * 100.0
    }
    else
    {
      0.0
    };
    
    // Determine significance and change type
    let abs_ops_change = ops_change.abs();
    let significance = if abs_ops_change > 20.0
    {
      ChangeSignificanceLevel::High
    }
    else if abs_ops_change > 5.0
    {
      ChangeSignificanceLevel::Medium
    }
    else if abs_ops_change > 1.0
    {
      ChangeSignificanceLevel::Low
    }
    else
    {
      ChangeSignificanceLevel::NotSignificant
    };
    
    let change_type = match significance
    {
      ChangeSignificanceLevel::High =>
      {
        if ops_change > 0.0
        {
          ChangeType::Improvement
        }
        else
        {
          ChangeType::Regression
        }
      }
      ChangeSignificanceLevel::Medium =>
      {
        if ops_change > 0.0
        {
          ChangeType::MinorImprovement
        }
        else
        {
          ChangeType::MinorRegression
        }
      }
      ChangeSignificanceLevel::Low =>
      {
        if ops_change > 0.0
        {
          ChangeType::MinorImprovement
        }
        else
        {
          ChangeType::MinorRegression
        }
      }
      ChangeSignificanceLevel::NotSignificant => ChangeType::NoChange,
    };
    
    // Generate summary
    let summary = match change_type
    {
      ChangeType::Improvement => format!("🚀 Performance improved by {:.1}%", ops_change),
      ChangeType::Regression => format!("📉 Performance regressed by {:.1}%", ops_change.abs()),
      ChangeType::MinorImprovement => format!("📈 Minor improvement: +{:.1}%", ops_change),
      ChangeType::MinorRegression => format!("📊 Minor regression: -{:.1}%", ops_change.abs()),
      ChangeType::NoChange => "🔄 No significant change".to_string(),
    };
    
    PerformanceChange
    {
      ops_per_sec_change: ops_change,
      mean_time_change: time_change,
      change_type,
      significance,
      summary,
    }
  }
  
  /// Generate a git-style diff output
  pub fn to_diff_format(&self) -> String
  {
    let mut output = String::new();
    
    // Header similar to git diff
    output.push_str(&format!("diff --benchmark a/{} b/{}\n", self.benchmark_name, self.benchmark_name));
    output.push_str(&format!("index baseline..current\n"));
    output.push_str(&format!("--- a/{}\n", self.benchmark_name));
    output.push_str(&format!("+++ b/{}\n", self.benchmark_name));
    output.push_str("@@");
    
    match self.analysis.change_type
    {
      ChangeType::Improvement => output.push_str(" Performance Improvement "),
      ChangeType::Regression => output.push_str(" Performance Regression "),
      ChangeType::MinorImprovement => output.push_str(" Minor Improvement "),
      ChangeType::MinorRegression => output.push_str(" Minor Regression "),
      ChangeType::NoChange => output.push_str(" No Change "),
    }
    
    output.push_str("@@\n");
    
    // Show the changes
    let baseline_ops = self.baseline.operations_per_second();
    let current_ops = self.current.operations_per_second();
    
    output.push_str(&format!("-Operations/sec: {:.0}\n", baseline_ops));
    output.push_str(&format!("+Operations/sec: {:.0}\n", current_ops));
    
    output.push_str(&format!("-Mean time: {:.2?}\n", self.baseline.mean_time()));
    output.push_str(&format!("+Mean time: {:.2?}\n", self.current.mean_time()));
    
    // Add summary
    output.push_str(&format!("\nSummary: {}\n", self.analysis.summary));
    
    output
  }
  
  /// Generate a concise diff summary
  pub fn to_summary(&self) -> String
  {
    let change_symbol = match self.analysis.change_type
    {
      ChangeType::Improvement => "✅",
      ChangeType::Regression => "❌",
      ChangeType::MinorImprovement => "📈",
      ChangeType::MinorRegression => "📉",
      ChangeType::NoChange => "🔄",
    };
    
    format!(
      "{} {}: {} ({:.0} → {:.0} ops/sec)",
      change_symbol,
      self.benchmark_name,
      self.analysis.summary,
      self.baseline.operations_per_second(),
      self.current.operations_per_second()
    )
  }
  
  /// Check if this represents a significant change
  pub fn is_significant(&self) -> bool
  {
    matches!(
      self.analysis.significance,
      ChangeSignificanceLevel::High | ChangeSignificanceLevel::Medium
    )
  }
  
  /// Check if this represents a regression
  pub fn is_regression(&self) -> bool
  {
    matches!(
      self.analysis.change_type,
      ChangeType::Regression | ChangeType::MinorRegression
    )
  }
  
  /// Check if this represents an improvement
  pub fn is_improvement(&self) -> bool
  {
    matches!(
      self.analysis.change_type,
      ChangeType::Improvement | ChangeType::MinorImprovement
    )
  }
}

/// Collection of benchmark diffs for comparing multiple benchmarks
#[derive(Debug, Clone)]
pub struct BenchmarkDiffSet
{
  /// Individual benchmark diffs
  pub diffs: Vec<BenchmarkDiff>,
  /// Timestamp of baseline results
  pub baseline_timestamp: Option<String>,
  /// Timestamp of current results
  pub current_timestamp: Option<String>,
  /// Overall summary statistics
  pub summary_stats: DiffSummaryStats,
}

/// Summary statistics for a diff set
#[derive(Debug, Clone)]
pub struct DiffSummaryStats
{
  /// Total number of benchmarks compared
  pub total_benchmarks: usize,
  /// Number of improvements
  pub improvements: usize,
  /// Number of regressions
  pub regressions: usize,
  /// Number of no-change results
  pub no_change: usize,
  /// Average performance change percentage
  pub average_change: f64,
}

impl BenchmarkDiffSet
{
  /// Create a new diff set from baseline and current results
  pub fn compare_results(
    baseline_results: &[(String, BenchmarkResult)],
    current_results: &[(String, BenchmarkResult)],
  ) -> Self
  {
    let mut diffs = Vec::new();
    let baseline_map: HashMap<&String, &BenchmarkResult> = baseline_results.iter().map(|(k, v)| (k, v)).collect();
    let _current_map: HashMap<&String, &BenchmarkResult> = current_results.iter().map(|(k, v)| (k, v)).collect();
    
    // Find matching benchmarks and create diffs
    for (name, current_result) in current_results
    {
      if let Some(baseline_result) = baseline_map.get(name)
      {
        let diff = BenchmarkDiff::new(name, (*baseline_result).clone(), current_result.clone());
        diffs.push(diff);
      }
    }
    
    let summary_stats = Self::calculate_summary_stats(&diffs);
    
    Self
    {
      diffs,
      baseline_timestamp: None,
      current_timestamp: None,
      summary_stats,
    }
  }
  
  /// Calculate summary statistics
  fn calculate_summary_stats(diffs: &[BenchmarkDiff]) -> DiffSummaryStats
  {
    let total = diffs.len();
    let mut improvements = 0;
    let mut regressions = 0;
    let mut no_change = 0;
    let mut total_change = 0.0;
    
    for diff in diffs
    {
      match diff.analysis.change_type
      {
        ChangeType::Improvement | ChangeType::MinorImprovement => improvements += 1,
        ChangeType::Regression | ChangeType::MinorRegression => regressions += 1,
        ChangeType::NoChange => no_change += 1,
      }
      
      total_change += diff.analysis.ops_per_sec_change;
    }
    
    let average_change = if total > 0 { total_change / total as f64 } else { 0.0 };
    
    DiffSummaryStats
    {
      total_benchmarks: total,
      improvements,
      regressions,
      no_change,
      average_change,
    }
  }
  
  /// Generate a comprehensive diff report
  pub fn to_report(&self) -> String
  {
    let mut output = String::new();
    
    // Header
    output.push_str("# Benchmark Diff Report\n\n");
    
    if let (Some(baseline), Some(current)) = (&self.baseline_timestamp, &self.current_timestamp)
    {
      output.push_str(&format!("**Baseline**: {}\n", baseline));
      output.push_str(&format!("**Current**: {}\n\n", current));
    }
    
    // Summary statistics
    output.push_str("## Summary\n\n");
    output.push_str(&format!("- **Total benchmarks**: {}\n", self.summary_stats.total_benchmarks));
    output.push_str(&format!("- **Improvements**: {} 📈\n", self.summary_stats.improvements));
    output.push_str(&format!("- **Regressions**: {} 📉\n", self.summary_stats.regressions));
    output.push_str(&format!("- **No change**: {} 🔄\n", self.summary_stats.no_change));
    output.push_str(&format!("- **Average change**: {:.1}%\n\n", self.summary_stats.average_change));
    
    // Individual diffs
    output.push_str("## Individual Results\n\n");
    
    for diff in &self.diffs
    {
      output.push_str(&format!("{}\n", diff.to_summary()));
    }
    
    // Detailed analysis for significant changes
    let significant_changes: Vec<_> = self.diffs.iter()
      .filter(|d| d.is_significant())
      .collect();
    
    if !significant_changes.is_empty()
    {
      output.push_str("\n## Significant Changes\n\n");
      
      for diff in significant_changes
      {
        output.push_str(&format!("### {}\n\n", diff.benchmark_name));
        output.push_str(&format!("{}\n", diff.to_diff_format()));
        output.push_str("\n");
      }
    }
    
    output
  }
  
  /// Get only the regressions from this diff set
  pub fn regressions(&self) -> Vec<&BenchmarkDiff>
  {
    self.diffs.iter().filter(|d| d.is_regression()).collect()
  }
  
  /// Get only the improvements from this diff set
  pub fn improvements(&self) -> Vec<&BenchmarkDiff>
  {
    self.diffs.iter().filter(|d| d.is_improvement()).collect()
  }
  
  /// Get only the significant changes from this diff set
  pub fn significant_changes(&self) -> Vec<&BenchmarkDiff>
  {
    self.diffs.iter().filter(|d| d.is_significant()).collect()
  }
}

/// Compare two benchmark results and return a diff
pub fn diff_benchmark_results(
  name: &str,
  baseline: BenchmarkResult,
  current: BenchmarkResult,
) -> BenchmarkDiff
{
  BenchmarkDiff::new(name, baseline, current)
}

/// Compare multiple benchmark results and return a diff set
pub fn diff_benchmark_sets(
  baseline_results: &[(String, BenchmarkResult)],
  current_results: &[(String, BenchmarkResult)],
) -> BenchmarkDiffSet
{
  BenchmarkDiffSet::compare_results(baseline_results, current_results)
}

