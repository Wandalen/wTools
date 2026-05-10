//! Integration tests for content commands
//!
//! Tests content operations (.content.list, .externalize, .internalize)

use std::fs;

mod cli_runner;

#[ test ]
fn content_list_shows_inline_files()
{
  let script = ".archive.new name::test\n\
                .file.add path::file1.txt content::\"content1\"\n\
                .file.add path::file2.txt content::\"content2\"\n\
                .content.list\n\
                exit";

  let output = cli_runner::repl_command( script )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Inline content (2):" ), "Should show inline content count" );
  assert!( stdout.contains( "file1.txt" ), "Should list file1" );
  assert!( stdout.contains( "file2.txt" ), "Should list file2" );
}

#[ test ]
fn content_externalize_creates_file_refs()
{
  let temp_dir = std::env::temp_dir();
  let content_dir = temp_dir.join( "test_content_externalize" );
  let archive_path = temp_dir.join( "test_externalize.json" );

  // Clean up
  let _ = fs::remove_dir_all( &content_dir );
  let _ = fs::remove_file( &archive_path );

  let script = format!(
    ".archive.new name::test\n\
     .file.add path::file1.txt content::\"Hello World\"\n\
     .file.add path::file2.txt content::\"Test Content\"\n\
     .content.externalize base_path::{}\n\
     .content.list\n\
     .archive.save path::{}\n\
     exit",
    content_dir.display(),
    archive_path.display()
  );

  let output = cli_runner::repl_command( &script )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Externalized 2 file(s)" ), "Should externalize 2 files" );
  assert!( stdout.contains( "File references (2):" ), "Should show file references" );

  // Verify content files were created
  assert!( content_dir.exists(), "Content directory should exist" );
  let content_files = fs::read_dir( &content_dir ).unwrap().count();
  assert_eq!( content_files, 2, "Should have 2 content files" );

  // Clean up
  let _ = fs::remove_dir_all( &content_dir );
  let _ = fs::remove_file( &archive_path );
}

#[ test ]
fn content_internalize_converts_refs_to_inline()
{
  let temp_dir = std::env::temp_dir();
  let content_dir = temp_dir.join( "test_content_internalize" );

  // Clean up
  let _ = fs::remove_dir_all( &content_dir );

  // First externalize, then internalize
  let script = format!(
    ".archive.new name::test\n\
     .file.add path::file1.txt content::\"Hello World\"\n\
     .file.add path::file2.txt content::\"Test Content\"\n\
     .content.externalize base_path::{}\n\
     .content.list\n\
     .content.internalize\n\
     .content.list\n\
     exit",
    content_dir.display()
  );

  let output = cli_runner::repl_command( &script )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "File references (2):" ), "Should show file refs before internalize" );
  assert!( stdout.contains( "Internalized 2 external reference(s)" ), "Should internalize 2 refs" );
  assert!( stdout.contains( "Inline content (2):" ), "Should show inline content after internalize" );

  // Clean up
  let _ = fs::remove_dir_all( &content_dir );
}

#[ test ]
fn content_externalize_dry_run()
{
  let temp_dir = std::env::temp_dir();
  let content_dir = temp_dir.join( "test_dry_run" );

  // Clean up
  let _ = fs::remove_dir_all( &content_dir );

  let script = format!(
    ".archive.new name::test\n\
     .file.add path::file1.txt content::\"Test\"\n\
     .content.externalize base_path::{} dry::true\n\
     .content.list\n\
     exit",
    content_dir.display()
  );

  let output = cli_runner::repl_command( &script )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Would externalize 1 file(s)" ), "Should show dry run message" );
  assert!( stdout.contains( "Inline content (1):" ), "Content should still be inline" );
  assert!( !content_dir.exists(), "Content directory should not be created in dry run" );
}

#[ test ]
fn content_internalize_dry_run()
{
  let temp_dir = std::env::temp_dir();
  let content_dir = temp_dir.join( "test_dry_run_internalize" );

  // Clean up
  let _ = fs::remove_dir_all( &content_dir );

  let script = format!(
    ".archive.new name::test\n\
     .file.add path::file1.txt content::\"Test\"\n\
     .content.externalize base_path::{}\n\
     .content.internalize dry::true\n\
     .content.list\n\
     exit",
    content_dir.display()
  );

  let output = cli_runner::repl_command( &script )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Would internalize 1 external reference(s)" ), "Should show dry run message" );
  assert!( stdout.contains( "File references (1):" ), "Content should still be external" );

  // Clean up
  let _ = fs::remove_dir_all( &content_dir );
}

#[ test ]
fn content_list_filter_by_type()
{
  let temp_dir = std::env::temp_dir();
  let content_dir = temp_dir.join( "test_filter" );

  // Clean up
  let _ = fs::remove_dir_all( &content_dir );

  let script = format!(
    ".archive.new name::test\n\
     .file.add path::file1.txt content::\"inline1\"\n\
     .file.add path::file2.txt content::\"inline2\"\n\
     .file.add path::file3.txt content::\"inline3\"\n\
     .content.externalize base_path::{}\n\
     .content.list filter::inline\n\
     .content.list filter::file\n\
     exit",
    content_dir.display()
  );

  let output = cli_runner::repl_command( &script )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  // After externalize, all should be file refs, so filter::inline shows nothing
  // and filter::file shows the 3 files
  assert!( stdout.contains( "File references (3):" ), "Should show 3 file references when filtered by file" );
  assert!( stdout.contains( "file1.txt" ), "Should show file1 in file refs" );
  assert!( stdout.contains( "file2.txt" ), "Should show file2 in file refs" );
  assert!( stdout.contains( "file3.txt" ), "Should show file3 in file refs" );

  // Clean up
  let _ = fs::remove_dir_all( &content_dir );
}

#[ test ]
fn content_list_without_archive_returns_error()
{
  let script = ".content.list\nexit";

  let output = cli_runner::repl_command( script )
    .output()
    .expect( "Command should execute" );

  let combined = String::from_utf8_lossy( &output.stdout );
  assert!(
    combined.contains( "No archive loaded" ) || combined.contains( "ERROR" ),
    "Should show error when no archive is loaded"
  );
}

#[ test ]
fn content_externalize_without_archive_returns_error()
{
  let script = ".content.externalize base_path::/tmp/test\nexit";

  let output = cli_runner::repl_command( script )
    .output()
    .expect( "Command should execute" );

  let combined = String::from_utf8_lossy( &output.stdout );
  assert!(
    combined.contains( "No archive loaded" ) || combined.contains( "ERROR" ),
    "Should show error when no archive is loaded"
  );
}

#[ test ]
fn content_internalize_without_archive_returns_error()
{
  let script = ".content.internalize\nexit";

  let output = cli_runner::repl_command( script )
    .output()
    .expect( "Command should execute" );

  let combined = String::from_utf8_lossy( &output.stdout );
  assert!(
    combined.contains( "No archive loaded" ) || combined.contains( "ERROR" ),
    "Should show error when no archive is loaded"
  );
}

// FT-03 (feature/005): Internalize makes archive self-contained (portable after save)
//
// WHY: The portability guarantee requires that after externalize→internalize,
// a saved archive carries all content inline and can be loaded elsewhere
// (no dependency on the original base_path directory).
#[ test ]
fn test_content_internalize_produces_self_contained_archive()
{
  let temp_dir = std::env::temp_dir();
  let content_dir = temp_dir.join( "test_self_contained_content" );
  let archive_path = temp_dir.join( "test_self_contained.json" );

  // Clean up
  let _ = fs::remove_dir_all( &content_dir );
  let _ = fs::remove_file( &archive_path );

  // Externalize then internalize and save
  let script = format!(
    ".archive.new name::portable\n\
     .file.add path::config.toml content::\"name = \\\"test\\\"\"\n\
     .content.externalize base_path::{}\n\
     .content.internalize\n\
     .archive.save path::{}\n\
     exit",
    content_dir.display(),
    archive_path.display()
  );

  let output = cli_runner::repl_command( &script )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!(
    output.status.success(),
    "Externalize → internalize → save should succeed. stdout: {stdout}"
  );
  assert!( stdout.contains( "Internalized 1 external reference(s)" ), "Should internalize" );
  assert!( archive_path.exists(), "Archive should be saved" );

  // Verify saved archive contains inline content (not a file reference)
  let json = fs::read_to_string( &archive_path ).expect( "Should read archive" );
  assert!(
    json.contains( "name = " ) || json.contains( "config.toml" ),
    "Saved archive should embed file content inline"
  );
  // File references store a path string; inline content stores the text directly.
  // The JSON for an inline file contains the actual text bytes, not an external path.
  assert!(
    !json.contains( content_dir.to_string_lossy().as_ref() ),
    "Saved archive should not reference the external content directory"
  );

  // Clean up
  let _ = fs::remove_dir_all( &content_dir );
  let _ = fs::remove_file( &archive_path );
}
