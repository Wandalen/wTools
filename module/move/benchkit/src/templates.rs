//! Template system for consistent documentation formatting
//!
//! Provides standardized report templates for common benchmarking scenarios
//! with customizable sections while maintaining professional output quality.

use crate::measurement::BenchmarkResult;
use std::collections::HashMap;

type Result< T > = std::result::Result< T, Box< dyn std::error::Error > >;

/// Trait for report template generation
pub trait ReportTemplate
{
  /// Generate the report content from benchmark results
  fn generate( &self, results : &HashMap< String, BenchmarkResult > ) -> Result< String >;
}

/// Standard performance benchmark report template
#[ derive( Debug, Clone ) ]
pub struct PerformanceReport
{
  /// Report title
  title : String,
  /// Context description for the benchmarks
  context : Option< String >,
  /// Whether to include detailed statistical analysis
  include_statistical_analysis : bool,
  /// Whether to include regression analysis section
  include_regression_analysis : bool,
  /// Custom sections to include
  custom_sections : Vec< CustomSection >,
}

impl PerformanceReport
{
  /// Create new performance report template
  #[ must_use ]
  pub fn new() -> Self
  {
    Self
    {
      title : "Performance Analysis".to_string(),
      context : None,
      include_statistical_analysis : true,
      include_regression_analysis : false,
      custom_sections : Vec::new(),
    }
  }

  /// Set the report title
  #[ must_use ]
  pub fn title( mut self, title : impl Into< String > ) -> Self
  {
    self.title = title.into();
    self
  }

  /// Add context description
  #[ must_use ]
  pub fn add_context( mut self, context : impl Into< String > ) -> Self
  {
    self.context = Some( context.into() );
    self
  }

  /// Enable or disable statistical analysis section
  #[ must_use ]
  pub fn include_statistical_analysis( mut self, include : bool ) -> Self
  {
    self.include_statistical_analysis = include;
    self
  }

  /// Enable or disable regression analysis section
  #[ must_use ]
  pub fn include_regression_analysis( mut self, include : bool ) -> Self
  {
    self.include_regression_analysis = include;
    self
  }

  /// Add custom section to the report
  #[ must_use ]
  pub fn add_custom_section( mut self, section : CustomSection ) -> Self
  {
    self.custom_sections.push( section );
    self
  }
}

impl Default for PerformanceReport
{
  fn default() -> Self
  {
    Self::new()
  }
}

impl ReportTemplate for PerformanceReport
{
  fn generate( &self, results : &HashMap< String, BenchmarkResult > ) -> Result< String >
  {
    let mut output = String::new();

    // Title and context
    output.push_str( &format!( "# {}\n\n", self.title ) );
    
    if let Some( ref context ) = self.context
    {
      output.push_str( &format!( "*{}*\n\n", context ) );
    }

    if results.is_empty()
    {
      output.push_str( "No benchmark results available.\n" );
      return Ok( output );
    }

    // Executive Summary
    output.push_str( "## Executive Summary\n\n" );
    self.add_executive_summary( &mut output, results );

    // Performance Results Table
    output.push_str( "## Performance Results\n\n" );
    self.add_performance_table( &mut output, results );

    // Statistical Analysis (optional)
    if self.include_statistical_analysis
    {
      output.push_str( "## Statistical Analysis\n\n" );
      self.add_statistical_analysis( &mut output, results );
    }

    // Regression Analysis (optional)
    if self.include_regression_analysis
    {
      output.push_str( "## Regression Analysis\n\n" );
      self.add_regression_analysis( &mut output, results );
    }

    // Custom sections
    for section in &self.custom_sections
    {
      output.push_str( &format!( "## {}\n\n", section.title ) );
      output.push_str( &section.content );
      output.push_str( "\n\n" );
    }

    // Methodology footer
    output.push_str( "## Methodology\n\n" );
    self.add_methodology_note( &mut output );

    Ok( output )
  }
}

impl PerformanceReport
{
  /// Add executive summary section
  fn add_executive_summary( &self, output : &mut String, results : &HashMap< String, BenchmarkResult > )
  {
    let total_tests = results.len();
    let reliable_tests = results.values().filter( | r | r.is_reliable() ).count();
    let reliability_rate = ( reliable_tests as f64 / total_tests as f64 ) * 100.0;

    output.push_str( &format!( "- **Total operations benchmarked**: {}\n", total_tests ) );
    output.push_str( &format!( "- **Statistically reliable results**: {}/{} ({:.1}%)\n", 
                               reliable_tests, total_tests, reliability_rate ) );

    if let Some( ( fastest_name, fastest_result ) ) = self.find_fastest( results )
    {
      output.push_str( &format!( "- **Best performing operation**: {} ({:.2?})\n", 
                                 fastest_name, fastest_result.mean_time() ) );
    }

    if results.len() > 1
    {
      if let Some( ( slowest_name, slowest_result ) ) = self.find_slowest( results )
      {
        if let Some( ( fastest_name_inner, fastest_result ) ) = self.find_fastest( results )
        {
          let ratio = slowest_result.mean_time().as_secs_f64() / fastest_result.mean_time().as_secs_f64();
          output.push_str( &format!( "- **Performance range**: {:.1}x difference ({} vs {})\n", 
                                     ratio, fastest_name_inner, slowest_name ) );
        }
      }
    }

    output.push_str( "\n" );
  }

  /// Add performance results table
  fn add_performance_table( &self, output : &mut String, results : &HashMap< String, BenchmarkResult > )
  {
    output.push_str( "| Operation | Mean Time | 95% CI | Ops/sec | CV | Reliability | Samples |\n" );
    output.push_str( "|-----------|-----------|--------|---------|----|-----------|---------|\n" );

    // Sort by performance
    let mut sorted_results : Vec< _ > = results.iter().collect();
    sorted_results.sort_by( | a, b | a.1.mean_time().cmp( &b.1.mean_time() ) );

    for ( name, result ) in sorted_results
    {
      let ( ci_lower, ci_upper ) = result.confidence_interval_95();
      let cv = result.coefficient_of_variation();
      let reliability = if result.is_reliable() { "✅" } else { "⚠️" };

      output.push_str( &format!(
        "| {} | {:.2?} | [{:.2?} - {:.2?}] | {:.0} | {:.1}% | {} | {} |\n",
        name,
        result.mean_time(),
        ci_lower,
        ci_upper,
        result.operations_per_second(),
        cv * 100.0,
        reliability,
        result.times.len()
      ) );
    }

    output.push_str( "\n" );
  }

  /// Add statistical analysis section
  fn add_statistical_analysis( &self, output : &mut String, results : &HashMap< String, BenchmarkResult > )
  {
    let mut high_quality = Vec::new();
    let mut needs_improvement = Vec::new();

    for ( name, result ) in results
    {
      if result.is_reliable()
      {
        high_quality.push( name );
      }
      else
      {
        let cv = result.coefficient_of_variation();
        let sample_size = result.times.len();
        let mut issues = Vec::new();

        if sample_size < 10
        {
          issues.push( "insufficient samples" );
        }
        if cv > 0.1
        {
          issues.push( "high variability" );
        }

        needs_improvement.push( ( name, issues ) );
      }
    }

    if !high_quality.is_empty()
    {
      output.push_str( "### ✅ Reliable Results\n" );
      output.push_str( "*These measurements meet research-grade statistical standards*\n\n" );
      for name in high_quality
      {
        let result = &results[ name ];
        output.push_str( &format!( "- **{}**: {} samples, CV={:.1}%\n",
                                   name,
                                   result.times.len(),
                                   result.coefficient_of_variation() * 100.0 ) );
      }
      output.push_str( "\n" );
    }

    if !needs_improvement.is_empty()
    {
      output.push_str( "### ⚠️ Measurements Needing Attention\n" );
      output.push_str( "*Consider additional measurements for more reliable conclusions*\n\n" );
      for ( name, issues ) in needs_improvement
      {
        output.push_str( &format!( "- **{}**: {}\n", name, issues.join( ", " ) ) );
      }
      output.push_str( "\n" );
    }
  }

  /// Add regression analysis section
  fn add_regression_analysis( &self, output : &mut String, _results : &HashMap< String, BenchmarkResult > )
  {
    // xxx: Implement regression analysis when historical data is available
    // This would compare against baseline measurements or historical trends
    output.push_str( "**Regression Analysis**: Not yet implemented. Historical baseline data required.\n\n" );
  }

  /// Add methodology note
  fn add_methodology_note( &self, output : &mut String )
  {
    output.push_str( "**Statistical Reliability Criteria**:\n" );
    output.push_str( "- Sample size ≥ 10 measurements\n" );
    output.push_str( "- Coefficient of variation ≤ 10%\n" );
    output.push_str( "- Maximum/minimum time ratio < 3.0x\n\n" );

    output.push_str( "**Confidence Intervals**: 95% CI calculated using t-distribution\n" );
    output.push_str( "**CV**: Coefficient of Variation (relative standard deviation)\n\n" );

    output.push_str( "---\n" );
    output.push_str( "*Generated by benchkit - Professional benchmarking toolkit*\n" );
  }

  /// Find fastest result
  fn find_fastest< 'a >( &self, results : &'a HashMap< String, BenchmarkResult > ) -> Option< ( &'a String, &'a BenchmarkResult ) >
  {
    results.iter().min_by( | a, b | a.1.mean_time().cmp( &b.1.mean_time() ) )
  }

  /// Find slowest result
  fn find_slowest< 'a >( &self, results : &'a HashMap< String, BenchmarkResult > ) -> Option< ( &'a String, &'a BenchmarkResult ) >
  {
    results.iter().max_by( | a, b | a.1.mean_time().cmp( &b.1.mean_time() ) )
  }
}

/// Comparison report template for A/B testing scenarios
#[ derive( Debug, Clone ) ]
pub struct ComparisonReport
{
  /// Report title
  title : String,
  /// Baseline algorithm name
  baseline : String,
  /// Candidate algorithm name
  candidate : String,
  /// Statistical significance threshold (default: 0.05)
  significance_threshold : f64,
  /// Practical significance threshold (default: 0.10)
  practical_significance_threshold : f64,
}

impl ComparisonReport
{
  /// Create new comparison report template
  #[ must_use ]
  pub fn new() -> Self
  {
    Self
    {
      title : "Performance Comparison".to_string(),
      baseline : "Baseline".to_string(),
      candidate : "Candidate".to_string(),
      significance_threshold : 0.05,
      practical_significance_threshold : 0.10,
    }
  }

  /// Set the report title
  #[ must_use ]
  pub fn title( mut self, title : impl Into< String > ) -> Self
  {
    self.title = title.into();
    self
  }

  /// Set baseline algorithm name
  #[ must_use ]
  pub fn baseline( mut self, baseline : impl Into< String > ) -> Self
  {
    self.baseline = baseline.into();
    self
  }

  /// Set candidate algorithm name
  #[ must_use ]
  pub fn candidate( mut self, candidate : impl Into< String > ) -> Self
  {
    self.candidate = candidate.into();
    self
  }

  /// Set statistical significance threshold
  #[ must_use ]
  pub fn significance_threshold( mut self, threshold : f64 ) -> Self
  {
    self.significance_threshold = threshold;
    self
  }

  /// Set practical significance threshold
  #[ must_use ]
  pub fn practical_significance_threshold( mut self, threshold : f64 ) -> Self
  {
    self.practical_significance_threshold = threshold;
    self
  }
}

impl Default for ComparisonReport
{
  fn default() -> Self
  {
    Self::new()
  }
}

impl ComparisonReport
{
  /// Get baseline name (for testing)
  #[ must_use ]
  pub fn baseline_name( &self ) -> &str
  {
    &self.baseline
  }

  /// Get candidate name (for testing)
  #[ must_use ]
  pub fn candidate_name( &self ) -> &str
  {
    &self.candidate
  }

  /// Get significance threshold (for testing)
  #[ must_use ]
  pub fn significance_threshold_value( &self ) -> f64
  {
    self.significance_threshold
  }

  /// Get practical significance threshold (for testing)
  #[ must_use ]
  pub fn practical_significance_threshold_value( &self ) -> f64
  {
    self.practical_significance_threshold
  }
}

impl ReportTemplate for ComparisonReport
{
  fn generate( &self, results : &HashMap< String, BenchmarkResult > ) -> Result< String >
  {
    let mut output = String::new();

    output.push_str( &format!( "# {}\n\n", self.title ) );

    // Get baseline and candidate results
    let baseline_result = results.get( &self.baseline )
      .ok_or_else( || -> Box< dyn std::error::Error > { format!( "Baseline result '{}' not found", self.baseline ).into() } )?;
    let candidate_result = results.get( &self.candidate )
      .ok_or_else( || -> Box< dyn std::error::Error > { format!( "Candidate result '{}' not found", self.candidate ).into() } )?;

    // Calculate comparison metrics
    let baseline_time = baseline_result.mean_time().as_secs_f64();
    let candidate_time = candidate_result.mean_time().as_secs_f64();
    let improvement_ratio = baseline_time / candidate_time;
    let improvement_percent = ( improvement_ratio - 1.0 ) * 100.0;

    // Executive summary
    output.push_str( "## Comparison Summary\n\n" );
    
    if improvement_ratio > 1.0 + self.practical_significance_threshold
    {
      output.push_str( &format!( "✅ **{} is {:.1}% faster** than {}\n\n", 
                                 self.candidate, improvement_percent, self.baseline ) );
    }
    else if improvement_ratio < 1.0 - self.practical_significance_threshold
    {
      let regression_percent = ( 1.0 - improvement_ratio ) * 100.0;
      output.push_str( &format!( "🚨 **{} is {:.1}% slower** than {}\n\n", 
                                 self.candidate, regression_percent, self.baseline ) );
    }
    else
    {
      output.push_str( &format!( "⚖️ **No significant difference** between {} and {}\n\n", 
                                self.baseline, self.candidate ) );
    }

    // Detailed comparison table
    output.push_str( "## Detailed Comparison\n\n" );
    output.push_str( "| Algorithm | Mean Time | 95% CI | Ops/sec | CV | Samples | Reliability |\n" );
    output.push_str( "|-----------|-----------|--------|---------|----|---------|-----------|\n" );

    for ( name, result ) in [ ( &self.baseline, baseline_result ), ( &self.candidate, candidate_result ) ]
    {
      let ( ci_lower, ci_upper ) = result.confidence_interval_95();
      let cv = result.coefficient_of_variation();
      let reliability = if result.is_reliable() { "✅" } else { "⚠️" };

      output.push_str( &format!(
        "| {} | {:.2?} | [{:.2?} - {:.2?}] | {:.0} | {:.1}% | {} | {} |\n",
        name,
        result.mean_time(),
        ci_lower,
        ci_upper,
        result.operations_per_second(),
        cv * 100.0,
        result.times.len(),
        reliability
      ) );
    }

    output.push_str( "\n" );

    // Statistical analysis
    output.push_str( "## Statistical Analysis\n\n" );
    output.push_str( &format!( "- **Performance ratio**: {:.3}x\n", improvement_ratio ) );
    output.push_str( &format!( "- **Improvement**: {:.1}%\n", improvement_percent ) );
    
    // Confidence interval overlap analysis
    let baseline_ci = baseline_result.confidence_interval_95();
    let candidate_ci = candidate_result.confidence_interval_95();
    let ci_overlap = baseline_ci.1 >= candidate_ci.0 && candidate_ci.1 >= baseline_ci.0;
    
    if ci_overlap
    {
      output.push_str( "- **Statistical significance**: ⚠️ Confidence intervals overlap - difference may not be statistically significant\n" );
    }
    else
    {
      output.push_str( "- **Statistical significance**: ✅ No confidence interval overlap - difference is likely statistically significant\n" );
    }

    // Practical significance
    if improvement_percent.abs() >= self.practical_significance_threshold * 100.0
    {
      output.push_str( &format!( "- **Practical significance**: ✅ Difference exceeds {:.1}% threshold\n", 
                                 self.practical_significance_threshold * 100.0 ) );
    }
    else
    {
      output.push_str( &format!( "- **Practical significance**: ⚠️ Difference below {:.1}% threshold\n", 
                                 self.practical_significance_threshold * 100.0 ) );
    }

    output.push_str( "\n" );

    // Reliability assessment
    output.push_str( "## Reliability Assessment\n\n" );
    
    if baseline_result.is_reliable() && candidate_result.is_reliable()
    {
      output.push_str( "✅ **Both measurements are statistically reliable** - conclusions can be drawn with confidence.\n\n" );
    }
    else
    {
      output.push_str( "⚠️ **One or both measurements have reliability concerns** - consider additional sampling.\n\n" );
      
      if !baseline_result.is_reliable()
      {
        output.push_str( &format!( "- **{}**: {} samples, CV={:.1}%\n",
                                   self.baseline,
                                   baseline_result.times.len(),
                                   baseline_result.coefficient_of_variation() * 100.0 ) );
      }
      
      if !candidate_result.is_reliable()
      {
        output.push_str( &format!( "- **{}**: {} samples, CV={:.1}%\n",
                                   self.candidate,
                                   candidate_result.times.len(),
                                   candidate_result.coefficient_of_variation() * 100.0 ) );
      }
      
      output.push_str( "\n" );
    }

    // Methodology
    output.push_str( "## Methodology\n\n" );
    output.push_str( &format!( "**Significance Thresholds**: Statistical p < {}, Practical > {:.1}%\n", 
                               self.significance_threshold, 
                               self.practical_significance_threshold * 100.0 ) );
    output.push_str( "**Confidence Intervals**: 95% CI using t-distribution\n" );
    output.push_str( "**Reliability Criteria**: ≥10 samples, CV ≤10%, max/min ratio <3x\n\n" );

    output.push_str( "---\n" );
    output.push_str( "*Generated by benchkit - Professional benchmarking toolkit*\n" );

    Ok( output )
  }
}

/// Custom section for reports
#[ derive( Debug, Clone ) ]
pub struct CustomSection
{
  /// Section title
  pub title : String,
  /// Section content
  pub content : String,
}

impl CustomSection
{
  /// Create new custom section
  #[ must_use ]
  pub fn new( title : impl Into< String >, content : impl Into< String > ) -> Self
  {
    Self
    {
      title : title.into(),
      content : content.into(),
    }
  }
}