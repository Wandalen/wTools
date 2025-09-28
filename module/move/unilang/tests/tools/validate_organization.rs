//! CLI tool for validating test organization
//!
//! This executable validates that all test files follow the systematic
//! organization principles and can be run as part of CI/CD pipelines.

use std::env;
use std::path::Path;
use std::process;

mod test_organization_validator;
use test_organization_validator::OrganizationValidator;

fn main()
{
  let args : Vec< String > = env::args().collect();

  let tests_dir = if args.len() > 1
  {
    &args[ 1 ]
  }
  else
  {
    // Default to current directory's tests subdirectory
    "."
  };

  let tests_path = Path::new( tests_dir );

  if !tests_path.exists()
  {
    eprintln!( "Error: Tests directory '{}' does not exist", tests_dir );
    process::exit( 1 );
  }

  println!( "🔍 Validating test organization in: {}", tests_path.display() );
  println!( "📋 Checking compliance with systematic organization standards...\n" );

  let validator = OrganizationValidator::new( tests_path );

  match validator.validate_all()
  {
    Ok( results ) => {
      let report = validator.generate_report( &results );
      println!( "{}", report );

      let invalid_count = results.iter().filter( |r| !r.is_valid ).count();

      if invalid_count > 0
      {
        eprintln!( "\n❌ Validation failed: {} files violate organization standards", invalid_count );
        process::exit( 1 );
      }
      else
      {
        println!( "\n✅ All test files comply with organization standards!" );
        process::exit( 0 );
      }
    }
    Err( error ) => {
      eprintln!( "Error during validation: {}", error );
      process::exit( 1 );
    }
  }
}