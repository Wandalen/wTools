use assert_fs::prelude::*;

use super::*;
use std::fs;
use std::fs::create_dir;
use the_module::action::workspace_renew;
use the_module::action::WorkspaceTemplate;

fn arrange( sample_dir : &str ) -> assert_fs::TempDir
{
  let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
  let assets_relative_path = std::path::Path::new( ASSET_PATH );
  let assets_path = root_path.join( assets_relative_path );

  let temp = assert_fs::TempDir::new().unwrap();
  temp.copy_from( assets_path.join( sample_dir ), &[ "**" ] ).unwrap();
  temp
}

#[ test ]
fn default_case()
{
  // Arrange
  let temp = assert_fs::TempDir::new().unwrap();
  let temp_path = temp.join( "test_project_name" );
  create_dir(temp.join("test_project_name" )).unwrap();

  // Act
  _ = workspace_renew( &temp.path().join( "test_project_name" ), WorkspaceTemplate::default(), "https://github.con/Username/TestRepository".to_string(), vec![ "master".to_string() ] ).unwrap();

  // Assets
  assert!( temp_path.join( "module" ).exists() );
  assert!( temp_path.join( "Readme.md" ).exists() );
  assert!( temp_path.join( ".gitattributes" ).exists() );
  assert!( temp_path.join( ".gitignore" ).exists() );
  assert!( temp_path.join( ".gitpod.yml" ).exists() );
  assert!( temp_path.join( "Cargo.toml" ).exists() );

  let actual = fs::read_to_string(temp_path.join( "Cargo.toml" ) ).unwrap();

  let name = "project_name = \"test_project_name\"";
  let repo_url = "repo_url = \"https://github.con/Username/TestRepository\"";
  let branches = "branches = [\"master\"]";
  assert!( actual.contains( &name) );
  assert!( actual.contains( &repo_url) );
  assert!( actual.contains( &branches) );

  assert!( temp_path.join( "Makefile" ).exists() );
  assert!( temp_path.join( ".cargo" ).exists() );
  assert!( temp_path.join( ".cargo/config.toml" ).exists() );
}

#[ test ]
fn non_empty_dir()
{
  // Arrange
  let temp = arrange( "single_module" );

  // Act
  let r = workspace_renew( temp.path(), WorkspaceTemplate::default(), "".to_string(), vec![] );

  // Assert
  assert!( r.is_err() );
}
