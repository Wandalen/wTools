//! Integration tests for cargo_unilang
//!
//! Tests CLI rulebook compliance and functional correctness.

use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

// ============================================================================
// Command Format Tests
// ============================================================================

#[test]
fn test_dot_prefix_help_works()
{
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( "." )
    .assert()
    .success()
    .stdout( predicate::str::contains( "cargo_unilang - Scaffolding and health check tool" ) );
}

#[test]
fn test_dot_help_works()
{
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".help" )
    .assert()
    .success()
    .stdout( predicate::str::contains( "cargo_unilang - Scaffolding and health check tool" ) );
}

#[test]
fn test_dot_new_with_param_value_format()
{
  let temp = assert_fs::TempDir::new().unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::test-cli" )
    .arg( "verbosity::0" )
    .current_dir( &temp )
    .assert()
    .success();

  temp.child( "test-cli/Cargo.toml" ).assert( predicate::path::exists() );
}

#[test]
fn test_invalid_command_without_dot_prefix_fails()
{
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( "new" )
    .arg( "project::test" )
    .assert()
    .failure()
    .code( 2 )
    .stderr( predicate::str::contains( "Unknown command: new" ) );
}

#[test]
fn test_invalid_parameter_format_fails()
{
  let temp = assert_fs::TempDir::new().unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "test-cli" ) // Missing param::value format
    .current_dir( &temp )
    .assert()
    .failure()
    .stderr( predicate::str::contains( "Invalid parameter format" ) );
}

// ============================================================================
// Help System Tests
// ============================================================================

#[test]
fn test_help_via_dot_shows_general_help()
{
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( "." )
    .assert()
    .success()
    .stdout( predicate::str::contains( "USAGE:" ) )
    .stdout( predicate::str::contains( "COMMANDS:" ) )
    .stdout( predicate::str::contains( "EXAMPLES:" ) );
}

#[test]
fn test_help_via_dot_help_same_as_dot()
{
  let dot_output = Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( "." )
    .output()
    .unwrap();

  let dot_help_output = Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".help" )
    .output()
    .unwrap();

  assert_eq!( dot_output.stdout, dot_help_output.stdout );
}

#[test]
fn test_new_help_shows_command_help()
{
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new.help" )
    .assert()
    .success()
    .stdout( predicate::str::contains( "cargo_unilang .new - Create new unilang project" ) )
    .stdout( predicate::str::contains( "PARAMETERS:" ) )
    .stdout( predicate::str::contains( "project::<name>" ) );
}

#[test]
fn test_check_help_shows_command_help()
{
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".check.help" )
    .assert()
    .success()
    .stdout( predicate::str::contains( "cargo_unilang .check - Validate unilang project" ) )
    .stdout( predicate::str::contains( "CHECKS PERFORMED:" ) );
}

#[test]
fn test_no_args_shows_help()
{
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .assert()
    .success()
    .stdout( predicate::str::contains( "cargo_unilang - Scaffolding and health check tool" ) );
}

// ============================================================================
// Verbosity Tests
// ============================================================================

#[test]
fn test_new_verbosity_0_silent()
{
  let temp = assert_fs::TempDir::new().unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::test-cli" )
    .arg( "verbosity::0" )
    .current_dir( &temp )
    .assert()
    .success()
    .stdout( predicate::str::is_empty() );
}

#[test]
fn test_new_verbosity_1_single_line()
{
  let temp = assert_fs::TempDir::new().unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::test-cli" )
    .arg( "verbosity::1" )
    .current_dir( &temp )
    .assert()
    .success()
    .stdout( predicate::str::contains( "Created test-cli/" ) )
    .stdout( predicate::str::contains( "Created test-cli/" ).count( 1 ) ); // Only one line
}

#[test]
fn test_new_verbosity_2_default_concise()
{
  let temp = assert_fs::TempDir::new().unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::test-cli" )
    .arg( "verbosity::2" )
    .current_dir( &temp )
    .assert()
    .success()
    .stdout( predicate::str::contains( "Created project:" ) )
    .stdout( predicate::str::contains( "Cargo.toml" ) )
    .stdout( predicate::str::contains( "You did NOT need to write build.rs" ) );
}

#[test]
fn test_new_verbosity_3_debug()
{
  let temp = assert_fs::TempDir::new().unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::test-cli" )
    .arg( "verbosity::3" )
    .current_dir( &temp )
    .assert()
    .success()
    .stderr( predicate::str::contains( "[INFO]" ) )
    .stderr( predicate::str::contains( "[DEBUG]" ) );
}

#[test]
fn test_check_verbosity_levels()
{
  let temp = assert_fs::TempDir::new().unwrap();
  temp.child( "Cargo.toml" ).write_str( "[dependencies]\nunilang = \"0.32\"\n" ).unwrap();

  // Verbosity 0 - silent
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".check" )
    .arg( "verbosity::0" )
    .current_dir( &temp )
    .assert()
    .success()
    .stdout( predicate::str::is_empty() );

  // Verbosity 1 - single line
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".check" )
    .arg( "verbosity::1" )
    .current_dir( &temp )
    .assert()
    .success()
    .stdout( predicate::str::contains( "All checks passed" ) );
}

// ============================================================================
// Exit Code Tests
// ============================================================================

#[test]
fn test_new_exit_0_on_success()
{
  let temp = assert_fs::TempDir::new().unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::test-project" )
    .arg( "verbosity::0" )
    .current_dir( &temp )
    .assert()
    .code( 0 );
}

#[test]
fn test_new_exit_2_invalid_params()
{
  let temp = assert_fs::TempDir::new().unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::../invalid" )
    .current_dir( &temp )
    .assert()
    .failure()
    .code( 1 ) // Parameter validation errors return 1
    .stderr( predicate::str::contains( "Project name cannot contain path separators" ) );
}

#[test]
fn test_new_exit_3_already_exists()
{
  let temp = assert_fs::TempDir::new().unwrap();

  // Create project first
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::test-project" )
    .arg( "verbosity::0" )
    .current_dir( &temp )
    .assert()
    .success();

  // Try to create again
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::test-project" )
    .current_dir( &temp )
    .assert()
    .failure()
    .code( 1 )
    .stderr( predicate::str::contains( "already exists" ) );
}

#[test]
fn test_check_exit_0_no_issues()
{
  let temp = assert_fs::TempDir::new().unwrap();
  temp.child( "Cargo.toml" ).write_str( "[dependencies]\nunilang = \"0.32\"\n" ).unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".check" )
    .arg( "verbosity::0" )
    .current_dir( &temp )
    .assert()
    .code( 0 );
}

#[test]
fn test_check_exit_1_issues_found()
{
  let temp = assert_fs::TempDir::new().unwrap();
  temp.child( "build.rs" ).write_str( "fn main() { serde_yaml::from_str(); }" ).unwrap();
  temp.child( "Cargo.toml" ).write_str( "[dependencies]\nunilang = \"0.32\"\n" ).unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".check" )
    .arg( "verbosity::0" )
    .current_dir( &temp )
    .assert()
    .code( 1 );
}

#[test]
fn test_check_exit_3_invalid_path()
{
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".check" )
    .arg( "path::/nonexistent/path" )
    .assert()
    .failure()
    .code( 1 )
    .stderr( predicate::str::contains( "does not exist" ) );
}

// ============================================================================
// Input Validation Tests
// ============================================================================

#[test]
fn test_new_validates_project_name_empty()
{
  let temp = assert_fs::TempDir::new().unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::" )
    .current_dir( &temp )
    .assert()
    .failure()
    .stderr( predicate::str::contains( "cannot be empty" ) );
}

#[test]
fn test_new_validates_project_name_path_traversal()
{
  let temp = assert_fs::TempDir::new().unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::../etc" )
    .current_dir( &temp )
    .assert()
    .failure()
    .stderr( predicate::str::contains( "path separators" ) );
}

#[test]
fn test_new_validates_project_name_invalid_chars()
{
  let temp = assert_fs::TempDir::new().unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::my@cli" )
    .current_dir( &temp )
    .assert()
    .failure()
    .stderr( predicate::str::contains( "Invalid character" ) );
}

#[test]
fn test_new_validates_template_name()
{
  let temp = assert_fs::TempDir::new().unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::test" )
    .arg( "template::invalid" )
    .current_dir( &temp )
    .assert()
    .failure()
    .stderr( predicate::str::contains( "Unknown template" ) );
}

#[test]
fn test_new_validates_verbosity()
{
  let temp = assert_fs::TempDir::new().unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::test" )
    .arg( "verbosity::10" )
    .current_dir( &temp )
    .assert()
    .failure()
    .stderr( predicate::str::contains( "Verbosity must be 0-5" ) );
}

#[test]
fn test_check_validates_path_exists()
{
  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".check" )
    .arg( "path::/definitely/does/not/exist" )
    .assert()
    .failure()
    .stderr( predicate::str::contains( "does not exist" ) );
}

// ============================================================================
// Functional Tests
// ============================================================================

#[test]
fn test_new_creates_correct_structure()
{
  let temp = assert_fs::TempDir::new().unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::my-cli" )
    .arg( "verbosity::0" )
    .current_dir( &temp )
    .assert()
    .success();

  // Verify structure
  temp.child( "my-cli/Cargo.toml" ).assert( predicate::path::exists() );
  temp.child( "my-cli/src/main.rs" ).assert( predicate::path::exists() );
  temp.child( "my-cli/commands.yaml" ).assert( predicate::path::exists() );
}

#[test]
fn test_new_does_not_create_build_rs()
{
  let temp = assert_fs::TempDir::new().unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::my-cli" )
    .arg( "verbosity::0" )
    .current_dir( &temp )
    .assert()
    .success();

  // Verify NO build.rs
  temp.child( "my-cli/build.rs" ).assert( predicate::path::missing() );
}

#[test]
fn test_new_includes_warnings_in_cargo_toml()
{
  let temp = assert_fs::TempDir::new().unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::my-cli" )
    .arg( "verbosity::0" )
    .current_dir( &temp )
    .assert()
    .success();

  // Read Cargo.toml
  let cargo_toml = temp.child( "my-cli/Cargo.toml" );
  cargo_toml.assert( predicate::path::exists() );

  let content = std::fs::read_to_string( cargo_toml.path() ).unwrap();
  assert!( content.contains( "Do NOT add these" ) );
  assert!( content.contains( "Do NOT create build.rs" ) );
}

#[test]
fn test_check_detects_custom_build_rs()
{
  let temp = assert_fs::TempDir::new().unwrap();
  temp.child( "build.rs" ).write_str( "fn main() { serde_yaml::from_str(); }" ).unwrap();
  temp.child( "Cargo.toml" ).write_str( "[dependencies]\nunilang = \"0.32\"\n" ).unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".check" )
    .arg( "verbosity::2" )
    .current_dir( &temp )
    .assert()
    .failure()
    .stdout( predicate::str::contains( "Custom build.rs found" ) );
}

#[test]
fn test_check_detects_duplicate_dependencies()
{
  let temp = assert_fs::TempDir::new().unwrap();
  temp.child( "Cargo.toml" ).write_str(
r#"[dependencies]
unilang = "0.32"
serde_yaml = "0.9"
"#
  ).unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".check" )
    .arg( "verbosity::2" )
    .current_dir( &temp )
    .assert()
    .failure()
    .stdout( predicate::str::contains( "Duplicate dependencies" ) )
    .stdout( predicate::str::contains( "serde_yaml" ) );
}

#[test]
fn test_check_detects_deprecated_api()
{
  let temp = assert_fs::TempDir::new().unwrap();
  let src = temp.child( "src" );
  src.create_dir_all().unwrap();
  src.child( "main.rs" ).write_str(
r#"use unilang::prelude::*;
fn main() {
  let registry = CommandRegistry::new();
}
"#
  ).unwrap();
  temp.child( "Cargo.toml" ).write_str( "[dependencies]\nunilang = \"0.32\"\n" ).unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".check" )
    .arg( "verbosity::2" )
    .current_dir( &temp )
    .assert()
    .failure()
    .stdout( predicate::str::contains( "Deprecated API" ) );
}

#[test]
fn test_check_passes_on_clean_project()
{
  let temp = assert_fs::TempDir::new().unwrap();
  let src = temp.child( "src" );
  src.create_dir_all().unwrap();
  src.child( "main.rs" ).write_str(
r#"use unilang::prelude::*;
fn main() {
  let registry = StaticCommandRegistry::from_commands(&STATIC_COMMANDS);
}
"#
  ).unwrap();
  temp.child( "Cargo.toml" ).write_str( "[dependencies]\nunilang = \"0.32\"\n" ).unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".check" )
    .arg( "verbosity::1" )
    .current_dir( &temp )
    .assert()
    .success()
    .stdout( predicate::str::contains( "All checks passed" ) );
}

#[test]
fn test_new_with_full_template()
{
  let temp = assert_fs::TempDir::new().unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::my-api" )
    .arg( "template::full" )
    .arg( "verbosity::0" )
    .current_dir( &temp )
    .assert()
    .success();

  // Verify main.rs contains full-featured code
  let main_rs = temp.child( "my-api/src/main.rs" );
  let content = std::fs::read_to_string( main_rs.path() ).unwrap();
  assert!( content.contains( "process::exit" ) );
}

#[test]
fn test_new_with_author_and_license()
{
  let temp = assert_fs::TempDir::new().unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".new" )
    .arg( "project::my-cli" )
    .arg( "author::TestAuthor" )
    .arg( "license::Apache-2.0" )
    .arg( "verbosity::0" )
    .current_dir( &temp )
    .assert()
    .success();

  // Verify Cargo.toml contains author and license
  let cargo_toml = temp.child( "my-cli/Cargo.toml" );
  let content = std::fs::read_to_string( cargo_toml.path() ).unwrap();
  assert!( content.contains( "authors = [ \"TestAuthor\" ]" ) );
  assert!( content.contains( "license = \"Apache-2.0\"" ) );
}

#[test]
fn test_check_ignores_comments_in_cargo_toml()
{
  let temp = assert_fs::TempDir::new().unwrap();
  temp.child( "Cargo.toml" ).write_str(
r#"[dependencies]
unilang = "0.32"

# ⚠️  IMPORTANT: Do NOT add these - unilang already provides them:
# ❌ serde_yaml (via yaml_parser feature)
# ❌ walkdir (via multi_file feature)
"#
  ).unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".check" )
    .arg( "verbosity::1" )
    .current_dir( &temp )
    .assert()
    .success()
    .stdout( predicate::str::contains( "All checks passed" ) );
}

#[test]
fn test_check_multiple_issues()
{
  let temp = assert_fs::TempDir::new().unwrap();
  temp.child( "build.rs" ).write_str( "fn main() { serde_yaml::from_str(); }" ).unwrap();
  temp.child( "Cargo.toml" ).write_str(
r#"[dependencies]
unilang = "0.32"
serde_yaml = "0.9"
walkdir = "2.0"
"#
  ).unwrap();

  Command::cargo_bin( "cargo_unilang" )
    .unwrap()
    .arg( ".check" )
    .arg( "verbosity::2" )
    .current_dir( &temp )
    .assert()
    .failure()
    .stdout( predicate::str::contains( "Custom build.rs" ) )
    .stdout( predicate::str::contains( "Duplicate dependencies" ) )
    .stdout( predicate::str::contains( "serde_yaml, walkdir" ) );
}
