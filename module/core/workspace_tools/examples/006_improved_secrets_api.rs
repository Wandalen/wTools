#![ allow( clippy ::uninlined_format_args, clippy ::redundant_closure_for_method_calls, clippy ::unnecessary_unwrap, clippy ::unnecessary_wraps ) ]

//! # 006 - Improved Secrets API (task 021)
//!
//! Demonstrates the enhanced secrets API with better error handling,
//! path-aware methods, and debugging tools
//! This example requires the "`secrets`" feature

#[ cfg( feature = "secrets" ) ]
fn main() -> Result< (), workspace_tools ::WorkspaceError >
{
  println!( "🔒 Enhanced Secrets API - Task 021 Demo\n" );

  // Setup workspace
  if std ::env ::var( "WORKSPACE_PATH" ).is_err()
  {
  std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
 }

  let ws = workspace_tools ::workspace()?;

  // 1. Enhanced error handling demonstration
  println!( "1️⃣  Enhanced Error Handling: " );
  demonstrate_enhanced_errors( &ws )?;

  // 2. Path-aware methods demonstration
  println!( "\n2️⃣  Path-aware Methods: " );
  demonstrate_path_methods( &ws )?;

  // 3. Helper methods demonstration
  println!( "\n3️⃣  Helper Methods: " );
  demonstrate_helper_methods( &ws )?;

  // 4. Debug methods demonstration
  println!( "\n4️⃣  Debug Methods: " );
  demonstrate_debug_methods( &ws )?;

  // 5. Migration examples
  println!( "\n5️⃣  Migration Examples: " );
  demonstrate_migration_patterns( &ws )?;

  cleanup_demo_files( &ws );

  println!( "\n🎉 Enhanced Secrets API Demo Complete!" );
  println!( "Key improvements: " );
  println!( "  • No more silent failures - explicit errors with helpful suggestions" );
  println!( "  • Path vs filename clarity - warnings guide correct usage" );
  println!( "  • New path-aware methods for flexible secret loading" );
  println!( "  • Debug helpers for troubleshooting secret issues" );
  println!( "  • Better error messages with resolved paths and available files" );

  Ok( () )
}

#[ cfg( feature = "secrets" ) ]
fn demonstrate_enhanced_errors( ws: &workspace_tools ::Workspace ) -> Result< (), workspace_tools ::WorkspaceError >
{
  use std ::fs;

  println!( "   Testing enhanced error handling: " );

  // Create some example files first
  let secret_dir = ws.secret_dir();
  fs ::create_dir_all( &secret_dir )
  .map_err( | e | workspace_tools ::WorkspaceError ::IoError( e.to_string() ) )?;

  let example_content = "EXAMPLE_KEY=example-value\nTEST_TOKEN=test-token-123";
  fs ::write( ws.secret_file( "example.env" ), example_content )
  .map_err( | e | workspace_tools ::WorkspaceError ::IoError( e.to_string() ) )?;
  fs ::write( ws.secret_file( "test.env" ), "TEST_KEY=test-value" )
  .map_err( | e | workspace_tools ::WorkspaceError ::IoError( e.to_string() ) )?;

  // Try to load nonexistent file - should show available files
  match ws.load_secrets_from_file( "nonexistent.env" )
  {
  Ok( _ ) => println!( "   ❌ Unexpected success" ),
  Err( e ) => println!( "   ✅ Enhanced error: {}", e ),
 }

  // Try with path-like parameter - should show warning
  println!( "   Testing path-like parameter warning: " );
  match ws.load_secrets_from_file( "config/secrets.env" )
  {
  Ok( _ ) => println!( "   ❌ Unexpected success" ),
  Err( e ) => println!( "   ✅ Path warning + error: {}", e ),
 }

  Ok( () )
}

#[ cfg( feature = "secrets" ) ]
fn demonstrate_path_methods( ws: &workspace_tools ::Workspace ) -> Result< (), workspace_tools ::WorkspaceError >
{
  use std ::fs;

  println!( "   Testing new path-aware methods: " );

  // Create nested directory structure
  let config_dir = ws.join( "config" );
  let lib_dir = ws.join( "lib/project/secret" );
  fs ::create_dir_all( &config_dir )
  .map_err( | e | workspace_tools ::WorkspaceError ::IoError( e.to_string() ) )?;
  fs ::create_dir_all( &lib_dir )
  .map_err( | e | workspace_tools ::WorkspaceError ::IoError( e.to_string() ) )?;

  // Create secrets in different locations
  let config_secrets = "CONFIG_KEY=config-value\nCONFIG_TOKEN=config-token-456";
  let lib_secrets = "LIB_KEY=lib-value\nNESTED_SECRET=deeply-nested-secret";

  fs ::write( config_dir.join( "secrets.env" ), config_secrets )
  .map_err( | e | workspace_tools ::WorkspaceError ::IoError( e.to_string() ) )?;
  fs ::write( lib_dir.join( "api.env" ), lib_secrets )
  .map_err( | e | workspace_tools ::WorkspaceError ::IoError( e.to_string() ) )?;

  // Load using path methods
  println!( "   ✅ Loading from config/secrets.env: " );
  let config_secrets_map = ws.load_secrets_from_path( "config/secrets.env" )?;
  println!( "      Found {} keys: {:?}", config_secrets_map.len(), config_secrets_map.keys().collect :: < Vec< _ > >() );

  println!( "   ✅ Loading from nested lib/project/secret/api.env: " );
  let lib_secrets_map = ws.load_secrets_from_path( "lib/project/secret/api.env" )?;
  println!( "      Found {} keys: {:?}", lib_secrets_map.len(), lib_secrets_map.keys().collect :: < Vec< _ > >() );

  // Create temporary file for absolute path demo
  let temp_file = std ::env ::temp_dir().join( "workspace_demo_secrets.env" );
  let abs_secrets = "ABSOLUTE_KEY=absolute-value\nTEMP_SECRET=temporary-secret";
  fs ::write( &temp_file, abs_secrets )
  .map_err( | e | workspace_tools ::WorkspaceError ::IoError( e.to_string() ) )?;

  println!( "   ✅ Loading from absolute path: {}", temp_file.display() );
  let abs_secrets_map = ws.load_secrets_from_absolute_path( &temp_file )?;
  println!( "      Found {} keys: {:?}", abs_secrets_map.len(), abs_secrets_map.keys().collect :: < Vec< _ > >() );

  // Clean up temp file
  let _ = fs ::remove_file( temp_file );

  Ok( () )
}

#[ cfg( feature = "secrets" ) ]
fn demonstrate_helper_methods( ws: &workspace_tools ::Workspace ) -> Result< (), workspace_tools ::WorkspaceError >
{
  println!( "   Testing helper methods: " );

  // List available secrets files
  let files = ws.list_secrets_files()?;
  println!( "   📁 Available secrets files: {:?}", files );

  // Check file existence
  for file in &files
  {
  let exists = ws.secrets_file_exists( file );
  let path = ws.resolve_secrets_path( file );
  println!( "   📄 {} : exists={}, path={}", file, exists, path.display() );
 }

  // Test with nonexistent file
  println!( "   📄 nonexistent.env: exists={}", ws.secrets_file_exists( "nonexistent.env" ) );

  Ok( () )
}

#[ cfg( feature = "secrets" ) ]
fn demonstrate_debug_methods( ws: &workspace_tools ::Workspace ) -> Result< (), workspace_tools ::WorkspaceError >
{
  println!( "   Testing debug methods: " );

  // Create a test file for debug demonstration
  if ws.list_secrets_files()?.is_empty()
  {
  let debug_content = "DEBUG_KEY=debug-value\nVERBOSE_TOKEN=verbose-token-789";
  std ::fs ::write( ws.secret_file( "debug.env" ), debug_content )
   .map_err( | e | workspace_tools ::WorkspaceError ::IoError( e.to_string() ) )?;
 }

  let available_files = ws.list_secrets_files()?;
  if let Some( first_file ) = available_files.first()
  {
  println!( "   🔍 Loading {} with debug information: ", first_file );
  let _secrets = ws.load_secrets_with_debug( first_file )?;
 }

  // Try debug load with path-like parameter to show warning
  println!( "   🔍 Testing debug with path-like parameter: " );
  let _result = ws.load_secrets_with_debug( "config/debug.env" );

  Ok( () )
}

#[ cfg( feature = "secrets" ) ]
fn demonstrate_migration_patterns( _ws: &workspace_tools ::Workspace ) -> Result< (), workspace_tools ::WorkspaceError >
{
  println!( "   Common migration patterns: " );

  println!( "   📚 Before (problematic patterns) : " );
  println!( r#"      // ❌ This used to silently fail
   let secrets = ws.load_secrets_from_file("lib/project/secret/api.env")?;

   // ❌ This gave unhelpful error messages
   let key = ws.load_secret_key("API_KEY", "nonexistent.env")?;"# );

  println!( "   📚 After (improved patterns) : " );
  println!( r#"      // ✅ Now gives explicit error with available files
   let secrets = ws.load_secrets_from_file("api.env")?;  // filename only

   // ✅ Or use path-aware method for paths
   let secrets = ws.load_secrets_from_path("lib/project/secret/api.env")?;

   // ✅ Better error messages with resolved paths
   let key = ws.load_secret_key("API_KEY", "api.env")?;

   // ✅ Debug helpers for troubleshooting
   let secrets = ws.load_secrets_with_debug("api.env")?;"# );

  Ok( () )
}

#[ cfg( feature = "secrets" ) ]
fn cleanup_demo_files( ws: &workspace_tools ::Workspace )
{
  let _ = std ::fs ::remove_dir_all( ws.secret_dir() );
  let _ = std ::fs ::remove_dir_all( ws.join( "config" ) );
  let _ = std ::fs ::remove_dir_all( ws.join( "lib" ) );
}

#[ cfg( not( feature = "secrets" ) ) ]
fn main()
{
  println!( "🚨 This example requires the 'secrets' feature" );
  println!( "Run with: cargo run --example 006_improved_secrets_api --features secrets" );
  println!();
  println!( "To enable secrets feature permanently, add to Cargo.toml: " );
  println!( r#"[dependencies]"# );
  println!( r#"workspace_tools = {{ version = "0.3", features = ["secrets"] }}"# );
}