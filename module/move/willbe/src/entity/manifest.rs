/// Internal namespace.
mod private
{
  use crate::*;

  use std::
  {
    io::{ self, Read },
    fs,
  };
  use error::
  {
    typed::Error,
    untyped::{ format_err },
  };

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
    // pub manifest_file : AbsolutePath,
    pub manifest_file : ManifestFile,
    // aaa : for Bohdan : for Petro : why not ManifestFile?
    /// Strict type of `Cargo.toml` manifest.
    pub data : toml_edit::Document,
    // pub data : Option< toml_edit::Document >,
  }

  impl TryFrom< ManifestFile > for Manifest
  {
    type Error = ManifestError;

    fn try_from( manifest_file : ManifestFile ) -> Result< Self, Self::Error >
    {

      let read = fs::read_to_string( &manifest_file )?;
      let data = read.parse::< toml_edit::Document >()
      .map_err( | e | io::Error::new( io::ErrorKind::InvalidData, e ) )?;

      Ok
      (
        Manifest
        {
          manifest_file,
          data,
        }
      )
    }
  }

  impl TryFrom< CrateDir > for Manifest
  {
    type Error = ManifestError;

    fn try_from( src : CrateDir ) -> Result< Self, Self::Error >
    {
      Self::try_from( src.manifest_file() )
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
    pub fn manifest_file( &self ) -> &AbsolutePath
    {
      &self.manifest_file
    }

    /// Path to directory where `Cargo.toml` located.
    pub fn crate_dir( &self ) -> CrateDir
    {
      self.manifest_file.parent().unwrap().try_into().unwrap()
      // CrateDir( self.manifest_file.parent().unwrap() )
    }

    /// Store manifest.
    pub fn store( &self ) -> io::Result< () >
    {
      fs::write( &self.manifest_file, self.data.to_string() )?;

      Ok( () )
    }

    /// Check that the current manifest is the manifest of the package (can also be a virtual workspace).
    pub fn package_is( &self ) -> bool
    {
      // let data = self.data.as_ref().ok_or_else( || ManifestError::EmptyManifestData )?;
      let data = &self.data;
      data.get( "package" ).is_some() && data[ "package" ].get( "name" ).is_some()
    }

    /// Check that module is local.
    /// The package is defined as local if the `publish` field is set to `false' or the registers are specified.
    pub fn local_is( &self ) -> bool
    {
      // let data = self.data.as_ref().ok_or_else( || ManifestError::EmptyManifestData )?;
      let data = &self.data;
      if data.get( "package" ).is_some() && data[ "package" ].get( "name" ).is_some()
      {
        let remote = data[ "package" ].get( "publish" ).is_none()
        || data[ "package" ][ "publish" ].as_bool().or( Some( true ) ).unwrap();

        return !remote;
      }
      true
    }
  }

  /// Retrieves the repository URL of a package from its `Cargo.toml` file.
  // qqq : use typed error
  pub fn repo_url( crate_dir : &CrateDir ) -> error::untyped::Result< String >
  {
    let path = crate_dir.clone().manifest_file().inner().inner();
    if path.exists()
    {
      let mut contents = String::new();
      // qqq : zzz : for Petro : redundant read and parse
      fs::File::open( path )?.read_to_string( &mut contents )?;
      let doc = contents.parse::< toml_edit::Document >()?;

      let repo_url = doc
      .get( "package" )
      .and_then( | package | package.get( "repository" ) )
      .and_then( | i | i.as_str() );
      if let Some( repo_url ) = repo_url
      {
        url::repo_url_extract( repo_url ).ok_or_else( || format_err!( "Fail to extract repository url ") )
      }
      else
      {
        let report = tool::git::ls_remote_url( crate_dir.clone().absolute_path() )?;
        url::repo_url_extract( &report.out.trim() ).ok_or_else( || format_err!( "Fail to extract repository url from git remote.") )
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
  own use repo_url;
}
