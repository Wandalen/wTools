//! Low-level Template API example
//!
//! Demonstrates using the Template<V,R,FS> API instead of `TemplateArchive`.
//! This provides more control over value types, renderers, and filesystems.
//!
//! Run with: cargo run --example `low_level_template`

use genfile_core::
{
  Template,
  HandlebarsRenderer,
  MemoryFileSystem,
  FileDescriptor,
  WriteMode,
  Value,
  FileSystem,
};
use std::path::PathBuf;

fn main() -> Result< (), Box< dyn core::error::Error > >
{
  println!( "=== Low-Level Template API Example ===" );
  println!();

  // Create renderer
  let renderer = HandlebarsRenderer::new();

  // Create two separate filesystems: one for templates, one for output
  let mut template_fs = MemoryFileSystem::new();

  // Prepare template files in the template filesystem
  template_fs.write(
    &PathBuf::from( "/templates/greeting.hbs" ),
    "Hello, {{name}}!\n"
  )?;

  template_fs.write(
    &PathBuf::from( "/templates/config.hbs" ),
    "server={{server}}\nport={{port}}\n"
  )?;

  println!( "Template files prepared in memory filesystem" );
  println!();

  // Create template with explicit types using the template filesystem
  let mut template = Template::< Value, _, _ >::new( renderer, template_fs );

  // Insert values
  template.insert_value( "name", Value::String( "World".into() ) );
  template.insert_value( "server", Value::String( "localhost".into() ) );
  template.insert_value( "port", Value::Number( 8080 ) );

  // Add file descriptors
  template.add_file( FileDescriptor
  {
    template_path: PathBuf::from( "/templates/greeting.hbs" ),
    file_path: PathBuf::from( "/output/greeting.txt" ),
    write_mode: WriteMode::Rewrite,
  });

  template.add_file( FileDescriptor
  {
    template_path: PathBuf::from( "/templates/config.hbs" ),
    file_path: PathBuf::from( "/output/config.txt" ),
    write_mode: WriteMode::Rewrite,
  });

  println!( "File descriptors added:" );
  println!( "  greeting.hbs -> greeting.txt" );
  println!( "  config.hbs -> config.txt" );
  println!();

  // Materialize (modifies the filesystem in place)
  println!( "Materializing templates..." );
  template.materialize()?;
  println!( "✅ Materialization complete" );
  println!();

  // Access the filesystem via reference to read results
  println!( "Generated files:" );
  println!();

  let greeting_content = template.filesystem().read( &PathBuf::from( "/output/greeting.txt" ) )?;
  println!( "--- /output/greeting.txt ---" );
  println!( "{greeting_content}" );
  println!();

  let config_content = template.filesystem().read( &PathBuf::from( "/output/config.txt" ) )?;
  println!( "--- /output/config.txt ---" );
  println!( "{config_content}" );
  println!();

  println!( "=== Advantages of Template API ===" );
  println!();
  println!( "• Generic over value types (V: TemplateValue)" );
  println!( "• Generic over renderers (R: TemplateRenderer)" );
  println!( "• Generic over filesystems (FS: FileSystem)" );
  println!( "• Direct control over materialization process" );
  println!( "• Useful for custom value types or renderers" );
  println!();

  println!( "=== When to use Template vs TemplateArchive ===" );
  println!();
  println!( "Use TemplateArchive when:" );
  println!( "  • You need serialization (JSON/YAML)" );
  println!( "  • You want self-contained archives" );
  println!( "  • You need parameter discovery and analysis" );
  println!( "  • Default Value type is sufficient" );
  println!();
  println!( "Use Template when:" );
  println!( "  • You need custom value types" );
  println!( "  • You need custom renderers" );
  println!( "  • You want fine-grained control" );
  println!( "  • Serialization is not needed" );
  println!();

  println!( "✅ Example completed successfully" );

  Ok( () )
}
