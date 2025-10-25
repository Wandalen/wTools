//! Integration tests for archive analysis commands
//!
//! Tests FR8: Archive Analysis - .info, .status, .discover.parameters, .analyze
//!
//! ## Why These Tests Exist
//!
//! Analysis commands are critical for understanding template archives before use.
//! These tests ensure users can:
//! 1. View archive metadata and statistics (.info)
//! 2. Auto-discover template parameters (.discover.parameters)
//! 3. Check archive readiness status (.status)
//! 4. Get comprehensive analysis (.analyze)
//!
//! ## Test Approach
//!
//! Uses REPL mode to test analysis workflow:
//! 1. Create archive with templates containing {{variables}}
//! 2. Load archive
//! 3. Run analysis commands to inspect archive
//!
//! This validates the natural workflow: load → analyze → understand → configure

use std::fs;

// FR8: Archive Analysis Tests

#[ test ]
fn info_displays_archive_metadata()
{
  // Test: .info command shows archive name, version, description, file counts
  //
  // WHY: Users need quick overview of archive contents before working with it.
  // This is the first command they'll run after loading an archive.
  //
  // VALIDATES:
  // - Archive name displayed
  // - File count statistics
  // - Metadata fields present
  // - Clear, readable output format

  let temp_dir = std::env::temp_dir();
  let source_dir = temp_dir.join( "test_info_source" );
  let archive_path = temp_dir.join( "test_info.json" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &archive_path );

  // Create source with multiple files
  fs::create_dir_all( &source_dir ).expect( "Should create source dir" );
  fs::write( source_dir.join( "file1.txt" ), "Content 1" ).expect( "Should write file1" );
  fs::write( source_dir.join( "file2.txt" ), "Content 2" ).expect( "Should write file2" );
  fs::create_dir_all( source_dir.join( "subdir" ) ).expect( "Should create subdir" );
  fs::write( source_dir.join( "subdir" ).join( "file3.txt" ), "Content 3" ).expect( "Should write file3" );

  // REPL workflow: pack → load → info
  let script = format!(
    ".pack input::{} output::{}\n\
     .archive.load path::{}\n\
     .info\n\
     exit",
    source_dir.display(),
    archive_path.display(),
    archive_path.display()
  );

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Info workflow should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!( output.status.success(), "Workflow should succeed. stdout: {stdout}, stderr: {stderr}" );

  // Verify output contains metadata
  assert!( stdout.contains( "test_info_source" ) || stdout.contains( "Archive" ), "Should show archive name" );
  assert!( stdout.contains( '3' ) || stdout.contains( "Files" ), "Should show file count (3 files)" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &archive_path );
}

#[ test ]
fn discover_parameters_finds_template_variables()
{
  // Test: .discover.parameters auto-detects {{variables}} in template files
  //
  // WHY: Manual parameter definition is error-prone. Auto-discovery helps users
  // find all template variables without reading every file.
  //
  // VALIDATES:
  // - Handlebars {{variable}} syntax detected
  // - All unique parameters found across multiple files
  // - Output lists discovered parameter names
  //
  // CRITICAL: This is the key differentiator - automatic parameter detection

  let temp_dir = std::env::temp_dir();
  let source_dir = temp_dir.join( "test_discover_source" );
  let archive_path = temp_dir.join( "test_discover.json" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &archive_path );

  // Create templates with multiple parameters
  fs::create_dir_all( &source_dir ).expect( "Should create source dir" );
  fs::write(
    source_dir.join( "template1.txt" ),
    "Project: {{project_name}}\nVersion: {{version}}"
  ).expect( "Should write template1" );
  fs::write(
    source_dir.join( "template2.txt" ),
    "Author: {{author}}\nLicense: {{license}}\nProject: {{project_name}}"
  ).expect( "Should write template2" );

  // REPL workflow: pack → load → discover
  let script = format!(
    ".pack input::{} output::{}\n\
     .archive.load path::{}\n\
     .discover.parameters\n\
     exit",
    source_dir.display(),
    archive_path.display(),
    archive_path.display()
  );

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Discover workflow should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!( output.status.success(), "Workflow should succeed. stdout: {stdout}, stderr: {stderr}" );

  // Verify all parameters discovered
  assert!( stdout.contains( "project_name" ), "Should discover project_name parameter" );
  assert!( stdout.contains( "version" ), "Should discover version parameter" );
  assert!( stdout.contains( "author" ), "Should discover author parameter" );
  assert!( stdout.contains( "license" ), "Should discover license parameter" );

  // Should show count
  assert!( stdout.contains( '4' ) || stdout.contains( "parameters" ), "Should show parameter count" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &archive_path );
}

#[ test ]
fn status_shows_archive_readiness()
{
  // Test: .status command shows completeness/readiness for materialization
  //
  // WHY: Users need to know if archive is ready to materialize or if they need
  // to configure parameters first. Status provides actionable guidance.
  //
  // VALIDATES:
  // - Shows defined parameters
  // - Shows set values
  // - Indicates missing mandatory parameters
  // - Clear readiness indicator

  let temp_dir = std::env::temp_dir();
  let source_dir = temp_dir.join( "test_status_source" );
  let archive_path = temp_dir.join( "test_status.json" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &archive_path );

  // Create simple template
  fs::create_dir_all( &source_dir ).expect( "Should create source dir" );
  fs::write( source_dir.join( "template.txt" ), "Name: {{name}}" )
    .expect( "Should write template" );

  // REPL workflow: pack → load → add parameter → status
  let script = format!(
    ".pack input::{} output::{}\n\
     .archive.load path::{}\n\
     .parameter.add name::name mandatory::1\n\
     .status\n\
     exit",
    source_dir.display(),
    archive_path.display(),
    archive_path.display()
  );

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Status workflow should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!( output.status.success(), "Workflow should succeed. stdout: {stdout}, stderr: {stderr}" );

  // Verify status shows parameter state
  assert!( stdout.contains( "name" ) || stdout.contains( "parameter" ), "Should show parameter name" );
  assert!( stdout.contains( "mandatory" ) || stdout.contains( "required" ) || stdout.contains( '1' ), "Should indicate mandatory status" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &archive_path );
}

#[ test ]
fn analyze_provides_comprehensive_overview()
{
  // Test: .analyze command combines all analysis information
  //
  // WHY: Power users want complete picture in one command rather than running
  // multiple analysis commands separately.
  //
  // VALIDATES:
  // - Includes archive metadata (like .info)
  // - Shows parameter information
  // - Displays file statistics
  // - Comprehensive output format

  let temp_dir = std::env::temp_dir();
  let source_dir = temp_dir.join( "test_analyze_source" );
  let archive_path = temp_dir.join( "test_analyze.json" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &archive_path );

  // Create archive with templates
  fs::create_dir_all( &source_dir ).expect( "Should create source dir" );
  fs::write( source_dir.join( "file1.txt" ), "Content {{var1}}" )
    .expect( "Should write file1" );
  fs::write( source_dir.join( "file2.txt" ), "Data {{var2}}" )
    .expect( "Should write file2" );

  // REPL workflow: pack → load → analyze
  let script = format!(
    ".pack input::{} output::{}\n\
     .archive.load path::{}\n\
     .analyze\n\
     exit",
    source_dir.display(),
    archive_path.display(),
    archive_path.display()
  );

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Analyze workflow should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!( output.status.success(), "Workflow should succeed. stdout: {stdout}, stderr: {stderr}" );

  // Verify comprehensive output
  assert!( stdout.contains( "test_analyze_source" ) || stdout.contains( "Archive" ), "Should show archive name" );
  assert!( stdout.contains( '2' ) || stdout.contains( "Files" ) || stdout.contains( "file" ), "Should show file count" );

  // Should mention analysis or summary
  assert!( stdout.contains( "Analysis" ) || stdout.contains( "Summary" ) || stdout.contains( "Overview" ), "Should indicate analysis output" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &archive_path );
}

#[ test ]
fn analysis_commands_require_loaded_archive()
{
  // Test: All analysis commands fail gracefully without loaded archive
  //
  // WHY: Common user error - running analysis before loading archive.
  // All analysis commands should provide consistent error messaging.
  //
  // VALIDATES:
  // - .info requires archive
  // - .discover.parameters requires archive
  // - .status requires archive
  // - .analyze requires archive
  // - Clear error messages

  let commands = vec![ ".info", ".discover.parameters", ".status", ".analyze" ];

  for cmd in commands
  {
    let output = std::process::Command::new( "cargo" )
      .args( [ "run", "--quiet", "--", cmd ] )
      .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
      .output()
      .expect( "Command should execute" );

    let combined = format!(
      "{}{}",
      String::from_utf8_lossy( &output.stdout ),
      String::from_utf8_lossy( &output.stderr )
    );

    assert!(
      !output.status.success() || combined.contains( "ERROR" ) || combined.contains( "No archive" ) || combined.contains( "load" ),
      "{cmd} should fail or error without loaded archive. output: {combined}"
    );
  }
}
