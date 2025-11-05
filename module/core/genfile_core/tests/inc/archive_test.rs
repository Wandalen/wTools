//! Tests for `TemplateArchive` functionality
//!
//! # Test Coverage
//!
//! - Binary file serialization with all byte values (0x00-0xFF)
//! - JSON/YAML serialization roundtrip for binary content
//! - Parameter discovery from templates
//! - Deep directory nesting
//! - File CRUD operations
//! - Parameter analysis and validation
//! - Materialization to filesystem
//!
//! # Special Focus: Binary File Handling
//!
//! Per requirements, we test that JSON/YAML formats properly represent
//! binary files and all non-textual symbols. This includes:
//! - All 256 possible byte values (0x00-0xFF)
//! - Special control characters (\0, \n, \r, \t, etc.)
//! - High-bit characters (0x80-0xFF)
//! - Base64 encoding/decoding accuracy

use genfile_core::
{
  TemplateArchive,
  FileContent,
  WriteMode,
  ParameterDescriptor,
  Value,
  ArchiveMetadata,
  MemoryFileSystem,
  HandlebarsRenderer,
  FileSystem,
};
use std::path::{ Path, PathBuf };
use std::collections::HashMap;

//

#[ test ]
fn archive_creation()
{
  let archive = TemplateArchive::new( "test-archive" );

  assert_eq!( archive.name, "test-archive" );
  assert_eq!( archive.version, "0.1.0" );
  assert!( archive.description.is_none() );
  assert_eq!( archive.file_count(), 0 );
}

#[ test ]
fn archive_metadata()
{
  let mut archive = TemplateArchive::new( "test" );
  archive.set_version( "2.0.0" );
  archive.set_description( "Test template" );

  assert_eq!( archive.version, "2.0.0" );
  assert_eq!( archive.description, Some( "Test template".to_string() ) );

  let metadata = ArchiveMetadata
  {
    author: Some( "Test Author".into() ),
    license: Some( "MIT".into() ),
    tags: vec![ "rust".into(), "template".into() ],
    created_at: Some( "2024-01-01".into() ),
    modified_at: None,
  };

  archive.set_metadata( metadata.clone() );
  assert!( archive.metadata.is_some() );
  assert_eq!( archive.metadata.unwrap().author, Some( "Test Author".to_string() ) );
}

//

#[ test ]
fn add_text_file()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file(
    PathBuf::from( "README.md" ),
    "# {{project_name}}",
    WriteMode::Rewrite
  );

  assert_eq!( archive.file_count(), 1 );
  assert!( archive.has_file( Path::new( "README.md" ) ) );

  let file = archive.get_file( Path::new( "README.md" ) ).unwrap();
  assert_eq!( file.path, PathBuf::from( "README.md" ) );
  assert!( matches!( file.content, FileContent::Text( _ ) ) );
}

#[ test ]
fn add_binary_file()
{
  let mut archive = TemplateArchive::new( "test" );

  let binary_data = vec![ 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A ];
  archive.add_binary_file( PathBuf::from( "logo.png" ), binary_data.clone() );

  assert_eq!( archive.file_count(), 1 );
  assert_eq!( archive.binary_file_count(), 1 );

  let file = archive.get_file( Path::new( "logo.png" ) ).unwrap();
  if let FileContent::Binary( bytes ) = &file.content
  {
    assert_eq!( bytes, &binary_data );
  }
  else
  {
    panic!( "Expected binary content" );
  }
}

#[ test ]
fn remove_file()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file( PathBuf::from( "test.txt" ), "content", WriteMode::Rewrite );
  assert_eq!( archive.file_count(), 1 );

  let removed = archive.remove_file( Path::new( "test.txt" ) );
  assert!( removed.is_some() );
  assert_eq!( archive.file_count(), 0 );

  let not_found = archive.remove_file( Path::new( "missing.txt" ) );
  assert!( not_found.is_none() );
}

#[ test ]
fn file_mutation()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file( PathBuf::from( "config.toml" ), "old = 1", WriteMode::Rewrite );

  let file = archive.get_file_mut( Path::new( "config.toml" ) ).unwrap();
  file.content = FileContent::Text( "new = 2".into() );

  let file = archive.get_file( Path::new( "config.toml" ) ).unwrap();
  if let FileContent::Text( content ) = &file.content
  {
    assert_eq!( content, "new = 2" );
  }
  else
  {
    panic!( "Expected text content" );
  }
}

#[ test ]
fn list_files()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file( PathBuf::from( "a.txt" ), "a", WriteMode::Rewrite );
  archive.add_text_file( PathBuf::from( "b.txt" ), "b", WriteMode::Rewrite );
  archive.add_binary_file( PathBuf::from( "c.bin" ), vec![ 1, 2, 3 ] );

  let files = archive.list_files();
  assert_eq!( files.len(), 3 );
  assert!( files.contains( &Path::new( "a.txt" ) ) );
  assert!( files.contains( &Path::new( "b.txt" ) ) );
  assert!( files.contains( &Path::new( "c.bin" ) ) );
}

//

#[ test ]
fn deep_directory_structure()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file( PathBuf::from( "root.txt" ), "x", WriteMode::Rewrite );
  archive.add_text_file( PathBuf::from( "src/lib.rs" ), "x", WriteMode::Rewrite );
  archive.add_text_file( PathBuf::from( "src/utils/helper.rs" ), "x", WriteMode::Rewrite );
  archive.add_text_file( PathBuf::from( "tests/integration/deep/nested/test.rs" ), "x", WriteMode::Rewrite );

  let dirs = archive.list_directories();
  assert!( dirs.contains( &PathBuf::from( "src" ) ) );
  assert!( dirs.contains( &PathBuf::from( "src/utils" ) ) );
  assert!( dirs.contains( &PathBuf::from( "tests" ) ) );
  assert!( dirs.contains( &PathBuf::from( "tests/integration" ) ) );
  assert!( dirs.contains( &PathBuf::from( "tests/integration/deep" ) ) );
  assert!( dirs.contains( &PathBuf::from( "tests/integration/deep/nested" ) ) );

  assert_eq!( archive.max_directory_depth(), 4 );
}

//

#[ test ]
fn add_parameter()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_parameter( ParameterDescriptor
  {
    parameter: "project_name".into(),
    is_mandatory: true,
    default_value: None,
    description: Some( "Project name".into() ),
  });

  let params = archive.list_parameters();
  assert_eq!( params.len(), 1 );
  assert!( params.contains( &"project_name" ) );

  let param = archive.get_parameter( "project_name" ).unwrap();
  assert!( param.is_mandatory );
  assert_eq!( param.description, Some( "Project name".to_string() ) );
}

#[ test ]
fn remove_parameter()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_parameter( ParameterDescriptor
  {
    parameter: "name".into(),
    is_mandatory: false,
    default_value: None,
    description: None,
  });

  let removed = archive.remove_parameter( "name" );
  assert!( removed.is_some() );
  assert_eq!( archive.list_parameters().len(), 0 );

  let not_found = archive.remove_parameter( "missing" );
  assert!( not_found.is_none() );
}

#[ test ]
fn mandatory_parameters()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_parameter( ParameterDescriptor
  {
    parameter: "required".into(),
    is_mandatory: true,
    default_value: None,
    description: None,
  });

  archive.add_parameter( ParameterDescriptor
  {
    parameter: "optional".into(),
    is_mandatory: false,
    default_value: Some( "default".into() ),
    description: None,
  });

  let mandatory = archive.list_mandatory_parameters();
  assert_eq!( mandatory.len(), 1 );
  assert!( mandatory.contains( &"required" ) );
}

//

#[ test ]
fn discover_parameters()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file(
    PathBuf::from( "config.txt" ),
    "Server: {{host}}:{{port}}\nUser: {{username}}",
    WriteMode::Rewrite
  );

  archive.add_text_file(
    PathBuf::from( "readme.md" ),
    "# {{project_name}}\nVersion: {{version}}",
    WriteMode::Rewrite
  );

  let discovered = archive.discover_parameters();
  assert_eq!( discovered.len(), 5 );
  assert!( discovered.contains( "host" ) );
  assert!( discovered.contains( "port" ) );
  assert!( discovered.contains( "username" ) );
  assert!( discovered.contains( "project_name" ) );
  assert!( discovered.contains( "version" ) );
}

#[ test ]
fn discover_parameters_ignores_binary()
{
  let mut archive = TemplateArchive::new( "test" );

  // Binary files should not be scanned for parameters
  let binary_with_pattern = b"{{fake_param}}".to_vec();
  archive.add_binary_file( PathBuf::from( "data.bin" ), binary_with_pattern );

  archive.add_text_file(
    PathBuf::from( "template.txt" ),
    "{{real_param}}",
    WriteMode::Rewrite
  );

  let discovered = archive.discover_parameters();
  assert_eq!( discovered.len(), 1 );
  assert!( discovered.contains( "real_param" ) );
  assert!( !discovered.contains( "fake_param" ) );
}

#[ test ]
fn undefined_parameters()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file(
    PathBuf::from( "template.txt" ),
    "{{defined}} {{undefined}}",
    WriteMode::Rewrite
  );

  archive.add_parameter( ParameterDescriptor
  {
    parameter: "defined".into(),
    is_mandatory: true,
    default_value: None,
    description: None,
  });

  let undefined = archive.get_undefined_parameters();
  assert_eq!( undefined.len(), 1 );
  assert!( undefined.contains( &"undefined".to_string() ) );
}

#[ test ]
fn unused_parameters()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file(
    PathBuf::from( "template.txt" ),
    "{{used}}",
    WriteMode::Rewrite
  );

  archive.add_parameter( ParameterDescriptor
  {
    parameter: "used".into(),
    is_mandatory: true,
    default_value: None,
    description: None,
  });

  archive.add_parameter( ParameterDescriptor
  {
    parameter: "unused".into(),
    is_mandatory: false,
    default_value: None,
    description: None,
  });

  let unused = archive.get_unused_parameters();
  assert_eq!( unused.len(), 1 );
  assert!( unused.contains( &"unused".to_string() ) );
}

#[ test ]
fn parameter_usage_analysis()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file(
    PathBuf::from( "file1.txt" ),
    "{{param1}} {{param2}}",
    WriteMode::Rewrite
  );

  archive.add_text_file(
    PathBuf::from( "file2.txt" ),
    "{{param1}} {{param3}}",
    WriteMode::Rewrite
  );

  let usage = archive.analyze_parameter_usage();

  assert_eq!( usage.get( "param1" ).unwrap().len(), 2 );
  assert!( usage.get( "param1" ).unwrap().contains( &PathBuf::from( "file1.txt" ) ) );
  assert!( usage.get( "param1" ).unwrap().contains( &PathBuf::from( "file2.txt" ) ) );

  assert_eq!( usage.get( "param2" ).unwrap().len(), 1 );
  assert!( usage.get( "param2" ).unwrap().contains( &PathBuf::from( "file1.txt" ) ) );

  assert_eq!( usage.get( "param3" ).unwrap().len(), 1 );
  assert!( usage.get( "param3" ).unwrap().contains( &PathBuf::from( "file2.txt" ) ) );
}

//

#[ test ]
fn set_and_get_values()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.set_value( "name", Value::String( "TestProject".into() ) );
  archive.set_value( "version", Value::Number( 1 ) );
  archive.set_value( "enabled", Value::Bool( true ) );

  assert_eq!( archive.get_value( "name" ), Some( &Value::String( "TestProject".into() ) ) );
  assert_eq!( archive.get_value( "version" ), Some( &Value::Number( 1 ) ) );
  assert_eq!( archive.get_value( "enabled" ), Some( &Value::Bool( true ) ) );
  assert_eq!( archive.get_value( "missing" ), None );
}

#[ test ]
fn set_multiple_values()
{
  let mut archive = TemplateArchive::new( "test" );

  let mut values = HashMap::new();
  values.insert( "a".to_string(), Value::Number( 1 ) );
  values.insert( "b".to_string(), Value::Number( 2 ) );
  values.insert( "c".to_string(), Value::Number( 3 ) );

  archive.set_values( values );

  assert_eq!( archive.get_value( "a" ), Some( &Value::Number( 1 ) ) );
  assert_eq!( archive.get_value( "b" ), Some( &Value::Number( 2 ) ) );
  assert_eq!( archive.get_value( "c" ), Some( &Value::Number( 3 ) ) );
}

#[ test ]
fn clear_values()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.set_value( "test", Value::String( "value".into() ) );
  assert!( archive.get_value( "test" ).is_some() );

  archive.clear_values();
  assert!( archive.get_value( "test" ).is_none() );
}

//

#[ test ]
fn statistics()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file( PathBuf::from( "a.txt" ), "hello", WriteMode::Rewrite );
  archive.add_text_file( PathBuf::from( "b.txt" ), "world", WriteMode::Rewrite );
  archive.add_binary_file( PathBuf::from( "c.bin" ), vec![ 1, 2, 3, 4 ] );

  assert_eq!( archive.file_count(), 3 );
  assert_eq!( archive.text_file_count(), 2 );
  assert_eq!( archive.binary_file_count(), 1 );
  assert_eq!( archive.total_size(), 5 + 5 + 4 );
}

//

/// Test JSON serialization roundtrip with all possible byte values
///
/// This is critical: we need to ensure JSON format properly represents
/// binary files with ALL non-textual symbols (0x00-0xFF).
#[ test ]
fn json_serialization_all_bytes()
{
  let mut archive = TemplateArchive::new( "binary-test" );

  // Create binary content with ALL possible byte values
  let all_bytes: Vec< u8 > = ( 0..=255 ).collect();

  archive.add_binary_file( PathBuf::from( "all_bytes.bin" ), all_bytes.clone() );

  // Serialize to JSON
  let json = archive.to_json().expect( "JSON serialization failed" );

  // Verify JSON contains base64-encoded data
  assert!( json.contains( "\"type\":\"Binary\"" ) );

  // Deserialize back
  let restored = TemplateArchive::from_json( &json ).expect( "JSON deserialization failed" );

  // Verify binary content is identical
  let file = restored.get_file( Path::new( "all_bytes.bin" ) ).unwrap();
  if let FileContent::Binary( bytes ) = &file.content
  {
    assert_eq!( bytes.len(), 256 );
    for ( i, &byte ) in bytes.iter().enumerate()
    {
      #[ allow( clippy::cast_possible_truncation ) ]
      let expected = i as u8;
      assert_eq!( byte, expected, "Byte at position {i} doesnt match" );
    }
  }
  else
  {
    panic!( "Expected binary content" );
  }
}

/// Test YAML serialization roundtrip with all possible byte values
#[ test ]
fn yaml_serialization_all_bytes()
{
  let mut archive = TemplateArchive::new( "binary-test" );

  // Create binary content with ALL possible byte values
  let all_bytes: Vec< u8 > = ( 0..=255 ).collect();

  archive.add_binary_file( PathBuf::from( "all_bytes.bin" ), all_bytes.clone() );

  // Serialize to YAML
  let yaml = archive.to_yaml().expect( "YAML serialization failed" );

  // Deserialize back
  let restored = TemplateArchive::from_yaml( &yaml ).expect( "YAML deserialization failed" );

  // Verify binary content is identical
  let file = restored.get_file( Path::new( "all_bytes.bin" ) ).unwrap();
  if let FileContent::Binary( bytes ) = &file.content
  {
    assert_eq!( bytes.len(), 256 );
    for ( i, &byte ) in bytes.iter().enumerate()
    {
      #[ allow( clippy::cast_possible_truncation ) ]
      let expected = i as u8;
      assert_eq!( byte, expected, "Byte at position {i} doesnt match" );
    }
  }
  else
  {
    panic!( "Expected binary content" );
  }
}

/// Test special control characters in binary files
#[ test ]
fn binary_control_characters()
{
  let mut archive = TemplateArchive::new( "test" );

  // All control characters including null, newline, carriage return, tab, etc.
  let control_chars: Vec< u8 > = vec![
    0x00, // NULL
    0x01, // SOH
    0x07, // BEL
    0x08, // BS
    0x09, // TAB
    0x0A, // LF (newline)
    0x0B, // VT
    0x0C, // FF
    0x0D, // CR (carriage return)
    0x1B, // ESC
    0x7F, // DEL
  ];

  archive.add_binary_file( PathBuf::from( "controls.bin" ), control_chars.clone() );

  let json = archive.to_json().unwrap();
  let restored = TemplateArchive::from_json( &json ).unwrap();

  let file = restored.get_file( Path::new( "controls.bin" ) ).unwrap();
  if let FileContent::Binary( bytes ) = &file.content
  {
    assert_eq!( bytes, &control_chars );
  }
  else
  {
    panic!( "Expected binary content" );
  }
}

/// Test high-bit characters (0x80-0xFF)
#[ test ]
fn binary_high_bit_characters()
{
  let mut archive = TemplateArchive::new( "test" );

  let high_bit: Vec< u8 > = ( 128..=255 ).collect();

  archive.add_binary_file( PathBuf::from( "highbit.bin" ), high_bit.clone() );

  let json = archive.to_json().unwrap();
  let restored = TemplateArchive::from_json( &json ).unwrap();

  let file = restored.get_file( Path::new( "highbit.bin" ) ).unwrap();
  if let FileContent::Binary( bytes ) = &file.content
  {
    assert_eq!( bytes, &high_bit );
  }
  else
  {
    panic!( "Expected binary content" );
  }
}

/// Test realistic binary file (PNG header)
#[ test ]
fn binary_png_header()
{
  let mut archive = TemplateArchive::new( "test" );

  // Real PNG file header
  let png_header = vec![ 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A ];

  archive.add_binary_file( PathBuf::from( "image.png" ), png_header.clone() );

  let json = archive.to_json().unwrap();
  let restored = TemplateArchive::from_json( &json ).unwrap();

  let file = restored.get_file( Path::new( "image.png" ) ).unwrap();
  if let FileContent::Binary( bytes ) = &file.content
  {
    assert_eq!( bytes, &png_header );
  }
  else
  {
    panic!( "Expected binary content" );
  }
}

//

#[ test ]
fn json_pretty_print()
{
  let mut archive = TemplateArchive::new( "test" );
  archive.add_text_file( PathBuf::from( "a.txt" ), "content", WriteMode::Rewrite );

  let json = archive.to_json_pretty().unwrap();

  // Pretty-printed JSON should have indentation
  assert!( json.contains( "  " ) );
  assert!( json.contains( '\n' ) );
}

#[ test ]
fn json_roundtrip_complete_archive()
{
  let mut archive = TemplateArchive::new( "complete-test" );
  archive.set_version( "1.2.3" );
  archive.set_description( "Test archive" );

  archive.add_text_file(
    PathBuf::from( "src/main.rs" ),
    "fn main() { println!(\"{{greeting}}\"); }",
    WriteMode::Rewrite
  );

  archive.add_binary_file(
    PathBuf::from( "assets/logo.png" ),
    vec![ 0x89, 0x50, 0x4E, 0x47 ]
  );

  archive.add_parameter( ParameterDescriptor
  {
    parameter: "greeting".into(),
    is_mandatory: true,
    default_value: None,
    description: Some( "Greeting message".into() ),
  });

  archive.set_value( "greeting", Value::String( "Hello".into() ) );

  let json = archive.to_json().unwrap();
  let restored = TemplateArchive::from_json( &json ).unwrap();

  assert_eq!( restored.name, "complete-test" );
  assert_eq!( restored.version, "1.2.3" );
  assert_eq!( restored.description, Some( "Test archive".to_string() ) );
  assert_eq!( restored.file_count(), 2 );
  assert_eq!( restored.list_parameters().len(), 1 );
  assert_eq!( restored.get_value( "greeting" ), Some( &Value::String( "Hello".into() ) ) );
}

#[ test ]
fn yaml_roundtrip_complete_archive()
{
  let mut archive = TemplateArchive::new( "complete-test" );
  archive.set_version( "2.0.0" );

  archive.add_text_file(
    PathBuf::from( "config.toml" ),
    "host = \"{{host}}\"",
    WriteMode::Rewrite
  );

  archive.add_parameter( ParameterDescriptor
  {
    parameter: "host".into(),
    is_mandatory: true,
    default_value: Some( "localhost".into() ),
    description: None,
  });

  let yaml = archive.to_yaml().unwrap();
  let restored = TemplateArchive::from_yaml( &yaml ).unwrap();

  assert_eq!( restored.name, "complete-test" );
  assert_eq!( restored.version, "2.0.0" );
  assert_eq!( restored.file_count(), 1 );

  let param = restored.get_parameter( "host" ).unwrap();
  assert_eq!( param.default_value, Some( "localhost".to_string() ) );
}

//

#[ test ]
fn materialize_simple()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file(
    PathBuf::from( "output.txt" ),
    "Hello {{name}}!",
    WriteMode::Rewrite
  );

  archive.set_value( "name", Value::String( "World".into() ) );

  let mut filesystem = MemoryFileSystem::new();
  let renderer = HandlebarsRenderer::new();

  let report = archive.materialize_with_components(
    Path::new( "/output" ),
    &renderer,
    &mut filesystem
  ).unwrap();

  assert_eq!( report.files_created.len(), 1 );
  assert!( report.files_created.contains( &PathBuf::from( "output.txt" ) ) );

  let content = filesystem.read( &PathBuf::from( "/output/output.txt" ) ).unwrap();
  assert_eq!( content, "Hello World!" );
}

#[ test ]
fn materialize_deep_directories()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file(
    PathBuf::from( "a/b/c/deep.txt" ),
    "content",
    WriteMode::Rewrite
  );

  let mut filesystem = MemoryFileSystem::new();
  let renderer = HandlebarsRenderer::new();

  let report = archive.materialize_with_components(
    Path::new( "/base" ),
    &renderer,
    &mut filesystem
  ).unwrap();

  assert_eq!( report.directories_created.len(), 3 );
  assert!( report.directories_created.contains( &PathBuf::from( "a" ) ) );
  assert!( report.directories_created.contains( &PathBuf::from( "a/b" ) ) );
  assert!( report.directories_created.contains( &PathBuf::from( "a/b/c" ) ) );

  let content = filesystem.read( &PathBuf::from( "/base/a/b/c/deep.txt" ) ).unwrap();
  assert_eq!( content, "content" );
}

#[ test ]
fn materialize_multiple_files()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file(
    PathBuf::from( "file1.txt" ),
    "First: {{value}}",
    WriteMode::Rewrite
  );

  archive.add_text_file(
    PathBuf::from( "dir/file2.txt" ),
    "Second: {{value}}",
    WriteMode::Rewrite
  );

  archive.set_value( "value", Value::Number( 42 ) );

  let mut filesystem = MemoryFileSystem::new();
  let renderer = HandlebarsRenderer::new();

  let report = archive.materialize_with_components(
    Path::new( "/out" ),
    &renderer,
    &mut filesystem
  ).unwrap();

  assert_eq!( report.files_created.len(), 2 );

  assert_eq!( filesystem.read( &PathBuf::from( "/out/file1.txt" ) ).unwrap(), "First: 42" );
  assert_eq!( filesystem.read( &PathBuf::from( "/out/dir/file2.txt" ) ).unwrap(), "Second: 42" );
}

#[ test ]
fn materialize_without_values()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file(
    PathBuf::from( "test.txt" ),
    "No substitution",
    WriteMode::Rewrite
  );

  let mut filesystem = MemoryFileSystem::new();
  let renderer = HandlebarsRenderer::new();

  // Should work even without setting any values
  let report = archive.materialize_with_components(
    Path::new( "/out" ),
    &renderer,
    &mut filesystem
  ).unwrap();

  assert_eq!( report.files_created.len(), 1 );

  let content = filesystem.read( &PathBuf::from( "/out/test.txt" ) ).unwrap();
  assert_eq!( content, "No substitution" );
}
