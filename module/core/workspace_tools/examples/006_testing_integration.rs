//! # 006 - Testing Integration
//!
//! testing patterns with `workspace_tools` for isolated test environments
//! demonstrates test utilities and best practices

use workspace_tools::WorkspaceError;

#[ cfg( feature = "testing" ) ]
use workspace_tools::testing::{ create_test_workspace, create_test_workspace_with_structure };

fn main() -> Result< (), WorkspaceError >
{
  println!( "🧪 testing integration with workspace_tools\n" );
  
  // this example demonstrates testing patterns rather than actual tests
  // the testing utilities require the "testing" feature (which is in default features)
  
  #[ cfg( feature = "testing" ) ]
  {
    demonstrate_basic_testing();
    demonstrate_structured_testing()?;
    demonstrate_config_testing()?;
    demonstrate_isolation_testing()?;
    demonstrate_cleanup_patterns()?;
  }
  
  #[ cfg( not( feature = "testing" ) ) ]
  {
    println!( "🚨 testing utilities require the 'testing' feature" );
    println!( "the 'testing' feature is in default features, so this should normally work" );
  }
  
  println!( "\n🧪 testing best practices:" );
  println!( "   • always use isolated test workspaces" );
  println!( "   • keep temp_dir alive for test duration" );
  println!( "   • test both success and failure scenarios" );
  println!( "   • use structured workspaces for complex tests" );
  println!( "   • clean up resources in test teardown" );
  println!( "   • test workspace boundary violations" );
  println!( "   • mock external dependencies in tests" );
  
  println!( "\n🎯 next: run example 007 to see real-world application patterns" );
  
  Ok( () )
}

#[ cfg( feature = "testing" ) ]
fn demonstrate_basic_testing()
{
  println!( "1️⃣  basic testing patterns:" );
  
  // create isolated test workspace
  let ( _temp_dir, ws ) = create_test_workspace();
  
  println!( "   ✅ created isolated test workspace: {}", ws.root().display() );
  
  // test basic operations
  let config_dir = ws.config_dir();
  let data_file = ws.join( "data/test.db" );
  
  println!( "   config dir: {}", config_dir.display() );
  println!( "   data file: {}", data_file.display() );
  
  // verify workspace isolation
  assert!( ws.is_workspace_file( &config_dir ) );
  assert!( ws.is_workspace_file( &data_file ) );
  assert!( !ws.is_workspace_file( "/tmp/external" ) );
  
  println!( "   ✅ workspace boundary checks passed" );
  
  // temp_dir automatically cleans up when dropped
  println!( "   ✅ automatic cleanup on scope exit" );
}

#[ cfg( feature = "testing" ) ]
fn demonstrate_structured_testing() -> Result< (), WorkspaceError >
{
  println!( "\n2️⃣  structured testing with standard directories:" );
  
  let ( _temp_dir, ws ) = create_test_workspace_with_structure();
  
  println!( "   ✅ created workspace with standard structure" );
  
  // verify all standard directories exist
  let standard_dirs = vec!
  [
    ( "config", ws.config_dir() ),
    ( "data", ws.data_dir() ),
    ( "logs", ws.logs_dir() ),
    ( "docs", ws.docs_dir() ),
    ( "tests", ws.tests_dir() ),
  ];
  
  for ( name, path ) in standard_dirs
  {
    if path.exists()
    {
      println!( "   ✅ {} directory exists: {}", name, path.display() );
    }
    else
    {
      println!( "   ❌ {} directory missing: {}", name, path.display() );
    }
  }
  
  // test file creation in standard directories
  std::fs::write( ws.config_dir().join( "test.toml" ), "[test]\nkey = \"value\"" )
    .map_err( | e | WorkspaceError::IoError( e.to_string() ) )?;
  
  std::fs::write( ws.data_dir().join( "test.json" ), "{\"test\": true}" )
    .map_err( | e | WorkspaceError::IoError( e.to_string() ) )?;
  
  println!( "   ✅ created test files in standard directories" );
  
  Ok( () )
}

#[ cfg( feature = "testing" ) ]
fn demonstrate_config_testing() -> Result< (), WorkspaceError >
{
  println!( "\n3️⃣  configuration testing patterns:" );
  
  let ( _temp_dir, ws ) = create_test_workspace_with_structure();
  
  // create test configuration files
  let configs = vec!
  [
    ( "app.toml", "[app]\nname = \"test-app\"\nport = 8080" ),
    ( "database.yaml", "host: localhost\nport: 5432\nname: test_db" ),
    ( "logging.json", r#"{"level": "debug", "format": "json"}"# ),
  ];
  
  for ( filename, content ) in configs
  {
    let config_path = ws.config_dir().join( filename );
    std::fs::write( &config_path, content )
      .map_err( | e | WorkspaceError::IoError( e.to_string() ) )?;
    println!( "   created test config: {}", config_path.display() );
  }
  
  // test configuration discovery
  #[ cfg( feature = "glob" ) ]
  {
    match ws.find_config( "app" )
    {
      Ok( config ) => println!( "   ✅ found app config: {}", config.display() ),
      Err( e ) => println!( "   ❌ failed to find app config: {e}" ),
    }
    
    match ws.find_config( "nonexistent" )
    {
      Ok( config ) => println!( "   unexpected config found: {}", config.display() ),
      Err( _ ) => println!( "   ✅ correctly failed to find nonexistent config" ),
    }
  }
  
  #[ cfg( not( feature = "glob" ) ) ]
  {
    println!( "   (config discovery requires glob feature)" );
  }
  
  Ok( () )
}

#[ cfg( feature = "testing" ) ]
fn demonstrate_isolation_testing() -> Result< (), WorkspaceError >
{
  println!( "\n4️⃣  testing workspace isolation:" );
  
  // create multiple isolated workspaces
  let ( _temp1, ws1 ) = create_test_workspace();
  let ( _temp2, ws2 ) = create_test_workspace();
  
  println!( "   workspace 1: {}", ws1.root().display() );
  println!( "   workspace 2: {}", ws2.root().display() );
  
  // verify they're completely separate
  assert_ne!( ws1.root(), ws2.root() );
  println!( "   ✅ workspaces are isolated" );
  
  // test cross-workspace boundary checking
  let ws1_file = ws1.join( "test1.txt" );
  let ws2_file = ws2.join( "test2.txt" );
  
  std::fs::write( &ws1_file, "workspace 1 content" )
    .map_err( | e | WorkspaceError::IoError( e.to_string() ) )?;
  std::fs::write( &ws2_file, "workspace 2 content" )
    .map_err( | e | WorkspaceError::IoError( e.to_string() ) )?;
  
  // verify boundary checking works across workspaces
  assert!( ws1.is_workspace_file( &ws1_file ) );
  assert!( !ws1.is_workspace_file( &ws2_file ) );
  assert!( ws2.is_workspace_file( &ws2_file ) );
  assert!( !ws2.is_workspace_file( &ws1_file ) );
  
  println!( "   ✅ cross-workspace boundary checking works" );
  
  Ok( () )
}

#[ cfg( feature = "testing" ) ]
fn demonstrate_cleanup_patterns() -> Result< (), WorkspaceError >
{
  println!( "\n5️⃣  cleanup and resource management patterns:" );
  
  // pattern 1: automatic cleanup with RAII
  {
    let ( _temp_dir, ws ) = create_test_workspace();
    let test_file = ws.join( "temp_file.txt" );
    std::fs::write( &test_file, "temporary content" )
      .map_err( | e | WorkspaceError::IoError( e.to_string() ) )?;
    
    println!( "   created temporary file: {}", test_file.display() );
    println!( "   workspace will be cleaned up when temp_dir drops" );
  } // temp_dir dropped here, cleaning up everything
  
  println!( "   ✅ automatic cleanup completed" );
  
  // pattern 2: manual cleanup for complex scenarios
  let ( temp_dir, ws ) = create_test_workspace();
  
  // do complex test operations...
  let complex_structure = vec!
  [
    "deep/nested/directory/file1.txt",
    "deep/nested/directory/file2.txt", 
    "another/branch/file3.txt",
  ];
  
  for file_path in &complex_structure
  {
    let full_path = ws.join( file_path );
    if let Some( parent ) = full_path.parent()
    {
      std::fs::create_dir_all( parent )
        .map_err( | e | WorkspaceError::IoError( e.to_string() ) )?;
    }
    std::fs::write( &full_path, "test content" )
      .map_err( | e | WorkspaceError::IoError( e.to_string() ) )?;
  }
  
  println!( "   created complex directory structure with {} files", complex_structure.len() );
  
  // manual cleanup if needed (though temp_dir will handle it automatically)
  drop( temp_dir );
  println!( "   ✅ manual cleanup completed" );
  
  Ok( () )
}

// note: actual tests have been moved to tests/testing_integration_examples.rs