//! Benchmark validation and quality assessment framework
//!
//! Provides tools for validating benchmark methodology and detecting
//! reliability issues before drawing performance conclusions.

use crate::measurement::BenchmarkResult;
use std::collections::HashMap;

#[ allow( dead_code ) ]
type Result< T > = std::result::Result< T, Box< dyn std::error::Error > >;

/// Validation warnings for benchmark quality
#[ derive( Debug, Clone ) ]
pub enum ValidationWarning
{
  /// Sample size too small for reliable analysis
  InsufficientSamples
  {
    /// Actual sample count
    actual : usize,
    /// Minimum recommended
    minimum : usize,
  },
  /// Coefficient of variation too high
  HighVariability
  {
    /// Actual CV
    actual : f64,
    /// Maximum recommended
    maximum : f64,
  },
  /// No warmup iterations detected
  NoWarmup,
  /// Wide performance range suggests outliers
  WidePerformanceRange
  {
    /// Ratio of max to min time
    ratio : f64,
  },
  /// Measurement time too short for accuracy
  ShortMeasurementTime
  {
    /// Mean duration
    duration : std::time::Duration,
  },
}

impl std::fmt::Display for ValidationWarning
{
  fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
  {
    match self
    {
      ValidationWarning::InsufficientSamples { actual, minimum } =>
      {
        write!( f, "Insufficient samples: {} (minimum: {})", actual, minimum )
      },
      ValidationWarning::HighVariability { actual, maximum } =>
      {
        write!( f, "High variability: CV={:.1}% (maximum: {:.1}%)", actual * 100.0, maximum * 100.0 )
      },
      ValidationWarning::NoWarmup =>
      {
        write!( f, "No warmup detected - first measurement may include setup overhead" )
      },
      ValidationWarning::WidePerformanceRange { ratio } =>
      {
        write!( f, "Wide performance range: {:.1}x difference between fastest and slowest", ratio )
      },
      ValidationWarning::ShortMeasurementTime { duration } =>
      {
        write!( f, "Short measurement time: {:.2?} (consider longer operations)", duration )
      },
    }
  }
}

/// Benchmark quality validator with configurable criteria
#[ derive( Debug, Clone ) ]
pub struct BenchmarkValidator
{
  /// Minimum sample size for reliable results
  min_samples : usize,
  /// Maximum coefficient of variation
  max_coefficient_variation : f64,
  /// Whether warmup is required
  require_warmup : bool,
  /// Maximum ratio between longest and shortest time
  max_time_ratio : f64,
  /// Minimum measurement duration
  min_measurement_time : std::time::Duration,
}

impl BenchmarkValidator
{
  /// Create new validator with default settings
  #[ must_use ]
  pub fn new() -> Self
  {
    Self
    {
      min_samples : 10,
      max_coefficient_variation : 0.1, // 10%
      require_warmup : true,
      max_time_ratio : 3.0,
      min_measurement_time : std::time::Duration::from_micros( 100 ), // 100μs
    }
  }

  /// Set minimum sample size
  #[ must_use ]
  pub fn min_samples( mut self, count : usize ) -> Self
  {
    self.min_samples = count;
    self
  }

  /// Set maximum coefficient of variation
  #[ must_use ]
  pub fn max_coefficient_variation( mut self, cv : f64 ) -> Self
  {
    self.max_coefficient_variation = cv;
    self
  }

  /// Set whether warmup is required
  #[ must_use ]
  pub fn require_warmup( mut self, required : bool ) -> Self
  {
    self.require_warmup = required;
    self
  }

  /// Set maximum time ratio (max/min)
  #[ must_use ]
  pub fn max_time_ratio( mut self, ratio : f64 ) -> Self
  {
    self.max_time_ratio = ratio;
    self
  }

  /// Set minimum measurement time
  #[ must_use ]
  pub fn min_measurement_time( mut self, duration : std::time::Duration ) -> Self
  {
    self.min_measurement_time = duration;
    self
  }

  /// Validate a single benchmark result
  #[ must_use ]
  pub fn validate_result( &self, result : &BenchmarkResult ) -> Vec< ValidationWarning >
  {
    let mut warnings = Vec::new();

    // Sample size check
    if result.times.len() < self.min_samples
    {
      warnings.push( ValidationWarning::InsufficientSamples
      {
        actual : result.times.len(),
        minimum : self.min_samples,
      });
    }

    // Coefficient of variation check
    let cv = result.coefficient_of_variation();
    if cv > self.max_coefficient_variation
    {
      warnings.push( ValidationWarning::HighVariability
      {
        actual : cv,
        maximum : self.max_coefficient_variation,
      });
    }

    // Time ratio check
    let time_ratio = result.max_time().as_secs_f64() / result.min_time().as_secs_f64();
    if time_ratio > self.max_time_ratio
    {
      warnings.push( ValidationWarning::WidePerformanceRange
      {
        ratio : time_ratio,
      });
    }

    // Measurement duration check
    if result.mean_time() < self.min_measurement_time
    {
      warnings.push( ValidationWarning::ShortMeasurementTime
      {
        duration : result.mean_time(),
      });
    }

    // Warmup check (heuristic: first measurement significantly slower)
    if self.require_warmup && result.times.len() >= 2
    {
      let first_time = result.times[ 0 ].as_secs_f64();
      let second_time = result.times[ 1 ].as_secs_f64();
      
      // If first measurement is not significantly different, assume no warmup
      if ( first_time / second_time ) < 1.2
      {
        warnings.push( ValidationWarning::NoWarmup );
      }
    }

    warnings
  }

  /// Validate multiple benchmark results
  #[ must_use ]
  pub fn validate_results( &self, results : &HashMap< String, BenchmarkResult > ) -> HashMap< String, Vec< ValidationWarning > >
  {
    results.iter()
      .map( | ( name, result ) |
      {
        let warnings = self.validate_result( result );
        ( name.clone(), warnings )
      })
      .collect()
  }

  /// Check if a result passes all validation criteria
  #[ must_use ]
  pub fn is_reliable( &self, result : &BenchmarkResult ) -> bool
  {
    self.validate_result( result ).is_empty()
  }

  /// Generate validation report
  #[ must_use ]
  pub fn generate_validation_report( &self, results : &HashMap< String, BenchmarkResult > ) -> String
  {
    let mut output = String::new();
    
    output.push_str( "# Benchmark Validation Report\n\n" );
    
    let validation_results = self.validate_results( results );
    let total_benchmarks = results.len();
    let reliable_benchmarks = validation_results.values()
      .filter( | warnings | warnings.is_empty() )
      .count();
    
    output.push_str( "## Summary\n\n" );
    output.push_str( &format!( "- **Total benchmarks**: {}\n", total_benchmarks ) );
    output.push_str( &format!( "- **Reliable benchmarks**: {}\n", reliable_benchmarks ) );
    output.push_str( &format!( "- **Reliability rate**: {:.1}%\n\n", 
                               ( reliable_benchmarks as f64 / total_benchmarks as f64 ) * 100.0 ) );

    // Reliable results
    let reliable_results : Vec< _ > = validation_results.iter()
      .filter( | ( _, warnings ) | warnings.is_empty() )
      .collect();
    
    if !reliable_results.is_empty()
    {
      output.push_str( "## ✅ Reliable Benchmarks\n\n" );
      output.push_str( "*These benchmarks meet all quality criteria*\n\n" );
      for ( name, _ ) in reliable_results
      {
        let result = &results[ name ];
        output.push_str( &format!( "- **{}**: {} samples, CV={:.1}%\n",
                                   name,
                                   result.times.len(),
                                   result.coefficient_of_variation() * 100.0 ) );
      }
      output.push_str( "\n" );
    }

    // Problematic results
    let problematic_results : Vec< _ > = validation_results.iter()
      .filter( | ( _, warnings ) | !warnings.is_empty() )
      .collect();
    
    if !problematic_results.is_empty()
    {
      output.push_str( "## ⚠️ Benchmarks Needing Attention\n\n" );
      output.push_str( "*Consider addressing these issues for more reliable results*\n\n" );
      
      for ( name, warnings ) in problematic_results
      {
        output.push_str( &format!( "### {}\n\n", name ) );
        for warning in warnings
        {
          output.push_str( &format!( "- {}\n", warning ) );
        }
        output.push_str( "\n" );
      }
    }

    // Recommendations
    output.push_str( "## Recommendations\n\n" );
    self.add_improvement_recommendations( &mut output, &validation_results );

    // Validation criteria
    output.push_str( "## Validation Criteria\n\n" );
    output.push_str( &format!( "- **Minimum samples**: {}\n", self.min_samples ) );
    output.push_str( &format!( "- **Maximum CV**: {:.1}%\n", self.max_coefficient_variation * 100.0 ) );
    output.push_str( &format!( "- **Maximum time ratio**: {:.1}x\n", self.max_time_ratio ) );
    output.push_str( &format!( "- **Minimum duration**: {:.2?}\n", self.min_measurement_time ) );
    output.push_str( &format!( "- **Warmup required**: {}\n\n", if self.require_warmup { "Yes" } else { "No" } ) );

    output.push_str( "---\n" );
    output.push_str( "*Generated by benchkit validation framework*\n" );

    output
  }

  /// Add improvement recommendations
  fn add_improvement_recommendations( &self, output : &mut String, validation_results : &HashMap< String, Vec< ValidationWarning > > )
  {
    let mut sample_issues = 0;
    let mut variability_issues = 0;
    let mut warmup_issues = 0;
    let mut duration_issues = 0;

    for warnings in validation_results.values()
    {
      for warning in warnings
      {
        match warning
        {
          ValidationWarning::InsufficientSamples { .. } => sample_issues += 1,
          ValidationWarning::HighVariability { .. } => variability_issues += 1,
          ValidationWarning::NoWarmup => warmup_issues += 1,
          ValidationWarning::ShortMeasurementTime { .. } => duration_issues += 1,
          ValidationWarning::WidePerformanceRange { .. } => variability_issues += 1,
        }
      }
    }

    if sample_issues > 0
    {
      output.push_str( &format!( "- **Increase sample sizes** ({} benchmarks affected): Run more iterations for better statistical power\n", sample_issues ) );
    }
    
    if variability_issues > 0
    {
      output.push_str( &format!( "- **Reduce measurement noise** ({} benchmarks affected): Consider isolating CPU cores, disabling frequency scaling, or running in controlled environment\n", variability_issues ) );
    }
    
    if warmup_issues > 0
    {
      output.push_str( &format!( "- **Add warmup iterations** ({} benchmarks affected): Run operation several times before measurement to stabilize performance\n", warmup_issues ) );
    }
    
    if duration_issues > 0
    {
      output.push_str( &format!( "- **Increase operation duration** ({} benchmarks affected): Make measured operations take longer to reduce timer precision effects\n", duration_issues ) );
    }

    output.push_str( "\n" );
  }
}

impl Default for BenchmarkValidator
{
  fn default() -> Self
  {
    Self::new()
  }
}

/// Validated benchmark results with reliability information
#[ derive( Debug ) ]
pub struct ValidatedResults
{
  /// Original benchmark results
  pub results : HashMap< String, BenchmarkResult >,
  /// Validation warnings for each benchmark
  pub warnings : HashMap< String, Vec< ValidationWarning > >,
  /// Validator used for validation
  pub validator : BenchmarkValidator,
}

impl ValidatedResults
{
  /// Create new validated results
  #[ must_use ]
  pub fn new( results : HashMap< String, BenchmarkResult >, validator : BenchmarkValidator ) -> Self
  {
    let warnings = validator.validate_results( &results );
    
    Self
    {
      results,
      warnings,
      validator,
    }
  }

  /// Get reliability warnings for all benchmarks
  #[ must_use ]
  pub fn reliability_warnings( &self ) -> Option< Vec< String > >
  {
    let warnings : Vec< String > = self.warnings.iter()
      .filter_map( | ( name, warnings ) |
      {
        if warnings.is_empty()
        {
          None
        }
        else
        {
          Some( format!( "{}: {}", name, warnings.iter()
            .map( | w | w.to_string() )
            .collect::< Vec< _ > >()
            .join( ", " ) ) )
        }
      })
      .collect();
    
    if warnings.is_empty()
    {
      None
    }
    else
    {
      Some( warnings )
    }
  }

  /// Check if all results are reliable
  #[ must_use ]
  pub fn all_reliable( &self ) -> bool
  {
    self.warnings.values().all( | warnings | warnings.is_empty() )
  }

  /// Get count of reliable benchmarks
  #[ must_use ]
  pub fn reliable_count( &self ) -> usize
  {
    self.warnings.values()
      .filter( | warnings | warnings.is_empty() )
      .count()
  }

  /// Get reliability rate as percentage
  #[ must_use ]
  pub fn reliability_rate( &self ) -> f64
  {
    if self.results.is_empty()
    {
      0.0
    }
    else
    {
      ( self.reliable_count() as f64 / self.results.len() as f64 ) * 100.0
    }
  }

  /// Generate validation report
  #[ must_use ]
  pub fn validation_report( &self ) -> String
  {
    self.validator.generate_validation_report( &self.results )
  }

  /// Get only the reliable results
  #[ must_use ]
  pub fn reliable_results( &self ) -> HashMap< String, BenchmarkResult >
  {
    self.results.iter()
      .filter_map( | ( name, result ) |
      {
        if self.warnings.get( name ).map_or( false, | w | w.is_empty() )
        {
          Some( ( name.clone(), result.clone() ) )
        }
        else
        {
          None
        }
      })
      .collect()
  }
}