mod private
{
  use crate::tools::
  {
    digest,
    http,
    process,
  };
  use crate::manifest;
  use crate::version::bump;
  use wtools::error::Result;
  use std::
  {
    fs,
    path::PathBuf,
    collections::HashMap,
    fmt::Write,
  };
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
    let mut buf = String::new();
    write!( &mut buf, "package/{0}-{1}.crate", name, version ).unwrap();

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

  pub fn publish( current_path : &PathBuf, path : &PathBuf, dry : bool ) -> Result< () >
  {
    let mut manifest = manifest::get( path ).unwrap();
    if !manifest.package_is() || manifest.local_is()
    {
      return Ok( () );
    }
    let data = manifest.manifest_data.as_deref_mut().unwrap();

    let mut package_dir = manifest.manifest_path.clone();
    package_dir.pop();

    let output = process::start_sync( "cargo package", &package_dir ).unwrap();
    process::log_output( &output );

    let name = &data[ "package" ][ "name" ].clone();
    let name = name.as_str().unwrap();
    let version = &data[ "package" ][ "version" ].clone();
    let version = version.as_str().unwrap();
    let local_package_path = local_path_get( name, version, &manifest.manifest_path );

    let local_package = fs::read( local_package_path ).unwrap();
    let remote_package = http::retrieve_bytes( name, version ).unwrap_or_default();

    let digest_of_local = digest::hash( &local_package );
    let digest_of_remote = digest::hash( &remote_package );

    if digest_of_local != digest_of_remote
    {
      data[ "package" ][ "version" ] = bump( version ).unwrap();
      let version = &data[ "package" ][ "version" ].clone();
      let version = version.as_str().unwrap();
      manifest.store().unwrap();

      if dry
      {
        let mut buf = String::new();
        write!( &mut buf, "git commit --dry-run -am \"{} v{}\"", name, version ).unwrap();
        let output = process::start_sync( &buf, current_path ).unwrap();
        process::log_output( &output );

        let output = process::start_sync( "git push --dry-run", current_path ).unwrap();
        process::log_output( &output );

        let output = process::start_sync( "cargo publish --dry-run --allow-dirty", &package_dir ).unwrap();
        process::log_output( &output );

        let output = process::start_sync( &format!( "git checkout {:?}", &package_dir ), current_path ).unwrap();
        process::log_output( &output );
      }
      else
      {
        let mut buf = String::new();
        write!( &mut buf, "git commit -am \"{} v{}\"", name, version ).unwrap();
        let output = process::start_sync( &buf, current_path ).unwrap();
        process::log_output( &output );

        let output = process::start_sync( "git push", current_path ).unwrap();
        process::log_output( &output );

        let output = process::start_sync( "cargo publish", &package_dir ).unwrap();
        process::log_output( &output );
      }
    }
    else
    {
      println!( "Package {} is up to date", name );
    }

    Ok( () )
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

    let sorted = pg_toposort( &deps, None ).unwrap();
    let names = sorted
    .iter()
    .rev()
    .map( | dep_idx | deps.node_weight( *dep_idx ).unwrap().to_string() )
    .collect::< Vec< String > >();

    names
  }
}

//

crate::mod_interface!
{
  protected(crate) use filter;
  protected(crate) use local_path_get;
  protected(crate) use publish;

  protected(crate) use graph_build;
  protected(crate) use toposort;
}
