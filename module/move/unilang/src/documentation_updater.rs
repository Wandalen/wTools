//! Automatic documentation updater for benchmark results
//!
//! Implements benchkit usage.md "Automatic Documentation Updates" section
//! with comprehensive multi-file documentation updates.

/// Internal namespace.
mod private
{
  #[ cfg( feature = "benchmarks" ) ]
  use benchkit::reporting::MarkdownUpdater;

  /// Comprehensive documentation updater for benchmark results
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug ) ]
  pub struct DocumentationUpdater
  {
    /// List of documentation files to update
    update_targets : Vec< ( String, String ) >,
  }

  #[ cfg( feature = "benchmarks" ) ]
  impl DocumentationUpdater
  {
    /// Create new documentation updater with default targets
    pub fn new() -> Self
    {
      Self
      {
        update_targets : vec![
          ( "benches/readme.md".to_string(), "Performance Overview".to_string() ),
          ( "PERFORMANCE.md".to_string(), "Detailed Results".to_string() ),
          ( "docs/optimization_guide.md".to_string(), "Current Benchmarks".to_string() ),
        ],
      }
    }

    /// Add custom documentation target
    pub fn add_target( mut self, file_path : &str, section_name : &str ) -> Self
    {
      self.update_targets.push( ( file_path.to_string(), section_name.to_string() ) );
      self
    }

    /// Update all documentation files with benchmark results
    pub fn update_documentation( &self, benchmark_name : &str, report : &str ) -> Result< (), Box< dyn std::error::Error > >
    {
      for ( file_path, section_name ) in &self.update_targets
      {
        let specific_section = format!( "{} - {}", section_name, benchmark_name );
        let updater = MarkdownUpdater::new( file_path, &specific_section )?;
        updater.update_section( report )?;
        println!( "📄 Updated {}: {}", file_path, specific_section );
      }
      
      println!( "✅ Documentation updated automatically for {}", benchmark_name );
      Ok( () )
    }

    /// Update single documentation file
    pub fn update_single_file( file_path : &str, section_name : &str, report : &str ) -> Result< (), Box< dyn std::error::Error > >
    {
      let updater = MarkdownUpdater::new( file_path, section_name )?;
      updater.update_section( report )?;
      println!( "📄 Updated {}: {}", file_path, section_name );
      Ok( () )
    }

    /// Generate comprehensive benchmark report
    pub fn generate_report( benchmark_name : &str, results : &str ) -> String
    {
      format!(
        "## {} Results\n\n{}\n\n*Last updated: {}*\n",
        benchmark_name,
        results,
        chrono::Utc::now().format( "%Y-%m-%d %H:%M:%S UTC" )
      )
    }
  }

  #[ cfg( feature = "benchmarks" ) ]
  impl Default for DocumentationUpdater
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
  orphan use DocumentationUpdater;
}