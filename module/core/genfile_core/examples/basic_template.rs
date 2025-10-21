//! Basic template materialization example
//!
//! Demonstrates the simplest use case: creating a template archive,
//! adding files, setting values, and materializing to disk.
//!
//! Run with: cargo run --example `basic_template`

use genfile_core::
{
  TemplateArchive,
  WriteMode,
  Value,
  HandlebarsRenderer,
  MemoryFileSystem,
  FileSystem,
};
use std::path::PathBuf;

fn main() -> Result< (), Box< dyn core::error::Error > >
{
  println!( "=== Basic Template Materialization Example ===" );
  println!();

  // Create a new template archive
  let mut archive = TemplateArchive::new( "hello-world" );
  archive.set_version( "1.0.0" );
  archive.set_description( "Simple greeting template" );

  // Add a text template file
  archive.add_text_file(
    PathBuf::from( "greeting.txt" ),
    "Hello, {{name}}!\nWelcome to {{project}}.",
    WriteMode::Rewrite
  );

  // Set parameter values
  archive.set_value( "name", Value::String( "Alice".into() ) );
  archive.set_value( "project", Value::String( "genfile_core".into() ) );

  println!( "Archive name: {}", archive.name );
  println!( "Version: {}", archive.version );
  println!( "Files: {}", archive.file_count() );
  println!();

  // Materialize to memory filesystem
  let renderer = HandlebarsRenderer::new();
  let mut fs = MemoryFileSystem::new();

  let report = archive.materialize_with_components(
    PathBuf::from( "/output" ).as_path(),
    &renderer,
    &mut fs
  )?;

  println!( "Materialization complete!" );
  println!( "Files created: {}", report.files_created.len() );
  println!();

  // Read and display the generated file
  let content = fs.read( &PathBuf::from( "/output/greeting.txt" ) )?;
  println!( "Generated content:" );
  println!( "{content}" );
  println!();

  println!( "✅ Example completed successfully" );

  Ok( () )
}
