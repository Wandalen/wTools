#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

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
  pub manifest_data : Option<toml_edit::Document>,
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
  pub fn manifest_path_from_str( &mut self, path : &str ) -> anyhow::Result<PathBuf>
  {
    let mut dst_path = env::current_dir()?;
    match path
    {
      "." | "./" => dst_path.push( "Cargo.toml" ),
      _ =>
      {
        dst_path.push( path );
        dst_path.push( "Cargo.toml" );
      },
    }
    self.manifest_path = dst_path.clone();
    Ok( dst_path )
  }

  /// Load manifest from path.
  pub fn load( &mut self ) -> anyhow::Result<()>
  {
    let read = fs::read_to_string( &self.manifest_path )?;
    let result = read.parse::<toml_edit::Document>()?;
    self.manifest_data = Some( result );
    Ok( () )
  }

  /// Store manifest.
  pub fn store( &self ) -> anyhow::Result<()>
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

  pub fn package_is( &self ) -> bool
  {
    let data = self.manifest_data.as_ref().unwrap();
    if data.get( "package" ).is_some() && data[ "package" ].get( "name" ).is_some()
    {
      return true;
    }
    false
  }

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
