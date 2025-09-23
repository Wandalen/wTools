#![ allow( missing_docs ) ]
#![ allow( dead_code ) ]

//! Comprehensive tests for performance analysis tools
//!
//! Tests coefficient of variation analysis, comparative benchmarking, optimization workflow tracking,
//! statistical significance testing, and benchmark result quality assessment functionality.
//!
//! ## Test Matrix
//!
//! | Test Category | Test Name | Purpose | Dependencies |
//! |---------------|-----------|---------|--------------|
//! | CV Analysis | `test_cv_analyzer_*` | Verify coefficient of variation analysis | benchkit |
//! | CV Quality | `test_cv_quality_*` | Verify CV quality assessment and classification | None |
//! | CV Improvements | `test_cv_improvement_*` | Verify CV improvement techniques | None |
//! | Comparative | `test_comparative_benchmark_*` | Verify side-by-side performance comparison | serde |
//! | Optimization | `test_optimization_workflow_*` | Verify optimization tracking workflow | serde, tempfile |
//! | Statistical | `test_statistical_significance_*` | Verify statistical significance testing | None |
//! | Quality Assessment | `test_benchmark_quality_*` | Verify benchmark result quality assessment | None |
//! | Report Generation | `test_report_generation_*` | Verify analysis report generation | None |
//! | Integration | `test_*_integration` | Verify tool integration and workflows | benchkit, serde |
//! | Error Handling | `test_*_error_handling` | Verify error handling and edge cases | None |
//! | Performance | `test_large_dataset_*` | Verify handling of large benchmark datasets | None |
//! | Serialization | `test_*_serialization` | Verify data persistence and loading | serde |

use std::collections::HashMap;
use core::time::Duration;

// Test structures for comprehensive testing
#[ derive( Debug, Clone, PartialEq ) ]
pub struct MockBenchmarkResult
{
  pub times: Vec< Duration >,
  pub algorithm_name: String,
  pub data_size: usize,
}

impl MockBenchmarkResult
{
  #[must_use]
  pub fn new( algorithm_name: &str, times: Vec< Duration >, data_size: usize ) -> Self
  {
  Self
  {
  times,
  algorithm_name: algorithm_name.to_string(),
  data_size,
 }
 }

  #[must_use]
  pub fn coefficient_of_variation( &self ) -> f64
  {
  if self.times.is_empty()
  {
  return 0.0;
 }

  let mean = self.times.iter().map( |t| t.as_nanos() as f64 ).sum::< f64 >() / self.times.len() as f64;
  if mean == 0.0
  {
  return 0.0;
 }

  let variance = self.times.iter()
  .map( |t| ( t.as_nanos() as f64 - mean ).powi( 2 ) )
  .sum::< f64 >() / self.times.len() as f64;

  variance.sqrt() / mean
 }

  #[must_use]
  pub fn average_time( &self ) -> Duration
  {
  if self.times.is_empty()
  {
  return Duration::ZERO;
 }

  let total_nanos: u128 = self.times.iter().map( Duration::as_nanos ).sum();
  #[allow(clippy::cast_possible_truncation)]
  let result = u64::try_from( total_nanos / self.times.len() as u128 ).unwrap_or( u64::MAX );
  Duration::from_nanos( result )
 }
}

#[ derive( Debug, Clone, PartialEq ) ]
pub enum CvQuality
{
  Excellent,  // CV < 5%
  Good,       // CV 5-10%
  Moderate,   // CV 10-15%
  Poor,       // CV > 15%
}

impl CvQuality
{
  #[must_use]
  pub fn from_cv_percentage( cv_percent: f64 ) -> Self
  {
  if cv_percent < 5.0
  {
  Self::Excellent
 }
  else if cv_percent < 10.0
  {
  Self::Good
 }
  else if cv_percent < 15.0
  {
  Self::Moderate
 }
  else
  {
  Self::Poor
 }
 }

  #[must_use]
  pub fn indicator( &self ) -> &'static str
  {
  match self
  {
  Self::Excellent => "✅",
  Self::Good => "🟢",
  Self::Moderate => "🟡",
  Self::Poor => "❌",
 }
 }

  #[must_use]
  pub fn description( &self ) -> &'static str
  {
  match self
  {
  Self::Excellent => "Excellent reliability (ready for production decisions)",
  Self::Good => "Good, acceptable for most use cases",
  Self::Moderate => "Moderate, consider improvements",
  Self::Poor => "Poor/Unreliable, must fix before using results",
 }
 }
}

#[ derive( Debug, Clone ) ]
pub struct CvAnalyzer
{
  cv_tolerance: f64,
  environment: String,
}

impl Default for CvAnalyzer
{
  fn default() -> Self
  {
  Self::new()
 }
}

impl CvAnalyzer
{
  #[must_use]
  pub fn new() -> Self
  {
  Self
  {
  cv_tolerance: 0.15, // Default development tolerance
  environment: "Development".to_string(),
 }
 }

  #[must_use]
  pub fn with_config( cv_tolerance: f64, environment: &str ) -> Self
  {
  Self
  {
  cv_tolerance,
  environment: environment.to_string(),
 }
 }

  #[must_use]
  pub fn analyze_result( &self, name: &str, result: &MockBenchmarkResult ) -> CvAnalysisReport
  {
  let cv_percent = result.coefficient_of_variation() * 100.0;
  let quality = CvQuality::from_cv_percentage( cv_percent );
  let meets_requirements = result.coefficient_of_variation() <= self.cv_tolerance;

  CvAnalysisReport
  {
  benchmark_name: name.to_string(),
  cv_percentage: cv_percent,
  quality,
  meets_environment_requirements: meets_requirements,
  environment: self.environment.clone(),
  cv_tolerance: self.cv_tolerance,
  current_sample_size: result.times.len(),
  recommended_sample_size: self.calculate_recommended_size( result.coefficient_of_variation() ),
 }
 }

  fn calculate_recommended_size( &self, cv: f64 ) -> usize
  {
  if cv <= self.cv_tolerance
  {
  20 // Minimum for good CV
 }
  else if cv > self.cv_tolerance * 2.0
  {
  100 // Maximum for poor CV
 }
  else
  {
  // Scale based on CV quality
  let scale_factor = cv / self.cv_tolerance;
  #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
  let result = ( 20.0 * scale_factor ).ceil() as usize;
  result
 }
 }
}

#[ derive( Debug, Clone ) ]
pub struct CvAnalysisReport
{
  pub benchmark_name: String,
  pub cv_percentage: f64,
  pub quality: CvQuality,
  pub meets_environment_requirements: bool,
  pub environment: String,
  pub cv_tolerance: f64,
  pub current_sample_size: usize,
  pub recommended_sample_size: usize,
}

#[ allow( missing_debug_implementations ) ]
pub struct ComparativeBenchmark< T >
{
  name: String,
  description: String,
  #[allow(clippy::type_complexity)]
  algorithms: HashMap< String, Box< dyn Fn( &T ) -> Duration + Send + Sync > >,
  baseline_name: Option< String >,
}

impl< T > ComparativeBenchmark< T >
{
  #[must_use]
  pub fn new( name: &str, description: &str ) -> Self
  {
  Self
  {
  name: name.to_string(),
  description: description.to_string(),
  algorithms: HashMap::new(),
  baseline_name: None,
 }
 }

  #[must_use]
  pub fn add_algorithm< F >( mut self, name: &str, algorithm: F ) -> Self
  where
  F: Fn( &T ) -> Duration + Send + Sync + 'static,
  {
  self.algorithms.insert( name.to_string(), Box::new( algorithm ) );
  self
 }

  #[must_use]
  pub fn set_baseline( mut self, baseline_name: &str ) -> Self
  {
  self.baseline_name = Some( baseline_name.to_string() );
  self
 }

  #[must_use]
  pub fn name( &self ) -> &str
  {
  &self.name
 }

  #[must_use]
  pub fn description( &self ) -> &str
  {
  &self.description
 }

  #[must_use]
  pub fn algorithm_count( &self ) -> usize
  {
  self.algorithms.len()
 }

  pub fn run_comparison( &self, data: &T ) -> ComparisonResult
  {
  let mut results = HashMap::new();

  for ( name, algorithm ) in &self.algorithms
  {
  let time = algorithm( data );
  results.insert( name.clone(), time );
 }

  let baseline_time = self.baseline_name.as_ref()
  .and_then( |name| results.get( name ) )
  .copied();

  ComparisonResult::new( results, baseline_time )
 }
}

#[ derive( Debug, Clone ) ]
pub struct ComparisonResult
{
  results: HashMap< String, Duration >,
  baseline_time: Option< Duration >,
}

impl ComparisonResult
{
  #[must_use]
  pub fn new( results: HashMap< String, Duration >, baseline_time: Option< Duration > ) -> Self
  {
  Self { results, baseline_time }
 }

  #[must_use]
  pub fn get_relative_performance( &self, algorithm: &str ) -> Option< f64 >
  {
  if let ( Some( time ), Some( baseline ) ) = ( self.results.get( algorithm ), self.baseline_time )
  {
  if baseline.as_nanos() > 0
  {
  Some( time.as_nanos() as f64 / baseline.as_nanos() as f64 )
 }
  else
  {
  None
 }
 }
  else
  {
  None
 }
 }

  #[must_use]
  pub fn fastest_algorithm( &self ) -> Option< ( &String, &Duration ) >
  {
  self.results.iter().min_by_key( |( _, time )| time.as_nanos() )
 }

  #[must_use]
  pub fn algorithm_count( &self ) -> usize
  {
  self.results.len()
 }
}

#[ derive( Debug, Clone ) ]
pub struct OptimizationWorkflow
{
  name: String,
  baseline: Option< MockBenchmarkResult >,
  current: Option< MockBenchmarkResult >,
  history: Vec< OptimizationStep >,
}

#[ derive( Debug, Clone ) ]
pub struct OptimizationStep
{
  step_name: String,
  result: MockBenchmarkResult,
  improvement_percent: f64,
  is_regression: bool,
}

impl OptimizationWorkflow
{
  #[must_use]
  pub fn new( name: &str ) -> Self
  {
  Self
  {
  name: name.to_string(),
  baseline: None,
  current: None,
  history: Vec::new(),
 }
 }

  #[must_use]
  pub fn set_baseline( mut self, baseline: MockBenchmarkResult ) -> Self
  {
  self.baseline = Some( baseline );
  self
 }

  #[must_use]
  pub fn add_optimization_step( mut self, step_name: &str, result: MockBenchmarkResult ) -> Self
  {
  let improvement_percent = if let Some( ref baseline ) = self.baseline
  {
  let baseline_avg = baseline.average_time().as_nanos() as f64;
  let current_avg = result.average_time().as_nanos() as f64;

  if baseline_avg > 0.0
  {
  ( ( baseline_avg - current_avg ) / baseline_avg ) * 100.0
 }
  else
  {
  0.0
 }
 }
  else
  {
  0.0
 };

  let is_regression = improvement_percent < 0.0;

  self.history.push( OptimizationStep
  {
  step_name: step_name.to_string(),
  result: result.clone(),
  improvement_percent,
  is_regression,
 });

  self.current = Some( result );
  self
 }

  #[must_use]
  pub fn name( &self ) -> &str
  {
  &self.name
 }

  #[must_use]
  pub fn step_count( &self ) -> usize
  {
  self.history.len()
 }

  #[must_use]
  pub fn total_improvement( &self ) -> Option< f64 >
  {
  if let ( Some( ref baseline ), Some( ref current ) ) = ( &self.baseline, &self.current )
  {
  let baseline_avg = baseline.average_time().as_nanos() as f64;
  let current_avg = current.average_time().as_nanos() as f64;

  if baseline_avg > 0.0
  {
  Some( ( ( baseline_avg - current_avg ) / baseline_avg ) * 100.0 )
 }
  else
  {
  Some( 0.0 )
 }
 }
  else
  {
  None
 }
 }

  #[must_use]
  pub fn has_regressions( &self ) -> bool
  {
  self.history.iter().any( |step| step.is_regression )
 }
}

// === Tests ===

/// Test CV analyzer creation and configuration
#[ test ]
fn test_cv_analyzer_creation()
{
  let analyzer = CvAnalyzer::new();
  assert!( ( analyzer.cv_tolerance - 0.15 ).abs() < f64::EPSILON );
  assert_eq!( analyzer.environment, "Development" );

  let custom_analyzer = CvAnalyzer::with_config( 0.05, "Production" );
  assert!( ( custom_analyzer.cv_tolerance - 0.05 ).abs() < f64::EPSILON );
  assert_eq!( custom_analyzer.environment, "Production" );
}

/// Test CV quality classification
#[ test ]
fn test_cv_quality_classification()
{
  assert!( matches!( CvQuality::from_cv_percentage( 3.0 ), CvQuality::Excellent ) );
  assert!( matches!( CvQuality::from_cv_percentage( 7.0 ), CvQuality::Good ) );
  assert!( matches!( CvQuality::from_cv_percentage( 12.0 ), CvQuality::Moderate ) );
  assert!( matches!( CvQuality::from_cv_percentage( 20.0 ), CvQuality::Poor ) );

  // Test edge cases
  assert!( matches!( CvQuality::from_cv_percentage( 5.0 ), CvQuality::Good ) );
  assert!( matches!( CvQuality::from_cv_percentage( 10.0 ), CvQuality::Moderate ) );
  assert!( matches!( CvQuality::from_cv_percentage( 15.0 ), CvQuality::Poor ) );
}

/// Test CV quality indicators and descriptions
#[ test ]
fn test_cv_quality_indicators()
{
  assert_eq!( CvQuality::Excellent.indicator(), "✅" );
  assert_eq!( CvQuality::Good.indicator(), "🟢" );
  assert_eq!( CvQuality::Moderate.indicator(), "🟡" );
  assert_eq!( CvQuality::Poor.indicator(), "❌" );

  assert!( CvQuality::Excellent.description().contains( "Excellent" ) );
  assert!( CvQuality::Good.description().contains( "Good" ) );
  assert!( CvQuality::Moderate.description().contains( "Moderate" ) );
  assert!( CvQuality::Poor.description().contains( "Poor" ) );
}

/// Test coefficient of variation calculation
#[ test ]
fn test_cv_calculation()
{
  // Test perfect consistency (CV = 0)
  let consistent_times = vec![ Duration::from_nanos( 100 ); 10 ];
  let consistent_result = MockBenchmarkResult::new( "consistent", consistent_times, 1000 );
  assert!( consistent_result.coefficient_of_variation() < 0.001 );

  // Test high variation
  let variable_times = vec![
  Duration::from_nanos( 50 ),
  Duration::from_nanos( 100 ),
  Duration::from_nanos( 150 ),
  Duration::from_nanos( 200 ),
 ];
  let variable_result = MockBenchmarkResult::new( "variable", variable_times, 1000 );
  assert!( variable_result.coefficient_of_variation() > 0.2 );

  // Test empty case
  let empty_result = MockBenchmarkResult::new( "empty", vec![], 1000 );
  assert!( empty_result.coefficient_of_variation().abs() < f64::EPSILON );
}

/// Test CV analysis report generation
#[ test ]
fn test_cv_analysis_report()
{
  let analyzer = CvAnalyzer::with_config( 0.10, "Staging" );

  // Test excellent quality result
  let excellent_times = vec![ Duration::from_nanos( 100 ), Duration::from_nanos( 102 ), Duration::from_nanos( 98 ) ];
  let excellent_result = MockBenchmarkResult::new( "excellent_algo", excellent_times, 1000 );

  let report = analyzer.analyze_result( "excellent_test", &excellent_result );

  assert_eq!( report.benchmark_name, "excellent_test" );
  assert!( report.cv_percentage < 5.0 );
  assert!( matches!( report.quality, CvQuality::Excellent ) );
  assert!( report.meets_environment_requirements );
  assert_eq!( report.environment, "Staging" );
  assert!( ( report.cv_tolerance - 0.10 ).abs() < f64::EPSILON );
}

/// Test sample size recommendations
#[ test ]
fn test_sample_size_recommendations()
{
  let analyzer = CvAnalyzer::with_config( 0.10, "Production" );

  // Test low CV - should recommend minimum samples
  let low_cv_result = MockBenchmarkResult::new( "low_cv", vec![ Duration::from_nanos( 100 ); 5 ], 1000 );
  let report = analyzer.analyze_result( "test", &low_cv_result );
  assert_eq!( report.recommended_sample_size, 20 );

  // Test high CV - should recommend more samples
  let high_cv_times = vec![ Duration::from_nanos( 50 ), Duration::from_nanos( 150 ) ];
  let high_cv_result = MockBenchmarkResult::new( "high_cv", high_cv_times, 1000 );
  let report = analyzer.analyze_result( "test", &high_cv_result );
  assert!( report.recommended_sample_size > 20 );
}

/// Test comparative benchmark creation
#[ test ]
fn test_comparative_benchmark_creation()
{
  let comparison: ComparativeBenchmark< Vec< i32 > > = ComparativeBenchmark::new(
  "Sorting Algorithms Comparison",
  "Performance comparison of different sorting algorithms"
 );

  assert_eq!( comparison.name(), "Sorting Algorithms Comparison" );
  assert_eq!( comparison.description(), "Performance comparison of different sorting algorithms" );
  assert_eq!( comparison.algorithm_count(), 0 );
}

/// Test comparative benchmark algorithm addition
#[ test ]
fn test_comparative_benchmark_algorithms()
{
  let comparison = ComparativeBenchmark::new( "Test", "Description" )
  .add_algorithm( "algorithm_a", |_data: &Vec< i32 >| Duration::from_nanos( 100 ) )
  .add_algorithm( "algorithm_b", |_data: &Vec< i32 >| Duration::from_nanos( 150 ) )
  .set_baseline( "algorithm_a" );

  assert_eq!( comparison.algorithm_count(), 2 );
}

/// Test comparative benchmark execution
#[ test ]
fn test_comparative_benchmark_execution()
{
  let comparison = ComparativeBenchmark::new( "Test", "Description" )
  .add_algorithm( "fast", |_data: &Vec< i32 >| Duration::from_nanos( 100 ) )
  .add_algorithm( "slow", |_data: &Vec< i32 >| Duration::from_nanos( 200 ) )
  .set_baseline( "fast" );

  let test_data = vec![ 1, 2, 3, 4, 5 ];
  let result = comparison.run_comparison( &test_data );

  assert_eq!( result.algorithm_count(), 2 );

  // Test relative performance
  let fast_perf = result.get_relative_performance( "fast" ).unwrap();
  let slow_perf = result.get_relative_performance( "slow" ).unwrap();

  assert!( ( fast_perf - 1.0 ).abs() < 0.001 ); // Baseline should be 1.0
  assert!( slow_perf > 1.0 ); // Slower algorithm should be > 1.0

  // Test fastest algorithm detection
  let ( fastest_name, _fastest_time ) = result.fastest_algorithm().unwrap();
  assert_eq!( fastest_name, "fast" );
}

/// Test optimization workflow creation
#[ test ]
fn test_optimization_workflow_creation()
{
  let workflow = OptimizationWorkflow::new( "String Processing Optimization" );

  assert_eq!( workflow.name(), "String Processing Optimization" );
  assert_eq!( workflow.step_count(), 0 );
  assert!( workflow.total_improvement().is_none() );
  assert!( !workflow.has_regressions() );
}

/// Test optimization workflow with baseline
#[ test ]
fn test_optimization_workflow_baseline()
{
  let baseline_times = vec![ Duration::from_nanos( 1000 ); 10 ];
  let baseline = MockBenchmarkResult::new( "baseline", baseline_times, 1000 );

  let workflow = OptimizationWorkflow::new( "Test Optimization" )
  .set_baseline( baseline );

  assert!( workflow.total_improvement().is_none() ); // No optimizations yet
}

/// Test optimization workflow steps
#[ test ]
fn test_optimization_workflow_steps()
{
  let baseline_times = vec![ Duration::from_nanos( 1000 ); 10 ];
  let baseline = MockBenchmarkResult::new( "baseline", baseline_times, 1000 );

  // First optimization - 20% improvement
  let optimized_times = vec![ Duration::from_nanos( 800 ); 10 ];
  let optimized = MockBenchmarkResult::new( "optimized", optimized_times, 1000 );

  let workflow = OptimizationWorkflow::new( "Test Optimization" )
  .set_baseline( baseline )
  .add_optimization_step( "Algorithm optimization", optimized );

  assert_eq!( workflow.step_count(), 1 );

  let total_improvement = workflow.total_improvement().unwrap();
  assert!( ( total_improvement - 20.0 ).abs() < 1.0 ); // ~20% improvement

  assert!( !workflow.has_regressions() );
}

/// Test optimization workflow with regression
#[ test ]
fn test_optimization_workflow_regression()
{
  let baseline_times = vec![ Duration::from_nanos( 1000 ); 10 ];
  let baseline = MockBenchmarkResult::new( "baseline", baseline_times, 1000 );

  // Regression - 50% slower
  let regression_times = vec![ Duration::from_nanos( 1500 ); 10 ];
  let regression = MockBenchmarkResult::new( "regression", regression_times, 1000 );

  let workflow = OptimizationWorkflow::new( "Test Optimization" )
  .set_baseline( baseline )
  .add_optimization_step( "Failed optimization", regression );

  assert_eq!( workflow.step_count(), 1 );
  assert!( workflow.has_regressions() );

  let total_improvement = workflow.total_improvement().unwrap();
  assert!( total_improvement < 0.0 ); // Negative improvement (regression)
}

/// Test statistical significance validation
#[ test ]
fn test_statistical_significance()
{
  // Test significant improvement
  let baseline_times = vec![ Duration::from_nanos( 1000 ); 50 ];
  let baseline = MockBenchmarkResult::new( "baseline", baseline_times, 1000 );

  let improved_times = vec![ Duration::from_nanos( 800 ); 50 ];
  let improved = MockBenchmarkResult::new( "improved", improved_times, 1000 );

  let significance = calculate_statistical_significance( &baseline, &improved );
  assert!( significance.is_significant );
  assert!( significance.improvement_percent > 15.0 );

  // Test non-significant change
  let similar_times = vec![ Duration::from_nanos( 990 ); 50 ];
  let similar = MockBenchmarkResult::new( "similar", similar_times, 1000 );

  let significance = calculate_statistical_significance( &baseline, &similar );
  assert!( !significance.is_significant );
  assert!( significance.improvement_percent.abs() < 5.0 );
}

/// Test benchmark quality assessment
#[ test ]
fn test_benchmark_quality_assessment()
{
  // High quality benchmark (low CV, sufficient samples)
  let high_quality_times = vec![
    Duration::from_nanos( 1000 ), Duration::from_nanos( 1001 ), Duration::from_nanos( 999 ),
    Duration::from_nanos( 1000 ), Duration::from_nanos( 1001 ), Duration::from_nanos( 999 ),
    Duration::from_nanos( 1000 ), Duration::from_nanos( 1001 ), Duration::from_nanos( 999 ),
    Duration::from_nanos( 1000 ), Duration::from_nanos( 1001 ), Duration::from_nanos( 999 )
  ];
  let high_quality = MockBenchmarkResult::new( "high_quality", high_quality_times, 1000 );

  let quality = assess_benchmark_quality( &high_quality );
  assert!( quality.is_reliable );
  assert!( quality.cv_percentage < 5.0 );

  // Low quality benchmark (high CV, sufficient samples)
  let low_quality_times = vec![
    Duration::from_nanos( 500 ), Duration::from_nanos( 1500 ), Duration::from_nanos( 800 ),
    Duration::from_nanos( 1200 ), Duration::from_nanos( 600 ), Duration::from_nanos( 1400 ),
    Duration::from_nanos( 700 ), Duration::from_nanos( 1300 ), Duration::from_nanos( 900 ),
    Duration::from_nanos( 1100 ), Duration::from_nanos( 750 ), Duration::from_nanos( 1250 )
  ];
  let low_quality = MockBenchmarkResult::new( "low_quality", low_quality_times, 1000 );

  let quality = assess_benchmark_quality( &low_quality );
  assert!( !quality.is_reliable );
  assert!( quality.cv_percentage > 15.0 );
}

/// Test large dataset handling
#[ test ]
fn test_large_dataset_handling()
{
  // Generate large dataset
  let mut large_times = Vec::new();
  for i in 0..10000
  {
  #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
  large_times.push( Duration::from_nanos( 1000 + ( i % 100 ) as u64 ) );
 }

  let large_result = MockBenchmarkResult::new( "large_test", large_times, 100_000 );

  let analyzer = CvAnalyzer::new();
  let report = analyzer.analyze_result( "large_benchmark", &large_result );

  assert_eq!( report.current_sample_size, 10000 );
  assert!( report.cv_percentage < 15.0 ); // Should have decent CV with this pattern
}

/// Test error handling for invalid data
#[ test ]
fn test_error_handling()
{
  // Test empty benchmark result
  let empty_result = MockBenchmarkResult::new( "empty", vec![], 1000 );
  let analyzer = CvAnalyzer::new();
  let report = analyzer.analyze_result( "empty_test", &empty_result );

  assert!( report.cv_percentage.abs() < f64::EPSILON );
  assert_eq!( report.current_sample_size, 0 );

  // Test single sample
  let single_sample = MockBenchmarkResult::new( "single", vec![ Duration::from_nanos( 1000 ) ], 1000 );
  let report = analyzer.analyze_result( "single_test", &single_sample );

  assert_eq!( report.current_sample_size, 1 );
  assert!( report.cv_percentage.abs() < f64::EPSILON ); // CV is 0 for single sample
}

/// Test integration with multiple analysis tools
#[ test ]
fn test_analysis_tools_integration()
{
  // Setup benchmark results
  let times_a = vec![ Duration::from_nanos( 1000 ); 20 ];
  let result_a = MockBenchmarkResult::new( "algorithm_a", times_a, 1000 );

  let times_b = vec![ Duration::from_nanos( 800 ); 20 ];
  let result_b = MockBenchmarkResult::new( "algorithm_b", times_b, 1000 );

  // CV Analysis
  let analyzer = CvAnalyzer::with_config( 0.10, "Production" );
  let report_a = analyzer.analyze_result( "test_a", &result_a );
  let report_b = analyzer.analyze_result( "test_b", &result_b );

  // Comparative Analysis
  let comparison = ComparativeBenchmark::new( "A vs B", "Performance comparison" )
  .add_algorithm( "a", |_: &Vec< i32 >| Duration::from_nanos( 1000 ) )
  .add_algorithm( "b", |_: &Vec< i32 >| Duration::from_nanos( 800 ) )
  .set_baseline( "a" );

  let test_data = vec![ 1, 2, 3 ];
  let comp_result = comparison.run_comparison( &test_data );

  // Optimization Workflow
  let workflow = OptimizationWorkflow::new( "A to B Optimization" )
  .set_baseline( result_a )
  .add_optimization_step( "Algorithm B implementation", result_b );

  // Verify integration
  assert!( matches!( report_a.quality, CvQuality::Excellent ) );
  assert!( matches!( report_b.quality, CvQuality::Excellent ) );

  let relative_perf = comp_result.get_relative_performance( "b" ).unwrap();
  assert!( relative_perf < 1.0 ); // B is faster than A

  let improvement = workflow.total_improvement().unwrap();
  assert!( improvement > 15.0 ); // Significant improvement
}

// Helper functions for testing

#[ derive( Debug, Clone ) ]
pub struct StatisticalSignificance
{
  pub is_significant: bool,
  pub improvement_percent: f64,
  pub confidence_level: f64,
}

fn calculate_statistical_significance( baseline: &MockBenchmarkResult, improved: &MockBenchmarkResult ) -> StatisticalSignificance
{
  let baseline_avg = baseline.average_time().as_nanos() as f64;
  let improved_avg = improved.average_time().as_nanos() as f64;

  let improvement_percent = if baseline_avg > 0.0
  {
  ( ( baseline_avg - improved_avg ) / baseline_avg ) * 100.0
 }
  else
  {
  0.0
 };

  // Simple significance test - in real implementation would use proper statistical tests
  let is_significant = improvement_percent.abs() > 5.0 && baseline.times.len() >= 10 && improved.times.len() >= 10;

  StatisticalSignificance
  {
  is_significant,
  improvement_percent,
  confidence_level: if is_significant { 0.95 } else { 0.5 },
 }
}

#[ derive( Debug, Clone ) ]
pub struct BenchmarkQuality
{
  pub is_reliable: bool,
  pub cv_percentage: f64,
  pub sample_size: usize,
  pub quality_score: f64,
}

fn assess_benchmark_quality( result: &MockBenchmarkResult ) -> BenchmarkQuality
{
  let cv_percentage = result.coefficient_of_variation() * 100.0;
  let sample_size = result.times.len();

  let is_reliable = cv_percentage < 10.0 && sample_size >= 10;

  // Quality score based on CV and sample size
  let cv_score = if cv_percentage < 5.0 { 1.0 } else { 1.0 / ( cv_percentage / 5.0 ) };
  let size_score = if sample_size >= 50 { 1.0 } else { sample_size as f64 / 50.0 };
  let quality_score = cv_score * size_score;

  BenchmarkQuality
  {
  is_reliable,
  cv_percentage,
  sample_size,
  quality_score,
 }
}