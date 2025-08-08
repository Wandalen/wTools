//! Core measurement and timing functionality
//!
//! This module provides the fundamental building blocks for timing operations
//! and collecting performance metrics. It focuses on accuracy and low overhead.

use std::time::{ Duration, Instant };
use std::fmt;

/// Result of a single benchmark measurement
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
  /// Individual timing measurements
  pub times: Vec<Duration>,
  /// Custom metrics collected during measurement
  pub metrics: std::collections::HashMap<String, f64>,
  /// Name of the benchmarked operation
  pub name: String,
}

impl BenchmarkResult {
  /// Create a new benchmark result
  pub fn new( name : impl Into< String >, times : Vec< Duration > ) -> Self
  {
    Self
    {
      name : name.into(),
      times,
      metrics : std::collections::HashMap::new(),
    }
  }

  /// Add a custom metric to the result
  pub fn with_metric( mut self, name : impl Into< String >, value : f64 ) -> Self
  {
    self.metrics.insert( name.into(), value );
    self
  }

  /// Get the mean execution time
  pub fn mean_time( &self ) -> Duration
  {
    if self.times.is_empty()
    {
      return Duration::ZERO;
    }
    let total : Duration = self.times.iter().sum();
    total / u32::try_from( self.times.len() ).unwrap_or( 1 )
  }

  /// Get the median execution time  
  pub fn median_time(&self) -> Duration {
    if self.times.is_empty() {
      return Duration::ZERO;
    }
    let mut sorted = self.times.clone();
    sorted.sort();
    sorted[sorted.len() / 2]
  }

  /// Get the minimum execution time
  pub fn min_time(&self) -> Duration {
    self.times.iter().min().copied().unwrap_or(Duration::ZERO)
  }

  /// Get the maximum execution time
  pub fn max_time(&self) -> Duration {
    self.times.iter().max().copied().unwrap_or(Duration::ZERO)
  }

  /// Calculate operations per second based on mean time
  pub fn operations_per_second(&self) -> f64 {
    let mean_secs = self.mean_time().as_secs_f64();
    if mean_secs > 0.0 {
      1.0 / mean_secs
    } else {
      0.0
    }
  }

  /// Get the standard deviation of timing measurements
  pub fn std_deviation(&self) -> Duration {
    if self.times.len() < 2 {
      return Duration::ZERO;
    }

    let mean = self.mean_time().as_secs_f64();
    let variance: f64 = self.times
      .iter()
      .map(|&time| {
        let diff = time.as_secs_f64() - mean;
        diff * diff
      })
      .sum::<f64>() / (self.times.len() - 1) as f64;

    Duration::from_secs_f64(variance.sqrt())
  }

  /// Get coefficient of variation (relative standard deviation) 
  pub fn coefficient_of_variation(&self) -> f64 {
    let mean_val = self.mean_time().as_secs_f64();
    if mean_val > 0.0 {
      self.std_deviation().as_secs_f64() / mean_val
    } else {
      0.0
    }
  }

  /// Get standard error of the mean
  pub fn standard_error(&self) -> Duration {
    if self.times.is_empty() {
      return Duration::ZERO;
    }
    let std_dev = self.std_deviation();
    Duration::from_secs_f64(std_dev.as_secs_f64() / (self.times.len() as f64).sqrt())
  }

  /// Get confidence interval for the mean (95% by default)
  pub fn confidence_interval_95(&self) -> (Duration, Duration) {
    let mean = self.mean_time();
    let margin = Duration::from_secs_f64(1.96 * self.standard_error().as_secs_f64());
    (mean.saturating_sub(margin), mean + margin)
  }

  /// Get percentile value (e.g., 0.95 for 95th percentile)
  pub fn percentile(&self, p: f64) -> Duration {
    if self.times.is_empty() {
      return Duration::ZERO;
    }
    let mut sorted = self.times.clone();
    sorted.sort();
    let index = (p * (sorted.len() - 1) as f64).round() as usize;
    sorted[index.min(sorted.len() - 1)]
  }

  /// Check if results are statistically reliable based on basic criteria
  pub fn is_reliable(&self) -> bool {
    // Basic reliability checks:
    // 1. Sufficient sample size (>= 10)
    // 2. Low coefficient of variation (<= 10%)
    // 3. Not too spread out (max/min ratio < 3.0)
    
    let sufficient_samples = self.times.len() >= 10;
    let low_variation = self.coefficient_of_variation() <= 0.1;
    let reasonable_spread = if self.min_time().as_secs_f64() > 0.0 {
      self.max_time().as_secs_f64() / self.min_time().as_secs_f64() < 3.0
    } else {
      false
    };

    sufficient_samples && low_variation && reasonable_spread
  }

  /// Compare this result with another, returning improvement percentage
  /// Positive percentage means this result is faster
  pub fn compare(&self, other: &BenchmarkResult) -> Comparison {
    let my_time = self.mean_time().as_secs_f64();
    let other_time = other.mean_time().as_secs_f64();
    
    let improvement = if other_time > 0.0 {
      ((other_time - my_time) / other_time) * 100.0
    } else {
      0.0
    };

    Comparison {
      baseline: other.clone(),
      current: self.clone(),
      improvement_percentage: improvement,
    }
  }
}

impl fmt::Display for BenchmarkResult {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}: {:.2?} (±{:.2?})", 
           self.name, 
           self.mean_time(), 
           self.std_deviation())
  }
}

/// Comparison between two benchmark results
#[derive(Debug, Clone)]
pub struct Comparison {
  /// The baseline benchmark result to compare against
  pub baseline: BenchmarkResult,
  /// The current benchmark result being compared
  pub current: BenchmarkResult,
  /// Improvement percentage (positive means current is faster than baseline)
  pub improvement_percentage: f64,
}

impl Comparison {
  /// Get the improvement percentage (positive means current is faster)
  pub fn improvement(&self) -> f64 {
    self.improvement_percentage
  }

  /// Check if current result shows significant improvement (>5%)
  pub fn is_improvement(&self) -> bool {
    self.improvement_percentage > 5.0
  }

  /// Check if current result shows significant regression (<-5%)
  pub fn is_regression(&self) -> bool {
    self.improvement_percentage < -5.0
  }
}

impl fmt::Display for Comparison {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let status = if self.is_improvement() {
      "IMPROVEMENT"
    } else if self.is_regression() {
      "REGRESSION"
    } else {
      "STABLE"
    };
    
    write!(f, "{}: {:.1}% {} ({:.2?} -> {:.2?})",
           status,
           self.improvement_percentage.abs(),
           if self.improvement_percentage >= 0.0 { "faster" } else { "slower" },
           self.baseline.mean_time(),
           self.current.mean_time())
  }
}

/// Measurement configuration
#[derive(Debug, Clone)]
pub struct MeasurementConfig {
  /// Number of iterations to run (default: 10)
  pub iterations: usize,
  /// Warm-up iterations before measurement (default: 3)
  pub warmup_iterations: usize,
  /// Maximum time to spend on measurement (default: 10 seconds)
  pub max_time: Duration,
}

impl Default for MeasurementConfig {
  fn default() -> Self {
    Self {
      iterations: 10,
      warmup_iterations: 3,
      max_time: Duration::from_secs(10),
    }
  }
}

/// Measure execution time of a function with default configuration
pub fn bench_function<F, R>(name: impl Into<String>, f: F) -> BenchmarkResult
where
  F: FnMut() -> R,
{
  bench_function_with_config(name, MeasurementConfig::default(), f)
}

/// Measure execution time of a function once (single iteration)
pub fn bench_once<F, R>(mut f: F) -> BenchmarkResult  
where
  F: FnMut() -> R,
{
  let start = Instant::now();
  let _ = f();
  let elapsed = start.elapsed();
  
  BenchmarkResult::new("single_measurement", vec![elapsed])
}

/// Measure execution time with custom configuration
pub fn bench_function_with_config<F, R>(
  name: impl Into<String>, 
  config: MeasurementConfig,
  mut f: F
) -> BenchmarkResult
where
  F: FnMut() -> R,
{
  let name = name.into();
  
  // Warmup iterations
  for _ in 0..config.warmup_iterations {
    let _ = f();
  }
  
  let mut times = Vec::with_capacity(config.iterations);
  let measurement_start = Instant::now();
  
  // Measurement iterations
  for _ in 0..config.iterations {
    // Check if we've exceeded maximum time
    if measurement_start.elapsed() > config.max_time {
      break;
    }
    
    let start = Instant::now();
    let _ = f();
    times.push(start.elapsed());
  }
  
  BenchmarkResult::new(name, times)
}

/// Measure a block of code (convenience macro)
#[macro_export]
macro_rules! bench_block {
  ($block:expr) => {
    bench_once(|| $block)
  };
  ($name:expr, $block:expr) => {
    bench_function($name, || $block)
  };
}

/// Time a block of code and return both result and timing
pub fn time_block<F, R>(f: F) -> (R, Duration)
where
  F: FnOnce() -> R,
{
  let start = Instant::now();
  let result = f();
  let elapsed = start.elapsed();
  (result, elapsed)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::thread;

  #[test]
  fn test_basic_measurement() {
    let result = bench_function("test_sleep", || {
      thread::sleep(Duration::from_millis(1));
    });
    
    assert!(result.mean_time() >= Duration::from_millis(1));
    assert!(!result.name.is_empty());
  }

  #[test] 
  fn test_comparison() {
    let fast = bench_once(|| {});
    let slow = bench_once(|| thread::sleep(Duration::from_millis(1)));
    
    let comparison = fast.compare(&slow);
    assert!(comparison.is_improvement());
  }

  #[test]
  fn test_bench_block_macro() {
    let result = bench_block!({
      let x = 42 + 42;
      std::hint::black_box( x );
    });
    
    assert!(result.times.len() == 1);
  }
}