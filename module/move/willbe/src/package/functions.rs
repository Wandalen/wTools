mod private
{
  use std::
  {
    fs,
    path::{ Path, PathBuf },
    collections::{ HashMap, HashSet },
  };
  use std::fmt::Formatter;
  use std::hash::Hash;
  use std::ops::Index;
  use cargo_metadata::{ Dependency, DependencyKind, Package };
  use petgraph::
  {
    graph::Graph,
    algo::toposort as pg_toposort,
  };
  use crate::tools::
  {
    manifest,
    process,
    digest,
    http,
  };
  use crate::{ cargo, git, version };
  use anyhow::{ Context, Error, anyhow };
  use crate::cache::Cache;

  use crate::path;
  use crate::wtools;


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
      // first command
      if get_info.is_none()
      {
        f.write_fmt( format_args!( "Empty report" ) )?;
      }
      let info = get_info.as_ref().unwrap();
      f.write_fmt( format_args!( "{}", info ) )?;
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

  ///
  /// Publish single package.
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
    let mut manifest = manifest::get( path ).map_err( |e | (report.clone(), e ) )?;
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
      report.bump = Some( format!( "`{package_name}` bumped to `{new_version}`" ) );

      let commit_message = format!( "{package_name}-v{new_version}" );
      let res = git::add( &manifest.manifest_path, [ "Cargo.toml" ], dry ).map_err( | e | ( report.clone(), e ) )?;
      report.add = Some( res );
      let res = git::commit( &manifest.manifest_path, commit_message, dry ).map_err( | e | ( report.clone(), e ) )?;
      report.commit = Some( res );
      let res = git::push( &manifest.manifest_path, dry ).map_err( | e | ( report.clone(), e ) )?;
      report.push = Some( res );

      let res = cargo::publish( &manifest.manifest_path, dry ).map_err( | e | ( report.clone(), e ) )?;
      report.publish = Some( res );
    }

    Ok( report )
  }

  /// Sorting variants for dependencies.
  #[ derive( Debug, Copy, Clone ) ]
  pub enum LocalDependenciesSort
  {
    /// List will be topologically sorted.
    Topological,
    /// List will be unsorted.
    Unordered,
  }

  #[ derive( Debug, Clone ) ]
  /// Args for `local_dependencies` function.
  pub struct LocalDependenciesOptions
  {
    /// With dependencies of dependencies.
    pub recursive : bool,
    /// With sorting.
    pub sort : LocalDependenciesSort,
    /// Include dev dependencies.
    pub with_dev : bool,
    /// Skip specific packets.
    pub exclude : HashSet< PathBuf >,
  }

  impl Default for LocalDependenciesOptions
  {
    fn default() -> Self
    {
      Self
      {
        recursive : true,
        sort : LocalDependenciesSort::Unordered,
        with_dev : false,
        exclude : HashSet::new(),
      }
    }
  }

  //

  /// Returns local dependencies of specified package by its manifest path from a workspace
  pub fn local_dependencies_back_end( metadata : &mut Cache, manifest_path : &Path, opts: LocalDependenciesOptions ) -> wtools::error::Result< Vec< PathBuf > >
  {
    let LocalDependenciesOptions
    {
      recursive,
      sort,
      with_dev,
      mut exclude,
    } = opts;

    let manifest_path = path::canonicalize( manifest_path )?;

    let deps = metadata
    .load()
    .package_find_by_manifest( &manifest_path )
    .ok_or( anyhow!( "Package not found in the workspace" ) )?
    .dependencies
    .iter()
    .filter( | dep | !with_dev || ( with_dev && dep.kind != DependencyKind::Development ) )
    .filter_map( | dep | dep.path.as_ref().map( | path | path.clone().into_std_path_buf() ) )
    .collect::< HashSet< _ > >();

    let mut output = deps.clone();

    if recursive
    {
      for dep in &deps
      {
        if !exclude.contains( dep )
        {
          exclude.insert( dep.clone() );
          let inner_opts = LocalDependenciesOptions
          {
            exclude: exclude.clone(),
            ..opts
          };
          output.extend( local_dependencies_back_end( metadata, &dep.join( "Cargo.toml" ), inner_opts )? );
        }
      }
    }

    let mut output : Vec< _ > = output.into_iter().collect();

    match sort
    {
      LocalDependenciesSort::Unordered => {},
      LocalDependenciesSort::Topological =>
      {
        output = toposort_by_paths( metadata, &output );
      },
    }

    Ok( output )
  }

  /// Returns local dependencies of specified package by its manifest path from a workspace
  pub fn local_dependencies( metadata : &mut Cache, manifest_path : &Path, opts: LocalDependenciesOptions ) -> wtools::error::Result< Vec< PathBuf > >
  {
    local_dependencies_back_end( metadata, manifest_path, opts )
  }

  //

  // pub fn filter( metadata : &Metadata ) -> HashMap< String, &Package >
  // {
  //   let mut packages_map = HashMap::new();
  //
  //   let _packages = metadata.packages.iter().filter( | package |
  //   {
  //     if package.publish.is_none()
  //     {
  //       packages_map.insert( package.name.clone(), *package );
  //
  //       return true;
  //     }
  //
  //     false
  //   }).collect::< Vec< _ > >();
  //
  //   packages_map
  // }

  //

  pub fn local_path_get< 'a >( name : &'a str, version : &'a str, manifest_path : &'a PathBuf ) -> PathBuf
  {
    let buf = format!( "package/{0}-{1}.crate", name, version );

    let package_metadata = Cache::with_manifest_path( manifest_path.parent().unwrap() );

    let mut local_package_path = PathBuf::new();
    local_package_path.push( package_metadata.target_directory() );
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

  pub type PackageName = String;

  /// Given a slice of `Package` instances and a set of filtering options,
  /// this function filters and maps the packages and their dependencies
  /// based on the provided filters. It returns a HashMap where the keys
  /// are package names, and the values are HashSet instances containing
  /// the names of filtered dependencies for each package.
  pub fn packages_filter_map( packages: &[ Package ], filter_map_options: FilterMapOptions ) -> HashMap< PackageName, HashSet< PackageName > >
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

  // string, str - package_name
  pub fn graph_build< PackageIdentifier >( packages : &HashMap< PackageIdentifier, HashSet< PackageIdentifier > > ) -> Graph< &PackageIdentifier, &PackageIdentifier >
  where
    PackageIdentifier : PartialEq + Eq + Hash,
  {
    let nudes: HashSet< _ > = packages
    .iter()
    .flat_map( | ( name, dependency ) |
    {
      dependency
      .iter()
      .chain( Some( name ) )
    }).collect();
    let mut deps = Graph::new();
    for nude in nudes
    {
      deps.add_node( nude );
    }
    for ( name, dependencies ) in packages
    {
      let root_node = deps.node_indices().find( | i | deps[ *i ] == name ).unwrap();
      for dep in dependencies
      {
        let dep_node = deps.node_indices().find( | i | deps[ *i ] == dep ).unwrap();
        deps.add_edge(root_node, dep_node, name );
      }
    }
    deps
  }

  //

  pub fn toposort_by_paths( metadata : &mut Cache, paths : &[ PathBuf ] ) -> Vec< PathBuf >
  {
    let map = metadata
    .load()
    .packages_get()
    .iter()
    .filter( | x | paths.contains( &x.manifest_path.as_std_path().parent().unwrap().to_path_buf() ) )
    .map( | p | ( p.name.clone(), p ) )
    .collect::< HashMap< _, _ > >();

    let edges = map
    .iter()
    .map
    (
      |( _, package )|
      (
        package.manifest_path.as_std_path().parent().unwrap().to_path_buf(),
        package.dependencies
        .iter()
        .filter_map( | dep | dep.path.clone() )
        .filter( | path | paths.contains( &path.as_std_path().to_path_buf() ) )
        .map( | path | path.into_std_path_buf() )
        .collect(),
      )
    )
    .collect();
    let graph = graph_build( &edges );

    toposort( graph )
  }

  //

  pub fn toposort< 'a, PackageIdentifier : Clone + std::fmt::Debug >( graph :  Graph< &'a PackageIdentifier, &'a PackageIdentifier > ) -> Vec< PackageIdentifier >
  {
    match pg_toposort( &graph, None )
    {
      Ok( list ) => list
      .iter()
      .rev()
      .map( | dep_idx | ( *graph.node_weight( *dep_idx ).unwrap() ).clone() )
      .collect::< Vec< _ > >(),
      Err( index ) => panic!( "Cycle: {:?}", graph.index( index.node_id() ) ),
    }
  }

  //

  /// Check if publish needed for a package
  ///
  /// Returns:
  /// - true - need
  /// - false - no need
  ///
  /// Panic: manifest must be loaded
  pub fn publish_need( manifest : &manifest::Manifest ) -> bool
  {
    let data = manifest.manifest_data.as_ref().expect( "Manifest data doesn't loaded" );

    let name = &data[ "package" ][ "name" ].clone();
    let name = name.as_str().expect( "Name should be valid UTF-8" );
    let version = &data[ "package" ][ "version" ].clone();
    let version = version.as_str().expect( "Version should be valid UTF-8" );
    let local_package_path = local_path_get( name, version, &manifest.manifest_path );

    let local_package = fs::read( local_package_path ).expect( "Failed to read local package. Please, run `cargo package` before." );
    // Is it ok? If there is any problem with the Internet, we will say that the packages are different.
    let remote_package = http::retrieve_bytes( name, version ).unwrap_or_default();

    digest::hash( &local_package ) != digest::hash( &remote_package )
  }
}

//

crate::mod_interface!
{
  protected( crate ) use PublishReport;
  protected( crate ) use publish_single;

  protected( crate ) use local_path_get;

  protected( crate ) use graph_build;
  protected( crate ) use toposort;
  protected( crate ) use toposort_by_paths;

  protected use FilterMapOptions;
  protected use packages_filter_map;
  protected use publish_need;

  orphan use LocalDependenciesSort;
  orphan use LocalDependenciesOptions;
  orphan use local_dependencies;
}
