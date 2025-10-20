//! Archive with parameter definitions and discovery
//!
//! Demonstrates parameter management: defining parameters with metadata,
//! discovering parameters from templates, and analyzing parameter usage.
//!
//! Run with: cargo run --example archive_with_parameters

use genfile_core::
{
  TemplateArchive,
  ParameterDescriptor,
  WriteMode,
  Value,
  HandlebarsRenderer,
  MemoryFileSystem,
  FileSystem,
};
use std::path::PathBuf;

fn main() -> Result< (), Box< dyn std::error::Error > >
{
  println!( "=== Archive with Parameters Example ===" );
  println!();

  // Create archive with multiple template files
  let mut archive = TemplateArchive::new( "rust-project" );

  // Add template files with various parameters
  archive.add_text_file(
    PathBuf::from( "Cargo.toml" ),
    "[package]\nname = \"{{project_name}}\"\nversion = \"{{version}}\"\n",
    WriteMode::Rewrite
  );

  archive.add_text_file(
    PathBuf::from( "src/main.rs" ),
    "fn main() {\n    println!(\"{{greeting}}\");\n}\n",
    WriteMode::Rewrite
  );

  archive.add_text_file(
    PathBuf::from( "README.md" ),
    "# {{project_name}}\n\n{{description}}\n\n## Version\n\n{{version}}\n",
    WriteMode::Rewrite
  );

  // Discover all parameters used in templates
  let discovered = archive.discover_parameters();
  println!( "Discovered parameters: {:?}", discovered );
  println!();

  // Add parameter definitions with metadata
  archive.add_parameter( ParameterDescriptor
  {
    parameter: "project_name".into(),
    is_mandatory: true,
    default_value: None,
    description: Some( "Name of the Rust project".into() ),
  });

  archive.add_parameter( ParameterDescriptor
  {
    parameter: "version".into(),
    is_mandatory: true,
    default_value: Some( "0.1.0".into() ),
    description: Some( "Project version following semver".into() ),
  });

  archive.add_parameter( ParameterDescriptor
  {
    parameter: "description".into(),
    is_mandatory: false,
    default_value: Some( "A Rust project".into() ),
    description: Some( "Project description for README".into() ),
  });

  archive.add_parameter( ParameterDescriptor
  {
    parameter: "greeting".into(),
    is_mandatory: false,
    default_value: Some( "Hello, world!".into() ),
    description: Some( "Greeting message".into() ),
  });

  // Analyze parameter usage
  let usage = archive.analyze_parameter_usage();
  println!( "Parameter usage analysis:" );
  for ( param, files ) in &usage
  {
    println!( "  {} -> used in {} file(s)", param, files.len() );
  }
  println!();

  // Check for undefined parameters (used but not defined)
  let undefined = archive.get_undefined_parameters();
  if undefined.is_empty()
  {
    println!( "✅ All parameters are defined" );
  }
  else
  {
    println!( "⚠️  Undefined parameters: {:?}", undefined );
  }
  println!();

  // Check for unused parameters (defined but not used)
  let unused = archive.get_unused_parameters();
  if unused.is_empty()
  {
    println!( "✅ All defined parameters are used" );
  }
  else
  {
    println!( "⚠️  Unused parameters: {:?}", unused );
  }
  println!();

  // Set parameter values
  archive.set_value( "project_name", Value::String( "awesome-app".into() ) );
  archive.set_value( "version", Value::String( "1.2.3".into() ) );
  archive.set_value( "description", Value::String( "An awesome Rust application".into() ) );
  archive.set_value( "greeting", Value::String( "Hello from awesome-app!".into() ) );

  // Materialize
  let renderer = HandlebarsRenderer::new();
  let mut fs = MemoryFileSystem::new();

  archive.materialize_with_components(
    PathBuf::from( "/project" ).as_path(),
    &renderer,
    &mut fs
  )?;

  // Display generated files
  println!( "Generated files:" );
  println!();

  println!( "--- Cargo.toml ---" );
  println!( "{}", fs.read( &PathBuf::from( "/project/Cargo.toml" ) )? );
  println!();

  println!( "--- src/main.rs ---" );
  println!( "{}", fs.read( &PathBuf::from( "/project/src/main.rs" ) )? );
  println!();

  println!( "--- README.md ---" );
  println!( "{}", fs.read( &PathBuf::from( "/project/README.md" ) )? );
  println!();

  println!( "✅ Example completed successfully" );

  Ok( () )
}
