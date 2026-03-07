//! Command Version Consistency Validation
//!
//! # Root Cause
//!
//! During REQ-011 Content-First Display implementation (v1.3.0), we discovered an implicit
//! release step that wasn't explicitly documented: command version synchronization.
//!
//! When the crate version was bumped from 1.2.1 to 1.3.0 due to a breaking change in `.show`
//! command behavior, the `.show` command version was correctly updated to v1.3.0. However,
//! the related `.show.project` command version was initially left at v1.2.0, creating an
//! inconsistency.
//!
//! Historical evidence from CHANGELOG v1.2.0 entry showed both commands were versioned together:
//! "Updated `.show` and `.show.project` to v1.2.0". This established the pattern that related
//! commands should have synchronized versions when released together.
//!
//! # Why Not Caught
//!
//! The 7-stage validation framework (104 checks) successfully validated all code correctness
//! but did NOT include metadata consistency checks. The framework focused on:
//! - Feature implementation
//! - Test coverage
//! - Code quality
//! - Anti-patterns
//! - Impossibility proofs
//! - Rollback resistance
//!
//! But it did NOT validate:
//! - Command version consistency in YAML
//! - Release metadata synchronization
//! - Cross-file version references
//!
//! This was discovered only through manual plan review when the user repeatedly asked to
//! "search for missing steps" - indicating the automated validation wasn't comprehensive enough.
//!
//! # Fix Applied
//!
//! 1. **Immediate**: Updated `.show.project` version from 1.2.0 to 1.3.0 in unilang.commands.yaml
//! 2. **Documentation**: Added CHANGELOG entry documenting command version updates
//! 3. **Prevention**: Created this test to validate command version consistency going forward
//!
//! # Prevention
//!
//! ## Command Versioning Policy
//!
//! This test enforces the following versioning rules:
//!
//! 1. **Breaking Changes**: When a command has breaking changes, its version MUST match the
//!    crate version for that release
//!    - Example: REQ-011 changed `.show` behavior → `.show` version = crate version (1.3.0)
//!
//! 2. **Related Commands**: Commands that are functionally related or deprecated in favor of
//!    each other should have synchronized versions when released together
//!    - Example: `.show` and `.show.project` (deprecated) released together → same version
//!
//! 3. **Unchanged Commands**: Commands that haven't changed can retain their original versions
//!    - Example: `.list`, `.status`, `.count` remain at v1.0.0 even when crate is v1.3.0
//!
//! 4. **Version Consistency**: All command versions in unilang.commands.yaml must be valid
//!    semantic versions and should not exceed the current crate version
//!
//! ## Historical Context
//!
//! - v1.0.0: Initial release (all commands v1.0.0)
//! - v1.2.0: `.show` and `.show.project` both updated to v1.2.0 (location-aware behavior)
//! - v1.3.0: `.show` and `.show.project` both updated to v1.3.0 (content-first display)
//!
//! This test validates these contracts to prevent future inconsistencies.
//!
//! # Pitfall to Avoid
//!
//! **Pitfall**: Assuming command versions are independent of crate versions.
//!
//! **Reality**: Command versions are part of the public API. When users see crate v1.3.0
//! released with "breaking changes to `.show` command", they expect `.show` command version
//! to reflect v1.3.0. Mismatched versions create confusion:
//! - "Why is crate v1.3.0 but `.show.project` still v1.2.0?"
//! - "Did `.show.project` get the breaking changes or not?"
//!
//! **Lesson**: Command versions are release metadata, not just implementation details. They
//! communicate API stability and breaking change boundaries to users. Synchronize them with
//! crate releases when commands are modified together.
//!
//! **Process**: When bumping crate version for breaking command changes:
//! 1. Identify ALL commands affected by the change
//! 2. Update version for each affected command in unilang.commands.yaml
//! 3. Document version updates in CHANGELOG.md
//! 4. This test will validate the consistency

use std::fs;
use std::path::PathBuf;

/// Parse the current crate version from Cargo.toml
fn get_crate_version() -> String
{
  let manifest_path = PathBuf::from( env!( "CARGO_MANIFEST_DIR" ) ).join( "Cargo.toml" );
  let manifest_content = fs::read_to_string( manifest_path )
    .expect( "Failed to read Cargo.toml" );

  for line in manifest_content.lines()
  {
    if line.starts_with( "version" )
    {
      let version = line.split( '=' ).nth( 1 )
        .expect( "Invalid version line in Cargo.toml" )
        .trim()
        .trim_matches( '"' );
      return version.to_string();
    }
  }

  panic!( "Version not found in Cargo.toml" );
}

/// Parse command versions from unilang.commands.yaml
fn get_command_versions() -> Vec< ( String, String ) >
{
  let yaml_path = PathBuf::from( env!( "CARGO_MANIFEST_DIR" ) ).join( "unilang.commands.yaml" );
  let yaml_content = fs::read_to_string( yaml_path )
    .expect( "Failed to read unilang.commands.yaml" );

  let mut commands = Vec::new();
  let mut current_name = None;

  for line in yaml_content.lines()
  {
    let trimmed = line.trim();

    if trimmed.starts_with( "- name:" )
    {
      current_name = Some(
        trimmed.split( ':' ).nth( 1 )
          .expect( "Invalid name line" )
          .trim()
          .trim_matches( '"' )
          .to_string()
      );
    }
    else if trimmed.starts_with( "version:" )
    {
      if let Some( name ) = current_name.take()
      {
        let version = trimmed.split( ':' ).nth( 1 )
          .expect( "Invalid version line" )
          .trim()
          .trim_matches( '"' )
          .to_string();
        commands.push( ( name, version ) );
      }
    }
  }

  commands
}

/// Validate that command versions follow the versioning policy
#[test]
fn test_command_version_consistency()
{
  let crate_version = get_crate_version();
  let commands = get_command_versions();

  println!( "\n=== Command Version Consistency Report ===" );
  println!( "Crate version: {crate_version}" );
  println!( "\nCommands:" );

  for ( name, version ) in &commands
  {
    println!( "  {name} → v{version}" );
  }

  // Validate: No command version should exceed crate version
  for ( name, version ) in &commands
  {
    assert!(
      version <= &crate_version,
      "Command '{name}' version ({version}) exceeds crate version ({crate_version})"
    );
  }

  // Validate: Related commands should have synchronized versions
  // .show and .show.project are functionally related (deprecated pair)
  let show_version = commands.iter()
    .find( |( n, _ )| n == ".show" )
    .map( |( _, v )| v )
    .expect( ".show command must exist" );

  let show_project_version = commands.iter()
    .find( |( n, _ )| n == ".show.project" )
    .map( |( _, v )| v )
    .expect( ".show.project command must exist" );

  assert_eq!(
    show_version, show_project_version,
    ".show and .show.project must have synchronized versions (related deprecated pair)"
  );

  // Validate: All versions are valid semantic versions (basic check)
  for ( name, version ) in &commands
  {
    let parts: Vec< &str > = version.split( '.' ).collect();
    assert!(
      parts.len() >= 2,
      "Command '{name}' version '{version}' is not a valid semantic version (must have at least major.minor)"
    );

    for part in parts
    {
      assert!(
        part.parse::< u32 >().is_ok(),
        "Command '{name}' version '{version}' contains non-numeric component '{part}'"
      );
    }
  }

  println!( "\n✅ All command versions are consistent" );
}

/// Validate current crate version expectations for v1.3.0 release
#[test]
fn test_v1_3_0_release_versions()
{
  let crate_version = get_crate_version();

  // This test validates the specific v1.3.0 release state
  // Update this test when moving to v1.4.0 or later

  if crate_version == "1.3.0"
  {
    let commands = get_command_versions();

    // REQ-011 Content-First Display affected these commands:
    let show_cmd = commands.iter().find( |( n, _ )| n == ".show" )
      .expect( ".show command exists" );
    let show_project_cmd = commands.iter().find( |( n, _ )| n == ".show.project" )
      .expect( ".show.project command exists" );

    assert_eq!(
      show_cmd.1, "1.3.0",
      ".show command must be v1.3.0 (breaking change in REQ-011)"
    );

    assert_eq!(
      show_project_cmd.1, "1.3.0",
      ".show.project command must be v1.3.0 (synchronized with .show)"
    );

    println!( "\n✅ v1.3.0 release: .show and .show.project correctly versioned" );
  }
  else
  {
    println!( "\nℹ️  Skipping v1.3.0-specific validation (current version: {crate_version})" );
  }
}
