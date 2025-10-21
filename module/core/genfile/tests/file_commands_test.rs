//! Integration tests for file commands
//!
//! Tests file operations (.file.add, .remove, .list, .show)

use std::fs;

#[ test ]
fn file_add_with_inline_content()
{
  let temp_dir = std::env::temp_dir();
  let archive_path = temp_dir.join( "test_file_add.json" );

  // Clean up
  let _ = fs::remove_file( &archive_path );

  let script = format!(
    ".archive.new name::test\n\
     .file.add path::src/main.rs content::\"fn main() {{}}\"\n\
     .file.list verbosity::2\n\
     .archive.save path::{}\n\
     exit",
    archive_path.display()
  );

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Added file: src/main.rs" ), "Should add file" );
  assert!( stdout.contains( "Files (1):" ), "Should list 1 file" );
  assert!( stdout.contains( "src/main.rs" ), "Should show file in list" );

  // Verify saved archive
  assert!( archive_path.exists(), "Archive should be saved" );
  let content = fs::read_to_string( &archive_path ).unwrap();
  assert!( content.contains( "src/main.rs" ), "Archive should contain file path" );
  assert!( content.contains( "fn main()" ), "Archive should contain file content" );

  // Clean up
  let _ = fs::remove_file( &archive_path );
}

#[ test ]
fn file_add_from_source_file()
{
  let temp_dir = std::env::temp_dir();
  let source_file = temp_dir.join( "test_source.txt" );
  let archive_path = temp_dir.join( "test_file_from_file.json" );

  // Clean up
  let _ = fs::remove_file( &source_file );
  let _ = fs::remove_file( &archive_path );

  // Create source file
  fs::write( &source_file, "Hello {{name}}!" ).expect( "Should write source file" );

  let script = format!(
    ".archive.new name::test\n\
     .file.add path::template.txt from_file::{}\n\
     .file.show path::template.txt\n\
     .archive.save path::{}\n\
     exit",
    source_file.display(),
    archive_path.display()
  );

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Added file: template.txt" ), "Should add file from source" );
  assert!( stdout.contains( "Hello {{name}}!" ), "Should show file content" );

  // Clean up
  let _ = fs::remove_file( &source_file );
  let _ = fs::remove_file( &archive_path );
}

#[ test ]
fn file_remove_deletes_file()
{
  let script = ".archive.new name::test\n\
                .file.add path::test.txt content::\"test\"\n\
                .file.list\n\
                .file.remove path::test.txt\n\
                .file.list\n\
                exit";

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );

  // First list should show file
  let lines: Vec<&str> = stdout.lines().collect();
  let first_list_idx = lines.iter().position( | l | l.contains( "Files (1):" ) );
  assert!( first_list_idx.is_some(), "First list should show 1 file" );

  // Should show removal
  assert!( stdout.contains( "Removed file: test.txt" ), "Should remove file" );

  // Second list should show no files
  assert!( stdout.contains( "No files in archive" ), "Should show no files after removal" );
}

#[ test ]
fn file_list_shows_all_files()
{
  let script = ".archive.new name::test\n\
                .file.add path::file1.txt content::\"content1\"\n\
                .file.add path::file2.txt content::\"content2\"\n\
                .file.add path::dir/file3.txt content::\"content3\"\n\
                .file.list\n\
                exit";

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Files (3):" ), "Should list 3 files" );
  assert!( stdout.contains( "file1.txt" ), "Should show file1" );
  assert!( stdout.contains( "file2.txt" ), "Should show file2" );
  assert!( stdout.contains( "dir/file3.txt" ), "Should show file3 with path" );
}

#[ test ]
fn file_show_displays_content()
{
  let script = ".archive.new name::test\n\
                .file.add path::test.txt content::\"Hello World\"\n\
                .file.show path::test.txt\n\
                exit";

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Hello World" ), "Should show file content" );
  assert!( stdout.contains( "test.txt" ), "Should show filename" );
}

#[ test ]
fn file_add_without_archive_returns_error()
{
  let script = ".file.add path::test.txt content::\"test\"\nexit";

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Command should execute" );

  let combined = String::from_utf8_lossy( &output.stdout );
  assert!(
    combined.contains( "No archive loaded" ) || combined.contains( "ERROR" ),
    "Should show error when no archive is loaded"
  );
}

#[ test ]
fn file_remove_nonexistent_returns_error()
{
  let script = ".archive.new name::test\n\
                .file.remove path::nonexistent.txt\n\
                exit";

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Command should execute" );

  let combined = String::from_utf8_lossy( &output.stdout );
  assert!(
    combined.contains( "not found" ) || combined.contains( "ERROR" ),
    "Should show error for nonexistent file"
  );
}

#[ test ]
fn file_show_nonexistent_returns_error()
{
  let script = ".archive.new name::test\n\
                .file.show path::missing.txt\n\
                exit";

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Command should execute" );

  let combined = String::from_utf8_lossy( &output.stdout );
  assert!(
    combined.contains( "not found" ) || combined.contains( "ERROR" ),
    "Should show error for nonexistent file"
  );
}

#[ test ]
fn file_add_without_content_or_from_file_returns_error()
{
  let script = ".archive.new name::test\n\
                .file.add path::test.txt\n\
                exit";

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Command should execute" );

  let combined = String::from_utf8_lossy( &output.stdout );
  assert!(
    combined.contains( "required" ) || combined.contains( "ERROR" ),
    "Should show error when neither content nor from_file is provided"
  );
}
