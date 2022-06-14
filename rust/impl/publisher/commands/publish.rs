
use crate::*;
use crate::wpublisher::bool::*;
use wca::*;
use std::env;
use std::fs;
use core::fmt::Write;
use std::path::PathBuf;
use toml_edit::value;
use cargo_metadata::MetadataCommand;
use wtools::error::BasicError;

///
/// Publish package.
///

pub fn publish( instruction : &instruction::Instruction ) -> Result< (), BasicError >
{
  let current_path = env::current_dir().unwrap();

  let paths = files::find( &current_path, instruction.subject.split( " " ).collect::<Vec<&str>>().as_slice() );
  let mut paths = paths.iter().filter_map( | s | if s.ends_with( "Cargo.toml" ) { Some( s.into() ) } else { None } ).collect::<Vec<PathBuf>>();
  if paths.is_empty() /* && !path.glob_is( &instruction.subject ) qqq : implement `glob_is` */
  {
    paths.push( PathBuf::from( &instruction.subject ) );
  }

  for path in paths
  {
    let mut manifest = manifest_get( path ).unwrap();
    if !manifest.package_is() || manifest.local_is()
    {
      continue;
    }
    let data = manifest.manifest_data.as_deref_mut().unwrap();

    let mut package_dir = manifest.manifest_path.clone();
    package_dir.pop();

    let output = process::start_sync( "cargo package", &package_dir ).unwrap();
    process::log_output( &output );

    let name = &data[ "package" ][ "name" ].clone();
    let name = name.as_str().unwrap();
    let version = &data[ "package" ][ "version" ].clone();
    let version = version.as_str().unwrap();
    let local_package_path = local_package_path_get( name, version, &manifest.manifest_path );

    let local_package = fs::read( &local_package_path ).unwrap();
    let remote_package = http::retrieve_bytes( name, version ).unwrap_or_default();

    let digest_of_local = digest::hash( &local_package );
    let digest_of_remote = digest::hash( &remote_package );

    if digest_of_local != digest_of_remote
    {
      data[ "package" ][ "version" ] = bump( version ).unwrap();
      let version = &data[ "package" ][ "version" ].clone();
      let version = version.as_str().unwrap();
      manifest.store().unwrap();

      let dry = match instruction.properties_map.get( "dry" )
      {
        Some( x ) => x.clone().primitive().unwrap().to_bool_like(),
        None => BoolLike::False,
      };

      if dry == BoolLike::True
      {
        let mut buf = String::new();
        write!( &mut buf, "git commit --dry-run -am \"{} v{}\"", name, version ).unwrap();
        let output = process::start_sync( &buf, &current_path ).unwrap();
        process::log_output( &output );

        let output = process::start_sync( "git push --dry-run", &current_path ).unwrap();
        process::log_output( &output );

        let output = process::start_sync( "cargo publish --dry-run --allow-dirty", &package_dir ).unwrap();
        process::log_output( &output );
      }
      else
      {
        let mut buf = String::new();
        write!( &mut buf, "git commit -am \"{} v{}\"", name, version ).unwrap();
        let output = process::start_sync( &buf, &current_path ).unwrap();
        process::log_output( &output );

        let output = process::start_sync( "git push", &current_path ).unwrap();
        process::log_output( &output );

        let output = process::start_sync( "cargo publish", &package_dir ).unwrap();
        process::log_output( &output );
      }
    }
    else
    {
      println!( "Package {} is up to date", name );
    }
  }

  Ok( () )
}

fn manifest_get( path : impl Into<PathBuf> ) -> anyhow::Result<manifest::Manifest>
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
