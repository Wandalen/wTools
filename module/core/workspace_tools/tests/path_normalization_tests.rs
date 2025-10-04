//! comprehensive tests for workspace path normalization
//!
//! verifies that workspace root paths are properly normalized
//! to remove redundant components like trailing `/.` and `/./`

use workspace_tools ::{ Workspace, WorkspaceError };
use std ::{ env, path ::PathBuf };

#[ test ]
fn test_normalize_trailing_dot()
{
  let temp_dir = tempfile ::tempdir().unwrap();
  let path = temp_dir.path().join( "." );

  let ws = Workspace ::new( path );
  let root_str = ws.root().to_string_lossy();

  assert!(
    !root_str.ends_with( "/." ) && !root_str.ends_with( "\\." ),
    "root should not end with '/.' - got: {root_str}"
  );
}

#[ test ]
fn test_normalize_dot_slash()
{
  let temp_dir = tempfile ::tempdir().unwrap();
  let base = temp_dir.path();
  let path = PathBuf ::from( format!( "{}/./test", base.display() ) );

  let ws = Workspace ::new( path );
  let root_str = ws.root().to_string_lossy();

  assert!(
    !root_str.contains( "/./" ) && !root_str.contains( "\\.\\" ),
    "root should not contain '/./' - got: {root_str}"
  );
}

#[ test ]
fn test_normalize_parent_dir()
{
  let temp_dir = tempfile ::tempdir().unwrap();
  let base = temp_dir.path();
  let path = base.join( "foo" ).join( ".." ).join( "test" );

  let ws = Workspace ::new( path.clone() );
  let root = ws.root();

  // after normalization, foo/.. should be removed
  assert_eq!( root, base.join( "test" ) );
}

#[ test ]
fn test_workspace_root_normalized_from_env() -> Result< (), WorkspaceError >
{
  let temp_dir = tempfile ::tempdir().unwrap();
  let original_dir = env ::current_dir().unwrap();

  // change to temp dir and set WORKSPACE_PATH to "."
  env ::set_current_dir( temp_dir.path() ).unwrap();
  env ::set_var( "WORKSPACE_PATH", "." );

  let ws = Workspace ::resolve()?;
  let root_str = ws.root().to_string_lossy();

  // restore original directory
  env ::set_current_dir( &original_dir ).unwrap();

  assert!(
    !root_str.ends_with( "/." ) && !root_str.ends_with( "\\." ),
    "root should not end with '/.' when WORKSPACE_PATH='.' - got: {root_str}"
  );

  assert!(
    !root_str.contains( "/./" ) && !root_str.contains( "\\.\\" ),
    "root should not contain '/./' - got: {root_str}"
  );

  assert!(
    ws.root().is_absolute(),
    "root should be absolute - got: {:?}",
    ws.root()
  );

  Ok( () )
}

#[ test ]
fn test_workspace_new_normalizes()
{
  let temp_dir = tempfile ::tempdir().unwrap();
  let path = temp_dir.path().join( "." );

  let ws = Workspace ::new( path );
  let root_str = ws.root().to_string_lossy();

  assert!(
    !root_str.ends_with( "/." ) && !root_str.ends_with( "\\." ),
    "new() should normalize paths - got: {root_str}"
  );
}

#[ test ]
fn test_joined_paths_remain_clean() -> Result< (), WorkspaceError >
{
  let temp_dir = tempfile ::tempdir().unwrap();
  let original_dir = env ::current_dir().unwrap();

  env ::set_current_dir( temp_dir.path() ).unwrap();
  env ::set_var( "WORKSPACE_PATH", "." );

  let ws = Workspace ::resolve()?;
  let secret_dir = ws.secret_dir();
  let secret_str = secret_dir.to_string_lossy();

  env ::set_current_dir( &original_dir ).unwrap();

  assert!(
    !secret_str.contains( "/./" ) && !secret_str.contains( "\\.\\" ),
    "joined path should not contain '/./' - got: {secret_str}"
  );

  Ok( () )
}

#[ test ]
fn test_multiple_dot_components()
{
  let temp_dir = tempfile ::tempdir().unwrap();
  let base = temp_dir.path();
  let path = PathBuf ::from( format!( "{}/./foo/./bar/.", base.display() ) );

  let ws = Workspace ::new( path );
  let root_str = ws.root().to_string_lossy();

  assert!(
    !root_str.contains( "/./" ) && !root_str.contains( "\\.\\" ),
    "all './' components should be removed - got: {root_str}"
  );

  assert!(
    !root_str.ends_with( "/." ) && !root_str.ends_with( "\\." ),
    "trailing '.' should be removed - got: {root_str}"
  );
}

#[ test ]
fn test_relative_workspace_path_normalized() -> Result< (), WorkspaceError >
{
  let temp_dir = tempfile ::tempdir().unwrap();
  let original_dir = env ::current_dir().unwrap();

  // create subdirectory
  let subdir = temp_dir.path().join( "subdir" );
  std ::fs ::create_dir_all( &subdir ).unwrap();

  env ::set_current_dir( temp_dir.path() ).unwrap();
  env ::set_var( "WORKSPACE_PATH", "./subdir" );

  let ws = Workspace ::resolve()?;
  let root_str = ws.root().to_string_lossy();

  env ::set_current_dir( &original_dir ).unwrap();

  assert!(
    !root_str.contains( "/./" ) && !root_str.contains( "\\.\\" ),
    "relative path should be normalized - got: {root_str}"
  );

  assert!(
    ws.root().is_absolute(),
    "relative WORKSPACE_PATH should be made absolute - got: {:?}",
    ws.root()
  );

  Ok( () )
}

#[ test ]
fn test_absolute_path_normalized()
{
  let temp_dir = tempfile ::tempdir().unwrap();
  let path = PathBuf ::from( format!( "{}/./test", temp_dir.path().display() ) );

  let ws = Workspace ::new( path );
  let root_str = ws.root().to_string_lossy();

  assert!(
    !root_str.contains( "/./" ) && !root_str.contains( "\\.\\" ),
    "absolute path should be normalized - got: {root_str}"
  );
}

#[ test ]
fn test_normalization_preserves_existing_paths()
{
  let temp_dir = tempfile ::tempdir().unwrap();

  // create the directory so it exists
  let test_dir = temp_dir.path().join( "test" );
  std ::fs ::create_dir_all( &test_dir ).unwrap();

  // create path with redundant components
  let path = test_dir.join( "." );

  let ws = Workspace ::new( path );

  // should be normalized and absolute but not necessarily canonical
  assert!( ws.root().is_absolute() );
  let root_str = ws.root().to_string_lossy();
  assert!( !root_str.ends_with( "/." ) && !root_str.ends_with( "\\." ) );
}

#[ test ]
fn test_normalization_of_nonexistent_paths()
{
  let temp_dir = tempfile ::tempdir().unwrap();
  let nonexistent = temp_dir.path().join( "does_not_exist" ).join( "." );

  let ws = Workspace ::new( nonexistent );
  let root_str = ws.root().to_string_lossy();

  // should still normalize even if path doesnt exist
  assert!(
    !root_str.ends_with( "/." ) && !root_str.ends_with( "\\." ),
    "nonexistent paths should still be normalized - got: {root_str}"
  );
}

#[ test ]
fn test_cargo_workspace_normalized()
{
  // this test only works if we're in a cargo workspace
  if let Ok( ws ) = Workspace ::from_cargo_workspace()
  {
    let root_str = ws.root().to_string_lossy();

    assert!(
      !root_str.contains( "/./" ) && !root_str.contains( "\\.\\" ),
      "cargo workspace root should be normalized - got: {root_str}"
    );

    assert!(
      !root_str.ends_with( "/." ) && !root_str.ends_with( "\\." ),
      "cargo workspace root should not have trailing '.' - got: {root_str}"
    );

    assert!(
      ws.root().is_absolute(),
      "cargo workspace root should be absolute"
    );
  }
}

#[ test ]
fn test_from_current_dir_normalized() -> Result< (), WorkspaceError >
{
  let ws = Workspace ::from_current_dir()?;
  let root_str = ws.root().to_string_lossy();

  assert!(
    !root_str.contains( "/./" ) && !root_str.contains( "\\.\\" ),
    "from_current_dir should return normalized path - got: {root_str}"
  );

  assert!(
    !root_str.ends_with( "/." ) && !root_str.ends_with( "\\." ),
    "from_current_dir should not have trailing '.' - got: {root_str}"
  );

  Ok( () )
}

#[ test ]
fn test_from_git_root_normalized()
{
  // only test if we're in a git repository
  if let Ok( ws ) = Workspace ::from_git_root()
  {
    let root_str = ws.root().to_string_lossy();

    assert!(
      !root_str.contains( "/./" ) && !root_str.contains( "\\.\\" ),
      "from_git_root should return normalized path - got: {root_str}"
    );

    assert!(
      !root_str.ends_with( "/." ) && !root_str.ends_with( "\\." ),
      "from_git_root should not have trailing '.' - got: {root_str}"
    );
  }
}
