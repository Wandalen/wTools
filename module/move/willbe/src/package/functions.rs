mod private
{
  use std::
  {
    fs,
    path::PathBuf,
    collections::{ HashMap, HashSet },
  };
  use std::path::Path;
  use cargo_metadata::
  {
    DependencyKind,
    Metadata,
    MetadataCommand,
    Package,
  };
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

  use crate::path;
  use crate::wtools;


  #[ derive( Debug, Default, Clone ) ]
  pub struct PublishReport
  {
    get_info : Option< process::CmdReport >,
    bump : Option< version::BumpReport >,
    add : Option< process::CmdReport >,
    commit : Option< process::CmdReport >,
    push : Option< process::CmdReport >,
    publish : Option< process::CmdReport >,
  }

  ///
  /// Publish single packages.
  ///

  pub fn publish( _current_path : &PathBuf, path : &PathBuf, dry : bool ) -> Result< PublishReport, ( PublishReport, Error ) >
  {
    let mut report = PublishReport::default();

    let manifest = manifest::get( path ).map_err( | e | ( report.clone(), e ) )?;
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
      let bump_report = version::bump( &manifest.manifest_path, dry ).context( "Try to bump package version" ).map_err( | e | ( report.clone(), e ) )?;
      let package_name = bump_report.package_name.clone().unwrap();
      let new_version = bump_report.new_version.clone().unwrap();
      report.bump = Some( bump_report );

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
    /// Skip packages.
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
        exclude : HashSet::new(),
      }
    }
  }

  //

  /// Returns local dependencies of specified package by its manifest path from a workspace
  pub fn local_dependencies( metadata : &Metadata, manifest_path : &Path, opts: LocalDependenciesOptions ) -> wtools::error::Result< Vec< PathBuf > >
  {
    let LocalDependenciesOptions
    {
      recursive,
      sort,
      mut exclude,
    } = opts;

    let manifest_path = path::canonicalize( manifest_path )?;

    let deps = metadata
    .packages
    .iter()
    .find( | package | package.manifest_path.as_std_path() == &manifest_path )
    .ok_or( anyhow!( "Package not found in the workspace" ) )?
    .dependencies
    .iter()
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
          let rebuild_opts = LocalDependenciesOptions
          {
            recursive,
            sort,
            exclude: exclude.clone(),
          };
          output.extend( local_dependencies( metadata, &dep.join( "Cargo.toml" ), rebuild_opts )? );
        }
      }
    }

    let mut output : Vec< _ > = output.into_iter().collect();

    match sort
    {
      LocalDependenciesSort::Unordered => {},
      LocalDependenciesSort::Topological =>
      {
        output = toposort_by_paths( &metadata, &output );
      },
    }

    Ok( output )
  }

  //

  pub fn filter( metadata : &Metadata ) -> HashMap< String, &Package >
  {
    let mut packages_map = HashMap::new();

    let _packages = metadata.packages.iter().filter( | package |
    {
      if package.publish.is_none()
      {
        packages_map.insert( package.name.clone(), *package );

        return true;
      }

      false
    }).collect::< Vec< _ > >();

    packages_map
  }

  //

  pub fn local_path_get< 'a >( name : &'a str, version : &'a str, manifest_path : &'a PathBuf ) -> PathBuf
  {
    let buf = format!( "package/{0}-{1}.crate", name, version );

    let package_metadata = MetadataCommand::new()
    .manifest_path( manifest_path )
    .exec()
    .unwrap();

    let mut local_package_path = PathBuf::new();
    local_package_path.push( package_metadata.target_directory );
    local_package_path.push( buf );

    local_package_path
  }

  //

  pub fn graph_build< 'a >( packages : &'a HashMap< String, &Package > ) -> Graph< &'a str, &'a str >
  {
    let mut deps = Graph::< &str, &str >::new();
    let _update_graph = packages.iter().map( | ( _name, package ) |
    {
      let root_node = if let Some( node ) = deps.node_indices().find( | i | deps[ *i ] == package.name )
      {
        node
      }
      else
      {
        deps.add_node( &package.name )
      };

      for dep in &package.dependencies
      {
        if dep.path.is_some() && dep.kind != DependencyKind::Development
        {
          let dep_node = if let Some( node ) = deps.node_indices().find( | i | deps[ *i ] == dep.name )
          {
            node
          }
          else
          {
            deps.add_node( &dep.name )
          };

          deps.add_edge( root_node, dep_node, &package.name );
        }
      }
    }).collect::< Vec< _ > >();

    deps
  }

  //

  pub fn toposort_by_paths( metadata : &Metadata, paths : &[ PathBuf ] ) -> Vec< PathBuf >
  {
    let map = metadata.packages
    .iter()
    .filter( | x | paths.contains( &x.manifest_path.as_std_path().parent().unwrap().to_path_buf() ) )
    .map( | p | ( p.name.clone(), p ) )
    .collect::< HashMap< _, _ > >();

    toposort( &map ).into_iter().map( | name | map[ &name ].manifest_path.parent().unwrap().to_path_buf().into_std_path_buf() ).collect()
  }

  //

  pub fn toposort( packages : &HashMap< String, &Package > ) -> Vec< String >
  {
    let deps = graph_build( packages );

    let sorted = pg_toposort( &deps, None ).expect( "Failed to process toposort for packages" );
    let names = sorted
    .iter()
    .rev()
    .map( | dep_idx | deps.node_weight( *dep_idx ).unwrap().to_string() )
    .collect::< Vec< String > >();

    names
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
  protected( crate ) use publish;

  protected( crate ) use filter;
  protected( crate ) use local_path_get;

  protected( crate ) use graph_build;
  protected( crate ) use toposort;

  protected use publish_need;

  orphan use LocalDependenciesSort;
  orphan use LocalDependenciesOptions;
  orphan use local_dependencies;
}
