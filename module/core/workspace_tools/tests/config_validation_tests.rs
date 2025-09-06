//! Config Validation Tests
//!
//! These tests verify the schema-based configuration validation functionality
//! that prevents runtime configuration errors and provides clear validation messages.

#![ cfg( feature = "testing" ) ]

use workspace_tools::testing::create_test_workspace_with_structure;
use std::fs;
use serde::{ Deserialize, Serialize };
use schemars::JsonSchema;

/// Test configuration struct for validation
#[ derive( Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq ) ]
struct AppConfig
{
  name : String,
  port : u16,
  debug : bool,
  features : Vec< String >,
  database : DatabaseConfig,
}

#[ derive( Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq ) ]
struct DatabaseConfig
{
  host : String,
  port : u16,
  ssl_enabled : bool,
}

/// Test automatic schema generation and validation with valid config
#[ test ]
#[ cfg( feature = "validation" ) ]
fn test_load_config_with_validation_success()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let config_content = r#"
name = "test-app"
port = 8080
debug = true
features = ["logging", "metrics"]

[database]
host = "localhost"
port = 5432
ssl_enabled = true
"#;
  
  let config_file = workspace.config_dir().join( "app.toml" );
  fs::write( &config_file, config_content ).unwrap();
  
  let loaded_config : AppConfig = workspace.load_config_with_validation( "app" ).unwrap();
  
  assert_eq!( loaded_config.name, "test-app" );
  assert_eq!( loaded_config.port, 8080 );
  assert!( loaded_config.debug );
  assert_eq!( loaded_config.features, vec![ "logging".to_string(), "metrics".to_string() ] );
  assert_eq!( loaded_config.database.host, "localhost" );
  assert_eq!( loaded_config.database.port, 5432 );
  assert!( loaded_config.database.ssl_enabled );
}

/// Test validation failure with invalid data types
#[ test ]
#[ cfg( feature = "validation" ) ]
fn test_load_config_with_validation_type_error()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // Invalid config: port should be u16, not string
  let config_content = r#"
name = "test-app"
port = "invalid-port"
debug = true
features = ["logging"]

[database]
host = "localhost"
port = 5432
ssl_enabled = true
"#;
  
  let config_file = workspace.config_dir().join( "app.toml" );
  fs::write( &config_file, config_content ).unwrap();
  
  let result = workspace.load_config_with_validation::< AppConfig >( "app" );
  
  assert!( result.is_err() );
  let error_msg = result.unwrap_err().to_string();
  assert!( error_msg.contains( "validation" ) );
}

/// Test validation failure with missing required fields
#[ test ]
#[ cfg( feature = "validation" ) ]
fn test_load_config_with_validation_missing_fields()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // Invalid config: missing required database section
  let config_content = r#"
name = "test-app"
port = 8080
debug = true
features = ["logging"]
"#;
  
  let config_file = workspace.config_dir().join( "app.toml" );
  fs::write( &config_file, config_content ).unwrap();
  
  let result = workspace.load_config_with_validation::< AppConfig >( "app" );
  
  assert!( result.is_err() );
  let error_msg = result.unwrap_err().to_string();
  assert!( error_msg.contains( "validation" ) );
}

/// Test validation with JSON format
#[ test ]
#[ cfg( feature = "validation" ) ]
fn test_load_config_with_validation_json()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let config_content = r#"
{
  "name": "json-app",
  "port": 9090,
  "debug": false,
  "features": ["api", "web"],
  "database": {
    "host": "db.example.com",
    "port": 3306,
    "ssl_enabled": false
  }
}
"#;
  
  let config_file = workspace.config_dir().join( "app.json" );
  fs::write( &config_file, config_content ).unwrap();
  
  let loaded_config : AppConfig = workspace.load_config_with_validation( "app" ).unwrap();
  
  assert_eq!( loaded_config.name, "json-app" );
  assert_eq!( loaded_config.port, 9090 );
  assert!( !loaded_config.debug );
  assert_eq!( loaded_config.database.host, "db.example.com" );
  assert_eq!( loaded_config.database.port, 3306 );
  assert!( !loaded_config.database.ssl_enabled );
}

/// Test validation with YAML format
#[ test ]
#[ cfg( feature = "validation" ) ]
fn test_load_config_with_validation_yaml()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let config_content = r"
name: yaml-app
port: 7070
debug: true
features:
  - yaml
  - validation
database:
  host: yaml-db.local
  port: 5433
  ssl_enabled: true
";
  
  let config_file = workspace.config_dir().join( "app.yaml" );
  fs::write( &config_file, config_content ).unwrap();
  
  let loaded_config : AppConfig = workspace.load_config_with_validation( "app" ).unwrap();
  
  assert_eq!( loaded_config.name, "yaml-app" );
  assert_eq!( loaded_config.port, 7070 );
  assert!( loaded_config.debug );
  assert_eq!( loaded_config.features, vec![ "yaml".to_string(), "validation".to_string() ] );
  assert_eq!( loaded_config.database.host, "yaml-db.local" );
  assert_eq!( loaded_config.database.port, 5433 );
}

/// Test validation with additional properties (should succeed as schema allows them)
#[ test ]
#[ cfg( feature = "validation" ) ]
fn test_load_config_with_extra_properties()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let config_content = r#"
name = "test-app"
port = 8080
debug = true
features = ["logging"]
extra_field = "should-be-ignored"

[database]
host = "localhost"
port = 5432
ssl_enabled = true
extra_db_field = 42
"#;
  
  let config_file = workspace.config_dir().join( "app.toml" );
  fs::write( &config_file, config_content ).unwrap();
  
  // Should succeed - extra fields are typically allowed in JSON Schema
  let loaded_config : AppConfig = workspace.load_config_with_validation( "app" ).unwrap();
  
  assert_eq!( loaded_config.name, "test-app" );
  assert_eq!( loaded_config.port, 8080 );
}

/// Test static content validation without loading
#[ test ]
#[ cfg( feature = "validation" ) ]
fn test_validate_config_content()
{
  use workspace_tools::Workspace;
  use jsonschema::Validator;
  
  // Generate schema
  let schema = schemars::schema_for!( AppConfig );
  let schema_json = serde_json::to_value( &schema ).unwrap();
  let compiled_schema = Validator::new( &schema_json ).unwrap();
  
  // Valid TOML content
  let valid_content = r#"
name = "test"
port = 8080
debug = true
features = []

[database]
host = "localhost"
port = 5432
ssl_enabled = false
"#;
  
  let result = Workspace::validate_config_content( valid_content, &compiled_schema, "toml" );
  assert!( result.is_ok() );
  
  // Invalid TOML content (missing database)
  let invalid_content = r#"
name = "test"
port = 8080
debug = true
features = []
"#;
  
  let result = Workspace::validate_config_content( invalid_content, &compiled_schema, "toml" );
  assert!( result.is_err() );
  let error_msg = result.unwrap_err().to_string();
  assert!( error_msg.contains( "validation" ) );
}

/// Test detailed validation error messages
#[ test ]
#[ cfg( feature = "validation" ) ]
fn test_validation_error_details()
{
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // Config with multiple validation errors
  let config_content = r#"
name = 123
port = "not-a-number"
debug = "not-a-boolean"
features = "not-an-array"

[database]
host = 456
port = "not-a-port"
ssl_enabled = "not-a-boolean"
"#;
  
  let config_file = workspace.config_dir().join( "app.toml" );
  fs::write( &config_file, config_content ).unwrap();
  
  let result = workspace.load_config_with_validation::< AppConfig >( "app" );
  
  assert!( result.is_err() );
  let error_msg = result.unwrap_err().to_string();
  assert!( error_msg.contains( "validation failed" ) );
  // The error should contain details about what went wrong
  assert!( error_msg.len() > 50 ); // Should be a detailed error message
}

/// Test validation with custom schema (external schema)
#[ test ]
#[ cfg( feature = "validation" ) ]
fn test_load_config_with_external_schema()
{
  use jsonschema::Validator;
  
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // Create a custom schema that's more restrictive
  let schema_json = serde_json::json!( {
    "type": "object",
    "properties": {
      "name": { "type": "string", "minLength": 3 },
      "port": { "type": "number", "minimum": 1000, "maximum": 9999 }
    },
    "required": [ "name", "port" ],
    "additionalProperties": false
  } );
  
  let compiled_schema = Validator::new( &schema_json ).unwrap();
  
  // Valid config according to custom schema
  let config_content = r#"
name = "valid-app"
port = 8080
"#;
  
  let config_file = workspace.config_dir().join( "custom.toml" );
  fs::write( &config_file, config_content ).unwrap();
  
  #[ derive( Deserialize ) ]
  struct CustomConfig
  {
    name : String,
    port : u16,
  }
  
  let loaded_config : CustomConfig = workspace.load_config_from_with_schema( &config_file, &compiled_schema ).unwrap();
  
  assert_eq!( loaded_config.name, "valid-app" );
  assert_eq!( loaded_config.port, 8080 );
  
  // Invalid config (port too low)
  let invalid_content = r#"
name = "app"
port = 80
"#;
  
  let invalid_file = workspace.config_dir().join( "invalid.toml" );
  fs::write( &invalid_file, invalid_content ).unwrap();
  
  let result = workspace.load_config_from_with_schema::< CustomConfig, _ >( &invalid_file, &compiled_schema );
  assert!( result.is_err() );
}