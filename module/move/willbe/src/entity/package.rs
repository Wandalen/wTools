#[ allow( clippy::std_instead_of_alloc, clippy::std_instead_of_core ) ]
mod private
{

  use crate::*;
  use std::hash::Hash;
  use crates_tools::CrateArchive;
  use error::
  {
    // Result,
    typed::Error,
  };
  
  // Explicit import for Result and its variants for pattern matching
  use std::result::Result::{self, Ok, Err};

  /// A wrapper type for representing the name of a package.
  ///
  /// This struct encapsulates a `String` that holds the name of a package.
  #[ derive
  (
    Debug, Default, Clone, Hash, Ord, PartialOrd, Eq, PartialEq,
    derive_tools::Display, derive_tools::Deref, derive_tools::From, derive_tools::AsRef,
  ) ]
  pub struct PackageName( String );

  //
  /// Represents different types of packages in a Cargo workspace.
  ///
  /// It is designed to accommodate the two primary types of package
  /// representations within a Cargo workspace.
  #[ derive( Debug, Clone ) ]
  pub enum Package< 'a >
  {

    /// `Cargo.toml` file.
    Manifest( Box< Manifest > ), // fix clippy
    /// Cargo package package.
    WorkspacePackageRef( WorkspacePackageRef< 'a > ),
  }

  /// Represents errors related to package handling.
  #[ derive( Debug, Error ) ]
  pub enum PackageError
  {
    /// Manifest error.
    #[ error( "Manifest error. Reason : {0}." ) ]
    Manifest( #[ from ] manifest::ManifestError ),
    /// Fail to load package.
    #[ error( "Fail to load package." ) ]
    WorkspacePackageRef,
    /// Fail to load remote package.
    #[ error( "Fail to load remote package." ) ]
    LoadRemotePackage,
    /// Fail to get crate local path.
    #[ error( "Fail to get crate local path." ) ]
    LocalPath,
    /// Fail to read archive
    #[ error( "Fail to read archive" ) ]
    ReadArchive,
    /// Try to identify something as a package.
    #[ error( "Not a package" ) ]
    NotAPackage,
  }

  // fix clippy
  impl TryFrom< ManifestFile > for Package< '_ >
  {
    type Error = PackageError;

    fn try_from( value : ManifestFile ) -> Result< Self, Self::Error >
    {
      let package = Manifest::try_from( value )?;
      if !package.package_is()
      {
        return Err( PackageError::NotAPackage );
      }

      Result::Ok( Self::Manifest( Box::new( package ) ) ) // fix clippy
    }
  }

  impl TryFrom< CrateDir > for Package< '_ > // fix clippy
  {
    type Error = PackageError;

    fn try_from( value : CrateDir ) -> Result< Self, Self::Error >
    {
      let package = Manifest::try_from( value )?;
      if !package.package_is()
      {
        return Err( PackageError::NotAPackage );
      }

      Result::Ok( Self::Manifest( Box::new( package ) ) ) // fix clippy
    }
  }

  impl TryFrom< Manifest > for Package< '_ > // fix clippy
  {
    type Error = PackageError;

    fn try_from( value : Manifest ) -> Result< Self, Self::Error >
    {
      if !value.package_is()
      {
        return Err( PackageError::NotAPackage );
      }

      Result::Ok( Self::Manifest( Box::new( value ) ) ) // fix clippy
    }
  }

  impl< 'a > From< WorkspacePackageRef< 'a > > for Package< 'a >
  {
    fn from( value : WorkspacePackageRef< 'a > ) -> Self
    {
      Self::WorkspacePackageRef( value )
    }
  }

  impl Package< '_ > // fix clippy
  {

    /// Path to `Cargo.toml`
    /// # Panics
    /// qqq: doc
    #[ must_use ]
    pub fn manifest_file( &self ) -> ManifestFile
    {
      match self
      {
        Self::Manifest( package ) => package.manifest_file.clone(),
        Self::WorkspacePackageRef( package ) => package.manifest_file().unwrap(),
      }
    }

    /// Path to folder with `Cargo.toml`
    /// # Panics
    /// qqq: doc
    #[ must_use ]
    pub fn crate_dir( &self ) -> CrateDir
    {
      match self
      {
        Self::Manifest( package ) => package.crate_dir(),
        Self::WorkspacePackageRef( package ) => package.crate_dir().unwrap(),
      }
    }

    /// Package version
    /// # Errors
    /// qqq: doc
    /// # Panics
    /// qqq: doc
    pub fn version( &self ) -> Result< String, PackageError >
    {
      match self
      {
        Self::Manifest( package ) =>
        {
          // let data = package.data.as_ref().ok_or_else( || PackageError::Manifest( ManifestError::EmptyManifestData ) )?;
          let data = &package.data;

          // Unwrap safely because of the `Package` type guarantee
          Result::Ok( data[ "package" ][ "version" ].as_str().unwrap().to_string() )
        }
        Self::WorkspacePackageRef( package ) =>
        {
          Result::Ok( package.version().to_string() )
        }
      }
    }

    /// Check that module is local.
    #[ must_use ]
    pub fn local_is( &self ) -> bool
    {
      match self
      {
        Self::Manifest( package ) =>
        {
          // verify that package not empty
          package.local_is()
        }
        Self::WorkspacePackageRef( package ) =>
        {
          !( package.publish().is_none() || package.publish().as_ref().is_some_and( | p | p.is_empty() ) )
          // Ok( !( package.publish().is_none() || package.publish().as_ref().is_some_and( | p | p.is_empty() ) ) )
        }
      }
    }

    /// Returns the `Manifest`
    /// # Errors
    /// qqq: doc
    pub fn manifest( &self ) -> Result< Manifest, PackageError >
    {
      match self
      {
        Package::Manifest( package ) => Ok( *package.clone() ), // fix clippy
        Package::WorkspacePackageRef( package ) => Manifest::try_from
        (
          package.manifest_file().map_err( | _ | PackageError::LocalPath )? // qqq : use trait
        )
        .map_err( | _ | PackageError::WorkspacePackageRef ),
      }
    }

  }

  //

  /// Determines if a package needs to be published by comparing its local `.crate` file against the version on crates.io.
  ///
  /// This function first locates the local, pre-packaged `.crate` file and then attempts to download
  /// the corresponding version from the remote registry. It returns `true` if there are differences
  /// or if the remote version does not exist (implying a new version to be published).
  ///
  /// **Prerequisite**: The local package must have been packaged beforehand (e.g., using `cargo package`).
  ///
  /// # Arguments
  ///
  /// * `package` - A reference to the `Package` struct for which the check is being performed.
  /// * `path` - An optional path to a directory that contains the packaged `.crate` file.
  ///   If `Some`, this path is used directly. If `None`, the path is constructed using `target_dir`.
  /// * `target_dir` - The path to the workspace's `target` directory, used to find the
  ///   local `.crate` file if a specific `path` is not provided.
  ///
  /// # Returns
  ///
  /// - `Ok(true)` if the local and remote `.crate` files have differences, or if the package
  ///   version does not exist on crates.io (e.g., a 403 Forbidden error is received).
  /// - `Ok(false)` if the local and remote packages are identical.
  ///
  /// # Errors
  ///
  /// This function will return an error in the following cases:
  ///
  /// - `PackageError::LocalPath`: If the path to the local `.crate` file cannot be determined.
  /// - `PackageError::ReadArchive`: If the local `.crate` file exists but cannot be read.
  /// - `PackageError::LoadRemotePackage`: If downloading the remote package fails for reasons
  ///   other than a non-existent version (e.g., network issues).
  /// - Any error that occurs while trying to read the package's name or version.
  pub fn publish_need( package : &Package< '_ >, path : Option< path::PathBuf >, target_dir : &std::path::Path ) -> Result< bool, PackageError >
  {
    let name = package.name()?;
    let version = package.version()?;
    let local_package_path = path
    .map( | p | p.join( format!( "package/{name}-{version}.crate" ) ) )
    .unwrap_or( packed_crate::local_path( name, &version, target_dir ).map_err( | _ | PackageError::LocalPath )? );

    let local_package = CrateArchive::read( local_package_path ).map_err( | _ | PackageError::ReadArchive )?;
    let remote_package = match CrateArchive::download_crates_io( name, version )
    {
      Ok( archive ) => archive,
      // qqq : fix. we don't have to know about the http status code
      Err( ureq::Error::Status( 403, _ ) ) => return Result::Ok( true ),
      _ => return Err( PackageError::LoadRemotePackage ),
    };

    Result::Ok( diff::crate_diff( &local_package, &remote_package ).exclude( diff::PUBLISH_IGNORE_LIST ).has_changes() )
  }

}

//

crate::mod_interface!
{

  exposed use Package;
  own use PackageName;
  own use PackageError;

  own use publish_need;

}
