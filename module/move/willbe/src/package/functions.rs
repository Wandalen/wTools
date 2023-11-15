mod private
{
  use std::
  {
    fs,
    path::PathBuf,
    collections::{ HashMap, HashSet },
    fmt::Write,
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
  use crate::version;
  use anyhow::{ Context, Error, anyhow };
  use toml_edit::value;

  use crate::path;
  use crate::wtools;


  #[ derive( Debug, Default, Clone ) ]
  pub struct PublishReport
  {
    get_info : Option< process::CmdReport >,
    bump : Option< String >,
    commit : Option< process::CmdReport >,
    push : Option< process::CmdReport >,
    publish : Option< process::CmdReport >,
  }

  ///
  /// Publish single packages.
  ///

  pub fn publish( current_path : &PathBuf, path : &PathBuf, dry : bool ) -> Result< PublishReport, ( PublishReport, Error ) >
  {
    let mut report = PublishReport::default();

    let mut manifest = manifest::get( path ).map_err( | e | ( report.clone(), e ) )?;
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

    if !publish_need( &manifest )
    {
      let data = manifest.manifest_data.as_deref_mut().ok_or( anyhow!( "Failed to get manifest data" ) ).map_err( | e | ( report.clone(), e ) )?;
      let name = &data[ "package" ][ "name" ].clone();
      let name = name.as_str().expect( "Name should be valid UTF-8" );
      let version = &data[ "package" ][ "version" ].clone();
      let version = version.as_str().expect( "Version should be valid UTF-8" );
      let new_version = version::bump( version ).map_err( | e | ( report.clone(), e ) )?;

      if dry
      {
        report.bump = Some( "Bump package version".into() );

        let buf = format!( "git commit -am {}-v{}", name, new_version );
        let output = process::CmdReport
        {
          command : buf,
          path : current_path.clone(),
          out : String::new(),
          err : String::new(),
        };
        report.commit = Some( output );

        let buf = "git push".to_string();
        let output = process::CmdReport
        {
          command : buf,
          path : current_path.clone(),
          out : String::new(),
          err : String::new(),
        };
        report.push = Some( output );

        let buf = "cargo publish".to_string();
        let output = process::CmdReport
        {
          command : buf,
          path : package_dir.clone(),
          out : String::new(),
          err : String::new(),
        };
        report.publish = Some( output );
      }
      else
      {
        data[ "package" ][ "version" ] = value( &new_version );
        manifest.store().map_err( | e | ( report.clone(), e ) )?;
        report.bump = Some( "Bump package version".into() );

        let buf = format!( "git commit -am {}-v{}", name, new_version );
        let output = process::start_sync( &buf, current_path ).context( "Commit changes while publishing" ).map_err( | e | ( report.clone(), e ) )?;
        report.commit = Some( output );

        let buf = "git push".to_string();
        let output = process::start_sync( &buf, current_path ).context( "Push while publishing" ).map_err( | e | ( report.clone(), e ) )?;
        report.push = Some( output );

        let buf = "cargo publish".to_string();
        let output = process::start_sync( &buf, &package_dir ).context( "Publish" ).map_err( | e | ( report.clone(), e ) )?;
        report.publish = Some( output );
      }
    }

    Ok( report )
  }

  //

  #[ derive( Debug, Clone ) ]
  /// Args for `local_dependencies` function
  pub struct LocalDependenciesOptions
  {
    /// With dependencies of dependencies
    pub recursive : bool,
    /// Skip packages
    pub exclude : HashSet< PathBuf >,
  }

  impl Default for LocalDependenciesOptions
  {
    fn default() -> Self
    {
      Self
      {
        recursive : true,
        exclude : HashSet::new(),
      }
    }
  }

  //

  /// Returns local dependencies of specified package by its manifest path from a workspace
  pub fn local_dependencies( metadata : &Metadata, manifest_path : &Path, mut opts: LocalDependenciesOptions ) -> wtools::error::Result< Vec< PathBuf > >
  {
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

    if opts.recursive
    {
      for dep in &deps
      {
        if !opts.exclude.contains( dep )
        {
          opts.exclude.insert( dep.clone() );
          output.extend( local_dependencies( metadata, &dep.join( "Cargo.toml" ), opts.clone() )? );
        }
      }
    }

    Ok( output.into_iter().collect() )
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

  // Panic: manifest must be loaded
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

    digest::hash( &local_package ) == digest::hash( &remote_package )
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

  orphan use LocalDependenciesOptions;
  orphan use local_dependencies;
}
