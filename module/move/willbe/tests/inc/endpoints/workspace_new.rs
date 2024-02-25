use assert_fs::prelude::*;

use crate::TheModule::endpoint;

const ASSETS_PATH : &str = "tests/assets";

//

mod workspace_new
{
  use std::fs;
  use std::fs::create_dir;
  use endpoint::workspace_new;

  use super::*;

  fn arrange( sample_dir : &str ) -> assert_fs::TempDir
  {
    let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
    let assets_relative_path = std::path::Path::new( ASSETS_PATH );
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
    _ = workspace_new( &temp.path().join("test_project_name" ) ).unwrap();
    
    // Assets
    assert!( temp_path.join( "module" ).exists() );
    assert!( temp_path.join( "Readme.md" ).exists() );
    assert!( temp_path.join( ".gitattributes" ).exists() );
    assert!( temp_path.join( ".gitignore" ).exists() );
    assert!( temp_path.join( ".gitpod.yml" ).exists() );
    assert!( temp_path.join( "Cargo.toml" ).exists() );
    
    let actual = fs::read_to_string(temp_path.join( "Cargo.toml")).unwrap();
    let expected = "project_name = \"test_project_name\"";
    
    assert!( actual.contains( &expected ) );
    assert!( temp_path.join( "Makefile" ).exists() );
    assert!( temp_path.join( "assets" ).exists() );
    assert!( temp_path.join( "docs" ).exists() );
    assert!( temp_path.join( ".github" ).exists() );
    assert!( temp_path.join( ".github/workflows" ).exists() );
    assert!( temp_path.join( ".circleci" ).exists() );
    assert!( temp_path.join( ".circleci/config.yml" ).exists() );
    assert!( temp_path.join( ".cargo" ).exists() );
    assert!( temp_path.join( ".cargo/config.toml" ).exists() );
  }
  
  #[ test ]
  fn non_empty_dir()
  {
    // Arrange
    let temp = arrange( "single_module" );
    
    // Act
    let r = workspace_new( temp.path() );
    
    // Assert
    assert!( r.is_err() );
  }
}
