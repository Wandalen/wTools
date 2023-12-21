/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use std::io::Read;
  use std::{ fs, process, path::PathBuf };
  use std::path::Path;
  use error_tools::for_lib::Error;
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

  #[ derive( Debug, Error ) ]
  pub enum  ManifestError
  {
    #[ error( "Manifest data not loaded." ) ]
    EmptyManifestData,
    #[ error( "Cannot find tag {0} in toml file." ) ]
    CannotFindValue(String),
  }
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

    /// Check that the current manifest is the manifest of the package (can also be a virtual workspace).
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
    /// The package is defined as local if the `publish` field is set to `false' or the registers are specified.
    pub fn local_is( &self ) -> Result<bool, ManifestError>
    {
      let data = self.manifest_data.as_ref().ok_or_else( || ManifestError::EmptyManifestData )?;
      if data.get( "package" ).is_some() && data[ "package" ].get( "name" ).is_some()
      {
        let remote = data[ "package" ].get( "publish" ).is_none()
                     || data[ "package" ][ "publish" ].as_bool().ok_or_else( || ManifestError::CannotFindValue( "[package], [publish]".into() ) )?;
        return Ok(!remote);
      }
      Ok(true)
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

  /// Retrieves the repository URL of a package from its `Cargo.toml` file.
  pub fn repo_url( package_path: &Path ) -> error::for_app::Result< String >
  {
    let path = package_path.join( "Cargo.toml" );
    if path.exists() 
    {
      let mut contents = String::new();
      std::fs::File::open( path )?.read_to_string( &mut contents )?;
      let doc = contents.parse::< toml_edit::Document >()?;

      let repo_url = doc
      .get( "package" )
      .and_then( | package | package.get( "repository" ) )
      .and_then( | i | i.as_str() );
      if let Some( repo_url ) = repo_url 
      {
        url::extract_repo_url( repo_url ).ok_or_else( || anyhow!( "Fail to extract repository url ") ) 
      }
      else 
      {
        let report = crate::process::start_sync( "git ls-remote --get-url", package_path )?;
        url::extract_repo_url( &report.out.trim() ).ok_or_else( || anyhow!( "Fail to extract repository url from git remote.") )
      }
    }
    else
    {
      Err( anyhow!( "No Cargo.toml found" ) )
    }
  }

}

//

crate::mod_interface!
{
  orphan use Manifest;
  orphan use CrateDir;
  orphan use ManifestError;
  protected use open;
  protected use repo_url;
}
