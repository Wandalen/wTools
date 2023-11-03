/// Internal namespace.
mod private
{
  use std::collections::HashMap;
  use crate::tools::*;
  use cargo_metadata::
  {
    DependencyKind,
    Metadata,
    Package,
  };
  use petgraph::
  {
    graph::Graph,
  };

  // duplicates publish.rs
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

  // duplicates publish.rs
  pub fn manifest_get( path : &std::path::Path ) -> manifest::Manifest
  {
    let mut manifest = manifest::Manifest::new();
    manifest.manifest_path_from_str( path ).unwrap();
    manifest.load().unwrap();

    manifest
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
}

//

crate::mod_interface!
{
  protected(crate) use packages_filter;
  protected(crate) use manifest_get;
  protected(crate) use graph_build;
}
