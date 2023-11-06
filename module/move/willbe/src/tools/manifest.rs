/// Internal namespace.
pub( crate ) mod private
{
  use std::fs;
  use std::env;
  use std::process;
  use std::path::PathBuf;
  use anyhow::*;

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
        let mut current_dir = env::current_dir().context( "Try to take current dir for relative manifest" )?;
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
      let read = fs::read_to_string( &self.manifest_path ).context( "Read manifest" )?;
      let result = read.parse::< toml_edit::Document >().context( "Pars manifest" )?;
      self.manifest_data = Some( result );

      Ok( () )
    }

    /// Store manifest.
    pub fn store( &self ) -> anyhow::Result< () >
    {
      let data = self.manifest_data.as_ref().ok_or( anyhow!( "Manifest data wasn't loaded" ) )?.to_string();
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
      let data = self.manifest_data.as_ref().expect( "Manifest data wasn't loaded" );
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
                     || data[ "package" ][ "publish" ].as_bool().unwrap();
        return !remote;
      }

      true
    }
  }

  pub fn get( path : impl Into< PathBuf > ) -> anyhow::Result< Manifest >
  {
    let mut manifest = Manifest::new();
    manifest.manifest_path_from_str( path )?;
    manifest.load()?;

    Ok( manifest )
  }
}

//

crate::mod_interface!
{
  prelude use Manifest;
  protected( crate ) use get;
}
