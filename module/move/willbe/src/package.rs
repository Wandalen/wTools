mod private
{
  use crate::*;
  use std::
  {
    fs,
    path::{ Path, PathBuf },
    collections::{ HashMap, HashSet },
  };
  use std::fmt::Formatter;
  use std::hash::Hash;
  use cargo_metadata::{ Dependency, DependencyKind, Package };
  use tools::
  {
    process,
    http,
  };
  use manifest::Manifest;
  use { cargo, git, version, path, wtools };
  use wca::wtools::Itertools; // qqq : use wtools::...!
  use wtools::error::for_app::{ anyhow, Error, Context };
  use workspace::Workspace;

  /// Describe publishing outcomes.
  #[ derive( Debug, Default, Clone ) ]
  pub struct PublishReport
  {
    get_info : Option< process::CmdReport >,
    bump : Option< String >,
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
        bump,
        add,
        commit,
        push,
        publish,
      } = self;

      if get_info.is_none()
      {
        f.write_fmt( format_args!( "Empty report" ) )?;
        return Ok( () )
      }
      let info = get_info.as_ref().unwrap();
      f.write_fmt( format_args!( "{}", info ) )?;
      if let Some( bump ) = bump
      {
        f.write_fmt( format_args!( "{}\n", bump ) )?;
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

  /// Publishes a single package without publishing its dependencies.
  ///
  /// This function is designed to publish a single package. It does not publish any of the package's dependencies.
  ///
  /// Args:
  ///
  /// - path - a path to package manifest file
  /// - dry - a flag that indicates whether to apply the changes or not
  ///   - true - do not publish, but only show what steps should be taken
  ///   - false - publishes the package
  ///
  /// Returns:
  /// Returns a result containing a report indicating the result of the operation.
  pub fn publish_single( path : &Path, dry : bool ) -> Result< PublishReport, ( PublishReport, Error ) >
  {
    let mut report = PublishReport::default();
    let mut manifest = manifest::open( path ).map_err( |e | ( report.clone(), e ) )?;
    if !manifest.package_is() || manifest.local_is()
    {
      return Ok( report );
    }

    let mut package_dir = manifest.manifest_path.clone();
    package_dir.pop();

    let output = process::start_sync( "cargo package", &package_dir ).context( "Take information about package" ).map_err( | e | ( report.clone(), e ) )?;
    if output.err.contains( "not yet committed")
    {
      return Err(( report, anyhow!( "Some changes wasn't committed. Please, commit or stash that changes and try again." ) ));
    }
    report.get_info = Some( output );

    if publish_need( &manifest )
    {
      let new_version = version::bump( &mut manifest, dry ).context( "Try to bump package version" ).map_err( | e | ( report.clone(), e ) )?;
      let package_name =
      {
        let data = manifest.manifest_data.as_ref().unwrap();
        data[ "package" ][ "name" ].as_str().unwrap()
      };
      let manifest_dir = manifest.manifest_path.parent().unwrap();

      report.bump = Some( format!( "`{package_name}` bumped to `{new_version}`" ) );

      let commit_message = format!( "{package_name}-v{new_version}" );
      let res = git::add( manifest_dir, [ "Cargo.toml" ], dry ).map_err( | e | ( report.clone(), e ) )?;
      report.add = Some( res );
      let res = git::commit( manifest_dir, commit_message, dry ).map_err( | e | ( report.clone(), e ) )?;
      report.commit = Some( res );
      let res = git::push( manifest_dir, dry ).map_err( | e | ( report.clone(), e ) )?;
      report.push = Some( res );

      let res = cargo::publish( manifest_dir, dry ).map_err( | e | ( report.clone(), e ) )?;
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
    pub path : Option< PathBuf >,
  }

  impl From< &Package > for CrateId
  {
    fn from( value : &Package ) -> Self
    {
      Self
      {
        name : value.name.clone(),
        path : Some( value.manifest_path.as_std_path().parent().unwrap().to_path_buf() ),
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
        path : value.path.clone().map( | path | path.into_std_path_buf() ),
      }
    }
  }

  /// Recursive implementation of the `dependencies` function
  pub fn _dependencies
  (
    workspace : &mut Workspace,
    manifest_path : &Path, // qqq : for Bohdan : new type!
    graph: &mut HashMap< CrateId, HashSet< CrateId > >,
    opts: DependenciesOptions
  ) -> wtools::error::Result< CrateId >
  {
    let DependenciesOptions
    {
      recursive,
      sort: _,
      with_dev,
      with_remote,
    } = opts;
    if recursive && with_remote { unimplemented!( "`recursive` + `with_remote` options") }

    let manifest_path = path::canonicalize( manifest_path )?;

    let package = workspace
    .load()
    .package_find_by_manifest( &manifest_path )
    .ok_or( anyhow!( "Package not found in the workspace with path: `{}`", manifest_path.display() ) )?;

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
          _dependencies( workspace, &dep.path.as_ref().unwrap().join( "Cargo.toml" ), graph, opts.clone() )?;
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
  /// - `manifest_path` - path to the package manifest file. The package manifest file contains metadata about the package such as its name, version, and dependencies.
  /// - `opts` - used to specify options or configurations for fetching local dependencies.
  ///
  /// # Returns
  ///
  /// If the operation is successful, returns a vector of `PathBuf` objects, where each `PathBuf` represents the path to a local dependency of the specified package.
  pub fn dependencies( workspace : &mut Workspace, manifest_path : &Path, opts: DependenciesOptions ) -> wtools::error::Result< Vec< CrateId > >
  {
    let mut graph = HashMap::new();
    let root = _dependencies( workspace, manifest_path, &mut graph, opts.clone() )?;

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
        graph::toposort( graph::construct( &graph ) ).into_iter().filter( | x | x != &root ).collect()
      },
    };

    Ok( output )
  }

  // qqq : for Bohdan : dont add _get at the end
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
  pub fn local_path_get< 'a >( name : &'a str, version : &'a str, manifest_path : &'a PathBuf ) -> PathBuf
  {
    let buf = format!( "package/{0}-{1}.crate", name, version );

    let workspace = Workspace::with_manifest_path( manifest_path.parent().unwrap() );

    let mut local_package_path = PathBuf::new();
    local_package_path.push( workspace.target_directory() );
    local_package_path.push( buf );

    local_package_path
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
    pub package_filter: Option< Box< dyn Fn( &Package ) -> bool > >,

    /// An optional dependency filtering function. If provided, this function
    /// is applied to each dependency of each package, and only dependencies
    /// that satisfy the condition are included in the final result. If not
    /// provided, a default filter that accepts all dependencies is used.
    pub dependency_filter: Option< Box< dyn Fn( &Package, &Dependency ) -> bool  > >,
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
  pub fn packages_filter_map( packages : &[ Package ], filter_map_options : FilterMapOptions ) -> HashMap< PackageName, HashSet< PackageName > >
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

  // qqq : for Bohdan : why manifest is argument? introduce newtype Package
  pub fn publish_need( manifest : &Manifest ) -> bool
  {
    // These files are ignored because they can be safely changed without affecting functionality
    //
    // - `.cargo_vcs_info.json` - contains the git sha1 hash that varies between different commits
    // - `Cargo.toml.orig` - can be safely modified because it is used to generate the `Cargo.toml` file automatically, and the `Cargo.toml` file is sufficient to check for changes
    const IGNORE_LIST : [ &str; 2 ] = [ ".cargo_vcs_info.json", "Cargo.toml.orig" ];

    let data = manifest.manifest_data.as_ref().expect( "Manifest data doesn't loaded" );

    let name = &data[ "package" ][ "name" ].clone();
    let name = name.as_str().expect( "Name should be valid UTF-8" );
    let version = &data[ "package" ][ "version" ].clone();
    let version = version.as_str().expect( "Version should be valid UTF-8" );
    let local_package_path = local_path_get( name, version, &manifest.manifest_path );

    let local_package = fs::read( local_package_path ).expect( "Failed to read local package. Please, run `cargo package` before." );
    // Is it ok? If there is any problem with the Internet, we will say that the packages are different.
    // qqq : for Bohdan : bad, properly handle errors
    let remote_package = http::retrieve_bytes( name, version ).unwrap_or_default();

    // qqq : for Bohdan : bad, properly handle errors
    let mut local_decoded_package = decode_reader( local_package ).expect( "Failed to unpack local package" );
    let mut remote_decoded_package = decode_reader( remote_package ).expect( "Failed to unpack remote package" );

    let package_root = std::path::PathBuf::from( format!( "{name}-{version}" ) );
    // all ignored files must be ignored
    for ignore in IGNORE_LIST.iter().map( | &object | package_root.join( object ) )
    {
      local_decoded_package.remove( &ignore );
      remote_decoded_package.remove( &ignore );
    }

    let mut is_same = true;
    // if remote has files that missing locally - it is also difference
    let mut remote_keys = remote_decoded_package.keys().collect::< HashSet< _ > >();
    for ( path, ref content ) in local_decoded_package
    {
      remote_keys.remove( &path );
      if let Some( remote_content ) = remote_decoded_package.get( &path )
      {
        is_same &= content == remote_content;
      }
      else
      {
        is_same = false;
      }
    }

    !( is_same && remote_keys.is_empty() )
  }

  // qqq : move out to tools::archive and introduce newtype
  /// Decode bytes archive to the dictionary of file path as a key and content as a value
  ///
  /// # Arg:
  /// - `bytes` - `.crate` file as bytes
  fn decode_reader( bytes : Vec< u8 > ) -> std::io::Result< HashMap< PathBuf, Vec< u8 > > >
  {
    use std::io::prelude::*;
    use flate2::bufread::GzDecoder;
    use tar::Archive;

    if bytes.is_empty()
    {
      return Ok( Default::default() );
    }

    let gz = GzDecoder::new( &bytes[ .. ] );
    let mut archive = Archive::new( gz );

    let mut output = HashMap::new();

    for file in archive.entries()?
    {
      let mut file = file?;
      let mut contents = vec![];
      file.read_to_end( &mut contents )?;
      output.insert( file.path()?.to_path_buf(), contents );
    }

    Ok( output )
  }

}


//

crate::mod_interface!
{

  protected use PublishReport;
  protected use publish_single;
  protected use local_path_get;

  protected use PackageName;

  protected use FilterMapOptions;
  protected use packages_filter_map;
  protected use publish_need;

  protected use CrateId;
  protected use DependenciesSort;
  protected use DependenciesOptions;
  protected use dependencies;

}
