//!
//! Performance metrics for command registry operations.
//!

/// Performance metrics for command registry operations.
///
/// **DESIGN RULE NOTICE:** This struct is for PRODUCTION performance tracking only.
///
/// ❌ **DO NOT** use this for performance testing in `tests/` directory:
/// ```no_run
/// // WRONG - This violates design rules
/// #[test]
/// fn test_performance() {
///     let start = std::time::Instant::now();
///     // ... operation
///     let metrics = registry.performance_metrics();
///     assert!(metrics.cache_hits > 0); // Performance assertion in test - VIOLATION
/// }
/// ```
///
/// ✅ **CORRECT** use for production monitoring:
/// ```ignore
/// // Production code monitoring
/// let metrics = registry.performance_metrics();
/// log::info!("Cache hit rate: {:.2}%", metrics.cache_hit_rate());
/// ```
///
/// **For performance testing, use `benchkit` framework separately.**
#[derive(Debug, Default, Clone)]
pub struct PerformanceMetrics
{
  /// Number of cache hits
  pub cache_hits: u64,
  /// Number of cache misses
  pub cache_misses: u64,
  /// Total number of lookups performed
  pub total_lookups: u64,
  /// Number of static command lookups
  pub static_lookups: u64,
  /// Number of dynamic command lookups
  pub dynamic_lookups: u64,
}

impl PerformanceMetrics
{
  /// Calculate cache hit rate as a value between 0.0 and 1.0
  pub fn cache_hit_rate( &self ) -> f64
  {
    if self.total_lookups == 0
    {
      0.0
    }
    else
    {
      self.cache_hits as f64 / self.total_lookups as f64
    }
  }

  /// Calculate ratio of static vs dynamic lookups
  pub fn static_ratio( &self ) -> f64
  {
    if self.total_lookups == 0
    {
      0.0
    }
    else
    {
      self.static_lookups as f64 / self.total_lookups as f64
    }
  }
}
