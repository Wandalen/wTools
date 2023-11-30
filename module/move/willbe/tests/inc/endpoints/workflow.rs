const ASSETS_PATH : &str = "tests/assets";

use assert_fs::prelude::*;
use crate::TheModule::endpoint::{ self, list::* };

//

// a -> b -> c
mod workflow_generate
{
  use std::{fs::File, io::Read};

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

    // Act
    let output = dbg!(endpoint::workflow_generate( &temp ).unwrap() );

    let entries = std::fs::read_dir(temp.path().join(".github").join("workflows")).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            dbg!(path.display());
            let mut content = String::new();
            let mut file = File::open(path).unwrap();
            _ = file.read_to_string(&mut content).unwrap();
            dbg!(content);
        }
    }

    // Assert
    assert!(false);
  }
}


