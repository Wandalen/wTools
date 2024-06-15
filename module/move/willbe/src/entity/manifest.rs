/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  use std::
  {
    io::{ self, Read },
    fs,
    path::{ Path },
  };
  use wtools::error::
  {
    Result,
    thiserror,
    for_lib::Error,
    for_app::format_err,
  };
  use _path::AbsolutePath;

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
    // pub manifest_path : ManifestFile, // xxx
    // qqq : for Bohdan : for Petro : why not CrateDir?
    /// Strict type of `Cargo.toml` manifest.
    pub data : toml_edit::Document,
    // pub data : Option< toml_edit::Document >,
  }

  impl TryFrom< AbsolutePath > for Manifest
  {
    type Error = ManifestError;

    // xxx
    fn try_from( manifest_path : AbsolutePath ) -> Result< Self, Self::Error >
    {
      if !manifest_path.as_ref().ends_with( "Cargo.toml" )
      {
        let err =  io::Error::new( io::ErrorKind::NotFound, format!( "Cannot find manifest at {manifest_path:?}" ) );
        return Err( ManifestError::Io( err ) );
      }

      let read = fs::read_to_string( &manifest_path )?;
      let data = read.parse::< toml_edit::Document >()
      .map_err( | e | io::Error::new( io::ErrorKind::InvalidData, e ) )?;

      Ok
      (
        Manifest
        {
          manifest_path,
          data,
        }
      )
    }
  }

  impl TryFrom< CrateDir > for Manifest
  {
    type Error = ManifestError;

    // qqq : xxx : introduce ManifestPath
    fn try_from( src : CrateDir ) -> Result< Self, Self::Error >
    {
      Self::try_from( src.manifest_path() )
      // Self
      // {
      //   manifest_path : src.inner().join( "Cargo.toml" ),
      //   data : None,
      // }
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
      // if self.data.is_none() { self.load().unwrap() }
      // self.data.as_mut().unwrap()
      &mut self.data
    }

    /// Returns path to `Cargo.toml`.
    pub fn manifest_path( &self ) -> &AbsolutePath
    {
      &self.manifest_path
    }

    /// Path to directory where `Cargo.toml` located.
    pub fn crate_dir( &self ) -> CrateDir
    {
      self.manifest_path.parent().unwrap().try_into().unwrap()
      // CrateDir( self.manifest_path.parent().unwrap() )
    }

    // /// Load manifest from path.
    // pub fn load( &mut self ) -> Result< (), ManifestError >
    // {
    //   let read = fs::read_to_string( &self.manifest_path )?;
    //   let result = read.parse::< toml_edit::Document >().map_err( | e | io::Error::new( io::ErrorKind::InvalidData, e ) )?;
    //   self.data = Some( result );
    //   Ok( () )
    // }

    // aaa : for Bohdan : don't abuse anyhow
    // aaa : return `io` error
    /// Store manifest.
    pub fn store( &self ) -> io::Result< () >
    {
      // If the `data` doesn't contain any data, then there's no point in attempting to write
      // if let Some( data ) = &self.data
      // {
        fs::write( &self.manifest_path, self.data.to_string() )?;
      // }

      Ok( () )
    }

    /// Check that the current manifest is the manifest of the package (can also be a virtual workspace).
    // pub fn package_is( &self ) -> Result< bool, ManifestError >
    pub fn package_is( &self ) -> bool
    {
      // let data = self.data.as_ref().ok_or_else( || ManifestError::EmptyManifestData )?;
      let data = &self.data;
      data.get( "package" ).is_some() && data[ "package" ].get( "name" ).is_some()
      // {
      //   return true;
      //   // return Ok( true );
      // }
      // Ok( false )
    }

    /// Check that module is local.
    /// The package is defined as local if the `publish` field is set to `false' or the registers are specified.
    // pub fn local_is( &self ) -> Result<bool, ManifestError>
    pub fn local_is( &self ) -> bool
    {
      // let data = self.data.as_ref().ok_or_else( || ManifestError::EmptyManifestData )?;
      let data = &self.data;
      if data.get( "package" ).is_some() && data[ "package" ].get( "name" ).is_some()
      {
        let remote = data[ "package" ].get( "publish" ).is_none()
        || data[ "package" ][ "publish" ].as_bool().or( Some( true ) ).unwrap();
        // .ok_or_else
        // (
        //   || ManifestError::CannotFindValue( "[package], [publish]".into() )
        // )?;
        // qqq : for Bohdan : bad. logic was wrong
        // In a Cargo.toml file, the package.publish field is used to control whether a package can be published to a registry like crates.io. By default, if the package.publish field is not specified, the package is allowed to be published. In other words, the default behavior is equivalent to having package.publish set to true.
        return !remote;
      }
      true
    }
  }

//   /// Create and load manifest by specified path
//   pub fn open( path : AbsolutePath ) -> Result< Manifest, ManifestError >
//   {
//     // xxx
//     let mut manifest = if let Ok( dir ) = CrateDir::try_from( path.clone() )
//     {
//       Manifest::from( dir )
//     }
//     else
//     {
//       Manifest::try_from( path )?
//     };
//
//     manifest.load()?;
//
//     Ok( manifest )
//   }

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
  orphan use ManifestError;
  // protected use open;
  protected use repo_url;
}
