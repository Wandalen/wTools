//! Feature Combination Tests for workspace_tools
//!
//! ## Test Matrix: Feature Combination Coverage
//!
//! | Test ID | Features | Scenario | Expected Behavior |
//! |---------|----------|----------|-------------------|
//! | FC.1 | cargo + serde | Load config from cargo workspace | Success |
//! | FC.2 | glob + secret_management | Find secret files with patterns | Success |
//! | FC.3 | cargo + glob | Find resources in cargo workspace | Success |
//! | FC.4 | serde + secret_management | Config with secrets | Success |
//! | FC.5 | All features | Full integration scenario | All work together |
//! | FC.6 | No features (minimal) | Basic workspace operations | Core works |
//! | FC.7 | cargo + serde + secrets | Complete workspace setup | Full functionality |
//! | FC.8 | Performance | All features enabled | No significant overhead |

use workspace_tools::{ Workspace, WorkspaceError };
use std::{ env, fs };
use tempfile::TempDir;

/// Test FC.1: Cargo + Serde integration
#[ cfg( all( feature = "cargo_integration", feature = "serde_integration" ) ) ]
#[ test ]
fn test_cargo_serde_integration()
{
  use serde::{ Serialize, Deserialize };
  
  #[ derive( Debug, Serialize, Deserialize, PartialEq ) ]
  struct ProjectConfig
  {
    name : String,
    version : String,
    features : Vec< String >,
  }
  
  let temp_dir = TempDir::new().unwrap();
  
  // Create a cargo workspace
  let cargo_toml = r#"
[workspace]
members = [ "test_crate" ]

[workspace.package]
version = "0.1.0"
edition = "2021"
"#;
  fs::write( temp_dir.path().join( "Cargo.toml" ), cargo_toml ).unwrap();
  
  // Create a test crate member
  let member_dir = temp_dir.path().join( "test_crate" );
  fs::create_dir_all( member_dir.join( "src" ) ).unwrap();
  fs::write( member_dir.join( "Cargo.toml" ), r#"
[package]
name = "test_crate"
version.workspace = true
edition.workspace = true
"# ).unwrap();
  fs::write( member_dir.join( "src/lib.rs" ), "// test crate" ).unwrap();
  
  // Create workspace using cargo integration
  let workspace = Workspace::from_cargo_manifest( temp_dir.path().join( "Cargo.toml" ) ).unwrap();
  
  // Create config directory
  fs::create_dir_all( workspace.config_dir() ).unwrap();
  
  // Test serde functionality within cargo workspace
  let config = ProjectConfig {
    name : "test_project".to_string(),
    version : "0.1.0".to_string(),
    features : vec![ "default".to_string(), "serde".to_string() ],
  };
  
  // Save config using serde
  let save_result = workspace.save_config( "project", &config );
  assert!( save_result.is_ok(), "Should save config in cargo workspace" );
  
  // Load config using serde
  let loaded : Result< ProjectConfig, WorkspaceError > = workspace.load_config( "project" );
  assert!( loaded.is_ok(), "Should load config from cargo workspace" );
  assert_eq!( loaded.unwrap(), config );
  
  // Verify cargo metadata works
  let metadata = workspace.cargo_metadata();
  if let Err( ref e ) = metadata
  {
    println!( "Cargo metadata error: {}", e );
  }
  assert!( metadata.is_ok(), "Should get cargo metadata" );
}

/// Test FC.2: Glob + Secret Management integration
#[ cfg( all( feature = "glob", feature = "secret_management" ) ) ]
#[ test ]
fn test_glob_secret_management_integration()
{
  let temp_dir = TempDir::new().unwrap();
  // Save original state and set workspace path
  let original = env::var( "WORKSPACE_PATH" ).ok();
  env::set_var( "WORKSPACE_PATH", temp_dir.path() );
  
  let workspace = Workspace::resolve().unwrap();
  
  // Restore state
  match original
  {
    Some( value ) => env::set_var( "WORKSPACE_PATH", value ),
    None => env::remove_var( "WORKSPACE_PATH" ),
  }
  
  // Create secret directory structure
  fs::create_dir_all( workspace.secret_dir() ).unwrap();
  
  // Create multiple secret files
  let secret_files = vec![
    ( "api.env", "API_KEY=secret123\nDATABASE_URL=postgres://localhost\n" ),
    ( "auth.env", "JWT_SECRET=jwt456\nOAUTH_CLIENT=oauth789\n" ),
    ( "config.env", "DEBUG=true\nLOG_LEVEL=info\n" ),
  ];
  
  for ( filename, content ) in &secret_files
  {
    fs::write( workspace.secret_dir().join( filename ), content ).unwrap();
  }
  
  // Use glob to find all secret files
  let secret_pattern = format!( "{}/*.env", workspace.secret_dir().display() );
  let found_files = workspace.find_resources( &secret_pattern );
  
  assert!( found_files.is_ok(), "Should find secret files with glob pattern" );
  let files = found_files.unwrap();
  assert_eq!( files.len(), 3, "Should find all 3 secret files" );
  
  // Load secrets from found files
  for file in &files
  {
    if let Some( filename ) = file.file_name()
    {
      let secrets = workspace.load_secrets_from_file( &filename.to_string_lossy() );
      assert!( secrets.is_ok(), "Should load secrets from file: {:?}", filename );
      assert!( !secrets.unwrap().is_empty(), "Secret file should not be empty" );
    }
  }
  
  // Test loading specific keys
  let api_key = workspace.load_secret_key( "API_KEY", "api.env" );
  assert!( api_key.is_ok(), "Should load API_KEY from api.env" );
  assert_eq!( api_key.unwrap(), "secret123" );
}

/// Test FC.3: Cargo + Glob integration  
#[ cfg( all( feature = "cargo_integration", feature = "glob" ) ) ]
#[ test ]
fn test_cargo_glob_integration()
{
  let temp_dir = TempDir::new().unwrap();
  
  // Create cargo workspace with members
  let cargo_toml = r#"
[workspace]
members = [ "lib1", "lib2" ]

[workspace.package]
version = "0.1.0"
edition = "2021"
"#;
  fs::write( temp_dir.path().join( "Cargo.toml" ), cargo_toml ).unwrap();
  
  // Create workspace members
  for member in [ "lib1", "lib2" ]
  {
    let member_dir = temp_dir.path().join( member );
    fs::create_dir_all( member_dir.join( "src" ) ).unwrap();
    
    let member_cargo = format!( r#"
[package]
name = "{}"
version.workspace = true
edition.workspace = true
"#, member );
    fs::write( member_dir.join( "Cargo.toml" ), member_cargo ).unwrap();
    fs::write( member_dir.join( "src/lib.rs" ), "// library code" ).unwrap();
  }
  
  let workspace = Workspace::from_cargo_manifest( temp_dir.path().join( "Cargo.toml" ) ).unwrap();
  
  // Use glob to find all Cargo.toml files
  let cargo_files = workspace.find_resources( "**/Cargo.toml" );
  assert!( cargo_files.is_ok(), "Should find Cargo.toml files" );
  
  let files = cargo_files.unwrap();
  assert!( files.len() >= 3, "Should find at least workspace + member Cargo.toml files" );
  
  // Use glob to find all Rust source files
  let rust_files = workspace.find_resources( "**/*.rs" );
  assert!( rust_files.is_ok(), "Should find Rust source files" );
  
  let rs_files = rust_files.unwrap();
  assert!( rs_files.len() >= 2, "Should find at least member lib.rs files" );
  
  // Verify cargo workspace members
  let members = workspace.workspace_members();
  assert!( members.is_ok(), "Should get workspace members" );
  assert_eq!( members.unwrap().len(), 2, "Should have 2 workspace members" );
}

/// Test FC.4: Serde + Secret Management integration
#[ cfg( all( feature = "serde_integration", feature = "secret_management" ) ) ]
#[ test ]
fn test_serde_secret_management_integration()
{
  use serde::{ Serialize, Deserialize };
  
  #[ derive( Debug, Serialize, Deserialize, PartialEq ) ]
  struct DatabaseConfig
  {
    host : String,
    port : u16,
    username : String,
    password : String,
  }
  
  let temp_dir = TempDir::new().unwrap();
  // Save original state and set workspace path
  let original = env::var( "WORKSPACE_PATH" ).ok();
  env::set_var( "WORKSPACE_PATH", temp_dir.path() );
  
  let workspace = Workspace::resolve().unwrap();
  
  // Restore state
  match original
  {
    Some( value ) => env::set_var( "WORKSPACE_PATH", value ),
    None => env::remove_var( "WORKSPACE_PATH" ),
  }
  
  // Create directories
  fs::create_dir_all( workspace.config_dir() ).unwrap();
  fs::create_dir_all( workspace.secret_dir() ).unwrap();
  
  // Create secret file with database password
  let secret_content = "DB_PASSWORD=super_secret_password\nDB_USERNAME=admin\n";
  fs::write( workspace.secret_dir().join( "database.env" ), secret_content ).unwrap();
  
  // Load secrets
  let username = workspace.load_secret_key( "DB_USERNAME", "database.env" ).unwrap();
  let password = workspace.load_secret_key( "DB_PASSWORD", "database.env" ).unwrap();
  
  // Create config with secrets
  let db_config = DatabaseConfig {
    host : "localhost".to_string(),
    port : 5432,
    username,
    password,
  };
  
  // Save config using serde
  let save_result = workspace.save_config( "database", &db_config );
  assert!( save_result.is_ok(), "Should save database config" );
  
  // Load config using serde
  let loaded : Result< DatabaseConfig, WorkspaceError > = workspace.load_config( "database" );
  assert!( loaded.is_ok(), "Should load database config" );
  
  let loaded_config = loaded.unwrap();
  assert_eq!( loaded_config.username, "admin" );
  assert_eq!( loaded_config.password, "super_secret_password" );
  assert_eq!( loaded_config, db_config );
}

/// Test FC.5: All features integration
#[ cfg( all( 
  feature = "cargo_integration", 
  feature = "serde_integration", 
  feature = "glob", 
  feature = "secret_management"
) ) ]
#[ test ]
fn test_all_features_integration()
{
  use serde::{ Serialize, Deserialize };
  
  #[ derive( Debug, Serialize, Deserialize, PartialEq ) ]
  struct FullConfig
  {
    project_name : String,
    database_url : String,
    api_keys : Vec< String >,
    debug_mode : bool,
  }
  
  let temp_dir = TempDir::new().unwrap();
  
  // Create cargo workspace
  let cargo_toml = r#"
[workspace]
members = [ "app" ]

[workspace.package]  
version = "0.2.0"
edition = "2021"
"#;
  fs::write( temp_dir.path().join( "Cargo.toml" ), cargo_toml ).unwrap();
  
  // Create app member
  let app_dir = temp_dir.path().join( "app" );
  fs::create_dir_all( app_dir.join( "src" ) ).unwrap();
  fs::write( app_dir.join( "Cargo.toml" ), r#"
[package]
name = "app"
version.workspace = true
edition.workspace = true
"# ).unwrap();
  fs::write( app_dir.join( "src/main.rs" ), "fn main() {}" ).unwrap();
  
  // Create workspace from cargo
  let workspace = Workspace::from_cargo_manifest( temp_dir.path().join( "Cargo.toml" ) ).unwrap();
  
  // Create all necessary directories
  fs::create_dir_all( workspace.config_dir() ).unwrap();
  fs::create_dir_all( workspace.secret_dir() ).unwrap();
  
  // Create secret files
  let api_secrets = "API_KEY_1=key123\nAPI_KEY_2=key456\nDATABASE_URL=postgres://user:pass@localhost/db\n";
  fs::write( workspace.secret_dir().join( "api.env" ), api_secrets ).unwrap();
  
  // Load secrets
  let db_url = workspace.load_secret_key( "DATABASE_URL", "api.env" ).unwrap();
  let api_key_1 = workspace.load_secret_key( "API_KEY_1", "api.env" ).unwrap();
  let api_key_2 = workspace.load_secret_key( "API_KEY_2", "api.env" ).unwrap();
  
  // Create full configuration
  let config = FullConfig {
    project_name : "integration_test".to_string(),
    database_url : db_url,
    api_keys : vec![ api_key_1, api_key_2 ],
    debug_mode : true,
  };
  
  // Save using serde
  let save_result = workspace.save_config( "full_app", &config );
  assert!( save_result.is_ok(), "Should save full configuration" );
  
  // Use glob to find all config files
  let config_pattern = format!( "{}/*.toml", workspace.config_dir().display() );
  let config_files = workspace.find_resources( &config_pattern );
  assert!( config_files.is_ok(), "Should find config files" );
  assert!( !config_files.unwrap().is_empty(), "Should have config files" );
  
  // Use glob to find all secret files  
  let secret_pattern = format!( "{}/*.env", workspace.secret_dir().display() );
  let secret_files = workspace.find_resources( &secret_pattern );
  assert!( secret_files.is_ok(), "Should find secret files" );
  assert!( !secret_files.unwrap().is_empty(), "Should have secret files" );
  
  // Load config back
  let loaded : Result< FullConfig, WorkspaceError > = workspace.load_config( "full_app" );
  assert!( loaded.is_ok(), "Should load full configuration" );
  assert_eq!( loaded.unwrap(), config );
  
  // Verify cargo functionality
  let metadata = workspace.cargo_metadata();
  assert!( metadata.is_ok(), "Should get cargo metadata" );
  
  let members = workspace.workspace_members();
  assert!( members.is_ok(), "Should get workspace members" );
  assert_eq!( members.unwrap().len(), 1, "Should have 1 member" );
}

/// Test FC.6: Minimal functionality (no optional features)
#[ test ]
fn test_minimal_functionality()
{
  let temp_dir = TempDir::new().unwrap();
  // Save original state and set workspace path
  let original = env::var( "WORKSPACE_PATH" ).ok();
  env::set_var( "WORKSPACE_PATH", temp_dir.path() );
  
  let workspace = Workspace::resolve().unwrap();
  
  // Restore state
  match original
  {
    Some( value ) => env::set_var( "WORKSPACE_PATH", value ),
    None => env::remove_var( "WORKSPACE_PATH" ),
  }
  
  // Basic workspace operations should always work
  assert!( workspace.validate().is_ok() );
  assert_eq!( workspace.root(), temp_dir.path() );
  
  // Standard directory paths should work
  assert_eq!( workspace.config_dir(), temp_dir.path().join( "config" ) );
  assert_eq!( workspace.data_dir(), temp_dir.path().join( "data" ) );
  assert_eq!( workspace.logs_dir(), temp_dir.path().join( "logs" ) );
  
  // Path operations should work
  let joined = workspace.join( "test.txt" );
  assert_eq!( joined, temp_dir.path().join( "test.txt" ) );
  
  // Basic path operations should work
  assert!( joined.is_absolute() );
  
  // Boundary checking should work
  assert!( workspace.is_workspace_file( &joined ) );
  assert!( !workspace.is_workspace_file( "/etc/passwd" ) );
  
  // Convenience function should work
  let original = env::var( "WORKSPACE_PATH" ).ok();
  env::set_var( "WORKSPACE_PATH", temp_dir.path() );
  
  let ws_result = workspace_tools::workspace();
  
  // Restore environment
  match original
  {
    Some( value ) => env::set_var( "WORKSPACE_PATH", value ),
    None => env::remove_var( "WORKSPACE_PATH" ),
  }
  
  assert!( ws_result.is_ok() );
  let ws = ws_result.unwrap();
  assert_eq!( ws.root(), temp_dir.path() );
}

/// Test FC.7: Performance with all features enabled
#[ cfg( all( 
  feature = "cargo_integration", 
  feature = "serde_integration", 
  feature = "glob", 
  feature = "secret_management"
) ) ]
#[ test ]
fn test_all_features_performance()
{
  use std::time::Instant;
  
  let temp_dir = TempDir::new().unwrap();
  
  // Create cargo workspace
  fs::write( temp_dir.path().join( "Cargo.toml" ), "[workspace]\nmembers = []\n" ).unwrap();
  
  let start = Instant::now();
  
  // Create workspace using cargo
  let workspace = Workspace::from_cargo_manifest( temp_dir.path().join( "Cargo.toml" ) ).unwrap();
  
  // Perform multiple operations quickly
  for i in 0..100
  {
    let _joined = workspace.join( format!( "file_{}.txt", i ) );
    let _config_dir = workspace.config_dir();
    let _is_cargo = workspace.is_cargo_workspace();
  }
  
  let duration = start.elapsed();
  
  // Should complete quickly (within reasonable time)
  assert!( duration.as_millis() < 1000, "Operations should complete within 1 second" );
}

/// Test FC.8: Feature interaction edge cases
#[ cfg( all( feature = "cargo_integration", feature = "serde_integration" ) ) ]
#[ test ]
fn test_feature_interaction_edge_cases()
{
  use serde::{ Serialize, Deserialize };
  
  #[ derive( Debug, Serialize, Deserialize, PartialEq ) ]
  struct EdgeConfig
  {
    name : String,
    values : Vec< i32 >,
  }
  
  let temp_dir = TempDir::new().unwrap();
  
  // Create minimal cargo workspace
  fs::write( temp_dir.path().join( "Cargo.toml" ), "[workspace]\nmembers = []\n" ).unwrap();
  
  let workspace = Workspace::from_cargo_manifest( temp_dir.path().join( "Cargo.toml" ) ).unwrap();
  
  // Create config directory
  fs::create_dir_all( workspace.config_dir() ).unwrap();
  
  // Test edge case: empty config
  let empty_config = EdgeConfig {
    name : String::new(),
    values : vec![],
  };
  
  let save_result = workspace.save_config( "empty", &empty_config );
  assert!( save_result.is_ok(), "Should save empty config" );
  
  let loaded : Result< EdgeConfig, WorkspaceError > = workspace.load_config( "empty" );
  assert!( loaded.is_ok(), "Should load empty config" );
  assert_eq!( loaded.unwrap(), empty_config );
  
  // Test edge case: large config
  let large_config = EdgeConfig {
    name : "x".repeat( 1000 ),
    values : (0..1000).collect(),
  };
  
  let save_large = workspace.save_config( "large", &large_config );
  assert!( save_large.is_ok(), "Should save large config" );
  
  let loaded_large : Result< EdgeConfig, WorkspaceError > = workspace.load_config( "large" );
  assert!( loaded_large.is_ok(), "Should load large config" );
  assert_eq!( loaded_large.unwrap(), large_config );
}