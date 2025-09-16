//! integration tests moved from `examples/006_testing_integration.rs` to comply with "no tests in examples" requirement

use workspace_tools::testing::{ create_test_workspace, create_test_workspace_with_structure };

#[ cfg( feature = "testing" ) ]
#[ test ]
fn test_workspace_basic_operations()
{
  let ( _temp_dir, ws ) = create_test_workspace();

  // test workspace resolution
  assert!( ws.root().exists() );
  assert!( ws.root().is_dir() );

  // test path operations
  let config = ws.join( "config.toml" );
  assert!( ws.is_workspace_file( &config ) );

  // test standard directories
  let data_dir = ws.data_dir();
  assert!( data_dir.starts_with( ws.root() ) );
}

#[ cfg( feature = "testing" ) ]
#[ test ]
fn test_workspace_with_structure()
{
  let ( _temp_dir, ws ) = create_test_workspace_with_structure();

  // verify standard directories exist
  assert!( ws.config_dir().exists() );
  assert!( ws.data_dir().exists() );
  assert!( ws.logs_dir().exists() );

  // test file creation
  let config_file = ws.config_dir().join( "test.toml" );
  std::fs::write( &config_file, "[test]" ).unwrap();
  assert!( config_file.exists() );
  assert!( ws.is_workspace_file( &config_file ) );
}

#[ cfg( all( feature = "testing", feature = "glob" ) ) ]
#[ test ]
fn test_config_discovery()
{
  let ( _temp_dir, ws ) = create_test_workspace_with_structure();

  // create test config
  let config_path = ws.config_dir().join( "app.toml" );
  std::fs::write( &config_path, "[app]" ).unwrap();

  // test discovery
  let found = ws.find_config( "app" ).unwrap();
  assert_eq!( found, config_path );

  // test missing config
  assert!( ws.find_config( "nonexistent" ).is_err() );
}