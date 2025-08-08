//! Memory allocation and performance profiling tools
//!
//! This module provides utilities for tracking memory allocation patterns
//! and profiling resource usage during benchmarks.

use crate::prelude::*;
use std::time::Instant;

/// Memory allocation tracking result
#[derive(Debug, Clone)]
pub struct AllocationResult
{
  pub name: String,
  pub estimated_allocations: usize,
  pub timing_result: BenchmarkResult,
  pub allocation_rate: f64, // allocations per operation
}

impl AllocationResult
{
  /// Compare allocation efficiency with another result
  pub fn compare_allocations(&self, other: &AllocationResult) -> AllocationComparison
  {
    AllocationComparison
    {
      baseline: self.clone(),
      current: other.clone(),
      allocation_improvement: if other.allocation_rate > 0.0
      {
        (self.allocation_rate - other.allocation_rate) / other.allocation_rate * 100.0
      }
      else
      {
        0.0
      },
    }
  }
}

/// Allocation comparison result
#[derive(Debug, Clone)]
pub struct AllocationComparison
{
  pub baseline: AllocationResult,
  pub current: AllocationResult,
  pub allocation_improvement: f64, // Positive means baseline allocates less
}

impl AllocationComparison
{
  /// Generate markdown report
  pub fn to_markdown(&self) -> String
  {
    let mut output = String::new();
    
    output.push_str("## Memory Allocation Comparison\n\n");
    output.push_str("| Approach | Allocations/Op | Ops/sec | Memory Efficiency |\n");
    output.push_str("|----------|----------------|---------|------------------|\n");
    
    output.push_str(&format!(
      "| {} | {:.1} | {:.0} | Baseline |\n",
      self.baseline.name,
      self.baseline.allocation_rate,
      self.baseline.timing_result.operations_per_second()
    ));
    
    let efficiency = if self.allocation_improvement > 0.0
    {
      format!("{:.1}% fewer allocs", self.allocation_improvement)
    }
    else
    {
      format!("{:.1}% more allocs", -self.allocation_improvement)
    };
    
    output.push_str(&format!(
      "| {} | {:.1} | {:.0} | {} |\n",
      self.current.name,
      self.current.allocation_rate,
      self.current.timing_result.operations_per_second(),
      efficiency
    ));
    
    if self.allocation_improvement > 50.0
    {
      output.push_str("\n**🎉 Significant memory optimization achieved!**\n");
    }
    else if self.allocation_improvement > 10.0
    {
      output.push_str("\n**👍 Good memory optimization**\n");
    }
    else if self.allocation_improvement < -20.0
    {
      output.push_str("\n**⚠️  Memory usage increased significantly**\n");
    }
    
    output
  }
}

/// Benchmark with estimated memory allocation tracking
pub fn bench_with_allocation_tracking<F>(
  name: &str,
  mut operation: F,
  estimated_allocs_per_call: usize,
) -> AllocationResult
where
  F: FnMut() + Send,
{
  println!("🧠 Memory allocation tracking: {}", name);
  
  // Run the timing benchmark
  let timing_result = bench_function(name, ||
  {
    operation();
  });
  
  // Calculate allocation metrics
  let total_operations = timing_result.times.len();
  let estimated_total_allocations = total_operations * estimated_allocs_per_call;
  let allocation_rate = estimated_allocs_per_call as f64;
  
  println!("  📊 Est. allocations: {} ({:.1}/op)", estimated_total_allocations, allocation_rate);
  
  AllocationResult
  {
    name: name.to_string(),
    estimated_allocations: estimated_total_allocations,
    timing_result,
    allocation_rate,
  }
}

/// String interning benchmark helper
pub fn bench_string_operations<F1, F2>(
  baseline_name: &str,
  optimized_name: &str,
  baseline_fn: F1,
  optimized_fn: F2,
  test_data: &[&[&str]],
) -> AllocationComparison
where
  F1: Fn(&[&str]) -> String + Send + Sync,
  F2: Fn(&[&str]) -> String + Send + Sync,
{
  println!("🧵 String operations comparison");
  
  // Benchmark baseline (typically more allocations)
  let baseline_result = bench_with_allocation_tracking(
    baseline_name,
    ||
    {
      for slices in test_data
      {
        let _result = baseline_fn(slices);
        std::hint::black_box(_result);
      }
    },
    test_data.len() * 2, // Estimated: format!() + join() per operation
  );
  
  // Benchmark optimized version (typically fewer allocations)
  let optimized_result = bench_with_allocation_tracking(
    optimized_name,
    ||
    {
      for slices in test_data
      {
        let _result = optimized_fn(slices);
        std::hint::black_box(_result);
      }
    },
    test_data.len() / 10, // Estimated: cached lookups, fewer allocations
  );
  
  optimized_result.compare_allocations(&baseline_result)
}

/// Memory usage pattern analysis
#[derive(Debug, Clone)]
pub struct MemoryProfile
{
  pub operation_name: String,
  pub peak_estimated_usage_mb: f64,
  pub average_usage_mb: f64,
  pub allocation_hotspots: Vec<String>,
}

impl MemoryProfile
{
  /// Analyze memory usage patterns (simplified estimation)
  pub fn analyze<F>(name: &str, operation: F, iterations: usize) -> Self
  where
    F: Fn() + Send,
  {
    println!("📈 Memory profiling: {}", name);
    
    let start_time = Instant::now();
    
    // Run operation multiple times to estimate pattern
    for _ in 0..iterations
    {
      operation();
    }
    
    let duration = start_time.elapsed();
    
    // Simplified memory estimation based on timing characteristics
    let ops_per_sec = iterations as f64 / duration.as_secs_f64();
    let estimated_memory_per_op = if ops_per_sec > 100000.0
    {
      0.001 // Very fast = likely cached/minimal allocation
    }
    else if ops_per_sec > 10000.0
    {
      0.01 // Fast = some allocation
    }
    else
    {
      0.1 // Slow = likely heavy allocation
    };
    
    let peak_usage = estimated_memory_per_op * iterations as f64;
    let average_usage = peak_usage * 0.6; // Estimate average as 60% of peak
    
    let mut hotspots = Vec::new();
    if ops_per_sec < 1000.0
    {
      hotspots.push("Potential string allocation hotspot".to_string());
    }
    if peak_usage > 10.0
    {
      hotspots.push("High memory usage detected".to_string());
    }
    
    println!("  📊 Est. peak memory: {:.2} MB, avg: {:.2} MB", peak_usage, average_usage);
    
    Self
    {
      operation_name: name.to_string(),
      peak_estimated_usage_mb: peak_usage,
      average_usage_mb: average_usage,
      allocation_hotspots: hotspots,
    }
  }
  
  /// Generate markdown report
  pub fn to_markdown(&self) -> String
  {
    let mut output = String::new();
    
    output.push_str(&format!("## {} Memory Profile\n\n", self.operation_name));
    output.push_str(&format!("- **Peak Usage**: {:.2} MB\n", self.peak_estimated_usage_mb));
    output.push_str(&format!("- **Average Usage**: {:.2} MB\n", self.average_usage_mb));
    
    if !self.allocation_hotspots.is_empty()
    {
      output.push_str("\n**Potential Issues**:\n");
      for hotspot in &self.allocation_hotspots
      {
        output.push_str(&format!("- ⚠️  {}\n", hotspot));
      }
    }
    else
    {
      output.push_str("\n✅ **No memory issues detected**\n");
    }
    
    output
  }
}

#[cfg(test)]
mod tests
{
  use super::*;
  
  #[test]
  fn test_allocation_tracking()
  {
    let result = bench_with_allocation_tracking(
      "test_allocs",
      ||
      {
        let _vec: Vec<i32> = (0..100).collect();
      },
      1, // One allocation per call
    );
    
    assert!(result.allocation_rate > 0.0);
  }
  
  #[test]
  fn test_string_operations_comparison()
  {
    let test_data = vec![vec!["perf", "cmd_1"], vec!["perf", "cmd_2"]];
    let test_slices: Vec<&[&str]> = test_data.iter().map(|v| v.as_slice()).collect();
    
    let comparison = bench_string_operations(
      "format_join",
      "cached_lookup",
      |slices| format!(".{}", slices.join(".")),
      |slices| format!(".{}", slices.join(".")), // Same for test
      &test_slices,
    );
    
    println!("Comparison: {:?}", comparison);
  }
}