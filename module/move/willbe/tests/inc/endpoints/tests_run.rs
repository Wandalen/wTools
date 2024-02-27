use std::fs::{ self, File };
use std::io::Write;
use std::path::{ Path, PathBuf };
use assert_fs::TempDir;

use crate::TheModule::*;
use endpoint::test::{ test, TestsArgs };
use endpoint::test::TestReport;
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
  let crate_dir = CrateDir::try_from( abs ).unwrap();

  let args = TestsArgs::former()
  .dir( crate_dir )
  .channels([ cargo::Channel::Stable ])
  .form();

  let rep : TestReport = test( args, false ).unwrap_err().0;
  println!( "========= OUTPUT =========\n{}\n==========================", rep );

  let stable = rep.tests.get( &cargo::Channel::Stable ).unwrap();
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
  let crate_dir = CrateDir::try_from( abs ).unwrap();

  let args = TestsArgs::former()
  .dir( crate_dir )
  .channels([ cargo::Channel::Stable ])
  .form();

  let rep : TestReport = test( args, false ).unwrap_err().0;
  println!( "========= OUTPUT =========\n{}\n==========================", rep );

  let stable = rep.tests.get( &cargo::Channel::Stable ).unwrap();
  let no_features = stable.get( "" ).unwrap();

  assert!( no_features.err.contains( "error" ) && no_features.err.contains( "achtung" ) );
}

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
