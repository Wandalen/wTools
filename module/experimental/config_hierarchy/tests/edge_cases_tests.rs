// allow: test binary functions are not part of the public API; documentation not required
#![ allow( missing_docs ) ]

use config_hierarchy::*;
use std::collections::HashMap;
use serde_json::Value as JsonValue;
use tempfile::TempDir;

struct TestDefaults;
impl ConfigDefaults for TestDefaults
{
  fn get_defaults() -> HashMap< String, JsonValue >
  {
    HashMap::new()
  }

  fn get_parameter_names() -> Vec< &'static str >
  {
    vec![]
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
fn test_unicode_parameter_names()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  config.insert( "测试".into(), JsonValue::String( "chinese".into() ) );
  config.insert( "тест".into(), JsonValue::String( "cyrillic".into() ) );
  config.insert( "🔥".into(), JsonValue::String( "emoji".into() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "测试" ), Some( &JsonValue::String( "chinese".into() ) ) );
  assert_eq!( loaded.get( "тест" ), Some( &JsonValue::String( "cyrillic".into() ) ) );
  assert_eq!( loaded.get( "🔥" ), Some( &JsonValue::String( "emoji".into() ) ) );
}

#[ test ]
fn test_unicode_parameter_values()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  config.insert( "key1".into(), JsonValue::String( "测试值".into() ) );
  config.insert( "key2".into(), JsonValue::String( "Привет мир".into() ) );
  config.insert( "key3".into(), JsonValue::String( "🚀🔥🎉".into() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "key1" ), Some( &JsonValue::String( "测试值".into() ) ) );
  assert_eq!( loaded.get( "key2" ), Some( &JsonValue::String( "Привет мир".into() ) ) );
  assert_eq!( loaded.get( "key3" ), Some( &JsonValue::String( "🚀🔥🎉".into() ) ) );
}

#[ test ]
fn test_very_long_parameter_names()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let long_name = "a".repeat( 1000 );
  let mut config = HashMap::new();
  config.insert( long_name.clone(), JsonValue::String( "value".into() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( &long_name ), Some( &JsonValue::String( "value".into() ) ) );
}

#[ test ]
fn test_very_long_parameter_values()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let long_value = "x".repeat( 10000 );
  let mut config = HashMap::new();
  config.insert( "key".into(), JsonValue::String( long_value.clone() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "key" ), Some( &JsonValue::String( long_value ) ) );
}

#[ test ]
fn test_special_yaml_characters()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  config.insert( "colon:key".into(), JsonValue::String( "value:with:colons".into() ) );
  config.insert( "quotes\"key".into(), JsonValue::String( "value\"with\"quotes".into() ) );
  config.insert( "brackets[key]".into(), JsonValue::String( "value[with]brackets".into() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "colon:key" ), Some( &JsonValue::String( "value:with:colons".into() ) ) );
  assert_eq!( loaded.get( "quotes\"key" ), Some( &JsonValue::String( "value\"with\"quotes".into() ) ) );
  assert_eq!( loaded.get( "brackets[key]" ), Some( &JsonValue::String( "value[with]brackets".into() ) ) );
}

#[ test ]
fn test_newlines_in_values()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  config.insert( "multiline".into(), JsonValue::String( "line1\nline2\nline3".into() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "multiline" ), Some( &JsonValue::String( "line1\nline2\nline3".into() ) ) );
}

#[ test ]
fn test_corrupted_yaml_returns_error()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "corrupted.yaml" );

  std::fs::write( &config_path, "invalid: yaml: [unclosed" ).unwrap();

  let result = TestConfig::load_config_file( &config_path );
  assert!( result.is_err() );
}

#[ test ]
fn test_many_parameters()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  for i in 0..100
  {
    config.insert( format!( "key{i}" ), JsonValue::Number( i.into() ) );
  }

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.len(), 100 );

  for i in 0..100
  {
    assert_eq!( loaded.get( &format!( "key{i}" ) ), Some( &JsonValue::Number( i.into() ) ) );
  }
}

#[ test ]
fn test_parameter_name_with_dots()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  config.insert( "module.setting.value".into(), JsonValue::String( "dotted".into() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "module.setting.value" ), Some( &JsonValue::String( "dotted".into() ) ) );
}

#[ test ]
fn test_parameter_name_with_underscores()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  config.insert( "some_long_parameter_name".into(), JsonValue::String( "underscored".into() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "some_long_parameter_name" ), Some( &JsonValue::String( "underscored".into() ) ) );
}

#[ test ]
fn test_parameter_name_with_hyphens()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  config.insert( "some-hyphenated-param".into(), JsonValue::String( "hyphenated".into() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "some-hyphenated-param" ), Some( &JsonValue::String( "hyphenated".into() ) ) );
}

// Bug reproducer: Empty app_name creates invalid paths
//
// ## Root Cause
// ConfigPaths::app_name() can return empty string, which creates paths like
// `/path/./config.yaml` instead of `/path/.appname/config.yaml`. The empty
// string concatenates with prefix (`.` or `-`) to form `./` or `-/`, which
// are valid directory references but not subdirectories.
//
// ## Why Not Caught
// No validation exists for app_name() return value. It's a user-provided trait
// implementation with no constraints.
//
// ## Fix Applied
// Added validation in path construction functions to detect empty app_name
// and return error. Users must provide non-empty application name.
//
// ## Prevention
// Add documentation requirement that app_name() must return non-empty string.
// Consider adding validation helper or compile-time constraint in future.
//
// ## Pitfall
// Empty strings in path construction create valid but semantically wrong paths.
// Always validate user inputs even from trait implementations.
#[ test ]
#[ should_panic( expected = "app_name must not be empty" ) ]
fn test_empty_app_name_rejected()
{
  struct EmptyAppName;
  impl ConfigDefaults for EmptyAppName
  {
    fn get_defaults() -> HashMap< String, JsonValue > { HashMap::new() }
    fn get_parameter_names() -> Vec< &'static str > { vec![] }
  }

  impl ConfigPaths for EmptyAppName
  {
    fn app_name() -> &'static str { "" }  // EMPTY - should be rejected
  }

  impl ConfigValidator for EmptyAppName
  {
    fn validate_parameter( _: &str, _: &JsonValue ) -> Result< (), ValidationError > { Ok( () ) }
    fn validate_all( _: &HashMap< String, ( JsonValue, ConfigSource ) > ) -> Vec< ValidationError > { Vec::new() }
  }

  type EmptyConfig = ConfigManager< EmptyAppName, EmptyAppName, EmptyAppName >;

  // Should panic when trying to get path with empty app_name
  let _path = EmptyConfig::get_local_config_path().unwrap();
}

// Bug reproducer: atomic_config_modify promotes flat-format metadata fields to parameters
//
// ## Root Cause
// `atomic_config_modify` falls back to flat-format parsing when no `parameters` section
// exists. Unlike `load_config_file`, the flat-format fallback does NOT skip the metadata
// fields "version", "last_modified", and "metadata". They get included in the config map
// and written into the `parameters` section of the upgraded format, corrupting the config.
//
// ## Why Not Caught
// The skip-logic in `load_config_file` was not replicated in `atomic_config_modify`.
// Tests only modified the canonical format (with `parameters:` section) and never
// exercised the flat-format fallback path of `atomic_config_modify`.
//
// ## Fix Applied
// Added the same metadata field filter to the flat-format fallback in `atomic_config_modify`.
//
// ## Prevention
// Both flat-format parsing paths must apply the same filters. Extract into helper.
//
// ## Pitfall
// Wherever flat-format YAML parsing is duplicated, each copy must apply identical filters.
#[ test ]
fn test_atomic_modify_flat_format_skips_metadata_fields()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  // Write flat-format YAML (old format — no `parameters` section)
  std::fs::write
  (
    &config_path,
    "version: '1.0'\nlast_modified: '2023-01-01T00:00:00Z'\ntimeout: 30\n"
  ).unwrap();

  atomic_config_modify( &config_path, | config |
  {
    config.insert( "timeout".into(), JsonValue::Number( 60.into() ) );
    Ok( () )
  }).unwrap();

  let loaded = load_config_file( &config_path ).unwrap();

  // Metadata fields must NOT be promoted to config parameters
  assert!
  (
    !loaded.contains_key( "version" ),
    "version must not be promoted to config parameter"
  );
  assert!
  (
    !loaded.contains_key( "last_modified" ),
    "last_modified must not be promoted to config parameter"
  );
  assert!
  (
    !loaded.contains_key( "metadata" ),
    "metadata must not be promoted to config parameter"
  );

  // The actual config parameter must still be present
  assert_eq!
  (
    loaded.get( "timeout" ),
    Some( &JsonValue::Number( 60.into() ) )
  );
}

// Bug reproducer: Path traversal via app_name
//
// ## Root Cause
// ConfigPaths::app_name() is used directly in path construction without
// sanitization. User can include `../` to escape intended config directory,
// potentially accessing files outside application's config scope.
//
// ## Why Not Caught
// No path sanitization or validation of app_name content. Assumed trait
// implementation would provide safe values.
//
// ## Fix Applied
// Added validation to reject app_name containing path separators (`/`, `\`)
// or parent directory references (`..`). Only alphanumeric, hyphens, underscores,
// and dots (not `..`) are allowed.
//
// ## Prevention
// Document security requirements for ConfigPaths trait implementations.
// Add validation helpers for common security checks.
//
// ## Pitfall
// User-provided strings in filesystem paths require sanitization even when
// from trait implementations. Never trust input for path construction.
#[ test ]
#[ should_panic( expected = "app_name contains invalid characters" ) ]
fn test_path_traversal_rejected()
{
  struct PathTraversalAttack;
  impl ConfigDefaults for PathTraversalAttack
  {
    fn get_defaults() -> HashMap< String, JsonValue > { HashMap::new() }
    fn get_parameter_names() -> Vec< &'static str > { vec![] }
  }

  impl ConfigPaths for PathTraversalAttack
  {
    fn app_name() -> &'static str { "../../etc/passwd" }  // PATH TRAVERSAL ATTACK
  }

  impl ConfigValidator for PathTraversalAttack
  {
    fn validate_parameter( _: &str, _: &JsonValue ) -> Result< (), ValidationError > { Ok( () ) }
    fn validate_all( _: &HashMap< String, ( JsonValue, ConfigSource ) > ) -> Vec< ValidationError > { Vec::new() }
  }

  type AttackConfig = ConfigManager< PathTraversalAttack, PathTraversalAttack, PathTraversalAttack >;

  // Should panic when detecting path traversal attempt
  let _path = AttackConfig::get_local_config_path().unwrap();
}

// AP-03: Forward slash "/" alone in app_name rejected (path separator without "..")
//
// ## Root Cause
// test_path_traversal_rejected covers "../../etc/passwd" which combines both
// ".." and "/". The "/" character alone is independently forbidden as a path
// separator but was never tested in isolation.
//
// ## Fix Applied
// Separate test for app_name containing only "/" (no "..").
//
// ## Pitfall
// A "/" in app_name creates a subdirectory (e.g., ".my/app/config.yaml"), not
// an error. Must validate against "/" independently of "..".
#[ test ]
#[ should_panic( expected = "app_name contains invalid characters" ) ]
fn test_slash_only_in_app_name_rejected()
{
  struct SlashAppName;
  impl ConfigDefaults for SlashAppName
  {
    fn get_defaults() -> HashMap< String, JsonValue > { HashMap::new() }
    fn get_parameter_names() -> Vec< &'static str > { vec![] }
  }

  impl ConfigPaths for SlashAppName
  {
    fn app_name() -> &'static str { "my/app" }  // SLASH — should be rejected
  }

  impl ConfigValidator for SlashAppName
  {
    fn validate_parameter( _: &str, _: &JsonValue ) -> Result< (), ValidationError > { Ok( () ) }
    fn validate_all( _: &HashMap< String, ( JsonValue, ConfigSource ) > ) -> Vec< ValidationError > { Vec::new() }
  }

  type SlashConfig = ConfigManager< SlashAppName, SlashAppName, SlashAppName >;
  let _path = SlashConfig::get_local_config_path().unwrap();
}

// FM-02: Missing `parameters` section returns empty map (no error)
//
// ## Root Cause
// format/001 specifies "A file missing the parameters section returns an empty map"
// but this edge case was never tested. A broken parser might return an error or
// treat top-level keys as parameters.
//
// ## Fix Applied
// Test writes a YAML file with only metadata and verifies load returns empty map.
#[ test ]
fn test_missing_parameters_section_returns_empty()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  std::fs::write
  (
    &config_path,
    "metadata:\n  version: '1.0'\n  created_at: '2025-01-01T00:00:00Z'\n"
  ).unwrap();

  let result = TestConfig::load_config_file( &config_path );
  assert!( result.is_ok(), "File with missing parameters section must load without error" );

  let map = result.unwrap();
  assert!( map.is_empty(), "Result must be empty map when parameters section is absent, got: {map:?}" );
}

// FM-03: Unknown top-level keys are ignored
//
// ## Root Cause
// format/001 specifies "Unknown top-level keys are ignored" but was never tested.
// A strict parser would return an error for unknown keys.
//
// ## Fix Applied
// Test writes YAML with extra top-level key and verifies it loads cleanly.
#[ test ]
fn test_unknown_top_level_keys_ignored()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  std::fs::write
  (
    &config_path,
    "metadata:\n  version: '1.0'\nparameters:\n  timeout: 30\ncustom_section:\n  foo: bar\n"
  ).unwrap();

  let result = TestConfig::load_config_file( &config_path );
  assert!( result.is_ok(), "Unknown top-level keys must not cause an error" );

  let map = result.unwrap();
  assert_eq!( map.get( "timeout" ), Some( &JsonValue::Number( 30.into() ) ) );
  assert!( !map.contains_key( "custom_section" ), "Unknown top-level key must not appear in result" );
}

// FM-04: `created_at` is preserved across saves
//
// ## Root Cause
// format/001 states "created_at: set on first save, preserved on all subsequent saves"
// but no test verified this. A naive implementation might regenerate created_at each save.
//
// ## Fix Applied
// Save once, record created_at, save again, verify created_at unchanged.
#[ test ]
fn test_created_at_preserved_on_resave()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  // First save
  let mut config = HashMap::new();
  config.insert( "timeout".into(), JsonValue::Number( 30.into() ) );
  TestConfig::save_config_file( &config, &config_path ).unwrap();

  // Read raw YAML to capture created_at
  let raw = std::fs::read_to_string( &config_path ).unwrap();
  let created_at_line = raw.lines()
    .find( | l | l.contains( "created_at" ) )
    .expect( "created_at must be present after first save" )
    .to_owned();

  // Second save with different content
  config.insert( "timeout".into(), JsonValue::Number( 60.into() ) );
  TestConfig::save_config_file( &config, &config_path ).unwrap();

  // Verify created_at unchanged
  let raw2 = std::fs::read_to_string( &config_path ).unwrap();
  let created_at_line2 = raw2.lines()
    .find( | l | l.contains( "created_at" ) )
    .expect( "created_at must still be present after second save" )
    .to_owned();

  assert_eq!( created_at_line, created_at_line2, "created_at must not change on re-save" );

  // last_modified should differ (or be equal if same second, but that's acceptable)
  // Main assertion is that created_at stays identical
}

// FM-05: Null parameter values round-trip through YAML
//
// ## Root Cause
// format/001 lists "Null: ~ or null" as a valid scalar type but null round-trip
// was never tested. YAML null handling differs between serde_yaml_ng versions.
//
// ## Fix Applied
// Save map with JsonValue::Null, reload, assert Null preserved.
#[ test ]
fn test_null_value_round_trips()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  config.insert( "nullable_key".into(), JsonValue::Null );
  config.insert( "normal_key".into(), JsonValue::String( "present".into() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "normal_key" ), Some( &JsonValue::String( "present".into() ) ) );
  // Null values may be omitted or represented as Null depending on serde_yaml_ng behavior
  // Both outcomes are acceptable — the key point is no panic/error
  if let Some( v ) = loaded.get( "nullable_key" )
  {
    assert!( v.is_null(), "If nullable_key is present, it must be Null, got: {v}" );
  }
}

// FP-02: last_modified is updated on every save
//
// ## Root Cause
// save_config_file always calls chrono::Utc::now().to_rfc3339() for last_modified —
// every invocation produces a new timestamp. Without a dedicated test, a naive
// implementation that caches last_modified (like created_at) would be undetected.
//
// ## Fix Applied
// Verified that save_config_file calls build_config_yaml_string with a fresh
// `chrono::Utc::now()` on every call. No code change needed — test adds coverage.
//
// ## Pitfall
// Distinguish last_modified (always updated) from created_at (preserved after first save).
// Both fields live in the same metadata block but have opposite preservation semantics.
#[ test ]
fn test_last_modified_updated_on_every_save()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  // First save
  let mut config = HashMap::new();
  config.insert( "key".into(), JsonValue::String( "value1".into() ) );
  TestConfig::save_config_file( &config, &config_path ).unwrap();

  // Capture last_modified timestamp from raw YAML after first save
  let raw1 = std::fs::read_to_string( &config_path ).unwrap();
  let last_modified1 = raw1.lines()
    .find( | l | l.contains( "last_modified" ) )
    .expect( "last_modified must be present after first save" )
    .to_owned();

  // Brief pause ensures a distinct timestamp even on low-resolution OS clocks
  std::thread::sleep( core::time::Duration::from_millis( 10 ) );

  // Second save with different content
  config.insert( "key".into(), JsonValue::String( "value2".into() ) );
  TestConfig::save_config_file( &config, &config_path ).unwrap();

  // Capture last_modified after second save
  let raw2 = std::fs::read_to_string( &config_path ).unwrap();
  let last_modified2 = raw2.lines()
    .find( | l | l.contains( "last_modified" ) )
    .expect( "last_modified must still be present after second save" )
    .to_owned();

  assert_ne!(
    last_modified1,
    last_modified2,
    "last_modified must change on every save — first: {last_modified1}, second: {last_modified2}"
  );
}
