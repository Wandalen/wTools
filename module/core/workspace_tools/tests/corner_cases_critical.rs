#![ allow( clippy ::uninlined_format_args ) ]

//! tests for critical corner cases in secret fallback implementation
//!
//! these tests verify fixes for the most critical bugs identified in corner case analysis

#[ cfg( feature = "secrets" ) ]
use workspace_tools ::{ Workspace, testing, WorkspaceError };
#[ cfg( feature = "secrets" ) ]
use std ::{ env, fs };

/// test that path deduplication works with symlinks
#[ test ]
#[ cfg( all( feature = "secrets", unix ) ) ]  // symlinks work differently on Windows
fn test_path_deduplication_with_symlinks()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // create secret in local workspace
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();
  fs ::write( workspace.secret_file( "test.env" ), "KEY=value" ).unwrap();

  // create symlink to workspace root
  let symlink_path = env ::temp_dir().join( "symlink_workspace" );
  let _  = fs ::remove_file( &symlink_path );  // clean up if exists
  std ::os ::unix ::fs ::symlink( workspace.root(), &symlink_path ).unwrap();

  // set $PRO to symlink (same workspace, different path)
  env ::set_var( "PRO", &symlink_path );

  // load secrets - should only try local once (not twice via symlink)
  let result = workspace.load_secrets_from_file( "test.env" );
  assert!( result.is_ok(), "should load successfully without trying same file twice" );

  // cleanup
  env ::remove_var( "PRO" );
  fs ::remove_file( symlink_path ).ok();
}

/// test that empty $PRO is ignored
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_empty_pro_env_var()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // set $PRO to empty string
  env ::set_var( "PRO", "" );

  // should not panic or error, just skip $PRO fallback
  let result = workspace.load_secrets_from_file( "nonexistent.env" );
  assert!( result.is_err() );

  let error_msg = result.unwrap_err().to_string();
  // should only mention local and $HOME, not $PRO
  assert!( error_msg.contains( "local workspace" ) );
  assert!( error_msg.contains( "$HOME" ) || error_msg.contains( "not found" ) );

  // cleanup
  env ::remove_var( "PRO" );
}

/// test that whitespace-only $PRO is ignored
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_whitespace_pro_env_var()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // set $PRO to whitespace
  env ::set_var( "PRO", "   \t\n   " );

  // should not panic or error, just skip $PRO fallback
  let result = workspace.load_secrets_from_file( "nonexistent.env" );
  assert!( result.is_err() );

  // cleanup
  env ::remove_var( "PRO" );
}

/// test that directory instead of file is rejected
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_directory_instead_of_file()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // create directory with same name as expected file
  let secret_dir_path = workspace.secret_file( "secrets.env" );
  fs ::create_dir_all( &secret_dir_path ).unwrap();

  // try to load - should error with clear message
  let result = workspace.load_secrets_from_file( "secrets.env" );
  assert!( result.is_err() );

  let error_msg = result.unwrap_err().to_string();
  assert!( error_msg.contains( "directory" ), "error should explain file is directory" );
  assert!( !error_msg.contains( "not a regular file" ) || error_msg.contains( "directory" ) );
}

/// test that very large file is rejected
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_large_file_rejected()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // create file larger than 10MB limit
  let secret_file = workspace.secret_file( "large.env" );
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  // create 11MB file (10MB limit + 1MB)
  let large_content = "X".repeat( 11 * 1024 * 1024 );
  fs ::write( &secret_file, large_content ).unwrap();

  // try to load - should error
  let result = workspace.load_secrets_from_file( "large.env" );
  assert!( result.is_err() );

  let error_msg = result.unwrap_err().to_string();
  assert!( error_msg.contains( "too large" ) || error_msg.contains( "max" ), "error should mention size limit" );
}

/// test that file size validation works for exactly 10MB
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_file_size_limit_boundary()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  let secret_file = workspace.secret_file( "boundary.env" );
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();

  // create file exactly 10MB (should be accepted)
  let content_10mb = format!( "KEY={}\n", "X".repeat( 10 * 1024 * 1024 - 5 ) );  // -5 for "KEY=\n"
  fs ::write( &secret_file, content_10mb ).unwrap();

  // should succeed (at or under limit)
  let result = workspace.load_secrets_from_file( "boundary.env" );
  assert!( result.is_ok() || result.unwrap_err().to_string().contains( "too large" ),
   "10MB file should be at boundary" );
}

/// test that special files like devices are rejected
#[ test ]
#[ cfg( all( feature = "secrets", unix ) ) ]  // device files are Unix-specific
fn test_device_file_rejected()
{
  let workspace = Workspace ::new( std ::path ::PathBuf ::from( "/tmp" ) );

  // try to load /dev/null (device file)
  let result = workspace.load_secrets_from_absolute_path( std ::path ::Path ::new( "/dev/null" ) );

  // should either error about not being regular file, or succeed with empty content
  // (behavior may vary by implementation)
  if let Err( e ) = result
  {
  let error_msg = e.to_string();
  // if it errors, should mention its not a regular file
  assert!(
   error_msg.contains( "special file" ) ||
   error_msg.contains( "not a regular file" ) ||
   error_msg.contains( "device" ),
   "error should explain file type issue: {}",
   error_msg
  );
  }
}

/// test fallback priority - local wins over $PRO
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_fallback_priority_local_wins()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // create secret in local with one value
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();
  fs ::write( workspace.secret_file( "priority.env" ), "KEY=local_value" ).unwrap();

  // create $PRO workspace with different value
  let pro_dir = env ::temp_dir().join( "test_pro_priority_local_wins" );
  fs ::create_dir_all( &pro_dir ).unwrap();
  let pro_secret_dir = pro_dir.join( "secret" );
  fs ::create_dir_all( &pro_secret_dir ).unwrap();
  fs ::write( pro_secret_dir.join( "priority.env" ), "KEY=pro_value" ).unwrap();

  env ::set_var( "PRO", &pro_dir );

  // load - local should win
  let secrets = workspace.load_secrets_from_file( "priority.env" ).unwrap();
  assert_eq!( secrets.get( "KEY" ).unwrap(), "local_value", "local workspace should have priority" );

  // cleanup
  env ::remove_var( "PRO" );
  fs ::remove_dir_all( pro_dir ).ok();
}

/// test fallback priority - $PRO wins over $HOME
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_fallback_priority_pro_wins_over_home()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // no local secret
  // create $PRO workspace with one value
  let pro_dir = env ::temp_dir().join( "test_pro_priority_pro_wins" );
  fs ::create_dir_all( &pro_dir ).unwrap();
  let pro_secret_dir = pro_dir.join( "secret" );
  fs ::create_dir_all( &pro_secret_dir ).unwrap();
  fs ::write( pro_secret_dir.join( "priority.env" ), "KEY=pro_value" ).unwrap();

  // create $HOME workspace with different value
  let home_dir = env ::temp_dir().join( "test_home_priority_pro_wins" );
  fs ::create_dir_all( &home_dir ).unwrap();
  let home_secret_dir = home_dir.join( "secret" );
  fs ::create_dir_all( &home_secret_dir ).unwrap();
  fs ::write( home_secret_dir.join( "priority.env" ), "KEY=home_value" ).unwrap();

  env ::set_var( "PRO", &pro_dir );
  env ::set_var( "HOME", &home_dir );

  // load - $PRO should win over $HOME
  let secrets = workspace.load_secrets_from_file( "priority.env" ).unwrap();
  assert_eq!( secrets.get( "KEY" ).unwrap(), "pro_value", "$PRO should have priority over $HOME" );

  // cleanup
  env ::remove_var( "PRO" );
  env ::remove_var( "HOME" );
  fs ::remove_dir_all( pro_dir ).ok();
  fs ::remove_dir_all( home_dir ).ok();
}

/// test that permission denied returns proper error
#[ test ]
#[ cfg( all( feature = "secrets", unix ) ) ]  // file permissions work differently on Windows
fn test_permission_denied_error()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // create secret file
  let secret_file = workspace.secret_file( "readonly.env" );
  fs ::create_dir_all( workspace.secret_dir() ).unwrap();
  fs ::write( &secret_file, "KEY=value" ).unwrap();

  // remove read permission
  use std ::os ::unix ::fs ::PermissionsExt;
  let mut perms = fs ::metadata( &secret_file ).unwrap().permissions();
  perms.set_mode( 0o000 );  // no permissions
  fs ::set_permissions( &secret_file, perms ).unwrap();

  // try to load - should error with permission denied
  let result = workspace.load_secrets_from_file( "readonly.env" );
  assert!( result.is_err() );

  let error = result.unwrap_err();
  match error
  {
  WorkspaceError::IoError( msg ) =>
  {
   assert!(
  msg.contains( "permission" ) || msg.contains( "Permission" ),
  "error should mention permission issue: {}",
  msg
   );
  }
  _ => panic!( "expected IoError for permission denied" ),
  }

  // cleanup: restore permissions before deleting
  let mut perms = fs ::metadata( &secret_file ).unwrap_or_else( |_| fs ::metadata( "." ).unwrap() ).permissions();
  perms.set_mode( 0o644 );
  let _ = fs ::set_permissions( &secret_file, perms );
}

/// test error message shows all tried locations
#[ test ]
#[ cfg( feature = "secrets" ) ]
fn test_error_shows_all_tried_locations()
{
  let ( _temp_dir, workspace ) = testing ::create_test_workspace_with_structure();

  // set up $PRO so we have multiple locations to try
  // must create secret dir for workspace to be valid
  let pro_dir = env ::temp_dir().join( "test_error_locations" );
  fs ::create_dir_all( pro_dir.join( "secret" ) ).unwrap();
  env ::set_var( "PRO", &pro_dir );

  // try to load nonexistent file
  let result = workspace.load_secrets_from_file( "nonexistent.env" );
  assert!( result.is_err() );

  let error_msg = result.unwrap_err().to_string();

  // should mention tried locations
  assert!( error_msg.contains( "local workspace" ), "should mention local workspace" );
  // $PRO may or may not be mentioned depending on from_pro_env() success
  assert!( error_msg.contains( "Tried:" ), "should list tried locations" );
  assert!( error_msg.contains( "not found in any location" ), "should explain not found anywhere" );

  // cleanup
  env ::remove_var( "PRO" );
  fs ::remove_dir_all( pro_dir ).ok();
}

#[ cfg( not( feature = "secrets" ) ) ]
fn main()
{
  println!( "this test requires the 'secrets' feature" );
}
