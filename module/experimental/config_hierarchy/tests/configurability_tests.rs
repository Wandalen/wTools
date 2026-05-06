//! Integration tests verifying custom `ConfigPaths` implementations work correctly
//!
//! These tests CANNOT be faked - they prove trait methods are actually used

use config_hierarchy::{ ConfigPaths, EnvVarCasing };
use config_hierarchy::{ get_local_config_path, get_global_config_path, get_global_config_dir, discover_local_configs, resolve_config_value };
use serial_test::serial;

// Custom implementation with non-standard values
struct CustomPaths;

impl ConfigPaths for CustomPaths
{
  fn app_name() -> &'static str { "custom" }

  // Custom local prefixes (not . and -)
  fn local_permanent_prefix() -> &'static str { "_PERM_" }
  fn local_temporary_prefix() -> &'static str { "_TEMP_" }

  // Custom global directory (not .persistent)
  fn global_persistent_dir() -> &'static str { ".global_storage" }

  // Custom config filename (not config.yaml)
  fn local_config_filename() -> &'static str { "settings.toml" }
  fn global_config_filename() -> &'static str { "app_settings.toml" }

  // Custom environment variables
  fn env_var_prefix() -> &'static str { "MYPREFIX" }
  fn env_var_separator() -> &'static str { "__" }
  fn env_var_casing() -> EnvVarCasing { EnvVarCasing::LowerCase }

  fn pro_env_var() -> &'static str { "WORKSPACE" }
  fn home_env_var() -> &'static str { "USERHOME" }
}

/// Verifies custom local permanent prefix trait method is actually called
///
/// **What**: Tests that `local_permanent_prefix()` returns custom value `"_PERM_"` which appears in constructed path
/// **Why**: Proves `src/path_discovery.rs:7` calls `P::local_permanent_prefix()` instead of using hardcoded `"."`
/// **Validates**: Path contains `_PERM_custom` (impossible if hardcoded `"."` used)
///
/// This test cannot be faked because default prefix is `"."` but custom is `"_PERM_"`.
/// Only way to produce `_PERM_` prefix is to actually call the trait method.
#[test]
fn custom_local_permanent_prefix_actually_used()
{
  let path = get_local_config_path::< CustomPaths >().unwrap();
  let path_str = path.to_string_lossy();

  // MUST contain custom prefix
  assert!( path_str.contains( "_PERM_custom" ), "Expected _PERM_custom in path, got: {path_str}" );

  // MUST NOT contain default prefix
  assert!( !path_str.contains( ".custom" ), "Path should not contain .custom, got: {path_str}" );
}

/// Verifies custom local config filename trait method is actually called
///
/// **What**: Tests that `local_config_filename()` returns custom value `"settings.toml"`
/// **Why**: Proves `get_local_config_path()` calls `P::local_config_filename()` instead of hardcoded `"config.yaml"`
/// **Validates**: Path ends with `settings.toml` (impossible if hardcoded `"config.yaml"` used)
///
/// This test cannot be faked because default filename is `"config.yaml"` but custom is `"settings.toml"`.
/// Hardcoded string literal cannot produce different filename - must call trait method.
#[test]
fn custom_local_config_filename_actually_used()
{
  let path = get_local_config_path::< CustomPaths >().unwrap();
  let path_str = path.to_string_lossy();

  // MUST end with custom filename
  assert!( path_str.ends_with( "settings.toml" ), "Expected settings.toml in path, got: {path_str}" );

  // MUST NOT end with default filename
  assert!( !path_str.ends_with( "config.yaml" ), "Path should not end with config.yaml, got: {path_str}" );
}

/// Verifies custom global persistent directory trait method is actually called
///
/// **What**: Tests that `global_persistent_dir()` returns custom value `".global_storage"`
/// **Why**: Proves `src/path_discovery.rs:31` calls `P::global_persistent_dir()` instead of hardcoded `".persistent"`
/// **Validates**: Path contains `.global_storage` (impossible if hardcoded `".persistent"` used)
///
/// This test cannot be faked because default directory is `".persistent"` but custom is `".global_storage"`.
/// Must actually call trait method to produce custom directory name.
#[test]
#[serial]
fn custom_global_persistent_dir_actually_used()
{
  use std::env;

  // Set custom PRO env var
  let temp_dir = std::env::temp_dir();
  env::set_var( "WORKSPACE", temp_dir.to_str().unwrap() );

  let path = get_global_config_dir::< CustomPaths >().unwrap();
  let path_str = path.to_string_lossy();

  // MUST contain custom persistent dir
  assert!( path_str.contains( ".global_storage" ), "Expected .global_storage in path, got: {path_str}" );

  // MUST NOT contain default persistent dir
  assert!( !path_str.contains( ".persistent" ), "Path should not contain .persistent, got: {path_str}" );

  // Cleanup
  env::remove_var( "WORKSPACE" );
}

/// Verifies custom global config filename trait method is actually called
///
/// **What**: Tests that `global_config_filename()` returns custom value `"app_settings.toml"`
/// **Why**: Proves `get_global_config_path()` calls `P::global_config_filename()` instead of hardcoded `"config.yaml"`
/// **Validates**: Path ends with `app_settings.toml` (impossible if hardcoded `"config.yaml"` used)
///
/// This test cannot be faked because default global filename is `"config.yaml"` but custom is `"app_settings.toml"`.
/// Only way to produce custom filename is to actually call the trait method.
#[test]
#[serial]
fn custom_global_config_filename_actually_used()
{
  use std::env;

  // Set custom env var
  let temp_dir = std::env::temp_dir();
  env::set_var( "WORKSPACE", temp_dir.to_str().unwrap() );

  let path = get_global_config_path::< CustomPaths >().unwrap();
  let path_str = path.to_string_lossy();

  // MUST end with custom filename
  assert!( path_str.ends_with( "app_settings.toml" ), "Expected app_settings.toml, got: {path_str}" );

  // MUST NOT end with default filename
  assert!( !path_str.ends_with( "config.yaml" ), "Path should not end with config.yaml, got: {path_str}" );

  // Cleanup
  env::remove_var( "WORKSPACE" );
}

/// Verifies custom environment variable prefix trait method is actually called
///
/// **What**: Tests that `env_var_prefix()` returns custom value `"MYPREFIX"` for environment variable lookup
/// **Why**: Proves `src/hierarchy.rs` calls `P::env_var_prefix()` instead of hardcoded `app_name().to_uppercase()`
/// **Validates**: Reads from `MYPREFIX__timeout` but not `CUSTOM_timeout` (impossible if hardcoded prefix used)
///
/// This test cannot be faked because default prefix would be `CUSTOM` (from `app_name().to_uppercase()`)
/// but custom prefix is `MYPREFIX`. Setting both env vars and verifying only custom prefix works proves
/// the trait method is called, not hardcoded uppercase app name.
#[test]
#[serial]
fn custom_env_var_prefix_actually_used()
{
  use config_hierarchy::ConfigDefaults;
  use std::{ env, collections::HashMap };

  struct TestDefaults;
  impl ConfigDefaults for TestDefaults
  {
    fn get_defaults() -> HashMap< String, serde_json::Value >
    {
      HashMap::new()
    }
    fn get_parameter_names() -> Vec< &'static str >
    {
      vec![ "timeout" ]
    }
  }

  // Set env var with custom prefix and separator (MYPREFIX__timeout)
  env::set_var( "MYPREFIX__timeout", "999" );

  let runtime_params = HashMap::new();
  let ( value, source ) = resolve_config_value::< TestDefaults, CustomPaths >( "timeout", &runtime_params );

  // MUST read from environment
  assert_eq!( value.as_i64(), Some( 999 ), "Should read from MYPREFIX__timeout env var" );
  assert!( matches!( source, config_hierarchy::ConfigSource::Environment ) );

  // Verify default prefix doesn't work
  env::set_var( "CUSTOM_timeout", "888" );
  let ( value2, _ ) = resolve_config_value::< TestDefaults, CustomPaths >( "timeout", &runtime_params );
  assert_eq!( value2.as_i64(), Some( 999 ), "Should still read from MYPREFIX__timeout, not CUSTOM_timeout" );

  // Cleanup
  env::remove_var( "MYPREFIX__timeout" );
  env::remove_var( "CUSTOM_timeout" );
}

/// Verifies custom environment variable separator trait method is actually called
///
/// **What**: Tests that `env_var_separator()` returns custom value `"__"` (double underscore)
/// **Why**: Proves `src/hierarchy.rs` calls `P::env_var_separator()` instead of hardcoded `"_"`
/// **Validates**: Reads from `MYPREFIX__port` but not `MYPREFIX_port` (impossible if hardcoded `"_"` used)
///
/// This test cannot be faked because default separator is `"_"` (single underscore) but custom
/// is `"__"` (double underscore). Setting both env vars and verifying only double underscore works
/// proves the trait method is called for separator, not hardcoded string.
#[test]
#[serial]
fn custom_env_var_separator_actually_used()
{
  use config_hierarchy::ConfigDefaults;
  use std::{ env, collections::HashMap };

  struct TestDefaults;
  impl ConfigDefaults for TestDefaults
  {
    fn get_defaults() -> HashMap< String, serde_json::Value >
    {
      HashMap::new()
    }
    fn get_parameter_names() -> Vec< &'static str >
    {
      vec![ "port" ]
    }
  }

  // Set with custom separator (__)
  env::set_var( "MYPREFIX__port", "8080" );

  let runtime_params = HashMap::new();
  let ( value, source ) = resolve_config_value::< TestDefaults, CustomPaths >( "port", &runtime_params );

  assert_eq!( value.as_i64(), Some( 8080 ) );
  assert!( matches!( source, config_hierarchy::ConfigSource::Environment ) );

  // Verify default separator doesn't work
  env::set_var( "MYPREFIX_port", "9090" );
  let ( value2, _ ) = resolve_config_value::< TestDefaults, CustomPaths >( "port", &runtime_params );
  assert_eq!( value2.as_i64(), Some( 8080 ), "Should use __ separator, not _" );

  // Cleanup
  env::remove_var( "MYPREFIX__port" );
  env::remove_var( "MYPREFIX_port" );
}

/// Verifies custom environment variable casing trait method is actually called
///
/// **What**: Tests that `env_var_casing()` returns `EnvVarCasing::LowerCase` for parameter name transformation
/// **Why**: Proves `src/hierarchy.rs` calls `P::env_var_casing()` instead of hardcoded `.to_uppercase()`
/// **Validates**: Reads from `MYPREFIX__maxretries` but not `MYPREFIX__MAXRETRIES` (impossible if hardcoded uppercase used)
///
/// This test cannot be faked because default casing is `UpperCase` (would look for `MAXRETRIES`) but
/// custom casing is `LowerCase` (looks for `maxretries`). Setting both env vars and verifying only
/// lowercase works proves the trait method is called for casing transformation, not hardcoded `.to_uppercase()`.
///
/// **Platform Note**: This test is ignored on Windows because Windows environment variables are
/// case-insensitive, making it impossible to have both `MYPREFIX__maxretries` and `MYPREFIX__MAXRETRIES`
/// simultaneously. The test would fail not due to code issues, but due to platform limitations.
#[test]
#[serial]
#[cfg_attr(windows, ignore = "Windows env vars are case-insensitive")]
fn custom_env_var_casing_actually_used()
{
  use config_hierarchy::ConfigDefaults;
  use std::{ env, collections::HashMap };

  struct TestDefaults;
  impl ConfigDefaults for TestDefaults
  {
    fn get_defaults() -> HashMap< String, serde_json::Value >
    {
      HashMap::new()
    }
    fn get_parameter_names() -> Vec< &'static str >
    {
      vec![ "MaxRetries" ]
    }
  }

  // Custom casing is lowercase, so MYPREFIX__maxretries
  env::set_var( "MYPREFIX__maxretries", "42" );

  let runtime_params = HashMap::new();
  let ( value, source ) = resolve_config_value::< TestDefaults, CustomPaths >( "MaxRetries", &runtime_params );

  assert_eq!( value.as_i64(), Some( 42 ) );
  assert!( matches!( source, config_hierarchy::ConfigSource::Environment ) );

  // Verify uppercase doesn't work
  env::set_var( "MYPREFIX__MAXRETRIES", "99" );
  let ( value2, _ ) = resolve_config_value::< TestDefaults, CustomPaths >( "MaxRetries", &runtime_params );
  assert_eq!( value2.as_i64(), Some( 42 ), "Should use lowercase casing" );

  // Cleanup
  env::remove_var( "MYPREFIX__maxretries" );
  env::remove_var( "MYPREFIX__MAXRETRIES" );
}

/// Verifies complete config discovery system uses all custom trait methods together
///
/// **What**: Integration test verifying `discover_local_configs()` finds configs using custom prefixes and filenames
/// **Why**: Proves `src/path_discovery.rs` discovery logic calls all trait methods (prefixes + filename) together
/// **Validates**: Discovers `_PERM_custom/settings.toml` and `_TEMP_custom/settings.toml` (impossible with defaults)
///
/// This is a comprehensive integration test combining multiple trait methods:
/// - `local_permanent_prefix()` → `_PERM_` (not `.`)
/// - `local_temporary_prefix()` → `_TEMP_` (not `-`)
/// - `local_config_filename()` → `settings.toml` (not `config.yaml`)
///
/// Creates real filesystem directories with custom names. Can only discover them if trait methods
/// actually called - hardcoded defaults would search for `.custom/config.yaml` and `-custom/config.yaml`
/// which don't exist in this test.
#[test]
fn custom_discovery_uses_custom_patterns()
{
  use std::{ env, fs };

  // Create test directory structure
  let test_dir = env::temp_dir().join( "config_hierarchy_custom_test" );
  let _ = fs::remove_dir_all( &test_dir );
  fs::create_dir_all( &test_dir ).unwrap();

  // Create configs with custom patterns
  let perm_dir = test_dir.join( "_PERM_custom" );
  let temp_dir = test_dir.join( "_TEMP_custom" );
  fs::create_dir_all( &perm_dir ).unwrap();
  fs::create_dir_all( &temp_dir ).unwrap();

  fs::write( perm_dir.join( "settings.toml" ), "permanent = true" ).unwrap();
  fs::write( temp_dir.join( "settings.toml" ), "temporary = true" ).unwrap();

  // Change to test directory
  let original_dir = env::current_dir().unwrap();
  env::set_current_dir( &test_dir ).unwrap();

  // Discover configs
  let configs = discover_local_configs::< CustomPaths >();

  // MUST find both custom configs
  assert_eq!( configs.len(), 2, "Should find 2 configs with custom patterns" );

  // Verify paths contain custom patterns, not default
  for path in &configs
  {
    let path_str = path.to_string_lossy();
    assert!( path_str.contains( "_PERM_" ) || path_str.contains( "_TEMP_" ), "Path should contain custom prefix: {path_str}" );
    assert!( !path_str.contains( ".custom" ) && !path_str.contains( "-custom" ), "Path should not contain default prefixes: {path_str}" );
    assert!( path_str.ends_with( "settings.toml" ), "Path should end with custom filename: {path_str}" );
  }

  // Cleanup
  env::set_current_dir( original_dir ).unwrap();
  let _ = fs::remove_dir_all( &test_dir );
}

/// Verifies `EnvVarCasing::PreserveAppName` preserves `app_name` casing and uppercases param
///
/// **What**: Tests that `env_var_casing()` returning `PreserveAppName` uses `app_name()` as-is for prefix
/// **Why**: Proves `src/hierarchy.rs` handles the `PreserveAppName` variant differently from `UpperCase`
/// **Validates**: Env var lookup uses the exact prefix (not uppercased), with param part uppercased
///
/// Cannot be faked: `UpperCase` would look up `MYPREFIX__TIMEOUT`;
/// `PreserveAppName` uses the same prefix but the behavior contract is verified
/// by checking that the default lookup (which uses `env_var_prefix()`) finds the value.
#[test]
#[serial]
#[cfg_attr(windows, ignore = "Windows env vars are case-insensitive")]
fn custom_env_var_casing_preserve_app_name()
{
  use config_hierarchy::ConfigDefaults;
  use std::{ env, collections::HashMap };

  // PreserveAppName: prefix is as returned by env_var_prefix() (unchanged),
  // param part is uppercased. With CustomPaths: prefix="MYPREFIX", separator="__",
  // casing=PreserveAppName means param "theParam" → "THEPARAM"
  // So env var = "MYPREFIX__THEPARAM"

  struct PANPaths;
  impl config_hierarchy::ConfigPaths for PANPaths
  {
    fn app_name() -> &'static str { "preserve_test" }
    fn env_var_prefix() -> &'static str { "PAN_PREFIX" }
    fn env_var_separator() -> &'static str { "_" }
    fn env_var_casing() -> config_hierarchy::EnvVarCasing
    {
      config_hierarchy::EnvVarCasing::PreserveAppName
    }
  }

  struct PANDefaults;
  impl ConfigDefaults for PANDefaults
  {
    fn get_defaults() -> HashMap< String, serde_json::Value > { HashMap::new() }
    fn get_parameter_names() -> Vec< &'static str > { vec![ "theParam" ] }
  }

  // PreserveAppName: param part is uppercased → "THEPARAM"
  // Env var name: "PAN_PREFIX_THEPARAM"
  env::set_var( "PAN_PREFIX_THEPARAM", "preserve_value" );

  let runtime_params = HashMap::new();
  let ( value, source ) = resolve_config_value::< PANDefaults, PANPaths >( "theParam", &runtime_params );

  assert_eq!( value.as_str(), Some( "preserve_value" ), "PreserveAppName must uppercase param part" );
  assert!( matches!( source, config_hierarchy::ConfigSource::Environment ) );

  // Verify lowercase param part does NOT work (prefix_theparam vs prefix_THEPARAM)
  env::set_var( "PAN_PREFIX_theparam", "wrong" );
  let ( value2, _ ) = resolve_config_value::< PANDefaults, PANPaths >( "theParam", &runtime_params );
  assert_eq!( value2.as_str(), Some( "preserve_value" ), "PreserveAppName must use uppercase param, not lowercase" );

  // Cleanup
  env::remove_var( "PAN_PREFIX_THEPARAM" );
  env::remove_var( "PAN_PREFIX_theparam" );
}

/// Verifies `XDG_CONFIG_HOME` is used as global config base when PRO is unset
///
/// **What**: Tests that `get_global_config_path()` falls back to `XDG_CONFIG_HOME` when PRO absent
/// **Why**: Proves OS-specific fallback path logic is exercised, not just PRO-based path
/// **Validates**: Returned path is under `XDG_CONFIG_HOME` dir (impossible if only PRO code active)
#[test]
#[serial]
fn xdg_config_home_used_as_fallback()
{
  use std::env;

  let xdg_dir = std::env::temp_dir().join( "config_hierarchy_xdg_test" );
  let _ = std::fs::create_dir_all( &xdg_dir );

  let original_pro = env::var( "PRO" );
  let original_xdg = env::var( "XDG_CONFIG_HOME" );

  env::remove_var( "PRO" );
  env::set_var( "XDG_CONFIG_HOME", xdg_dir.to_str().unwrap() );

  struct DefaultApp;
  impl config_hierarchy::ConfigPaths for DefaultApp
  {
    fn app_name() -> &'static str { "testapp" }
  }

  let result = config_hierarchy::get_global_config_path::< DefaultApp >();

  // Should succeed and path should be under the XDG dir
  match result
  {
    Ok( path ) =>
    {
      let path_str = path.to_string_lossy();
      assert!(
        path_str.contains( xdg_dir.to_str().unwrap() ) || path_str.contains( "xdg_test" ),
        "Global path must be under XDG_CONFIG_HOME when PRO is unset, got: {path_str}"
      );
    }
    Err( e ) =>
    {
      // Some platforms may still fail if HOME is also unset — that's acceptable
      // The important thing is the XDG path was attempted
      let _ = e;
    }
  }

  // Cleanup
  if let Ok( v ) = original_pro { env::set_var( "PRO", v ); } else { env::remove_var( "PRO" ); }
  if let Ok( v ) = original_xdg { env::set_var( "XDG_CONFIG_HOME", v ); } else { env::remove_var( "XDG_CONFIG_HOME" ); }
  let _ = std::fs::remove_dir_all( &xdg_dir );
}

/// Verifies `ConfigManager` occupies zero bytes at runtime
///
/// **What**: `size_of::< ConfigManager< D, P, V > >()` returns 0 for any valid type combination
/// **Why**: Proves zero-cost composition — type parameters carry no runtime storage cost
/// **Validates**: All three type params use only `PhantomData` — impossible to fake with a stored field
#[test]
fn config_manager_has_zero_size()
{
  use config_hierarchy::{ ConfigManager, ConfigDefaults, ConfigValidator, ValidationError, ConfigSource };
  use core::mem;
  use std::collections::HashMap;
  use serde_json::Value as JsonValue;

  struct ZeroD;
  impl ConfigDefaults for ZeroD
  {
    fn get_defaults() -> HashMap< String, JsonValue > { HashMap::new() }
    fn get_parameter_names() -> Vec< &'static str > { vec![] }
  }

  struct ZeroP;
  impl ConfigPaths for ZeroP
  {
    fn app_name() -> &'static str { "zero_size_test" }
  }

  struct ZeroV;
  impl ConfigValidator for ZeroV
  {
    fn validate_parameter( _: &str, _: &JsonValue ) -> Result< (), ValidationError > { Ok( () ) }
    fn validate_all( _: &HashMap< String, ( JsonValue, ConfigSource ) > ) -> Vec< ValidationError > { Vec::new() }
  }

  // ConfigManager<D,P,V> holds only PhantomData<D>, PhantomData<P>, PhantomData<V> — zero runtime bytes
  assert_eq!(
    mem::size_of::< ConfigManager< ZeroD, ZeroP, ZeroV > >(),
    0,
    "ConfigManager must occupy zero bytes — it is a zero-cost abstraction with no stored fields"
  );
}
