//! External content sources example
//!
//! Demonstrates using `FileRef` and `ContentSource` to reference external files
//! instead of embedding content directly in the archive.
//!
//! Run with: cargo run --example `external_content`

use genfile_core::
{
  TemplateArchive,
  FileRef,
  InlineContent,
  FileContent,
  WriteMode,
  Value,
  HandlebarsRenderer,
  MemoryFileSystem,
  DefaultContentResolver,
  FileSystem,
};
use std::path::PathBuf;

fn main() -> Result< (), Box< dyn core::error::Error > >
{
  println!( "=== External Content Sources Example ===" );
  println!();

  // Create archive
  let mut archive = TemplateArchive::new( "docs" );

  // Add file with inline content (normal way)
  archive.add_file_from(
    PathBuf::from( "inline.txt" ),
    InlineContent::new( FileContent::Text( "This is inline content".into() ) ),
    WriteMode::Rewrite
  );

  // Add file referencing external file
  archive.add_file_from(
    PathBuf::from( "external_header.txt" ),
    FileRef::new( PathBuf::from( "/templates/header.hbs" ) ),
    WriteMode::Rewrite
  );

  archive.add_file_from(
    PathBuf::from( "external_footer.txt" ),
    FileRef::new( PathBuf::from( "/templates/footer.hbs" ) ),
    WriteMode::Rewrite
  );

  println!( "Archive with mixed content sources:" );
  println!( "  Total files: {}", archive.file_count() );
  println!();

  // === Internalization Example ===

  println!( "=== Internalization (fetch and embed) ===" );
  println!();

  // Create a mock filesystem with template content
  let mut mock_fs = MemoryFileSystem::new();
  mock_fs.write(
    &PathBuf::from( "/templates/header.hbs" ),
    "=== {{title}} ===\n"
  )?;
  mock_fs.write(
    &PathBuf::from( "/templates/footer.hbs" ),
    "© {{year}} {{author}}\n"
  )?;

  // Create resolver that can fetch from our mock filesystem
  let _resolver = DefaultContentResolver::new();

  // Note: DefaultContentResolver doesnt support actual file fetching,
  // but demonstrates the API. In real usage, implement custom ContentResolver
  // that reads from disk, HTTP, database, etc.

  println!( "To internalize external content:" );
  println!( "  let resolver = CustomContentResolver::new();" );
  println!( "  archive.internalize( &resolver )?;" );
  println!();
  println!( "This fetches all external content and embeds it in the archive." );
  println!();

  // === Externalization Example ===

  println!( "=== Externalization (extract to files) ===" );
  println!();

  // Create archive with inline content
  let mut inline_archive = TemplateArchive::new( "app" );
  inline_archive.add_text_file(
    PathBuf::from( "config.txt" ),
    "app={{app_name}}\nversion={{version}}\n",
    WriteMode::Rewrite
  );
  inline_archive.add_text_file(
    PathBuf::from( "readme.txt" ),
    "# {{app_name}}\n",
    WriteMode::Rewrite
  );

  println!( "Archive before externalization:" );
  println!( "  Total files: {}", inline_archive.file_count() );
  println!();

  // Externalize would extract inline content to separate files
  println!( "To externalize inline content:" );
  println!( "  let storage = CustomContentStorage::new();" );
  println!( "  archive.externalize( &storage, Path::new( \"/templates\" ) )?;" );
  println!();
  println!( "This extracts inline content to external files and updates references." );
  println!();

  // === Serialization with External Sources ===

  println!( "=== Serialization ===" );
  println!();

  let json = archive.to_json_pretty()?;
  println!( "JSON with external sources (first 600 chars):" );
  let json_preview: String = json.chars().take( 600 ).collect();
  println!( "{json_preview}" );
  println!( "..." );
  println!();

  println!( "External file references are preserved in serialization." );
  println!( "FileRef stores the path, not the content." );
  println!();

  // === Materialization ===

  println!( "=== Materialization ===" );
  println!();

  // For demonstration, use inline content version
  let mut demo_archive = TemplateArchive::new( "demo" );
  demo_archive.add_text_file(
    PathBuf::from( "output.txt" ),
    "Title: {{title}}\nAuthor: {{author}}\nYear: {{year}}\n",
    WriteMode::Rewrite
  );

  demo_archive.set_value( "title", Value::String( "External Content Guide".into() ) );
  demo_archive.set_value( "author", Value::String( "genfile_core".into() ) );
  demo_archive.set_value( "year", Value::Number( 2024 ) );

  let renderer = HandlebarsRenderer::new();
  let mut fs = MemoryFileSystem::new();

  demo_archive.materialize_with_components(
    PathBuf::from( "/output" ).as_path(),
    &renderer,
    &mut fs
  )?;

  let content = fs.read( &PathBuf::from( "/output/output.txt" ) )?;
  println!( "Generated content:" );
  println!( "{content}" );
  println!();

  println!( "✅ Example completed successfully" );

  Ok( () )
}
