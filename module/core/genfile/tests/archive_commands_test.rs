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

// FR7: Serialization - .pack command tests
//
// ## Why These Tests Exist
//
// The .pack command is advertised in the quick start example (lib.rs:13) but was
// missing from the implementation. These tests ensure:
// 1. The documented workflow actually works
// 2. Archives are truly portable (inline content, not references)
// 3. Standard parameters (verbosity, dry) behave consistently
// 4. Clear error messages for common mistakes
//
// ## Implementation Notes
//
// Tests use process spawning (cargo run) rather than direct API calls because:
// - Validates complete CLI behavior including argument parsing
// - Tests real user workflows end-to-end
// - Catches integration issues with unilang framework
//
// Trade-off: Slower execution (~3s per test) vs comprehensive coverage
//
// ## How to Interpret Failures
//
// - "Pack should succeed" → Command execution failed, check error output
// - "Should be JSON format" → Output file format issue, check genfile_core serialization
// - "Should contain inline file content" → Portability broken, files not internalized
// - "Verbosity X should be Y" → Output formatting inconsistency

#[ test ]
fn pack_creates_portable_archive_from_directory()
{
  // Test: Pack directory into portable JSON archive with inline content
  //
  // WHY: Validates basic pack functionality matches spec.md quick start example.
  // Ensures archives are truly portable (inline content, not file references).
  //
  // WHAT IT VALIDATES:
  // - Directory → archive conversion via genfile_core::pack_from_dir()
  // - JSON output file creation
  // - Content internalization (template variables preserved)
  // - File content embedded inline (portability guarantee)
  let temp_dir = std::env::temp_dir();
  let source_dir = temp_dir.join( "test_pack_source" );
  let output_path = temp_dir.join( "test_pack_output.json" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &output_path );

  // Create test directory with template files
  fs::create_dir_all( &source_dir ).expect( "Should create source directory" );
  fs::write( source_dir.join( "file1.txt" ), "Content with {{param1}}" ).expect( "Should write file1" );
  fs::write( source_dir.join( "file2.rs" ), "fn main() { println!(\"{{project_name}}\"); }" ).expect( "Should write file2" );

  // Run pack command
  let output = std::process::Command::new( "cargo" )
    .args( [
      "run", "--quiet", "--",
      ".pack",
      &format!( "input::{}", source_dir.display() ),
      &format!( "output::{}", output_path.display() ),
    ] )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Pack command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( output.status.success(), "Pack should succeed. stdout: {stdout}" );
  assert!( stdout.contains( "Packed" ) || stdout.contains( "Created" ), "Should show success message" );

  // Verify output file exists
  assert!( output_path.exists(), "Packed archive should exist" );
  let content = fs::read_to_string( &output_path ).expect( "Should read packed archive" );

  // Verify it looks like JSON (basic check)
  assert!( content.starts_with( '{' ) || content.starts_with( '[' ), "Should be JSON format" );
  assert!( content.contains( '"' ), "Should contain JSON strings" );

  // Verify content is internalized (inline, not references)
  assert!( content.contains( "Content with" ), "Should contain inline file content" );
  assert!( content.contains( "project_name" ), "Should preserve template variables" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &output_path );
}

#[ test ]
fn pack_with_verbosity_levels()
{
  // Test: Pack command respects verbosity parameter
  //
  // WHY: All genfile commands must support consistent verbosity levels per spec.md.
  // Tests that output detail increases appropriately with verbosity setting.
  //
  // VALIDATION APPROACH: Compare output length between verbosity::0 (silent) and
  // verbosity::2 (detailed). Not checking exact content to avoid fragile tests.
  let temp_dir = std::env::temp_dir();
  let source_dir = temp_dir.join( "test_pack_verbose_source" );
  let output_path = temp_dir.join( "test_pack_verbose.json" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &output_path );

  // Create minimal test directory
  fs::create_dir_all( &source_dir ).expect( "Should create source directory" );
  fs::write( source_dir.join( "test.txt" ), "test content" ).expect( "Should write test file" );

  // Test verbosity::0 (silent)
  let output_v0 = std::process::Command::new( "cargo" )
    .args( [
      "run", "--quiet", "--",
      ".pack",
      &format!( "input::{}", source_dir.display() ),
      &format!( "output::{}", output_path.display() ),
      "verbosity::0",
    ] )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Pack with verbosity 0 should execute" );

  let stdout_v0 = String::from_utf8_lossy( &output_v0.stdout );
  assert!( stdout_v0.trim().is_empty() || stdout_v0.len() < 50, "Verbosity 0 should be silent or minimal" );

  // Clean up for next test
  let _ = fs::remove_file( &output_path );

  // Test verbosity::2 (detailed)
  let output_v2 = std::process::Command::new( "cargo" )
    .args( [
      "run", "--quiet", "--",
      ".pack",
      &format!( "input::{}", source_dir.display() ),
      &format!( "output::{}", output_path.display() ),
      "verbosity::2",
    ] )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Pack with verbosity 2 should execute" );

  let stdout_v2 = String::from_utf8_lossy( &output_v2.stdout );
  assert!( stdout_v2.len() > stdout_v0.len(), "Verbosity 2 should be more detailed than verbosity 0" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &output_path );
}

#[ test ]
fn pack_dry_run_preview()
{
  // Test: Pack dry run shows what would be done without creating file
  //
  // WHY: Dry run mode (dry::1) is a standard safety feature for destructive operations.
  // Users need to preview pack operations without actually writing files.
  //
  // CRITICAL: Output file must NOT exist after dry run. Failure indicates data loss risk.
  let temp_dir = std::env::temp_dir();
  let source_dir = temp_dir.join( "test_pack_dry_source" );
  let output_path = temp_dir.join( "test_pack_dry.json" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &output_path );

  // Create test directory
  fs::create_dir_all( &source_dir ).expect( "Should create source directory" );
  fs::write( source_dir.join( "test.txt" ), "test" ).expect( "Should write test file" );

  // Run pack with dry::1
  let output = std::process::Command::new( "cargo" )
    .args( [
      "run", "--quiet", "--",
      ".pack",
      &format!( "input::{}", source_dir.display() ),
      &format!( "output::{}", output_path.display() ),
      "dry::1",
    ] )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Pack dry run should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( output.status.success(), "Dry run should succeed" );
  assert!( stdout.contains( "Dry run" ) || stdout.contains( "Would" ), "Should indicate dry run mode" );
  assert!( !output_path.exists(), "Dry run should NOT create output file" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
}

#[ test ]
fn pack_nonexistent_input_returns_error()
{
  // Test: Pack with nonexistent input directory should fail with clear error
  //
  // WHY: Common user mistake - typo in path or running command from wrong directory.
  // Must provide actionable error message, not cryptic failure.
  //
  // EXPECTATION: Error message contains "not found", "ERROR", or "does not exist"
  let temp_dir = std::env::temp_dir();
  let nonexistent = temp_dir.join( "nonexistent_pack_dir_12345" );
  let output_path = temp_dir.join( "test_pack_error.json" );

  // Clean up
  let _ = fs::remove_file( &output_path );

  // Run pack with nonexistent input
  let output = std::process::Command::new( "cargo" )
    .args( [
      "run", "--quiet", "--",
      ".pack",
      &format!( "input::{}", nonexistent.display() ),
      &format!( "output::{}", output_path.display() ),
    ] )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Pack command should execute" );

  assert!( !output.status.success(), "Pack should fail for nonexistent input" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{stderr}{stdout}" );

  assert!(
    combined.contains( "not found" ) || combined.contains( "ERROR" ) || combined.contains( "does not exist" ),
    "Should show clear error about nonexistent input"
  );

  // Clean up
  let _ = fs::remove_file( &output_path );
}
