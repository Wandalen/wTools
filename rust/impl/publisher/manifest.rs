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
    fs::write( &self.manifest_path, self.manifest_data.as_ref().unwrap().to_string() ).unwrap_or_else
    (
      | err |
      {
        eprintln!( "{}", err );
        process::exit( -1 );
      }
    );
    Ok( () )
  }
}
