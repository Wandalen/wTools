//! Environment-specific benchmark configuration
//!
//! This module provides environment-aware benchmark configuration that adapts
//! coefficient of variation (CV) requirements, sample counts, and performance
//! thresholds based on the execution environment.
//!
//! ## Environment Types
//!
//! - **Development**: Fast feedback with relaxed accuracy (CV < 15%)
//! - **CI/CD**: Reliable regression detection (CV < 10%)
//! - **Production**: Decision-grade analysis (CV < 5%)

/// Internal namespace.
mod private
{
  use std::time::Duration;

/// Environment-specific benchmark configuration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
  /// Coefficient of variation tolerance (0.05 = 5%)
  pub cv_tolerance: f64,
  /// Minimum number of samples required
  pub min_sample_size: usize,
  /// Maximum number of samples (for performance control)
  pub max_sample_size: usize,
  /// Regression detection threshold (0.05 = 5% change)
  pub regression_threshold: f64,
  /// Warmup iterations before measurement
  pub warmup_iterations: usize,
  /// Maximum time to spend on benchmarking
  pub max_benchmark_time: Duration,
  /// Environment name
  pub environment: BenchmarkEnvironment,
}

/// Supported benchmark environments
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BenchmarkEnvironment {
  /// Development environment: fast feedback, relaxed accuracy
  Development,
  /// Staging/CI environment: moderate accuracy for regression detection
  Staging,
  /// Production analysis: high accuracy for decision-making
  Production,
}

impl BenchmarkConfig {
  /// Create configuration based on environment variable
  pub fn from_environment() -> Self {
    let env_var = std::env::var("BENCHMARK_ENV")
      .unwrap_or_else(|_| "development".to_string())
      .to_lowercase();

    match env_var.as_str() {
      "production" | "prod" => Self::production(),
      "staging" | "stage" | "ci" | "cicd" => Self::staging(),
      _ => Self::development(),
    }
  }

  /// Development environment configuration
  ///
  /// Optimized for quick feedback cycles during active development.
  /// Accepts higher variation in exchange for faster execution.
  pub fn development() -> Self {
    Self {
      cv_tolerance: 0.15,           // 15% CV tolerance
      min_sample_size: 10,
      max_sample_size: 20,
      regression_threshold: 0.15,   // 15% change threshold
      warmup_iterations: 3,
      max_benchmark_time: Duration::from_secs(30),
      environment: BenchmarkEnvironment::Development,
    }
  }

  /// Staging/CI environment configuration
  ///
  /// Balanced configuration for continuous integration pipelines.
  /// Provides reliable regression detection without excessive runtime.
  pub fn staging() -> Self {
    Self {
      cv_tolerance: 0.10,           // 10% CV tolerance
      min_sample_size: 20,
      max_sample_size: 30,
      regression_threshold: 0.10,   // 10% change threshold
      warmup_iterations: 5,
      max_benchmark_time: Duration::from_secs(120),
      environment: BenchmarkEnvironment::Staging,
    }
  }

  /// Production analysis configuration
  ///
  /// High-accuracy configuration for decision-grade performance analysis.
  /// Uses statistical rigor appropriate for production optimization decisions.
  pub fn production() -> Self {
    Self {
      cv_tolerance: 0.05,           // 5% CV tolerance
      min_sample_size: 50,
      max_sample_size: 100,
      regression_threshold: 0.05,   // 5% change threshold  
      warmup_iterations: 10,
      max_benchmark_time: Duration::from_secs(600),
      environment: BenchmarkEnvironment::Production,
    }
  }

  /// Check if coefficient of variation meets environment requirements
  pub fn cv_meets_requirements(&self, cv: f64) -> bool {
    cv <= self.cv_tolerance
  }

  /// Check if performance change is significant for this environment
  pub fn is_significant_change(&self, change_ratio: f64) -> bool {
    change_ratio.abs() > self.regression_threshold
  }

  /// Get appropriate sample size based on initial CV estimate
  pub fn adaptive_sample_size(&self, initial_cv: f64) -> usize {
    if initial_cv <= self.cv_tolerance {
      self.min_sample_size
    } else if initial_cv > self.cv_tolerance * 2.0 {
      self.max_sample_size
    } else {
      // Scale sample size based on CV quality
      let scale_factor = initial_cv / self.cv_tolerance;
      let scaled_size = (self.min_sample_size as f64 * scale_factor).ceil() as usize;
      scaled_size.min(self.max_sample_size).max(self.min_sample_size)
    }
  }

  /// Get environment-appropriate measurement configuration for benchkit
  pub fn to_measurement_config(&self) -> crate::MeasurementConfigWrapper {
    MeasurementConfigWrapper {
      iterations: self.min_sample_size,
      warmup_iterations: self.warmup_iterations,
      max_time: self.max_benchmark_time,
      cv_tolerance: self.cv_tolerance,
      regression_threshold: self.regression_threshold,
    }
  }
}

/// Wrapper for benchkit MeasurementConfig with environment-specific extensions
#[derive(Debug, Clone)]
pub struct MeasurementConfigWrapper {
  /// Number of measurement iterations to perform
  pub iterations: usize,
  /// Number of warmup iterations before measurement
  pub warmup_iterations: usize,
  /// Maximum time to spend on benchmarking
  pub max_time: Duration,
  /// Coefficient of variation tolerance threshold
  pub cv_tolerance: f64,
  /// Regression detection threshold for significant changes
  pub regression_threshold: f64,
}

#[cfg(feature = "benchkit")]
impl From<MeasurementConfigWrapper> for benchkit::measurement::MeasurementConfig {
  fn from(wrapper: MeasurementConfigWrapper) -> Self {
    Self {
      iterations: wrapper.iterations,
      warmup_iterations: wrapper.warmup_iterations,
      max_time: wrapper.max_time,
    }
  }
}

impl std::fmt::Display for BenchmarkEnvironment {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Development => write!(f, "Development"),
      Self::Staging => write!(f, "Staging/CI"),
      Self::Production => write!(f, "Production"),
    }
  }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
  use super::*;

  #[test]
  fn test_environment_detection_development() {
    std::env::set_var("BENCHMARK_ENV", "development");
    let config = BenchmarkConfig::from_environment();
    assert_eq!(config.environment, BenchmarkEnvironment::Development);
    assert_eq!(config.cv_tolerance, 0.15);
    assert_eq!(config.min_sample_size, 10);
    std::env::remove_var("BENCHMARK_ENV");
  }

  #[test]
  fn test_environment_detection_staging() {
    std::env::set_var("BENCHMARK_ENV", "staging");
    let config = BenchmarkConfig::from_environment();
    assert_eq!(config.environment, BenchmarkEnvironment::Staging);
    assert_eq!(config.cv_tolerance, 0.10);
    assert_eq!(config.min_sample_size, 20);
    std::env::remove_var("BENCHMARK_ENV");
  }

  #[test]
  fn test_environment_detection_production() {
    std::env::set_var("BENCHMARK_ENV", "production");
    let config = BenchmarkConfig::from_environment();
    assert_eq!(config.environment, BenchmarkEnvironment::Production);
    assert_eq!(config.cv_tolerance, 0.05);
    assert_eq!(config.min_sample_size, 50);
    std::env::remove_var("BENCHMARK_ENV");
  }

  #[test]
  fn test_cv_requirements() {
    let dev_config = BenchmarkConfig::development();
    assert!(dev_config.cv_meets_requirements(0.10));  // 10% < 15%
    assert!(!dev_config.cv_meets_requirements(0.20)); // 20% > 15%

    let prod_config = BenchmarkConfig::production();
    assert!(prod_config.cv_meets_requirements(0.03));  // 3% < 5%
    assert!(!prod_config.cv_meets_requirements(0.08)); // 8% > 5%
  }

  #[test]
  fn test_significance_threshold() {
    let config = BenchmarkConfig::staging();
    assert!(config.is_significant_change(0.12));   // 12% > 10%
    assert!(config.is_significant_change(-0.15));  // -15% > 10%
    assert!(!config.is_significant_change(0.05));  // 5% < 10%
  }

  #[test]
  fn test_adaptive_sample_size() {
    let config = BenchmarkConfig::staging();
    
    // Low CV - use minimum samples
    assert_eq!(config.adaptive_sample_size(0.05), 20);
    
    // High CV - use maximum samples
    assert_eq!(config.adaptive_sample_size(0.25), 30);
    
    // Moderate CV - scale appropriately
    let moderate_size = config.adaptive_sample_size(0.15);
    assert!(moderate_size > 20 && moderate_size <= 30);
  }

  #[test]
  fn test_default_environment() {
    // Clear environment variable
    std::env::remove_var("BENCHMARK_ENV");
    let config = BenchmarkConfig::from_environment();
    assert_eq!(config.environment, BenchmarkEnvironment::Development);
  }
  }
}

mod_interface::mod_interface!
{
  exposed use private::BenchmarkConfig;
  exposed use private::BenchmarkEnvironment;
  exposed use private::MeasurementConfigWrapper;
}