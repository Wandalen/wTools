/// Tests for builder patterns (FR21)
use super :: *;
use std ::path ::PathBuf;

//

#[ test ]
fn parameter_descriptor_builder_basic()
{
  // FR21: ParameterDescriptor::former() must work with fluent API
  let param = ParameterDescriptor ::former()
    .parameter( "project_name" )
    .is_mandatory( true )
    .form();

  assert_eq!( param.parameter, "project_name" );
  assert!( param.is_mandatory );
  assert!( param.default_value.is_none() );
  assert!( param.description.is_none() );
}

#[ test ]
fn parameter_descriptor_builder_with_default()
{
  // FR21: Builder should support optional default value
  let param = ParameterDescriptor ::former()
    .parameter( "region" )
    .is_mandatory( false )
    .default_value( "us-east-1" )
    .form();

  assert_eq!( param.parameter, "region" );
  assert!( !param.is_mandatory );
  assert_eq!( param.default_value, Some( "us-east-1".to_string() ) );
  assert!( param.description.is_none() );
}

#[ test ]
fn parameter_descriptor_builder_with_description()
{
  // FR21: Builder should support optional description
  let param = ParameterDescriptor ::former()
    .parameter( "api_key" )
    .is_mandatory( true )
    .description( "API key for authentication" )
    .form();

  assert_eq!( param.parameter, "api_key" );
  assert!( param.is_mandatory );
  assert_eq!( param.description, Some( "API key for authentication".to_string() ) );
}

#[ test ]
fn parameter_descriptor_builder_all_fields()
{
  // FR21: Builder should support all fields in fluent API
  let param = ParameterDescriptor ::former()
    .parameter( "database_url" )
    .is_mandatory( false )
    .default_value( "localhost:5432" )
    .description( "Database connection URL" )
    .form();

  assert_eq!( param.parameter, "database_url" );
  assert!( !param.is_mandatory );
  assert_eq!( param.default_value, Some( "localhost:5432".to_string() ) );
  assert_eq!( param.description, Some( "Database connection URL".to_string() ) );
}

#[ test ]
fn parameter_descriptor_builder_default_is_mandatory()
{
  // FR21: is_mandatory should default to false
  let param = ParameterDescriptor ::former()
    .parameter( "optional_param" )
    .form();

  assert_eq!( param.parameter, "optional_param" );
  assert!( !param.is_mandatory );
}

#[ test ]
fn parameters_builder_basic()
{
  // FR21: Parameters::former() must work with fluent API
  let params = Parameters ::former()
    .descriptors( vec!
    [
      ParameterDescriptor ::former()
        .parameter( "name" )
        .is_mandatory( true )
        .form(),
    ])
    .form();

  assert_eq!( params.descriptors.len(), 1 );
  assert_eq!( params.descriptors[ 0 ].parameter, "name" );
}

#[ test ]
fn parameters_builder_multiple_descriptors()
{
  // FR21: Builder should handle multiple parameter descriptors
  let params = Parameters ::former()
    .descriptors( vec!
    [
      ParameterDescriptor ::former()
        .parameter( "name" )
        .is_mandatory( true )
        .form(),
      ParameterDescriptor ::former()
        .parameter( "version" )
        .is_mandatory( true )
        .form(),
      ParameterDescriptor ::former()
        .parameter( "description" )
        .is_mandatory( false )
        .default_value( "No description" )
        .form(),
    ])
    .form();

  assert_eq!( params.descriptors.len(), 3 );

  let mandatory = params.list_mandatory();
  assert_eq!( mandatory.len(), 2 );
  assert!( mandatory.contains( &"name" ) );
  assert!( mandatory.contains( &"version" ) );
}

#[ test ]
fn file_descriptor_builder_basic()
{
  // FR21: FileDescriptor::former() must work with fluent API
  let descriptor = FileDescriptor ::former()
    .file_path( PathBuf ::from( "output.txt" ) )
    .template_path( PathBuf ::from( "template.hbs" ) )
    .write_mode( WriteMode ::Rewrite )
    .form();

  assert_eq!( descriptor.file_path, PathBuf ::from( "output.txt" ) );
  assert_eq!( descriptor.template_path, PathBuf ::from( "template.hbs" ) );
  assert_eq!( descriptor.write_mode, WriteMode ::Rewrite );
}

#[ test ]
fn file_descriptor_builder_toml_extend()
{
  // FR21: Builder should support TomlExtend write mode
  let descriptor = FileDescriptor ::former()
    .file_path( PathBuf ::from( "Cargo.toml" ) )
    .template_path( PathBuf ::from( "cargo_template.toml" ) )
    .write_mode( WriteMode ::TomlExtend )
    .form();

  assert_eq!( descriptor.file_path, PathBuf ::from( "Cargo.toml" ) );
  assert_eq!( descriptor.template_path, PathBuf ::from( "cargo_template.toml" ) );
  assert_eq!( descriptor.write_mode, WriteMode ::TomlExtend );
}

#[ test ]
fn file_descriptor_builder_complex_paths()
{
  // FR21: Builder should handle complex paths
  let descriptor = FileDescriptor ::former()
    .file_path( PathBuf ::from( "src/generated/module.rs" ) )
    .template_path( PathBuf ::from( "templates/rust/module.hbs" ) )
    .write_mode( WriteMode ::Rewrite )
    .form();

  assert_eq!( descriptor.file_path, PathBuf ::from( "src/generated/module.rs" ) );
  assert_eq!( descriptor.template_path, PathBuf ::from( "templates/rust/module.hbs" ) );
}

#[ test ]
fn builder_pattern_integration()
{
  // FR21: Builders should work together in realistic scenarios
  let params = Parameters ::former()
    .descriptors( vec!
    [
      ParameterDescriptor ::former()
        .parameter( "project_name" )
        .is_mandatory( true )
        .description( "Name of the project" )
        .form(),
      ParameterDescriptor ::former()
        .parameter( "author" )
        .is_mandatory( false )
        .default_value( "Unknown" )
        .description( "Project author" )
        .form(),
    ])
    .form();

  let file_desc = FileDescriptor ::former()
    .file_path( PathBuf ::from( "readme.md" ) )
    .template_path( PathBuf ::from( "templates/readme.hbs" ) )
    .write_mode( WriteMode ::Rewrite )
    .form();

  // Verify all built correctly
  assert_eq!( params.descriptors.len(), 2 );
  assert_eq!( params.list_mandatory().len(), 1 );
  assert_eq!( file_desc.file_path, PathBuf ::from( "readme.md" ) );
}

#[ test ]
fn builder_fluent_api_chaining()
{
  // FR21: All builders must provide fluent API (method chaining)
  let _param = ParameterDescriptor ::former()
    .parameter( "test" )
    .is_mandatory( true )
    .default_value( "default" )
    .description( "desc" )
    .form();

  let _params = Parameters ::former()
    .descriptors( vec![] )
    .form();

  let _file = FileDescriptor ::former()
    .file_path( PathBuf ::from( "test" ) )
    .template_path( PathBuf ::from( "template" ) )
    .write_mode( WriteMode ::Rewrite )
    .form();

  // If this compiles, fluent API works
}
