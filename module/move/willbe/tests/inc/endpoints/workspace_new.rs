use assert_fs::prelude::*;

use crate::TheModule::endpoint;

const ASSETS_PATH : &str = "tests/assets";

//

mod workspace_new
{

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

    // Act
    _ = workspace_new( temp.path() ).unwrap();
    
    // Assets
    assert!( temp.path().join( "module" ).exists() );
    assert!( temp.path().join( "Readme.md" ).exists() );
    assert!( temp.path().join( ".gitattributes" ).exists() );
    assert!( temp.path().join( ".gitignore" ).exists() );
    assert!( temp.path().join( ".gitpod.yml" ).exists() );
    assert!( temp.path().join( "Cargo.toml" ).exists() );
    assert!( temp.path().join( "Makefile" ).exists() );
    assert!( temp.path().join( "assets" ).exists() );
    assert!( temp.path().join( "docs" ).exists() );
    assert!( temp.path().join( ".github" ).exists() );
    assert!( temp.path().join( ".github/workflows" ).exists() );
    assert!( temp.path().join( ".circleci" ).exists() );
    assert!( temp.path().join( ".circleci/config.yml" ).exists() );
    assert!( temp.path().join( ".cargo" ).exists() );
    assert!( temp.path().join( ".cargo/config.toml" ).exists() );
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
