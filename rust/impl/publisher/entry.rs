#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Utility to publish modules on `crates.io` from a command line.
//!

use std::env;
use std::fs;
use std::fmt::Write;
use toml_edit::value;
use wpublisher::*;
use cargo_metadata::MetadataCommand;

//

fn publish( instruction : &instruction::Instruction ) -> anyhow::Result<()>
{
  let current_path = env::current_dir()?;

  let mut manifest = manifest_get( instruction.subject[ 0 ].as_ref() )?;
  let data = manifest.manifest_data.as_deref_mut().unwrap();

  let mut package_dir = manifest.manifest_path.clone();
  package_dir.pop();
  let output = process::start_sync( "cargo package", &package_dir )?;
  process::log_output( &output );

  let name = &data[ "package" ][ "name" ].clone();
  let name = name.as_str().unwrap();
  let version = &data[ "package" ][ "version" ].clone();
  let version = version.as_str().unwrap();
  let local_package_path = local_package_path_get( name, version, &manifest.manifest_path );

  let local_package = fs::read( &local_package_path )?;
  let remote_package = http::retrieve_bytes( name, version )?;

  let digest_of_local = digest::hash( &local_package );
  let digest_of_remote = digest::hash( &remote_package );

  if digest_of_local != digest_of_remote
  {
    data[ "package" ][ "version" ] = bump( version )?;
    let version = &data[ "package" ][ "version" ].clone();
    let version = version.as_str().unwrap();
    manifest.store()?;

    let mut buf = String::new();
    write!( &mut buf, "git commit -am \"{} v{}\"", name, version )?;
    let output = process::start_sync( &buf, &current_path )?;
    process::log_output( &output );

    let output = process::start_sync( "git push", &current_path )?;
    process::log_output( &output );

    let output = process::start_sync( "cargo publish", &package_dir )?;
    process::log_output( &output );
  }
  else
  {
    println!( "Package {} is up to date", name );
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

fn local_package_path_get<'a>( name : &'a str, version : &'a str, manifest_path : &'a std::path::PathBuf ) -> std::path::PathBuf
{
  let mut buf = String::new();
  write!( &mut buf, "package/{0}-{1}.crate", name, version ).unwrap();
  let package_metadata = MetadataCommand::new()
  .manifest_path( manifest_path )
  .exec()
  .unwrap();
  let mut local_package_path = std::path::PathBuf::new();
  local_package_path.push( package_metadata.target_directory );
  local_package_path.push( buf );
  local_package_path
}

fn bump( version : &str ) -> anyhow::Result<toml_edit::Item>
{
  let mut splits : Vec<&str> = version.split( "." ).collect();
  let patch_version = splits[ 2 ].parse::<u32>()? + 1;
  let v = &patch_version.to_string();
  splits[ 2 ] = v;
  Ok( value( splits.join( "." ) ) )
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
