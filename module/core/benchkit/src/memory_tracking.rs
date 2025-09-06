//! Memory allocation tracking and analysis for benchmarks
//!
//! This module provides tools for tracking memory allocations during benchmark
//! execution, analyzing allocation patterns, and comparing memory efficiency
//! across different implementations.

use crate::measurement::BenchmarkResult;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

/// Global allocation tracker for memory analysis
#[derive(Debug)]
pub struct AllocationTracker
{
  /// Total number of allocations
  allocation_count: AtomicUsize,
  /// Total bytes allocated
  total_allocated: AtomicUsize,
  /// Peak memory usage
  peak_usage: AtomicUsize,
  /// Current memory usage
  current_usage: AtomicUsize,
}

impl Default for AllocationTracker
{
  fn default() -> Self
  {
    Self
    {
      allocation_count: AtomicUsize::new(0),
      total_allocated: AtomicUsize::new(0),
      peak_usage: AtomicUsize::new(0),
      current_usage: AtomicUsize::new(0),
    }
  }
}

impl AllocationTracker
{
  /// Create a new allocation tracker
  pub fn new() -> Self
  {
    Self::default()
  }

  /// Record an allocation
  pub fn record_allocation(&self, size: usize)
  {
    self.allocation_count.fetch_add(1, Ordering::Relaxed);
    self.total_allocated.fetch_add(size, Ordering::Relaxed);
    
    let new_usage = self.current_usage.fetch_add(size, Ordering::Relaxed) + size;
    
    // Update peak usage if necessary
    let mut current_peak = self.peak_usage.load(Ordering::Relaxed);
    loop 
    {
      if new_usage <= current_peak 
      {
        break;
      }
      
      match self.peak_usage.compare_exchange_weak(
        current_peak, 
        new_usage, 
        Ordering::Relaxed, 
        Ordering::Relaxed
      ) 
      {
        Ok(_) => break,
        Err(actual) => current_peak = actual,
      }
    }
  }

  /// Record a deallocation
  pub fn record_deallocation(&self, size: usize)
  {
    self.current_usage.fetch_sub(size, Ordering::Relaxed);
  }

  /// Get current allocation statistics
  pub fn get_stats(&self) -> AllocationStats
  {
    AllocationStats
    {
      allocation_count: self.allocation_count.load(Ordering::Relaxed),
      total_allocated: self.total_allocated.load(Ordering::Relaxed),
      peak_usage: self.peak_usage.load(Ordering::Relaxed),
      current_usage: self.current_usage.load(Ordering::Relaxed),
    }
  }

  /// Reset all counters
  pub fn reset(&self)
  {
    self.allocation_count.store(0, Ordering::Relaxed);
    self.total_allocated.store(0, Ordering::Relaxed);
    self.peak_usage.store(0, Ordering::Relaxed);
    self.current_usage.store(0, Ordering::Relaxed);
  }

  /// Take snapshot of current stats and reset
  pub fn snapshot_and_reset(&self) -> AllocationStats
  {
    let stats = self.get_stats();
    self.reset();
    stats
  }
}

/// Memory allocation statistics snapshot
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AllocationStats
{
  /// Total number of allocations performed
  pub allocation_count: usize,
  /// Total bytes allocated across all allocations
  pub total_allocated: usize,
  /// Peak memory usage reached
  pub peak_usage: usize,
  /// Current memory usage
  pub current_usage: usize,
}

impl AllocationStats
{
  /// Calculate average allocation size
  pub fn average_allocation_size(&self) -> f64
  {
    if self.allocation_count > 0 
    {
      self.total_allocated as f64 / self.allocation_count as f64
    } 
    else 
    {
      0.0
    }
  }

  /// Calculate memory efficiency (peak/total ratio)
  pub fn memory_efficiency(&self) -> f64
  {
    if self.total_allocated > 0 
    {
      self.peak_usage as f64 / self.total_allocated as f64
    } 
    else 
    {
      0.0
    }
  }

  /// Get human-readable description
  pub fn description(&self) -> String
  {
    format!(
      "Allocs: {}, Total: {}, Peak: {}, Avg: {:.1} bytes/alloc, Efficiency: {:.1}%",
      self.allocation_count,
      format_bytes(self.total_allocated),
      format_bytes(self.peak_usage), 
      self.average_allocation_size(),
      self.memory_efficiency() * 100.0
    )
  }
}

/// Memory-aware benchmark runner that tracks allocations
#[derive(Debug)]
pub struct MemoryBenchmark
{
  /// Name of the benchmark
  pub name: String,
  /// Allocation tracker for this benchmark
  pub tracker: AllocationTracker,
}

impl MemoryBenchmark
{
  /// Create a new memory-aware benchmark
  pub fn new(name: impl Into<String>) -> Self
  {
    Self
    {
      name: name.into(),
      tracker: AllocationTracker::new(),
    }
  }

  /// Run a function while tracking memory allocations
  pub fn run_with_tracking<F, R>(&self, iterations: usize, mut f: F) -> (BenchmarkResult, AllocationStats)
  where
    F: FnMut() -> R,
  {
    let mut durations = Vec::with_capacity(iterations);
    self.tracker.reset();
    
    for _ in 0..iterations 
    {
      let start = Instant::now();
      let _result = f();
      let duration = start.elapsed();
      durations.push(duration);
    }
    
    let benchmark_result = BenchmarkResult::new(&self.name, durations);
    let allocation_stats = self.tracker.get_stats();
    
    (benchmark_result, allocation_stats)
  }

  /// Compare memory usage of different implementations
  pub fn compare_memory_usage<F1, F2, R1, R2>(
    &self, 
    impl1_name: &str,
    mut impl1: F1,
    impl2_name: &str, 
    mut impl2: F2,
    iterations: usize,
  ) -> MemoryComparison
  where
    F1: FnMut() -> R1,
    F2: FnMut() -> R2,
  {
    // Run first implementation
    self.tracker.reset();
    let mut impl1_durations = Vec::with_capacity(iterations);
    for _ in 0..iterations 
    {
      let start = Instant::now();
      let _result = impl1();
      impl1_durations.push(start.elapsed());
    }
    let impl1_stats = self.tracker.snapshot_and_reset();
    
    // Run second implementation
    let mut impl2_durations = Vec::with_capacity(iterations);
    for _ in 0..iterations 
    {
      let start = Instant::now();
      let _result = impl2();
      impl2_durations.push(start.elapsed());
    }
    let impl2_stats = self.tracker.get_stats();
    
    let impl1_result = BenchmarkResult::new(impl1_name, impl1_durations);
    let impl2_result = BenchmarkResult::new(impl2_name, impl2_durations);
    
    MemoryComparison
    {
      benchmark_name: self.name.clone(),
      impl1_name: impl1_name.to_string(),
      impl1_result,
      impl1_stats,
      impl2_name: impl2_name.to_string(), 
      impl2_result,
      impl2_stats,
    }
  }
}

/// Comparison of memory usage between two implementations
#[derive(Debug, Clone)]
pub struct MemoryComparison
{
  /// Name of the benchmark
  pub benchmark_name: String,
  /// First implementation name
  pub impl1_name: String,
  /// First implementation benchmark results
  pub impl1_result: BenchmarkResult,
  /// First implementation allocation stats
  pub impl1_stats: AllocationStats,
  /// Second implementation name
  pub impl2_name: String,
  /// Second implementation benchmark results 
  pub impl2_result: BenchmarkResult,
  /// Second implementation allocation stats
  pub impl2_stats: AllocationStats,
}

impl MemoryComparison
{
  /// Get the more memory-efficient implementation
  pub fn more_memory_efficient(&self) -> (&str, &AllocationStats)
  {
    if self.impl1_stats.peak_usage <= self.impl2_stats.peak_usage 
    {
      (&self.impl1_name, &self.impl1_stats)
    } 
    else 
    {
      (&self.impl2_name, &self.impl2_stats)
    }
  }

  /// Calculate memory usage reduction percentage
  pub fn memory_reduction_percentage(&self) -> f64
  {
    let (efficient_stats, other_stats) = if self.impl1_stats.peak_usage <= self.impl2_stats.peak_usage 
    {
      (&self.impl1_stats, &self.impl2_stats)
    } 
    else 
    {
      (&self.impl2_stats, &self.impl1_stats)
    };
    
    if other_stats.peak_usage > 0 
    {
      ((other_stats.peak_usage - efficient_stats.peak_usage) as f64 / other_stats.peak_usage as f64) * 100.0
    } 
    else 
    {
      0.0
    }
  }

  /// Generate comprehensive memory comparison report
  pub fn to_markdown(&self) -> String
  {
    let mut report = String::new();
    
    report.push_str(&format!("## {} Memory Usage Comparison\n\n", self.benchmark_name));
    
    // Executive summary
    let (efficient_name, _) = self.more_memory_efficient();
    let reduction = self.memory_reduction_percentage();
    
    report.push_str(&format!(
      "**Most memory efficient**: {} ({:.1}% less peak memory usage)\n\n",
      efficient_name, reduction
    ));
    
    // Detailed comparison table
    report.push_str("### Memory Usage Metrics\n\n");
    report.push_str("| Implementation | Peak Memory | Total Allocated | Allocations | Avg Size | Efficiency |\n");
    report.push_str("|----------------|-------------|-----------------|-------------|----------|------------|\n");
    
    report.push_str(&format!(
      "| {} | {} | {} | {} | {:.1} B | {:.1}% |\n",
      self.impl1_name,
      format_bytes(self.impl1_stats.peak_usage),
      format_bytes(self.impl1_stats.total_allocated),
      self.impl1_stats.allocation_count,
      self.impl1_stats.average_allocation_size(),
      self.impl1_stats.memory_efficiency() * 100.0
    ));
    
    report.push_str(&format!(
      "| {} | {} | {} | {} | {:.1} B | {:.1}% |\n",
      self.impl2_name,
      format_bytes(self.impl2_stats.peak_usage),
      format_bytes(self.impl2_stats.total_allocated), 
      self.impl2_stats.allocation_count,
      self.impl2_stats.average_allocation_size(),
      self.impl2_stats.memory_efficiency() * 100.0
    ));
    
    report.push('\n');
    
    // Performance vs memory trade-offs
    report.push_str("### Performance vs Memory Trade-offs\n\n");
    
    let impl1_ops = self.impl1_result.operations_per_second();
    let impl2_ops = self.impl2_result.operations_per_second();
    
    report.push_str(&format!(
      "- **{}**: {:.0} ops/sec, {} peak memory\n",
      self.impl1_name, impl1_ops, format_bytes(self.impl1_stats.peak_usage)
    ));
    
    report.push_str(&format!(
      "- **{}**: {:.0} ops/sec, {} peak memory\n",
      self.impl2_name, impl2_ops, format_bytes(self.impl2_stats.peak_usage)
    ));
    
    // Calculate memory efficiency per operation
    let impl1_memory_per_op = if impl1_ops > 0.0 { self.impl1_stats.peak_usage as f64 / impl1_ops } else { 0.0 };
    let impl2_memory_per_op = if impl2_ops > 0.0 { self.impl2_stats.peak_usage as f64 / impl2_ops } else { 0.0 };
    
    report.push_str(&format!(
      "\n**Memory efficiency per operation**:\n\
       - {}: {:.1} bytes/op\n\
       - {}: {:.1} bytes/op\n\n",
      self.impl1_name, impl1_memory_per_op,
      self.impl2_name, impl2_memory_per_op
    ));
    
    // Recommendations
    report.push_str("### Recommendations\n\n");
    
    if reduction > 20.0 
    {
      report.push_str(&format!(
        "- **Strong recommendation**: Use {} for significant {:.1}% memory reduction\n",
        efficient_name, reduction
      ));
    } 
    else if reduction > 5.0 
    {
      report.push_str(&format!(
        "- **Consider**: {} provides {:.1}% memory reduction\n", 
        efficient_name, reduction
      ));
    } 
    else 
    {
      report.push_str("- **Equivalent**: Both implementations have similar memory usage\n");
    }
    
    report.push('\n');
    report
  }
}

/// Format bytes in human-readable form
fn format_bytes(bytes: usize) -> String
{
  if bytes >= 1_073_741_824 // 1 GB
  {
    format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
  } 
  else if bytes >= 1_048_576 // 1 MB
  {
    format!("{:.1} MB", bytes as f64 / 1_048_576.0)
  } 
  else if bytes >= 1_024 // 1 KB
  {
    format!("{:.1} KB", bytes as f64 / 1_024.0)
  } 
  else 
  {
    format!("{} B", bytes)
  }
}

/// Memory profiler for analyzing allocation patterns
#[derive(Debug)]
pub struct MemoryProfiler
{
  /// Allocation events recorded
  events: Vec<AllocationEvent>,
  /// Start time for profiling session
  start_time: Instant,
}

/// Single allocation event
#[derive(Debug, Clone)]
struct AllocationEvent
{
  /// Time since profiling started
  _timestamp: Duration, // Keep for future timeline analysis features
  /// Event type
  event_type: AllocationEventType,
  /// Size of allocation/deallocation
  size: usize,
}

#[derive(Debug, Clone)]
enum AllocationEventType
{
  Allocation,
  Deallocation,
}

impl Default for MemoryProfiler
{
  fn default() -> Self
  {
    Self::new()
  }
}

impl MemoryProfiler
{
  /// Create a new memory profiler
  pub fn new() -> Self
  {
    Self
    {
      events: Vec::new(),
      start_time: Instant::now(),
    }
  }

  /// Record an allocation event
  pub fn record_allocation(&mut self, size: usize)
  {
    self.events.push(AllocationEvent
    {
      _timestamp: self.start_time.elapsed(),
      event_type: AllocationEventType::Allocation,
      size,
    });
  }

  /// Record a deallocation event
  pub fn record_deallocation(&mut self, size: usize)
  {
    self.events.push(AllocationEvent
    {
      _timestamp: self.start_time.elapsed(),
      event_type: AllocationEventType::Deallocation,
      size,
    });
  }

  /// Analyze allocation patterns
  pub fn analyze_patterns(&self) -> AllocationPatternAnalysis
  {
    let mut total_allocated = 0;
    let mut total_deallocated = 0;
    let mut peak_usage = 0;
    let mut current_usage = 0;
    let mut allocation_sizes = Vec::new();
    
    for event in &self.events 
    {
      match event.event_type 
      {
        AllocationEventType::Allocation => 
        {
          total_allocated += event.size;
          current_usage += event.size;
          allocation_sizes.push(event.size);
          
          if current_usage > peak_usage 
          {
            peak_usage = current_usage;
          }
        }
        AllocationEventType::Deallocation => 
        {
          total_deallocated += event.size;
          current_usage = current_usage.saturating_sub(event.size);
        }
      }
    }
    
    AllocationPatternAnalysis
    {
      total_events: self.events.len(),
      total_allocated,
      total_deallocated,
      peak_usage,
      final_usage: current_usage,
      allocation_sizes,
      duration: self.start_time.elapsed(),
    }
  }
}

/// Analysis of allocation patterns over time
#[derive(Debug)]
pub struct AllocationPatternAnalysis
{
  /// Total allocation events
  pub total_events: usize,
  /// Total bytes allocated
  pub total_allocated: usize,
  /// Total bytes deallocated  
  pub total_deallocated: usize,
  /// Peak memory usage
  pub peak_usage: usize,
  /// Final memory usage (potential leaks)
  pub final_usage: usize,
  /// All allocation sizes for distribution analysis
  pub allocation_sizes: Vec<usize>,
  /// Total duration of profiling session
  pub duration: Duration,
}

impl AllocationPatternAnalysis
{
  /// Check for potential memory leaks
  pub fn has_potential_leaks(&self) -> bool
  {
    self.final_usage > 0
  }

  /// Get allocation size statistics
  pub fn size_statistics(&self) -> Option<AllocationSizeStats>
  {
    if self.allocation_sizes.is_empty() 
    {
      return None;
    }
    
    let mut sizes = self.allocation_sizes.clone();
    sizes.sort_unstable();
    
    let min = sizes[0];
    let max = sizes[sizes.len() - 1];
    let median = sizes[sizes.len() / 2];
    let mean = self.allocation_sizes.iter().sum::<usize>() as f64 / self.allocation_sizes.len() as f64;
    
    Some(AllocationSizeStats
    {
      min,
      max,
      median,
      mean,
      total_allocations: self.allocation_sizes.len(),
    })
  }
}

/// Statistics about allocation sizes
#[derive(Debug, Clone)]
pub struct AllocationSizeStats
{
  /// Minimum allocation size
  pub min: usize,
  /// Maximum allocation size
  pub max: usize,
  /// Median allocation size
  pub median: usize,
  /// Mean allocation size
  pub mean: f64,
  /// Total number of allocations
  pub total_allocations: usize,
}

