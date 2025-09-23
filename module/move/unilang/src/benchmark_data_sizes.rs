//! Standard benchmark data sizes for consistent performance comparison
//!
//! Implements benchkit usage.md "Use Standard Data Sizes" section
//! providing consistent sizing across all benchmarks.

/// Internal namespace.
mod private
{
  /// Standard benchmark data size categories
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, Hash ) ]
  pub enum BenchmarkDataSize
  {
    /// Small: 10 elements - Quick operations, edge cases
    Small,
    /// Medium: 100 elements - Typical usage scenarios  
    Medium,
    /// Large: 1000 elements - Stress testing, scaling analysis
    Large,
    /// Huge: 10000 elements - Performance bottleneck detection
    Huge,
  }

  #[ cfg( feature = "benchmarks" ) ]
  impl BenchmarkDataSize
  {
    /// Get the numeric value for this size category
    pub fn value( &self ) -> usize
    {
      match self
      {
        Self::Small => 10,
        Self::Medium => 100,
        Self::Large => 1000,
        Self::Huge => 10000,
      }
    }

    /// Get the lowercase name for benchmark naming
    pub fn name( &self ) -> &'static str
    {
      match self
      {
        Self::Small => "small",
        Self::Medium => "medium", 
        Self::Large => "large",
        Self::Huge => "huge",
      }
    }

    /// Get the descriptive name for documentation
    pub fn description( &self ) -> &'static str
    {
      match self
      {
        Self::Small => "Small (10) - Quick operations, edge cases",
        Self::Medium => "Medium (100) - Typical usage scenarios",
        Self::Large => "Large (1000) - Stress testing, scaling analysis",
        Self::Huge => "Huge (10000) - Performance bottleneck detection",
      }
    }

    /// Get all standard data sizes in order
    pub fn all() -> Vec< Self >
    {
      vec![ Self::Small, Self::Medium, Self::Large, Self::Huge ]
    }

    /// Get all standard data sizes as (name, value) pairs
    pub fn all_pairs() -> Vec< ( &'static str, usize ) >
    {
      Self::all().into_iter().map( | size | ( size.name(), size.value() ) ).collect()
    }

    /// Create benchmark name with size category
    pub fn benchmark_name( &self, base_name : &str ) -> String
    {
      format!( "{}_{}", base_name, self.name() )
    }

    /// Format size info for documentation
    pub fn format_info( &self ) -> String
    {
      format!( "{} ({})", self.name().to_uppercase(), self.value() )
    }
  }

  /// Standard data size generator trait
  #[ cfg( feature = "benchmarks" ) ]
  pub trait StandardDataGenerator< T >
  {
    /// Generate test data for the specified size category
    fn generate_for_size( size : BenchmarkDataSize ) -> T;
    
    /// Generate test data for a specific count
    fn generate_for_count( count : usize ) -> T;
  }

  /// Utility functions for standard benchmark data
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug ) ]
  pub struct BenchmarkDataUtils;

  #[ cfg( feature = "benchmarks" ) ]
  impl BenchmarkDataUtils
  {
    /// Generate command names for the specified size
    pub fn generate_command_names( size : BenchmarkDataSize ) -> Vec< String >
    {
      Self::generate_command_names_count( size.value() )
    }

    /// Generate command names for a specific count
    pub fn generate_command_names_count( count : usize ) -> Vec< String >
    {
      ( 0..count ).map( | i | format!( ".command_{:04}", i ) ).collect()
    }

    /// Generate test strings for the specified size  
    pub fn generate_test_strings( size : BenchmarkDataSize ) -> Vec< String >
    {
      Self::generate_test_strings_count( size.value() )
    }

    /// Generate test strings for a specific count
    pub fn generate_test_strings_count( count : usize ) -> Vec< String >
    {
      ( 0..count ).map( | i | format!( "test_string_{:04}", i ) ).collect()
    }

    /// Generate JSON test data for the specified size
    pub fn generate_json_data( size : BenchmarkDataSize ) -> String
    {
      let count = size.value();
      let items : Vec< String > = ( 0..count ).map( | i | 
        format!( r#"{{"id": {}, "name": "item_{:04}", "active": {}}}"#, i, i, i % 2 == 0 )
      ).collect();
      format!( r#"{{"items": [{}]}}"#, items.join( ", " ) )
    }

    /// Get size category description for documentation
    pub fn document_sizes() -> String
    {
      let descriptions : Vec< String > = BenchmarkDataSize::all()
        .into_iter()
        .map( | size | format!( "- **{}**", size.description() ) )
        .collect();
      
      format!(
        "## Standard Data Size Categories\n\n{}\n\nThis standardization enables consistent performance comparison across different implementations and projects.",
        descriptions.join( "\n" )
      )
    }
  }
}

mod_interface::mod_interface!
{
  #[ cfg( feature = "benchmarks" ) ]
  orphan use BenchmarkDataSize;
  #[ cfg( feature = "benchmarks" ) ]
  orphan use StandardDataGenerator;
  #[ cfg( feature = "benchmarks" ) ]
  orphan use BenchmarkDataUtils;
}