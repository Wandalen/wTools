/// Integration tests combining multiple components
use super :: *;
use std ::path ::PathBuf;

//

#[ test ]
fn end_to_end_single_file_generation()
{
  // Complete workflow: create template, add values, generate file
  let renderer = HandlebarsRenderer ::new();
  let mut filesystem = MemoryFileSystem ::new();

  // Setup template
  let template_content = r#"
# Configuration
name = "{{project_name}}"
version = {{version}}
enabled = {{enabled}}
"#;

  filesystem.write( &PathBuf ::from( "config.toml.hbs" ), template_content ).unwrap();

  // Create template instance
  let mut template: Template< Value, _, _ > = Template ::new( renderer, filesystem );

  // Add values
  template.insert_value( "project_name", Value ::String( "my_project".into() ) );
  template.insert_value( "version", Value ::Number( 1 ) );
  template.insert_value( "enabled", Value ::Bool( true ) );

  // Add file descriptor
  template.add_file( FileDescriptor
  {
    file_path: PathBuf ::from( "output/config.toml" ),
    template_path: PathBuf ::from( "config.toml.hbs" ),
    write_mode: WriteMode ::Rewrite,
  });

  // Generate
  let result = template.materialize();
  assert!( result.is_ok() );

  // Verify output
  let content = template.filesystem().read( &PathBuf ::from( "output/config.toml" ) ).unwrap();
  assert!( content.contains( r#"name = "my_project""# ) );
  assert!( content.contains( "version = 1" ) );
  assert!( content.contains( "enabled = true" ) );
}

#[ test ]
fn end_to_end_multiple_file_generation()
{
  // Generate multiple related files in one pass
  let renderer = HandlebarsRenderer ::new();
  let mut filesystem = MemoryFileSystem ::new();

  // Setup templates
  filesystem.write( &PathBuf ::from( "readme.md.hbs" ), "# {{name}}\n\n{{description}}" ).unwrap();
  filesystem.write( &PathBuf ::from( "cargo.toml.hbs" ), "[package]\nname = \"{{name}}\"" ).unwrap();
  filesystem.write( &PathBuf ::from( "lib.rs.hbs" ), "//! {{description}}\n\npub fn hello() {}" ).unwrap();

  let mut template: Template< Value, _, _ > = Template ::new( renderer, filesystem );

  template.insert_value( "name", Value ::String( "my_lib".into() ) );
  template.insert_value( "description", Value ::String( "A test library".into() ) );

  // Add all files
  template.add_file( FileDescriptor
  {
    file_path: PathBuf ::from( "readme.md" ),
    template_path: PathBuf ::from( "readme.md.hbs" ),
    write_mode: WriteMode ::Rewrite,
  });

  template.add_file( FileDescriptor
  {
    file_path: PathBuf ::from( "Cargo.toml" ),
    template_path: PathBuf ::from( "cargo.toml.hbs" ),
    write_mode: WriteMode ::Rewrite,
  });

  template.add_file( FileDescriptor
  {
    file_path: PathBuf ::from( "src/lib.rs" ),
    template_path: PathBuf ::from( "lib.rs.hbs" ),
    write_mode: WriteMode ::Rewrite,
  });

  template.materialize().unwrap();

  // Verify all files generated
  let fs = template.filesystem();
  assert!( fs.exists( &PathBuf ::from( "readme.md" ) ) );
  assert!( fs.exists( &PathBuf ::from( "Cargo.toml" ) ) );
  assert!( fs.exists( &PathBuf ::from( "src/lib.rs" ) ) );

  // Verify content
  let readme = fs.read( &PathBuf ::from( "readme.md" ) ).unwrap();
  assert!( readme.contains( "# my_lib" ) );

  let cargo = fs.read( &PathBuf ::from( "Cargo.toml" ) ).unwrap();
  assert!( cargo.contains( r#"name = "my_lib""# ) );
}

#[ test ]
fn custom_value_type_integration()
{
  // Verify custom value types work with the full system
  // Use a simple string-wrapping type that serializes as a string
  #[ derive( Clone ) ]
  struct CustomValue( String );

  impl serde ::Serialize for CustomValue
  {
    fn serialize< S >( &self, serializer: S ) -> Result< S ::Ok, S ::Error >
    where
      S: serde ::Serializer,
    {
      // Serialize as plain string, not as struct
      serializer.serialize_str( &self.0 )
    }
  }

  impl< 'de > serde ::Deserialize< 'de > for CustomValue
  {
    fn deserialize< D >( deserializer: D ) -> Result< Self, D ::Error >
    where
      D: serde ::Deserializer< 'de >,
    {
      String ::deserialize( deserializer ).map( CustomValue )
    }
  }

  impl TemplateValue for CustomValue
  {
    fn to_template_string( &self ) -> String
    {
      self.0.clone()
    }

    fn from_string( s: String ) -> Self
    {
      CustomValue( s )
    }

    fn is_empty( &self ) -> bool
    {
      self.0.is_empty()
    }
  }

  let renderer = HandlebarsRenderer ::new();
  let mut filesystem = MemoryFileSystem ::new();

  filesystem.write( &PathBuf ::from( "t.hbs" ), "Value: {{val}}" ).unwrap();

  let mut template: Template< CustomValue, _, _ > = Template ::new( renderer, filesystem );
  template.insert_value( "val", CustomValue( "test_data".into() ) );

  let output_path = PathBuf ::from( "output.txt" );
  template.add_file( FileDescriptor
  {
    file_path: output_path.clone(),
    template_path: PathBuf ::from( "t.hbs" ),
    write_mode: WriteMode ::Rewrite,
  });

  template.materialize().unwrap();

  let content = template.filesystem().read( &output_path ).unwrap();
  // Custom value type works through the full pipeline
  assert_eq!( content, "Value: test_data" );
}

#[ test ]
fn real_filesystem_integration()
{
  // Test with actual filesystem (still safe - uses unique temp directory)
  use std ::env;

  let renderer = HandlebarsRenderer ::new();
  let filesystem = RealFileSystem ::new();

  // Use temp directory
  let temp_dir = env ::temp_dir().join( format!( "genfile_test_{}", std ::process ::id() ) );

  // Create template in temp
  let template_path = temp_dir.join( "template.hbs" );
  let output_path = temp_dir.join( "output.txt" );

  std ::fs ::create_dir_all( &temp_dir ).unwrap();
  std ::fs ::write( &template_path, "Hello {{name}}!" ).unwrap();

  let mut template: Template< Value, _, _ > = Template ::new( renderer, filesystem );
  template.insert_value( "name", Value ::String( "World".into() ) );

  template.add_file( FileDescriptor
  {
    file_path: output_path.clone(),
    template_path: template_path.clone(),
    write_mode: WriteMode ::Rewrite,
  });

  template.materialize().unwrap();

  // Verify file was created
  assert!( output_path.exists() );
  let content = std ::fs ::read_to_string( &output_path ).unwrap();
  assert_eq!( content, "Hello World!" );

  // Cleanup
  std ::fs ::remove_dir_all( &temp_dir ).unwrap();
}

#[ test ]
fn values_with_conditional_rendering()
{
  // Integration: Values + Handlebars conditional logic
  let renderer = HandlebarsRenderer ::new();
  let mut filesystem = MemoryFileSystem ::new();

  let template_content = "
{{#if debug}}
Debug mode enabled
{{else}}
Production mode
{{/if}}
Count: {{count}}
";

  filesystem.write( &PathBuf ::from( "t.hbs" ), template_content ).unwrap();

  let mut template: Template< Value, _, _ > = Template ::new( renderer, filesystem );
  template.insert_value( "debug", Value ::Bool( true ) );
  template.insert_value( "count", Value ::Number( 5 ) );

  let output_path = PathBuf ::from( "output.txt" );
  template.add_file( FileDescriptor
  {
    file_path: output_path.clone(),
    template_path: PathBuf ::from( "t.hbs" ),
    write_mode: WriteMode ::Rewrite,
  });

  template.materialize().unwrap();

  let content = template.filesystem().read( &output_path ).unwrap();
  assert!( content.contains( "Debug mode enabled" ) );
  assert!( !content.contains( "Production mode" ) );
  assert!( content.contains( "Count: 5" ) );
}

#[ test ]
fn error_propagation_through_layers()
{
  // Verify errors propagate correctly through all layers
  let renderer = HandlebarsRenderer ::new();
  let filesystem = MemoryFileSystem ::new();

  let mut template: Template< Value, _, _ > = Template ::new( renderer, filesystem );

  // Add file with non-existent template
  template.add_file( FileDescriptor
  {
    file_path: PathBuf ::from( "output.txt" ),
    template_path: PathBuf ::from( "missing.hbs" ),
    write_mode: WriteMode ::Rewrite,
  });

  let result = template.materialize();
  assert!( result.is_err() );

  // Error should be Fs error from filesystem layer
  match result.unwrap_err()
  {
    Error ::Fs( _ ) => {}, // Expected
    other => panic!( "Expected Fs error, got {other:?}" ),
  }
}

#[ test ]
fn multiple_value_insertions()
{
  // Integration: verify multiple value operations work together
  let renderer = HandlebarsRenderer ::new();
  let mut filesystem = MemoryFileSystem ::new();

  filesystem.write( &PathBuf ::from( "t.hbs" ), "{{a}} {{b}} {{c}}" ).unwrap();

  let mut template: Template< Value, _, _ > = Template ::new( renderer, filesystem );

  // Insert values one by one
  template.insert_value( "a", Value ::String( "first".into() ) );
  template.insert_value( "b", Value ::String( "second".into() ) );
  template.insert_value( "c", Value ::String( "third".into() ) );

  // Verify all are stored
  assert!( template.has_value( "a" ) );
  assert!( template.has_value( "b" ) );
  assert!( template.has_value( "c" ) );

  let output_path = PathBuf ::from( "output.txt" );
  template.add_file( FileDescriptor
  {
    file_path: output_path.clone(),
    template_path: PathBuf ::from( "t.hbs" ),
    write_mode: WriteMode ::Rewrite,
  });

  template.materialize().unwrap();

  let content = template.filesystem().read( &output_path ).unwrap();
  assert_eq!( content, "first second third" );
}
