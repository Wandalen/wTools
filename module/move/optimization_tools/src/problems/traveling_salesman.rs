//! Solving Traveling Salesman Problem using HybridOptiomizer.
//!
//! Initial population generated as random routes where each node appears exactly once( except for starting node, which apperars at the beginning and at the end ).
//!
//! Selection operator performes tourmanent selection: randomly selecting a group of individuals from the population( the number of individuals selected is equal to the tournament_size value).
//! Likelihood of win of the fittest participant is determined by tournament_selection_pressure.
//!
//! Crossover operator performs ordered crossover to preserve uniqueness of each node in route: a subroute from the first parent is selected and the remainder of the route is filled
//! with the nodes from the second parent in the order in which they appear, without duplicating any nodes in the selected subroute from the first parent.
//!
//! Mutation operator alters solution in one of three different ways, determined randomly:
//! - by swapping two nodes within the route( start and end nodes excluded ),
//! - by reversing subroute,
//! - by changing position of subroute.
//!

use std::collections::HashMap;
use crate::hybrid_optimizer::*;

use derive_tools::{ From, InnerFrom };
use deterministic_rand::{ Hrng, seq::{ SliceRandom, IteratorRandom } };
use iter_tools::Itertools;

/// Functionality for symmetrical traveling salesman problem undirected graph representation.
pub trait Graph
{
  /// Graph node type.
  type N;
  /// Graph edge type.
  type E;

  /// Checks if edge connecting two nodes exists.
  fn has_edge( &self, node1 : &Self::N, node2 : &Self::N ) -> bool;

  /// Adds edge to graph, connecting two nodes.
  fn add_edge( &mut self, node1 : Self::N, node2 : Self::N, weight : f64 );

  /// Return list of graph nodes.
  fn nodes( &self ) -> Vec< Self::N >;

  /// Get edge that connects two given nodes. Returns None if edge doesn't exist.
  fn get_edge( &self, node1 : &Self::N, node2 : &Self::N  ) -> Option< Self::E >;
}

/// Graph for traveling salesman problem.
#[ derive( Debug, Clone ) ]
pub struct TSPGraph
{
  /// Maps nodes of the graph with list of connected nodes and weight of edge that connects them.
  adjacency_list : HashMap< NodeIndex, Vec < ( NodeIndex, EdgeWeight ) > >,
}

impl TSPGraph
{
  /// Create new instance of graph.
  pub fn new() -> Self
  {
    Self { adjacency_list : HashMap::new() }
  }
}

impl Default for TSPGraph
{
  fn default() -> Self
  {
    let mut graph = TSPGraph::new();
    graph.add_edge( NodeIndex( 1 ), NodeIndex( 2 ), 10.0 );
    graph.add_edge( NodeIndex( 1 ), NodeIndex( 3 ), 15.0 );
    graph.add_edge( NodeIndex( 1 ), NodeIndex( 4 ), 20.0 );
    graph.add_edge( NodeIndex( 2 ), NodeIndex( 3 ), 35.0 );
    graph.add_edge( NodeIndex( 2 ), NodeIndex( 4 ), 25.0 );
    graph.add_edge( NodeIndex( 3 ), NodeIndex( 4 ), 30.0 );
    graph
  }
}

/// Node for traveling salesman route graph.
#[ derive( Debug, PartialEq, Eq, Hash ) ]
pub struct Node< T >
{
  /// Value of node.
  pub value : T,
  /// Index of node.
  pub index : NodeIndex,
}

/// Wrapper for index of graph node.
#[ derive( Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord ) ]
pub struct NodeIndex( pub usize );

/// Weight of graph edge.
#[ derive( Debug, From, InnerFrom, Clone, Copy ) ]
pub struct EdgeWeight( pub f64 );

/// Edge for undirected weighted graph.
#[ derive( Debug, Clone ) ]
pub struct Edge( NodeIndex, NodeIndex, EdgeWeight );

impl Edge
{
  /// Get weight of the edge.
  pub fn weight( &self ) -> EdgeWeight
  {
    self.2
  }
}

impl Graph for TSPGraph
{
  type N = NodeIndex;
  type E = Edge;
  fn has_edge( &self, node1 : &Self::N, node2 : &Self::N ) -> bool
  {
    if let Some( node_vec ) = self.adjacency_list.get( &node1 )
    {
      if node_vec.iter().find( | ( n, _ ) | n == node2 ).is_some()
      {
        return true;
      }
    }
    false
  }

  fn get_edge( &self, node1 : &Self::N, node2 : &Self::N ) -> Option< Edge >
  {
    if let Some( node_vec ) = self.adjacency_list.get( &node1 )
    {
      if let Some( ( _, weight ) ) = node_vec.iter().find( | ( n, _ ) | n == node2 )
      {
        return Some( Edge( *node1, *node2, *weight ) );
      }
    }
    None
  }

  fn add_edge( &mut self, node1 : Self::N, node2 : Self::N, weight : f64 )
  {
    self.adjacency_list.entry( node1 ).or_default().push( ( node2, weight.into() ) );
    self.adjacency_list.entry( node2 ).or_default().push( ( node1, weight.into() ) );
  }

  fn nodes( &self ) -> Vec< NodeIndex >
  {
    self.adjacency_list.keys().map( | k | *k ).collect_vec()
  }
}

/// Initial configuration of symmetrical traveling salesman problem.
#[ derive( Debug, Clone ) ]
pub struct TSProblem
{
  /// Node to start route from.
  pub starting_node : NodeIndex,

  /// Weighted graph with nodes and weighted edges that connect them.
  pub graph : TSPGraph,
}

impl TSProblem
{
  /// Create new instance of Traveling Salesman Problem.
  pub fn new( graph : TSPGraph, starting_node : NodeIndex ) -> Self
  {
    Self { graph, starting_node }
  }
}

/// Possible solution of traveling salesman problem, contains route and its distance.
#[ derive( Debug, PartialEq, Clone ) ]
pub struct TSPerson
{
  /// Route which contains starting node at first and last position and every other node exactly once.
  pub route : Vec< NodeIndex >,

  /// Total distance of the route.
  pub distance : f64,
}

impl TSPerson
{
  /// Create new instance of TSPerson from given list of nodes and with defaul distance.
  pub fn new( route : Vec< NodeIndex > ) -> Self
  {
    Self { route, distance : Default::default() }
  }
}

impl Individual for TSPerson
{
  fn fitness( &self ) -> usize
  {
    self.distance as usize
  }

  fn is_optimal( &self ) -> bool
  {
    false
  }

  fn update_fitness( &mut self, value : f64 )
  {
    self.distance = value;
  }
}

impl InitialProblem for TSProblem
{
  type Person = TSPerson;

  fn get_random_person( &self, hrng : Hrng ) -> TSPerson
  {
    let mut list = Vec::new();
    list.push( self.starting_node );

    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();

    let mut nodes = self.graph.nodes().iter().cloned().sorted_by( | n1, n2 | n1.cmp( &n2 ) ).filter( | &n | n != self.starting_node ).collect_vec();
    deterministic_rand::seq::SliceRandom::shuffle( nodes.as_mut_slice(), &mut *rng );

    list.append( &mut nodes );
    list.push( self.starting_node );
    let mut person = TSPerson::new( list );
    let dist = self.evaluate( &person );

    person.update_fitness( dist );

    person
  }

  fn evaluate( &self, person : &TSPerson ) -> f64
  {
    let mut dist = 0.0;
    for ( node1, node2 ) in person.route.iter().tuple_windows()
    {
      if let Some( edge ) = self.graph.get_edge( node1, node2 )
      {
        dist += f64::from( edge.weight() )
      }
      else
      {
        dist += f64::from( f64::INFINITY );
      }
    }

    dist
  }
}

/// Randomly selects a subroute from the first parent and fills the remainder of the route with the nodes from the second parent in the order in which they appear, without duplicating any nodes in the selected subroute from the first parent.
#[ derive( Debug, Clone ) ]
pub struct OrderedRouteCrossover;

impl CrossoverOperator for OrderedRouteCrossover
{
  type Person = TSPerson;
  fn crossover( &self, hrng : Hrng, parent1 : &Self::Person, parent2 : &Self::Person ) -> Self::Person
  {
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();

    let mut child_list = Vec::new();

    let subroute_point1 = ( 1..parent1.route.len() - 2 ).choose( &mut *rng ).unwrap();
    let subroute_point2 = ( 1..parent1.route.len() - 2 ).choose( &mut *rng ).unwrap();

    let start = subroute_point1.min( subroute_point2 );
    let end = subroute_point1.max( subroute_point2 );

    let mut parent1_part = parent1.route.iter().skip( start ).take( end - start ).collect_vec();
    let mut parent2_part = parent2.route.iter().filter( | n | !parent1_part.contains( n ) ).collect_vec();

    for i in ( 0..parent1.route.len() ).rev()
    {
      if i >= start && i < end
      {
        child_list.push( *parent1_part.pop().unwrap() );
      }
      else
      {
        child_list.push( *parent2_part.pop().unwrap() );
      }
    }

    child_list.reverse();

    TSPerson::new( child_list )
  }
}

/// Randomly mutates route in three different ways: by swapping two nodes, by reversing subroute, or by changing position of subroute.
#[ derive( Debug, Clone ) ]
pub struct TSRouteMutation;

impl TSRouteMutation
{
  /// Randomly selects subroute(omitting starting node) and reverses it.
  pub fn reverse_subroute( hrng : Hrng, person : &mut TSPerson )
  {
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();
    let ( pos1, pos2 ) = ( 1..person.route.len() - 2 ).choose_multiple( &mut *rng, 2 ).into_iter().collect_tuple().unwrap();
    let start = pos1.min( pos2 );
    let mut end = pos1.max( pos2 );

    if end - start == 0
    {
      end += 1;
    }

    let mut new_route = person.route.iter().take( start ).collect_vec();
    new_route.extend( person.route.iter().skip( start ).take( end - start - 1 ).rev() );
    new_route.extend( person.route.iter().skip( end - 1 ) );
    let new_route = new_route.into_iter().map( | n | *n ).collect_vec();

    person.route = new_route;
  }

  /// Randomly chooses two nodes that aren't starting node, and swaps them.
  pub fn swap_nodes( hrng : Hrng, person : &mut TSPerson )
  {
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();

    let ( pos1, pos2 ) = ( 1..person.route.len() - 2 ).choose_multiple( &mut *rng, 2 ).into_iter().collect_tuple().unwrap();
    let node1 = person.route[ pos1 ];
    let node2 = std::mem::replace( &mut person.route[ pos2 ], node1 );
    let _ = std::mem::replace( &mut person.route[ pos1 ], node2 );
  }

  /// Randomly selects subroute(omitting starting node) and inserts selected subroute into random position within route.
  pub fn move_subroute( hrng :Hrng, person : &mut TSPerson )
  {
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();
    let ( pos1, pos2,  ) = ( 1..person.route.len() - 1 ).choose_multiple( &mut *rng, 2 ).into_iter().collect_tuple().unwrap();
    let start = pos1.min( pos2 );
    let end = pos1.max( pos2 );

    let mut sub_route = Vec::new();
    sub_route.extend( person.route.iter().take( start ) );
    sub_route.extend( person.route.iter().skip( end ) );
    let insert_position = ( 1..sub_route.len() - 1 ).choose( &mut *rng ).unwrap();

    let mut new_route = Vec::new();
    new_route.extend( sub_route.iter().take( insert_position ) );
    new_route.extend( person.route.iter().skip( start ).take( end - start ) );
    new_route.extend( sub_route.iter().skip( insert_position ) );

    person.route = new_route;
  }
}

impl MutationOperator for TSRouteMutation
{
  type Person = TSPerson;
  type Problem = TSProblem;

  fn mutate( &self, hrng : Hrng, person : &mut Self::Person, _context : &Self::Problem )
  {
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();
    let mutation = [ 1, 2, 3 ].choose( &mut *rng ).unwrap();

    drop( rng );

    match mutation
    {
      1 => Self::move_subroute( hrng.clone(), person ),
      2 => Self::reverse_subroute( hrng.clone(), person ),
      3 => Self::swap_nodes( hrng.clone(), person ),
      _ => unreachable!()
    }
  }
}
