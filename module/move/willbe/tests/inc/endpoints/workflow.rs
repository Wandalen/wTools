const ASSETS_PATH : &str = "tests/assets";

use assert_fs::prelude::*;
use crate::TheModule::endpoint::{ self, list::* };

//

// a -> b -> c
mod workflow_generate
{
  use std::
  {
    fs::File, 
    io::Read, 
    collections::HashMap
  };

  use super::*;

  fn arrange( sample_dir: &str ) -> assert_fs::TempDir
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
    let temp = arrange( "single_module" );
    let mut expected: HashMap<&str, &str> = HashMap::new();
    expected.insert( "ModuleTestModulePush.yml", "\n          name : test_module\n\n          on : push\n                  \n          env :\n            CARGO_TERM_COLOR : always\n                  \n          jobs :\n                test :\n              uses : Wandalen/wTools/.github/workflows/StandardRustPush.yml@alpha\n              with :\n                manifest_path : 'test_module\\Cargo.toml'\n                module_name : 'test_module'\n                commit_message : ${{ github.event.head_commit.message }}" );

    // Act
    let output = endpoint::workflow_generate( &temp ).unwrap();

    let entries = std::fs::read_dir( temp.path().join( ".github" ).join( "workflows" ) ).unwrap();

    for entry in entries 
    {
      let entry = entry.unwrap();
      let path = entry.path();
      if path.is_file() 
      {
        let mut content = String::new();
        let mut file = File::open( &path ).unwrap();
        _ = file.read_to_string( &mut content ).unwrap();
        assert_eq!( expected.get( path.file_name().unwrap().to_str().unwrap() ).unwrap(), &content.as_str() );
      }
    }
    // Assert
  }
}
