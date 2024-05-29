use super::*;
use assert_fs::prelude::*;
use the_module::action;

//

// aaa : for Petro : rid off redundant namespace. ask
// aaa : remove
use std::
{
  fs::File,
  io::Read,
  collections::HashMap
};
use std::fs::create_dir_all;
use serde::Deserialize;

fn arrange( sample_dir : &str ) -> assert_fs::TempDir
{
  let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
  let assets_relative_path = std::path::Path::new( ASSET_PATH );
  let assets_path = root_path.join( assets_relative_path );

  let temp = assert_fs::TempDir::new().unwrap();
  temp.copy_from( assets_path.join( sample_dir ), &[ "**" ] ).unwrap();
  create_dir_all( temp.path().join( ".github" ).join( "workflows") ).unwrap();
  temp
}

#[ derive( Debug, PartialEq, Deserialize ) ]
struct Workflow
{
  name : String,
  on : HashMap<String, HashMap<String, Vec<String>>>,
  env : HashMap< String, String >,
  jobs : HashMap< String, Job >,
}

#[ derive( Debug, PartialEq, Deserialize ) ]
struct Job
{
  uses : String,
  with : With,
}

#[ derive( Debug, PartialEq, Deserialize ) ]
struct With
{
  manifest_path : String,
  module_name : String,
  commit_message : String,
}

#[ test ]
fn default_case()
{
  // Arrange
  let temp = arrange( "single_module" );
  let base_path = temp.path().join( ".github" ).join( "workflows" );
  let file_path = base_path.join( "module_test_module_push.yml" );
  let with = With
  {
    manifest_path : "test_module/Cargo.toml".into(),
    module_name : "test_module".into(),
    commit_message : "${{ github.event.head_commit.message }}".into()
  };
  let job = Job
  {
    uses : "Username/test/.github/workflows/standard_rust_push.yml@alpha".into(),
    with
  };
  let expected = Workflow
  {
    name : "test_module".into(),
    on : 
    {
      let mut map = HashMap::new();
      let mut push_map = HashMap::new();
      push_map.insert
      (
        "branches".to_string(),
        vec![ "alpha".to_string(), "beta".to_string(), "master".to_string() ],
      );
      map.insert( "push".to_string(), push_map );
      map
    },
    env : HashMap::from_iter( [ ( "CARGO_TERM_COLOR".to_string(), "always".to_string() ) ] ),
    jobs : HashMap::from_iter( [ ( "test".to_string(), job ) ] ),
  };

  // Act
  _ = action::cicd_renew( &temp ).unwrap();

  // Assert
  let mut file = File::open( file_path ).unwrap();
  let mut content = String::new();
  _ = file.read_to_string( &mut content ).unwrap();
  let actual: Workflow = serde_yaml::from_str( &content ).unwrap();
  assert_eq!( expected, actual );

  assert!( base_path.join( "appropriate_branch.yml" ).exists() );
  assert!( base_path.join( "appropriate_branch_beta.yml" ).exists() );
  assert!( base_path.join( "appropriate_branch_master.yml" ).exists() );
  assert!( base_path.join( "auto_merge_to_beta.yml" ).exists() );
  assert!( base_path.join( "auto_pr.yml" ).exists() );
  assert!( base_path.join( "auto_pr_to_alpha.yml" ).exists() );
  assert!( base_path.join( "auto_pr_to_beta.yml" ).exists() );
  assert!( base_path.join( "auto_pr_to_master.yml" ).exists() );
  assert!( base_path.join( "runs_clean.yml" ).exists() );
  assert!( base_path.join( "standard_rust_pull_request.yml" ).exists() );
  assert!( base_path.join( "standard_rust_push.yml" ).exists() );
  assert!( base_path.join( "for_pr_rust_push.yml" ).exists() );
  assert!( base_path.join( "standard_rust_scheduled.yml" ).exists() );
  assert!( base_path.join( "standard_rust_status.yml" ).exists() );
  assert!( base_path.join( "status_checks_rules_update.yml" ).exists() );
  assert!( base_path.join( "Readme.md" ).exists() );
}

// aaa : for Petro : fix styles
// aaa : ✅
