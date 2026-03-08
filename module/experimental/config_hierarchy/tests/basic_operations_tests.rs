#![ allow( missing_docs ) ]

use config_hierarchy::*;
use std::collections::HashMap;
use serde_json::Value as JsonValue;
use tempfile::TempDir;

// Test implementation of traits
struct TestDefaults;
impl ConfigDefaults for TestDefaults
{
  fn get_defaults() -> HashMap< String, JsonValue >
  {
    let mut map = HashMap::new();
    map.insert( "key1".into(), JsonValue::String( "default1".into() ) );
    map.insert( "key2".into(), JsonValue::Number( 42.into() ) );
    map
  }

  fn get_parameter_names() -> Vec< &'static str >
  {
    vec![ "key1", "key2" ]
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
fn test_load_nonexistent_file()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "nonexistent.yaml" );

  let result = TestConfig::load_config_file( &config_path );
  assert!( result.is_ok() );
  assert!( result.unwrap().is_empty() );
}

#[ test ]
fn test_save_and_load_config()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  config.insert( "test_key".into(), JsonValue::String( "test_value".into() ) );
  config.insert( "number".into(), JsonValue::Number( 123.into() ) );
  config.insert( "flag".into(), JsonValue::Bool( true ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  assert!( config_path.exists() );

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "test_key" ), Some( &JsonValue::String( "test_value".into() ) ) );
  assert_eq!( loaded.get( "number" ), Some( &JsonValue::Number( 123.into() ) ) );
  assert_eq!( loaded.get( "flag" ), Some( &JsonValue::Bool( true ) ) );
}

#[ test ]
fn test_metadata_preserved_on_save()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  config.insert( "key1".into(), JsonValue::String( "value1".into() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let content = std::fs::read_to_string( &config_path ).unwrap();
  assert!( content.contains( "metadata:" ) );
  assert!( content.contains( "version:" ) );
  assert!( content.contains( "created_at:" ) );
  assert!( content.contains( "last_modified:" ) );
  assert!( content.contains( "parameters:" ) );
}

#[ test ]
#[ allow( clippy::std_instead_of_core ) ]
fn test_created_at_preserved_last_modified_updated()
{
  let temp_dir = TempDir::new().unwrap();

  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  config.insert( "key1".into(), JsonValue::String( "value1".into() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let content1 = std::fs::read_to_string( &config_path ).unwrap();
  let yaml1 : serde_yaml::Value = serde_yaml::from_str( &content1 ).unwrap();

  std::thread::sleep( std::time::Duration::from_millis( 10 ) );

  config.insert( "key2".into(), JsonValue::String( "value2".into() ) );
  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let content2 = std::fs::read_to_string( &config_path ).unwrap();
  let yaml2 : serde_yaml::Value = serde_yaml::from_str( &content2 ).unwrap();

  let created1 = yaml1[ "metadata" ][ "created_at" ].as_str().unwrap();
  let created2 = yaml2[ "metadata" ][ "created_at" ].as_str().unwrap();
  assert_eq!( created1, created2, "created_at should be preserved" );

  let modified1 = yaml1[ "metadata" ][ "last_modified" ].as_str().unwrap();
  let modified2 = yaml2[ "metadata" ][ "last_modified" ].as_str().unwrap();
  assert_ne!( modified1, modified2, "last_modified should be updated" );
}

#[ test ]
fn test_delete_config_file()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  config.insert( "key".into(), JsonValue::String( "value".into() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();
  assert!( config_path.exists() );

  let deleted = TestConfig::delete_config_file( &config_path ).unwrap();
  assert!( deleted );
  assert!( !config_path.exists() );

  let deleted_again = TestConfig::delete_config_file( &config_path ).unwrap();
  assert!( !deleted_again );
}

#[ test ]
fn test_get_defaults()
{
  let defaults = TestConfig::get_defaults();

  assert_eq!( defaults.get( "key1" ), Some( &JsonValue::String( "default1".into() ) ) );
  assert_eq!( defaults.get( "key2" ), Some( &JsonValue::Number( 42.into() ) ) );
}

#[ test ]
fn test_empty_config_file()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "empty.yaml" );

  std::fs::write( &config_path, "" ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert!( loaded.is_empty() );
}

#[ test ]
fn test_whitespace_only_config_file()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "whitespace.yaml" );

  std::fs::write( &config_path, "   \n  \n  " ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert!( loaded.is_empty() );
}

#[ test ]
fn test_save_to_nonexistent_file_creates_new_file()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "new_config.yaml" );

  // Verify file doesn't exist
  assert!( !config_path.exists(), "File should not exist before save" );

  let mut config = HashMap::new();
  config.insert( "key".into(), JsonValue::String( "value".into() ) );

  // Save should create new file
  let result = TestConfig::save_config_file( &config, &config_path );
  assert!( result.is_ok(), "Save should succeed for new file" );
  assert!( config_path.exists(), "File should exist after save" );

  // Verify content
  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "key" ), Some( &JsonValue::String( "value".into() ) ) );
}

#[ test ]
fn test_save_to_existing_file_updates_file()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "existing_config.yaml" );

  // Create initial file
  let mut config1 = HashMap::new();
  config1.insert( "key1".into(), JsonValue::String( "value1".into() ) );
  TestConfig::save_config_file( &config1, &config_path ).unwrap();
  assert!( config_path.exists() );

  // Get initial timestamp
  let metadata1 = std::fs::metadata( &config_path ).unwrap();
  let modified1 = metadata1.modified().unwrap();

  std::thread::sleep( core::time::Duration::from_millis( 10 ) );

  // Update file with new content
  let mut config2 = HashMap::new();
  config2.insert( "key2".into(), JsonValue::String( "value2".into() ) );
  let result = TestConfig::save_config_file( &config2, &config_path );
  assert!( result.is_ok(), "Save should succeed for existing file" );
  assert!( config_path.exists(), "File should still exist after update" );

  // Verify content was updated
  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "key2" ), Some( &JsonValue::String( "value2".into() ) ) );
  assert!( !loaded.contains_key( "key1" ), "Old key should be replaced" );

  // Verify file was modified
  let metadata2 = std::fs::metadata( &config_path ).unwrap();
  let modified2 = metadata2.modified().unwrap();
  assert!( modified2 > modified1, "File modification time should be updated" );
}

#[ test ]
#[ cfg( unix ) ]
fn test_save_to_readonly_directory_returns_error()
{
  use std::os::unix::fs::PermissionsExt;

  let temp_dir = TempDir::new().unwrap();
  let readonly_dir = temp_dir.path().join( "readonly" );
  std::fs::create_dir( &readonly_dir ).unwrap();

  // Make directory read-only
  let mut perms = std::fs::metadata( &readonly_dir ).unwrap().permissions();
  perms.set_mode( 0o444 );
  std::fs::set_permissions( &readonly_dir, perms ).unwrap();

  let config_path = readonly_dir.join( "config.yaml" );
  let mut config = HashMap::new();
  config.insert( "key".into(), JsonValue::String( "value".into() ) );

  // Attempt to save should fail
  let result = TestConfig::save_config_file( &config, &config_path );

  // Restore permissions for cleanup
  let mut perms = std::fs::metadata( &readonly_dir ).unwrap().permissions();
  perms.set_mode( 0o755 );
  std::fs::set_permissions( &readonly_dir, perms ).unwrap();

  assert!( result.is_err(), "Save should fail for read-only directory" );
  let error_msg = result.unwrap_err();
  assert!( error_msg.contains( "Failed to write config file" ) ||
           error_msg.contains( "Failed to create config directory" ),
    "Error message should indicate write failure. Got: {error_msg}" );
}

#[ test ]
fn test_save_to_empty_existing_file()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "empty_config.yaml" );

  // Create empty file
  std::fs::write( &config_path, "" ).unwrap();
  assert!( config_path.exists(), "Empty file should exist" );

  let mut config = HashMap::new();
  config.insert( "key".into(), JsonValue::String( "value".into() ) );

  // Save to empty file should succeed
  let result = TestConfig::save_config_file( &config, &config_path );
  assert!( result.is_ok(), "Save should succeed for empty file" );

  // Verify content was written
  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "key" ), Some( &JsonValue::String( "value".into() ) ) );

  // Verify metadata was added
  let content = std::fs::read_to_string( &config_path ).unwrap();
  assert!( content.contains( "metadata:" ) );
  assert!( content.contains( "version:" ) );
}

#[ test ]
fn test_save_when_directory_exists_at_path()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config_as_directory" );

  // Create directory where file should be
  std::fs::create_dir( &config_path ).unwrap();
  assert!( config_path.exists() );
  assert!( config_path.is_dir() );

  let mut config = HashMap::new();
  config.insert( "key".into(), JsonValue::String( "value".into() ) );

  // Attempt to save should fail
  let result = TestConfig::save_config_file( &config, &config_path );
  assert!( result.is_err(), "Save should fail when path is directory" );

  let error_msg = result.unwrap_err();
  assert!( error_msg.contains( "Failed to write config file" ),
    "Error should indicate write failure. Got: {error_msg}" );
}

#[ test ]
fn test_save_with_corrupted_existing_file()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "corrupted.yaml" );

  // Create file with corrupted YAML
  std::fs::write( &config_path, "{ invalid: yaml: content: [ unclosed" ).unwrap();
  assert!( config_path.exists() );

  let mut config = HashMap::new();
  config.insert( "key".into(), JsonValue::String( "value".into() ) );

  // Save should succeed (overwrites corrupted content)
  let result = TestConfig::save_config_file( &config, &config_path );
  assert!( result.is_ok(), "Save should succeed even with corrupted existing file" );

  // Verify new content is valid YAML
  let content = std::fs::read_to_string( &config_path ).unwrap();
  let parsed : Result< serde_yaml::Value, _ > = serde_yaml::from_str( &content );
  assert!( parsed.is_ok(), "Saved content should be valid YAML" );

  // Verify content
  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "key" ), Some( &JsonValue::String( "value".into() ) ) );
}

#[ test ]
#[ cfg( unix ) ]
fn test_save_to_readonly_file()
{
  use std::os::unix::fs::PermissionsExt;

  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "readonly_file.yaml" );

  // Create file first
  let mut initial_config = HashMap::new();
  initial_config.insert( "initial".into(), JsonValue::String( "value".into() ) );
  TestConfig::save_config_file( &initial_config, &config_path ).unwrap();

  // Make file read-only
  let mut perms = std::fs::metadata( &config_path ).unwrap().permissions();
  perms.set_mode( 0o444 );
  std::fs::set_permissions( &config_path, perms ).unwrap();

  // Attempt to save new content
  let mut new_config = HashMap::new();
  new_config.insert( "new".into(), JsonValue::String( "value".into() ) );
  let result = TestConfig::save_config_file( &new_config, &config_path );

  // Restore permissions for cleanup
  let mut perms = std::fs::metadata( &config_path ).unwrap().permissions();
  perms.set_mode( 0o644 );
  std::fs::set_permissions( &config_path, perms ).unwrap();

  assert!( result.is_err(), "Save should fail for read-only file" );
  let error_msg = result.unwrap_err();
  assert!( error_msg.contains( "Failed to write config file" ),
    "Error should indicate write failure. Got: {error_msg}" );
}

#[ test ]
fn test_load_empty_file()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "empty.yaml" );

  // Create empty file
  std::fs::write( &config_path, "" ).unwrap();

  // Load should return empty config
  let result = TestConfig::load_config_file( &config_path );
  assert!( result.is_ok(), "Load should succeed for empty file" );

  let loaded = result.unwrap();
  assert!( loaded.is_empty(), "Loaded config should be empty" );
}

#[ test ]
fn test_load_corrupted_file()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "corrupted.yaml" );

  // Create file with corrupted YAML
  std::fs::write( &config_path, "{ this is: not valid [ yaml" ).unwrap();

  // Load should fail with error
  let result = TestConfig::load_config_file( &config_path );
  assert!( result.is_err(), "Load should fail for corrupted YAML" );

  let error_msg = result.unwrap_err();
  assert!( error_msg.contains( "Failed to parse" ) || error_msg.contains( "YAML" ),
    "Error should mention parsing failure. Got: {error_msg}" );
}
