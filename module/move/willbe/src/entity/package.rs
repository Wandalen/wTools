mod private
{
  use crate::*;

  use std::{ hash::Hash, collections, path };
  // qqq : for Petro : for Bohdan : group uses

  use crates_tools::CrateArchive;
  use workspace::Workspace;
  use
  {
    iter::Itertools,
    error::
    {
      Result,
      typed::Error,
      untyped::format_err,
    }
  };

  // aaa : fro Bohdan : write better description : is it better?
  /// A wrapper type for representing the name of a package.
  ///
  /// This struct encapsulates a `String` that holds the name of a package.
  #[ derive
  (
    Debug, Default, Clone, Hash, Ord, PartialOrd, Eq, PartialEq,
    derive_tools::Display, derive_tools::Deref, derive_tools::From, derive_tools::AsRef,
  ) ]
  pub struct PackageName( String );

  // aaa : fro Bohdan : write description : done
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
    pub fn manifest_file( &self ) -> ManifestFile
    {
      match self
      {
        Self::Manifest( package ) => package.manifest_file.clone(),
        Self::WorkspacePackageRef( package ) => package.manifest_file().unwrap(),
      }
    }

    /// Path to folder with `Cargo.toml`
    pub fn crate_dir( &self ) -> CrateDir
    {
      match self
      {
        Self::Manifest( package ) => package.crate_dir(),
        Self::WorkspacePackageRef( package ) => package.crate_dir().unwrap(),
      }
    }

    /// Package version
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

  // aaa : for Bohdan : should not be here


  // aaa : bad : move out to publish.rs
  // zzz : watch


  /// Sorting variants for dependencies.
  #[ derive( Debug, Copy, Clone ) ]
  pub enum DependenciesSort
  {
    /// List will be topologically sorted.
    Topological,
    /// List will be unsorted.
    Unordered,
  }

  #[ derive( Debug, Clone ) ]
  /// Args for `local_dependencies` function.
  pub struct DependenciesOptions
  {
    /// With dependencies of dependencies.
    pub recursive : bool,
    /// With sorting.
    pub sort : DependenciesSort,
    /// Include dev dependencies.
    pub with_dev : bool,
    /// Include remote dependencies.
    pub with_remote : bool,
  }

  impl Default for DependenciesOptions
  {
    fn default() -> Self
    {
      Self
      {
        recursive : true,
        sort : DependenciesSort::Unordered,
        with_dev : false,
        with_remote : false,
      }
    }
  }

  //

  /// Identifier of any crate (local and remote).
  #[ derive( Debug, Clone, Hash, Eq, PartialEq ) ]
  pub struct CrateId
  {
    /// The name of the crate.
    pub name : String,
    /// The absolute path to the crate, if available.
    pub crate_dir : Option< CrateDir >,
    // pub path : Option< AbsolutePath >,
  }

  impl< 'a > From< &WorkspacePackageRef< 'a > > for CrateId
  {
    fn from( value : &WorkspacePackageRef< 'a > ) -> Self
    {
      Self
      {
        name : value.name().into(),
        crate_dir : Some( value.crate_dir().unwrap() )
        // path : Some( AbsolutePath::try_from( value.manifest_file().parent().unwrap() ).unwrap() ),
      }
    }
  }

  impl From< &DependencyRef< '_ > > for CrateId
  {
    fn from( value : &DependencyRef< '_ > ) -> Self
    {
      Self
      {
        name : value.name().into(),
        crate_dir : value.crate_dir(),
        // path : value.path().clone().map( | path | AbsolutePath::try_from( path ).unwrap() ),
      }
    }
  }

  // qqq : for Bohdan : move out
  /// Recursive implementation of the `dependencies` function
  pub fn _dependencies< 'a >
  (
    workspace : &Workspace, // aaa : for Bohdan : no mut // aaa : no mut
    package : &Package< 'a >,
    graph : &mut collections::HashMap< CrateId, collections::HashSet< CrateId > >,
    opts : DependenciesOptions
  ) -> Result< CrateId >
  {
    let DependenciesOptions
    {
      recursive,
      sort : _,
      with_dev,
      with_remote,
    } = opts;
    if recursive && with_remote { unimplemented!( "`recursive` + `with_remote` options") }

    let manifest_file = &package.manifest_file();

    let package = workspace
    .package_find_by_manifest( &manifest_file )
    .ok_or( format_err!( "Package not found in the workspace with path : `{}`", manifest_file.as_ref().display() ) )?;

    let deps : collections::HashSet< _ > = package
    .dependencies()
    // .iter()
    .filter( | dep | ( with_remote || dep.crate_dir().is_some() ) && ( with_dev || dep.kind() != DependencyKind::Development ) )
    .map( | dep | CrateId::from( &dep ) )
    .collect();

    let package = CrateId::from( &package );
    graph.insert( package.clone(), deps.clone() );

    if recursive
    {
      for dep in deps
      {
        if graph.get( &dep ).is_none()
        {
          // unwrap because `recursive` + `with_remote` not yet implemented
          _dependencies
          (
            workspace,
            &dep.crate_dir.unwrap().try_into()?,
            // &dep.path.as_ref().unwrap().join( "Cargo.toml" ).try_into().unwrap(),
            graph,
            opts.clone(),
          )?;
        }
      }
    }

    Ok( package )
  }

  /// Returns local dependencies of a specified package by its package path from a workspace.
  ///
  /// # Arguments
  ///
  /// - `workspace` - holds cached information about the workspace, such as the packages it contains and their dependencies. By passing it as a mutable reference, function can update the cache as needed.
  /// - `package` - The package package file contains package about the package such as its name, version, and dependencies.
  /// - `opts` - used to specify options or configurations for fetching local dependencies.
  ///
  /// # Returns
  ///
  /// If the operation is successful, returns a vector of `PathBuf` objects, where each `PathBuf` represents the path to a local dependency of the specified package.
  pub fn dependencies< 'a >
  (
    workspace : &mut Workspace,
    package : &Package< 'a >,
    opts : DependenciesOptions
  )
  -> Result< Vec< CrateId > >
  {
    let mut graph = collections::HashMap::new();
    let root = _dependencies( workspace, package, &mut graph, opts.clone() )?;

    let output = match opts.sort
    {
      DependenciesSort::Unordered =>
      {
        graph
        .into_iter()
        .flat_map( | ( id, dependency ) |
        {
          dependency
          .into_iter()
          .chain( Some( id ) )
        })
        .unique()
        .filter( | x | x != &root )
        .collect()
      }
      DependenciesSort::Topological =>
      {
        graph::toposort( graph::construct( &graph ) ).map_err( | err | format_err!( "{}", err ) )?.into_iter().filter( | x | x != &root ).collect()
      },
    };

    Ok( output )
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

  pub fn publish_need< 'a >( package : &Package< 'a >, path : Option< path::PathBuf > ) -> Result< bool, PackageError >
  {
    let name = package.name()?;
    let version = package.version()?;
    let local_package_path = path
    .map( | p | p.join( format!( "package/{0}-{1}.crate", name, version ) ) )
    .unwrap_or( packed_crate::local_path( &name, &version, package.crate_dir() ).map_err( | _ | PackageError::LocalPath )? );

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

  protected use Package;
  protected use PackageName;
  protected use PackageError;

  protected use publish_need;

  protected use CrateId;
  protected use DependenciesSort;
  protected use DependenciesOptions;
  protected use dependencies;

}
