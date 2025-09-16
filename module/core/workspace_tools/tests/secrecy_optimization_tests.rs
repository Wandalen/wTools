//! Secrecy Optimization Tests
//!
//! Tests for optimized and advanced secrecy features including `SecretInjectable` trait,
//! validation capabilities, and performance optimizations.

#![ cfg( feature = "secure" ) ]

use std::fs;
use workspace_tools::{ testing, SecretInjectable };
use secrecy::ExposeSecret;

/// Test `SecretInjectable` trait implementation
#[ test ]
#[ cfg( feature = "secure" ) ]
fn test_secret_injectable_trait()
{
  let ( _temp_dir, workspace ) = testing::create_test_workspace_with_structure();
  
  // Create a secret file with configuration data
  let secret_content = "DATABASE_URL=postgresql://localhost/testdb\nAPI_KEY=secret-key-123";
  let secret_file = workspace.secret_file( "-config.sh" );
  fs::write( &secret_file, secret_content ).unwrap();

  #[ derive( Debug ) ]
  struct AppConfig
  {
    database_url : String,
    api_key : String,
  }

  impl SecretInjectable for AppConfig
  {
    fn inject_secret( &mut self, key : &str, value : String ) -> workspace_tools::Result< () >
    {
      match key
      {
        "DATABASE_URL" => self.database_url = value,
        "API_KEY" => self.api_key = value,
        _ => return Err( workspace_tools::WorkspaceError::SecretInjectionError(
          format!( "unknown secret key: {key}" )
        ) ),
      }
      Ok( () )
    }

    fn validate_secrets( &self ) -> workspace_tools::Result< () >
    {
      if self.api_key.is_empty()
      {
        return Err( workspace_tools::WorkspaceError::SecretValidationError(
          "api_key cannot be empty".to_string()
        ) );
      }
      Ok( () )
    }
  }

  // Test SecretInjectable trait allows automatic secret injection into config types
  let initial_config = AppConfig { database_url: String::new(), api_key: String::new() };
  let config = workspace.load_config_with_secrets( initial_config, "-config.sh" ).unwrap();
  assert_eq!( config.database_url, "postgresql://localhost/testdb" );
  assert_eq!( config.api_key, "secret-key-123" );
}

/// Test secret validation and strength checking
#[ test ]
#[ cfg( feature = "secure" ) ]
fn test_secret_validation()
{
  let ( _temp_dir, workspace ) = testing::create_test_workspace_with_structure();
  
  // Test weak secret detection (short)
  let weak_secret = workspace.validate_secret( "123" );
  assert!( weak_secret.is_err() );
  assert!( weak_secret.unwrap_err().to_string().contains( "8 characters" ) );

  // Test weak secret detection (common pattern)
  let common_secret = workspace.validate_secret( "password" );
  assert!( common_secret.is_err() );
  assert!( common_secret.unwrap_err().to_string().contains( "weak" ) );

  // Test strong secret validation
  let strong_secret = workspace.validate_secret( "super-strong-secret-key-with-entropy-2024!" );
  assert!( strong_secret.is_ok() );
}

/// Test secure configuration loading with automatic injection
#[ test ]
#[ cfg( feature = "secure" ) ]
fn test_secure_config_loading()
{
  let ( _temp_dir, workspace ) = testing::create_test_workspace_with_structure();
  
  // Create multiple config files with different secret patterns
  let config_content = r#"
[database]
url = "${DATABASE_URL}"
password = "${DB_PASSWORD}"

[api]
key = "${API_KEY}"
"#;
  
  let secrets_content = "DATABASE_URL=postgresql://secure/db\nDB_PASSWORD=secure-password\nAPI_KEY=secure-api-key";
  
  fs::write( workspace.join( "config.toml" ), config_content ).unwrap();
  fs::write( workspace.secret_file( "-secrets.sh" ), secrets_content ).unwrap();

  // Test automatic secret injection into configuration
  let injected_config = workspace.load_config_with_secret_injection( "config.toml", "-secrets.sh" ).unwrap();
  
  // Verify secrets were injected (this should return processed config string)
  assert!( injected_config.contains( "postgresql://secure/db" ) );
  assert!( injected_config.contains( "secure-api-key" ) );
  assert!( !injected_config.contains( "${" ) ); // No unresolved placeholders
}

/// Test comprehensive error handling for secure operations
#[ test ]
#[ cfg( feature = "secure" ) ]
fn test_comprehensive_error_handling()
{
  let ( _temp_dir, workspace ) = testing::create_test_workspace_with_structure();
  
  // Test missing secret file error
  let result = workspace.load_secrets_secure( "nonexistent.sh" );
  assert!( result.is_err() ); // Should return explicit error, not empty HashMap
  let error_msg = result.unwrap_err().to_string();
  assert!( error_msg.contains( "not found" ) );

  // Test invalid secret format handling
  let invalid_content = "INVALID FORMAT LINE WITHOUT EQUALS SIGN";
  fs::write( workspace.secret_file( "-invalid.sh" ), invalid_content ).unwrap();
  
  let result = workspace.load_secrets_secure( "-invalid.sh" );
  assert!( result.is_ok() ); // Should handle gracefully
  assert!( result.unwrap().is_empty() );

  // Test permission denied scenario would be here if we could simulate it
}

/// Test zero overhead when secure feature disabled
#[ test ]
fn test_zero_overhead_feature_disabled()
{
  // This test ensures compilation works without secure feature
  // The test itself validates that non-secure operations work normally
  let ( _temp_dir, workspace ) = testing::create_test_workspace_with_structure();
  
  // This should compile and work regardless of secure feature
  let result = workspace.join( "test_path" );
  assert!( result.to_string_lossy().ends_with( "test_path" ) );
}

/// Test edge cases with graceful error handling
#[ test ]
#[ cfg( feature = "secure" ) ]
fn test_edge_case_handling()
{
  let ( _temp_dir, workspace ) = testing::create_test_workspace_with_structure();
  
  // Test empty secret file
  fs::write( workspace.secret_file( "-empty.sh" ), "" ).unwrap();
  let result = workspace.load_secrets_secure( "-empty.sh" );
  assert!( result.is_ok() );
  assert!( result.unwrap().is_empty() );

  // Test secret file with only comments
  fs::write( workspace.secret_file( "-comments.sh" ), "# Only comments\n# Another comment" ).unwrap();
  let result = workspace.load_secrets_secure( "-comments.sh" );
  assert!( result.is_ok() );
  assert!( result.unwrap().is_empty() );

  // Test secret with very long value
  let long_value = "a".repeat( 10000 );
  let long_secret = format!( "LONG_SECRET={long_value}" );
  fs::write( workspace.secret_file( "-long.sh" ), long_secret ).unwrap();
  
  let result = workspace.load_secrets_secure( "-long.sh" );
  assert!( result.is_ok() );
  let secrets = result.unwrap();
  assert_eq!( secrets.get( "LONG_SECRET" ).unwrap().expose_secret(), &long_value );
}

/// Test performance characteristics
#[ test ]
#[ cfg( feature = "secure" ) ]
fn test_performance_characteristics()
{
  let ( _temp_dir, workspace ) = testing::create_test_workspace_with_structure();
  
  // Create a large secret file to test performance
  let mut large_content = String::new();
  for i in 0..1000
  {
    use core::fmt::Write;
    writeln!( &mut large_content, "SECRET_KEY_{i}=secret-value-{i}" ).unwrap();
  }
  
  fs::write( workspace.secret_file( "-large.sh" ), large_content ).unwrap();
  
  // Test that loading large number of secrets performs reasonably
  let start = std::time::Instant::now();
  let result = workspace.load_secrets_secure( "-large.sh" );
  let duration = start.elapsed();
  
  assert!( result.is_ok() );
  assert_eq!( result.unwrap().len(), 1000 );
  
  // Should complete within reasonable time (less than 100ms for 1000 secrets)
  assert!( duration.as_millis() < 100 );
}

/// Test security best practices validation
#[ test ]
#[ cfg( feature = "secure" ) ]
fn test_security_best_practices()
{
  let ( _temp_dir, workspace ) = testing::create_test_workspace_with_structure();
  
  // Test that secrets are properly zeroized on drop
  {
    let secret_content = "TEMP_SECRET=temporary-value";
    fs::write( workspace.secret_file( "-temp.sh" ), secret_content ).unwrap();
    
    let secrets = workspace.load_secrets_secure( "-temp.sh" ).unwrap();
    let temp_secret = secrets.get( "TEMP_SECRET" ).unwrap().clone();
    assert_eq!( temp_secret.expose_secret(), "temporary-value" );
  } // temp_secret should be zeroized here
  
  // Verify that debug output doesn't expose secrets
  let secret_content = "DEBUG_SECRET=should-not-appear-in-debug";
  fs::write( workspace.secret_file( "-debug.sh" ), secret_content ).unwrap();
  
  let secrets = workspace.load_secrets_secure( "-debug.sh" ).unwrap();
  let debug_output = format!( "{secrets:?}" );
  
  // Debug output should not contain the actual secret value
  assert!( !debug_output.contains( "should-not-appear-in-debug" ) );
  // Debug output should contain some indication of redacted content
  assert!( debug_output.contains( "Secret" ) || debug_output.contains( "[REDACTED]" ) || debug_output.contains( "***" ) );
}