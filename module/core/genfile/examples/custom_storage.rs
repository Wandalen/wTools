//! Custom storage backend example
//!
//! Demonstrates implementing the ContentStorage trait to create
//! a custom storage backend (e.g., cloud storage, database, etc.).
//!
//! Run with: cargo run --example custom_storage

use genfile_core::
{
  TemplateArchive,
  WriteMode,
  Value,
  HandlebarsRenderer,
  ContentStorage,
  FileContent,
  DefaultContentResolver,
  Error,
};
use std::path::{ Path, PathBuf };
use std::collections::HashMap;

/// Custom in-memory storage backend
///
/// This example shows how to implement ContentStorage for a custom backend.
/// In real applications, this could be S3, Azure Blob Storage, database, etc.
struct CloudStorage
{
  /// Simulated cloud storage
  files: HashMap< String, FileContent >,
  /// Upload counter
  upload_count: usize,
}

impl CloudStorage
{
  fn new() -> Self
  {
    Self
    {
      files: HashMap::new(),
      upload_count: 0,
    }
  }

  fn list_files( &self ) -> Vec< String >
  {
    self.files.keys().cloned().collect()
  }
}

impl ContentStorage for CloudStorage
{
  fn store( &mut self, path: &Path, content: &FileContent ) -> Result< (), Error >
  {
    let key = path.to_string_lossy().to_string();

    println!( "☁️  Uploading to cloud: {}", key );

    // Simulate upload
    self.files.insert( key.clone(), content.clone() );
    self.upload_count += 1;

    let size_desc = match content
    {
      FileContent::Text( s ) => format!( "{} chars", s.len() ),
      FileContent::Binary( b ) => format!( "{} bytes", b.len() ),
    };
    println!( "   ✅ Upload complete ({})", size_desc );

    Ok( () )
  }
}

fn main() -> Result< (), Box< dyn std::error::Error > >
{
  println!( "=== Custom Storage Backend Example ===" );
  println!();

  // Create archive
  let mut archive = TemplateArchive::new( "cloud-app" );
  archive.set_version( "1.0.0" );

  // Add files
  archive.add_text_file(
    PathBuf::from( "config/app.conf" ),
    "name={{app_name}}\nregion={{region}}\n",
    WriteMode::Rewrite
  );

  archive.add_text_file(
    PathBuf::from( "config/database.conf" ),
    "host={{db_host}}\nport={{db_port}}\n",
    WriteMode::Rewrite
  );

  archive.add_text_file(
    PathBuf::from( "README.md" ),
    "# {{app_name}}\n\nDeployed to: {{region}}\n",
    WriteMode::Rewrite
  );

  // Add binary file (simulated logo)
  let logo_bytes = vec![ 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A ];
  archive.add_binary_file(
    PathBuf::from( "assets/logo.png" ),
    logo_bytes
  );

  // Set values
  archive.set_value( "app_name", Value::String( "CloudApp".into() ) );
  archive.set_value( "region", Value::String( "us-west-2".into() ) );
  archive.set_value( "db_host", Value::String( "db.example.com".into() ) );
  archive.set_value( "db_port", Value::Number( 5432 ) );

  println!( "Archive prepared:" );
  println!( "  Files: {}", archive.file_count() );
  println!( "  Text files: {}", archive.text_file_count() );
  println!( "  Binary files: {}", archive.binary_file_count() );
  println!();

  // Create custom storage backend
  let mut cloud = CloudStorage::new();
  let renderer = HandlebarsRenderer::new();
  let resolver = DefaultContentResolver::new();

  println!( "=== Materializing to Cloud Storage ===" );
  println!();

  // Materialize to custom storage
  let report = archive.materialize_with_storage(
    Path::new( "s3://my-bucket/app-v1.0.0" ),
    &renderer,
    &mut cloud,
    &resolver
  )?;

  println!();
  println!( "=== Upload Complete ===" );
  println!();
  println!( "Materialization report:" );
  println!( "  Files created: {}", report.files_created.len() );
  println!( "  Total uploads: {}", cloud.upload_count );
  println!();

  println!( "Files in cloud storage:" );
  for file in cloud.list_files()
  {
    println!( "  ☁️  {}", file );
  }
  println!();

  // Verify content
  if let Some( readme_content ) = cloud.files.get( "s3://my-bucket/app-v1.0.0/README.md" )
  {
    match readme_content
    {
      FileContent::Text( text ) =>
      {
        println!( "Sample file content (README.md):" );
        println!( "{}", text );
        println!();
      },
      _ => {},
    }
  }

  println!( "✅ Example completed successfully" );
  println!();
  println!( "This demonstrates how to implement ContentStorage for:" );
  println!( "  • Cloud storage (AWS S3, Azure Blob, Google Cloud Storage)" );
  println!( "  • Databases (PostgreSQL, MongoDB, etc.)" );
  println!( "  • Custom backends (FTP, WebDAV, etc.)" );

  Ok( () )
}
