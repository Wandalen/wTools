//! Context-rich benchmark documentation generator for comprehensive reporting
//!
//! Implements benchkit usage.md "Write Context-Rich Reports" section requirements:
//! - Measurement specifications clearly stated before results
//! - Before/After optimization comparisons where applicable  
//! - Key findings and insights included with results
//! - Actionable recommendations provided
//! - Environment specifications documented

/// Internal namespace.
mod private
{
  #[ cfg( feature = "benchmarks" ) ]
  use std::fmt::Write;
  #[ cfg( feature = "benchmarks" ) ]
  use crate::comparative_benchmark_structure::ComparativeResults;

  /// Benchmark measurement context for documentation
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug, Clone ) ]
  pub struct BenchmarkMeasurementContext
  {
    /// What is being measured (e.g., "Cache-friendly optimization algorithms on dataset of 50K records")
    pub what_is_measured : String,
    /// How to reproduce measurements (e.g., "cargo bench --bench cache_optimizations --features large_datasets")
    pub how_to_measure : String,
    /// Environment specifications
    pub environment : EnvironmentContext,
    /// Purpose or objective of the benchmark
    pub purpose : String,
  }

  /// Environment context for reproducible benchmarks
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug, Clone ) ]
  pub struct EnvironmentContext  
  {
    /// CPU information
    pub cpu : String,
    /// RAM amount
    pub ram : String,
    /// Storage type
    pub storage : String,
    /// Load characteristics
    pub load_characteristics : String,
    /// Additional environment notes
    pub notes : Vec< String >,
  }

  /// Status indicator for optimization progress
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug, Clone, PartialEq ) ]
  pub enum OptimizationStatus
  {
    /// Optimization completed successfully
    Optimized,
    /// Optimization in progress or needed
    NeedsWork,
    /// Production ready
    ProductionReady,
    /// Baseline measurement
    Baseline,
    /// Regression detected
    Regression,
  }

  #[ cfg( feature = "benchmarks" ) ]
  impl OptimizationStatus
  {
    /// Get emoji indicator for status
    pub fn indicator( &self ) -> &'static str
    {
      match self
      {
        Self::Optimized => "✅",
        Self::NeedsWork => "⚠️",
        Self::ProductionReady => "🚀", 
        Self::Baseline => "📊",
        Self::Regression => "❌",
      }
    }

    /// Get status description
    pub fn description( &self ) -> &'static str
    {
      match self
      {
        Self::Optimized => "Optimized",
        Self::NeedsWork => "Needs work",
        Self::ProductionReady => "Production ready",
        Self::Baseline => "Baseline", 
        Self::Regression => "Regression",
      }
    }
  }

  /// Before/after comparison data for optimization tracking
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug, Clone ) ]
  pub struct BeforeAfterComparison
  {
    /// Algorithm name
    pub algorithm_name : String,
    /// Performance before optimization (nanoseconds)
    pub before_nanos : f64,
    /// Performance after optimization (nanoseconds)
    pub after_nanos : f64,
    /// Status indicator
    pub status : OptimizationStatus,
  }

  #[ cfg( feature = "benchmarks" ) ]
  impl BeforeAfterComparison
  {
    /// Calculate improvement percentage
    pub fn improvement_percentage( &self ) -> f64
    {
      if self.before_nanos == 0.0
      {
        0.0
      }
      else
      {
        ( ( self.before_nanos - self.after_nanos ) / self.before_nanos ) * 100.0
      }
    }

    /// Format improvement as human-readable string
    pub fn format_improvement( &self ) -> String
    {
      let improvement = self.improvement_percentage();
      if improvement > 0.0
      {
        format!( "{:.1}% faster", improvement )
      }
      else if improvement < 0.0
      {
        format!( "{:.1}% slower", improvement.abs() )
      }
      else
      {
        "No change".to_string()
      }
    }

    /// Format time in human-readable units
    pub fn format_time( time_nanos : f64 ) -> String
    {
      let time_ms = time_nanos / 1_000_000.0;
      
      if time_ms >= 1000.0
      {
        format!( "{:.2}s", time_ms / 1000.0 )
      }
      else if time_ms >= 0.1
      {
        format!( "{:.2}ms", time_ms )
      }
      else
      {
        let time_us = time_nanos / 1_000.0;
        format!( "{:.2}µs", time_us )
      }
    }
  }

  /// Context-rich documentation generator
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug ) ]
  pub struct ContextRichDocGenerator
  {
    /// Environment context for all benchmarks
    environment : EnvironmentContext,
    /// Generated documentation sections
    sections : Vec< String >,
  }

  #[ cfg( feature = "benchmarks" ) ]
  impl ContextRichDocGenerator
  {
    /// Create new documentation generator with environment context
    pub fn new( environment : EnvironmentContext ) -> Self
    {
      Self
      {
        environment,
        sections : Vec::new(),
      }
    }

    /// Create default documentation generator for typical development environment
    pub fn default_environment() -> Self
    {
      let environment = EnvironmentContext
      {
        cpu : "x86_64 CPU".to_string(),
        ram : "16GB RAM".to_string(),
        storage : "SSD storage".to_string(),
        load_characteristics : "typical development load".to_string(),
        notes : vec![ "Results may vary in production environments".to_string() ],
      };
      
      Self::new( environment )
    }

    /// Get reference to environment context
    pub fn environment( &self ) -> &EnvironmentContext
    {
      &self.environment
    }

    /// Get number of generated sections
    pub fn section_count( &self ) -> usize
    {
      self.sections.len()
    }

    /// Add comparative benchmark results with context
    pub fn add_comparative_results( 
      &mut self, 
      context : BenchmarkMeasurementContext, 
      results : &ComparativeResults 
    )
    {
      let mut section = String::new();
      
      // Title and context
      writeln!( &mut section, "## {} Performance Analysis", results.benchmark_name ).unwrap();
      writeln!( &mut section, "" ).unwrap();
      writeln!( &mut section, "**What is measured**: {}", context.what_is_measured ).unwrap();
      writeln!( &mut section, "**How to measure**: `{}`", context.how_to_measure ).unwrap();
      writeln!( &mut section, "**Purpose**: {}", context.purpose ).unwrap();
      writeln!( &mut section, "" ).unwrap();
      
      // Results table
      writeln!( &mut section, "### Performance Comparison Results" ).unwrap();
      writeln!( &mut section, "" ).unwrap();
      writeln!( &mut section, "| Algorithm | Average Time | Std Dev | Relative Performance | Status |" ).unwrap();
      writeln!( &mut section, "|-----------|--------------|---------|---------------------|---------|" ).unwrap();
      
      for result in &results.results
      {
        let relative = result.relative_performance( results.baseline_time );
        let status = if relative <= 1.0 
        { 
          OptimizationStatus::ProductionReady 
        } 
        else if relative <= 1.2 
        { 
          OptimizationStatus::Optimized 
        } 
        else 
        { 
          OptimizationStatus::NeedsWork 
        };
        
        let performance_text = if relative <= 1.0
        {
          "1.00x (baseline)".to_string()
        }
        else
        {
          format!( "{:.2}x slower", relative )
        };
        
        writeln!(
          &mut section,
          "| {} | {} | {:.2}µs | {} | {} {} |",
          result.algorithm_name,
          result.format_time(),
          result.std_dev_nanos / 1000.0,
          performance_text,
          status.indicator(),
          status.description()
        ).unwrap();
      }
      
      writeln!( &mut section, "" ).unwrap();
      
      // Key findings
      self.add_key_findings( &mut section, results );
      
      // Environment specification
      self.add_environment_spec( &mut section );
      
      self.sections.push( section );
    }

    /// Add before/after optimization comparison
    pub fn add_before_after_comparison(
      &mut self,
      title : &str,
      context : BenchmarkMeasurementContext,
      comparisons : &[ BeforeAfterComparison ]
    )
    {
      let mut section = String::new();
      
      // Title and context
      writeln!( &mut section, "## {}", title ).unwrap();
      writeln!( &mut section, "" ).unwrap();
      writeln!( &mut section, "**What is measured**: {}", context.what_is_measured ).unwrap();
      writeln!( &mut section, "**How to measure**: `{}`", context.how_to_measure ).unwrap();
      writeln!( &mut section, "" ).unwrap();
      writeln!( &mut section, "Performance comparison after implementing optimizations:" ).unwrap();
      writeln!( &mut section, "" ).unwrap();
      
      // Before/after table
      writeln!( &mut section, "| Algorithm | Before | After | Improvement | Status |" ).unwrap();
      writeln!( &mut section, "|-----------|---------|--------|-------------|---------|" ).unwrap();
      
      for comparison in comparisons
      {
        writeln!(
          &mut section,
          "| {} | {} | {} | {} | {} {} |",
          comparison.algorithm_name,
          BeforeAfterComparison::format_time( comparison.before_nanos ),
          BeforeAfterComparison::format_time( comparison.after_nanos ),
          comparison.format_improvement(),
          comparison.status.indicator(),
          comparison.status.description()
        ).unwrap();
      }
      
      writeln!( &mut section, "" ).unwrap();
      
      // Analysis and recommendations
      self.add_optimization_analysis( &mut section, comparisons );
      
      // Environment specification
      self.add_environment_spec( &mut section );
      
      self.sections.push( section );
    }

    fn add_key_findings( &self, section : &mut String, results : &ComparativeResults )
    {
      writeln!( section, "### Key Findings" ).unwrap();
      writeln!( section, "" ).unwrap();
      
      let performance_range = results.performance_range();
      if performance_range > 2.0
      {
        writeln!( 
          section, 
          "**Significant Performance Differences**: Algorithms show up to {:.1}x performance variation.",
          performance_range
        ).unwrap();
      }
      
      writeln!( 
        section, 
        "**Winner**: {} provides the best performance and is recommended for production use.", 
        results.fastest_algorithm 
      ).unwrap();
      
      if results.has_significant_differences( 1.5 )
      {
        writeln!( section, "**Optimization Opportunity**: Some algorithms could benefit from performance improvements." ).unwrap();
      }
      
      writeln!( section, "" ).unwrap();
    }

    fn add_optimization_analysis( &self, section : &mut String, comparisons : &[ BeforeAfterComparison ] )
    {
      writeln!( section, "### Analysis & Recommendations" ).unwrap();
      writeln!( section, "" ).unwrap();
      
      let optimized_count = comparisons.iter().filter( | c | c.improvement_percentage() > 5.0 ).count();
      let total_count = comparisons.len();
      
      if optimized_count > 0
      {
        writeln!( 
          section, 
          "**Optimization Success**: {}/{} algorithms showed meaningful improvements (>5%).", 
          optimized_count, total_count
        ).unwrap();
      }
      
      let needs_work : Vec< _ > = comparisons.iter()
        .filter( | c | c.status == OptimizationStatus::NeedsWork )
        .map( | c | c.algorithm_name.as_str() )
        .collect();
        
      if !needs_work.is_empty()
      {
        writeln!( 
          section, 
          "**Action Required**: {} algorithm(s) need optimization work: {}.", 
          needs_work.len(),
          needs_work.join( ", " )
        ).unwrap();
      }
      
      // Next steps
      writeln!( section, "" ).unwrap();
      writeln!( section, "**Next Steps**:" ).unwrap();
      
      if !needs_work.is_empty()
      {
        writeln!( section, "- Investigate optimization opportunities for underperforming algorithms" ).unwrap();
      }
      
      let production_ready : Vec< _ > = comparisons.iter()
        .filter( | c | c.status == OptimizationStatus::ProductionReady )
        .map( | c | c.algorithm_name.as_str() )
        .collect();
        
      if !production_ready.is_empty()
      {
        writeln!( section, "- Deploy optimized algorithms: {}", production_ready.join( ", " ) ).unwrap();
      }
      
      writeln!( section, "- Monitor performance in production environment" ).unwrap();
      writeln!( section, "" ).unwrap();
    }

    fn add_environment_spec( &self, section : &mut String )
    {
      writeln!( section, "### Environment Specification" ).unwrap();
      writeln!( section, "" ).unwrap();
      writeln!( section, "**Hardware**: {}, {}, {}", self.environment.cpu, self.environment.ram, self.environment.storage ).unwrap();
      writeln!( section, "**Load**: {}", self.environment.load_characteristics ).unwrap();
      
      if !self.environment.notes.is_empty()
      {
        writeln!( section, "**Notes**: {}", self.environment.notes.join( "; " ) ).unwrap();
      }
      
      writeln!( section, "" ).unwrap();
    }

    /// Generate complete documentation report
    pub fn generate_report( &self, title : &str ) -> String
    {
      let mut report = String::new();
      
      writeln!( &mut report, "# {}", title ).unwrap();
      writeln!( &mut report, "" ).unwrap();
      writeln!( 
        &mut report, 
        "*Generated on {} with context-rich benchmark documentation*", 
        chrono::Utc::now().format( "%Y-%m-%d %H:%M:%S UTC" )
      ).unwrap();
      writeln!( &mut report, "" ).unwrap();
      
      for section in &self.sections
      {
        writeln!( &mut report, "{}", section ).unwrap();
      }
      
      writeln!( &mut report, "---" ).unwrap();
      writeln!( &mut report, "" ).unwrap();
      writeln!( &mut report, "*This report provides context-rich benchmark documentation following benchkit standards.*" ).unwrap();
      
      report
    }

    /// Clear all sections for reuse
    pub fn clear_sections( &mut self )
    {
      self.sections.clear();
    }
  }

  /// Default implementation provides typical development environment
  #[ cfg( feature = "benchmarks" ) ]
  impl Default for ContextRichDocGenerator
  {
    fn default() -> Self
    {
      Self::default_environment()
    }
  }
}

mod_interface::mod_interface!
{
  #[ cfg( feature = "benchmarks" ) ]
  orphan use BenchmarkMeasurementContext;
  #[ cfg( feature = "benchmarks" ) ]
  orphan use EnvironmentContext;
  #[ cfg( feature = "benchmarks" ) ]
  orphan use OptimizationStatus;
  #[ cfg( feature = "benchmarks" ) ]
  orphan use BeforeAfterComparison;
  #[ cfg( feature = "benchmarks" ) ]
  orphan use ContextRichDocGenerator;
}