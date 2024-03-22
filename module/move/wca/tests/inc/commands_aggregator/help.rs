use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use assert_fs::fixture::PathCopy;

const ASSET_PATH : &str = concat!( env!("CARGO_MANIFEST_DIR"), "/tests/assets/" );


fn arrange( source: &str ) -> assert_fs::TempDir
{
  let root_path = Path::new( env!( "CARGO_MANIFEST_DIR" ) );
  let assets_relative_path = Path::new( ASSET_PATH );
  let assets_path = root_path.join( assets_relative_path );

  let temp = assert_fs::TempDir::new().unwrap();
  temp.copy_from( assets_path.join( source ), &[ "**" ] ).unwrap();

  temp
}
pub fn start_sync< AP, Args, Arg, P >
(
  application : AP,
  args: Args,
  path : P,
) -> String where AP : AsRef< Path >, Args : IntoIterator< Item = Arg >, Arg : AsRef< std::ffi::OsStr >, P : AsRef< Path >,
{
  let ( application, path ) = ( application.as_ref(), path.as_ref() );
  let args = args.into_iter().map( | a | a.as_ref().into() ).collect::< Vec< std::ffi::OsString > >();
  let child = Command::new( application ).args( &args ).stdout( Stdio::piped() ).stderr( Stdio::piped() ).current_dir( path ).spawn().unwrap();
  let output = child.wait_with_output().unwrap();
  String::from_utf8( output.stdout ).unwrap()
}

#[ test ]
fn help_command_with_optional_params()
{
  let toml = format!
  (
    r#"[package]
name = "wca_hello_test"
version = "0.1.0"
edition = "2021"
[dependencies]
wca = {{path = "{}"}}"#,
    env!( "CARGO_MANIFEST_DIR" ).replace( "\\", "/" )
  ) ;

  let temp = arrange( "wca_hello_test" );
  let mut file = File::create( temp.path().join( "Cargo.toml" ) ).unwrap();
  file.write_all( toml.as_bytes() ).unwrap();
  let result = start_sync( "cargo", [ "r", ".help", "echo" ], temp.path() );

  assert_eq!
  (
    "echo < subjects >  < properties > - prints all subjects and properties\n\nSubjects:\n\t- Subject [String] ?\nProperties:\n\tproperty - simple property [String] ?\n",
    result
  );
}
