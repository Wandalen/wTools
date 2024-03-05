use std::env::consts::EXE_EXTENSION;
use std::ffi::OsString;
use std::path::{ Path, PathBuf };
use std::process::Command;
use std::sync::Once;
use super::TheModule::*;


fn workspace_dir() -> PathBuf 
{
  let output = Command::new( env!( "CARGO" ) )
  .arg( "locate-project" )
  .arg( "--workspace" )
  .arg( "--message-format=plain" )
  .output()
  .unwrap()
  .stdout;
  let cargo_path = Path::new( std::str::from_utf8( &output ).unwrap().trim() );
  cargo_path
  .parent()
  .unwrap()
  .to_path_buf()
}

pub fn path_to_exe( name : &str ) -> PathBuf 
{
  static CARGO_BUILD_ONCE: Once = Once::new();
  CARGO_BUILD_ONCE.call_once
  (
    || 
    { 
      let build_status = Command::new("cargo")
      .arg("build")
      .arg("--quiet")
      .status()
      .unwrap();
      assert!
      (
        build_status.success(), 
        "Cargo failed to build associated binaries."
      ); 
    }
  );
  
  workspace_dir()
  .join( "target" )
  .join( "debug" )
  .join( name )
  .with_extension( EXE_EXTENSION )
}

#[ test ]
fn err_first()
{
  let args: [ OsString ; 0 ] = [];
  let report = process::start3_sync( path_to_exe( "err_first" ), args, workspace_dir() ).unwrap().out;
  assert_eq!( "This is stderr text\nThis is stdout text\n", report );
}

#[ test ]
fn out_first()
{
  let args: [ OsString ; 0 ] = [];
  let report = process::start3_sync( path_to_exe( "out_first" ), args, workspace_dir() ).unwrap().out;
  assert_eq!( "This is stdout text\nThis is stderr text\n", report );
}

