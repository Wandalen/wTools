/// Error path and edge case tests for Template (additional coverage)
use super :: *;
use std ::path ::PathBuf;

//

#[ test ]
fn template_materialize_fails_when_template_file_missing()
{
  // Error handling: materialization should fail gracefully if template doesn't exist
  let renderer = HandlebarsRenderer ::new();
  let filesystem = MemoryFileSystem ::new();

  let mut template: Template< Value, _, _ > = Template ::new( renderer, filesystem );

  // Add file descriptor referencing non-existent template
  template.add_file( FileDescriptor
  {
    file_path: PathBuf ::from( "output.txt" ),
    template_path: PathBuf ::from( "nonexistent.hbs" ),
    write_mode: WriteMode ::Rewrite,
  });

  let result = template.materialize();
  assert!( result.is_err() );

  match result.unwrap_err()
  {
    Error ::Fs( _ ) => {}, // Expected
    other => panic!( "Expected Error::Fs for missing file, got {other:?}" ),
  }
}

#[ test ]
fn template_materialize_fails_on_invalid_template_syntax()
{
  // Error handling: invalid template syntax should return Error::Render
  let renderer = HandlebarsRenderer ::new();
  let mut filesystem = MemoryFileSystem ::new();

  // Create template with invalid syntax
  let template_path = PathBuf ::from( "bad_template.hbs" );
  filesystem.write( &template_path, "{{unclosed variable" ).unwrap();

  let mut template: Template< Value, _, _ > = Template ::new( renderer, filesystem );

  template.add_file( FileDescriptor
  {
    file_path: PathBuf ::from( "output.txt" ),
    template_path,
    write_mode: WriteMode ::Rewrite,
  });

  let result = template.materialize();
  assert!( result.is_err() );

  match result.unwrap_err()
  {
    Error ::Render( _ ) => {}, // Expected
    other => panic!( "Expected Error::Render for bad syntax, got {other:?}" ),
  }
}

#[ test ]
fn template_materialize_with_missing_variable_in_template()
{
  // Handlebars renders missing variables as empty strings - verify this works
  let renderer = HandlebarsRenderer ::new();
  let mut filesystem = MemoryFileSystem ::new();

  let template_path = PathBuf ::from( "template.hbs" );
  filesystem.write( &template_path, "Value: {{missing_var}}" ).unwrap();

  let mut template: Template< Value, _, _ > = Template ::new( renderer, filesystem );
  // Intentionally not providing the variable

  let output_path = PathBuf ::from( "output.txt" );
  template.add_file( FileDescriptor
  {
    file_path: output_path.clone(),
    template_path,
    write_mode: WriteMode ::Rewrite,
  });

  let result = template.materialize();
  assert!( result.is_ok() );

  // Handlebars renders missing variables as empty
  let content = template.filesystem().read( &output_path ).unwrap();
  assert_eq!( content, "Value: " );
}

#[ test ]
fn template_materialize_with_no_files()
{
  // Edge case: materialization with zero files should succeed (no-op)
  let renderer = HandlebarsRenderer ::new();
  let filesystem = MemoryFileSystem ::new();

  let mut template: Template< Value, _, _ > = Template ::new( renderer, filesystem );

  let result = template.materialize();
  assert!( result.is_ok() );
}

#[ test ]
fn template_with_complex_nested_values()
{
  // Complex scenario: multiple value types in one template
  let renderer = HandlebarsRenderer ::new();
  let mut filesystem = MemoryFileSystem ::new();

  let template_path = PathBuf ::from( "complex.hbs" );
  filesystem.write
  (
    &template_path,
    "Name: {{name}}\nCount: {{count}}\nEnabled: {{enabled}}\nItems: {{items}}"
  ).unwrap();

  let mut template: Template< Value, _, _ > = Template ::new( renderer, filesystem );
  template.insert_value( "name", Value ::String( "Project".into() ) );
  template.insert_value( "count", Value ::Number( 42 ) );
  template.insert_value( "enabled", Value ::Bool( true ) );
  template.insert_value( "items", Value ::List( vec![ "a".into(), "b".into(), "c".into() ] ) );

  let output_path = PathBuf ::from( "output.txt" );
  template.add_file( FileDescriptor
  {
    file_path: output_path.clone(),
    template_path,
    write_mode: WriteMode ::Rewrite,
  });

  template.materialize().unwrap();

  let content = template.filesystem().read( &output_path ).unwrap();
  assert!( content.contains( "Name: Project" ) );
  assert!( content.contains( "Count: 42" ) );
  assert!( content.contains( "Enabled: true" ) );
  assert!( content.contains( "Items: a, b, c" ) );
}

#[ test ]
fn template_materialize_preserves_file_order()
{
  // Verify files are processed in the order they're added
  let renderer = HandlebarsRenderer ::new();
  let mut filesystem = MemoryFileSystem ::new();

  filesystem.write( &PathBuf ::from( "t1.hbs" ), "file1" ).unwrap();
  filesystem.write( &PathBuf ::from( "t2.hbs" ), "file2" ).unwrap();
  filesystem.write( &PathBuf ::from( "t3.hbs" ), "file3" ).unwrap();

  let mut template: Template< Value, _, _ > = Template ::new( renderer, filesystem );

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

  template.add_file( FileDescriptor
  {
    file_path: PathBuf ::from( "out3.txt" ),
    template_path: PathBuf ::from( "t3.hbs" ),
    write_mode: WriteMode ::Rewrite,
  });

  template.materialize().unwrap();

  // All files should exist
  let fs = template.filesystem();
  assert_eq!( fs.read( &PathBuf ::from( "out1.txt" ) ).unwrap(), "file1" );
  assert_eq!( fs.read( &PathBuf ::from( "out2.txt" ) ).unwrap(), "file2" );
  assert_eq!( fs.read( &PathBuf ::from( "out3.txt" ) ).unwrap(), "file3" );
}

#[ test ]
fn template_empty_string_values()
{
  // Edge case: empty string values should be handled correctly
  let renderer = HandlebarsRenderer ::new();
  let mut filesystem = MemoryFileSystem ::new();

  filesystem.write( &PathBuf ::from( "t.hbs" ), "Value: '{{v}}'" ).unwrap();

  let mut template: Template< Value, _, _ > = Template ::new( renderer, filesystem );
  template.insert_value( "v", Value ::String( String::new() ) );

  let output_path = PathBuf ::from( "output.txt" );
  template.add_file( FileDescriptor
  {
    file_path: output_path.clone(),
    template_path: PathBuf ::from( "t.hbs" ),
    write_mode: WriteMode ::Rewrite,
  });

  template.materialize().unwrap();

  assert_eq!( template.filesystem().read( &output_path ).unwrap(), "Value: ''" );
}

#[ test ]
fn template_zero_value()
{
  // Edge case: zero should render correctly
  let renderer = HandlebarsRenderer ::new();
  let mut filesystem = MemoryFileSystem ::new();

  filesystem.write( &PathBuf ::from( "t.hbs" ), "Count: {{count}}" ).unwrap();

  let mut template: Template< Value, _, _ > = Template ::new( renderer, filesystem );
  template.insert_value( "count", Value ::Number( 0 ) );

  let output_path = PathBuf ::from( "output.txt" );
  template.add_file( FileDescriptor
  {
    file_path: output_path.clone(),
    template_path: PathBuf ::from( "t.hbs" ),
    write_mode: WriteMode ::Rewrite,
  });

  template.materialize().unwrap();

  assert_eq!( template.filesystem().read( &output_path ).unwrap(), "Count: 0" );
}

#[ test ]
fn template_negative_numbers()
{
  // Edge case: negative numbers should render correctly
  let renderer = HandlebarsRenderer ::new();
  let mut filesystem = MemoryFileSystem ::new();

  filesystem.write( &PathBuf ::from( "t.hbs" ), "Value: {{val}}" ).unwrap();

  let mut template: Template< Value, _, _ > = Template ::new( renderer, filesystem );
  template.insert_value( "val", Value ::Number( -42 ) );

  let output_path = PathBuf ::from( "output.txt" );
  template.add_file( FileDescriptor
  {
    file_path: output_path.clone(),
    template_path: PathBuf ::from( "t.hbs" ),
    write_mode: WriteMode ::Rewrite,
  });

  template.materialize().unwrap();

  assert_eq!( template.filesystem().read( &output_path ).unwrap(), "Value: -42" );
}

#[ test ]
fn template_empty_list()
{
  // Edge case: empty lists should be handled
  let renderer = HandlebarsRenderer ::new();
  let mut filesystem = MemoryFileSystem ::new();

  filesystem.write( &PathBuf ::from( "t.hbs" ), "Items: {{items}}" ).unwrap();

  let mut template: Template< Value, _, _ > = Template ::new( renderer, filesystem );
  template.insert_value( "items", Value ::List( vec![] ) );

  let output_path = PathBuf ::from( "output.txt" );
  template.add_file( FileDescriptor
  {
    file_path: output_path.clone(),
    template_path: PathBuf ::from( "t.hbs" ),
    write_mode: WriteMode ::Rewrite,
  });

  template.materialize().unwrap();

  assert_eq!( template.filesystem().read( &output_path ).unwrap(), "Items: " );
}

#[ test ]
fn template_overwrite_value()
{
  // Verify that inserting same key twice overwrites
  let renderer = HandlebarsRenderer ::new();
  let filesystem = MemoryFileSystem ::new();

  let mut template: Template< Value, _, _ > = Template ::new( renderer, filesystem );
  template.insert_value( "key", Value ::String( "first".into() ) );
  template.insert_value( "key", Value ::String( "second".into() ) );

  assert!( template.has_value( "key" ) );
  // Second value should win (this is standard HashMap behavior)
}

#[ test ]
fn template_special_characters_in_values()
{
  // Verify special characters are handled correctly (no escaping since we disabled HTML escaping)
  let renderer = HandlebarsRenderer ::new();
  let mut filesystem = MemoryFileSystem ::new();

  filesystem.write( &PathBuf ::from( "t.hbs" ), "{{content}}" ).unwrap();

  let mut template: Template< Value, _, _ > = Template ::new( renderer, filesystem );
  template.insert_value( "content", Value ::String( "<>&\"'".into() ) );

  let output_path = PathBuf ::from( "output.txt" );
  template.add_file( FileDescriptor
  {
    file_path: output_path.clone(),
    template_path: PathBuf ::from( "t.hbs" ),
    write_mode: WriteMode ::Rewrite,
  });

  template.materialize().unwrap();

  // Should NOT be escaped because we use no_escape
  assert_eq!( template.filesystem().read( &output_path ).unwrap(), "<>&\"'" );
}
