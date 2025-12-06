//! Tests for dual-pattern config directory support (. and -)
//!
//! # Design Rationale
//!
//! This feature enables temporary configuration overrides that are automatically gitignored,
//! allowing developers to test different settings without polluting the repository.
//!
//! **Use Case:** Developer wants to test with `verbosity=3` without committing that change.
//! Solution: Create `-wplan/config.yaml` with verbosity=3, which overrides `.wplan/config.yaml`.
//!
//! # Supported Patterns
//!
//! - `.{app}/config.yaml` - Permanent configuration (committed to repo)
//! - `-{app}/config.yaml` - Temporary configuration (gitignored via `-*` pattern)
//!
//! # Priority Algorithm
//!
//! Priority = `(directory_depth, pattern_type)` where:
//! 1. **Directory depth takes absolute precedence** - Current dir always beats parent dir
//! 2. **Pattern type is secondary** - Within same dir: temporary (`-`) beats permanent (`.`)
//! 3. **Rationale:** Directory locality matters more than temp vs perm distinction
//!
//! # Implementation Decisions
//!
//! **Why depth tracking?** Index-based detection breaks when multiple configs exist in same
//! directory. Using explicit depth (0=current, 1=parent) ensures correct `ConfigSource` labels.
//!
//! **Why atomic modify prefers temp?** If developer has temp config, they're actively testing -
//! modifications should go to temp file, not permanent config.
//!
//! **Why create defaults to permanent?** Creating temp config requires explicit intent.
//! Default behavior should be permanent (committed) configs.
//!
//! # Known Pitfalls
//!
//! 1. **`TempDir` cleanup in tests:** Tests must `cd /tmp` before `TempDir` drops, otherwise
//!    subsequent tests fail with "No such file or directory" when trying to restore cwd.
//!
//! 2. **Serial test execution:** Tests that change `env::current_dir()` must use `#[serial]`
//!    to prevent race conditions between parallel test threads.
//!
//! 3. **`ConfigSource` detection bug:** Using `index == 0` to detect `LocalCurrent` breaks with
//!    dual patterns. Must use `depth == 0` instead.

use config_hierarchy::{ ConfigPaths, ConfigManager, ConfigDefaults, ConfigValidator };
use std::{ collections::HashMap, fs, env };
use serde_json::Value as JsonValue;
use tempfile::TempDir;
use serial_test::serial;

// Test fixtures
struct TestApp;

impl ConfigPaths for TestApp
{
  fn app_name() -> &'static str { "testapp" }
}

struct TestDefaults;

impl ConfigDefaults for TestDefaults
{
  fn get_defaults() -> HashMap< String, JsonValue >
  {
    let mut defaults = HashMap::new();
    defaults.insert( "verbosity".into(), JsonValue::Number( 1.into() ) );
    defaults
  }

  fn get_parameter_names() -> Vec< &'static str >
  {
    vec![ "verbosity" ]
  }
}

struct TestValidator;

impl ConfigValidator for TestValidator
{
  fn validate_parameter( _param_name : &str, _value : &JsonValue )
    -> Result< (), config_hierarchy::ValidationError >
  {
    Ok( () )
  }

  fn validate_all( _config : &HashMap< String, ( JsonValue, config_hierarchy::ConfigSource ) > )
    -> Vec< config_hierarchy::ValidationError >
  {
    Vec::new()
  }
}

type TestConfig = ConfigManager< TestDefaults, TestApp, TestValidator >;

/// Verifies temporary config overrides permanent config within same directory.
///
/// **What it tests:** When both `-testapp/` and `.testapp/` exist in same directory,
/// the temporary config value should win.
///
/// **Why it matters:** This is the core dual-pattern behavior - temp must override perm
/// to enable local testing without committing changes.
///
/// **Edge case:** Both configs exist with different values - verifies `pattern_type` priority.
#[ test ]
#[ serial ]
fn test_temp_overrides_perm_same_directory()
{
  let temp_dir = TempDir::new().expect( "Failed to create temp dir" );
  env::set_current_dir( temp_dir.path() ).expect( "Failed to change dir" );

  // Create permanent config with verbosity=2
  let perm_dir = temp_dir.path().join( ".testapp" );
  fs::create_dir( &perm_dir ).expect( "Failed to create perm dir" );
  fs::write(
    perm_dir.join( "config.yaml" ),
    "metadata:\n  version: '1.0'\nparameters:\n  verbosity: 2\n"
  ).expect( "Failed to write perm config" );

  // Create temporary config with verbosity=0
  let temp_dir_path = temp_dir.path().join( "-testapp" );
  fs::create_dir( &temp_dir_path ).expect( "Failed to create temp dir" );
  fs::write(
    temp_dir_path.join( "config.yaml" ),
    "metadata:\n  version: '1.0'\nparameters:\n  verbosity: 0\n"
  ).expect( "Failed to write temp config" );

  // Resolve config
  let config = TestConfig::resolve_all_config( &HashMap::new() );
  let ( value, source ) = config.get( "verbosity" ).expect( "verbosity should exist" );

  // Should use temp value (0), not perm value (2)
  assert_eq!( value.as_i64(), Some( 0 ), "Temp config should override perm config" );

  // Should be labeled as LocalCurrent (not LocalParent)
  assert!(
    matches!( source, config_hierarchy::ConfigSource::LocalCurrent( _ ) ),
    "Source should be LocalCurrent"
  );

  // Cleanup - change to /tmp before TempDir drops
  env::set_current_dir( "/tmp" ).expect( "Failed to change to /tmp" );
}

/// Verifies directory depth takes absolute precedence over pattern type.
///
/// **What it tests:** Current directory's permanent config (`.testapp/`) beats
/// parent directory's temporary config (`-testapp/`).
///
/// **Why it matters:** This validates the core priority rule - directory locality
/// matters more than temp vs perm. Without this, parent `-testapp/` would incorrectly
/// override current `.testapp/`.
///
/// **Design decision:** We chose `(depth, pattern)` ordering because configs closer
/// to the working directory are more contextually relevant than distant temp configs.
#[ test ]
#[ serial ]
fn test_current_perm_overrides_parent_temp()
{
  let temp_dir = TempDir::new().expect( "Failed to create temp dir" );
  let child_dir = temp_dir.path().join( "child" );
  fs::create_dir( &child_dir ).expect( "Failed to create child dir" );
  env::set_current_dir( &child_dir ).expect( "Failed to change dir" );

  // Parent: temporary config with verbosity=3
  let parent_temp = temp_dir.path().join( "-testapp" );
  fs::create_dir( &parent_temp ).expect( "Failed to create parent temp dir" );
  fs::write(
    parent_temp.join( "config.yaml" ),
    "metadata:\n  version: '1.0'\nparameters:\n  verbosity: 3\n"
  ).expect( "Failed to write parent temp config" );

  // Current: permanent config with verbosity=1
  let child_perm = child_dir.join( ".testapp" );
  fs::create_dir( &child_perm ).expect( "Failed to create child perm dir" );
  fs::write(
    child_perm.join( "config.yaml" ),
    "metadata:\n  version: '1.0'\nparameters:\n  verbosity: 1\n"
  ).expect( "Failed to write child perm config" );

  // Resolve
  let config = TestConfig::resolve_all_config( &HashMap::new() );
  let ( value, source ) = config.get( "verbosity" ).expect( "verbosity should exist" );

  // Current perm (1) beats parent temp (3)
  assert_eq!( value.as_i64(), Some( 1 ), "Current perm should override parent temp" );
  assert!(
    matches!( source, config_hierarchy::ConfigSource::LocalCurrent( _ ) ),
    "Source should be LocalCurrent"
  );

  // Cleanup - change to /tmp before TempDir drops
  env::set_current_dir( "/tmp" ).expect( "Failed to change to /tmp" );
}

/// Verifies path discovery returns configs in correct priority order.
///
/// **What it tests:** When all 4 config combinations exist (current temp, current perm,
/// parent temp, parent perm), discovery returns them in correct priority order.
///
/// **Why it matters:** Downstream code relies on discovery order for priority resolution.
/// Wrong order = wrong config values at runtime.
///
/// **Implementation detail:** Uses path string matching to verify order since `PathBuf`
/// doesn't implement comparison. Checks for "child" substring to distinguish current
/// from parent directories.
#[ test ]
#[ serial ]
fn test_discovery_order()
{
  let temp_dir = TempDir::new().expect( "Failed to create temp dir" );
  let child_dir = temp_dir.path().join( "child" );
  fs::create_dir( &child_dir ).expect( "Failed to create child dir" );
  env::set_current_dir( &child_dir ).expect( "Failed to change dir" );

  // Create all 4 combinations
  let current_temp = child_dir.join( "-testapp" );
  fs::create_dir( &current_temp ).expect( "Failed to create current temp" );
  fs::write(
    current_temp.join( "config.yaml" ),
    "metadata:\n  version: '1.0'\nparameters: {}\n"
  ).expect( "Failed to write current temp config" );

  let current_perm = child_dir.join( ".testapp" );
  fs::create_dir( &current_perm ).expect( "Failed to create current perm" );
  fs::write(
    current_perm.join( "config.yaml" ),
    "metadata:\n  version: '1.0'\nparameters: {}\n"
  ).expect( "Failed to write current perm config" );

  let parent_temp = temp_dir.path().join( "-testapp" );
  fs::create_dir( &parent_temp ).expect( "Failed to create parent temp" );
  fs::write(
    parent_temp.join( "config.yaml" ),
    "metadata:\n  version: '1.0'\nparameters: {}\n"
  ).expect( "Failed to write parent temp config" );

  let parent_perm = temp_dir.path().join( ".testapp" );
  fs::create_dir( &parent_perm ).expect( "Failed to create parent perm" );
  fs::write(
    parent_perm.join( "config.yaml" ),
    "metadata:\n  version: '1.0'\nparameters: {}\n"
  ).expect( "Failed to write parent perm config" );

  // Discover
  let configs = config_hierarchy::discover_local_configs::< TestApp >();

  // Should find all 4 in correct order
  assert_eq!( configs.len(), 4, "Should discover all 4 configs" );

  let paths : Vec< _ > = configs.iter().map( | p | p.to_string_lossy().to_string() ).collect();

  // Order: current temp, current perm, parent temp, parent perm
  assert!( paths[ 0 ].contains( "child" ) && paths[ 0 ].contains( "-testapp" ), "First should be child/-testapp" );
  assert!( paths[ 1 ].contains( "child" ) && paths[ 1 ].contains( ".testapp" ), "Second should be child/.testapp" );
  assert!( paths[ 2 ].contains( "-testapp" ) && !paths[ 2 ].contains( "child" ), "Third should be parent -testapp" );
  assert!( paths[ 3 ].contains( ".testapp" ) && !paths[ 3 ].contains( "child" ), "Fourth should be parent .testapp" );

  // Cleanup - change to /tmp before TempDir drops
  env::set_current_dir( "/tmp" ).expect( "Failed to change to /tmp" );
}

/// Verifies atomic modify operation prefers temporary config when both exist.
///
/// **What it tests:** When both `-testapp/` and `.testapp/` exist, calling
/// `save_local_config()` modifies the temporary config, leaving permanent unchanged.
///
/// **Why it matters:** If developer has temp config, they're actively testing - we should
/// modify their temp file, not their committed permanent config. Prevents accidental
/// commits of test values.
///
/// **Design rationale:** Temp existence signals active testing session. Modifications
/// should stay in temp layer until developer explicitly moves them to permanent.
///
/// **Pitfall avoided:** Without this behavior, save operations would unpredictably modify
/// whichever config was "first" alphabetically or by creation time.
#[ test ]
#[ serial ]
fn test_atomic_modify_prefers_temp()
{
  let temp_dir = TempDir::new().expect( "Failed to create temp dir" );
  env::set_current_dir( temp_dir.path() ).expect( "Failed to change dir" );

  // Create both configs
  let temp_dir_path = temp_dir.path().join( "-testapp" );
  fs::create_dir( &temp_dir_path ).expect( "Failed to create temp dir" );
  fs::write(
    temp_dir_path.join( "config.yaml" ),
    "metadata:\n  version: '1.0'\nparameters:\n  verbosity: 0\n"
  ).expect( "Failed to write temp config" );

  let perm_dir = temp_dir.path().join( ".testapp" );
  fs::create_dir( &perm_dir ).expect( "Failed to create perm dir" );
  fs::write(
    perm_dir.join( "config.yaml" ),
    "metadata:\n  version: '1.0'\nparameters:\n  verbosity: 2\n"
  ).expect( "Failed to write perm config" );

  // Modify config
  let mut new_config = HashMap::new();
  new_config.insert( "verbosity".to_string(), JsonValue::Number( 1.into() ) );

  TestConfig::save_local_config( &new_config ).expect( "Failed to save config" );

  // Should modify temp (not perm)
  let temp_content = fs::read_to_string( temp_dir_path.join( "config.yaml" ) )
    .expect( "Failed to read temp config" );
  assert!( temp_content.contains( "verbosity: 1" ), "Temp config should be modified" );

  // Perm should be unchanged
  let perm_content = fs::read_to_string( perm_dir.join( "config.yaml" ) )
    .expect( "Failed to read perm config" );
  assert!( perm_content.contains( "verbosity: 2" ), "Perm config should be unchanged" );

  // Cleanup - change to /tmp before TempDir drops
  env::set_current_dir( "/tmp" ).expect( "Failed to change to /tmp" );
}

/// Verifies new config creation defaults to permanent (not temporary).
///
/// **What it tests:** When neither `-testapp/` nor `.testapp/` exists, calling
/// `save_local_config()` creates `.testapp/config.yaml` (permanent).
///
/// **Why it matters:** Default behavior should favor committed configs. Creating temp
/// config requires explicit intent (developer must create `-testapp/` directory first).
///
/// **Design rationale:** Temporary configs are for active testing sessions. New projects
/// should start with permanent configs that get committed to version control.
///
/// **Alternative considered:** Could default to temp, but that would require developers
/// to manually migrate every new config to permanent. Current approach minimizes friction.
#[ test ]
#[ serial ]
fn test_create_defaults_to_permanent()
{
  let temp_dir = TempDir::new().expect( "Failed to create temp dir" );
  env::set_current_dir( temp_dir.path() ).expect( "Failed to change dir" );

  // No configs exist
  let mut new_config = HashMap::new();
  new_config.insert( "verbosity".to_string(), JsonValue::Number( 1.into() ) );

  TestConfig::save_local_config( &new_config ).expect( "Failed to save config" );

  // Should create permanent (not temp)
  let perm_dir = temp_dir.path().join( ".testapp" );
  assert!( perm_dir.exists(), "Perm dir should be created" );
  assert!( perm_dir.join( "config.yaml" ).exists(), "Perm config should exist" );

  // Temp should not exist
  let temp_dir_path = temp_dir.path().join( "-testapp" );
  assert!( !temp_dir_path.exists(), "Temp dir should not be created" );

  // Cleanup - change to /tmp before TempDir drops
  env::set_current_dir( "/tmp" ).expect( "Failed to change to /tmp" );
}

#[ test ]
#[ serial ]
fn test_backward_compatibility_dot_prefix_only()
{
  let temp_dir = TempDir::new().expect( "Failed to create temp dir" );
  env::set_current_dir( temp_dir.path() ).expect( "Failed to change dir" );

  // Create only permanent config (existing behavior)
  let perm_dir = temp_dir.path().join( ".testapp" );
  fs::create_dir( &perm_dir ).expect( "Failed to create perm dir" );
  fs::write(
    perm_dir.join( "config.yaml" ),
    "metadata:\n  version: '1.0'\nparameters:\n  verbosity: 2\n"
  ).expect( "Failed to write perm config" );

  // Resolve
  let config = TestConfig::resolve_all_config( &HashMap::new() );
  let ( value, _ ) = config.get( "verbosity" ).expect( "verbosity should exist" );

  // Should work exactly as before
  assert_eq!( value.as_i64(), Some( 2 ), "Backward compatibility maintained" );

  // Cleanup - change to /tmp before TempDir drops
  env::set_current_dir( "/tmp" ).expect( "Failed to change to /tmp" );
}
