const ASSETS_PATH : &str = "tests/assets";

use assert_fs::prelude::*;
use crate::TheModule::endpoint::
{ 
  self, 
};

//

mod workflow_generate
{
  use std::
  {
    fs::File, 
    io::Read, 
    collections::HashMap
  };
  use std::fs::{create_dir, create_dir_all};
  use serde::Deserialize;

  use super::*;

  fn arrange( sample_dir: &str ) -> assert_fs::TempDir
  {
    let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
    let assets_relative_path = std::path::Path::new( ASSETS_PATH );
    let assets_path = root_path.join( assets_relative_path );

    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from( assets_path.join( sample_dir ), &[ "**" ] ).unwrap();
    create_dir_all(temp.path().join(".github").join("workflows")).unwrap();
    temp
  }

  #[ derive( Debug, PartialEq, Deserialize ) ]
  struct Workflow 
  {
    name: String,
    on: String,
    env: HashMap< String, String >,
    jobs: HashMap< String, Job >,
  }
  
  #[ derive( Debug, PartialEq, Deserialize ) ]
  struct Job 
  {
    uses: String,
    with: With,
  }
  
  #[ derive( Debug, PartialEq, Deserialize ) ]
  struct With 
  {
    manifest_path: String,
    module_name: String,
    commit_message: String,
  }
  
  // qqq for Petro: this test does not work
  // error: called `Result::unwrap()` on an `Err` value: No such file or directory (os error 2)
  #[ test ]
  fn default_case()
  {
    // Arrange
    let temp = arrange( "single_module" );
    let base_path = temp.path().join( ".github" ).join( "workflows" );
    let file_path = base_path.join( "ModuleTestModulePush.yml" );
    let with = With
    { 
      manifest_path: "test_module/Cargo.toml".into(), 
      module_name: "test_module".into(), 
      commit_message: "${{ github.event.head_commit.message }}".into() 
    };
    let job = Job
    { 
      uses: "Username/test/.github/workflows/StandardRustPush.yml@alpha".into(), 
      with 
    };
    let expected = Workflow
    {
      name: "test_module".into(),
      on: "push".into(),
      env: HashMap::from_iter( [ ( "CARGO_TERM_COLOR".to_string(), "always".to_string() ) ] ),
      jobs: HashMap::from_iter( [ ( "test".to_string(), job ) ] ),
    };

    // Act
    _ = endpoint::workflow_generate( &temp ).unwrap();

    // Assert
    let mut file = File::open( file_path ).unwrap();
    let mut content = String::new();
    _ = file.read_to_string( &mut content ).unwrap();
    let actual: Workflow = serde_yaml::from_str( &content ).unwrap();
    assert_eq!( expected, actual );

    assert!( base_path.join( "AppropriateBranch.yml" ).exists() );
    assert!( base_path.join( "AppropriateBranchBeta.yml" ).exists() );
    assert!( base_path.join( "AppropriateBranchMaster.yml" ).exists() );
    assert!( base_path.join( "AutoMergeToBeta.yml" ).exists() );
    assert!( base_path.join( "AutoPr.yml" ).exists() );
    assert!( base_path.join( "AutoPrToAlpha.yml" ).exists() );
    assert!( base_path.join( "AutoPrToBeta.yml" ).exists() );
    assert!( base_path.join( "AutoPrToMaster.yml" ).exists() );
    assert!( base_path.join( "RunsClean.yml" ).exists() );
    assert!( base_path.join( "StandardRustPullRequest.yml" ).exists() );
    assert!( base_path.join( "StandardRustPush.yml" ).exists() );
    assert!( base_path.join( "StandardRustScheduled.yml" ).exists() );
    assert!( base_path.join( "StandardRustStatus.yml" ).exists() );
    assert!( base_path.join( "StatusChecksRulesUpdate.yml" ).exists() );
  }
}
