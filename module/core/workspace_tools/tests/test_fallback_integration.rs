#![ allow( clippy ::uninlined_format_args ) ]

//! Integration test to verify fallback functionality works in practice

#[ cfg( feature = "secrets" ) ]
use workspace_tools ::{ Workspace, testing };
#[ cfg( feature = "secrets" ) ]
use std ::{ env, fs };

#[ test ]
#[ cfg( feature = "secrets" ) ]
#[ ignore = "depends on environment setup" ]
fn test_fallback_to_pro_workspace()
{
  // Setup: Create temp workspace without local secrets
  let temp_dir = env ::temp_dir().join( "test_fallback_to_pro" );
  fs ::create_dir_all( &temp_dir ).unwrap();

  // Setup: Create $PRO workspace with secrets
  let pro_dir = env ::temp_dir().join( "test_pro_workspace" );
  fs ::create_dir_all( &pro_dir ).unwrap();
  let pro_secret_dir = pro_dir.join( "secret" );
  fs ::create_dir_all( &pro_secret_dir ).unwrap();

  // Create a test secret in $PRO workspace
  let test_secret = "TEST_TOKEN=test_value_from_pro\nTEST_KEY=pro_secret_123";
  fs ::write( pro_secret_dir.join( "-test.env" ), test_secret ).unwrap();

  // Set $PRO environment variable
  env ::set_var( "PRO", &pro_dir );

  println!( "Testing fallback to $PRO workspace..." );
  println!( "Local workspace: {}", temp_dir.display() );
  println!( "$PRO workspace: {}", pro_dir.display() );

  // Create workspace from temp directory (no local secrets)
  let workspace = Workspace ::new( temp_dir.clone() );

  // Try to load secret - should fallback to $PRO
  let result = workspace.load_secret_key( "TEST_TOKEN", "-test.env" );

  match result
  {
  Ok( token ) =>
  {
   println!( "✅ SUCCESS! Loaded from fallback location" );
   assert_eq!( token, "test_value_from_pro" );
  }
  Err( e ) =>
  {
   println!( "❌ FAILED: {}", e );
   panic!( "Expected fallback to work" );
  }
  }

  // Cleanup
  env ::remove_var( "PRO" );
  fs ::remove_dir_all( temp_dir ).ok();
  fs ::remove_dir_all( pro_dir ).ok();
}

#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_fallback_priority()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // Setup: Create secret in local workspace
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();
  fs ::write( workspace.secret_file( "priority.env" ), "KEY=local_value" ).unwrap();

  // Setup: Create $PRO workspace with different value
  let pro_dir = env ::temp_dir().join( "test_pro_priority" );
  fs ::create_dir_all( &pro_dir ).unwrap();
  let pro_secret_dir = pro_dir.join( "secret" );
  fs ::create_dir_all( &pro_secret_dir ).unwrap();
  fs ::write( pro_secret_dir.join( "priority.env" ), "KEY=pro_value" ).unwrap();

  env ::set_var( "PRO", &pro_dir );

  // Load secret - local should win over $PRO
  let value = workspace.load_secret_key( "KEY", "priority.env" ).unwrap();
  assert_eq!( value, "local_value", "Local workspace should have priority over $PRO" );

  // Cleanup
  env ::remove_var( "PRO" );
  fs ::remove_dir_all( pro_dir ).ok();
}

#[ cfg( not( feature = "secrets" ) ) ]
fn main()
{
  println!( "This test requires the 'secrets' feature" );
}
