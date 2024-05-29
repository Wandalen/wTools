
// xxx2 : incorporate the function into a tool

pub const ASSET_PATH : &str = "tests/asset";

macro_rules! ERR_MSG
{
  ()
  =>
  {
    "Create `.cargo/config.toml` file at root of your project and append it by
```
[env]
WORKSPACE_PATH = { value = \".\", relative = true }
```"
  };
}

pub fn path() -> std::io::Result< std::path::PathBuf >
{
  use std::
  {
    path::Path,
    io::{ self, ErrorKind }
  };
  let workspace_path = Path::new( env!( "WORKSPACE_PATH", ERR_MSG!{} ) );
  // dbg!( workspace_path );
  // let crate_path = Path::new( env!( "CARGO_MANIFEST_DIR" ) );
  // dbg!( file!() );
  let dir_path = workspace_path.join( Path::new( file!() ) );
  let dir_path = dir_path.canonicalize()?;
  let test_dir = dir_path
  .parent()
  .ok_or_else( || io::Error::new( ErrorKind::NotFound, format!( "Failed to find parent directory {}", dir_path.display() ) ) )?
  .parent()
  .ok_or_else( || io::Error::new( ErrorKind::NotFound, format!( "Failed to find parent directory {}", dir_path.display() ) ) )?
  .parent()
  .ok_or_else( || io::Error::new( ErrorKind::NotFound, format!( "Failed to find parent directory {}", dir_path.display() ) ) )?
  ;
  // dbg!( &test_dir );
  let assets_path = test_dir.join( Path::new( ASSET_PATH ) );
  // dbg!( &assets_path );
  Ok( assets_path )
}

//

// xxx2 : adjust Former to generate required code easier
// xxx2 : implement the interface

use former::Former;
use std::
{
  path::{ Path, PathBuf },
  // process::Command,
};

#[ derive( Debug, Default, Former ) ]
pub struct SourceFile
{
  file_path : PathBuf,
  data : GetData,
}

#[ derive( Debug, Default, Former ) ]
pub struct Entry
{
  source_file : SourceFile,
  typ : EntryType,
}

#[ derive( Debug, Default, Former ) ]
pub struct CargoFile
{
  file_path : PathBuf,
  data : GetData,
}

#[ derive( Debug, Default, Former ) ]
// #[ debug ]
pub struct Program
{
  write_path : Option< PathBuf >,
  read_path : Option< PathBuf >,
  entries : Vec< Entry >,
  sources : Vec< SourceFile >,
  cargo_file : Option< CargoFile >,
}

#[ derive( Debug, Default, Former ) ]
pub struct ProgramRun
{
  // #[ embed ]
  program : Program,
  calls : Vec< ProgramCall >,
}

#[ derive( Debug ) ]
pub enum GetData
{
  FromStr( &'static str ),
  FromBin( &'static [ u8 ] ),
  FromFile( PathBuf ),
  FromString( String ),
}

impl Default for GetData
{
  fn default() -> Self
  {
    GetData::FromStr( "" )
  }
}

#[ derive( Debug, Default ) ]
pub struct ProgramCall
{
  action : ProgramAction,
  current_path : Option< PathBuf >,
  args : Vec< String >,
  index_of_entry : i32,
}

#[ derive( Debug, Default ) ]
pub enum ProgramAction
{
  #[ default ]
  Run,
  Build,
  Test,
}

#[ derive( Debug, Default ) ]
pub enum EntryType
{
  #[ default ]
  Bin,
  Lib,
  Test,
}
