/// Tests for Template holder (FR16, FR17, FR18, FR19)
use super :: *;
use std ::path ::PathBuf;

//

#[ test ]
fn template_can_be_created()
{
  // FR16, FR17: Template must hold renderer and filesystem
  let renderer = HandlebarsRenderer ::new();
  let filesystem = MemoryFileSystem ::new();

  let template: Template< Value, _, _ > = Template ::new( renderer, filesystem );

  // Just verify it compiles and creates
  drop( template );
}

#[ test ]
fn template_holds_values()
{
  // FR17: Template must hold values
  let renderer = HandlebarsRenderer ::new();
  let filesystem = MemoryFileSystem ::new();

  let mut template = Template ::new( renderer, filesystem );

  template.insert_value( "name", Value ::String( "test".into() ) );
  template.insert_value( "count", Value ::Number( 42 ) );

  // Verify values are stored
  assert!( template.has_value( "name" ) );
  assert!( template.has_value( "count" ) );
}

#[ test ]
fn template_materialize_simple()
{
  // FR18: Template::materialize() must render template and write to file
  let renderer = HandlebarsRenderer ::new();
  let mut filesystem = MemoryFileSystem ::new();

  // Set up template file
  let template_path = PathBuf ::from( "template.hbs" );
  filesystem.write( &template_path, "Hello {{name}}!" ).unwrap();

  let mut template = Template ::new( renderer, filesystem );
  template.insert_value( "name", Value ::String( "World".into() ) );

  // Add file descriptor
  let output_path = PathBuf ::from( "output.txt" );
  template.add_file( FileDescriptor
  {
    file_path: output_path.clone(),
    template_path: template_path.clone(),
    write_mode: WriteMode ::Rewrite,
  });

  // Materialize
  let result = template.materialize();
  assert!( result.is_ok() );

  // Verify output file was created with rendered content
  let fs_ref = template.filesystem();
  assert!( fs_ref.exists( &output_path ) );
  assert_eq!( fs_ref.read( &output_path ).unwrap(), "Hello World!" );
}

#[ test ]
fn template_materialize_multiple_files()
{
  // FR18: Should handle multiple file descriptors
  let renderer = HandlebarsRenderer ::new();
  let mut filesystem = MemoryFileSystem ::new();

  filesystem.write( &PathBuf ::from( "t1.hbs" ), "File 1: {{v}}" ).unwrap();
  filesystem.write( &PathBuf ::from( "t2.hbs" ), "File 2: {{v}}" ).unwrap();

  let mut template = Template ::new( renderer, filesystem );
  template.insert_value( "v", Value ::String( "data".into() ) );

  template.add_file( FileDescriptor
  {
    file_path: PathBuf ::from( "out1.txt" ),
    template_path: PathBuf ::from( "t1.hbs" ),
    write_mode: WriteMode ::Rewrite,
  });

  template.add_file( FileDescriptor
  {
    file_path: PathBuf ::from( "out2.txt" ),
    template_path: PathBuf ::from( "t2.hbs" ),
    write_mode: WriteMode ::Rewrite,
  });

  template.materialize().unwrap();

  let fs = template.filesystem();
  assert_eq!( fs.read( &PathBuf ::from( "out1.txt" ) ).unwrap(), "File 1: data" );
  assert_eq!( fs.read( &PathBuf ::from( "out2.txt" ) ).unwrap(), "File 2: data" );
}

#[ test ]
fn template_rewrite_mode_replaces_content()
{
  // FR18: Rewrite mode should completely replace file content
  let renderer = HandlebarsRenderer ::new();
  let mut filesystem = MemoryFileSystem ::new();

  let output_path = PathBuf ::from( "output.txt" );

  // Pre-existing file
  filesystem.write( &output_path, "old content that should be replaced" ).unwrap();

  filesystem.write( &PathBuf ::from( "t.hbs" ), "new content" ).unwrap();

  let mut template: Template< Value, _, _ > = Template ::new( renderer, filesystem );
  template.add_file( FileDescriptor
  {
    file_path: output_path.clone(),
    template_path: PathBuf ::from( "t.hbs" ),
    write_mode: WriteMode ::Rewrite,
  });

  template.materialize().unwrap();

  // Should have completely replaced content
  assert_eq!( template.filesystem().read( &output_path ).unwrap(), "new content" );
}
