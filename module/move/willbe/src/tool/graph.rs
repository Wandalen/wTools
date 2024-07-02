/// Internal namespace.
pub( crate ) mod private
{
  #[ allow( unused_imports ) ]
  use crate::*;

  // use crate::tool::*;
  // qqq : bad : for Bohdan : asterist only crate::* and prelude::*

  use std::
  {
    ops::Index,
    fmt::Debug,
    hash::Hash,
  };
  use collection::{ HashMap, HashSet, VecDeque };
  use path::PathBuf;
  use petgraph::
  {
    graph::Graph,
    algo::toposort as pg_toposort,
  };
  use petgraph::graph::NodeIndex;
  use petgraph::prelude::*;

  use error::
  {
    typed::Error,
  };

  use package::{ Package, publish_need };
  // qqq : for Bohdan : bad : tools can't depend on entitties!

  #[ derive( Debug, Error ) ]
  pub enum GraphError< T : Debug >
  {
    #[ error( "Cycle: {0:?}" ) ]
    Cycle( T ),
  }

  /// Build a graph from map of packages and its dependencies
  ///
  /// Arg :
  /// - packages - a map, where key is a package identifier and value - the package dependencies identifiers
  ///
  /// Returns :
  /// The graph with all accepted packages
  pub fn construct< PackageIdentifier >
  (
    packages : &HashMap
    <
      PackageIdentifier,
      HashSet< PackageIdentifier >,
    >
  )
  -> Graph< &PackageIdentifier, &PackageIdentifier >
  where
    PackageIdentifier : PartialEq + Eq + Hash,
  {
    let nudes : HashSet< _ > = packages
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
  /// Arg :
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
  -> error::Result< Vec< PackageIdentifier >, GraphError< PackageIdentifier > >
  {
    match pg_toposort( &graph, None )
    {
      Ok( list ) => Ok
      (
        list
        .iter()
        .rev()
        .map( | dep_idx | ( *graph.node_weight( *dep_idx ).unwrap() ).clone() )
        .collect()
      ),
      Err( index ) => Err( GraphError::Cycle( ( *graph.index( index.node_id() ) ).clone() ) ),
      // aaa : for Bohdan : bad, make proper error handling
      // aaa : now returns `GraphError`
    }
  }

  /// The function performs a topological sort of a graph with grouping.
  ///
  /// # Arguments
  ///
  /// * `graph` - A graph represented as an adjacency list. Each node in the graph represents a task, and edges represent dependencies.
  ///
  /// # Returns
  ///
  /// The function returns a vector of vectors, where each inner vector represents a group of nodes that can be executed in parallel. Tasks within each group are sorted in topological order.
  pub fn topological_sort_with_grouping< 'a, PackageIdentifier : Clone + std::fmt::Debug >
  (
    graph : Graph< &'a PackageIdentifier, &'a PackageIdentifier >
  )
  -> Vec< Vec< PackageIdentifier > >
  {
    let mut in_degree = HashMap::new();
    for node in graph.node_indices()
    {
      in_degree.insert( node, graph.neighbors_directed( node, Incoming ).count() );
    }

    let mut roots = VecDeque::new();
    for ( node, &degree ) in in_degree.iter()
    {
      if degree == 0
      {
        roots.push_back( *node );
      }
    }

    let mut result = Vec::new();
    while !roots.is_empty()
    {
      let mut next_roots = Vec::new();
      let mut group = Vec::new();
      while let Some( node ) = roots.pop_front()
      {
        group.push( node );
        for edge in graph.neighbors( node )
        {
          let degree = in_degree.get_mut( &edge ).unwrap();
          *degree -= 1;
          if *degree == 0
          {
            next_roots.push( edge );
          }
        }
      }
      roots = VecDeque::from( next_roots );
      result.push( group );
    }
    result
    .into_iter()
    .map
    (
      | vec |
      vec
      .iter()
      .map( | dep_idx | ( *graph.node_weight( *dep_idx ).unwrap() ).clone() )
      .collect()
    )
    .rev()
    .collect()
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

  /// Removes nodes that are not required to be published from the graph.
  ///
  /// # Arguments
  ///
  /// * `package_map` - A reference to a `HashMap` mapping `String` keys to `Package` values.
  /// * `graph` - A reference to a `Graph` of nodes and edges, where nodes are of type `String` and edges are of type `String`.
  /// * `roots` - A slice of `String` representing the root nodes of the graph.
  ///
  /// # Returns
  ///
  /// A new `Graph` with the nodes that are not required to be published removed.

  // qqq : for Bohdan : typed error
  pub fn remove_not_required_to_publish< 'a >
  (
    package_map : &HashMap< String, Package< 'a > >,
    graph : &Graph< String, String >,
    roots : &[ String ],
    temp_path : Option< PathBuf >,
  )
  -> error::Result< Graph< String, String > >
  // qqq : don't use 1-prameter Result
  {
    let mut nodes = HashSet::new();
    let mut cleared_graph = Graph::new();

    for root in roots
    {
      let root = graph.node_indices().find( | &i | graph[ i ] == *root ).unwrap();
      let mut dfs = DfsPostOrder::new( &graph, root );
      'main : while let Some( n ) = dfs.next(&graph)
      {
        for neighbor in graph.neighbors_directed( n, Outgoing )
        {
          if nodes.contains( &neighbor )
          {
            nodes.insert( n );
            continue 'main;
          }
        }
        let package = package_map.get( &graph[ n ] ).unwrap();
        _ = cargo::pack
        (
          cargo::PackOptions::former()
          .path( package.crate_dir().absolute_path() )
          .option_temp_path( temp_path.clone() )
          .dry( false )
          .allow_dirty( true )
          .form()
        )?;
        if publish_need( package, temp_path.clone() ).unwrap()
        {
          nodes.insert( n );
        }
      }
    }
    let mut new_map = HashMap::new();
    for node in nodes.iter().copied() { new_map.insert( node, cleared_graph.add_node( graph[ node ].clone() ) ); }

    for sub_node_id in nodes
    {
      for edge in graph.edges( sub_node_id )
      {
        match ( new_map.get( &edge.source() ), new_map.get( &edge.target() ) )
        {
          ( Some( &from ), Some( &to ) ) =>
          {
            cleared_graph.add_edge( from, to, graph[ edge.id() ].clone() );
          }
          _ => {}
        }
      }
    }

    Ok( cleared_graph )
  }
}

//

crate::mod_interface!
{
  protected use construct;
  protected use toposort;
  protected use topological_sort_with_grouping;
  protected use subgraph;
  protected use remove_not_required_to_publish;
}
