/// Internal namespace.
mod private
{
  use crate::tools::
  {
    digest,
    http,
    manifest,
    process,
  };
  use wtools::error::Result;
  use std::
  {
    fs,
    path::PathBuf,
    collections::HashMap,
    fmt::Write,
  };
  use toml_edit::value;
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
    algo::toposort,
  };

  // duplicates list.rs
  pub fn packages_filter( metadata : &Metadata ) -> HashMap< String, &Package >
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

  // duplicates list.rs
  fn manifest_get( path : impl Into< PathBuf > ) -> anyhow::Result< manifest::Manifest >
  {
    let mut manifest = manifest::Manifest::new();
    manifest.manifest_path_from_str( path )?;
    manifest.load()?;

    Ok( manifest )
  }

  //

  fn local_package_path_get< 'a >( name : &'a str, version : &'a str, manifest_path : &'a PathBuf ) -> PathBuf
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

  fn bump( version : &str ) -> anyhow::Result< toml_edit::Item >
  {
    let mut splits : Vec< &str > = version.split( '.' ).collect();
    let patch_version = splits[ 2 ].parse::< u32 >()? + 1;
    let v = &patch_version.to_string();
    splits[ 2 ] = v;

    Ok( value( splits.join( "." ) ) )
  }

  //

  pub fn package_publish( current_path : &PathBuf, path : &PathBuf, dry : bool ) -> Result< () >
  {
    let mut manifest = manifest_get( path ).unwrap();
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
    let local_package_path = local_package_path_get( name, version, &manifest.manifest_path );

    let local_package = fs::read( &local_package_path ).unwrap();
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
        let output = process::start_sync( &buf, &current_path ).unwrap();
        process::log_output( &output );

        let output = process::start_sync( "git push --dry-run", &current_path ).unwrap();
        process::log_output( &output );

        let output = process::start_sync( "cargo publish --dry-run --allow-dirty", &package_dir ).unwrap();
        process::log_output( &output );

        let output = process::start_sync( &format!( "git checkout {:?}", &package_dir ), &current_path ).unwrap();
        process::log_output( &output );
      }
      else
      {
        let mut buf = String::new();
        write!( &mut buf, "git commit -am \"{} v{}\"", name, version ).unwrap();
        let output = process::start_sync( &buf, &current_path ).unwrap();
        process::log_output( &output );

        let output = process::start_sync( "git push", &current_path ).unwrap();
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

  pub fn toposort_local_packages( packages : &HashMap< String, &Package > ) -> Vec< String >
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

    let sorted = toposort( &deps, None ).unwrap();
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
  protected(crate) use packages_filter;
  protected(crate) use package_publish;
  protected(crate) use toposort_local_packages;
}
