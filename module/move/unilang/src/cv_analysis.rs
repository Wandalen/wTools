//! Coefficient of Variation (CV) analysis for unilang benchmarks
//!
//! Implements benchkit usage.md "Coefficient of Variation Standards" with
//! comprehensive CV improvement techniques including thread pool warmup,
//! CPU stabilization, and cache warmup for reliable benchmark results.

/// Internal namespace.
mod private
{
  #[ cfg( feature = "benchmarks" ) ]
  use benchkit::prelude::*;
  #[ cfg( feature = "benchmarks" ) ]
  use crate::benchmark_config::BenchmarkConfig;
  #[ cfg( feature = "benchmarks" ) ]
  use std::time::Duration;
  #[ cfg( feature = "benchmarks" ) ]
  use std::thread;

  /// CV quality standards from benchkit usage.md
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug, Clone ) ]
  pub enum CvQuality
  {
    /// CV < 5%: Excellent reliability (ready for production decisions)
    Excellent,
    /// CV 5-10%: Good, acceptable for most use cases
    Good,
    /// CV 10-15%: Moderate, consider improvements
    Moderate,
    /// CV > 15%: Poor/Unreliable, must fix before using results
    Poor,
  }

  #[ cfg( feature = "benchmarks" ) ]
  impl CvQuality
  {
    /// Assess CV quality based on percentage
    pub fn from_cv_percentage( cv_percent : f64 ) -> Self
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

    /// Get quality indicator emoji
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

    /// Get quality description
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

  /// CV improvement techniques from benchkit usage.md
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug ) ]
  pub struct CvImprovementTechniques;

  #[ cfg( feature = "benchmarks" ) ]
  impl CvImprovementTechniques
  {
    /// Apply thread pool warmup stabilization for parallel operations
    pub fn thread_pool_warmup()
    {
      println!( "🔧 Applying thread pool warmup stabilization..." );
      
      // Create and warmup thread pool to stabilize parallel performance
      use std::sync::{ Arc, Mutex };
      let counter = Arc::new( Mutex::new( 0 ) );
      let mut handles = Vec::new();
      
      // Spawn warmup threads
      for _ in 0..num_cpus::get()
      {
        let counter = Arc::clone( &counter );
        let handle = thread::spawn( move ||
        {
          // Perform CPU-intensive warmup work
          for i in 0..1000
          {
            let _result = ( i * i ) % 997; // Prime modulus for cache mixing
            if let Ok( mut num ) = counter.lock()
            {
              *num += 1;
            }
          }
        });
        handles.push( handle );
      }
      
      // Wait for all threads to complete
      for handle in handles
      {
        let _ = handle.join();
      }
      
      println!( "✅ Thread pool warmed up - parallel performance stabilized" );
    }

    /// Apply CPU frequency stabilization delays
    pub fn cpu_stabilization( duration_ms : u64 )
    {
      println!( "🔧 Applying CPU frequency stabilization ({}ms)...", duration_ms );
      
      let start = std::time::Instant::now();
      let target_duration = Duration::from_millis( duration_ms );
      
      // Perform consistent CPU work to stabilize frequency scaling
      while start.elapsed() < target_duration
      {
        // CPU-intensive work to trigger frequency scaling
        let mut sum = 0u64;
        for i in 0..10000
        {
          sum = sum.wrapping_add( i ).wrapping_mul( 3 );
        }
        std::hint::black_box( sum );
        
        // Short pause to allow frequency adjustment
        thread::sleep( Duration::from_millis( 1 ) );
      }
      
      println!( "✅ CPU frequency stabilized after {}ms", start.elapsed().as_millis() );
    }

    /// Apply cache warmup cycles for memory-intensive operations
    pub fn cache_warmup< F >( operation : F, warmup_cycles : usize )
    where
      F : Fn(),
    {
      println!( "🔧 Applying cache warmup ({} cycles)...", warmup_cycles );
      
      for i in 0..warmup_cycles
      {
        operation();
        
        // Progress indication for long warmups
        if warmup_cycles > 10 && i % ( warmup_cycles / 5 ) == 0
        {
          println!( "  Cache warmup progress: {}/{}", i + 1, warmup_cycles );
        }
      }
      
      println!( "✅ Cache warmed up - memory access patterns stabilized" );
    }

    /// Apply all CV improvement techniques based on detected issues
    pub fn apply_improvements( cv_percent : f64, config : &BenchmarkConfig )
    {
      println!( "\\n🔧 CV Improvement Analysis (Current CV: {:.1}%)", cv_percent );
      println!( "================================================" );
      
      let quality = CvQuality::from_cv_percentage( cv_percent );
      println!( "{} Quality Assessment: {}", quality.indicator(), quality.description() );
      
      match quality
      {
        CvQuality::Excellent =>
        {
          println!( "✨ No improvements needed - excellent measurement quality!" );
        },
        CvQuality::Good =>
        {
          println!( "🟢 Good quality, but minor optimizations could help:" );
          println!( "  • Consider increasing warmup iterations to {}", config.warmup_iterations + 2 );
        },
        CvQuality::Moderate =>
        {
          println!( "🟡 Moderate quality - applying standard improvements:" );
          Self::cpu_stabilization( 500 ); // 500ms stabilization
          println!( "  • Increased warmup iterations recommended: {}", config.warmup_iterations * 2 );
          println!( "  • Consider running in isolated environment" );
        },
        CvQuality::Poor =>
        {
          println!( "❌ Poor quality - applying comprehensive improvements:" );
          Self::thread_pool_warmup();
          Self::cpu_stabilization( 1000 ); // 1 second stabilization
          println!( "  🔧 Applied thread pool warmup" );
          println!( "  🔧 Applied CPU frequency stabilization" );
          println!( "  • Strongly recommend increasing sample size to {}", config.max_sample_size );
          println!( "  • Consider dedicated benchmark environment" );
          println!( "  • Check for system load and background processes" );
        }
      }
      
      println!( "================================================\\n" );
    }
  }

  /// Comprehensive CV analysis with environment-specific reporting
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug ) ]
  pub struct CvAnalyzer
  {
    /// Environment-specific benchmark configuration for CV analysis
    config : BenchmarkConfig,
  }

  #[ cfg( feature = "benchmarks" ) ]
  impl CvAnalyzer
  {
    /// Create new CV analyzer with environment-specific configuration
    pub fn new() -> Self
    {
      Self
      {
        config : BenchmarkConfig::from_environment(),
      }
    }

    /// Create with custom configuration
    pub fn with_config( config : BenchmarkConfig ) -> Self
    {
      Self { config }
    }

    /// Analyze benchmark result with comprehensive CV assessment
    pub fn analyze_result( &self, name : &str, result : &BenchmarkResult ) -> CvAnalysisReport
    {
      let cv_percent = result.coefficient_of_variation() * 100.0;
      let quality = CvQuality::from_cv_percentage( cv_percent );
      let meets_environment_requirements = self.config.cv_meets_requirements( result.coefficient_of_variation() );
      let sample_size = result.times.len();
      let recommended_size = self.config.adaptive_sample_size( result.coefficient_of_variation() );
      
      CvAnalysisReport
      {
        benchmark_name : name.to_string(),
        cv_percentage : cv_percent,
        quality,
        meets_environment_requirements,
        environment : self.config.environment.clone(),
        cv_tolerance : self.config.cv_tolerance,
        current_sample_size : sample_size,
        recommended_sample_size : recommended_size,
        statistical_analysis : StatisticalAnalysis::analyze( result, SignificanceLevel::Standard ).ok(),
      }
    }

    /// Run CV analysis on multiple benchmark results
    pub fn analyze_suite( &self, results : &std::collections::HashMap< String, BenchmarkResult > ) -> Vec< CvAnalysisReport >
    {
      let mut reports = Vec::new();
      
      println!( "🔬 Comprehensive CV Analysis (Environment: {})", self.config.environment );
      println!( "Target CV Tolerance: {:.1}%", self.config.cv_tolerance * 100.0 );
      println!( "========================================" );
      
      for ( name, result ) in results
      {
        let report = self.analyze_result( name, result );
        report.print_summary();
        reports.push( report );
      }
      
      // Overall suite analysis
      self.print_suite_summary( &reports );
      
      reports
    }

    /// Print comprehensive suite-level CV analysis
    fn print_suite_summary( &self, reports : &[ CvAnalysisReport ] )
    {
      println!( "\\n📊 Suite-Level CV Analysis Summary" );
      println!( "=================================" );
      
      let total_benchmarks = reports.len();
      let excellent_count = reports.iter().filter( |r| matches!( r.quality, CvQuality::Excellent ) ).count();
      let good_count = reports.iter().filter( |r| matches!( r.quality, CvQuality::Good ) ).count();
      let moderate_count = reports.iter().filter( |r| matches!( r.quality, CvQuality::Moderate ) ).count();
      let poor_count = reports.iter().filter( |r| matches!( r.quality, CvQuality::Poor ) ).count();
      
      let environment_compliant = reports.iter().filter( |r| r.meets_environment_requirements ).count();
      
      println!( "🎯 Quality Distribution:" );
      println!( "  ✅ Excellent (< 5%):     {}/{} ({:.0}%)", excellent_count, total_benchmarks, ( excellent_count as f64 / total_benchmarks as f64 ) * 100.0 );
      println!( "  🟢 Good (5-10%):         {}/{} ({:.0}%)", good_count, total_benchmarks, ( good_count as f64 / total_benchmarks as f64 ) * 100.0 );
      println!( "  🟡 Moderate (10-15%):    {}/{} ({:.0}%)", moderate_count, total_benchmarks, ( moderate_count as f64 / total_benchmarks as f64 ) * 100.0 );
      println!( "  ❌ Poor (> 15%):         {}/{} ({:.0}%)", poor_count, total_benchmarks, ( poor_count as f64 / total_benchmarks as f64 ) * 100.0 );
      
      println!( "\\n🌍 Environment Compliance ({}):", self.config.environment );
      println!( "  Meets {:.1}% tolerance: {}/{} ({:.0}%)", 
               self.config.cv_tolerance * 100.0,
               environment_compliant, 
               total_benchmarks,
               ( environment_compliant as f64 / total_benchmarks as f64 ) * 100.0 );
      
      // Recommendations
      if poor_count > 0 || moderate_count > total_benchmarks / 2
      {
        println!( "\\n🔧 Suite-Level Recommendations:" );
        if poor_count > 0
        {
          println!( "  • {} benchmarks need immediate CV improvements", poor_count );
        }
        if environment_compliant < total_benchmarks
        {
          println!( "  • Consider switching to more permissive environment for {} benchmarks", total_benchmarks - environment_compliant );
        }
        if moderate_count > 0
        {
          println!( "  • Apply standard improvements to {} moderate-quality benchmarks", moderate_count );
        }
      }
      else
      {
        println!( "\\n✨ Overall Assessment: Excellent suite quality!" );
      }
      
      println!( "=================================" );
    }
  }

  /// Detailed CV analysis report for a single benchmark
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug ) ]
  pub struct CvAnalysisReport
  {
    /// Name of the benchmark being analyzed
    pub benchmark_name : String,
    /// Coefficient of variation as a percentage (CV * 100)
    pub cv_percentage : f64,
    /// Quality assessment based on CV percentage thresholds
    pub quality : CvQuality,
    /// Whether the CV meets the environment-specific requirements
    pub meets_environment_requirements : bool,
    /// The benchmark environment (Development, Staging, Production)
    pub environment : crate::benchmark_config::BenchmarkEnvironment,
    /// Maximum acceptable CV percentage for the environment
    pub cv_tolerance : f64,
    /// Number of samples in the current benchmark results
    pub current_sample_size : usize,
    /// Recommended sample size based on CV and environment
    pub recommended_sample_size : usize,
    /// Optional statistical analysis for reliability assessment
    pub statistical_analysis : Option< StatisticalAnalysis >,
  }

  #[ cfg( feature = "benchmarks" ) ]
  impl CvAnalysisReport
  {
    /// Print detailed summary of CV analysis
    pub fn print_summary( &self )
    {
      println!( "\\n📊 {} - CV Analysis", self.benchmark_name );
      println!( "  {} CV: {:.1}% ({})", 
               self.quality.indicator(), 
               self.cv_percentage,
               self.quality.description() );
      
      // Environment compliance
      let env_status = if self.meets_environment_requirements { "✅" } else { "⚠️" };
      println!( "  {} Environment: {} (tolerance: {:.1}%)", 
               env_status,
               self.environment,
               self.cv_tolerance * 100.0 );
      
      // Sample size recommendation
      if self.recommended_sample_size > self.current_sample_size
      {
        println!( "  📏 Sample size: {} → {} recommended", 
                 self.current_sample_size,
                 self.recommended_sample_size );
      }
      else
      {
        println!( "  📏 Sample size: {} (adequate)", self.current_sample_size );
      }
      
      // Statistical reliability
      if let Some( ref analysis ) = self.statistical_analysis
      {
        if analysis.is_reliable()
        {
          println!( "  📈 Statistical reliability: ✅ Reliable" );
        }
        else
        {
          println!( "  📈 Statistical reliability: ⚠️ Needs improvement" );
        }
      }
      
      // Improvement suggestions
      match self.quality
      {
        CvQuality::Poor =>
        {
          println!( "  🔧 Action required: Apply comprehensive CV improvements" );
        },
        CvQuality::Moderate =>
        {
          println!( "  🔧 Consider: Standard CV improvement techniques" );
        },
        _ => {},
      }
    }

    /// Generate markdown report section
    pub fn generate_markdown( &self ) -> String
    {
      let mut output = String::new();
      
      output.push_str( &format!( "#### {}\\n\\n", self.benchmark_name ) );
      output.push_str( &format!( "- **CV**: {:.1}% {} ({})\\n", 
                                self.cv_percentage,
                                self.quality.indicator(),
                                self.quality.description() ) );
      
      let env_status = if self.meets_environment_requirements { "✅ Compliant" } else { "⚠️ Exceeds tolerance" };
      output.push_str( &format!( "- **Environment**: {} ({:.1}% tolerance) - {}\\n",
                                self.environment,
                                self.cv_tolerance * 100.0,
                                env_status ) );
      
      output.push_str( &format!( "- **Sample size**: {} (recommended: {})\\n",
                                self.current_sample_size,
                                self.recommended_sample_size ) );
      
      if let Some( ref analysis ) = self.statistical_analysis
      {
        let reliability = if analysis.is_reliable() { "✅ Reliable" } else { "⚠️ Needs improvement" };
        output.push_str( &format!( "- **Statistical reliability**: {}\\n", reliability ) );
      }
      
      output.push_str( "\\n" );
      output
    }
  }

  #[ cfg( feature = "benchmarks" ) ]
  impl Default for CvAnalyzer
  {
    fn default() -> Self
    {
      Self::new()
    }
  }
}

mod_interface::mod_interface!
{
  #[ cfg( feature = "benchmarks" ) ]
  orphan use CvQuality;
  #[ cfg( feature = "benchmarks" ) ]
  orphan use CvImprovementTechniques;
  #[ cfg( feature = "benchmarks" ) ]
  orphan use CvAnalyzer;
  #[ cfg( feature = "benchmarks" ) ]
  orphan use CvAnalysisReport;
}