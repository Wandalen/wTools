/// Tests for Template holder (docs/feature/013, docs/feature/014, docs/feature/016, docs/feature/017)
use super :: *;
use std ::path ::PathBuf;

//

#[ test ]
fn template_can_be_created()
{
  // docs/feature/013, docs/feature/014: Template must hold renderer and filesystem
  let renderer = HandlebarsRenderer ::new();
  let filesystem = MemoryFileSystem ::new();

  let template: Template< Value, _, _ > = Template ::new( renderer, filesystem );

  // Just verify it compiles and creates
  drop( template );
}

#[ test ]
fn template_holds_values()
{
  // docs/feature/014: Template must hold values
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
  // docs/feature/016: Template::materialize() must render template and write to file
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
  // docs/feature/016: Should handle multiple file descriptors
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
  // docs/feature/016: Rewrite mode should completely replace file content
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

/// Test validates Template API allows reading results via `filesystem()` accessor
///
/// ## Root Cause
///
/// Example `low_level_template.rs` (lines 80-89) incorrectly claimed that "Template API owns the
/// filesystem, so you cannot easily read back the results when using `MemoryFileSystem`", directing
/// users to use `TemplateArchive` instead. This was factually incorrect - the Template struct
/// provides a public `filesystem()` method (src/template.rs:228) that returns `&FS`, allowing
/// read access to the filesystem after materialization.
///
/// ## Why Not Caught
///
/// The example documentation diverged from test coverage. Tests (`template_materialize_simple`,
/// `template_materialize_multiple_files`, `template_rewrite_mode_replaces_content`) all correctly
/// demonstrated reading results via `template.filesystem()`, but the example wasn't validated
/// against this test pattern. No automated check existed to verify example correctness against
/// actual API capabilities.
///
/// ## Fix Applied
///
/// Updated `examples/low_level_template.rs` (lines 79-91) to use `template.filesystem().read()` to
/// read and display generated files, replacing the incorrect "cannot read" message with actual
/// result verification. This aligns the example with existing test patterns.
///
/// ## Prevention
///
/// 1. Example code should reference existing tests for API usage patterns
/// 2. Add CI check to validate examples demonstrate features covered by tests
/// 3. Review example documentation claims against API surface area during PR review
/// 4. When examples claim API limitations, verify no accessor methods exist
///
/// ## Pitfall
///
/// When APIs use ownership patterns (Template owns filesystem), it's easy to assume no access
/// is possible after transfer. Always check for accessor methods (`&self` getters) before
/// documenting limitations. The pattern `pub fn field_name(&self) -> &FieldType` is common
/// for read-only access to owned fields.
#[ test ]
fn template_filesystem_accessor_allows_result_verification()
{
  // Validates that Template::filesystem() provides read access after materialization
  // This is the pattern that low_level_template.rs example should use

  let renderer = HandlebarsRenderer ::new();
  let mut filesystem = MemoryFileSystem ::new();

  // Set up templates (same pattern as low_level_template.rs example)
  filesystem.write( &PathBuf ::from( "/templates/greeting.hbs" ), "Hello, {{name}}!\n" ).unwrap();
  filesystem.write( &PathBuf ::from( "/templates/config.hbs" ), "server={{server}}\nport={{port}}\n" ).unwrap();

  let mut template = Template ::new( renderer, filesystem );

  // Insert values
  template.insert_value( "name", Value ::String( "World".into() ) );
  template.insert_value( "server", Value ::String( "localhost".into() ) );
  template.insert_value( "port", Value ::Number( 8080 ) );

  // Add file descriptors
  template.add_file( FileDescriptor
  {
    template_path: PathBuf ::from( "/templates/greeting.hbs" ),
    file_path: PathBuf ::from( "/output/greeting.txt" ),
    write_mode: WriteMode ::Rewrite,
  });

  template.add_file( FileDescriptor
  {
    template_path: PathBuf ::from( "/templates/config.hbs" ),
    file_path: PathBuf ::from( "/output/config.txt" ),
    write_mode: WriteMode ::Rewrite,
  });

  // Materialize
  template.materialize().unwrap();

  // ✅ CAN read results via filesystem() accessor (contrary to old example claim)
  let greeting = template.filesystem().read( &PathBuf ::from( "/output/greeting.txt" ) ).unwrap();
  assert_eq!( greeting, "Hello, World!\n" );

  let config = template.filesystem().read( &PathBuf ::from( "/output/config.txt" ) ).unwrap();
  assert_eq!( config, "server=localhost\nport=8080\n" );
}

/// Test validates `Value::List` works in template rendering
///
/// ## Root Cause
///
/// None of the 7 examples demonstrate `Value::List` usage, despite it being a documented feature
/// (src/value.rs:89). The serialization.rs example demonstrates String, Number, and Bool variants
/// (lines 82-86) but omits List. This creates a documentation gap where users cannot see how to
/// use comma-separated value rendering in templates (e.g., "Items: {{tags}}" → "Items: rust, cli, tools").
///
/// ## Why Not Caught
///
/// Examples were written focusing on common use cases (string interpolation, numbers, booleans).
/// List variant was added to support comma-separated rendering but no example was created to
/// demonstrate it. Test coverage exists (value_test.rs:67-77) validating List renders correctly,
/// but no integration test or example showed List in actual template materialization context.
///
/// ## Fix Applied
///
/// This test demonstrates `Value::List` works correctly in template materialization. Updated
/// serialization.rs example (added tags parameter showing List usage) to provide user-facing
/// documentation of this feature.
///
/// ## Prevention
///
/// 1. When adding new Value variants, require corresponding example demonstration
/// 2. Cross-check examples against Value enum variants during PR review
/// 3. Add CI check validating all public enum variants are demonstrated in examples
/// 4. Maintain checklist in examples/readme.md of features that must be shown
///
/// ## Pitfall
///
/// `Value::List` is for simple comma-separated rendering, NOT for template loops. Users familiar
/// with Handlebars {{#each}} loops might expect List to enable iteration, but it only produces
/// a comma-separated string. For iteration, users should pass structured data and use Handlebars
/// loop syntax. List is specifically for cases like "tags: foo, bar, baz" where comma separation
/// is the desired output format.
#[ test ]
fn template_value_list_renders_comma_separated()
{
  // Validates that Value::List renders as comma-separated string in templates
  // This is the pattern that serialization.rs example should demonstrate

  let renderer = HandlebarsRenderer ::new();
  let mut filesystem = MemoryFileSystem ::new();

  // Template using List value for comma-separated rendering
  filesystem.write( &PathBuf ::from( "template.hbs" ), "Tags: {{tags}}\nAuthors: {{authors}}\n" ).unwrap();

  let mut template = Template ::new( renderer, filesystem );

  // Insert List values
  template.insert_value( "tags", Value ::List( vec![ "rust".into(), "cli".into(), "tools".into() ] ) );
  template.insert_value( "authors", Value ::List( vec![ "Alice".into(), "Bob".into() ] ) );

  template.add_file( FileDescriptor
  {
    template_path: PathBuf ::from( "template.hbs" ),
    file_path: PathBuf ::from( "output.txt" ),
    write_mode: WriteMode ::Rewrite,
  });

  template.materialize().unwrap();

  // Verify List renders as comma-separated
  let content = template.filesystem().read( &PathBuf ::from( "output.txt" ) ).unwrap();
  assert_eq!( content, "Tags: rust, cli, tools\nAuthors: Alice, Bob\n" );
}

