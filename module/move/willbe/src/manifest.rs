/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use std::{ fs, process, path::PathBuf };
  use std::path::Path;
  use wtools::error;
  use wtools::error::for_app::{ anyhow, Context };
  use path::AbsolutePath;

  /// Path to crate directory
  #[ derive( Debug, Clone ) ]
  pub struct CrateDir( AbsolutePath );

  impl AsRef< Path > for CrateDir
  {
    fn as_ref( &self ) -> &Path
    {
      self.0.as_ref()
    }
  }

  impl TryFrom< AbsolutePath > for CrateDir
  {
    // qqq : make better errors
    type Error = error::for_app::Error;

    fn try_from( crate_dir_path : AbsolutePath ) -> Result< Self, Self::Error >
    {
      if !crate_dir_path.as_ref().join( "Cargo.toml" ).exists()
      {
        return Err( anyhow!( "The path is not a crate directory path" ) );
      }

      Ok( Self( crate_dir_path ) )
    }
  }

  ///
  /// Hold manifest data.
  ///

  #[ derive( Debug, Clone ) ]
  pub struct Manifest
  {
    /// Path to `Cargo.toml`
    pub manifest_path : AbsolutePath,
    /// Strict type of `Cargo.toml` manifest.
    pub manifest_data : Option< toml_edit::Document >,
  }

  impl TryFrom< AbsolutePath > for Manifest
  {
    // qqq : make better errors
    type Error = error::for_app::Error;

    fn try_from( manifest_path : AbsolutePath ) -> Result< Self, Self::Error >
    {
      if !manifest_path.as_ref().ends_with( "Cargo.toml" )
      {
        return Err( anyhow!( "The path is not a manifest path" ) );
      }

      Ok
      (
        Manifest
        {
          manifest_path,
          manifest_data : None,
        }
      )
    }
  }

  impl From< CrateDir > for Manifest
  {
    fn from( value : CrateDir ) -> Self
    {
      Self
      {
        manifest_path : value.0.join( "Cargo.toml" ),
        manifest_data : None,
      }
    }
  }

  impl Manifest
  {
    /// Returns path to `Cargo.toml`.
    pub fn manifest_path( &self ) -> &AbsolutePath
    {
      &self.manifest_path
    }

    /// Path to directory where `Cargo.toml` located.
    pub fn crate_dir( &self ) -> CrateDir
    {
      CrateDir( self.manifest_path.parent().unwrap() )
    }

    /// Load manifest from path.
    pub fn load( &mut self ) -> error::for_app::Result< () >
    {
      let read = fs::read_to_string( &self.manifest_path ).context( "Read manifest" )?;
      let result = read.parse::< toml_edit::Document >().context( "Pars manifest" )?;
      self.manifest_data = Some( result );

      Ok( () )
    }

    // qqq : for Bohdan : don't abuse anyhow
    /// Store manifest.
    pub fn store( &self ) -> error::for_app::Result< () >
    {
      let data = self.manifest_data.as_ref().ok_or( anyhow!( "Manifest data wasn't loaded" ) )?.to_string();
      println!( "Saved manifest data to {:?}\n", &self.manifest_path );
      println!( "{}", &data );

      // qqq : for Bohdan : make proper errors handling
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
    // qqq : for Bohdan : poor description, what else could it be?
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
    // qqq : for Bohdan : poor description, how?
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

  /// Create and load manifest by specified path
  // qqq : for Bohdan : use newtype, add proper errors handing
  pub fn open( path : impl Into< PathBuf > ) -> error::for_app::Result< Manifest >
  {
    let path = AbsolutePath::try_from( path.into() )?;
    let mut manifest = if let Ok( dir ) = CrateDir::try_from( path.clone() )
    {
      Manifest::from( dir )
    }
    else
    {
      Manifest::try_from( path )?
    };

    manifest.load()?;

    Ok( manifest )
  }

}

//

crate::mod_interface!
{
  orphan use Manifest;
  orphan use CrateDir;
  protected use open;
}
