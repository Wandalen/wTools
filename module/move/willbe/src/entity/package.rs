#[ allow( clippy::std_instead_of_alloc, clippy::std_instead_of_core ) ]
mod private
{
  #[ allow( clippy::wildcard_imports ) ]
  use crate::*;

  use std::
  {
    hash::Hash,
  };

  use crates_tools::CrateArchive;
  use error::
  {
    // Result,
    typed::Error,
  };

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
    Manifest( Manifest ),
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

  impl< 'a > TryFrom< ManifestFile > for Package< 'a >
  {
    type Error = PackageError;

    fn try_from( value : ManifestFile ) -> Result< Self, Self::Error >
    {
      let package = Manifest::try_from( value )?;
      if !package.package_is()
      {
        return Err( PackageError::NotAPackage );
      }

      Ok( Self::Manifest( package ) )
    }
  }

  impl< 'a > TryFrom< CrateDir > for Package< 'a >
  {
    type Error = PackageError;

    fn try_from( value : CrateDir ) -> Result< Self, Self::Error >
    {
      let package = Manifest::try_from( value )?;
      if !package.package_is()
      {
        return Err( PackageError::NotAPackage );
      }

      Ok( Self::Manifest( package ) )
    }
  }

  impl< 'a > TryFrom< Manifest > for Package< 'a >
  {
    type Error = PackageError;

    fn try_from( value : Manifest ) -> Result< Self, Self::Error >
    {
      if !value.package_is()
      {
        return Err( PackageError::NotAPackage );
      }

      Ok( Self::Manifest( value ) )
    }
  }

  impl< 'a > From< WorkspacePackageRef< 'a > > for Package< 'a >
  {
    fn from( value : WorkspacePackageRef< 'a > ) -> Self
    {
      Self::WorkspacePackageRef( value )
    }
  }

  impl< 'a > Package< 'a >
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
          Ok( data[ "package" ][ "version" ].as_str().unwrap().to_string() )
        }
        Self::WorkspacePackageRef( package ) =>
        {
          Ok( package.version().to_string() )
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
        Package::Manifest( package ) => Ok( package.clone() ),
        Package::WorkspacePackageRef( package ) => Manifest::try_from
        (
          package.manifest_file().map_err( | _ | PackageError::LocalPath )? // qqq : use trait
        )
        .map_err( | _ | PackageError::WorkspacePackageRef ),
      }
    }

  }

  //

  /// Determines whether a package needs to be published by comparing `.crate` files from the local and remote package.
  ///
  /// This function requires the local package to be previously packed.
  ///
  /// # Returns :
  /// - `true` if the package needs to be published.
  /// - `false` if there is no need to publish the package.
  ///
  /// Panics if the package is not loaded or local package is not packed.
  /// # Errors
  /// qqq: doc

  pub fn publish_need( package : &Package< '_ >, path : Option< path::PathBuf > ) -> Result< bool, PackageError >
  {
    let name = package.name()?;
    let version = package.version()?;
    let local_package_path = path
    .map( | p | p.join( format!( "package/{name}-{version}.crate" ) ) )
    .unwrap_or( packed_crate::local_path( name, &version, package.crate_dir() ).map_err( | _ | PackageError::LocalPath )? );

    let local_package = CrateArchive::read( local_package_path ).map_err( | _ | PackageError::ReadArchive )?;
    let remote_package = match CrateArchive::download_crates_io( name, version )
    {
      Ok( archive ) => archive,
      // qqq : fix. we don't have to know about the http status code
      Err( ureq::Error::Status( 403, _ ) ) => return Ok( true ),
      _ => return Err( PackageError::LoadRemotePackage ),
    };

    Ok( diff::crate_diff( &local_package, &remote_package ).exclude( diff::PUBLISH_IGNORE_LIST ).has_changes() )
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
