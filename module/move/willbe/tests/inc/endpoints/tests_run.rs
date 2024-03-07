use std::fs::{ self, File };
use std::io::Write;
use std::path::{ Path, PathBuf };
use assert_fs::TempDir;

use crate::TheModule::*;
use endpoint::test::{test, TestsCommandOptions};
use path::AbsolutePath;

#[ test ]
fn fail_test()
{
  let temp = TempDir::new().unwrap();
  let temp = &temp;

  let project = ProjectBuilder::new( "fail_test" )
  .toml_file( "" )
  .test_file( r#"
    #[test]
    fn should_fail() {
      panic!()
    }
  "#)
  .build( temp )
  .unwrap();
  let abs = AbsolutePath::try_from( project ).unwrap();

  let args = TestsCommandOptions::former()
  .dir( abs )
  .channels([ cargo::Channel::Stable ])
  .form();

  let rep = test( args, false ).unwrap_err().0;
  println!( "========= OUTPUT =========\n{}\n==========================", rep );

  let stable = rep.failure_reports[0].tests.get( &cargo::Channel::Stable ).unwrap();
  let no_features = stable.get( "" ).unwrap();

  assert!( no_features.out.contains( "failures" ) );
}

#[ test ]
fn fail_build()
{
  let temp = TempDir::new().unwrap();
  let temp = &temp;

  let project = ProjectBuilder::new( "fail_build" )
  .lib_file( "compile_error!( \"achtung\" );" )
  .toml_file( "" )
  .test_file( r#"
    #[test]
    fn should_pass() {
      assert!(true);
    }
  "#)
  .build( temp )
  .unwrap();
  let abs = AbsolutePath::try_from( project ).unwrap();

  let args = TestsCommandOptions::former()
  .dir( abs )
  .channels([ cargo::Channel::Stable ])
  .form();

  let rep = test( args, false ).unwrap_err().0;
  println!( "========= OUTPUT =========\n{}\n==========================", rep );

  let stable = rep.failure_reports[ 0 ].tests.get( &cargo::Channel::Stable ).unwrap();
  let no_features = stable.get( "" ).unwrap();

  assert!( no_features.out.contains( "error" ) && no_features.out.contains( "achtung" ) );
}

#[ test ]
fn call_from_workspace_root()
{
  let temp = TempDir::new().unwrap();
  let temp = &temp;

  let fail_project = ProjectBuilder::new( "fail_test" )
  .toml_file( "" )
  .test_file( r#"
  #[test]
  fn should_fail123() {
    panic!()
  }
  "#);

  let pass_project = ProjectBuilder::new( "apass_test" )
  .toml_file( "" )
  .test_file( r#"
  #[test]
  fn should_pass() {
    assert_eq!(1,1);
  }
  "#);

  let pass_project2 = ProjectBuilder::new( "pass_test2" )
  .toml_file( "" )
  .test_file( r#"
  #[test]
  fn should_pass() {
    assert_eq!(1,1);
  }
  "#);

  let workspace = WorkspaceBuilder::new()
  .member( fail_project )
  .member( pass_project )
  .member( pass_project2 )
  .build( temp );

  // from workspace root
  let abs = AbsolutePath::try_from( workspace.clone() ).unwrap();

  let args = TestsCommandOptions::former()
  .dir( abs )
  .concurrent( 1u32 )
  .channels([ cargo::Channel::Stable ])
  .form();


  let rep = test( args, false ).unwrap_err().0;


  assert_eq!( rep.failure_reports.len(), 1 );
  assert_eq!( rep.succses_reports.len(), 2 );
}

#[ derive( Debug ) ]
pub struct ProjectBuilder
{
  name : String,
  lib_content: Option< String >,
  test_content : Option< String >,
  toml_content : Option< String >,
}

impl ProjectBuilder
{
  pub fn new( name : &str ) -> Self
  {
    Self
    {
      name : String::from( name ),
      lib_content : None,
      test_content : None,
      toml_content : None,
    }
  }

  pub fn lib_file< S : Into< String > >( mut self, content : S ) -> Self
  {
    self.lib_content = Some( content.into() );
    self
  }

  pub fn test_file< S : Into< String > >( mut self, content : S ) -> Self
  {
    self.test_content = Some( content.into() );
    self
  }

  pub fn toml_file( mut self, content : &str ) -> Self
  {
    self.toml_content = Some( format!( "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n{}", self.name, content ) );
    self
  }

  pub fn build< P: AsRef< Path > >( &self, path : P ) -> std::io::Result< PathBuf >
  {
    let project_path = path.as_ref();

    fs::create_dir_all( project_path.join( "src" ) )?;
    fs::create_dir_all( project_path.join( "tests" ) )?;

    if let Some( content ) = &self.toml_content
    {
      let mut file = File::create( project_path.join( "Cargo.toml" ) )?;
      write!( file, "{}", content )?;
    }

    let mut file = File::create( project_path.join( "src/lib.rs" ) )?;
    if let Some( content ) = &self.lib_content
    {
      write!( file, "{}", content )?;
    }

    if let Some( content ) = &self.test_content
    {
      let mut file = File::create( project_path.join( "tests/tests.rs" ) )?;
      write!( file, "{}", content )?;
    }

    Ok( project_path.to_path_buf() )
  }
}

struct WorkspaceBuilder
{
  members: Vec< ProjectBuilder >,
  toml_content: String,
}

impl WorkspaceBuilder
{
  fn new() -> Self
  {
    Self
    {
      members: vec![],
      toml_content: "[workspace]\nresolver = \"2\"\nmembers = [\n    \"modules/*\",\n]\n".to_string(),
    }
  }

  fn member( mut self, project : ProjectBuilder ) -> Self
  {
    self.members.push( project );
    self
  }

  fn build<  P: AsRef< Path > >( self, path : P ) -> PathBuf
  {
    let project_path = path.as_ref();
    fs::create_dir_all( project_path.join( "modules" ) ).unwrap();
    let mut file = File::create( project_path.join( "Cargo.toml" ) ).unwrap();
    write!( file, "{}", self.toml_content ).unwrap();
    for member in self.members {
      member.build( project_path.join( "modules" ).join( &member.name ) ).unwrap();
    }
    project_path.into()
  }
}
