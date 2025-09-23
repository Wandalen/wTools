//! Test Matrix: Serde Integration
//!
//! | Test ID | Feature | Scenario | Expected Result |
//! |---------|---------|----------|-----------------|
//! | SI001   | load_config | Load TOML configuration | Success with deserialized data |
//! | SI002   | load_config | Load JSON configuration | Success with deserialized data |
//! | SI003   | load_config | Load YAML configuration | Success with deserialized data |
//! | SI004   | load_config | Config file not found | Error |
//! | SI005   | load_config_from | Load from specific file path | Success |
//! | SI006   | save_config | Save configuration as TOML | Success, file created |
//! | SI007   | save_config_to | Save to specific path with format detection | Success |
//! | SI008   | load_config_layered | Merge multiple config layers | Success with merged data |
//! | SI009   | update_config | Partial configuration update | Success with updated config |
//! | SI010   | WorkspacePath | Serialize and deserialize workspace paths | Success |

#![ cfg( feature = "serde" ) ]

use workspace_tools :: { Workspace, WorkspaceError, ConfigMerge, WorkspacePath };
use serde :: { Serialize, Deserialize };
use std ::fs;
use tempfile ::TempDir;

#[ derive( Debug, Clone, PartialEq, Serialize, Deserialize ) ]
struct TestConfig
{
  name: String,
  port: u16,
  features: Vec< String >,
  database: DatabaseConfig,
}

#[ derive( Debug, Clone, PartialEq, Serialize, Deserialize ) ]
struct DatabaseConfig
{
  host: String,
  port: u16,
  name: String,
}

impl ConfigMerge for TestConfig
{
  fn merge( mut self, other: Self ) -> Self
  {
  // simple merge strategy - other overwrites self
  self.name = other.name;
  self.port = other.port;
  self.features.extend( other.features );
  self.database = other.database;
  self
 }
}

/// Test SI001: Load TOML configuration
#[ test ]
fn test_load_config_toml()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_config();
  
  let result: Result< TestConfig, WorkspaceError > = workspace.load_config( "app" );
  
  assert!( result.is_ok() );
  let config = result.unwrap();
  assert_eq!( config.name, "test_app" );
  assert_eq!( config.port, 8080 );
}

/// Test SI002: Load JSON configuration  
#[ test ]
fn test_load_config_json()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_json_config();
  let json_path = workspace.config_dir().join( "app.json" );
  
  let result: Result< TestConfig, WorkspaceError > = workspace.load_config_from( json_path );
  
  assert!( result.is_ok() );
  let config = result.unwrap();
  assert_eq!( config.name, "json_app" );
  assert_eq!( config.port, 3000 );
}

/// Test SI003: Load YAML configuration
#[ test ]  
fn test_load_config_yaml()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_yaml_config();
  let yaml_path = workspace.config_dir().join( "app.yaml" );
  
  let result: Result< TestConfig, WorkspaceError > = workspace.load_config_from( yaml_path );
  
  assert!( result.is_ok() );
  let config = result.unwrap();
  assert_eq!( config.name, "yaml_app" );
  assert_eq!( config.port, 5000 );
}

/// Test SI004: Config file not found
#[ test ]
fn test_load_config_not_found()
{
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let result: Result< TestConfig, WorkspaceError > = workspace.load_config( "nonexistent" );
  
  assert!( result.is_err() );
  assert!( matches!( result.unwrap_err(), WorkspaceError ::PathNotFound( _ ) ) );
}

/// Test SI005: Load from specific file path
#[ test ]
fn test_load_config_from()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_config();
  let config_path = workspace.config_dir().join( "app.toml" );
  
  let result: Result< TestConfig, WorkspaceError > = workspace.load_config_from( config_path );
  
  assert!( result.is_ok() );
  let config = result.unwrap();
  assert_eq!( config.name, "test_app" );
}

/// Test SI006: Save configuration as TOML
#[ test ]
fn test_save_config()
{
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let config = TestConfig {
  name: "saved_app".to_string(),
  port: 9090,
  features: vec![ "auth".to_string(), "logging".to_string() ],
  database: DatabaseConfig {
   host: "localhost".to_string(),
   port: 5432,
   name: "test_db".to_string(),
 },
 };
  
  let result = workspace.save_config( "saved", &config );
  
  assert!( result.is_ok() );
  
  // verify file was created
  let config_path = workspace.config_dir().join( "saved.toml" );
  assert!( config_path.exists() );
  
  // verify we can load it back
  let loaded: TestConfig = workspace.load_config_from( config_path ).unwrap();
  assert_eq!( loaded, config );
}

/// Test SI007: Save to specific path with format detection
#[ test ]
fn test_save_config_to()
{
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let config = TestConfig {
  name: "json_saved".to_string(),
  port: 4040,
  features: vec![ "metrics".to_string() ],
  database: DatabaseConfig {
   host: "127.0.0.1".to_string(),
   port: 3306,
   name: "metrics_db".to_string(),
 },
 };
  
  let json_path = workspace.config_dir().join( "custom.json" );
  let result = workspace.save_config_to( &json_path, &config );
  
  assert!( result.is_ok() );
  assert!( json_path.exists() );
  
  // verify it's valid JSON
  let content = fs ::read_to_string( &json_path ).unwrap();
  let parsed: serde_json ::Value = serde_json ::from_str( &content ).unwrap();
  assert_eq!( parsed[ "name" ], "json_saved" );
}

/// Test SI008: Merge multiple config layers
#[ test ]
#[ cfg( test ) ]
fn test_load_config_layered()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_layered_configs();
  
  let result: Result< TestConfig, WorkspaceError > = workspace.load_config_layered( &[ "base", "override" ] );
  
  assert!( result.is_ok() );
  let config = result.unwrap();
  
  // should have base config with overridden values
  assert_eq!( config.name, "overridden_app" ); // from override
  assert_eq!( config.port, 8080 ); // from base
  assert!( config.features.contains( &"base_feature".to_string() ) ); // from base
  assert!( config.features.contains( &"override_feature".to_string() ) ); // from override
}

/// Test SI009: Partial configuration update
#[ test ]
fn test_update_config()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_config();
  
  // create update data using serde_json ::Value
  let updates = serde_json ::json!({
  "port" : 9999,
  "name" : "updated_app"
 });
  
  let result: Result< TestConfig, WorkspaceError > = workspace.update_config( "app", updates );
  
  assert!( result.is_ok() );
  let updated_config = result.unwrap();
  assert_eq!( updated_config.name, "updated_app" );
  assert_eq!( updated_config.port, 9999 );
  // other fields should remain unchanged
  assert_eq!( updated_config.database.host, "localhost" );
}

/// Test SI010: Serialize and deserialize workspace paths
#[ test ]
fn test_workspace_path_serde()
{
  use std ::path ::PathBuf;
  
  let original_path = WorkspacePath( PathBuf ::from( "/test/path" ) );
  
  // serialize to JSON
  let serialized = serde_json ::to_string( &original_path ).unwrap();
  assert!( serialized.contains( "/test/path" ) );
  
  // deserialize back
  let deserialized: WorkspacePath = serde_json ::from_str( &serialized ).unwrap();
  assert_eq!( deserialized, original_path );
}

/// Helper function to create test workspace with proper cleanup
fn create_test_workspace() -> ( TempDir, Workspace )
{
  let temp_dir = TempDir ::new().unwrap();
  
  // Create workspace directly with temp directory path to avoid environment variable issues
  let workspace = Workspace ::new( temp_dir.path() );
  
  // Create config directory within temp directory to avoid creating permanent directories
  let config_dir = workspace.config_dir();
  fs ::create_dir_all( &config_dir ).unwrap();
  
  ( temp_dir, workspace )
}

/// Helper function to create test workspace with TOML config
fn create_test_workspace_with_config() -> ( TempDir, Workspace )
{
  let ( temp_dir, workspace ) = create_test_workspace();
  
  let config = r#"
name = "test_app"
port = 8080
features = [ "auth", "logging" ]

[database]
host = "localhost"
port = 5432
name = "app_db"
"#;

  fs ::write( workspace.config_dir().join( "app.toml" ), config ).unwrap();
  
  ( temp_dir, workspace )
}

/// Helper function to create test workspace with JSON config
fn create_test_workspace_with_json_config() -> ( TempDir, Workspace )
{
  let ( temp_dir, workspace ) = create_test_workspace();
  
  let config = r#"{
  "name" : "json_app",
  "port" : 3000,
  "features" : [ "metrics", "health_check" ],
  "database" : {
  "host" : "db.example.com",
  "port" : 5432,
  "name" : "prod_db"
 }
}"#;

  fs ::write( workspace.config_dir().join( "app.json" ), config ).unwrap();
  
  ( temp_dir, workspace )
}

/// Helper function to create test workspace with YAML config
fn create_test_workspace_with_yaml_config() -> ( TempDir, Workspace )
{
  let ( temp_dir, workspace ) = create_test_workspace();
  
  let config = r"
name: yaml_app
port: 5000
features :
  - tracing
  - cors
database :
  host: yaml.db.com
  port: 5432
  name: yaml_db
";

  fs ::write( workspace.config_dir().join( "app.yaml" ), config ).unwrap();
  
  ( temp_dir, workspace )
}

/// Helper function to create workspace with layered configs
fn create_test_workspace_with_layered_configs() -> ( TempDir, Workspace )
{
  let ( temp_dir, workspace ) = create_test_workspace();
  
  // base config
  let base_config = r#"
name = "base_app"
port = 8080
features = [ "base_feature" ]

[database]
host = "localhost"
port = 5432
name = "base_db"
"#;

  fs ::write( workspace.config_dir().join( "base.toml" ), base_config ).unwrap();
  
  // override config - must be complete for TOML parsing
  let override_config = r#"
name = "overridden_app"
port = 8080
features = [ "override_feature" ]

[database]
host = "localhost"
port = 5432
name = "override_db"
"#;

  fs ::write( workspace.config_dir().join( "override.toml" ), override_config ).unwrap();
  
  ( temp_dir, workspace )
}