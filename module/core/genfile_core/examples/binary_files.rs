//! Binary file handling example
//!
//! Demonstrates working with binary files alongside text templates.
//! Shows how binary content is base64-encoded for serialization.
//!
//! Run with: cargo run --example `binary_files`

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
  println!( "=== Binary Files Example ===" );
  println!();

  // Create archive with both text and binary files
  let mut archive = TemplateArchive::new( "website" );

  // Add text template
  archive.add_text_file(
    PathBuf::from( "index.html" ),
    "<!DOCTYPE html>\n<html>\n<head><title>{{title}}</title></head>\n<body>\n  <h1>{{title}}</h1>\n  <img src=\"logo.png\" alt=\"Logo\">\n</body>\n</html>",
    WriteMode::Rewrite
  );

  // Add binary files (simulated image files)

  // PNG header (simplified)
  let png_bytes = vec![ 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A ];
  archive.add_binary_file(
    PathBuf::from( "logo.png" ),
    png_bytes
  );

  // JPEG header (simplified)
  let jpeg_bytes = vec![ 0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46 ];
  archive.add_binary_file(
    PathBuf::from( "background.jpg" ),
    jpeg_bytes
  );

  // ICO header (simplified)
  let ico_bytes = vec![ 0x00, 0x00, 0x01, 0x00, 0x01, 0x00 ];
  archive.add_binary_file(
    PathBuf::from( "favicon.ico" ),
    ico_bytes
  );

  println!( "Archive statistics:" );
  println!( "  Total files: {}", archive.file_count() );
  println!( "  Text files: {}", archive.text_file_count() );
  println!( "  Binary files: {}", archive.binary_file_count() );
  println!();

  // Set values
  archive.set_value( "title", Value::String( "My Website".into() ) );

  // Serialize to JSON (binary content is base64-encoded)
  let json = archive.to_json_pretty()?;
  println!( "JSON representation (truncated):" );
  let json_preview: String = json.chars().take( 500 ).collect();
  println!( "{json_preview}..." );
  println!();

  // Deserialize from JSON
  let restored = TemplateArchive::from_json( &json )?;
  println!( "✅ Successfully deserialized from JSON" );
  println!( "  Restored files: {}", restored.file_count() );
  println!();

  // Materialize
  let renderer = HandlebarsRenderer::new();
  let mut fs = MemoryFileSystem::new();

  restored.materialize_with_components(
    PathBuf::from( "/website" ).as_path(),
    &renderer,
    &mut fs
  )?;

  // Verify files were created
  println!( "Generated files:" );
  for path in [ "index.html", "logo.png", "background.jpg", "favicon.ico" ]
  {
    let full_path = PathBuf::from( format!( "/website/{path}" ) );
    let exists = fs.exists( &full_path );
    println!( "  {} - {}", path, if exists { "✅ exists" } else { "❌ missing" } );
  }
  println!();

  // Display generated HTML
  let html = fs.read( &PathBuf::from( "/website/index.html" ) )?;
  println!( "Generated HTML:" );
  println!( "{html}" );
  println!();

  println!( "✅ Example completed successfully" );

  Ok( () )
}
