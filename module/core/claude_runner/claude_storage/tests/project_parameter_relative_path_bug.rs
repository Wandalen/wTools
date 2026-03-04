//! Bug fix test for relative path resolution in project parameter (Finding #013)
//!
//! ## Root Cause
//!
//! `parse_project_parameter()` at line 641 does not handle relative paths like ".",
//! "..", "~", or "foo/bar". These inputs fall through to the default UUID case,
//! causing them to be treated as literal UUID strings instead of being resolved
//! to filesystem paths. For example, `.show project::.` creates a project lookup
//! for `Uuid(".")` instead of resolving "." to the current working directory.
//!
//! ## Why Not Caught
//!
//! All existing tests used either absolute paths, path-encoded values, or UUIDs.
//! No test exercised special directory references (".", "..", "~") or relative
//! paths without leading slash. The manual testing corner case matrix exposed
//! this gap during exploratory testing.
//!
//! ## Fix Applied
//!
//! Added relative path detection in `parse_project_parameter()` before the UUID
//! default case. Checks for:
//! - "." → Canonicalize to absolute path
//! - ".." → Canonicalize to absolute path
//! - "~" or "~/" prefix → Expand home directory
//! - Paths containing "/" but not absolute → Canonicalize to absolute path
//!
//! ## Prevention
//!
//! Path parameters must handle ALL filesystem path conventions, not just absolute
//! paths. When adding path parsing logic, consider: absolute paths, relative paths
//! (./foo), parent references (..), home expansion (~), path-encoded, and special
//! directory markers (., ..). Test edge cases using the corner case matrix.
//!
//! ## Pitfall
//!
//! Assuming only absolute paths need path treatment. Users commonly use "." for
//! "current directory" and "~" for "home directory". Any string that represents
//! a filesystem location should be treated as a path, not a UUID.

mod common;

/// Test `.show project::.` resolves "." to current working directory (Finding #013)
///
/// ## Purpose
/// Validates that parse_project_parameter handles "." as current directory marker.
///
/// ## Coverage
/// Tests the edge case where user specifies project::. to refer to CWD project.
///
/// ## Validation Strategy
/// Execute .show with project::. from a known project directory. Assert the
/// output shows the project matching CWD, not a literal "." UUID project.
/// Note: Project may not exist in storage - we verify path RESOLUTION, not project existence.
///
/// ## Related Requirements
/// Smart path resolution for all filesystem conventions.
#[ test ]
fn test_show_project_dot_resolves_to_cwd()
{
  // Execute from claude_storage directory which has associated project
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );

  let output = common::claude_storage_cmd()
    .args( [ ".show", "project::." ] )
    .current_dir( manifest_dir )
    .output()
    .expect( "Failed to execute command" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );
  let combined = format!( "{}{}", stdout, stderr );

  // Bug behavior: Shows "Uuid(".")" or "Uuid("projects")" or references "/projects/."
  // Fixed behavior: Shows actual path like "Path("/home/.../claude_storage")"
  //                 or error mentioning the resolved path (not ".")

  // Should NOT contain literal "." as UUID
  let has_dot_uuid = combined.contains( r#"Uuid(".")"# ) ||
    combined.contains( r#"Uuid("projects")"# ) ||
    combined.contains( "/projects/." );

  assert!(
    !has_dot_uuid,
    "Bug: '.' treated as literal UUID instead of resolved to CWD.\n\
    Expected: Path resolved to current directory\n\
    Got stdout: {}\nstderr: {}",
    stdout, stderr
  );

  // Should contain the actual project path when resolved correctly.
  // Either in success output (Path("...")) or in error message (Project not found: ...)
  // The path should be canonicalized (containing manifest_dir, no "." in path)
  let is_resolved = combined.contains( "Path(" ) ||
    combined.contains( manifest_dir ) ||
    combined.contains( "claude_storage" );

  assert!(
    is_resolved,
    "Expected project path to be resolved from '.'\n\
    stdout: {}\nstderr: {}",
    stdout, stderr
  );
}

/// Test `.show project::..` resolves ".." to parent directory (Finding #013)
///
/// ## Purpose
/// Validates that parse_project_parameter handles ".." as parent directory marker.
///
/// ## Coverage
/// Tests the edge case where user specifies project::.. to refer to parent dir.
///
/// ## Validation Strategy
/// Execute .show with project::.. and verify it does not treat ".." as a UUID.
#[ test ]
fn test_show_project_dotdot_resolves_to_parent()
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );

  let output = common::claude_storage_cmd()
    .args( [ ".show", "project::.." ] )
    .current_dir( manifest_dir )
    .output()
    .expect( "Failed to execute command" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  // Bug behavior: Shows "Uuid("..")" or fails strangely
  // Fixed behavior: Resolves to parent directory path

  let has_dotdot_uuid = stdout.contains( r#"Uuid("..")"# ) ||
    stdout.contains( "/projects/.." );

  assert!(
    !has_dotdot_uuid,
    "Bug: '..' treated as literal UUID instead of resolved to parent dir.\n\
    Got stdout: {}\nstderr: {}",
    stdout, stderr
  );
}

/// Test `.show project::~` resolves "~" to home directory (Finding #013)
///
/// ## Purpose
/// Validates that parse_project_parameter handles "~" as home directory marker.
///
/// ## Coverage
/// Tests home directory expansion, a common shell convention.
///
/// ## Validation Strategy
/// Execute .show with project::~ and verify it resolves to home directory.
#[ test ]
fn test_show_project_tilde_resolves_to_home()
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );

  let output = common::claude_storage_cmd()
    .args( [ ".show", "project::~" ] )
    .current_dir( manifest_dir )
    .output()
    .expect( "Failed to execute command" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  // Bug behavior: Shows "Uuid("~")"
  // Fixed behavior: Resolves to home directory path

  let has_tilde_uuid = stdout.contains( r#"Uuid("~")"# ) ||
    stdout.contains( r#"Uuid("projects")"# );

  assert!(
    !has_tilde_uuid,
    "Bug: '~' treated as literal UUID instead of resolved to home dir.\n\
    Got stdout: {}\nstderr: {}",
    stdout, stderr
  );
}

/// Test unit: parse_project_parameter handles relative paths (Finding #013)
///
/// ## Purpose
/// Unit test validating parse_project_parameter correctly identifies relative paths.
///
/// ## Coverage
/// Tests: ".", "..", "~", "~/foo", "./bar", "../baz"
#[ test ]
fn test_parse_project_parameter_relative_paths()
{
  use claude_storage::parse_project_parameter;
  use claude_storage_core::ProjectId;

  // Test "." - should be Path, not Uuid
  let result = parse_project_parameter( "." ).unwrap();
  assert!(
    matches!( result, ProjectId::Path( _ ) ),
    "'.' should resolve to ProjectId::Path, got: {:?}",
    result
  );

  // Test ".." - should be Path, not Uuid
  let result = parse_project_parameter( ".." ).unwrap();
  assert!(
    matches!( result, ProjectId::Path( _ ) ),
    "'..' should resolve to ProjectId::Path, got: {:?}",
    result
  );

  // Test "~" - should be Path, not Uuid
  let result = parse_project_parameter( "~" ).unwrap();
  assert!(
    matches!( result, ProjectId::Path( _ ) ),
    "'~' should resolve to ProjectId::Path, got: {:?}",
    result
  );

  // Test "~/projects" - should be Path, not Uuid
  let result = parse_project_parameter( "~/projects" ).unwrap();
  assert!(
    matches!( result, ProjectId::Path( _ ) ),
    "'~/projects' should resolve to ProjectId::Path, got: {:?}",
    result
  );

  // Test "./subdir" - should be Path, not Uuid
  let result = parse_project_parameter( "./subdir" ).unwrap();
  assert!(
    matches!( result, ProjectId::Path( _ ) ),
    "'./subdir' should resolve to ProjectId::Path, got: {:?}",
    result
  );

  // Test "../parent" - should be Path, not Uuid
  let result = parse_project_parameter( "../parent" ).unwrap();
  assert!(
    matches!( result, ProjectId::Path( _ ) ),
    "'../parent' should resolve to ProjectId::Path, got: {:?}",
    result
  );
}
