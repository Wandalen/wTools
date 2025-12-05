//! Comprehensive scope operations tests
//!
//! Tests for local vs global config operations including:
//! - Path resolution for both scopes
//! - File operations (save/load/delete) on both scopes
//! - Atomic operations on both scopes
//! - Concurrent access between scopes
//! - Isolation between local and global configs
//! - Edge cases specific to scope handling

#![ allow( missing_docs ) ]

use config_hierarchy::*;
use std::collections::HashMap;
use serde_json::Value as JsonValue;
use tempfile::TempDir;
use serial_test::serial;

// Test implementation of traits
struct TestDefaults;
impl ConfigDefaults for TestDefaults
{
  fn get_defaults() -> HashMap< String, JsonValue >
  {
    let mut map = HashMap::new();
    map.insert( "default_param".into(), JsonValue::String( "default_value".into() ) );
    map
  }

  fn get_parameter_names() -> Vec< &'static str >
  {
    vec![ "default_param" ]
  }
}

struct TestPaths;
impl ConfigPaths for TestPaths
{
  fn app_name() -> &'static str { "scopetest" }
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

//
// Path Resolution Tests
//

#[ test ]
#[ serial ]
fn test_local_path_resolution_in_current_dir()
{

  let temp_dir = TempDir::new().unwrap();
  let _old_dir = std::env::current_dir().unwrap();
  std::env::set_current_dir( temp_dir.path() ).unwrap();

  let local_path = get_local_config_path::< TestPaths >().unwrap();

  // Should be ./.scopetest/config.yaml relative to current dir
  assert!( local_path.starts_with( temp_dir.path() ) );
  assert!( local_path.to_string_lossy().contains( ".scopetest" ) );
  assert!( local_path.ends_with( "config.yaml" ) );

  std::env::set_current_dir( "/tmp" ).unwrap();
}

#[ test ]
#[ serial ]
fn test_global_path_resolution_with_pro_env()
{
  let temp_pro = TempDir::new().unwrap();
  std::env::set_var( "PRO", temp_pro.path() );

  let global_path = get_global_config_path::< TestPaths >().unwrap();

  // Should be $PRO/.persistent/.scopetest/config.yaml
  assert!( global_path.starts_with( temp_pro.path() ) );
  assert!( global_path.to_string_lossy().contains( ".persistent" ) );
  assert!( global_path.to_string_lossy().contains( ".scopetest" ) );
  assert!( global_path.ends_with( "config.yaml" ) );

  std::env::remove_var( "PRO" );
}

#[ test ]
#[ serial ]
fn test_local_and_global_paths_are_different()
{
  let temp_pro = TempDir::new().unwrap();
  let temp_local = TempDir::new().unwrap();

  std::env::set_var( "PRO", temp_pro.path() );
  let _old_dir = std::env::current_dir().unwrap();
  std::env::set_current_dir( temp_local.path() ).unwrap();

  let local_path = get_local_config_path::< TestPaths >().unwrap();
  let global_path = get_global_config_path::< TestPaths >().unwrap();

  // Paths must be different
  assert_ne!( local_path, global_path, "Local and global paths must be different" );

  // Local should be in temp_local
  assert!( local_path.starts_with( temp_local.path() ) );

  // Global should be in temp_pro
  assert!( global_path.starts_with( temp_pro.path() ) );

  std::env::set_current_dir( "/tmp" ).unwrap();
  std::env::remove_var( "PRO" );
}

//
// File Operations - Local Scope
//

#[ test ]
#[ serial ]
fn test_save_and_load_local_config()
{
  let temp_dir = TempDir::new().unwrap();
  let _old_dir = std::env::current_dir().unwrap();
  std::env::set_current_dir( temp_dir.path() ).unwrap();

  let local_path = get_local_config_path::< TestPaths >().unwrap();

  // Save local config
  let mut config = HashMap::new();
  config.insert( "local_key".into(), JsonValue::String( "local_value".into() ) );

  TestConfig::save_config_file( &config, &local_path ).unwrap();

  // Verify file exists
  assert!( local_path.exists(), "Local config file should exist after save" );

  // Load local config
  let loaded = TestConfig::load_config_file( &local_path ).unwrap();
  assert_eq!( loaded.get( "local_key" ), Some( &JsonValue::String( "local_value".into() ) ) );

  std::env::set_current_dir( "/tmp" ).unwrap();
}

#[ test ]
#[ serial ]
fn test_delete_local_config()
{
  let temp_dir = TempDir::new().unwrap();
  let _old_dir = std::env::current_dir().unwrap();
  std::env::set_current_dir( temp_dir.path() ).unwrap();

  let local_path = get_local_config_path::< TestPaths >().unwrap();

  // Create local config
  let mut config = HashMap::new();
  config.insert( "key".into(), JsonValue::String( "value".into() ) );
  TestConfig::save_config_file( &config, &local_path ).unwrap();
  assert!( local_path.exists() );

  // Delete local config
  let deleted = delete_config_file( &local_path ).unwrap();
  assert!( deleted, "Should return true when file was deleted" );
  assert!( !local_path.exists(), "Local config should be deleted" );

  // Try deleting again
  let deleted_again = delete_config_file( &local_path ).unwrap();
  assert!( !deleted_again, "Should return false when file doesnt exist" );

  std::env::set_current_dir( "/tmp" ).unwrap();
}

//
// File Operations - Global Scope
//

#[ test ]
#[ serial ]
fn test_save_and_load_global_config()
{
  let temp_pro = TempDir::new().unwrap();
  std::env::set_var( "PRO", temp_pro.path() );

  let global_path = get_global_config_path::< TestPaths >().unwrap();

  // Save global config
  let mut config = HashMap::new();
  config.insert( "global_key".into(), JsonValue::String( "global_value".into() ) );

  TestConfig::save_config_file( &config, &global_path ).unwrap();

  // Verify file exists
  assert!( global_path.exists(), "Global config file should exist after save" );

  // Load global config
  let loaded = TestConfig::load_config_file( &global_path ).unwrap();
  assert_eq!( loaded.get( "global_key" ), Some( &JsonValue::String( "global_value".into() ) ) );

  std::env::remove_var( "PRO" );
}

#[ test ]
#[ serial ]
fn test_delete_global_config()
{
  let temp_pro = TempDir::new().unwrap();
  std::env::set_var( "PRO", temp_pro.path() );

  let global_path = get_global_config_path::< TestPaths >().unwrap();

  // Create global config
  let mut config = HashMap::new();
  config.insert( "key".into(), JsonValue::String( "value".into() ) );
  TestConfig::save_config_file( &config, &global_path ).unwrap();
  assert!( global_path.exists() );

  // Delete global config
  let deleted = delete_config_file( &global_path ).unwrap();
  assert!( deleted, "Should return true when file was deleted" );
  assert!( !global_path.exists(), "Global config should be deleted" );

  std::env::remove_var( "PRO" );
}

//
// Isolation Tests - Local and Global Don't Interfere
//

#[ test ]
#[ serial ]
fn test_local_and_global_configs_are_independent()
{
  let temp_pro = TempDir::new().unwrap();
  let temp_local = TempDir::new().unwrap();

  std::env::set_var( "PRO", temp_pro.path() );
  let _old_dir = std::env::current_dir().unwrap();
  std::env::set_current_dir( temp_local.path() ).unwrap();

  let local_path = get_local_config_path::< TestPaths >().unwrap();
  let global_path = get_global_config_path::< TestPaths >().unwrap();

  // Create local config with local_param
  let mut local_config = HashMap::new();
  local_config.insert( "local_param".into(), JsonValue::String( "local_value".into() ) );
  TestConfig::save_config_file( &local_config, &local_path ).unwrap();

  // Create global config with global_param
  let mut global_config = HashMap::new();
  global_config.insert( "global_param".into(), JsonValue::String( "global_value".into() ) );
  TestConfig::save_config_file( &global_config, &global_path ).unwrap();

  // Load and verify local config
  let loaded_local = TestConfig::load_config_file( &local_path ).unwrap();
  assert!( loaded_local.contains_key( "local_param" ) );
  assert!( !loaded_local.contains_key( "global_param" ), "Local config should not contain global params" );

  // Load and verify global config
  let loaded_global = TestConfig::load_config_file( &global_path ).unwrap();
  assert!( loaded_global.contains_key( "global_param" ) );
  assert!( !loaded_global.contains_key( "local_param" ), "Global config should not contain local params" );

  std::env::set_current_dir( "/tmp" ).unwrap();
  std::env::remove_var( "PRO" );
}

#[ test ]
#[ serial ]
fn test_deleting_local_doesnt_affect_global()
{
  let temp_pro = TempDir::new().unwrap();
  let temp_local = TempDir::new().unwrap();

  std::env::set_var( "PRO", temp_pro.path() );
  let _old_dir = std::env::current_dir().unwrap();
  std::env::set_current_dir( temp_local.path() ).unwrap();

  let local_path = get_local_config_path::< TestPaths >().unwrap();
  let global_path = get_global_config_path::< TestPaths >().unwrap();

  // Create both configs
  let mut config = HashMap::new();
  config.insert( "key".into(), JsonValue::String( "value".into() ) );
  TestConfig::save_config_file( &config, &local_path ).unwrap();
  TestConfig::save_config_file( &config, &global_path ).unwrap();

  // Get global content before deletion
  let global_content_before = std::fs::read_to_string( &global_path ).unwrap();

  // Delete local config
  delete_config_file( &local_path ).unwrap();
  assert!( !local_path.exists(), "Local config should be deleted" );

  // Verify global config is untouched
  assert!( global_path.exists(), "Global config should still exist" );
  let global_content_after = std::fs::read_to_string( &global_path ).unwrap();
  assert_eq!( global_content_before, global_content_after, "Global config content should be unchanged" );

  std::env::set_current_dir( "/tmp" ).unwrap();
  std::env::remove_var( "PRO" );
}

#[ test ]
#[ serial ]
fn test_deleting_global_doesnt_affect_local()
{
  let temp_pro = TempDir::new().unwrap();
  let temp_local = TempDir::new().unwrap();

  std::env::set_var( "PRO", temp_pro.path() );
  let _old_dir = std::env::current_dir().unwrap();
  std::env::set_current_dir( temp_local.path() ).unwrap();

  let local_path = get_local_config_path::< TestPaths >().unwrap();
  let global_path = get_global_config_path::< TestPaths >().unwrap();

  // Create both configs
  let mut config = HashMap::new();
  config.insert( "key".into(), JsonValue::String( "value".into() ) );
  TestConfig::save_config_file( &config, &local_path ).unwrap();
  TestConfig::save_config_file( &config, &global_path ).unwrap();

  // Get local content before deletion
  let local_content_before = std::fs::read_to_string( &local_path ).unwrap();

  // Delete global config
  delete_config_file( &global_path ).unwrap();
  assert!( !global_path.exists(), "Global config should be deleted" );

  // Verify local config is untouched
  assert!( local_path.exists(), "Local config should still exist" );
  let local_content_after = std::fs::read_to_string( &local_path ).unwrap();
  assert_eq!( local_content_before, local_content_after, "Local config content should be unchanged" );

  std::env::set_current_dir( "/tmp" ).unwrap();
  std::env::remove_var( "PRO" );
}

//
// Atomic Operations Tests
//

#[ test ]
#[ serial ]
fn test_atomic_modify_local_config()
{
  let temp_dir = TempDir::new().unwrap();
  let _old_dir = std::env::current_dir().unwrap();
  std::env::set_current_dir( temp_dir.path() ).unwrap();

  let local_path = get_local_config_path::< TestPaths >().unwrap();

  // Initialize config
  let mut initial = HashMap::new();
  initial.insert( "counter".into(), JsonValue::Number( 0.into() ) );
  TestConfig::save_config_file( &initial, &local_path ).unwrap();

  // Atomic modify
  atomic_config_modify( &local_path, | config | {
    config.insert( "counter".into(), JsonValue::Number( 1.into() ) );
    config.insert( "new_key".into(), JsonValue::String( "new_value".into() ) );
    Ok( () )
  }).unwrap();

  // Verify changes
  let loaded = TestConfig::load_config_file( &local_path ).unwrap();
  assert_eq!( loaded.get( "counter" ), Some( &JsonValue::Number( 1.into() ) ) );
  assert_eq!( loaded.get( "new_key" ), Some( &JsonValue::String( "new_value".into() ) ) );

  std::env::set_current_dir( "/tmp" ).unwrap();
}

#[ test ]
#[ serial ]
fn test_atomic_modify_global_config()
{
  let temp_pro = TempDir::new().unwrap();
  std::env::set_var( "PRO", temp_pro.path() );

  let global_path = get_global_config_path::< TestPaths >().unwrap();

  // Initialize config
  let mut initial = HashMap::new();
  initial.insert( "counter".into(), JsonValue::Number( 0.into() ) );
  TestConfig::save_config_file( &initial, &global_path ).unwrap();

  // Atomic modify
  atomic_config_modify( &global_path, | config | {
    config.insert( "counter".into(), JsonValue::Number( 1.into() ) );
    config.insert( "new_key".into(), JsonValue::String( "new_value".into() ) );
    Ok( () )
  }).unwrap();

  // Verify changes
  let loaded = TestConfig::load_config_file( &global_path ).unwrap();
  assert_eq!( loaded.get( "counter" ), Some( &JsonValue::Number( 1.into() ) ) );
  assert_eq!( loaded.get( "new_key" ), Some( &JsonValue::String( "new_value".into() ) ) );

  std::env::remove_var( "PRO" );
}

#[ test ]
#[ serial ]
fn test_atomic_modify_creates_file_if_not_exists_local()
{
  let temp_dir = TempDir::new().unwrap();
  let _old_dir = std::env::current_dir().unwrap();
  std::env::set_current_dir( temp_dir.path() ).unwrap();

  let local_path = get_local_config_path::< TestPaths >().unwrap();

  // File doesnt exist yet
  assert!( !local_path.exists() );

  // Atomic modify should create it
  atomic_config_modify( &local_path, | config | {
    config.insert( "key".into(), JsonValue::String( "value".into() ) );
    Ok( () )
  }).unwrap();

  // Verify file was created
  assert!( local_path.exists() );
  let loaded = TestConfig::load_config_file( &local_path ).unwrap();
  assert_eq!( loaded.get( "key" ), Some( &JsonValue::String( "value".into() ) ) );

  std::env::set_current_dir( "/tmp" ).unwrap();
}

#[ test ]
#[ serial ]
fn test_atomic_modify_creates_file_if_not_exists_global()
{
  let temp_pro = TempDir::new().unwrap();
  std::env::set_var( "PRO", temp_pro.path() );

  let global_path = get_global_config_path::< TestPaths >().unwrap();

  // File doesnt exist yet
  assert!( !global_path.exists() );

  // Atomic modify should create it
  atomic_config_modify( &global_path, | config | {
    config.insert( "key".into(), JsonValue::String( "value".into() ) );
    Ok( () )
  }).unwrap();

  // Verify file was created
  assert!( global_path.exists() );
  let loaded = TestConfig::load_config_file( &global_path ).unwrap();
  assert_eq!( loaded.get( "key" ), Some( &JsonValue::String( "value".into() ) ) );

  std::env::remove_var( "PRO" );
}

//
// Concurrent Access Between Scopes
//

#[ test ]
#[ serial ]
fn test_concurrent_access_to_different_scopes()
{
  use std::sync::Arc;
  use std::thread;

  let temp_pro = TempDir::new().unwrap();
  let temp_local = TempDir::new().unwrap();

  // Share paths between threads
  let pro_path = Arc::new( temp_pro.path().to_path_buf() );
  let local_path_base = Arc::new( temp_local.path().to_path_buf() );

  let pro_path_clone = Arc::clone( &pro_path );
  let local_path_clone = Arc::clone( &local_path_base );

  // Thread 1: Write to global config
  let thread1 = thread::spawn( move || {
    std::env::set_var( "PRO", pro_path_clone.as_path() );
    let global_path = get_global_config_path::< TestPaths >().unwrap();

    for i in 0..10
    {
      atomic_config_modify( &global_path, move | config | {
        config.insert( format!( "global_key{i}" ), JsonValue::Number( i.into() ) );
        Ok( () )
      }).unwrap();
    }

    std::env::remove_var( "PRO" );
  });

  // Thread 2: Write to local config
  let thread2 = thread::spawn( move || {
    let _old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir( local_path_clone.as_path() ).unwrap();
    let local_path = get_local_config_path::< TestPaths >().unwrap();

    for i in 0..10
    {
      atomic_config_modify( &local_path, move | config | {
        config.insert( format!( "local_key{i}" ), JsonValue::Number( i.into() ) );
        Ok( () )
      }).unwrap();
    }

    std::env::set_current_dir( "/tmp" ).unwrap();
  });

  // Wait for both threads
  thread1.join().unwrap();
  thread2.join().unwrap();

  // Verify both configs exist and have correct data
  std::env::set_var( "PRO", pro_path.as_path() );
  let _old_dir = std::env::current_dir().unwrap();
  std::env::set_current_dir( local_path_base.as_path() ).unwrap();

  let global_path = get_global_config_path::< TestPaths >().unwrap();
  let local_path = get_local_config_path::< TestPaths >().unwrap();

  let global_config = TestConfig::load_config_file( &global_path ).unwrap();
  let local_config = TestConfig::load_config_file( &local_path ).unwrap();

  // Verify all global keys exist
  for i in 0..10
  {
    assert!( global_config.contains_key( &format!( "global_key{i}" ) ), "Missing global_key{i}" );
  }

  // Verify all local keys exist
  for i in 0..10
  {
    assert!( local_config.contains_key( &format!( "local_key{i}" ) ), "Missing local_key{i}" );
  }

  // Verify no cross-contamination
  assert!( !local_config.contains_key( "global_key0" ), "Local should not have global keys" );
  assert!( !global_config.contains_key( "local_key0" ), "Global should not have local keys" );

  std::env::set_current_dir( "/tmp" ).unwrap();
  std::env::remove_var( "PRO" );
}

//
// Edge Cases
//

#[ test ]
#[ serial ]
fn test_local_config_in_nested_directory()
{
  let temp_dir = TempDir::new().unwrap();
  let nested = temp_dir.path().join( "a" ).join( "b" ).join( "c" );
  std::fs::create_dir_all( &nested ).unwrap();

  let _old_dir = std::env::current_dir().unwrap();
  std::env::set_current_dir( &nested ).unwrap();

  let local_path = get_local_config_path::< TestPaths >().unwrap();

  // Should be in nested directory
  assert!( local_path.starts_with( &nested ) );

  // Should be able to save and load
  let mut config = HashMap::new();
  config.insert( "nested_key".into(), JsonValue::String( "nested_value".into() ) );
  TestConfig::save_config_file( &config, &local_path ).unwrap();

  assert!( local_path.exists() );
  let loaded = TestConfig::load_config_file( &local_path ).unwrap();
  assert_eq!( loaded.get( "nested_key" ), Some( &JsonValue::String( "nested_value".into() ) ) );

  std::env::set_current_dir( "/tmp" ).unwrap();
}

#[ test ]
#[ serial ]
fn test_multiple_local_configs_in_hierarchy()
{
  let temp_dir = TempDir::new().unwrap();

  // Create directory hierarchy: root/child/grandchild
  let root = temp_dir.path();
  let child = root.join( "child" );
  let grandchild = child.join( "grandchild" );
  std::fs::create_dir_all( &grandchild ).unwrap();

  // Create local config in root
  let root_config_dir = root.join( ".scopetest" );
  std::fs::create_dir( &root_config_dir ).unwrap();
  let root_config = root_config_dir.join( "config.yaml" );
  let mut config = HashMap::new();
  config.insert( "level".into(), JsonValue::String( "root".into() ) );
  TestConfig::save_config_file( &config, &root_config ).unwrap();

  // Create local config in child
  let child_config_dir = child.join( ".scopetest" );
  std::fs::create_dir_all( &child_config_dir ).unwrap();
  let child_config = child_config_dir.join( "config.yaml" );
  let mut config = HashMap::new();
  config.insert( "level".into(), JsonValue::String( "child".into() ) );
  TestConfig::save_config_file( &config, &child_config ).unwrap();

  // Change to grandchild directory
  let _old_dir = std::env::current_dir().unwrap();
  std::env::set_current_dir( &grandchild ).unwrap();

  // Discover configs
  let discovered = discover_local_configs::< TestPaths >();

  // Should find at least the 2 configs we created (may find more if /tmp has configs from previous runs)
  assert!( discovered.len() >= 2, "Should discover at least 2 local configs in hierarchy, found {}", discovered.len() );

  // First two should be our configs in correct order (child, then root)
  assert!( discovered[ 0 ].to_string_lossy().contains( "child" ), "First config should be from child directory" );
  assert!( discovered[ 1 ].starts_with( root ), "Second config should be from root directory" );

  // All discovered paths should be valid
  for path in &discovered
  {
    assert!( path.exists(), "Discovered path should exist: {}", path.display() );
    assert!( path.to_string_lossy().contains( ".scopetest" ), "Path should contain .scopetest" );
  }

  std::env::set_current_dir( "/tmp" ).unwrap();
}

#[ test ]
#[ serial ]
fn test_empty_local_and_global_configs_dont_interfere()
{
  let temp_pro = TempDir::new().unwrap();
  let temp_local = TempDir::new().unwrap();

  std::env::set_var( "PRO", temp_pro.path() );
  let _old_dir = std::env::current_dir().unwrap();
  std::env::set_current_dir( temp_local.path() ).unwrap();

  let local_path = get_local_config_path::< TestPaths >().unwrap();
  let global_path = get_global_config_path::< TestPaths >().unwrap();

  // Create empty configs
  let empty = HashMap::new();
  TestConfig::save_config_file( &empty, &local_path ).unwrap();
  TestConfig::save_config_file( &empty, &global_path ).unwrap();

  // Load and verify both are empty
  let local_loaded = TestConfig::load_config_file( &local_path ).unwrap();
  let global_loaded = TestConfig::load_config_file( &global_path ).unwrap();

  assert!( local_loaded.is_empty() );
  assert!( global_loaded.is_empty() );

  // Both files should exist and be valid
  assert!( local_path.exists() );
  assert!( global_path.exists() );

  std::env::set_current_dir( "/tmp" ).unwrap();
  std::env::remove_var( "PRO" );
}

#[ test ]
#[ serial ]
fn test_overwrite_local_multiple_times()
{
  let temp_dir = TempDir::new().unwrap();
  let _old_dir = std::env::current_dir().unwrap();
  std::env::set_current_dir( temp_dir.path() ).unwrap();

  let local_path = get_local_config_path::< TestPaths >().unwrap();

  // Write config multiple times with different values
  for i in 0..5
  {
    let mut config = HashMap::new();
    config.insert( "iteration".into(), JsonValue::Number( i.into() ) );
    TestConfig::save_config_file( &config, &local_path ).unwrap();

    // Verify current value
    let loaded = TestConfig::load_config_file( &local_path ).unwrap();
    assert_eq!( loaded.get( "iteration" ), Some( &JsonValue::Number( i.into() ) ) );
  }

  // Final check - should have last value
  let loaded = TestConfig::load_config_file( &local_path ).unwrap();
  assert_eq!( loaded.get( "iteration" ), Some( &JsonValue::Number( 4.into() ) ) );

  std::env::set_current_dir( "/tmp" ).unwrap();
}

#[ test ]
#[ serial ]
fn test_overwrite_global_multiple_times()
{
  let temp_pro = TempDir::new().unwrap();
  std::env::set_var( "PRO", temp_pro.path() );

  let global_path = get_global_config_path::< TestPaths >().unwrap();

  // Write config multiple times with different values
  for i in 0..5
  {
    let mut config = HashMap::new();
    config.insert( "iteration".into(), JsonValue::Number( i.into() ) );
    TestConfig::save_config_file( &config, &global_path ).unwrap();

    // Verify current value
    let loaded = TestConfig::load_config_file( &global_path ).unwrap();
    assert_eq!( loaded.get( "iteration" ), Some( &JsonValue::Number( i.into() ) ) );
  }

  // Final check - should have last value
  let loaded = TestConfig::load_config_file( &global_path ).unwrap();
  assert_eq!( loaded.get( "iteration" ), Some( &JsonValue::Number( 4.into() ) ) );

  std::env::remove_var( "PRO" );
}
