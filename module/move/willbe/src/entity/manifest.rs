/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  use std::
  {
    io::{ self, Read },
    fs,
    path::{ Path, PathBuf },
  };
  use wtools::error::
  {
    Result,
    thiserror,
    for_lib::Error,
    for_app::format_err,
  };
  use _path::AbsolutePath;

  /// `CrateDirError` enum represents errors when creating a `CrateDir` object.
  #[ derive( Debug, Error ) ]
  pub enum CrateDirError
  {
    /// Indicates a validation error with a descriptive message.
    #[ error( "Failed to create a `CrateDir` object due to `{0}`" ) ]
    Validation( String ),
  }

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
    // aaa : make better errors
    // aaa : use `CrateDirError` for it
    type Error = CrateDirError;

    fn try_from( crate_dir_path : AbsolutePath ) -> Result< Self, Self::Error >
    {
      if !crate_dir_path.as_ref().join( "Cargo.toml" ).exists()
      {
        return Err( CrateDirError::Validation( "The path is not a crate directory path".into() ) );
      }

      Ok( Self( crate_dir_path ) )
    }
  }
  
  impl TryFrom< PathBuf > for CrateDir
  {
    type Error = CrateDirError;

    fn try_from( crate_dir_path : PathBuf ) -> Result< Self, Self::Error >
    {
      if !crate_dir_path.join( "Cargo.toml" ).exists()
      {
        return Err( CrateDirError::Validation( "The path is not a crate directory path".into() ) );
      }

      Ok( Self( AbsolutePath::try_from( crate_dir_path ).unwrap() ) )
    }
  }

  impl CrateDir
  {
    /// Returns an absolute path.
    pub fn absolute_path( &self ) -> AbsolutePath
    {
      self.0.clone()
    }
  }



/// Represents errors related to manifest data processing.
  #[ derive( Debug, Error ) ]
  pub enum  ManifestError
  {
    /// Manifest data not loaded.
    #[ error( "Manifest data not loaded." ) ]
    EmptyManifestData,
    /// Cannot find the specified tag in the TOML file.
    #[ error( "Cannot find tag {0} in toml file." ) ]
    CannotFindValue( String ),
    /// Try to read or write
    #[ error( "Io operation with manifest failed. Details : {0}" ) ]
    Io( #[ from ] io::Error ),
    /// It was expected to be a package, but it wasn't
    #[ error( "Is not a package" ) ]
    NotAPackage,
    /// It was expected to be a package, but it wasn't
    #[ error( "Invalid value `{0}` in manifest file." ) ]
    InvalidValue( String ),
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
    // aaa : make better errors
    // aaa : return `ManifestError`
    type Error = ManifestError;

    fn try_from( manifest_path : AbsolutePath ) -> Result< Self, Self::Error >
    {
      if !manifest_path.as_ref().ends_with( "Cargo.toml" )
      {
        let err =  io::Error::new( io::ErrorKind::NotFound, "Cannot find manifest" );
        return Err( ManifestError::Io( err ) );
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
    /// Returns a mutable reference to the TOML document.
    ///
    /// If the TOML document has not been loaded yet, this function will load it
    /// by calling the `load` method. If loading fails, this function will panic.
    ///
    /// # Returns
    ///
    /// A mutable reference to the TOML document.
    pub fn data( &mut self ) -> &mut toml_edit::Document
    {
      if self.manifest_data.is_none() { self.load().unwrap() }
      
      self.manifest_data.as_mut().unwrap()
    }
    
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
    pub fn load( &mut self ) -> Result< (), ManifestError >
    {
      let read = fs::read_to_string( &self.manifest_path )?;
      let result = read.parse::< toml_edit::Document >().map_err( | e | io::Error::new( io::ErrorKind::InvalidData, e ) )?;
      self.manifest_data = Some( result );

      Ok( () )
    }

    // aaa : for Bohdan : don't abuse anyhow
    // aaa : return `io` error
    /// Store manifest.
    pub fn store( &self ) -> io::Result< () >
    {
      // If the `manifest_data` doesn't contain any data, then there's no point in attempting to write
      if let Some( data ) = &self.manifest_data
      {
        fs::write( &self.manifest_path, data.to_string() )?;
      }

      Ok( () )
    }

    /// Check that the current manifest is the manifest of the package (can also be a virtual workspace).
    pub fn package_is( &self ) -> Result< bool, ManifestError>
    {
      let data = self.manifest_data.as_ref().ok_or_else( || ManifestError::EmptyManifestData )?;
      if data.get( "package" ).is_some() && data[ "package" ].get( "name" ).is_some()
      {
        return Ok( true );
      }
      Ok( false )
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
  // aaa : for Bohdan : use newtype, add proper errors handing
  // aaa : return `ManifestError`
  pub fn open( path : AbsolutePath ) -> Result< Manifest, ManifestError >
  {
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
  pub fn repo_url( package_path : &Path ) -> Result< String >
  {
    let path = package_path.join( "Cargo.toml" );
    if path.exists()
    {
      let mut contents = String::new();
      fs::File::open( path )?.read_to_string( &mut contents )?;
      let doc = contents.parse::< toml_edit::Document >()?;

      let repo_url = doc
      .get( "package" )
      .and_then( | package | package.get( "repository" ) )
      .and_then( | i | i.as_str() );
      if let Some( repo_url ) = repo_url
      {
        url::extract_repo_url( repo_url ).ok_or_else( || format_err!( "Fail to extract repository url ") )
      }
      else
      {
        let report = git::ls_remote_url( package_path )?;
        url::extract_repo_url( &report.out.trim() ).ok_or_else( || format_err!( "Fail to extract repository url from git remote.") )
      }
    }
    else
    {
      Err( format_err!( "No Cargo.toml found" ) )
    }
  }

}

//

crate::mod_interface!
{
  exposed use Manifest;
  exposed use CrateDir;
  orphan use ManifestError;
  orphan use CrateDirError;
  protected use open;
  protected use repo_url;
}
