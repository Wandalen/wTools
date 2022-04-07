#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Utility to publish modules on `crates.io` from a command line.
//!

use std::env;
use std::fs;
use wpublisher::*;
use std::fmt::Write;

//

fn publish( instruction : &instruction::Instruction ) -> anyhow::Result<()>
{
  let current_path = env::current_dir()?;

  let manifest = manifest_get( instruction.subject[ 0 ].as_ref() )?;
  let data = manifest.manifest_data.as_ref().unwrap();
  let name = &data[ "package" ][ "name" ].as_str().unwrap();
  let version = &data[ "package" ][ "version" ].as_str().unwrap();

  let mut package_dir = manifest.manifest_path.clone();
  package_dir.pop();
  process::start_sync( "cargo package", package_dir.to_str().unwrap() )?;

  let mut buf = String::new();
  write!( &mut buf, "target/package/{0}-{1}.crate", name, version )?;
  let mut local_package_path = current_path.clone();
  local_package_path.push( buf );

  let local_package = fs::read( &local_package_path )?;
  let remote_package = http::retrieve( name, version )?;

  let digest_of_local = digest::hash( &local_package );
  let digest_of_remote = digest::hash( &remote_package );

  println!("{:?}", digest_of_local);
  println!("{:?}", digest_of_remote);

  manifest.store()?;

  Ok( () )
}

//

fn main() -> anyhow::Result<()>
{
  let instruction = wpublisher::instruction::parse_from_splits( env::args().skip( 1 ) );

  match instruction.command_name.as_ref()
  {
    ".publish" => publish( &instruction )?,
    _ => panic!( "Unknown command" ),
  }

  Ok( () )
}

fn manifest_get( path : &str ) -> anyhow::Result<manifest::Manifest>
{
  let mut manifest = manifest::Manifest::new();
  manifest.manifest_path_from_str( path )?;
  manifest.load()?;
  Ok( manifest )
}
