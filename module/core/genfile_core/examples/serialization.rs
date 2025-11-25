//! Serialization example
//!
//! Demonstrates JSON and YAML serialization/deserialization of template archives.
//! Shows how archives are self-contained with files, parameters, and values.
//!
//! Run with: cargo run --example serialization

use genfile_core::
{
  TemplateArchive,
  ParameterDescriptor,
  WriteMode,
  Value,
};
use std::path::PathBuf;

fn main() -> Result< (), Box< dyn core::error::Error > >
{
  println!( "=== Serialization Example ===" );
  println!();

  // Create a complete archive with files, parameters, and values
  let mut archive = TemplateArchive::new( "config-generator" );
  archive.set_version( "2.0.0" );
  archive.set_description( "Configuration file generator" );

  // Add template files
  archive.add_text_file(
    PathBuf::from( "app.conf" ),
    "server_name={{server_name}}\nport={{port}}\ndebug={{debug}}\n",
    WriteMode::Rewrite
  );

  archive.add_text_file(
    PathBuf::from( "logging.conf" ),
    "level={{log_level}}\noutput={{log_output}}\n",
    WriteMode::Rewrite
  );

  // Add parameters
  archive.add_parameter( ParameterDescriptor
  {
    parameter: "server_name".into(),
    is_mandatory: true,
    default_value: Some( "localhost".into() ),
    description: Some( "Server hostname".into() ),
  });

  archive.add_parameter( ParameterDescriptor
  {
    parameter: "port".into(),
    is_mandatory: true,
    default_value: Some( "8080".into() ),
    description: Some( "Server port".into() ),
  });

  archive.add_parameter( ParameterDescriptor
  {
    parameter: "debug".into(),
    is_mandatory: false,
    default_value: Some( "false".into() ),
    description: Some( "Debug mode".into() ),
  });

  archive.add_parameter( ParameterDescriptor
  {
    parameter: "log_level".into(),
    is_mandatory: false,
    default_value: Some( "info".into() ),
    description: Some( "Logging level".into() ),
  });

  archive.add_parameter( ParameterDescriptor
  {
    parameter: "log_output".into(),
    is_mandatory: false,
    default_value: Some( "stdout".into() ),
    description: Some( "Log output destination".into() ),
  });

  // Set values
  archive.set_value( "server_name", Value::String( "production.example.com".into() ) );
  archive.set_value( "port", Value::Number( 443 ) );
  archive.set_value( "debug", Value::Bool( false ) );
  archive.set_value( "log_level", Value::String( "warn".into() ) );
  archive.set_value( "log_output", Value::String( "/var/log/app.log".into() ) );

  println!( "Original archive:" );
  println!( "  Name: {}", archive.name );
  println!( "  Version: {}", archive.version );
  println!( "  Files: {}", archive.file_count() );
  println!( "  Parameters: {}", archive.parameters.descriptors.len() );
  println!();

  // === JSON Serialization ===

  println!( "=== JSON Serialization ===" );
  println!();

  let json = archive.to_json_pretty()?;
  println!( "JSON output (first 800 chars):" );
  let json_preview: String = json.chars().take( 800 ).collect();
  println!( "{json_preview}" );
  println!( "..." );
  println!();

  // Deserialize from JSON
  let from_json = TemplateArchive::from_json( &json )?;
  println!( "✅ Deserialized from JSON" );
  println!( "  Name: {}", from_json.name );
  println!( "  Version: {}", from_json.version );
  println!( "  Files: {}", from_json.file_count() );
  println!();

  // === YAML Serialization ===

  println!( "=== YAML Serialization ===" );
  println!();

  let yaml = archive.to_yaml()?;
  println!( "YAML output (first 1000 chars):" );
  let yaml_preview: String = yaml.chars().take( 1000 ).collect();
  println!( "{yaml_preview}" );
  println!( "..." );
  println!();

  // Deserialize from YAML
  let from_yaml = TemplateArchive::from_yaml( &yaml )?;
  println!( "✅ Deserialized from YAML" );
  println!( "  Name: {}", from_yaml.name );
  println!( "  Version: {}", from_yaml.version );
  println!( "  Files: {}", from_yaml.file_count() );
  println!();

  // === File I/O ===

  println!( "=== File I/O ===" );
  println!();

  // These would work with real filesystem:
  // archive.save_to_file( "archive.json" )?;
  // archive.save_to_file( "archive.yaml" )?;
  // let loaded = TemplateArchive::load_from_file( "archive.json" )?;

  println!( "Archive can be saved/loaded with:" );
  println!( "  archive.save_to_file( \"archive.json\" )?;" );
  println!( "  archive.save_to_file( \"archive.yaml\" )?;" );
  println!( "  TemplateArchive::load_from_file( \"archive.json\" )?;" );
  println!();

  println!( "Format is auto-detected from file extension (.json, .yaml, .yml)" );
  println!();

  println!( "✅ Example completed successfully" );

  Ok( () )
}
