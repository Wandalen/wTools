//!
//! A module to manipulate manifest data.
//!

use std::fs;
use std::env;
use std::process;
use std::path::PathBuf;

///
/// Hold manifest data.
///

#[ derive( Debug ) ]
pub struct Manifest
{
  /// Path to `Cargo.toml`
  pub manifest_path : PathBuf,
  /// Strict type of `Cargo.toml` manifest.
  pub manifest_data : Option< toml_edit::Document >,
}

impl Manifest
{
  /// Create instance.
  pub fn new() -> Self
  {
    Manifest
    {
      manifest_path : PathBuf::default(),
      manifest_data : None,
    }
  }

  /// Join manifest path.
  pub fn manifest_path_from_str( &mut self, path : impl Into< PathBuf > ) -> anyhow::Result< PathBuf >
  {
    let mut path_buf : PathBuf = path.into();
    if path_buf.is_relative()
    {
      let mut current_dir = env::current_dir()?;
      current_dir.push( path_buf );
      path_buf = current_dir;
    }

    if !path_buf.ends_with( "Cargo.toml" )
    {
      path_buf.push( "Cargo.toml" );
    }
    self.manifest_path = path_buf.clone();
    Ok( path_buf )
  }

  /// Load manifest from path.
  pub fn load( &mut self ) -> anyhow::Result< () >
  {
    let read = fs::read_to_string( &self.manifest_path )?;
    let result = read.parse::< toml_edit::Document >()?;
    self.manifest_data = Some( result );
    Ok( () )
  }

  /// Store manifest.
  pub fn store( &self ) -> anyhow::Result< () >
  {
    let data = self.manifest_data.as_ref().unwrap().to_string();
    println!( "Saved manifest data to {:?}\n", &self.manifest_path );
    println!( "{}", &data );

    fs::write( &self.manifest_path, &data ).unwrap_or_else
    (
      | err |
      {
        eprintln!( "{}", err );
        process::exit( -1 );
      }
    );
    Ok( () )
  }

  /// Check that current manifest is manifest for a package.
  pub fn package_is( &self ) -> bool
  {
    let data = self.manifest_data.as_ref().unwrap();
    if data.get( "package" ).is_some() && data[ "package" ].get( "name" ).is_some()
    {
      return true;
    }
    false
  }

  /// Check that module is local.
  pub fn local_is( &self ) -> bool
  {
    let data = self.manifest_data.as_ref().unwrap();
    if data.get( "package" ).is_some() && data[ "package" ].get( "name" ).is_some()
    {
      let remote = data[ "package" ].get( "publish" ).is_none()
                   || data[ "package" ][ "publish" ].as_bool().unwrap() == true;
      return !remote;
    }

    true
  }
}

// qqq : for Dima : use mod_interface
