//! Integration tests for materialization commands
//!
//! Tests FR6: Template Materialization - .materialize and .unpack commands
//!
//! ## Why These Tests Exist
//!
//! Materialization is the core purpose of genfile - transforming template archives
//! into actual files with parameter substitution. These tests ensure:
//! 1. Templates render correctly with parameter values
//! 2. File structure preserved from archive to output
//! 3. Mandatory parameters validated before materialization
//! 4. Dry run previews work without creating files
//!
//! ## Test Approach
//!
//! Uses REPL mode (piping commands) to test stateful workflow:
//! 1. Create/load archive with template files
//! 2. Set parameter values
//! 3. Materialize to destination
//!
//! This mirrors real user workflow from quick start example (lib.rs:16-18).
//!
//! ## Implementation Lessons
//!
//! **Destination Parameter Type (Critical):**
//! The `.materialize` command uses `Kind::Path` (not `Kind::Directory`) for the
//! destination parameter. WHY: Users expect materialization to create the output
//! directory if it doesn't exist. Using `Kind::Directory` causes validation errors
//! when the path doesn't exist yet, breaking the natural workflow.
//!
//! **Mandatory Parameter Validation:**
//! Validation must occur BEFORE calling `archive.materialize()` to prevent partial
//! output. Use `archive.values.as_ref().is_none_or(|v| !v.has_value(p))` pattern
//! to check if mandatory parameters have values set.

use std::fs;

// FR6: Template Materialization Tests

#[ test ]
fn materialize_renders_templates_with_parameters()
{
  // Test: Basic materialization workflow - pack → load → set values → materialize
  //
  // WHY: Validates the documented quick start workflow actually works end-to-end.
  // This is the primary use case for genfile.
  //
  // VALIDATES:
  // - Template variable substitution via Handlebars
  // - File creation in destination directory
  // - Parameter values correctly applied
  // - Archive state persistence across REPL commands

  let temp_dir = std::env::temp_dir();
  let source_dir = temp_dir.join( "test_materialize_source" );
  let archive_path = temp_dir.join( "test_materialize_archive.json" );
  let destination = temp_dir.join( "test_materialize_output" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &archive_path );
  let _ = fs::remove_dir_all( &destination );

  // Create source template directory
  fs::create_dir_all( &source_dir ).expect( "Should create source dir" );
  fs::write(
    source_dir.join( "readme.md" ),
    "# {{project_name}}\n\nCreated by {{author}}"
  ).expect( "Should write template file" );
  fs::write(
    source_dir.join( "config.toml" ),
    "name = \"{{project_name}}\"\nversion = \"{{version}}\""
  ).expect( "Should write config template" );

  // REPL workflow: pack → set values → materialize
  let script = format!(
    ".pack input::{} output::{}\n\
     .archive.load path::{}\n\
     .parameter.add name::project_name mandatory::1\n\
     .parameter.add name::author mandatory::1\n\
     .parameter.add name::version mandatory::0\n\
     .value.set name::project_name value::\"my-project\"\n\
     .value.set name::author value::\"Test User\"\n\
     .value.set name::version value::\"1.0.0\"\n\
     .materialize destination::{}\n\
     exit",
    source_dir.display(),
    archive_path.display(),
    archive_path.display(),
    destination.display()
  );

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Materialize workflow should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!( output.status.success(), "Workflow should succeed. stdout: {stdout}, stderr: {stderr}" );
  assert!( stdout.contains( "Materialized" ) || stdout.contains( "Created" ), "Should show success message" );

  // Verify output files exist and contain rendered content
  let readme_path = destination.join( "readme.md" );
  let config_path = destination.join( "config.toml" );

  assert!( readme_path.exists(), "readme.md should be created" );
  assert!( config_path.exists(), "config.toml should be created" );

  let readme_content = fs::read_to_string( &readme_path ).expect( "Should read readme" );
  let config_content = fs::read_to_string( &config_path ).expect( "Should read config" );

  // Verify templates rendered (no {{}} left, values substituted)
  assert!( readme_content.contains( "my-project" ), "Should substitute project_name in readme" );
  assert!( readme_content.contains( "Test User" ), "Should substitute author in readme" );
  assert!( !readme_content.contains( "{{" ), "Should not contain unreplaced variables in readme" );

  assert!( config_content.contains( "my-project" ), "Should substitute project_name in config" );
  assert!( config_content.contains( "1.0.0" ), "Should substitute version in config" );
  assert!( !config_content.contains( "{{" ), "Should not contain unreplaced variables in config" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &archive_path );
  let _ = fs::remove_dir_all( &destination );
}

#[ test ]
fn materialize_fails_without_mandatory_parameters()
{
  // Test: Materialization must validate mandatory parameters before rendering
  //
  // WHY: Prevents partial/broken output if user forgets to set required values.
  // Critical for maintaining output quality.
  //
  // EXPECTATION: Clear error message listing missing mandatory parameters

  let temp_dir = std::env::temp_dir();
  let source_dir = temp_dir.join( "test_materialize_mandatory_source" );
  let archive_path = temp_dir.join( "test_materialize_mandatory.json" );
  let destination = temp_dir.join( "test_materialize_mandatory_output" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &archive_path );
  let _ = fs::remove_dir_all( &destination );

  // Create template with mandatory parameter
  fs::create_dir_all( &source_dir ).expect( "Should create source dir" );
  fs::write( source_dir.join( "file.txt" ), "Value: {{mandatory_param}}" )
    .expect( "Should write template" );

  // Workflow: pack → add mandatory param → materialize WITHOUT setting value
  let script = format!(
    ".pack input::{} output::{}\n\
     .archive.load path::{}\n\
     .parameter.add name::mandatory_param mandatory::1\n\
     .materialize destination::{}\n\
     exit",
    source_dir.display(),
    archive_path.display(),
    archive_path.display(),
    destination.display()
  );

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Command should execute" );

  let combined = format!( "{}{}", String::from_utf8_lossy( &output.stdout ), String::from_utf8_lossy( &output.stderr ) );

  // Should fail with validation error
  assert!(
    !output.status.success() || combined.contains( "ERROR" ) || combined.contains( "mandatory" ),
    "Should fail or error for missing mandatory parameter. output: {combined}"
  );

  // Destination should NOT be created for failed materialize
  assert!( !destination.exists(), "Destination should not exist after failed materialize" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &archive_path );
}

#[ test ]
fn materialize_dry_run_preview()
{
  // Test: Dry run shows what would be done without creating files
  //
  // WHY: Safety feature - users can preview output before committing.
  // Prevents accidental overwrites of existing files.
  //
  // CRITICAL: No files should be created in destination directory

  let temp_dir = std::env::temp_dir();
  let source_dir = temp_dir.join( "test_materialize_dry_source" );
  let archive_path = temp_dir.join( "test_materialize_dry.json" );
  let destination = temp_dir.join( "test_materialize_dry_output" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &archive_path );
  let _ = fs::remove_dir_all( &destination );

  // Create simple template
  fs::create_dir_all( &source_dir ).expect( "Should create source dir" );
  fs::write( source_dir.join( "test.txt" ), "Hello {{name}}" )
    .expect( "Should write template" );

  // Workflow with dry::1
  let script = format!(
    ".pack input::{} output::{}\n\
     .archive.load path::{}\n\
     .value.set name::name value::\"World\"\n\
     .materialize destination::{} dry::1\n\
     exit",
    source_dir.display(),
    archive_path.display(),
    archive_path.display(),
    destination.display()
  );

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Dry run should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );

  assert!( output.status.success(), "Dry run should succeed" );
  assert!( stdout.contains( "Dry run" ) || stdout.contains( "Would" ), "Should indicate dry run mode" );

  // CRITICAL: No files should be created
  assert!( !destination.exists(), "Destination should NOT exist after dry run" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &archive_path );
}

#[ test ]
fn materialize_without_archive_returns_error()
{
  // Test: Materialize requires loaded archive in REPL state
  //
  // WHY: Common user error - running materialize before loading archive.
  // Must provide clear, actionable error message.

  let temp_dir = std::env::temp_dir();
  let destination = temp_dir.join( "test_materialize_no_archive" );

  // Clean up
  let _ = fs::remove_dir_all( &destination );

  // Try to materialize without loading archive first
  let output = std::process::Command::new( "cargo" )
    .args( [
      "run", "--quiet", "--",
      ".materialize",
      &format!( "destination::{}", destination.display() ),
    ] )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Command should execute" );

  assert!( !output.status.success(), "Should fail without loaded archive" );

  let combined = format!(
    "{}{}",
    String::from_utf8_lossy( &output.stdout ),
    String::from_utf8_lossy( &output.stderr )
  );

  assert!(
    combined.contains( "No archive" ) || combined.contains( "ERROR" ) || combined.contains( "load" ),
    "Should show clear error about missing archive"
  );

  // Clean up
  let _ = fs::remove_dir_all( &destination );
}

// FR6: Unpack Tests - Raw extraction without template rendering

#[ test ]
fn unpack_preserves_template_variables()
{
  // Test: Unpack writes raw template files WITHOUT parameter substitution
  //
  // WHY: This is the CRITICAL difference between .unpack and .materialize.
  // Users need .unpack to get the raw template files with {{}} placeholders intact,
  // for editing templates or distributing them to others.
  //
  // VALIDATES:
  // - Template variables {{}} are NOT replaced
  // - Files created in destination directory
  // - File content matches archive content exactly (no rendering)
  //
  // CRITICAL VERIFICATION:
  // Output files MUST contain {{variable}} placeholders, NOT substituted values

  let temp_dir = std::env::temp_dir();
  let source_dir = temp_dir.join( "test_unpack_source" );
  let archive_path = temp_dir.join( "test_unpack_archive.json" );
  let destination = temp_dir.join( "test_unpack_output" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &archive_path );
  let _ = fs::remove_dir_all( &destination );

  // Create source template directory with template variables
  fs::create_dir_all( &source_dir ).expect( "Should create source dir" );
  fs::write(
    source_dir.join( "template.txt" ),
    "Project: {{project_name}}\nAuthor: {{author}}"
  ).expect( "Should write template file" );

  // REPL workflow: pack → load → unpack (NO value setting)
  let script = format!(
    ".pack input::{} output::{}\n\
     .archive.load path::{}\n\
     .unpack destination::{}\n\
     exit",
    source_dir.display(),
    archive_path.display(),
    archive_path.display(),
    destination.display()
  );

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Unpack workflow should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!( output.status.success(), "Workflow should succeed. stdout: {stdout}, stderr: {stderr}" );
  assert!( stdout.contains( "Unpacked" ) || stdout.contains( "files" ), "Should show success message" );

  // Verify output file exists
  let template_path = destination.join( "template.txt" );
  assert!( template_path.exists(), "template.txt should be created" );

  let content = fs::read_to_string( &template_path ).expect( "Should read unpacked file" );

  // CRITICAL: Template variables must be preserved, NOT substituted
  assert!( content.contains( "{{project_name}}" ), "Should preserve {{project_name}} placeholder" );
  assert!( content.contains( "{{author}}" ), "Should preserve {{author}} placeholder" );
  assert!( !content.contains( "my-project" ), "Should NOT contain substituted values" );
  assert!( !content.contains( "Test User" ), "Should NOT contain substituted values" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &archive_path );
  let _ = fs::remove_dir_all( &destination );
}

#[ test ]
fn unpack_dry_run_preview()
{
  // Test: Dry run shows what would be unpacked without creating files
  //
  // WHY: Consistent with other commands - users should be able to preview
  // unpack operation before committing to filesystem changes.
  //
  // CRITICAL: No files should be created in destination directory

  let temp_dir = std::env::temp_dir();
  let source_dir = temp_dir.join( "test_unpack_dry_source" );
  let archive_path = temp_dir.join( "test_unpack_dry.json" );
  let destination = temp_dir.join( "test_unpack_dry_output" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &archive_path );
  let _ = fs::remove_dir_all( &destination );

  // Create simple template
  fs::create_dir_all( &source_dir ).expect( "Should create source dir" );
  fs::write( source_dir.join( "file.txt" ), "Hello {{name}}" )
    .expect( "Should write template" );

  // Workflow with dry::1
  let script = format!(
    ".pack input::{} output::{}\n\
     .archive.load path::{}\n\
     .unpack destination::{} dry::1\n\
     exit",
    source_dir.display(),
    archive_path.display(),
    archive_path.display(),
    destination.display()
  );

  let output = std::process::Command::new( "sh" )
    .arg( "-c" )
    .arg( format!( "echo '{script}' | cargo run --quiet 2>&1" ) )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Dry run should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );

  assert!( output.status.success(), "Dry run should succeed" );
  assert!( stdout.contains( "Dry run" ) || stdout.contains( "Would" ), "Should indicate dry run mode" );

  // CRITICAL: No files should be created
  assert!( !destination.exists(), "Destination should NOT exist after dry run" );

  // Clean up
  let _ = fs::remove_dir_all( &source_dir );
  let _ = fs::remove_file( &archive_path );
}

#[ test ]
fn unpack_without_archive_returns_error()
{
  // Test: Unpack requires loaded archive in REPL state
  //
  // WHY: Common user error - running unpack before loading archive.
  // Must provide clear, actionable error message.

  let temp_dir = std::env::temp_dir();
  let destination = temp_dir.join( "test_unpack_no_archive" );

  // Clean up
  let _ = fs::remove_dir_all( &destination );

  // Try to unpack without loading archive first
  let output = std::process::Command::new( "cargo" )
    .args( [
      "run", "--quiet", "--",
      ".unpack",
      &format!( "destination::{}", destination.display() ),
    ] )
    .current_dir( "/home/user1/pro/lib/wTools/module/core/genfile" )
    .output()
    .expect( "Command should execute" );

  assert!( !output.status.success(), "Should fail without loaded archive" );

  let combined = format!(
    "{}{}",
    String::from_utf8_lossy( &output.stdout ),
    String::from_utf8_lossy( &output.stderr )
  );

  assert!(
    combined.contains( "No archive" ) || combined.contains( "ERROR" ) || combined.contains( "load" ),
    "Should show clear error about missing archive"
  );

  // Clean up
  let _ = fs::remove_dir_all( &destination );
}
