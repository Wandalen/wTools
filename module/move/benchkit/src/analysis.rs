//! Analysis tools for benchmark results
//!
//! This module provides tools for analyzing benchmark results, including
//! comparative analysis, regression detection, and statistical analysis.

use crate::measurement::{ BenchmarkResult, Comparison };
use std::collections::HashMap;

/// Comparative analysis for multiple algorithm variants
pub struct ComparativeAnalysis {
  name: String,
  variants: HashMap<String, Box<dyn FnMut() + Send>>,
  results: HashMap<String, BenchmarkResult>,
}

impl ComparativeAnalysis {
  /// Create a new comparative analysis
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      variants: HashMap::new(), 
      results: HashMap::new(),
    }
  }

  /// Add an algorithm variant to compare
  pub fn add_variant<F>(mut self, name: impl Into<String>, f: F) -> Self 
  where
    F: FnMut() + Send + 'static,
  {
    self.variants.insert(name.into(), Box::new(f));
    self
  }

  /// Add an algorithm variant to compare (builder pattern alias)
  pub fn algorithm<F>(self, name: impl Into<String>, f: F) -> Self
  where  
    F: FnMut() + Send + 'static,
  {
    self.add_variant(name, f)
  }

  /// Run the comparative analysis
  pub fn run(self) -> ComparisonReport {
    let mut results = HashMap::new();
    
    for (name, mut variant) in self.variants {
      let result = crate::measurement::bench_function(&name, || variant());
      results.insert(name.clone(), result);
    }
    
    ComparisonReport {
      name: self.name,
      results,
    }
  }
}

/// Report containing results of comparative analysis
#[derive(Debug)]
pub struct ComparisonReport {
  pub name: String,
  pub results: HashMap<String, BenchmarkResult>,
}

impl ComparisonReport {
  /// Get the fastest result
  pub fn fastest(&self) -> Option<(&String, &BenchmarkResult)> {
    self.results
      .iter()
      .min_by(|a, b| a.1.mean_time().cmp(&b.1.mean_time()))
  }

  /// Get the slowest result
  pub fn slowest(&self) -> Option<(&String, &BenchmarkResult)> {
    self.results
      .iter()
      .max_by(|a, b| a.1.mean_time().cmp(&b.1.mean_time()))
  }

  /// Get all results sorted by performance (fastest first)
  pub fn sorted_by_performance(&self) -> Vec<(&String, &BenchmarkResult)> {
    let mut results: Vec<_> = self.results.iter().collect();
    results.sort_by(|a, b| a.1.mean_time().cmp(&b.1.mean_time()));
    results
  }

  /// Print a summary of the comparison
  pub fn print_summary(&self) {
    println!("=== {} Comparison ===", self.name);
    
    if let Some((fastest_name, fastest_result)) = self.fastest() {
      println!("🏆 Fastest: {} ({:.2?})", fastest_name, fastest_result.mean_time());
      
      // Show relative performance of all variants
      println!("\nRelative Performance:");
      for (name, result) in self.sorted_by_performance() {
        let _comparison = result.compare(fastest_result);
        let relative_speed = if name == fastest_name {
          "baseline".to_string()
        } else {
          format!("{:.1}x slower", 
                  result.mean_time().as_secs_f64() / fastest_result.mean_time().as_secs_f64())
        };
        
        println!("  {} - {:.2?} ({})", name, result.mean_time(), relative_speed);
      }
    }
    
    println!(); // Empty line for readability
  }

  /// Generate markdown summary
  pub fn to_markdown(&self) -> String {
    let mut output = String::new();
    output.push_str(&format!("## {} Comparison\n\n", self.name));
    
    if self.results.is_empty() {
      output.push_str("No results available.\n");
      return output;
    }
    
    // Results table
    output.push_str("| Algorithm | Mean Time | Operations/sec | Relative Performance |\n");
    output.push_str("|-----------|-----------|----------------|----------------------|\n");
    
    let fastest = self.fastest().map(|(_, result)| result);
    
    for (name, result) in self.sorted_by_performance() {
      let relative = if let Some(fastest_result) = fastest {
        if result.mean_time() == fastest_result.mean_time() {
          "**Fastest**".to_string()
        } else {
          format!("{:.1}x slower", 
                  result.mean_time().as_secs_f64() / fastest_result.mean_time().as_secs_f64())
        }
      } else {
        "N/A".to_string()
      };
      
      output.push_str(&format!("| {} | {:.2?} | {:.0} | {} |\n",
                               name,
                               result.mean_time(),
                               result.operations_per_second(),
                               relative));
    }
    
    output.push('\n');
    
    // Key insights
    if let (Some((fastest_name, _)), Some((slowest_name, slowest_result))) = 
      (self.fastest(), self.slowest()) {
      output.push_str("### Key Insights\n\n");
      output.push_str(&format!("- **Best performing**: {} algorithm\n", fastest_name));
      if fastest_name != slowest_name {
        let fastest = self.fastest().unwrap().1;
        let speedup = slowest_result.mean_time().as_secs_f64() / fastest.mean_time().as_secs_f64();
        output.push_str(&format!("- **Performance range**: {:.1}x difference between fastest and slowest\n", speedup));
      }
    }
    
    output
  }
}

/// Performance regression analysis
#[derive(Debug, Clone)]
pub struct RegressionAnalysis {
  pub baseline_results: HashMap<String, BenchmarkResult>,
  pub current_results: HashMap<String, BenchmarkResult>,
}

impl RegressionAnalysis {
  /// Create new regression analysis from baseline and current results
  pub fn new(
    baseline: HashMap<String, BenchmarkResult>,
    current: HashMap<String, BenchmarkResult>
  ) -> Self {
    Self {
      baseline_results: baseline,
      current_results: current,
    }
  }

  /// Detect regressions (performance degradations > threshold)
  pub fn detect_regressions(&self, threshold_percent: f64) -> Vec<Comparison> {
    let mut regressions = Vec::new();
    
    for (name, current) in &self.current_results {
      if let Some(baseline) = self.baseline_results.get(name) {
        let comparison = current.compare(baseline);
        if comparison.improvement_percentage < -threshold_percent {
          regressions.push(comparison);
        }
      }
    }
    
    regressions
  }

  /// Detect improvements (performance gains > threshold)  
  pub fn detect_improvements(&self, threshold_percent: f64) -> Vec<Comparison> {
    let mut improvements = Vec::new();
    
    for (name, current) in &self.current_results {
      if let Some(baseline) = self.baseline_results.get(name) {
        let comparison = current.compare(baseline);
        if comparison.improvement_percentage > threshold_percent {
          improvements.push(comparison);
        }
      }
    }
    
    improvements
  }

  /// Get overall regression percentage (worst case)
  pub fn worst_regression_percentage(&self) -> f64 {
    self.detect_regressions(0.0)
      .iter()
      .map(|c| c.improvement_percentage.abs())
      .fold(0.0, f64::max)
  }

  /// Generate regression report
  pub fn generate_report(&self) -> String {
    let mut report = String::new();
    report.push_str("# Performance Regression Analysis\n\n");
    
    let regressions = self.detect_regressions(5.0);
    let improvements = self.detect_improvements(5.0);
    
    if !regressions.is_empty() {
      report.push_str("## 🚨 Performance Regressions\n\n");
      for regression in &regressions {
        report.push_str(&format!("- **{}**: {:.1}% slower ({:.2?} -> {:.2?})\n",
                                 regression.current.name,
                                 regression.improvement_percentage.abs(),
                                 regression.baseline.mean_time(),
                                 regression.current.mean_time()));
      }
      report.push('\n');
    }
    
    if !improvements.is_empty() {
      report.push_str("## 🎉 Performance Improvements\n\n");
      for improvement in &improvements {
        report.push_str(&format!("- **{}**: {:.1}% faster ({:.2?} -> {:.2?})\n",
                                 improvement.current.name,
                                 improvement.improvement_percentage,
                                 improvement.baseline.mean_time(),
                                 improvement.current.mean_time()));
      }
      report.push('\n');
    }
    
    if regressions.is_empty() && improvements.is_empty() {
      report.push_str("## ✅ No Significant Changes\n\n");
      report.push_str("Performance appears stable compared to baseline.\n\n");
    }
    
    report
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::measurement::bench_once;
  use std::thread;
  use std::time::Duration;

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
}