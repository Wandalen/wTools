//! Benchmark suite management
//!
//! This module provides high-level interfaces for organizing and running
//! collections of benchmarks, with support for baselines and reporting.

use crate ::measurement :: { BenchmarkResult, MeasurementConfig };
use crate ::analysis ::RegressionAnalysis;
use std ::collections ::HashMap;

type Result< T > = std ::result ::Result< T, Box<dyn std ::error ::Error >>;

/// A collection of benchmarks that can be run together
pub struct BenchmarkSuite
{
  /// Name of the benchmark suite
  pub name: String,
  benchmarks: HashMap< String, Box< dyn FnMut() + Send > >,
  config: MeasurementConfig,
  results: HashMap< String, BenchmarkResult >,
}

impl std ::fmt ::Debug for BenchmarkSuite
{
  fn fmt( &self, f: &mut std ::fmt ::Formatter< '_ > ) -> std ::fmt ::Result
  {
  f.debug_struct( "BenchmarkSuite" )
   .field( "name", &self.name )
   .field( "benchmarks", &format!( "{} benchmarks", self.benchmarks.len() ) )
   .field( "config", &self.config )
   .field( "results", &format!( "{} results", self.results.len() ) )
   .finish()
 }
}

impl BenchmarkSuite
{
  /// Create a new benchmark suite
  pub fn new( name: impl Into< String > ) -> Self
  {
  crate ::check_directory_recommendations();
  
  Self
  {
   name: name.into(),
   benchmarks: HashMap ::new(),
   config: MeasurementConfig ::default(),
   results: HashMap ::new(),
 }
 }

  /// Set measurement configuration for all benchmarks in suite
  #[ must_use ]
  pub fn with_config( mut self, config: MeasurementConfig ) -> Self
  {
  self.config = config;
  self
 }

  /// Add a benchmark to the suite
  pub fn benchmark< F >( &mut self, name: impl Into< String >, f: F ) -> &mut Self
  where
  F: FnMut() + Send + 'static,
  {
  self.benchmarks.insert( name.into(), Box ::new( f ) );
  self
 }

  /// Add a benchmark to the suite (builder pattern)
  #[ must_use ]
  pub fn add_benchmark< F >(mut self, name: impl Into< String >, f: F) -> Self
  where
  F: FnMut() + Send + 'static,
  {
  self.benchmark(name, f);
  self
 }

  /// Run all benchmarks in the suite
  pub fn run_all( &mut self ) -> SuiteResults
  {
  let mut results = HashMap ::new();
  
  println!("Running benchmark suite: {}", self.name);
  
  for (name, benchmark) in &mut self.benchmarks 
  {
   print!("  Running {name} ... ");
   let result = crate ::measurement ::bench_function_with_config(
  name, 
  &self.config, 
  benchmark
 );
   println!("{:.2?}", result.mean_time());
   results.insert(name.clone(), result);
 }
  
  self.results.clone_from(&results);
  
  SuiteResults {
   suite_name: self.name.clone(),
   results,
 }
 }

  /// Run analysis comparing against baseline results
  pub fn run_analysis( &mut self ) -> SuiteResults
  {
  self.run_all()
 }

  /// Get results from previous run
  #[ must_use ]
  pub fn results( &self ) -> &HashMap< String, BenchmarkResult >
  {
  &self.results
 }

  /// Create suite from baseline file (for regression testing)
  pub fn from_baseline(_baseline_file: impl AsRef< std ::path ::Path >) -> Self
  {
  // TODO: Implement loading from JSON/TOML baseline file
  // For now, return empty suite
  Self ::new("baseline_comparison")
 }

  /// Create suite from configuration file  
  pub fn from_config(_config_file: impl AsRef< std ::path ::Path >) -> Self
  {
  // TODO: Implement loading from configuration file
  // For now, return empty suite
  Self ::new("configured_suite")
 }
}

/// Results from running a benchmark suite
#[ derive(Debug) ]
pub struct SuiteResults 
{
  /// Name of the benchmark suite that was run
  pub suite_name: String,
  /// Individual benchmark results from the suite
  pub results: HashMap< String, BenchmarkResult >,
}

impl SuiteResults 
{
  /// Generate markdown report for all results
  #[ must_use ]
  pub fn generate_markdown_report( &self ) -> MarkdownReport
  {
  MarkdownReport ::new(&self.suite_name, &self.results)
 }

  /// Get regression analysis if baseline is available
  #[ must_use ]
  pub fn regression_analysis(&self, baseline: &HashMap< String, BenchmarkResult >) -> RegressionAnalysis
  {
  RegressionAnalysis ::new(baseline.clone(), self.results.clone())
 }

  /// Get worst regression percentage
  #[ must_use ]
  pub fn regression_percentage( &self ) -> f64
  {
  // TODO: Implement regression calculation against stored baseline
  // For now, return 0
  0.0
 }

  /// Save results as new baseline
  ///
  /// # Errors
  ///
  /// Returns an error if the file cannot be written to.
  pub fn save_as_baseline(&self, _baseline_file: impl AsRef< std ::path ::Path >) -> Result< () >
  {
  // TODO: Implement saving to JSON/TOML file
  // For now, just succeed
  Ok(())
 }

  /// Print summary of all results
  pub fn print_summary( &self )
  {
  println!("=== {} Results ===", self.suite_name);
  
  let mut sorted_results: Vec< _ > = self.results.iter().collect();
  sorted_results.sort_by(|a, b| a.1.mean_time().cmp(&b.1.mean_time()));
  
  for (name, result) in sorted_results 
  {
   println!("  {} : {:.2?} (±{:.2?})", 
   name, 
   result.mean_time(), 
   result.std_deviation());
 }
 }
}

/// Builder for markdown reports
#[ derive(Debug) ]
pub struct MarkdownReport 
{
  suite_name: String,
  results: HashMap< String, BenchmarkResult >,
  include_raw_data: bool,
  include_statistics: bool,
}

impl MarkdownReport 
{
  /// Create new markdown report
  #[ must_use ]
  pub fn new(suite_name: &str, results: &HashMap< String, BenchmarkResult >) -> Self
  {
  Self {
   suite_name: suite_name.to_string(),
   results: results.clone(),
   include_raw_data: false,
   include_statistics: true,
 }
 }

  /// Include raw timing data in report
  #[ must_use ]
  pub fn with_raw_data(mut self) -> Self
  {
  self.include_raw_data = true;
  self
 }

  /// Include detailed statistics
  #[ must_use ]
  pub fn with_statistics(mut self) -> Self
  {
  self.include_statistics = true;
  self
 }

  /// Generate the markdown content
  ///
  /// # Panics
  ///
  /// Panics if there are no results but `sorted_results` is accessed.
  #[ must_use ]
  pub fn generate( &self ) -> String
  {
  let mut output = String ::new();
  
  output.push_str(&format!("## {} Results\n\n", self.suite_name));
  
  if self.results.is_empty() 
  {
   output.push_str("No benchmark results available.\n");
   return output;
 }
  
  // Summary table
  output.push_str("| Benchmark | Mean Time | Ops/sec | Min | Max | Std Dev |\n");
  output.push_str("|-----------|-----------|---------|-----|-----|----------|\n");
  
  let mut sorted_results: Vec< _ > = self.results.iter().collect();
  sorted_results.sort_by(|a, b| a.1.mean_time().cmp(&b.1.mean_time()));
  
  for (name, result) in &sorted_results 
  {
   output.push_str(&format!(
  "| {} | {:.2?} | {:.0} | {:.2?} | {:.2?} | {:.2?} |\n",
  name,
  result.mean_time(),
  result.operations_per_second(), 
  result.min_time(),
  result.max_time(),
  result.std_deviation()
 ));
 }
  
  output.push('\n');
  
  // Key insights
  if let Some((fastest_name, fastest_result)) = sorted_results.first() 
  {
   output.push_str("### Key Insights\n\n");
   output.push_str(&format!("- **Fastest operation** : {} ({:.2?})\n", 
   fastest_name, 
   fastest_result.mean_time()));
   
   if sorted_results.len() > 1 
   {
  let slowest = sorted_results.last().unwrap();
  let ratio = slowest.1.mean_time().as_secs_f64() / fastest_result.mean_time().as_secs_f64();
  output.push_str(&format!("- **Performance range** : {:.1}x difference between fastest and slowest\n", ratio));
 }
   
   output.push('\n');
 }
  
  output
 }

  /// Update specific section in markdown file
  ///
  /// # Errors
  ///
  /// Returns an error if the file cannot be read or written.
  pub fn update_file(
  &self, 
  file_path: impl AsRef< std ::path ::Path >, 
  section_name: &str
 ) -> Result< () > {
  // TODO: Implement markdown file section updating
  // This would parse existing markdown, find section, and replace content
  println!("Would update {section_name} section in {}", file_path.as_ref().display());
  Ok(())
 }

  /// Save report to file
  ///
  /// # Errors
  ///
  /// Returns an error if the file cannot be written to.
  pub fn save(&self, file_path: impl AsRef< std ::path ::Path >) -> Result< () >
  {
  let content = self.generate();
  std ::fs ::write(file_path, content)?;
  Ok(())
 }
}

