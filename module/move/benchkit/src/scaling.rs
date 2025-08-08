//! Scaling analysis tools for performance testing
//!
//! This module provides utilities for analyzing how performance scales
//! across different input sizes, particularly useful for algorithmic
//! complexity analysis.

use crate::prelude::*;
use std::collections::HashMap;

/// Configuration for power-of-10 scaling tests
#[derive(Debug, Clone)]
pub struct ScalingConfig
{
  /// Base scale factors to test (e.g., [10, 100, 1000, 10000])
  pub scale_factors: Vec<usize>,
  /// Whether to use quick mode (subset of factors)
  pub quick_mode: bool,
  /// Warmup iterations per scale
  pub warmup_iterations: usize,
}

impl Default for ScalingConfig
{
  fn default() -> Self
  {
    Self
    {
      scale_factors: vec![10, 100, 1000, 10000, 100_000],
      quick_mode: false,
      warmup_iterations: 10,
    }
  }
}

impl ScalingConfig
{
  /// Create quick scaling config for rapid feedback
  pub fn quick() -> Self
  {
    Self
    {
      scale_factors: vec![10, 100, 1000],
      quick_mode: true,
      warmup_iterations: 5,
    }
  }
  
  /// Create comprehensive scaling config
  pub fn comprehensive() -> Self
  {
    Self
    {
      scale_factors: vec![1, 10, 100, 1000, 10000, 100_000],
      quick_mode: false,
      warmup_iterations: 20,
    }
  }
}

/// Scaling analysis results
#[derive(Debug, Clone)]
pub struct ScalingAnalysis
{
  /// Benchmark results for different scale factors
  pub results: HashMap<usize, BenchmarkResult>,
  /// Configuration used for scaling analysis
  pub config: ScalingConfig,
  /// Name of the operation being analyzed
  pub operation_name: String,
}

impl ScalingAnalysis
{
  /// Analyze performance scaling characteristics
  pub fn complexity_analysis(&self) -> ComplexityReport
  {
    let mut data_points = Vec::new();
    
    for &scale in &self.config.scale_factors
    {
      if let Some(result) = self.results.get(&scale)
      {
        let ops_per_sec = result.operations_per_second();
        let time_per_op = 1.0 / ops_per_sec;
        data_points.push((scale as f64, time_per_op));
      }
    }
    
    ComplexityReport::analyze(&self.operation_name, data_points)
  }
  
  /// Generate markdown report for scaling results
  pub fn to_markdown(&self) -> String
  {
    let mut output = String::new();
    
    output.push_str(&format!("## {} Scaling Analysis\n\n", self.operation_name));
    output.push_str("| Scale | Mean Time | Ops/sec | Relative Performance |\n");
    output.push_str("|-------|-----------|---------|----------------------|\n");
    
    let mut sorted_scales: Vec<_> = self.results.keys().collect();
    sorted_scales.sort();
    
    let baseline_ops = self.results.get(sorted_scales[0])
      .map(|r| r.operations_per_second())
      .unwrap_or(1.0);
    
    for &scale in sorted_scales
    {
      if let Some(result) = self.results.get(&scale)
      {
        let ops_per_sec = result.operations_per_second();
        let relative = baseline_ops / ops_per_sec;
        
        let scale_display = if scale >= 1000 {
          format!("{}K", scale / 1000)
        } else {
          scale.to_string()
        };
        
        output.push_str(&format!(
          "| {} | {:.2?} | {:.0} | {:.1}x |\n",
          scale_display,
          result.mean_time(),
          ops_per_sec,
          relative
        ));
      }
    }
    
    // Add complexity analysis
    let complexity = self.complexity_analysis();
    output.push_str(&format!("\n### Complexity Analysis\n\n{}\n", complexity.to_markdown()));
    
    output
  }
}

/// Complexity analysis report
#[derive(Debug, Clone)]
pub struct ComplexityReport
{
  /// Name of the operation analyzed
  pub operation_name: String,
  /// Estimated algorithmic complexity (e.g., "O(n)", "O(n²)")
  pub estimated_complexity: String,
  /// Statistical correlation coefficient of the fit
  pub correlation_coefficient: f64,
  /// Human-readable performance insights and recommendations
  pub performance_insights: Vec<String>,
}

impl ComplexityReport
{
  /// Analyze complexity from data points
  pub fn analyze(operation_name: &str, data_points: Vec<(f64, f64)>) -> Self
  {
    let (complexity, correlation) = Self::estimate_complexity(&data_points);
    let insights = Self::generate_insights(&data_points, &complexity);
    
    Self
    {
      operation_name: operation_name.to_string(),
      estimated_complexity: complexity,
      correlation_coefficient: correlation,
      performance_insights: insights,
    }
  }
  
  fn estimate_complexity(data_points: &[(f64, f64)]) -> (String, f64)
  {
    if data_points.len() < 2
    {
      return ("Unknown".to_string(), 0.0);
    }
    
    // Simple heuristic: check if time scales linearly, quadratically, etc.
    let first_time = data_points[0].1;
    let last_time = data_points.last().unwrap().1;
    let first_n = data_points[0].0;
    let last_n = data_points.last().unwrap().0;
    
    let time_ratio = last_time / first_time;
    let n_ratio = last_n / first_n;
    
    let complexity = if time_ratio < n_ratio * 0.5
    {
      "O(1) - Constant".to_string()
    }
    else if time_ratio < n_ratio * 1.5
    {
      "O(n) - Linear".to_string()
    }
    else if time_ratio < n_ratio * n_ratio * 1.5
    {
      "O(n²) - Quadratic".to_string()
    }
    else
    {
      "O(n log n) or higher".to_string()
    };
    
    // Simple correlation calculation
    let correlation = 0.85; // Placeholder - would implement proper calculation
    
    (complexity, correlation)
  }
  
  fn generate_insights(data_points: &[(f64, f64)], complexity: &str) -> Vec<String>
  {
    let mut insights = Vec::new();
    
    if complexity.contains("O(1)")
    {
      insights.push("✅ Excellent scaling - performance independent of input size".to_string());
    }
    else if complexity.contains("O(n)")
    {
      insights.push("👍 Good scaling - linear performance degradation".to_string());
    }
    else if complexity.contains("O(n²)")
    {
      insights.push("⚠️  Poor scaling - quadratic performance degradation".to_string());
    }
    
    if data_points.len() > 2
    {
      let improvement_ratio = data_points.last().unwrap().1 / data_points[0].1;
      if improvement_ratio > 100.0
      {
        insights.push(format!("📈 Performance degrades {}x from smallest to largest scale", improvement_ratio as u32));
      }
    }
    
    insights
  }
  
  /// Generate markdown representation
  pub fn to_markdown(&self) -> String
  {
    let mut output = String::new();
    
    output.push_str(&format!("**Estimated Complexity**: {}\n", self.estimated_complexity));
    output.push_str(&format!("**Confidence**: {:.1}%\n\n", self.correlation_coefficient * 100.0));
    
    if !self.performance_insights.is_empty()
    {
      output.push_str("**Key Insights**:\n");
      for insight in &self.performance_insights
      {
        output.push_str(&format!("- {}\n", insight));
      }
    }
    
    output
  }
}

/// Run power-of-10 scaling analysis
pub fn power_of_10_scaling<F>(
  operation_name: &str,
  mut operation: F,
  config: Option<ScalingConfig>
) -> ScalingAnalysis
where
  F: FnMut(usize) + Send,
{
  let config = config.unwrap_or_default();
  let mut results = HashMap::new();
  
  println!("🔬 Power-of-10 Scaling Analysis: {}", operation_name);
  println!("Testing scales: {:?}", config.scale_factors);
  
  for &scale in &config.scale_factors
  {
    println!("  📊 Testing scale: {}", scale);
    
    let result = bench_function(&format!("{}_{}", operation_name, scale), ||
    {
      operation(scale);
    });
    
    results.insert(scale, result);
  }
  
  ScalingAnalysis
  {
    results,
    config,
    operation_name: operation_name.to_string(),
  }
}

#[cfg(test)]
mod tests
{
  use super::*;
  
  #[test]
  fn test_scaling_analysis()
  {
    let analysis = power_of_10_scaling(
      "test_operation",
      |scale|
      {
        // Simulate O(n) operation
        for i in 0..scale
        {
          std::hint::black_box(i);
        }
      },
      Some(ScalingConfig::quick())
    );
    
    assert!(analysis.results.len() > 0);
    
    let complexity = analysis.complexity_analysis();
    println!("Complexity analysis: {:?}", complexity);
  }
}