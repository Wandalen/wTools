mod private
{
  use crate::*;

  use std::
  {
    path::{ Path, PathBuf },
    collections::{ HashMap, HashSet },
  };
  use std::fmt::Formatter;
  use std::hash::Hash;
  use cargo_metadata::{ Dependency, DependencyKind, Package as PackageMetadata };
  use toml_edit::value;

  use tools::process;
  use manifest::{ Manifest, ManifestError };
  // use { cargo, git, version, path, wtools }; // qqq: why is it required?
  use crates_tools::CrateArchive;
  
  use workspace::Workspace;
  use path::AbsolutePath;

  use wtools::
  {
    iter::Itertools,
    error::
    {
      thiserror,
      Result,
      for_lib::Error,
      for_app::{ anyhow, Error as wError, Context },
    }
  };

  ///
  #[ derive( Debug ) ]
  pub enum Package
  {
    /// `Cargo.toml` file.
    Manifest( Manifest ),
    /// Cargo metadata package.
    Metadata( PackageMetadata ),
  }

  /// Represents errors related to package handling.
  #[ derive( Debug, Error ) ]
  pub enum PackageError
  {
    /// Manifest error.
    #[ error( "Manifest error. Reason: {0}." ) ]
    Manifest(ManifestError),
    /// Fail to load metadata.
    #[ error( "Fail to load metadata." ) ]
    Metadata,
    /// Fail to load remote package.
    #[ error( "Fail to load remote package." ) ]
    LoadRemotePackage,
    /// Fail to get crate local path.
    #[ error( "Fail to get crate local path." ) ]
    LocalPath,
    /// Fail to read archive
    #[ error( "Fail to read archive" ) ]
    ReadArchive,
  }

  impl TryFrom< AbsolutePath > for Package
  {
    // qqq : make better errors
    type Error = wError;

    fn try_from( value : AbsolutePath ) -> Result< Self, Self::Error >
    {
      let manifest =  manifest::open( value.as_ref() )?;
      if !manifest.package_is()?
      {
        return Err( anyhow!( "`{}` - not a package", value.as_ref().display() ) );
      }

      Ok( Self::Manifest( manifest ) )
    }
  }

  impl TryFrom< Manifest > for Package
  {
    // qqq : make better errors
    type Error = wError;

    fn try_from( value : Manifest ) -> Result< Self, Self::Error >
    {
      if !value.package_is()?
      {
        return Err( anyhow!( "`{}` - not a package", value.manifest_path.as_ref().display() ) );
      }

      Ok( Self::Manifest( value ) )
    }
  }

  impl From< PackageMetadata > for Package
  {
    fn from( value : PackageMetadata ) -> Self
    {
      Self::Metadata( value )
    }
  }

  impl Package
  {
    /// Path to `Cargo.toml`
    pub fn manifest_path( &self ) -> AbsolutePath
    {
      match self
      {
        Self::Manifest( manifest ) => manifest.manifest_path.clone(),
        Self::Metadata( metadata ) => AbsolutePath::try_from( metadata.manifest_path.as_std_path().to_path_buf() ).unwrap(),
      }
    }

    /// Path to folder with `Cargo.toml`
    pub fn crate_dir( &self ) -> CrateDir
    {
      match self
      {
        Self::Manifest( manifest ) => manifest.crate_dir(),
        Self::Metadata( metadata ) =>
        {
          let path = metadata.manifest_path.parent().unwrap().as_std_path().to_path_buf();
          let absolute = AbsolutePath::try_from( path ).unwrap();

          CrateDir::try_from( absolute ).unwrap()
        },
      }
    }

    /// Package name
    pub fn name( &self ) -> Result< String, PackageError >
    {
      match self
      {
        Self::Manifest( manifest ) =>
        {
          let data = manifest.manifest_data.as_ref().ok_or_else( || PackageError::Manifest( ManifestError::EmptyManifestData ) )?;

          data["package"]["name"]
          .as_str()
          .ok_or_else( || PackageError::Manifest( ManifestError::CannotFindValue(" [package], [name]".into() ) ) )
          .map( | r | r.to_string() )
        }
        Self::Metadata( metadata ) =>
        {
          Ok(metadata.name.clone())
        }
      }
    }

    /// Package version
    pub fn version( &self ) -> Result< String, PackageError >
    {
      match self
      {
        Self::Manifest( manifest ) =>
        {
          let data = manifest.manifest_data.as_ref().ok_or_else( || PackageError::Manifest( ManifestError::EmptyManifestData ) )?;

          data["package"]["version"]
          .as_str()
          .ok_or_else( || PackageError::Manifest( ManifestError::CannotFindValue(" [package], [name]".into() ) ) )
          .map( | r | r.to_string() )
        }
        Self::Metadata( metadata ) =>
        {
          Ok( metadata.version.to_string() )
        }
      }
    }

    /// Check that module is local.
    pub fn local_is( &self ) -> Result< bool, ManifestError >
    {
      match self
      {
        Self::Manifest( manifest ) =>
        {
          manifest.local_is()
        }
        Self::Metadata( metadata ) =>
        {
          Ok( metadata.publish.is_none() || metadata.publish.as_ref().is_some_and( | p | p.is_empty() ) )
        }
      }
    }

    /// Returns the `Manifest`
    pub fn manifest( &self ) -> Result< Manifest, PackageError >
    {
      match self
      {
        Package::Manifest( manifest ) => Ok(manifest.clone()),
        Package::Metadata( metadata ) => manifest::open( metadata.manifest_path.as_std_path() ).map_err( | _ | PackageError::Metadata ),
      }
    }

    /// Returns the `Metadata`
    pub fn metadata( &self ) -> Result< PackageMetadata, PackageError >
    {
      match self
      {
        Package::Manifest( manifest ) =>
        Workspace::with_crate_dir( manifest.crate_dir() ).map_err( | _ | PackageError::Metadata )?
        .package_find_by_manifest( &manifest.manifest_path )
        .ok_or_else( || PackageError::Metadata )
        .cloned(),
        Package::Metadata( metadata ) => Ok( metadata.clone() ),
      }
    }
  }

  /// Describe publishing outcomes.
  #[ derive( Debug, Default, Clone ) ]
  pub struct PublishReport
  {
    get_info : Option< process::CmdReport >,
    publish_required : bool,
    bump : Option< BumpReport >,
    add : Option< process::CmdReport >,
    commit : Option< process::CmdReport >,
    push : Option< process::CmdReport >,
    publish : Option< process::CmdReport >,
  }

  impl std::fmt::Display for PublishReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      let PublishReport
      {
        get_info,
        publish_required,
        bump,
        add,
        commit,
        push,
        publish,
      } = self;

      if get_info.is_none()
      {
        f.write_str( "Empty report" )?;
        return Ok( () )
      }
      let info = get_info.as_ref().unwrap();
      f.write_fmt( format_args!( "{}", info ) )?;

      if !publish_required
      {
        f.write_str( "The package has no changes, so no publishing is required" )?;
        return Ok( () )
      }

      if let Some( bump ) = bump
      {
        f.write_fmt( format_args!( "{}", bump ) )?;
      }
      if let Some( add ) = add
      {
        f.write_fmt( format_args!( "{add}" ) )?;
      }
      if let Some( commit ) = commit
      {
        f.write_fmt( format_args!( "{commit}" ) )?;
      }
      if let Some( push ) = push
      {
        f.write_fmt( format_args!( "{push}" ) )?;
      }
      if let Some( publish ) = publish
      {
        f.write_fmt( format_args!( "{publish}" ) )?;
      }

      Ok( () )
    }
  }

  /// Report about changing version.
  #[ derive( Debug, Default, Clone ) ]
  pub struct BumpReport
  {
    package_name : String,
    new_version : String,
    changed_files : Vec< AbsolutePath >
  }

  impl std::fmt::Display for BumpReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      if self.changed_files.is_empty()
      {
        f.write_str( "Files were not changed during bumping the version" )?;
        return Ok( () )
      }

      let files = self.changed_files.iter().map( | f | f.as_ref().display() ).join( ",\n    " );
      f.write_fmt( format_args!( "`{}` bumped to `{}`\n  changed files:\n    {files}\n", self.package_name, self.new_version ) )?;

      Ok( () )
    }
  }

  /// Publishes a single package without publishing its dependencies.
  ///
  /// This function is designed to publish a single package. It does not publish any of the package's dependencies.
  ///
  /// Args:
  ///
  /// - package - a package that will be published
  /// - dry - a flag that indicates whether to apply the changes or not
  ///   - true - do not publish, but only show what steps should be taken
  ///   - false - publishes the package
  ///
  /// Returns:
  /// Returns a result containing a report indicating the result of the operation.
  pub fn publish_single( package : &Package, dry : bool ) -> Result< PublishReport, ( PublishReport, wError ) >
  {
    let mut report = PublishReport::default();
    if package.local_is().map_err( | err | ( report.clone(), anyhow!( err ) ) )?
    {
      return Ok( report );
    }

    let package_dir = &package.crate_dir();

    let output = cargo::package( &package_dir, false ).context( "Take information about package" ).map_err( | e | ( report.clone(), e ) )?;
    if output.err.contains( "not yet committed")
    {
      return Err(( report, anyhow!( "Some changes wasn't committed. Please, commit or stash that changes and try again." ) ));
    }
    report.get_info = Some( output );

    if publish_need( &package ).map_err( | err | (report.clone(), anyhow!( err ) ) )?
    {
      report.publish_required = true;

      let mut files_changed_for_bump = vec![];
      let mut manifest = package.manifest().map_err( | err | ( report.clone(), anyhow!( err ) ) )?;
      // bump version in the package manifest
      let new_version = version::bump( &mut manifest, dry ).context( "Try to bump package version" ).map_err( | e | ( report.clone(), e ) )?;
      files_changed_for_bump.push( package.manifest_path() );

      let package_name = package.name().map_err( | err | ( report.clone(), anyhow!( err ) ) )?;

      // bump the package version in dependents(so far, only workspace)
      let workspace_manifest_dir : AbsolutePath = Workspace::with_crate_dir( package.crate_dir() ).map_err( | err | ( report.clone(), err ) )?.workspace_root().map_err( | err | ( report.clone(), anyhow!( err ) ) )?.try_into().unwrap();
      let workspace_manifest_path = workspace_manifest_dir.join( "Cargo.toml" );

      // qqq: should be refactored
      if !dry
      {
        let mut workspace_manifest = manifest::open( workspace_manifest_path.as_ref() ).map_err( | e | ( report.clone(), e ) )?;
        let workspace_manifest_data = workspace_manifest.manifest_data.as_mut().ok_or_else( || ( report.clone(), anyhow!( PackageError::Manifest( ManifestError::EmptyManifestData ) ) ) )?;
        workspace_manifest_data
        .get_mut( "workspace" )
        .and_then( | workspace | workspace.get_mut( "dependencies" ) )
        .and_then( | dependencies | dependencies.get_mut( &package_name ) )
        .map
        (
          | dependency |
            {
              if let Some( previous_version ) = dependency.get( "version" ).and_then( | v | v.as_str() ).map( | v | v.to_string() )
              {
                if previous_version.starts_with('~')
                {
                  dependency["version"] = value(format!("~{new_version}"));
                }
              }
            }
        );
        workspace_manifest.store().map_err( | err | ( report.clone(), err ) )?;
      }

      files_changed_for_bump.push( workspace_manifest_path );
      let files_changed_for_bump : Vec< _ > = files_changed_for_bump.into_iter().unique().collect();
      let objects_to_add : Vec< _ > = files_changed_for_bump.iter().map( | f | f.as_ref().strip_prefix( &workspace_manifest_dir ).unwrap().to_string_lossy() ).collect();

      report.bump = Some( BumpReport { package_name : package_name.to_string(), new_version : new_version.clone(), changed_files : files_changed_for_bump.clone() } );

      let commit_message = format!( "{package_name}-v{new_version}" );
      let res = git::add( workspace_manifest_dir, objects_to_add, dry ).map_err( | e | ( report.clone(), e ) )?;
      report.add = Some( res );
      let res = git::commit( package_dir, commit_message, dry ).map_err( | e | ( report.clone(), e ) )?;
      report.commit = Some( res );
      let res = git::push( package_dir, dry ).map_err( | e | ( report.clone(), e ) )?;
      report.push = Some( res );

      let res = cargo::publish( package_dir, dry ).map_err( | e | ( report.clone(), e ) )?;
      report.publish = Some( res );
    }

    Ok( report )
  }

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

  /// Identifier of any crate(local and remote)
  #[ derive( Debug, Clone, Hash, Eq, PartialEq ) ]
  pub struct CrateId
  {
    /// TODO: make it private
    pub name : String,
    /// TODO: make it private
    pub path : Option< AbsolutePath >,
  }

  impl From< &PackageMetadata > for CrateId
  {
    fn from( value : &PackageMetadata ) -> Self
    {
      Self
      {
        name : value.name.clone(),
        path : Some( AbsolutePath::try_from( value.manifest_path.parent().unwrap() ).unwrap() ),
      }
    }
  }

  impl From< &Dependency > for CrateId
  {
    fn from( value : &Dependency ) -> Self
    {
      Self
      {
        name : value.name.clone(),
        path : value.path.clone().map( | path | AbsolutePath::try_from( path ).unwrap() ),
      }
    }
  }

  /// Recursive implementation of the `dependencies` function
  pub fn _dependencies
  (
    workspace : &mut Workspace,
    manifest : &Package,
    graph: &mut HashMap< CrateId, HashSet< CrateId > >,
    opts: DependenciesOptions
  ) -> Result< CrateId >
  {
    let DependenciesOptions
    {
      recursive,
      sort: _,
      with_dev,
      with_remote,
    } = opts;
    if recursive && with_remote { unimplemented!( "`recursive` + `with_remote` options") }

    let manifest_path = &manifest.manifest_path();

    let package = workspace
    .load()?
    .package_find_by_manifest( &manifest_path )
    .ok_or( anyhow!( "Package not found in the workspace with path: `{}`", manifest_path.as_ref().display() ) )?;

    let deps = package
    .dependencies
    .iter()
    .filter( | dep | ( with_remote || dep.path.is_some() ) && ( with_dev || dep.kind != DependencyKind::Development ) )
    .map( CrateId::from )
    .collect::< HashSet< _ > >();

    let package = CrateId::from( package );
    graph.insert( package.clone(), deps.clone() );

    if recursive
    {
      for dep in deps
      {
        if graph.get( &dep ).is_none()
        {
          // unwrap because `recursive` + `with_remote` not yet implemented
          _dependencies( workspace, &dep.path.as_ref().unwrap().join( "Cargo.toml" ).try_into().unwrap(), graph, opts.clone() )?;
        }
      }
    }

    Ok( package )
  }

  /// Returns local dependencies of a specified package by its manifest path from a workspace.
  ///
  /// # Arguments
  ///
  /// - `workspace` - holds cached information about the workspace, such as the packages it contains and their dependencies. By passing it as a mutable reference, function can update the cache as needed.
  /// - `manifest` - The package manifest file contains metadata about the package such as its name, version, and dependencies.
  /// - `opts` - used to specify options or configurations for fetching local dependencies.
  ///
  /// # Returns
  ///
  /// If the operation is successful, returns a vector of `PathBuf` objects, where each `PathBuf` represents the path to a local dependency of the specified package.
  pub fn dependencies( workspace : &mut Workspace, manifest : &Package, opts: DependenciesOptions ) -> Result< Vec< CrateId > >
  {
    let mut graph = HashMap::new();
    let root = _dependencies( workspace, manifest, &mut graph, opts.clone() )?;

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
        graph::toposort( graph::construct( &graph ) ).map_err( | err | anyhow!( "{}", err ) )?.into_iter().filter( | x | x != &root ).collect()
      },
    };

    Ok( output )
  }

  // qqq : for Bohdan : move to file packed_crate as well as relevant functions

  /// Returns the local path of a packed `.crate` file based on its name, version, and manifest path.
  ///
  /// # Args:
  /// - `name` - the name of the package.
  /// - `version` - the version of the package.
  /// - `manifest_path` - path to the package `Cargo.toml` file.
  ///
  /// # Returns:
  /// The local packed `.crate` file of the package
  pub fn local_path< 'a >( name : &'a str, version : &'a str, crate_dir: CrateDir ) -> Result< PathBuf >
  {
    let buf = format!( "package/{0}-{1}.crate", name, version );

    let workspace = Workspace::with_crate_dir( crate_dir )?;

    let mut local_package_path = PathBuf::new();
    local_package_path.push( workspace.target_directory()? );
    local_package_path.push( buf );

    Ok( local_package_path )
  }

  //

  /// A configuration struct for specifying optional filters when using the
  /// `packages_filter_map` function. It allows users to provide custom filtering
  /// functions for packages and dependencies.
  #[ derive( Default ) ]
  pub struct FilterMapOptions
  {
    /// An optional package filtering function. If provided, this function is
    /// applied to each package, and only packages that satisfy the condition
    /// are included in the final result. If not provided, a default filter that
    /// accepts all packages is used.
    pub package_filter: Option< Box< dyn Fn( &PackageMetadata ) -> bool > >,

    /// An optional dependency filtering function. If provided, this function
    /// is applied to each dependency of each package, and only dependencies
    /// that satisfy the condition are included in the final result. If not
    /// provided, a default filter that accepts all dependencies is used.
    pub dependency_filter: Option< Box< dyn Fn( &PackageMetadata, &Dependency ) -> bool  > >,
  }

  impl std::fmt::Debug for FilterMapOptions
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      f
      .debug_struct( "FilterMapOptions" )
      .field( "package_filter", &"package_filter" )
      .field( "dependency_filter", &"dependency_filter" )
      .finish()
    }
  }

  /// Type aliasing for String
  pub type PackageName = String;

  // qqq : for Bohdan : move to packages::filter

  /// Given a slice of `Package` instances and a set of filtering options,
  /// this function filters and maps the packages and their dependencies
  /// based on the provided filters. It returns a `HashMap` where the keys
  /// are package names, and the values are `HashSet` instances containing
  /// the names of filtered dependencies for each package.
  pub fn packages_filter_map( packages : &[ PackageMetadata ], filter_map_options : FilterMapOptions ) -> HashMap< PackageName, HashSet< PackageName > >
  {
    let FilterMapOptions { package_filter, dependency_filter } = filter_map_options;
    let package_filter = package_filter.unwrap_or_else( || Box::new( | _ | true ) );
    let dependency_filter = dependency_filter.unwrap_or_else( || Box::new( | _, _ | true ) );
    packages
    .iter()
    .filter( | &p | package_filter( p ) )
    .map
    (
      | package |
      (
        package.name.clone(),
        package.dependencies
        .iter()
        .filter( | &d | dependency_filter( package, d ) )
        .map( | d | d.name.clone() )
        .collect::< HashSet< _ > >()
      )
    ).collect()
  }

  //

  /// Determines whether a package needs to be published by comparing `.crate` files from the local and remote package.
  ///
  /// This function requires the local package to be previously packed.
  ///
  /// # Returns:
  /// - `true` if the package needs to be published.
  /// - `false` if there is no need to publish the package.
  ///
  /// Panics if the manifest is not loaded or local package is not packed.

  pub fn publish_need( package : &Package ) -> Result< bool, PackageError >
  {
    // These files are ignored because they can be safely changed without affecting functionality
    //
    // - `.cargo_vcs_info.json` - contains the git sha1 hash that varies between different commits
    // - `Cargo.toml.orig` - can be safely modified because it is used to generate the `Cargo.toml` file automatically, and the `Cargo.toml` file is sufficient to check for changes
    const IGNORE_LIST : [ &str; 2 ] = [ ".cargo_vcs_info.json", "Cargo.toml.orig" ];

    let name = package.name()?;
    let version = package.version()?;
    let local_package_path = local_path( &name, &version, package.crate_dir() ).map_err( | _ | PackageError::LocalPath )?;

    // qqq : for Bohdan : bad, properly handle errors
    let local_package = CrateArchive::read( local_package_path ).map_err( | _ | PackageError::ReadArchive )?;
    let remote_package = match CrateArchive::download_crates_io( name, version )
    {
      Ok( archive ) => archive,
      // qqq: fix. we don't have to know about the http status code
      Err( ureq::Error::Status( 403, _ ) ) => return Ok( true ),
      _ => return Err( PackageError::LoadRemotePackage ),
    };

    let filter_ignore_list = | p : &&Path | !IGNORE_LIST.contains( &p.file_name().unwrap().to_string_lossy().as_ref() );
    let local_package_files : Vec< _ > = local_package.list().into_iter().filter( filter_ignore_list ).sorted().collect();
    let remote_package_files : Vec< _ > = remote_package.list().into_iter().filter( filter_ignore_list ).sorted().collect();

    if local_package_files != remote_package_files { return Ok( true ); }

    let mut is_same = true;
    for path in local_package_files
    {
      // unwraps is safe because the paths to the files was compared previously
      let local = local_package.content_bytes( path ).unwrap();
      let remote = remote_package.content_bytes( path ).unwrap();

      is_same &= local == remote;
    }

    Ok( !is_same )
  }

}

//

crate::mod_interface!
{

  protected use PublishReport;
  protected use publish_single;
  protected use local_path;
  protected use PackageName;
  protected use Package;
  protected use PackageError;

  protected use FilterMapOptions;
  protected use packages_filter_map;
  protected use publish_need;

  protected use CrateId;
  protected use DependenciesSort;
  protected use DependenciesOptions;
  protected use dependencies;
}
