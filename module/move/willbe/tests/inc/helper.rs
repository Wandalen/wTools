use super::*;
use the_module::*;
use path::{ Path, PathBuf };
use std::
{
  fs::{ self, File },
  io::Write,
};

pub const BINARY_NAME : &'static str = "will";

#[ derive( Debug ) ]
pub struct ProjectBuilder
{
  name : String,
  lib_content : Option< String >,
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

  pub fn build< P : AsRef< Path > >( &self, path : P ) -> std::io::Result< PathBuf >
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

pub struct WorkspaceBuilder
{
  pub members : Vec< ProjectBuilder >,
  pub toml_content : String,
}

impl WorkspaceBuilder
{
  pub fn new() -> Self
  {
    Self
    {
      members : vec![],
      toml_content : "[workspace]\nresolver = \"2\"\nmembers = [\n    \"modules/*\",\n]\n".to_string(),
    }
  }

  pub fn member( mut self, project : ProjectBuilder ) -> Self
  {
    self.members.push( project );
    self
  }

  pub fn build<  P : AsRef< Path > >( self, path : P ) -> PathBuf
  {
    let project_path = path.as_ref();
    fs::create_dir_all( project_path.join( "modules" ) ).unwrap();
    let mut file = File::create( project_path.join( "Cargo.toml" ) ).unwrap();
    write!( file, "{}", self.toml_content ).unwrap();
    for member in self.members
    {
      member.build( project_path.join( "modules" ).join( &member.name ) ).unwrap();
    }
    project_path.into()
  }
}
