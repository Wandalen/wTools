/// Internal namespace.
pub( crate ) mod private
{
  use std::
  {
    ops::Index,
    fmt::Debug,
    hash::Hash,
    collections::{ HashMap, HashSet }
  };
  use petgraph::
  {
    graph::Graph,
    algo::toposort as pg_toposort,
  };
  use petgraph::graph::NodeIndex;
  use petgraph::prelude::*;

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
    graph : Graph< &'a PackageIdentifier, &'a PackageIdentifier >
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
      // aaa : now returns `GraphError`
    }
  }

  /// Creates a subgraph from the given graph, containing only the nodes and edges reachable from the roots.
  ///
  /// # Arguments
  /// * `graph` - The original graph from which to create the subgraph.
  /// * `roots` - An array of nodes that will serve as the roots of the subgraph.
  ///
  /// # Returns
  /// A new graph that represents the subgraph.
  ///
  /// # Generic Types
  /// * `N` - The type of the node in the original graph.
  /// * `E` - The type of the edge in the original graph.
  ///
  /// # Constraints
  /// * `N` must implement the `PartialEq` trait.
  pub fn subgraph< N, E >( graph : &Graph< N, E >, roots : &[ N ] ) -> Graph< NodeIndex, EdgeIndex >
  where
    N : PartialEq< N >,
  {
    let mut subgraph = Graph::new();
    let mut node_map = HashMap::new();

    for root in roots
    {
      let root_id = graph.node_indices().find( | x | graph[ *x ] == *root ).unwrap();
      let mut dfs = Dfs::new( graph, root_id );
      while let Some( nx ) = dfs.next( &graph )
      {
        if !node_map.contains_key( &nx )
        {
          let sub_node = subgraph.add_node( nx );
          node_map.insert( nx, sub_node );
        }
      }
    }

    for ( _, sub_node_id ) in &node_map
    {
      let node_id_graph = subgraph[ *sub_node_id ];

      for edge in graph.edges( node_id_graph )
      {
        match ( node_map.get( &edge.source() ), node_map.get( &edge.target() ) )
        {
          ( Some( &from ), Some( &to ) ) =>
          {
            subgraph.add_edge( from, to, edge.id() );
          }
          _ => {}
        }
      }
    }

    subgraph
  }
}

//

crate::mod_interface!
{
  protected use construct;
  protected use toposort;
  protected use subgraph;
}
