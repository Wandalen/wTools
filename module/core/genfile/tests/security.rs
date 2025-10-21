//! Security validation tests
//!
//! Tests for path traversal prevention and other security features.

use genfile_core::
{
  TemplateArchive,
  FileContent,
  WriteMode,
  MemoryFileSystem,
  HandlebarsRenderer,
  validate_path,
};
use std::path::{ Path, PathBuf };

// === Unit Tests for validate_path Function ===

#[ test ]
fn validate_path_accepts_simple_filename()
{
  assert!( validate_path( Path::new( "file.txt" ) ).is_ok() );
  assert!( validate_path( Path::new( "readme.md" ) ).is_ok() );
}

#[ test ]
fn validate_path_accepts_nested_path()
{
  assert!( validate_path( Path::new( "src/lib.rs" ) ).is_ok() );
  assert!( validate_path( Path::new( "a/b/c/d.txt" ) ).is_ok() );
  assert!( validate_path( Path::new( "deeply/nested/directory/structure/file.txt" ) ).is_ok() );
}

#[ test ]
fn validate_path_accepts_current_dir_prefix()
{
  assert!( validate_path( Path::new( "./file.txt" ) ).is_ok() );
  assert!( validate_path( Path::new( "./src/lib.rs" ) ).is_ok() );
}

#[ test ]
fn validate_path_accepts_current_dir_in_middle()
{
  assert!( validate_path( Path::new( "foo/./bar.txt" ) ).is_ok() );
  assert!( validate_path( Path::new( "a/./b/./c.txt" ) ).is_ok() );
}

#[ test ]
fn validate_path_rejects_parent_dir_prefix()
{
  let result = validate_path( Path::new( "../etc/passwd" ) );
  assert!( result.is_err() );
  let msg = format!( "{}", result.unwrap_err() );
  assert!( msg.contains( "directory traversal" ) );
}

#[ test ]
fn validate_path_rejects_parent_dir_suffix()
{
  let result = validate_path( Path::new( "foo/../bar" ) );
  assert!( result.is_err() );
}

#[ test ]
fn validate_path_rejects_nested_parent_dirs()
{
  assert!( validate_path( Path::new( "../../etc/passwd" ) ).is_err() );
  assert!( validate_path( Path::new( "foo/../../bar" ) ).is_err() );
  assert!( validate_path( Path::new( "a/b/c/../../../d" ) ).is_err() );
}

#[ test ]
fn validate_path_rejects_parent_in_middle()
{
  assert!( validate_path( Path::new( "a/../b" ) ).is_err() );
  assert!( validate_path( Path::new( "src/../etc/passwd" ) ).is_err() );
}

#[ test ]
fn validate_path_error_message_includes_path()
{
  let result = validate_path( Path::new( "../malicious/file.txt" ) );
  assert!( result.is_err() );

  let err = result.unwrap_err();
  let msg = format!( "{}", err );
  assert!( msg.contains( "directory traversal" ) );
  assert!( msg.contains( "../malicious/file.txt" ) );
}

// === Integration Tests with TemplateArchive Materialization ===

#[ test ]
fn materialize_blocks_parent_dir_traversal()
{
  let mut archive = TemplateArchive::new( "test" );

  // Try to add file with malicious path
  archive.add_file(
    PathBuf::from( "../etc/passwd" ),
    FileContent::Text( "malicious content".to_string() ),
    WriteMode::Rewrite
  );

  // Materialization should fail
  let renderer = HandlebarsRenderer::new();
  let mut fs = MemoryFileSystem::new();
  let result = archive.materialize_with_components(
    Path::new( "/safe-output" ),
    &renderer,
    &mut fs
  );

  assert!( result.is_err() );
  let err = result.unwrap_err();
  let msg = format!( "{}", err );
  assert!( msg.contains( "directory traversal" ) );
}

#[ test ]
fn materialize_blocks_nested_parent_dirs()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file(
    PathBuf::from( "foo/../../etc/passwd" ),
    "malicious",
    WriteMode::Rewrite
  );

  let renderer = HandlebarsRenderer::new();
  let mut fs = MemoryFileSystem::new();
  let result = archive.materialize_with_components(
    Path::new( "/output" ),
    &renderer,
    &mut fs
  );

  assert!( result.is_err() );
}

#[ test ]
fn materialize_blocks_parent_in_middle()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file(
    PathBuf::from( "src/../etc/passwd" ),
    "malicious",
    WriteMode::Rewrite
  );

  let renderer = HandlebarsRenderer::new();
  let mut fs = MemoryFileSystem::new();
  let result = archive.materialize_with_components(
    Path::new( "/output" ),
    &renderer,
    &mut fs
  );

  assert!( result.is_err() );
}

#[ test ]
fn materialize_allows_safe_paths()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file(
    PathBuf::from( "src/lib.rs" ),
    "safe content",
    WriteMode::Rewrite
  );

  archive.add_text_file(
    PathBuf::from( "README.md" ),
    "# Safe",
    WriteMode::Rewrite
  );

  let renderer = HandlebarsRenderer::new();
  let mut fs = MemoryFileSystem::new();
  let result = archive.materialize_with_components(
    Path::new( "/output" ),
    &renderer,
    &mut fs
  );

  assert!( result.is_ok() );
  let report = result.unwrap();
  assert_eq!( report.files_created.len(), 2 );
}

#[ test ]
fn materialize_allows_current_dir_references()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file(
    PathBuf::from( "./src/main.rs" ),
    "content",
    WriteMode::Rewrite
  );

  archive.add_text_file(
    PathBuf::from( "foo/./bar.txt" ),
    "content",
    WriteMode::Rewrite
  );

  let renderer = HandlebarsRenderer::new();
  let mut fs = MemoryFileSystem::new();
  let result = archive.materialize_with_components(
    Path::new( "/output" ),
    &renderer,
    &mut fs
  );

  assert!( result.is_ok() );
}

#[ test ]
fn materialize_with_multiple_files_stops_on_first_malicious()
{
  let mut archive = TemplateArchive::new( "test" );

  // Add some safe files
  archive.add_text_file(
    PathBuf::from( "file1.txt" ),
    "safe",
    WriteMode::Rewrite
  );

  archive.add_text_file(
    PathBuf::from( "file2.txt" ),
    "safe",
    WriteMode::Rewrite
  );

  // Add malicious file
  archive.add_text_file(
    PathBuf::from( "../malicious.txt" ),
    "bad",
    WriteMode::Rewrite
  );

  // Add more safe files (shouldnt be processed)
  archive.add_text_file(
    PathBuf::from( "file3.txt" ),
    "safe",
    WriteMode::Rewrite
  );

  let renderer = HandlebarsRenderer::new();
  let mut fs = MemoryFileSystem::new();
  let result = archive.materialize_with_components(
    Path::new( "/output" ),
    &renderer,
    &mut fs
  );

  // Should fail on malicious file
  assert!( result.is_err() );
}

#[ test ]
fn materialize_validates_before_rendering()
{
  let mut archive = TemplateArchive::new( "test" );

  // Add template with variables AND malicious path
  archive.add_text_file(
    PathBuf::from( "../etc/{{filename}}" ),
    "template content",
    WriteMode::Rewrite
  );

  archive.set_value( "filename", genfile_core::Value::String( "passwd".to_string() ) );

  let renderer = HandlebarsRenderer::new();
  let mut fs = MemoryFileSystem::new();
  let result = archive.materialize_with_components(
    Path::new( "/output" ),
    &renderer,
    &mut fs
  );

  // Should fail BEFORE template rendering (path validation happens first)
  assert!( result.is_err() );
  let err = result.unwrap_err();
  let msg = format!( "{}", err );
  assert!( msg.contains( "directory traversal" ) );
}

#[ test ]
fn validate_path_works_with_deeply_nested_structures()
{
  // Valid deep nesting
  assert!( validate_path( Path::new( "a/b/c/d/e/f/g/h/i/j/file.txt" ) ).is_ok() );

  // Invalid with parent dir anywhere
  assert!( validate_path( Path::new( "a/b/../c/d/e/f/file.txt" ) ).is_err() );
  assert!( validate_path( Path::new( "a/b/c/d/e/../../../f/file.txt" ) ).is_err() );
}

#[ test ]
fn validate_path_handles_various_extensions()
{
  // All should be accepted
  assert!( validate_path( Path::new( "file.txt" ) ).is_ok() );
  assert!( validate_path( Path::new( "image.png" ) ).is_ok() );
  assert!( validate_path( Path::new( "script.sh" ) ).is_ok() );
  assert!( validate_path( Path::new( "Cargo.toml" ) ).is_ok() );
  assert!( validate_path( Path::new( ".gitignore" ) ).is_ok() );
  assert!( validate_path( Path::new( "no_extension" ) ).is_ok() );
}

#[ test ]
fn validate_path_handles_unicode()
{
  assert!( validate_path( Path::new( "файл.txt" ) ).is_ok() );
  assert!( validate_path( Path::new( "文件.md" ) ).is_ok() );
  assert!( validate_path( Path::new( "καταχώριση.rs" ) ).is_ok() );

  // Parent dir still blocked regardless of unicode
  assert!( validate_path( Path::new( "../файл.txt" ) ).is_err() );
}

// === Security Documentation Tests ===

/// Verify the security guarantee documented in spec:
/// "File descriptor paths must be validated to prevent directory traversal attacks"
#[ test ]
fn spec_requirement_path_traversal_validation()
{
  // This test documents the security requirement from spec.md
  let archive = TemplateArchive::new( "security-test" );

  // Attempt various directory traversal patterns
  let malicious_paths = vec![
    "../etc/passwd",
    "../../root/.ssh/id_rsa",
    "foo/../../../etc/shadow",
    "./../malicious",
  ];

  for malicious_path in malicious_paths
  {
    let mut test_archive = archive.clone();
    test_archive.add_text_file(
      PathBuf::from( malicious_path ),
      "malicious content",
      WriteMode::Rewrite
    );

    let renderer = HandlebarsRenderer::new();
    let mut fs = MemoryFileSystem::new();
    let result = test_archive.materialize_with_components(
      Path::new( "/output" ),
      &renderer,
      &mut fs
    );

    assert!(
      result.is_err(),
      "Path '{}' should be rejected but was allowed",
      malicious_path
    );
  }
}
