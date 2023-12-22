/// Internal namespace.
pub( crate ) mod private
{
  use std::collections::{ HashMap, HashSet };
  use std::fmt::Debug;
  use std::hash::Hash;
  use std::ops::Index;
  use petgraph::
  {
    graph::Graph,
    algo::toposort as pg_toposort,
  };

  use error_tools::for_lib::Error;

  #[ derive( Debug, Error ) ]
  pub enum GraphError< T : Debug >
  {
    #[ error( "Cycle: {0:?}" ) ]
    Cycle( T ),
  }

  /// Build a graph from map of packages and its dependencies
  ///
  /// Arg:
  /// - packages - a map, where key is a package identifier and value - the package dependencies identifiers
  ///
  /// Returns:
  /// The graph with all accepted packages
  pub fn construct< PackageIdentifier >
  (
    packages : &HashMap< PackageIdentifier,
    HashSet< PackageIdentifier > >
  )
  -> Graph< &PackageIdentifier, &PackageIdentifier >
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

  // qqq : add test
  // qqq : cyclic test?
  /// Performs a topological sort of a graph of packages
  ///
  /// Arg:
  /// - `graph` - a directed graph of packages and their dependencies.
  ///
  /// Returns
  /// A list that contains the sorted packages identifiers in topological order.
  ///
  /// # Panics
  /// If there is a cycle in the dependency graph
  pub fn toposort< 'a, PackageIdentifier : Clone + std::fmt::Debug >
  (
    graph : Graph< &'a PackageIdentifier,
    &'a PackageIdentifier >
  )
  -> Result< Vec< PackageIdentifier >, GraphError< PackageIdentifier > >
  {
    match pg_toposort( &graph, None )
    {
      Ok( list ) => Ok
      (
        list
        .iter()
        .rev()
        .map( | dep_idx | ( *graph.node_weight( *dep_idx ).unwrap() ).clone() )
        .collect::< Vec< _ > >()
      ),
      Err( index ) => Err( GraphError::Cycle( ( *graph.index( index.node_id() ) ).clone() ) ),
      // qqq : for Bohdan : bad, make proper error handling
    }
  }
}

//

crate::mod_interface!
{
  protected use construct;
  protected use toposort;
}
