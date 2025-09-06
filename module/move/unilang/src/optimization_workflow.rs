//! Before/after optimization workflow system for systematic performance tracking
//!
//! Implements benchkit usage.md "Before/After Optimization Workflow" requirements:
//! - 3-step systematic workflow: baseline -> optimize -> measure impact  
//! - Baseline establishment and persistence
//! - Performance regression detection and reporting
//! - Statistical significance validation of improvements
//! - Automatic documentation updates at each step

/// Internal namespace.
mod private
{
  #[ cfg( feature = "benchmarks" ) ]
  use std::fs;
  #[ cfg( feature = "benchmarks" ) ]
  use std::path::{ Path, PathBuf };
  #[ cfg( feature = "benchmarks" ) ]
  use serde::{ Serialize, Deserialize };
  #[ cfg( feature = "benchmarks" ) ]
  use crate::
  {
    BenchmarkResult,
    ContextRichDocGenerator,
    BenchmarkMeasurementContext,
    BeforeAfterComparison,
    OptimizationStatus,
  };

  /// Simple CV analysis for optimization workflow
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug, Clone, serde::Serialize, serde::Deserialize ) ]
  pub struct CoefficientsOfVariationAnalysis
  {
    /// CV values for each algorithm
    pub cv_values : Vec< f64 >,
    /// Analysis name
    pub analysis_name : String,
  }

  #[ cfg( feature = "benchmarks" ) ]
  impl CoefficientsOfVariationAnalysis
  {
    /// Create new CV analysis
    pub fn new( cv_values : Vec< f64 >, analysis_name : String ) -> Self
    {
      Self
      {
        cv_values,
        analysis_name,
      }
    }

    /// Assess overall quality based on average CV
    pub fn overall_quality_assessment( &self ) -> String
    {
      if self.cv_values.is_empty()
      {
        return "Unknown".to_string();
      }

      let average_cv = self.cv_values.iter().sum::< f64 >() / self.cv_values.len() as f64;
      
      if average_cv < 5.0
      {
        "Excellent".to_string()
      }
      else if average_cv < 10.0
      {
        "Good".to_string()
      }
      else if average_cv < 15.0
      {
        "Moderate".to_string()
      }
      else
      {
        "Poor".to_string()
      }
    }
  }

  /// Baseline benchmark results for comparison
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct BaselineResults
  {
    /// Timestamp when baseline was established
    pub timestamp : String,
    /// Benchmark name/identifier
    pub benchmark_name : String,
    /// Environment description
    pub environment_info : String,
    /// Individual benchmark results
    pub results : Vec< BenchmarkResult >,
    /// Coefficient of variation for baseline quality
    pub cv_analysis : CoefficientsOfVariationAnalysis,
    /// Notes about baseline conditions
    pub notes : Vec< String >,
  }

  /// Optimization impact comparison results
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug, Clone ) ]
  pub struct OptimizationImpact
  {
    /// Baseline results for comparison
    pub baseline : BaselineResults,
    /// Current results after optimization
    pub current_results : Vec< BenchmarkResult >,
    /// Before/after comparisons for each algorithm
    pub comparisons : Vec< BeforeAfterComparison >,
    /// Statistical significance indicators
    pub significance_analysis : SignificanceAnalysis,
    /// Overall optimization summary
    pub summary : OptimizationSummary,
  }

  /// Statistical significance analysis for optimization validation
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug, Clone ) ]
  pub struct SignificanceAnalysis
  {
    /// Number of algorithms with significant improvements (>5%)
    pub significant_improvements : usize,
    /// Number of algorithms with regressions (>5% slower)
    pub regressions : usize,
    /// Total algorithms tested
    pub total_algorithms : usize,
    /// Average improvement percentage across all algorithms
    pub average_improvement : f64,
    /// Quality assessment based on CV analysis
    pub baseline_quality : String,
    /// Current measurement quality
    pub current_quality : String,
  }

  /// Summary of optimization impact
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug, Clone ) ]
  pub struct OptimizationSummary
  {
    /// Overall optimization success indicator
    pub success : bool,
    /// Key achievements
    pub achievements : Vec< String >,
    /// Issues requiring attention
    pub concerns : Vec< String >,
    /// Next steps recommendations
    pub next_steps : Vec< String >,
  }

  /// Before/after optimization workflow manager
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug ) ]
  pub struct OptimizationWorkflow
  {
    /// Working directory for baseline storage
    baseline_dir : PathBuf,
    /// Documentation generator for reports
    doc_generator : ContextRichDocGenerator,
    /// Current benchmark identifier
    benchmark_name : String,
  }

  #[ cfg( feature = "benchmarks" ) ]
  impl OptimizationWorkflow
  {
    /// Create new optimization workflow manager
    pub fn new< P : AsRef< Path > >( 
      baseline_dir : P, 
      benchmark_name : String 
    ) -> std::io::Result< Self >
    {
      let baseline_path = baseline_dir.as_ref().to_path_buf();
      
      // Ensure baseline directory exists
      if !baseline_path.exists()
      {
        fs::create_dir_all( &baseline_path )?;
      }

      let doc_generator = ContextRichDocGenerator::default_environment();

      Ok( Self
      {
        baseline_dir : baseline_path,
        doc_generator,
        benchmark_name,
      } )
    }

    /// Step 1: Establish performance baseline
    pub fn establish_baseline(
      &self,
      results : Vec< BenchmarkResult >,
      cv_analysis : CoefficientsOfVariationAnalysis,
      environment_info : String,
      notes : Vec< String >
    ) -> std::io::Result< BaselineResults >
    {
      println!( "🔍 Step 1: Establishing performance baseline for '{}'", self.benchmark_name );

      let baseline = BaselineResults
      {
        timestamp : chrono::Utc::now().format( "%Y-%m-%d %H:%M:%S UTC" ).to_string(),
        benchmark_name : self.benchmark_name.clone(),
        environment_info,
        results,
        cv_analysis,
        notes,
      };

      // Save baseline to file
      self.save_baseline_results( &baseline )?;
      
      println!( "✅ Baseline established with {} algorithms", baseline.results.len() );
      println!( "   Quality: {}", baseline.cv_analysis.overall_quality_assessment() );

      Ok( baseline )
    }

    /// Step 3: Measure optimization impact
    pub fn measure_optimization_impact(
      &mut self,
      current_results : Vec< BenchmarkResult >,
      current_cv_analysis : CoefficientsOfVariationAnalysis
    ) -> std::io::Result< OptimizationImpact >
    {
      println!( "📊 Step 3: Measuring optimization impact for '{}'", self.benchmark_name );

      // Load baseline results
      let baseline = self.load_baseline_results()?;
      
      // Create before/after comparisons
      let mut comparisons = Vec::new();
      
      for current_result in &current_results
      {
        if let Some( baseline_result ) = baseline.results.iter()
          .find( | b | b.algorithm_name == current_result.algorithm_name )
        {
          let comparison = BeforeAfterComparison
          {
            algorithm_name : current_result.algorithm_name.clone(),
            before_nanos : baseline_result.average_time_nanos,
            after_nanos : current_result.average_time_nanos,
            status : self.determine_optimization_status(
              baseline_result.average_time_nanos,
              current_result.average_time_nanos
            ),
          };
          
          comparisons.push( comparison );
        }
      }

      // Analyze statistical significance
      let significance_analysis = self.analyze_significance( &comparisons, &baseline.cv_analysis, &current_cv_analysis );
      
      // Generate summary
      let summary = Self::generate_optimization_summary( &significance_analysis, &comparisons );

      let impact = OptimizationImpact
      {
        baseline,
        current_results,
        comparisons,
        significance_analysis,
        summary,
      };

      // Report findings
      Self::report_optimization_impact( &impact );
      
      // Update documentation
      self.update_optimization_documentation( &impact )?;

      Ok( impact )
    }

    /// Check if baseline exists for current benchmark
    pub fn has_baseline( &self ) -> bool
    {
      self.baseline_file_path().exists()
    }

    /// Load existing baseline results
    pub fn load_baseline_results( &self ) -> std::io::Result< BaselineResults >
    {
      let baseline_path = self.baseline_file_path();
      
      if !baseline_path.exists()
      {
        return Err( std::io::Error::new(
          std::io::ErrorKind::NotFound,
          format!( "No baseline found for benchmark '{}'. Run establish_baseline() first.", self.benchmark_name )
        ) );
      }

      let content = fs::read_to_string( baseline_path )?;
      let baseline : BaselineResults = serde_json::from_str( &content )
        .map_err( | e | std::io::Error::new( std::io::ErrorKind::InvalidData, e ) )?;

      Ok( baseline )
    }

    /// Get baseline file path for current benchmark
    fn baseline_file_path( &self ) -> PathBuf
    {
      self.baseline_dir.join( format!( "{}_baseline.json", self.benchmark_name ) )
    }

    /// Save baseline results to file
    fn save_baseline_results( &self, baseline : &BaselineResults ) -> std::io::Result< () >
    {
      let baseline_path = self.baseline_file_path();
      let content = serde_json::to_string_pretty( baseline )
        .map_err( | e | std::io::Error::new( std::io::ErrorKind::InvalidData, e ) )?;
        
      fs::write( baseline_path, content )?;
      
      Ok( () )
    }

    /// Determine optimization status based on performance change
    fn determine_optimization_status( &self, before_nanos : f64, after_nanos : f64 ) -> OptimizationStatus
    {
      let improvement_pct = ( ( before_nanos - after_nanos ) / before_nanos ) * 100.0;
      
      if improvement_pct >= 20.0
      {
        OptimizationStatus::ProductionReady
      }
      else if improvement_pct >= 5.0
      {
        OptimizationStatus::Optimized
      }
      else if improvement_pct >= -5.0
      {
        OptimizationStatus::Baseline
      }
      else if improvement_pct >= -20.0
      {
        OptimizationStatus::NeedsWork
      }
      else
      {
        OptimizationStatus::Regression
      }
    }

    /// Analyze statistical significance of optimization results
    fn analyze_significance(
      &self,
      comparisons : &[ BeforeAfterComparison ],
      baseline_cv : &CoefficientsOfVariationAnalysis,
      current_cv : &CoefficientsOfVariationAnalysis
    ) -> SignificanceAnalysis
    {
      let significant_improvements = comparisons.iter()
        .filter( | c | c.improvement_percentage() >= 5.0 )
        .count();
        
      let regressions = comparisons.iter()
        .filter( | c | c.improvement_percentage() <= -5.0 )
        .count();
        
      let total_algorithms = comparisons.len();
      
      let average_improvement = if total_algorithms > 0
      {
        comparisons.iter()
          .map( | c | c.improvement_percentage() )
          .sum::< f64 >() / total_algorithms as f64
      }
      else
      {
        0.0
      };

      SignificanceAnalysis
      {
        significant_improvements,
        regressions,
        total_algorithms,
        average_improvement,
        baseline_quality : baseline_cv.overall_quality_assessment(),
        current_quality : current_cv.overall_quality_assessment(),
      }
    }

    /// Generate optimization summary with recommendations
    fn generate_optimization_summary(
      significance : &SignificanceAnalysis,
      comparisons : &[ BeforeAfterComparison ]
    ) -> OptimizationSummary
    {
      let mut achievements = Vec::new();
      let mut concerns = Vec::new();
      let mut next_steps = Vec::new();

      // Assess success
      let success = significance.significant_improvements > 0 && significance.regressions == 0;

      // Identify achievements
      if significance.significant_improvements > 0
      {
        achievements.push( format!(
          "{} algorithm(s) showed significant improvements (≥5%)",
          significance.significant_improvements
        ) );
      }
      
      if significance.average_improvement > 0.0
      {
        achievements.push( format!(
          "Average performance improvement: {:.1}%",
          significance.average_improvement
        ) );
      }

      // Identify concerns
      if significance.regressions > 0
      {
        concerns.push( format!(
          "{} algorithm(s) showed performance regressions (≥5% slower)",
          significance.regressions
        ) );
        
        let regression_names : Vec< String > = comparisons.iter()
          .filter( | c | c.improvement_percentage() <= -5.0 )
          .map( | c | c.algorithm_name.clone() )
          .collect();
          
        concerns.push( format!( "Regressions in: {}", regression_names.join( ", " ) ) );
      }

      if significance.baseline_quality != "Good" || significance.current_quality != "Good"
      {
        concerns.push( "Measurement quality issues detected - results may be unreliable".to_string() );
      }

      // Generate next steps
      if significance.regressions > 0
      {
        next_steps.push( "Investigate and fix performance regressions before deployment".to_string() );
      }
      
      if significance.significant_improvements > 0
      {
        next_steps.push( "Consider deploying successful optimizations to production".to_string() );
      }
      
      if significance.significant_improvements == 0 && significance.regressions == 0
      {
        next_steps.push( "Explore alternative optimization approaches".to_string() );
      }

      next_steps.push( "Monitor performance in production environment".to_string() );

      OptimizationSummary
      {
        success,
        achievements,
        concerns,
        next_steps,
      }
    }

    /// Report optimization impact to console
    fn report_optimization_impact( impact : &OptimizationImpact )
    {
      let sig = &impact.significance_analysis;
      
      println!( "\n📈 Optimization Impact Analysis:" );
      println!( "   Algorithms tested: {}", sig.total_algorithms );
      println!( "   Significant improvements: {} ({:.1}%)", 
        sig.significant_improvements, 
        ( sig.significant_improvements as f64 / sig.total_algorithms as f64 ) * 100.0 
      );
      println!( "   Performance regressions: {} ({:.1}%)", 
        sig.regressions,
        ( sig.regressions as f64 / sig.total_algorithms as f64 ) * 100.0 
      );
      println!( "   Average improvement: {:.1}%", sig.average_improvement );

      if sig.regressions > 0
      {
        println!( "\n⚠️  Warning: Performance regressions detected!" );
        for comparison in &impact.comparisons
        {
          if comparison.improvement_percentage() <= -5.0
          {
            println!( "  - {}: {}", comparison.algorithm_name, comparison.format_improvement() );
          }
        }
      }

      if impact.summary.success
      {
        println!( "\n✅ Optimization successful!" );
      }
      else
      {
        println!( "\n❌ Optimization requires attention" );
      }
    }

    /// Update documentation with optimization results
    fn update_optimization_documentation( &mut self, impact : &OptimizationImpact ) -> std::io::Result< () >
    {
      // Clear previous sections
      self.doc_generator.clear_sections();

      // Create measurement context
      let context = BenchmarkMeasurementContext
      {
        what_is_measured : format!( 
          "Optimization impact analysis for {} algorithms in {}",
          impact.comparisons.len(),
          self.benchmark_name
        ),
        how_to_measure : "OptimizationWorkflow::measure_optimization_impact()".to_string(),
        purpose : "Validate optimization effectiveness and detect regressions".to_string(),
        environment : self.doc_generator.environment().clone(),
      };

      // Add before/after comparison documentation
      self.doc_generator.add_before_after_comparison(
        &format!( "{} Optimization Impact", self.benchmark_name ),
        context,
        &impact.comparisons
      );

      // Generate and save report
      let report = self.doc_generator.generate_report( 
        &format!( "{} Optimization Analysis", self.benchmark_name )
      );
      
      let doc_path = self.baseline_dir.join( format!( "{}_optimization_report.md", self.benchmark_name ) );
      fs::write( doc_path, report )?;

      Ok( () )
    }
  }
}

mod_interface::mod_interface!
{
  #[ cfg( feature = "benchmarks" ) ]
  orphan use CoefficientsOfVariationAnalysis;
  #[ cfg( feature = "benchmarks" ) ]
  orphan use BaselineResults;
  #[ cfg( feature = "benchmarks" ) ]
  orphan use OptimizationImpact;
  #[ cfg( feature = "benchmarks" ) ]
  orphan use SignificanceAnalysis;
  #[ cfg( feature = "benchmarks" ) ]
  orphan use OptimizationSummary;
  #[ cfg( feature = "benchmarks" ) ]
  orphan use OptimizationWorkflow;
}