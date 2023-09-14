/// Internal namespace.
pub( crate ) mod private
{
  use crate::protected::*;
  use std::env;
  use wca::{ Args, Props };
  use wtools::error::BasicError;
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
    algo::has_path_connecting,
  };

  ///
  /// List packages.
  ///

  pub fn list( ( args, _ ) : ( Args, Props ) ) -> Result< (), BasicError >
  {
    let current_path = env::current_dir().unwrap();

    let patterns = args.get_owned::< Vec< String > >( 0 ).unwrap_or_default();
    let paths = files::find( current_path, patterns.as_slice() );
    let paths = paths.iter().filter_map( | s | if s.ends_with( "Cargo.toml" ) { Some( s ) } else { None } );

    for path in paths
    {
      let manifest = manifest_get( path );
      if manifest.package_is()
      {
        let local_is = manifest.local_is();
        let remote = if local_is { "local" } else { "remote" };
        let data = manifest.manifest_data.as_ref().unwrap();
        println!( "{} - {:?}, {}", data[ "package" ][ "name" ].to_string().trim(), path.parent().unwrap(), remote );
      }
    }

    Ok( () )
  }

  ///
  /// List workspace packages.
  ///

  pub fn workspace_list( ( args, properties ) : ( Args, Props ) ) -> Result< (), BasicError >
  {
    let list_type = properties.get_owned( "type" ).unwrap_or( "tree" );
    if list_type != "tree" && list_type != "topsort" {
      return Err( BasicError::new( format!( "Unknown option 'type:{}'", list_type ) ) );
    }

    let mut manifest = manifest::Manifest::new();
    let path_to_workspace = args.get_owned::< String >( 0 ).unwrap_or_default();
    let manifest_path = manifest.manifest_path_from_str( &path_to_workspace ).unwrap();
    let package_metadata = MetadataCommand::new()
    .manifest_path( &manifest_path )
    .no_deps()
    .exec()
    .unwrap();

    let packages_map = packages_filter( &package_metadata );
    let graph = graph_build( &packages_map );
    let sorted = toposort( &graph, None ).unwrap();

    if list_type == "tree" {

      let root_crate: &str = properties.get_owned( "root_module" ).unwrap_or_default();

      if root_crate.is_empty() {
        let mut names = vec![ sorted[ 0 ] ];
        for node in sorted.iter().skip( 1 ) {
          if names.iter().all( | name | !has_path_connecting( &graph, *name, *node, None ) ) && !names.contains( node ) {
              names.push( *node );
          }
        }
        names.iter().for_each( | n | ptree::graph::print_graph( &graph, *n ).unwrap() );
      }
      else
      {
        sorted
          .iter()
          .filter_map( | idx | if graph.node_weight( *idx ).unwrap() == &root_crate { Some( *idx ) } else { None } )
          .for_each( | e | ptree::graph::print_graph(&graph, e ).unwrap() );
      }
    }
    else
    {
      let names = sorted
      .iter()
      .rev()
      .map( | dep_idx | graph.node_weight( *dep_idx ).unwrap().to_string() )
      .collect::< Vec< String > >();
      names.iter().enumerate().for_each( | ( i, e ) | println!( "{i}) {e}" ) );
    }


    Ok( () )
  }

  fn packages_filter( metadata : &Metadata ) -> HashMap< String, &Package >
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

  fn graph_build< 'a >( packages : &'a HashMap< String, &Package > ) -> Graph< &'a str, &'a str >
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

  fn manifest_get( path : &std::path::Path ) -> manifest::Manifest
  {
    let mut manifest = manifest::Manifest::new();
    manifest.manifest_path_from_str( path ).unwrap();
    manifest.load().unwrap();
    manifest
  }
}

//

crate::mod_interface!
{
  prelude use list;
  prelude use workspace_list;
}
