// allow: test binary functions are not part of the public API; documentation not required
#![ allow( missing_docs ) ]

use config_hierarchy::*;
use std::collections::HashMap;
use serde_json::Value as JsonValue;
use tempfile::TempDir;
use std::env;
use serial_test::serial;

struct TestDefaults;
impl ConfigDefaults for TestDefaults
{
  fn get_defaults() -> HashMap< String, JsonValue >
  {
    let mut map = HashMap::new();
    map.insert( "param1".into(), JsonValue::String( "default".into() ) );
    map.insert( "param2".into(), JsonValue::Number( 100.into() ) );
    map
  }

  fn get_parameter_names() -> Vec< &'static str >
  {
    vec![ "param1", "param2" ]
  }
}

struct TestPaths;
impl ConfigPaths for TestPaths
{
  fn app_name() -> &'static str { "testapp" }
}

struct TestValidator;
impl ConfigValidator for TestValidator
{
  fn validate_parameter( _param : &str, _value : &JsonValue ) -> Result< (), ValidationError >
  {
    Ok( () )
  }

  fn validate_all( _config : &HashMap< String, ( JsonValue, ConfigSource ) > ) -> Vec< ValidationError >
  {
    Vec::new()
  }
}

type TestConfig = ConfigManager< TestDefaults, TestPaths, TestValidator >;

#[ test ]
#[ serial ]
fn test_default_source()
{
  // Save and unset all environment variables that could affect config resolution
  let original_pro = env::var( "PRO" );
  let original_home = env::var( "HOME" );
  let original_userprofile = env::var( "USERPROFILE" );
  let original_xdg_config = env::var( "XDG_CONFIG_HOME" );
  let original_appdata = env::var( "APPDATA" );

  env::remove_var( "PRO" );
  env::remove_var( "HOME" );
  env::remove_var( "USERPROFILE" );
  env::remove_var( "XDG_CONFIG_HOME" );
  env::remove_var( "APPDATA" );

  let runtime_params = HashMap::new();
  let ( value, source ) = TestConfig::resolve_config_value( "param1", &runtime_params );

  assert_eq!( value, JsonValue::String( "default".into() ) );
  assert!( matches!( source, ConfigSource::Default ) );

  // Restore environment variables
  if let Ok( val ) = original_pro { env::set_var( "PRO", val ); }
  if let Ok( val ) = original_home { env::set_var( "HOME", val ); }
  if let Ok( val ) = original_userprofile { env::set_var( "USERPROFILE", val ); }
  if let Ok( val ) = original_xdg_config { env::set_var( "XDG_CONFIG_HOME", val ); }
  if let Ok( val ) = original_appdata { env::set_var( "APPDATA", val ); }
}

#[ test ]
fn test_runtime_overrides_default()
{
  let mut runtime_params = HashMap::new();
  runtime_params.insert( "param1".into(), "runtime_value".into() );

  let ( value, source ) = TestConfig::resolve_config_value( "param1", &runtime_params );

  assert_eq!( value, JsonValue::String( "runtime_value".into() ) );
  assert!( matches!( source, ConfigSource::Runtime ) );
}

#[ test ]
#[ serial ]
fn test_env_overrides_default()
{
  env::set_var( "TESTAPP_ENVPARAM", "env_value" );

  let runtime_params = HashMap::new();
  let ( value, source ) = TestConfig::resolve_config_value( "envparam", &runtime_params );

  assert_eq!( value, JsonValue::String( "env_value".into() ) );
  assert!( matches!( source, ConfigSource::Environment ) );

  env::remove_var( "TESTAPP_ENVPARAM" );
}

#[ test ]
#[ serial ]
fn test_runtime_overrides_env()
{
  env::set_var( "TESTAPP_TESTPARAM", "env_value" );

  let mut runtime_params = HashMap::new();
  runtime_params.insert( "testparam".into(), "runtime_value".into() );

  let ( value, source ) = TestConfig::resolve_config_value( "testparam", &runtime_params );

  assert_eq!( value, JsonValue::String( "runtime_value".into() ) );
  assert!( matches!( source, ConfigSource::Runtime ) );

  env::remove_var( "TESTAPP_TESTPARAM" );
}

#[ test ]
#[ serial ]
fn test_global_config_overrides_default()
{
  let temp_dir = TempDir::new().unwrap();
  let original_pro = env::var( "PRO" );

  env::set_var( "PRO", temp_dir.path() );

  let mut config = HashMap::new();
  config.insert( "param1".into(), JsonValue::String( "global_value".into() ) );

  TestConfig::save_global_config( &config ).unwrap();

  let runtime_params = HashMap::new();
  let ( value, source ) = TestConfig::resolve_config_value( "param1", &runtime_params );

  assert_eq!( value, JsonValue::String( "global_value".into() ) );
  assert!( matches!( source, ConfigSource::Global( _ ) ) );

  if let Ok( original ) = original_pro
  {
    env::set_var( "PRO", original );
  }
  else
  {
    env::remove_var( "PRO" );
  }
}

#[ test ]
#[ serial ]
fn test_resolve_all_config()
{
  // Save and unset all environment variables that could affect config resolution
  let original_pro = env::var( "PRO" );
  let original_home = env::var( "HOME" );
  let original_userprofile = env::var( "USERPROFILE" );
  let original_xdg_config = env::var( "XDG_CONFIG_HOME" );
  let original_appdata = env::var( "APPDATA" );

  env::remove_var( "PRO" );
  env::remove_var( "HOME" );
  env::remove_var( "USERPROFILE" );
  env::remove_var( "XDG_CONFIG_HOME" );
  env::remove_var( "APPDATA" );

  let runtime_params = HashMap::new();
  let all_config = TestConfig::resolve_all_config( &runtime_params );

  assert!( all_config.contains_key( "param1" ) );
  assert!( all_config.contains_key( "param2" ) );

  let ( value1, source1 ) = &all_config[ "param1" ];
  assert_eq!( value1, &JsonValue::String( "default".into() ) );
  assert!( matches!( source1, ConfigSource::Default ) );

  // Restore environment variables
  if let Ok( val ) = original_pro { env::set_var( "PRO", val ); }
  if let Ok( val ) = original_home { env::set_var( "HOME", val ); }
  if let Ok( val ) = original_userprofile { env::set_var( "USERPROFILE", val ); }
  if let Ok( val ) = original_xdg_config { env::set_var( "XDG_CONFIG_HOME", val ); }
  if let Ok( val ) = original_appdata { env::set_var( "APPDATA", val ); }
}

#[ test ]
#[ serial ]
fn test_unknown_parameter_returns_null()
{
  let runtime_params = HashMap::new();
  let ( value, source ) = TestConfig::resolve_config_value( "unknown_param", &runtime_params );

  assert_eq!( value, JsonValue::Null );
  assert!( matches!( source, ConfigSource::Default ) );
}

#[ test ]
#[ serial ]
fn test_case_sensitive_env_var()
{
  env::set_var( "TESTAPP_PARAM_WITH_CASE", "value" );

  let runtime_params = HashMap::new();
  let ( value, _ ) = TestConfig::resolve_config_value( "param_with_case", &runtime_params );

  assert_eq!( value, JsonValue::String( "value".into() ) );

  env::remove_var( "TESTAPP_PARAM_WITH_CASE" );
}

#[ test ]
#[ serial ]
fn test_type_detection_in_hierarchy()
{
  env::set_var( "TESTAPP_BOOL_PARAM", "true" );

  let runtime_params = HashMap::new();
  let ( value, _ ) = TestConfig::resolve_config_value( "bool_param", &runtime_params );

  assert_eq!( value, JsonValue::Bool( true ) );

  env::remove_var( "TESTAPP_BOOL_PARAM" );
}

// IN-03: Local config (L3/L4) overrides global config (L5)
//
// ## Root Cause (of original gap)
// invariant/001 defines L3 (LocalCurrent) > L5 (Global) but no test exercised
// a scenario where both a local file and a global file provide the same param.
//
// ## Fix Applied
// Test creates both a global config file and a local config file in a temp CWD,
// verifying the local value wins and source is LocalCurrent.
#[ test ]
#[ serial ]
fn test_local_config_overrides_global()
{
  use std::fs;

  let global_dir = TempDir::new().unwrap();
  let local_dir  = TempDir::new().unwrap();

  let original_pro = env::var( "PRO" );
  env::set_var( "PRO", global_dir.path() );

  // Write global config
  let mut global_config = HashMap::new();
  global_config.insert( "param1".into(), JsonValue::String( "global_value".into() ) );
  TestConfig::save_global_config( &global_config ).unwrap();

  // Write local config in the temp directory
  let local_app_dir = local_dir.path().join( ".testapp" );
  fs::create_dir_all( &local_app_dir ).unwrap();
  let local_path = local_app_dir.join( "config.yaml" );
  let mut local_config = HashMap::new();
  local_config.insert( "param1".into(), JsonValue::String( "local_value".into() ) );
  TestConfig::save_config_file( &local_config, &local_path ).unwrap();

  // Change CWD to local_dir so the local config is discovered
  let original_dir = env::current_dir().unwrap();
  env::set_current_dir( local_dir.path() ).unwrap();

  let runtime_params = HashMap::new();
  let ( value, source ) = TestConfig::resolve_config_value( "param1", &runtime_params );

  assert_eq!( value, JsonValue::String( "local_value".into() ), "Local config must override global" );
  assert!( matches!( source, ConfigSource::LocalCurrent( _ ) ), "Source must be LocalCurrent, got: {source:?}" );

  // Restore
  env::set_current_dir( original_dir ).unwrap();
  if let Ok( v ) = original_pro { env::set_var( "PRO", v ); } else { env::remove_var( "PRO" ); }
}

// IN-02: Directory depth beats pattern type within local configs
//
// ## Root Cause (of original gap)
// invariant/001 defines L3 (LocalCurrent, depth=0) > L4 (LocalParent, depth>0)
// regardless of temporary vs permanent pattern type. No test exercised a case
// where a permanent config at depth=0 competes with a temporary config at depth=1.
//
// ## Fix Applied
// Creates a child CWD with a permanent (.testapp) config and places a temporary
// (-testapp) config in the parent dir one level up. Verifies the child permanent
// config wins and source is LocalCurrent — proving depth beats pattern type.
#[ test ]
#[ serial ]
fn test_local_current_overrides_local_parent()
{
  use std::fs;

  // Parent dir gets a TEMPORARY (-testapp) local config (would be L4 LocalParent)
  let parent_dir = TempDir::new().unwrap();
  let temp_app_dir = parent_dir.path().join( "-testapp" );
  fs::create_dir_all( &temp_app_dir ).unwrap();
  let parent_config_path = temp_app_dir.join( "config.yaml" );
  let mut parent_config = HashMap::new();
  parent_config.insert( "param1".into(), JsonValue::String( "parent_temp_value".into() ) );
  TestConfig::save_config_file( &parent_config, &parent_config_path ).unwrap();

  // Child dir (inside parent) gets a PERMANENT (.testapp) local config (L3 LocalCurrent)
  let child_dir = parent_dir.path().join( "child" );
  fs::create_dir_all( &child_dir ).unwrap();
  let perm_app_dir = child_dir.join( ".testapp" );
  fs::create_dir_all( &perm_app_dir ).unwrap();
  let child_config_path = perm_app_dir.join( "config.yaml" );
  let mut child_config = HashMap::new();
  child_config.insert( "param1".into(), JsonValue::String( "current_perm_value".into() ) );
  TestConfig::save_config_file( &child_config, &child_config_path ).unwrap();

  // Change CWD to child dir: depth=0 is permanent, depth=1 is temporary
  let original_dir     = env::current_dir().unwrap();
  let original_pro     = env::var( "PRO" );
  let original_home    = env::var( "HOME" );
  let original_xdg     = env::var( "XDG_CONFIG_HOME" );
  let original_appdata = env::var( "APPDATA" );

  env::set_current_dir( &child_dir ).unwrap();
  // Unset global config vars so only local configs are consulted
  env::remove_var( "PRO" );
  env::remove_var( "HOME" );
  env::remove_var( "XDG_CONFIG_HOME" );
  env::remove_var( "APPDATA" );

  let runtime_params = HashMap::new();
  let ( value, source ) = TestConfig::resolve_config_value( "param1", &runtime_params );

  assert_eq!
  (
    value,
    JsonValue::String( "current_perm_value".into() ),
    "Current-dir permanent config must override parent-dir temporary config (depth beats pattern type)"
  );
  assert!
  (
    matches!( source, ConfigSource::LocalCurrent( _ ) ),
    "Source must be LocalCurrent (depth=0), not LocalParent; got: {source:?}"
  );

  // Restore
  env::set_current_dir( original_dir ).unwrap();
  if let Ok( v ) = original_pro     { env::set_var( "PRO", v );             } else { env::remove_var( "PRO" );             }
  if let Ok( v ) = original_home    { env::set_var( "HOME", v );            } else { env::remove_var( "HOME" );            }
  if let Ok( v ) = original_xdg     { env::set_var( "XDG_CONFIG_HOME", v ); } else { env::remove_var( "XDG_CONFIG_HOME" ); }
  if let Ok( v ) = original_appdata { env::set_var( "APPDATA", v );         } else { env::remove_var( "APPDATA" );         }
}

// AP-04 (api/002): resolve_all includes params found in config files but not in get_parameter_names()
//
// ## Root Cause (of original gap)
// api/002 documents the secondary scan behavior but no test exercised it.
// A broken secondary scan would silently drop all undeclared config file params.
//
// ## Fix Applied
// Test writes a global config file with a key not in get_parameter_names(),
// then verifies resolve_all_config() returns it via the secondary scan.
#[ test ]
#[ serial ]
fn test_resolve_all_includes_undeclared_config_file_params()
{
  let temp_dir = TempDir::new().unwrap();
  let original_pro = env::var( "PRO" );
  let original_home = env::var( "HOME" );
  let original_xdg = env::var( "XDG_CONFIG_HOME" );
  let original_appdata = env::var( "APPDATA" );

  env::set_var( "PRO", temp_dir.path() );
  // Unset fallback paths so global config uses PRO
  env::remove_var( "HOME" );
  env::remove_var( "XDG_CONFIG_HOME" );
  env::remove_var( "APPDATA" );

  // Write global config with "extra_param" not declared in get_parameter_names()
  let mut config = HashMap::new();
  config.insert( "extra_param".into(), JsonValue::String( "from_global_file".into() ) );
  TestConfig::save_global_config( &config ).unwrap();

  let runtime_params = HashMap::new();
  let all_config = TestConfig::resolve_all_config( &runtime_params );

  assert!
  (
    all_config.contains_key( "extra_param" ),
    "resolve_all must include params from config files not in get_parameter_names()"
  );
  assert_eq!
  (
    all_config[ "extra_param" ].0,
    JsonValue::String( "from_global_file".into() )
  );

  // Restore
  if let Ok( v ) = original_pro { env::set_var( "PRO", v ); } else { env::remove_var( "PRO" ); }
  if let Ok( v ) = original_home { env::set_var( "HOME", v ); } else { env::remove_var( "HOME" ); }
  if let Ok( v ) = original_xdg { env::set_var( "XDG_CONFIG_HOME", v ); } else { env::remove_var( "XDG_CONFIG_HOME" ); }
  if let Ok( v ) = original_appdata { env::set_var( "APPDATA", v ); } else { env::remove_var( "APPDATA" ); }
}
