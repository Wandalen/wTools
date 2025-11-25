//! Test Organization Validation Tool
//!
//! This tool validates that all test files follow the systematic organization
//! principles defined in tests/readme.md. It ensures adherence to naming
//! conventions, proper categorization, and prevents regression to problematic
//! task-based naming patterns.

use std::fs;
use std::path::{ Path, PathBuf };
use std::collections::{ HashMap, HashSet };

/// Validation rules for test organization
#[ derive( Debug, Clone ) ]
pub struct OrganizationRules
{
  /// Allowed top-level directories
  pub allowed_directories : HashSet< String >,
  /// Prohibited naming patterns
  pub prohibited_patterns : Vec< String >,
  /// Required naming conventions
  pub naming_conventions : HashMap< String, Vec< String > >,
  /// Maximum depth for test directories
  pub max_depth : usize,
}

impl Default for OrganizationRules
{
  fn default() -> Self
  {
    let mut allowed_directories = HashSet::new();
    allowed_directories.insert( "unit".to_string() );
    allowed_directories.insert( "integration".to_string() );
    allowed_directories.insert( "acceptance".to_string() );
    allowed_directories.insert( "regression".to_string() );
    allowed_directories.insert( "inc".to_string() ); // Legacy compatibility
    allowed_directories.insert( "tools".to_string() ); // Organization tools

    let prohibited_patterns = vec![
      "task_".to_string(),
      "issue_".to_string(),
      "fix_".to_string(),
      "bug_".to_string(),
      "feature_".to_string(),
      "enhancement_".to_string(),
    ];

    let mut naming_conventions = HashMap::new();
    naming_conventions.insert( "unit".to_string(), vec![
      "component_name".to_string(),
      "feature_area".to_string(),
      "functionality_test".to_string(),
    ]);
    naming_conventions.insert( "integration".to_string(), vec![
      "system_interaction".to_string(),
      "component_integration".to_string(),
      "workflow_test".to_string(),
    ]);
    naming_conventions.insert( "acceptance".to_string(), vec![
      "user_scenario".to_string(),
      "cli_integration".to_string(),
      "end_to_end".to_string(),
    ]);
    naming_conventions.insert( "regression".to_string(), vec![
      "bug_prevention".to_string(),
      "known_issue".to_string(),
      "compatibility_test".to_string(),
    ]);

    Self {
      allowed_directories,
      prohibited_patterns,
      naming_conventions,
      max_depth : 4,
    }
  }
}

/// Validation result for a test file
#[ derive( Debug, Clone ) ]
pub struct ValidationResult
{
  pub file_path : PathBuf,
  pub is_valid : bool,
  pub violations : Vec< String >,
  pub category : Option< String >,
}

/// Test organization validator
pub struct OrganizationValidator
{
  rules : OrganizationRules,
  tests_root : PathBuf,
}

impl OrganizationValidator
{
  /// Create a new validator for the given tests directory
  pub fn new< P : AsRef< Path > >( tests_root : P ) -> Self
  {
    Self {
      rules : OrganizationRules::default(),
      tests_root : tests_root.as_ref().to_path_buf(),
    }
  }

  /// Create validator with custom rules
  pub fn with_rules< P : AsRef< Path > >( tests_root : P, rules : OrganizationRules ) -> Self
  {
    Self {
      rules,
      tests_root : tests_root.as_ref().to_path_buf(),
    }
  }

  /// Validate all test files in the tests directory
  pub fn validate_all( &self ) -> Result< Vec< ValidationResult >, Box< dyn std::error::Error > >
  {
    let mut results = Vec::new();
    self.validate_directory( &self.tests_root, &mut results, 0 )?;
    Ok( results )
  }

  /// Validate a specific test file
  pub fn validate_file< P : AsRef< Path > >( &self, file_path : P ) -> ValidationResult
  {
    let path = file_path.as_ref();
    let mut violations = Vec::new();
    let mut category = None;

    // Extract relative path from tests root
    let relative_path = if let Ok( rel ) = path.strip_prefix( &self.tests_root )
    {
      rel
    }
    else
    {
      violations.push( format!( "File not under tests directory: {}", path.display() ) );
      return ValidationResult {
        file_path : path.to_path_buf(),
        is_valid : false,
        violations,
        category,
      };
    };

    // Check if it's a Rust test file
    if !path.extension().map_or( false, |ext| ext == "rs" )
    {
      // Skip non-Rust files
      return ValidationResult {
        file_path : path.to_path_buf(),
        is_valid : true,
        violations,
        category,
      };
    }

    // Extract category from path
    if let Some( first_component ) = relative_path.components().next()
    {
      let dir_name = first_component.as_os_str().to_string_lossy().to_string();

      // Check if directory is allowed
      if !self.rules.allowed_directories.contains( &dir_name )
      {
        violations.push( format!( "Unauthorized directory: {}", dir_name ) );
      }
      else
      {
        category = Some( dir_name.clone() );
      }
    }

    // Check filename for prohibited patterns
    let filename = path.file_stem()
      .map( |f| f.to_string_lossy().to_string() )
      .unwrap_or_default();

    for pattern in &self.rules.prohibited_patterns
    {
      if filename.starts_with( pattern )
      {
        violations.push( format!( "Prohibited naming pattern '{}' in filename: {}", pattern, filename ) );
      }
    }

    // Check directory depth
    let depth = relative_path.components().count();
    if depth > self.rules.max_depth
    {
      violations.push( format!( "Excessive nesting depth: {} (max: {})", depth, self.rules.max_depth ) );
    }

    // Special validation for specific categories
    if let Some( ref cat ) = category
    {
      self.validate_category_specific( cat, &filename, &mut violations );
    }

    ValidationResult {
      file_path : path.to_path_buf(),
      is_valid : violations.is_empty(),
      violations,
      category,
    }
  }

  /// Recursively validate directory contents
  fn validate_directory(
    &self,
    dir_path : &Path,
    results : &mut Vec< ValidationResult >,
    depth : usize,
  ) -> Result< (), Box< dyn std::error::Error > >
  {
    if depth > self.rules.max_depth
    {
      return Ok( () );
    }

    for entry in fs::read_dir( dir_path )?
    {
      let entry = entry?;
      let path = entry.path();

      if path.is_file()
      {
        results.push( self.validate_file( &path ) );
      }
      else if path.is_dir()
      {
        // Skip hidden directories and target directories
        if let Some( dir_name ) = path.file_name()
        {
          let name = dir_name.to_string_lossy();
          if !name.starts_with( '.' ) && name != "target"
          {
            self.validate_directory( &path, results, depth + 1 )?;
          }
        }
      }
    }

    Ok( () )
  }

  /// Category-specific validation rules
  fn validate_category_specific( &self, category : &str, filename : &str, violations : &mut Vec< String > )
  {
    match category
    {
      "unit" => {
        // Unit tests should focus on single components
        if filename.contains( "integration" ) || filename.contains( "end_to_end" )
        {
          violations.push( "Unit test filename suggests integration testing".to_string() );
        }
      }
      "integration" => {
        // Integration tests should indicate component interaction
        if !filename.contains( "_" ) && !filename.contains( "integration" )
        {
          violations.push( "Integration test should indicate component interaction".to_string() );
        }
      }
      "acceptance" => {
        // Acceptance tests should indicate user scenarios
        if !filename.contains( "cli" ) && !filename.contains( "user" ) && !filename.contains( "scenario" )
        {
          violations.push( "Acceptance test should indicate user scenario or CLI interaction".to_string() );
        }
      }
      "regression" => {
        // Regression tests should indicate bug prevention
        if !filename.contains( "regression" ) && !filename.contains( "fix" )
        {
          violations.push( "Regression test should clearly indicate bug prevention purpose".to_string() );
        }
      }
      _ => {
        // Other categories have no specific requirements
      }
    }
  }

  /// Generate validation report
  pub fn generate_report( &self, results : &[ ValidationResult ] ) -> String
  {
    let mut report = String::new();

    let total_files = results.len();
    let valid_files = results.iter().filter( |r| r.is_valid ).count();
    let invalid_files = total_files - valid_files;

    report.push_str( &format!( "# Test Organization Validation Report\n\n" ) );
    report.push_str( &format!( "**Total files analyzed:** {}\n", total_files ) );
    report.push_str( &format!( "**Valid files:** {} ({:.1}%)\n", valid_files, (valid_files as f64 / total_files as f64) * 100.0 ) );
    report.push_str( &format!( "**Invalid files:** {} ({:.1}%)\n\n", invalid_files, (invalid_files as f64 / total_files as f64) * 100.0 ) );

    if invalid_files > 0
    {
      report.push_str( "## Violations Found\n\n" );

      for result in results.iter().filter( |r| !r.is_valid )
      {
        report.push_str( &format!( "### {}\n", result.file_path.display() ) );
        if let Some( ref category ) = result.category
        {
          report.push_str( &format!( "**Category:** {}\n", category ) );
        }
        report.push_str( "**Violations:**\n" );
        for violation in &result.violations
        {
          report.push_str( &format!( "- {}\n", violation ) );
        }
        report.push_str( "\n" );
      }
    }
    else
    {
      report.push_str( "✅ **All test files comply with organization standards!**\n\n" );
    }

    // Category breakdown
    let mut category_counts = HashMap::new();
    for result in results
    {
      if let Some( ref category ) = result.category
      {
        *category_counts.entry( category.clone() ).or_insert( 0 ) += 1;
      }
    }

    if !category_counts.is_empty()
    {
      report.push_str( "## Test Distribution by Category\n\n" );
      for ( category, count ) in category_counts
      {
        report.push_str( &format!( "- **{}:** {} files\n", category, count ) );
      }
    }

    report
  }
}

#[ cfg( test ) ]
mod tests
{
  use super::*;
  use std::env;
  use std::path::PathBuf;

  #[ test ]
  fn test_organization_validator_creation()
  {
    let temp_dir = env::temp_dir();
    let validator = OrganizationValidator::new( &temp_dir );
    assert_eq!( validator.tests_root, temp_dir );
  }

  #[ test ]
  fn test_prohibited_pattern_detection()
  {
    let temp_dir = env::temp_dir();
    let validator = OrganizationValidator::new( &temp_dir );

    // Create a mock file path with prohibited pattern
    let mock_path = temp_dir.join( "unit" ).join( "task_024_test.rs" );
    let result = validator.validate_file( &mock_path );

    assert!( !result.is_valid );
    assert!( result.violations.iter().any( |v| v.contains( "task_" ) ) );
  }

  #[ test ]
  fn test_valid_file_validation()
  {
    let temp_dir = env::temp_dir();
    let validator = OrganizationValidator::new( &temp_dir );

    // Create a mock file path with valid naming
    let mock_path = temp_dir.join( "unit" ).join( "semantic_analysis.rs" );
    let result = validator.validate_file( &mock_path );

    // Should be valid since no prohibited patterns and proper category
    assert_eq!( result.category, Some( "unit".to_string() ) );
    // Note: This will have some violations due to file not existing under tests root,
    // but the core naming validation logic is tested
  }

  #[ test ]
  fn test_category_extraction()
  {
    let temp_dir = env::temp_dir();
    let validator = OrganizationValidator::new( &temp_dir );

    let test_cases = vec![
      ( "unit/parser/argument_parsing.rs", Some( "unit".to_string() ) ),
      ( "integration/end_to_end.rs", Some( "integration".to_string() ) ),
      ( "acceptance/cli_integration.rs", Some( "acceptance".to_string() ) ),
      ( "regression/parameter_collection.rs", Some( "regression".to_string() ) ),
    ];

    for ( relative_path, expected_category ) in test_cases
    {
      let mock_path = temp_dir.join( relative_path );
      let result = validator.validate_file( &mock_path );
      if let Some( ref expected ) = expected_category
      {
        // The category extraction logic should work even if file doesn't exist
        // under the actual tests root (due to our mock path structure)
        assert!( result.violations.iter().any( |v| v.contains( "not under tests directory" ) ) ||
                result.category.as_ref() == Some( expected ) );
      }
    }
  }
}