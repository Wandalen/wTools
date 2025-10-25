//! `.new` command handler
//!
//! Creates new unilang project with correct structure, preventing common mistakes.

use crate::templates::{ cargo_toml, main_rs_minimal, main_rs_full, commands_yaml_minimal, commands_yaml_full };
use std::{ fs, path::{ Path, PathBuf } };

/// Parameters for `.new` command
#[derive( Debug )]
pub struct NewParams
{
  pub project : String,
  pub template : String,
  pub author : Option< String >,
  pub license : Option< String >,
  pub verbosity : u8,
}

impl NewParams
{
  /// Parse parameters from command arguments
  pub fn parse( args : &[ ( String, String ) ] ) -> Result< Self, String >
  {
    let mut project = None;
    let mut template = "minimal".to_string();
    let mut author = None;
    let mut license = None;
    let mut verbosity = 2u8;

    for ( key, value ) in args
    {
      match key.as_str()
      {
        "project" | "p" => project = Some( value.clone() ),
        "template" | "t" => template = value.clone(),
        "author" | "a" => author = Some( value.clone() ),
        "license" | "l" => license = Some( value.clone() ),
        "verbosity" | "v" => verbosity = validate_verbosity( value )?,
        _ => return Err( format!( "Unknown parameter: {}", key ) ),
      }
    }

    let project = project.ok_or( "Missing required parameter: project" )?;

    // Validate parameters
    validate_project_name( &project )?;
    validate_template( &template )?;

    Ok( Self { project, template, author, license, verbosity } )
  }
}

/// Validate project name
fn validate_project_name( name : &str ) -> Result< (), String >
{
  // Must not be empty
  if name.is_empty()
  {
    return Err( "Project name cannot be empty".to_string() );
  }

  // Length: 1-64 characters
  if name.len() > 64
  {
    return Err( "Project name too long (max 64 characters)".to_string() );
  }

  // Security: Prevent path traversal
  if name.contains( ".." ) || name.contains( "/" ) || name.contains( "\\" )
  {
    return Err( "Project name cannot contain path separators".to_string() );
  }

  // Must start with letter or underscore
  let first_char = name.chars().next().unwrap();
  if !first_char.is_ascii_alphabetic() && first_char != '_'
  {
    return Err( "Project name must start with letter or underscore".to_string() );
  }

  // Only alphanumeric, underscore, hyphen
  for ch in name.chars()
  {
    if !ch.is_ascii_alphanumeric() && ch != '_' && ch != '-'
    {
      return Err( format!( "Invalid character '{}' in project name", ch ) );
    }
  }

  Ok( () )
}

/// Validate template name
fn validate_template( template : &str ) -> Result< (), String >
{
  match template
  {
    "minimal" | "full" => Ok( () ),
    _ => Err( format!( "Unknown template '{}'. Valid: minimal, full", template ) ),
  }
}

/// Validate verbosity level
fn validate_verbosity( level : &str ) -> Result< u8, String >
{
  match level.parse::< u8 >()
  {
    Ok( n ) if n <= 5 => Ok( n ),
    Ok( n ) => Err( format!( "Verbosity must be 0-5, got {}", n ) ),
    Err( _ ) => Err( format!( "Invalid verbosity '{}', must be 0-5", level ) ),
  }
}

/// Execute `.new` command
pub fn execute( params : NewParams ) -> Result< i32, String >
{
  // Determine project path
  let project_path = PathBuf::from( &params.project );

  // Check if project already exists
  if project_path.exists()
  {
    return Err( format!( "Project directory '{}' already exists", params.project ) );
  }

  // Output based on verbosity
  if params.verbosity >= 3
  {
    eprintln!( "[INFO] Creating unilang project: {}", params.project );
    eprintln!( "[DEBUG] Validating project name..." );
    eprintln!( "[DEBUG] Project name '{}' is valid", params.project );
  }

  // Create project directory
  if params.verbosity >= 3
  {
    eprintln!( "[DEBUG] Creating directory: {}/", params.project );
  }

  fs::create_dir_all( &project_path )
    .map_err( |e| format!( "Failed to create directory: {}", e ) )?;

  // Generate Cargo.toml
  if params.verbosity >= 3
  {
    eprintln!( "[DEBUG] Generating Cargo.toml from template..." );
  }

  let cargo_toml_content = cargo_toml(
    &params.project,
    params.author.as_deref(),
    params.license.as_deref()
  );

  write_file( &project_path.join( "Cargo.toml" ), &cargo_toml_content, params.verbosity )?;

  // Generate src/main.rs
  if params.verbosity >= 3
  {
    eprintln!( "[DEBUG] Generating src/main.rs from template..." );
  }

  let src_dir = project_path.join( "src" );
  fs::create_dir_all( &src_dir )
    .map_err( |e| format!( "Failed to create src directory: {}", e ) )?;

  let main_rs_content = match params.template.as_str()
  {
    "minimal" => main_rs_minimal(),
    "full" => main_rs_full(),
    _ => main_rs_minimal(), // Default to minimal
  };

  write_file( &src_dir.join( "main.rs" ), main_rs_content, params.verbosity )?;

  // Generate commands.yaml
  if params.verbosity >= 3
  {
    eprintln!( "[DEBUG] Generating commands.yaml from template..." );
  }

  let commands_yaml_content = match params.template.as_str()
  {
    "minimal" => commands_yaml_minimal(),
    "full" => commands_yaml_full(),
    _ => commands_yaml_minimal(),
  };

  write_file( &project_path.join( "commands.yaml" ), commands_yaml_content, params.verbosity )?;

  // Output based on verbosity
  match params.verbosity
  {
    0 =>
    {
      // Silent - no output
    }
    1 =>
    {
      // Single line
      println!( "Created {}/", params.project );
    }
    2 =>
    {
      // Concise (default)
      println!( "Created project: {}/", params.project );
      println!( "  ├── Cargo.toml (unilang = \"0.30\" with warnings)" );
      println!( "  ├── src/main.rs (15-line minimal example)" );
      println!( "  └── commands.yaml (example command)" );
      println!();
      println!( "✅ You did NOT need to write build.rs" );
      println!( "✅ You did NOT need to add serde_yaml, walkdir, or phf" );
      println!();
      println!( "Next steps:" );
      println!( "  cd {}", params.project );
      println!( "  cargo run -- .help" );
      println!();
      println!( "⚠️  IMPORTANT:" );
      println!( "  - Do NOT create build.rs" );
      println!( "  - Do NOT add serde_yaml, walkdir, or phf to Cargo.toml" );
      println!( "  - Use CommandRegistry::with_static_commands() (not ::new())" );
    }
    _ =>
    {
      // Debug (3+)
      if params.verbosity >= 3
      {
        eprintln!( "✅ Created project: {}/ (3 files)", params.project );
      }
      println!( "Created project: {}/", params.project );
      println!( "  ├── Cargo.toml" );
      println!( "  ├── src/main.rs" );
      println!( "  └── commands.yaml" );
      println!();
      println!( "✅ You did NOT need to write build.rs" );
      println!( "✅ You did NOT need to add serde_yaml, walkdir, or phf" );
      println!();
      println!( "Next steps:" );
      println!( "  cd {}", params.project );
      println!( "  cargo run -- .help" );
    }
  }

  Ok( 0 ) // Success
}

/// Write file with verbosity logging
fn write_file( path : &Path, content : &str, verbosity : u8 ) -> Result< (), String >
{
  if verbosity >= 3
  {
    eprintln!( "[DEBUG] Writing: {}", path.display() );
  }

  fs::write( path, content )
    .map_err( |e| format!( "Failed to write {}: {}", path.display(), e ) )
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_validate_project_name_valid()
  {
    assert!( validate_project_name( "my-cli" ).is_ok() );
    assert!( validate_project_name( "my_cli" ).is_ok() );
    assert!( validate_project_name( "mycli" ).is_ok() );
    assert!( validate_project_name( "_private" ).is_ok() );
  }

  #[test]
  fn test_validate_project_name_empty()
  {
    assert!( validate_project_name( "" ).is_err() );
  }

  #[test]
  fn test_validate_project_name_too_long()
  {
    let long_name = "a".repeat( 65 );
    assert!( validate_project_name( &long_name ).is_err() );
  }

  #[test]
  fn test_validate_project_name_path_traversal()
  {
    assert!( validate_project_name( "../etc" ).is_err() );
    assert!( validate_project_name( "foo/../bar" ).is_err() );
    assert!( validate_project_name( "/absolute" ).is_err() );
    assert!( validate_project_name( "foo\\bar" ).is_err() );
  }

  #[test]
  fn test_validate_project_name_invalid_chars()
  {
    assert!( validate_project_name( "my@cli" ).is_err() );
    assert!( validate_project_name( "my cli" ).is_err() );
    assert!( validate_project_name( "my.cli" ).is_err() );
  }

  #[test]
  fn test_validate_project_name_must_start_with_letter_or_underscore()
  {
    assert!( validate_project_name( "1cli" ).is_err() );
    assert!( validate_project_name( "-cli" ).is_err() );
  }

  #[test]
  fn test_validate_template()
  {
    assert!( validate_template( "minimal" ).is_ok() );
    assert!( validate_template( "full" ).is_ok() );
    assert!( validate_template( "invalid" ).is_err() );
  }

  #[test]
  fn test_validate_verbosity()
  {
    assert_eq!( validate_verbosity( "0" ).unwrap(), 0 );
    assert_eq!( validate_verbosity( "5" ).unwrap(), 5 );
    assert!( validate_verbosity( "6" ).is_err() );
    assert!( validate_verbosity( "abc" ).is_err() );
  }

  #[test]
  fn test_params_parse_minimal()
  {
    let args = vec![
      ( "project".to_string(), "my-cli".to_string() ),
    ];

    let params = NewParams::parse( &args ).unwrap();
    assert_eq!( params.project, "my-cli" );
    assert_eq!( params.template, "minimal" );
    assert_eq!( params.verbosity, 2 );
  }

  #[test]
  fn test_params_parse_full()
  {
    let args = vec![
      ( "project".to_string(), "my-api".to_string() ),
      ( "template".to_string(), "full".to_string() ),
      ( "author".to_string(), "John Doe".to_string() ),
      ( "license".to_string(), "Apache-2.0".to_string() ),
      ( "verbosity".to_string(), "1".to_string() ),
    ];

    let params = NewParams::parse( &args ).unwrap();
    assert_eq!( params.project, "my-api" );
    assert_eq!( params.template, "full" );
    assert_eq!( params.author.unwrap(), "John Doe" );
    assert_eq!( params.license.unwrap(), "Apache-2.0" );
    assert_eq!( params.verbosity, 1 );
  }

  #[test]
  fn test_params_parse_missing_project()
  {
    let args = vec![
      ( "template".to_string(), "minimal".to_string() ),
    ];

    assert!( NewParams::parse( &args ).is_err() );
  }

  #[test]
  fn test_params_parse_invalid_project_name()
  {
    let args = vec![
      ( "project".to_string(), "../invalid".to_string() ),
    ];

    assert!( NewParams::parse( &args ).is_err() );
  }
}
