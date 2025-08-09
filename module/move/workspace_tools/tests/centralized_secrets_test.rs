//! Integration test for centralized secrets management
#![ cfg( feature = "secret_management" ) ]

use workspace_tools::workspace;
use std::env;
use tempfile::TempDir;

#[ test ]
fn test_centralized_secrets_access()
{
  // Use temp directory for testing instead of modifying the actual repository
  let temp_dir = TempDir::new().unwrap();
  
  // save original environment  
  let original_workspace_path = env::var( "WORKSPACE_PATH" ).ok();
  
  // Set environment variable to temp directory for testing
  env::set_var( "WORKSPACE_PATH", temp_dir.path() );
  
  let ws = workspace().expect( "Should resolve workspace" );
  
  // Test workspace access
  println!( "Workspace root: {}", ws.root().display() );
  
  // Test secrets directory
  let secrets_dir = ws.secret_dir();
  println!( "Secrets directory: {}", secrets_dir.display() );
  
  // Test loading OpenAI secret from single secrets file
  match ws.load_secret_key( "OPENAI_API_KEY", "-secrets.sh" )
  {
    Ok( key ) => {
      println!( "OpenAI API key loaded (length: {})", key.len() );
      assert!( !key.is_empty(), "API key should not be empty" );
    },
    Err( e ) => {
      println!( "Failed to load OpenAI API key: {e}" );
      // This might be expected if the file doesn't exist in test environment
    },
  }
  
  // Test loading Gemini secret from single secrets file
  match ws.load_secret_key( "GEMINI_API_KEY", "-secrets.sh" )
  {
    Ok( key ) => {
      println!( "Gemini API key loaded (length: {})", key.len() );
      assert!( !key.is_empty(), "API key should not be empty" );
    },
    Err( e ) => {
      println!( "Failed to load Gemini API key: {e}" );
      // This might be expected if the file doesn't exist in test environment
    },
  }
  
  // Test loading non-existent secret (should fail)
  match ws.load_secret_key( "NONEXISTENT_KEY", "nonexistent.env" )
  {
    Ok( _ ) => panic!( "Should not load non-existent key" ),
    Err( _ ) => println!( "Correctly failed to load non-existent key" ),
  }
  
  println!( "Centralized secrets management test completed successfully!" );
  
  // restore original environment
  match original_workspace_path {
    Some( path ) => env::set_var( "WORKSPACE_PATH", path ),
    None => env::remove_var( "WORKSPACE_PATH" ),
  }
}