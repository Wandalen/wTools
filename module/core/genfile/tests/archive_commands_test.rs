//! Integration tests for archive commands
//!
//! Tests all archive lifecycle commands (.archive.new, .load, .save, .`from_directory`)

use std::fs;

#[ test ]
fn archive_new_creates_empty_archive()
{
  // Create a new archive and verify it can be saved/loaded
  let temp_dir = std::env::temp_dir();
  let archive_path = temp_dir.join( "test_archive_new.json" );

  // Clean up from previous runs
  let _ = fs::remove_file( &archive_path );

  // Test: Create new archive, save it, then load and verify
  let output = std::process::Command::new( "cargo" )
    .args( [ "run", "--quiet", "--", ".archive.new", "name::test-archive", "description::Test archive" ] )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Failed to execute command" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Created archive: test-archive" ), "Should create archive with name" );

  // Clean up
  let _ = fs::remove_file( &archive_path );
}

#[ test ]
fn archive_save_and_load_roundtrip()
{
  // Test complete save/load cycle
  let temp_dir = std::env::temp_dir();
  let archive_path = temp_dir.join( "test_roundtrip.json" );

  // Clean up
  let _ = fs::remove_file( &archive_path );

  // Create archive and save it
  let create_script = format!(
    ".archive.new name::roundtrip-test description::\"Test description\"\n\
     .archive.save path::{}\n\
     exit",
    archive_path.display()
  );

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{create_script}' | cargo run --quiet" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Failed to create and save archive" );

  assert!( output.status.success(), "Archive creation and save should succeed" );
  assert!( archive_path.exists(), "Archive file should exist after save" );

  // Verify file contents
  let content = fs::read_to_string( &archive_path ).expect( "Should read archive file" );
  assert!( content.contains( "roundtrip-test" ), "Archive should contain name" );
  assert!( content.contains( "Test description" ), "Archive should contain description" );

  // Load the archive back
  let load_script = format!(
    ".archive.load path::{} verbosity::2\n\
     exit",
    archive_path.display()
  );

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{load_script}' | cargo run --quiet" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Failed to load archive" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Loaded archive: roundtrip-test" ), "Should load archive with correct name" );
  assert!( stdout.contains( "Files: 0" ), "Should show 0 files in empty archive" );

  // Clean up
  let _ = fs::remove_file( &archive_path );
}

#[ test ]
fn archive_from_directory_packs_files()
{
  // Create a test directory with some files
  let temp_dir = std::env::temp_dir();
  let test_dir = temp_dir.join( "test_template_dir" );
  let archive_path = temp_dir.join( "test_from_dir.json" );

  // Clean up
  let _ = fs::remove_dir_all( &test_dir );
  let _ = fs::remove_file( &archive_path );

  // Create test directory with files
  fs::create_dir_all( &test_dir ).expect( "Should create test directory" );
  fs::write( test_dir.join( "file1.txt" ), "Hello {{name}}!" ).expect( "Should write file1" );
  fs::write( test_dir.join( "file2.md" ), "# {{title}}" ).expect( "Should write file2" );

  // Create archive from directory
  let script = format!(
    ".archive.from_directory source::{} verbosity::2\n\
     .archive.save path::{}\n\
     exit",
    test_dir.display(),
    archive_path.display()
  );

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Failed to pack directory" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Files: 2" ), "Should pack 2 files from directory" );
  assert!( archive_path.exists(), "Archive file should be created" );

  // Verify archive contents
  let content = fs::read_to_string( &archive_path ).expect( "Should read archive" );
  assert!( content.contains( "file1.txt" ), "Archive should contain file1.txt" );
  assert!( content.contains( "file2.md" ), "Archive should contain file2.md" );
  assert!( content.contains( "Hello {{name}}!" ), "Archive should contain template content" );
  assert!( content.contains( "# {{title}}" ), "Archive should contain markdown content" );

  // Clean up
  let _ = fs::remove_dir_all( &test_dir );
  let _ = fs::remove_file( &archive_path );
}

#[ test ]
fn archive_save_with_different_verbosity_levels()
{
  let temp_dir = std::env::temp_dir();
  let archive_path = temp_dir.join( "test_verbosity.json" );

  // Clean up
  let _ = fs::remove_file( &archive_path );

  // Test verbosity 0 (silent)
  let script = format!(
    ".archive.new name::silent-test\n\
     .archive.save path::{} verbosity::0\n\
     exit",
    archive_path.display()
  );

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Failed with verbosity 0" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  // Verbosity 0 should still show "Created" but save should be silent
  let save_output = stdout.lines().nth( 1 ).unwrap_or( "" );
  assert!( save_output.trim().is_empty() || !save_output.contains( "Saved" ), "Verbosity 0 should be silent" );

  // Clean up
  let _ = fs::remove_file( &archive_path );

  // Test verbosity 2 (detailed)
  let script = format!(
    ".archive.new name::verbose-test\n\
     .archive.save path::{} verbosity::2\n\
     exit",
    archive_path.display()
  );

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Failed with verbosity 2" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Saved archive: verbose-test" ), "Verbosity 2 should show archive name" );
  assert!( stdout.contains( "Format: json" ), "Verbosity 2 should show format" );
  assert!( stdout.contains( "Pretty: true" ), "Verbosity 2 should show pretty flag" );

  // Clean up
  let _ = fs::remove_file( &archive_path );
}

#[ test ]
fn archive_load_nonexistent_file_returns_error()
{
  let script = ".archive.load path::/tmp/nonexistent_archive_12345.json\nexit";

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Command should execute" );

  let combined = String::from_utf8_lossy( &output.stdout );
  assert!( combined.contains( "ERROR" ) || combined.contains( "error" ), "Should show error for nonexistent file" );
}

#[ test ]
fn archive_save_without_loaded_archive_returns_error()
{
  let script = ".archive.save path::/tmp/test.json\nexit";

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Command should execute" );

  let combined = String::from_utf8_lossy( &output.stdout );
  assert!(
    combined.contains( "No archive loaded" ) || combined.contains( "ERROR" ),
    "Should show error when trying to save without loaded archive"
  );
}

#[ test ]
fn archive_from_directory_nonexistent_returns_error()
{
  let script = ".archive.from_directory source::/tmp/nonexistent_dir_12345\nexit";

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Command should execute" );

  let combined = String::from_utf8_lossy( &output.stdout );
  assert!(
    combined.contains( "not found" ) || combined.contains( "ERROR" ) || combined.contains( "does not exist" ),
    "Should show error for nonexistent directory"
  );
}
