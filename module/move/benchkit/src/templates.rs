//! Template system for consistent documentation formatting
//!
//! Provides standardized report templates for common benchmarking scenarios
//! with customizable sections while maintaining professional output quality.

use crate::measurement::BenchmarkResult;
use std::collections::HashMap;
use std::time::SystemTime;

type Result< T > = std::result::Result< T, Box< dyn std::error::Error > >;

/// Historical benchmark results for regression analysis
#[ derive( Debug, Clone ) ]
pub struct HistoricalResults
{
  baseline_data : HashMap< String, BenchmarkResult >,
  historical_runs : Vec< TimestampedResults >,
}

/// Timestamped benchmark results
#[ derive( Debug, Clone ) ]
pub struct TimestampedResults
{
  timestamp : SystemTime,
  results : HashMap< String, BenchmarkResult >,
}

impl TimestampedResults
{
  /// Create new timestamped results
  #[ must_use ]
  pub fn new( timestamp : SystemTime, results : HashMap< String, BenchmarkResult > ) -> Self
  {
    Self { timestamp, results }
  }

  /// Get timestamp
  #[ must_use ]
  pub fn timestamp( &self ) -> SystemTime
  {
    self.timestamp
  }

  /// Get results
  #[ must_use ]
  pub fn results( &self ) -> &HashMap< String, BenchmarkResult >
  {
    &self.results
  }
}

impl HistoricalResults
{
  /// Create new empty historical results
  #[ must_use ]
  pub fn new() -> Self
  {
    Self
    {
      baseline_data : HashMap::new(),
      historical_runs : Vec::new(),
    }
  }

  /// Set baseline data for comparison
  #[ must_use ]
  pub fn with_baseline( mut self, baseline : HashMap< String, BenchmarkResult > ) -> Self
  {
    self.baseline_data = baseline;
    self
  }

  /// Add historical run data
  #[ must_use ]
  pub fn with_historical_run( mut self, timestamp : SystemTime, results : HashMap< String, BenchmarkResult > ) -> Self
  {
    self.historical_runs.push( TimestampedResults::new( timestamp, results ) );
    self
  }

  /// Add multiple historical runs
  #[ must_use ]
  pub fn with_historical_runs( mut self, runs : Vec< TimestampedResults > ) -> Self
  {
    self.historical_runs = runs;
    self
  }

  /// Set the previous run (most recent historical run)
  #[ must_use ]
  pub fn with_previous_run( mut self, run : TimestampedResults ) -> Self
  {
    self.historical_runs = vec![ run ];
    self
  }

  /// Get baseline data
  #[ must_use ]
  pub fn baseline_data( &self ) -> &HashMap< String, BenchmarkResult >
  {
    &self.baseline_data
  }

  /// Get historical runs
  #[ must_use ]
  pub fn historical_runs( &self ) -> &Vec< TimestampedResults >
  {
    &self.historical_runs
  }
}

impl Default for HistoricalResults
{
  fn default() -> Self
  {
    Self::new()
  }
}

/// Baseline strategy for regression analysis
#[ derive( Debug, Clone, PartialEq ) ]
pub enum BaselineStrategy
{
  /// Compare against fixed baseline
  FixedBaseline,
  /// Compare against rolling average of historical runs
  RollingAverage,
  /// Compare against previous run
  PreviousRun,
}

/// Performance trend detected in regression analysis
#[ derive( Debug, Clone, PartialEq ) ]
pub enum PerformanceTrend
{
  /// Performance improving over time
  Improving,
  /// Performance degrading over time
  Degrading,
  /// Performance stable within normal variation
  Stable,
}

/// Regression analysis configuration and engine
#[ derive( Debug, Clone ) ]
pub struct RegressionAnalyzer
{
  /// Statistical significance threshold (default: 0.05)
  significance_threshold : f64,
  /// Number of historical runs to consider for trends (default: 5)
  trend_window : usize,
  /// Strategy for baseline comparison
  baseline_strategy : BaselineStrategy,
}

impl RegressionAnalyzer
{
  /// Create new regression analyzer with default settings
  #[ must_use ]
  pub fn new() -> Self
  {
    Self
    {
      significance_threshold : 0.05,
      trend_window : 5,
      baseline_strategy : BaselineStrategy::FixedBaseline,
    }
  }

  /// Set baseline strategy
  #[ must_use ]
  pub fn with_baseline_strategy( mut self, strategy : BaselineStrategy ) -> Self
  {
    self.baseline_strategy = strategy;
    self
  }

  /// Set significance threshold
  #[ must_use ]
  pub fn with_significance_threshold( mut self, threshold : f64 ) -> Self
  {
    self.significance_threshold = threshold;
    self
  }

  /// Set trend window size
  #[ must_use ]
  pub fn with_trend_window( mut self, window : usize ) -> Self
  {
    self.trend_window = window;
    self
  }

  /// Analyze current results against historical data
  #[ must_use ]
  pub fn analyze( &self, results : &HashMap< String, BenchmarkResult >, historical : &HistoricalResults ) -> RegressionReport
  {
    let mut report = RegressionReport::new();

    for ( operation_name, current_result ) in results
    {
      let analysis = self.analyze_single_operation( operation_name, current_result, historical );
      report.add_operation_analysis( operation_name.clone(), analysis );
    }

    report
  }

  /// Analyze single operation
  fn analyze_single_operation( &self, operation_name : &str, current_result : &BenchmarkResult, historical : &HistoricalResults ) -> OperationAnalysis
  {
    match self.baseline_strategy
    {
      BaselineStrategy::FixedBaseline => self.analyze_against_fixed_baseline( operation_name, current_result, historical ),
      BaselineStrategy::RollingAverage => self.analyze_against_rolling_average( operation_name, current_result, historical ),
      BaselineStrategy::PreviousRun => self.analyze_against_previous_run( operation_name, current_result, historical ),
    }
  }

  /// Analyze against fixed baseline
  fn analyze_against_fixed_baseline( &self, operation_name : &str, current_result : &BenchmarkResult, historical : &HistoricalResults ) -> OperationAnalysis
  {
    if let Some( baseline_result ) = historical.baseline_data().get( operation_name )
    {
      let current_time = current_result.mean_time().as_secs_f64();
      let baseline_time = baseline_result.mean_time().as_secs_f64();
      let improvement_ratio = baseline_time / current_time;
      
      let trend = if improvement_ratio > 1.0 + self.significance_threshold
      {
        PerformanceTrend::Improving
      }
      else if improvement_ratio < 1.0 - self.significance_threshold
      {
        PerformanceTrend::Degrading
      }
      else
      {
        PerformanceTrend::Stable
      };

      let is_significant = ( improvement_ratio - 1.0 ).abs() > self.significance_threshold;

      OperationAnalysis
      {
        trend,
        improvement_ratio,
        is_statistically_significant : is_significant,
        baseline_time : Some( baseline_time ),
        has_historical_data : true,
      }
    }
    else
    {
      OperationAnalysis::no_data()
    }
  }

  /// Analyze against rolling average  
  fn analyze_against_rolling_average( &self, operation_name : &str, current_result : &BenchmarkResult, historical : &HistoricalResults ) -> OperationAnalysis
  {
    let historical_runs = historical.historical_runs();
    if historical_runs.is_empty()
    {
      return OperationAnalysis::no_data();
    }

    // Calculate rolling average from recent runs
    let recent_runs : Vec< _ > = historical_runs
      .iter()
      .rev() // Most recent first
      .take( self.trend_window )
      .filter_map( | run | run.results().get( operation_name ) )
      .collect();

    if recent_runs.is_empty()
    {
      return OperationAnalysis::no_data();
    }

    let avg_time = recent_runs.iter()
      .map( | result | result.mean_time().as_secs_f64() )
      .sum::< f64 >() / recent_runs.len() as f64;

    let current_time = current_result.mean_time().as_secs_f64();
    let improvement_ratio = avg_time / current_time;

    let trend = if improvement_ratio > 1.0 + self.significance_threshold
    {
      PerformanceTrend::Improving
    }
    else if improvement_ratio < 1.0 - self.significance_threshold
    {
      PerformanceTrend::Degrading
    }
    else
    {
      PerformanceTrend::Stable
    };

    let is_significant = ( improvement_ratio - 1.0 ).abs() > self.significance_threshold;

    OperationAnalysis
    {
      trend,
      improvement_ratio,
      is_statistically_significant : is_significant,
      baseline_time : Some( avg_time ),
      has_historical_data : true,
    }
  }

  /// Analyze against previous run
  fn analyze_against_previous_run( &self, operation_name : &str, current_result : &BenchmarkResult, historical : &HistoricalResults ) -> OperationAnalysis
  {
    let historical_runs = historical.historical_runs();
    if let Some( previous_run ) = historical_runs.last()
    {
      if let Some( previous_result ) = previous_run.results().get( operation_name )
      {
        let current_time = current_result.mean_time().as_secs_f64();
        let previous_time = previous_result.mean_time().as_secs_f64();
        let improvement_ratio = previous_time / current_time;

        let trend = if improvement_ratio > 1.0 + self.significance_threshold
        {
          PerformanceTrend::Improving
        }
        else if improvement_ratio < 1.0 - self.significance_threshold
        {
          PerformanceTrend::Degrading
        }
        else
        {
          PerformanceTrend::Stable
        };

        let is_significant = ( improvement_ratio - 1.0 ).abs() > self.significance_threshold;

        OperationAnalysis
        {
          trend,
          improvement_ratio,
          is_statistically_significant : is_significant,
          baseline_time : Some( previous_time ),
          has_historical_data : true,
        }
      }
      else
      {
        OperationAnalysis::no_data()
      }
    }
    else
    {
      OperationAnalysis::no_data()
    }
  }
}

impl Default for RegressionAnalyzer
{
  fn default() -> Self
  {
    Self::new()
  }
}

/// Analysis results for a single operation
#[ derive( Debug, Clone ) ]
pub struct OperationAnalysis
{
  trend : PerformanceTrend,
  improvement_ratio : f64,
  is_statistically_significant : bool,
  baseline_time : Option< f64 >,
  has_historical_data : bool,
}

impl OperationAnalysis
{
  /// Create analysis indicating no historical data available
  #[ must_use ]
  fn no_data() -> Self
  {
    Self
    {
      trend : PerformanceTrend::Stable,
      improvement_ratio : 1.0,
      is_statistically_significant : false,
      baseline_time : None,
      has_historical_data : false,
    }
  }
}

/// Complete regression analysis report
#[ derive( Debug, Clone ) ]
pub struct RegressionReport
{
  operations : HashMap< String, OperationAnalysis >,
}

impl RegressionReport
{
  /// Create new regression report
  #[ must_use ]
  fn new() -> Self
  {
    Self
    {
      operations : HashMap::new(),
    }
  }

  /// Add analysis for an operation
  fn add_operation_analysis( &mut self, operation : String, analysis : OperationAnalysis )
  {
    self.operations.insert( operation, analysis );
  }

  /// Check if any operations have significant changes
  #[ must_use ]
  pub fn has_significant_changes( &self ) -> bool
  {
    self.operations.values().any( | analysis | analysis.is_statistically_significant )
  }

  /// Get trend for specific operation
  #[ must_use ]
  pub fn get_trend_for( &self, operation : &str ) -> Option< PerformanceTrend >
  {
    self.operations.get( operation ).map( | analysis | analysis.trend.clone() )
  }

  /// Check if operation has statistically significant changes
  #[ must_use ]
  pub fn is_statistically_significant( &self, operation : &str ) -> bool
  {
    self.operations.get( operation )
      .is_some_and( | analysis | analysis.is_statistically_significant )
  }

  /// Check if operation has historical data
  #[ must_use ]
  pub fn has_historical_data( &self, operation : &str ) -> bool
  {
    self.operations.get( operation )
      .is_some_and( | analysis | analysis.has_historical_data )
  }

  /// Check if report has previous run data (for PreviousRun strategy)
  #[ must_use ]
  pub fn has_previous_run_data( &self ) -> bool
  {
    self.operations.values().any( | analysis | analysis.has_historical_data )
  }

  /// Format report as markdown
  #[ must_use ]
  pub fn format_markdown( &self ) -> String
  {
    let mut output = String::new();

    output.push_str( "### Performance Comparison Against Baseline\n\n" );

    for ( operation_name, analysis ) in &self.operations
    {
      if !analysis.has_historical_data
      {
        output.push_str( &format!( 
          "**{}**: ℹ️ **New operation** - no baseline data available for comparison\n\n",
          operation_name
        ) );
        continue;
      }

      if let Some( _baseline_time ) = analysis.baseline_time
      {
        let improvement_percent = ( analysis.improvement_ratio - 1.0 ) * 100.0;
        
        match analysis.trend
        {
          PerformanceTrend::Improving =>
          {
            output.push_str( &format!( 
              "**{}**: 🎉 **Performance improvement detected** - {:.1}% faster than baseline\n\n",
              operation_name,
              improvement_percent
            ) );
          },
          PerformanceTrend::Degrading =>
          {
            output.push_str( &format!( 
              "**{}**: ⚠️ **Performance regression detected** - {:.1}% slower than baseline\n\n",
              operation_name,
              improvement_percent.abs()
            ) );
          },
          PerformanceTrend::Stable =>
          {
            output.push_str( &format!( 
              "**{}**: ✅ **Performance stable** - within normal variation of baseline\n\n",
              operation_name
            ) );
          },
        }
      }
    }

    output.push_str( "### Analysis Summary & Recommendations\n\n" );
    output.push_str( "Regression analysis complete. See individual operation results above for detailed findings.\n\n" );

    output
  }
}

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
  /// Historical data for regression analysis
  historical_data : Option< HistoricalResults >,
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
      historical_data : None,
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

  /// Set historical data for regression analysis
  #[ must_use ]
  pub fn with_historical_data( mut self, historical : HistoricalResults ) -> Self
  {
    self.historical_data = Some( historical );
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
  fn add_regression_analysis( &self, output : &mut String, results : &HashMap< String, BenchmarkResult > )
  {
    if let Some( ref historical ) = self.historical_data
    {
      // Use RegressionAnalyzer for enhanced analysis capabilities
      let analyzer = RegressionAnalyzer::new()
        .with_baseline_strategy( BaselineStrategy::FixedBaseline )
        .with_significance_threshold( 0.05 );
      
      let regression_report = analyzer.analyze( results, historical );
      let markdown_output = regression_report.format_markdown();
      
      output.push_str( &markdown_output );

      // Add enhanced recommendations with more context
      self.add_enhanced_recommendations( output, &regression_report, results );
    }
    else
    {
      // Fallback to placeholder when no historical data available
      output.push_str( "**Regression Analysis**: Not yet implemented. Historical baseline data required.\n\n" );
      output.push_str( "**📖 Setup Guide**: See [`recommendations.md`](recommendations.md) for comprehensive guidelines on:\n" );
      output.push_str( "- Historical data collection and baseline management\n" );
      output.push_str( "- Statistical analysis requirements and validation criteria\n" );
      output.push_str( "- Integration with CI/CD pipelines for automated regression detection\n" );
      output.push_str( "- Documentation automation best practices\n\n" );
    }
  }


  /// Add enhanced recommendations based on regression report
  fn add_enhanced_recommendations( &self, output : &mut String, regression_report : &RegressionReport, results : &HashMap< String, BenchmarkResult > )
  {
    // Collect operations by trend for enhanced reporting
    let mut improving_ops = Vec::new();
    let mut degrading_ops = Vec::new();
    let mut stable_ops = Vec::new();
    let mut new_ops = Vec::new();

    for operation_name in results.keys()
    {
      match regression_report.get_trend_for( operation_name )
      {
        Some( PerformanceTrend::Improving ) =>
        {
          if regression_report.is_statistically_significant( operation_name )
          {
            improving_ops.push( operation_name );
          }
        },
        Some( PerformanceTrend::Degrading ) =>
        {
          if regression_report.is_statistically_significant( operation_name )
          {
            degrading_ops.push( operation_name );
          }
        },
        Some( PerformanceTrend::Stable ) =>
        {
          stable_ops.push( operation_name );
        },
        None =>
        {
          if !regression_report.has_historical_data( operation_name )
          {
            new_ops.push( operation_name );
          }
        },
      }
    }

    if !improving_ops.is_empty() || !degrading_ops.is_empty() || regression_report.has_significant_changes()
    {
      output.push_str( "### 📊 **Statistical Analysis Summary**\n\n" );
      
      if regression_report.has_significant_changes()
      {
        output.push_str( "**Statistically Significant Changes Detected**: This analysis identified performance changes that exceed normal measurement variance.\n\n" );
      }
      else
      {
        output.push_str( "**No Statistically Significant Changes**: All performance variations are within expected measurement noise.\n\n" );
      }
    }

    if !improving_ops.is_empty()
    {
      output.push_str( "### 🎯 **Performance Optimization Insights**\n\n" );
      output.push_str( "The following operations show statistically significant improvements:\n" );
      for op in &improving_ops
      {
        output.push_str( &format!( "- **{}**: Consider documenting optimization techniques for knowledge sharing\n", op ) );
      }
      output.push_str( "\n**Next Steps**: Update performance baselines and validate improvements under production conditions.\n\n" );
    }

    if !degrading_ops.is_empty()
    {
      output.push_str( "### ⚠️ **Regression Investigation Required**\n\n" );
      output.push_str( "**Critical**: The following operations show statistically significant performance degradation:\n" );
      for op in &degrading_ops
      {
        output.push_str( &format!( "- **{}**: Requires immediate investigation\n", op ) );
      }
      output.push_str( "\n**Recommended Actions**:\n" );
      output.push_str( "1. **Profile regressed operations** to identify bottlenecks\n" );
      output.push_str( "2. **Review recent code changes** affecting these operations\n" );
      output.push_str( "3. **Run additional validation** with increased sample sizes\n" );
      output.push_str( "4. **Consider deployment hold** until regressions are resolved\n\n" );
    }

    // Add project-specific recommendations
    output.push_str( "### 🔗 **Integration Resources**\n\n" );
    output.push_str( "For enhanced regression analysis capabilities:\n" );
    output.push_str( "- **Configure baseline strategies**: Use `RegressionAnalyzer::with_baseline_strategy()` for rolling averages or previous-run comparisons\n" );
    output.push_str( "- **Adjust significance thresholds**: Use `with_significance_threshold()` for domain-specific sensitivity\n" );
    output.push_str( "- **Historical data management**: Implement `TimestampedResults` for comprehensive trend analysis\n" );
    output.push_str( "- **Automated monitoring**: Integrate with CI/CD pipelines for continuous performance validation\n\n" );
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