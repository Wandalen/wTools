//! Comparative benchmark structure for side-by-side algorithm performance analysis
//!
//! Implements benchkit usage.md "Write Comparative Benchmarks" section
//! providing systematic comparison with baseline establishment and relative performance.

/// Internal namespace.
mod private
{
  #[ cfg( feature = "benchmarks" ) ]
  use std::collections::HashMap;
  #[ cfg( feature = "benchmarks" ) ]
  use std::fmt::Write;
  #[ cfg( feature = "benchmarks" ) ]
  use crate::benchmark_data_sizes::BenchmarkDataSize;

  /// Results from a single benchmark run
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug, Clone, serde::Serialize, serde::Deserialize ) ]
  pub struct BenchmarkResult
  {
    /// Name of the algorithm tested
    pub algorithm_name : String,
    /// Average execution time in nanoseconds
    pub average_time_nanos : f64,
    /// Standard deviation in nanoseconds
    pub std_dev_nanos : f64,
    /// Minimum execution time in nanoseconds
    pub min_time_nanos : u64,
    /// Maximum execution time in nanoseconds
    pub max_time_nanos : u64,
    /// Number of samples taken
    pub sample_count : usize,
  }

  #[ cfg( feature = "benchmarks" ) ]
  impl BenchmarkResult
  {
    /// Calculate relative performance compared to baseline
    #[ cfg( feature = "benchmarks" ) ]
    pub fn relative_performance( &self, baseline_time : f64 ) -> f64
    {
      if baseline_time == 0.0
      {
        1.0
      }
      else
      {
        self.average_time_nanos / baseline_time
      }
    }

    /// Format time in human-readable units
    #[ cfg( feature = "benchmarks" ) ]
    pub fn format_time( &self ) -> String
    {
      let time_ms = self.average_time_nanos / 1_000_000.0;
      let std_dev_ms = self.std_dev_nanos / 1_000_000.0;
      
      if time_ms >= 1000.0
      {
        format!( "{:.2}s ±{:.2}s", time_ms / 1000.0, std_dev_ms / 1000.0 )
      }
      else if time_ms >= 1.0
      {
        format!( "{:.2}ms ±{:.2}ms", time_ms, std_dev_ms )
      }
      else
      {
        let time_us = self.average_time_nanos / 1_000.0;
        let std_dev_us = self.std_dev_nanos / 1_000.0;
        format!( "{:.2}µs ±{:.2}µs", time_us, std_dev_us )
      }
    }

    /// Calculate coefficient of variation as percentage
    #[ cfg( feature = "benchmarks" ) ]
    pub fn coefficient_of_variation( &self ) -> f64
    {
      if self.average_time_nanos == 0.0
      {
        0.0
      }
      else
      {
        ( self.std_dev_nanos / self.average_time_nanos ) * 100.0
      }
    }

    /// Create BenchmarkResult from timing samples
    #[ cfg( feature = "benchmarks" ) ]
    pub fn from_samples( algorithm_name : &str, times : Vec< core::time::Duration > ) -> Self
    {
      if times.is_empty()
      {
        return Self
        {
          algorithm_name : algorithm_name.to_string(),
          average_time_nanos : 0.0,
          std_dev_nanos : 0.0,
          min_time_nanos : 0,
          max_time_nanos : 0,
          sample_count : 0,
        };
      }

      let times_nanos : Vec< f64 > = times.iter().map( |t| t.as_nanos() as f64 ).collect();

      let sum : f64 = times_nanos.iter().sum();
      let mean = sum / times_nanos.len() as f64;

      let variance = times_nanos.iter()
        .map( |t| ( t - mean ).powi( 2 ) )
        .sum::< f64 >() / times_nanos.len() as f64;

      let std_dev = variance.sqrt();

      let min_time = times.iter().min().unwrap_or( &core::time::Duration::ZERO ).as_nanos() as u64;
      let max_time = times.iter().max().unwrap_or( &core::time::Duration::ZERO ).as_nanos() as u64;

      Self
      {
        algorithm_name : algorithm_name.to_string(),
        average_time_nanos : mean,
        std_dev_nanos : std_dev,
        min_time_nanos : min_time,
        max_time_nanos : max_time,
        sample_count : times.len(),
      }
    }

    /// Get coefficient of variation as ratio (0.0 to 1.0) for compatibility
    #[ cfg( feature = "benchmarks" ) ]
    pub fn coefficient_of_variation_ratio( &self ) -> f64
    {
      self.coefficient_of_variation() / 100.0
    }

    /// Get average time as Duration for compatibility
    #[ cfg( feature = "benchmarks" ) ]
    pub fn average_time( &self ) -> core::time::Duration
    {
      #[allow(clippy::cast_possible_truncation)]
      core::time::Duration::from_nanos( self.average_time_nanos as u64 )
    }
  }

  /// Comparative benchmark runner for side-by-side algorithm testing
  #[ cfg( feature = "benchmarks" ) ]
  #[ allow( missing_debug_implementations ) ]
  pub struct ComparativeBenchmark< T >
  {
    /// Name of the benchmark
    name : String,
    /// Description of what is being measured
    description : String,
    /// List of algorithms to compare
    algorithms : Vec< ( String, Box< dyn Fn( &T ) -> () + Send + Sync > ) >,
    /// Test data for different size categories
    test_data : HashMap< BenchmarkDataSize, T >,
  }

  #[ cfg( feature = "benchmarks" ) ]
  impl< T > ComparativeBenchmark< T >
  where
    T : Clone + Send + Sync + 'static,
  {
    /// Create new comparative benchmark
    pub fn new( name : &str, description : &str ) -> Self
    {
      Self
      {
        name : name.to_string(),
        description : description.to_string(),
        algorithms : Vec::new(),
        test_data : HashMap::new(),
      }
    }

    /// Get the benchmark name
    pub fn name( &self ) -> &str
    {
      &self.name
    }

    /// Get the benchmark description
    pub fn description( &self ) -> &str
    {
      &self.description
    }

    /// Get the number of registered algorithms
    pub fn algorithm_count( &self ) -> usize
    {
      self.algorithms.len()
    }

    /// Add algorithm to comparison
    pub fn add_algorithm< F >( &mut self, name : &str, algorithm : F ) -> &mut Self
    where
      F : Fn( &T ) -> () + Send + Sync + 'static,
    {
      self.algorithms.push( ( name.to_string(), Box::new( algorithm ) ) );
      self
    }

    /// Set test data for specific size category
    pub fn set_test_data( &mut self, size : BenchmarkDataSize, data : T ) -> &mut Self
    {
      self.test_data.insert( size, data );
      self
    }

    /// Run all algorithms for a specific data size and return comparison results
    pub fn run_comparison( &self, size : BenchmarkDataSize, iterations : usize ) -> ComparativeResults
    {
      let test_data = match self.test_data.get( &size )
      {
        Some( data ) => data,
        None => panic!( "No test data available for size {:?}", size ),
      };

      let mut results = Vec::new();

      for ( name, algorithm ) in &self.algorithms
      {
        let mut times = Vec::new();

        // Warmup runs
        for _ in 0..10
        {
          algorithm( test_data );
        }

        // Actual benchmark runs
        for _ in 0..iterations
        {
          let start = std::time::Instant::now();
          algorithm( test_data );
          let duration = start.elapsed();
          times.push( duration.as_nanos() as u64 );
        }

        // Calculate statistics
        times.sort_unstable();
        let average_time_nanos = times.iter().sum::< u64 >() as f64 / times.len() as f64;
        let variance = times.iter()
          .map( | &x | ( x as f64 - average_time_nanos ).powi( 2 ) )
          .sum::< f64 >() / times.len() as f64;
        let std_dev_nanos = variance.sqrt();

        results.push( BenchmarkResult
        {
          algorithm_name : name.clone(),
          average_time_nanos,
          std_dev_nanos,
          min_time_nanos : *times.first().unwrap(),
          max_time_nanos : *times.last().unwrap(),
          sample_count : times.len(),
        } );
      }

      ComparativeResults::new( self.name.clone(), self.description.clone(), size, results )
    }
  }

  /// Results from comparative benchmark run
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug, Clone ) ]
  pub struct ComparativeResults
  {
    /// Name of the benchmark
    pub benchmark_name : String,
    /// Description of what was measured
    pub description : String,
    /// Data size category used
    pub data_size : BenchmarkDataSize,
    /// Individual algorithm results
    pub results : Vec< BenchmarkResult >,
    /// Baseline time for relative performance calculation
    pub baseline_time : f64,
    /// Name of the fastest algorithm
    pub fastest_algorithm : String,
  }

  #[ cfg( feature = "benchmarks" ) ]
  impl ComparativeResults
  {
    /// Create new comparative results with baseline establishment
    pub fn new( 
      benchmark_name : String, 
      description : String, 
      data_size : BenchmarkDataSize, 
      mut results : Vec< BenchmarkResult > 
    ) -> Self
    {
      // Sort by average time to establish baseline (fastest = 1.00x)
      results.sort_by( | a, b | a.average_time_nanos.partial_cmp( &b.average_time_nanos ).unwrap() );
      
      let baseline_time = results.first()
        .map( | r | r.average_time_nanos )
        .unwrap_or( 1.0 );
        
      let fastest_algorithm = results.first()
        .map( | r | r.algorithm_name.clone() )
        .unwrap_or_default();

      Self
      {
        benchmark_name,
        description,
        data_size,
        results,
        baseline_time,
        fastest_algorithm,
      }
    }

    /// Generate comparison table in markdown format
    pub fn generate_comparison_table( &self ) -> String
    {
      let mut table = String::new();
      
      writeln!( &mut table, "## {} Comparison", self.benchmark_name ).unwrap();
      writeln!( &mut table, "" ).unwrap();
      writeln!( &mut table, "**What is measured**: {}", self.description ).unwrap();
      writeln!( &mut table, "**Data size**: {} ({})", self.data_size.format_info(), self.data_size.value() ).unwrap();
      writeln!( &mut table, "**Winner**: {} 🏆", self.fastest_algorithm ).unwrap();
      writeln!( &mut table, "" ).unwrap();
      
      writeln!( &mut table, "| Algorithm | Average Time | Std Dev | Min | Max | Relative Performance |" ).unwrap();
      writeln!( &mut table, "|-----------|--------------|---------|-----|-----|---------------------|" ).unwrap();
      
      for result in &self.results
      {
        let relative = result.relative_performance( self.baseline_time );
        let performance_indicator = if relative <= 1.0
        {
          "1.00x (baseline) 🏆"
        }
        else
        {
          &format!( "{:.2}x slower", relative )
        };
        
        writeln!(
          &mut table,
          "| {} | {} | {:.2}µs | {:.2}µs | {:.2}µs | {} |",
          result.algorithm_name,
          result.format_time(),
          result.std_dev_nanos / 1000.0,
          result.min_time_nanos as f64 / 1000.0,
          result.max_time_nanos as f64 / 1000.0,
          performance_indicator
        ).unwrap();
      }
      
      writeln!( &mut table, "" ).unwrap();
      table
    }

    /// Get performance improvement factor of fastest vs slowest
    pub fn performance_range( &self ) -> f64
    {
      if let ( Some( fastest ), Some( slowest ) ) = ( self.results.first(), self.results.last() )
      {
        slowest.average_time_nanos / fastest.average_time_nanos
      }
      else
      {
        1.0
      }
    }

    /// Check if results show statistically significant differences
    pub fn has_significant_differences( &self, threshold_factor : f64 ) -> bool
    {
      self.performance_range() > threshold_factor
    }
  }

  /// Multi-size comparative benchmark for comprehensive analysis
  #[ cfg( feature = "benchmarks" ) ]
  #[ allow( missing_debug_implementations ) ]
  pub struct MultiSizeComparison< T >
  {
    /// The underlying comparative benchmark
    benchmark : ComparativeBenchmark< T >,
    /// Results for each data size
    results : HashMap< BenchmarkDataSize, ComparativeResults >,
  }

  #[ cfg( feature = "benchmarks" ) ]
  impl< T > MultiSizeComparison< T >
  where
    T : Clone + Send + Sync + 'static,
  {
    /// Create from existing comparative benchmark
    pub fn new( benchmark : ComparativeBenchmark< T > ) -> Self
    {
      Self
      {
        benchmark,
        results : HashMap::new(),
      }
    }

    /// Run comparison across all configured data sizes
    pub fn run_all_sizes( &mut self, iterations_per_size : usize )
    {
      for size in BenchmarkDataSize::all()
      {
        if self.benchmark.test_data.contains_key( &size )
        {
          let results = self.benchmark.run_comparison( size, iterations_per_size );
          self.results.insert( size, results );
        }
      }
    }

    /// Generate comprehensive comparison report across all sizes
    pub fn generate_comprehensive_report( &self ) -> String
    {
      let mut report = String::new();
      
      writeln!( &mut report, "# {} - Comprehensive Size Analysis", self.benchmark.name ).unwrap();
      writeln!( &mut report, "" ).unwrap();
      writeln!( &mut report, "{}", self.benchmark.description ).unwrap();
      writeln!( &mut report, "" ).unwrap();
      
      // Results for each size
      for size in BenchmarkDataSize::all()
      {
        if let Some( results ) = self.results.get( &size )
        {
          writeln!( &mut report, "{}", results.generate_comparison_table() ).unwrap();
        }
      }
      
      // Summary analysis
      writeln!( &mut report, "## Performance Summary" ).unwrap();
      writeln!( &mut report, "" ).unwrap();
      
      for size in BenchmarkDataSize::all()
      {
        if let Some( results ) = self.results.get( &size )
        {
          writeln!( 
            &mut report, 
            "- **{}**: {} wins with {:.2}x performance advantage",
            size.format_info(),
            results.fastest_algorithm,
            results.performance_range()
          ).unwrap();
        }
      }
      
      writeln!( &mut report, "" ).unwrap();
      report
    }
  }
}

mod_interface::mod_interface!
{
  #[ cfg( feature = "benchmarks" ) ]
  orphan use BenchmarkResult;
  #[ cfg( feature = "benchmarks" ) ]
  orphan use ComparativeBenchmark;
  #[ cfg( feature = "benchmarks" ) ]
  orphan use ComparativeResults;
  #[ cfg( feature = "benchmarks" ) ]
  orphan use MultiSizeComparison;
}