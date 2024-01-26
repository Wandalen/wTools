use std::collections::HashMap;
use crate::optimization::*;

use derive_tools::{ FromInner, InnerFrom };
use deterministic_rand::{ Hrng, Rng, seq::SliceRandom };
use iter_tools::Itertools;


trait Graph 
{
  type N;
  type E;
  fn has_edge( &self, node1 : &Self::N, node2 : &Self::N ) -> bool;
  fn add_edge( &mut self, nodes : ( Self::N, Self::N ), weight : f64 );
  fn nodes_number( &self ) -> usize; 
  fn nodes( &self ) -> Vec< Self::N >;
  fn get_edge( &self, node1 : &Self::N, node2 : &Self::N  ) -> Option< Self::E >;
}

pub struct TSPGraph
{
  adjacency_list : HashMap< NodeIndex, Vec < ( NodeIndex, EdgeWeight ) > >,
}

#[ derive( Debug, PartialEq, Eq, Hash ) ]
pub struct Node
{
  pub value : String,
  pub index : usize,
}

#[ derive( Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord ) ]
pub struct NodeIndex( usize );

#[ derive( Debug, FromInner, InnerFrom, Clone, Copy ) ]
pub struct EdgeWeight( f64 );

pub struct Edge( NodeIndex, NodeIndex, EdgeWeight );

impl Edge
{
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

  fn add_edge( &mut self, nodes : ( Self::N, Self::N ), weight : f64 ) 
  {
    self.adjacency_list.entry( nodes.0 ).or_default().push( ( nodes.1, weight.into() ) );
  }

  fn nodes_number( &self ) -> usize 
  {
    self.adjacency_list.keys().len()
  }

  fn nodes( &self ) -> Vec< NodeIndex >
  {
    self.adjacency_list.keys().map( | k | *k ).collect_vec()
  }
}

pub struct TSPSeeder
{
  starting_node : NodeIndex,
  graph : TSPGraph,
}

impl TSPSeeder
{
  pub fn evaluate( &self, person : &TSPerson ) -> f64
  {
    let mut dist = 0.0;
    let edges = person.route.iter().tuples();
    for ( node1, node2 ) in edges
    {
      dist += f64::from( self.graph.get_edge( node1, node2 ).unwrap().weight() );
    }

    dist
  }
}

#[ derive( Debug, PartialEq, Clone ) ]
pub struct TSPerson 
{
  route : Vec< NodeIndex >,
  distance : f64,
}

impl TSPerson
{
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

impl SeederOperator for TSPSeeder
{
  type Person = TSPerson;
  type Context = ();
  fn initial_generation( &self, hrng : Hrng, size : usize ) -> Vec< Self::Person > 
  {
    let mut population = Vec::new();
    
    for _ in 0..size
    {
      let mut list = Vec::new();
      list.push( self.starting_node );

      let rng_ref = hrng.rng_ref();
      let mut rng = rng_ref.lock().unwrap();

      let mut nodes = self.graph.nodes();
      nodes.shuffle( &mut *rng );

      list.append( &mut nodes );
      list.push( self.starting_node );
      let mut person = TSPerson::new( list );
      let dist = self.evaluate( &person );

      person.update_fitness( dist );

      population.push( person );
    }

    population
  }

  fn evaluate( &self, person : &TSPerson ) -> f64 
  {
    let mut dist = 0.0;
    for ( node1, node2 ) in person.route.iter().tuples()
    {
      dist += f64::from( self.graph.get_edge( node1, node2 ).unwrap().weight() );
    }

    dist
  }

  fn context( &self ) -> &Self::Context 
  {
    &()
  }

  fn initial_temperature( &self, _hrng : Hrng ) -> Temperature 
  {
        
    let nodes = self.graph.nodes();

    let mut dist_vec = Vec::new();
    for i in 0..nodes.len() - 1
    {
      for j in i + 1..nodes.len()
      {
        dist_vec.push( self.graph.get_edge( &nodes[ i ], &nodes[ j ] ).unwrap().weight() );
      }
    }

    dist_vec.sort_by( | w1, w2 | w1.0.total_cmp( &w2.0 ) );

    let dist_len = dist_vec.len();

    let mut prev_diff = dist_vec.iter().skip( 1 ).fold( 0.0, | acc, w | acc + ( w.0 - dist_vec[ 0 ].0 ));

    let mut total_diff = prev_diff;
    for i in 1..dist_len
    {
      prev_diff = prev_diff - ( dist_vec[ i ].0 - dist_vec[ i - 1 ].0 ) * ( ( dist_len - i ) as f64 );
      total_diff += prev_diff;
    }

    ( total_diff / ( ( dist_len * ( dist_len - 1 ) ) as f64 / 2.0 ) ).into()
  }
}

impl SelectionOperator< TSPerson > for TournamentSelection
{
  fn select< 'a >( &self, hrng : Hrng, population : &'a Vec< TSPerson > ) -> &'a TSPerson 
  {
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();
    let mut candidates = Vec::new();
    for _ in 0..self.size
    {
      candidates.push( population.choose( &mut *rng ).unwrap() );
    }
    candidates.sort_by( | c1, c2 | c1.fitness().cmp( &c2.fitness() ) );

    let rand : f64 = rng.gen();
    let mut selection_pressure = self.selection_pressure;
    let mut winner = *candidates.last().unwrap();
    for i in 0..self.size
    {
      if rand < selection_pressure
      {
        winner = candidates[ i ];
        break;
      }
      selection_pressure += selection_pressure * ( 1.0 - selection_pressure );
    }
    winner
  }
}

#[ derive( Debug ) ]
pub struct OrderedRouteCrossover {}

impl CrossoverOperator for OrderedRouteCrossover
{
  type Person = TSPerson;
  fn crossover( &self, hrng : Hrng, parent1 : &Self::Person, parent2 : &Self::Person ) -> Self::Person 
  {
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();

    let mut child_list = vec![ NodeIndex( 0 ); parent1.route.len() ];

    let subroute_point1 = ( 1..parent1.route.len() - 1 ).choose( &mut *rng ).unwrap();
    let subroute_point2 = ( 1..parent1.route.len() - 1 ).choose( &mut *rng ).unwrap();

    let start = subroute_point1.min( subroute_point2 );
    let end = subroute_point1.max( subroute_point2 );

    let mut parent1_part = parent1.route.iter().skip( start ).take( end - start ).collect_vec();
    let mut parent2_part = parent2.route.iter().filter( | n | parent1_part.contains( n ) ).collect_vec();
    child_list.push( parent1.route[ 0 ] );

    for i in parent2.route.len()-1..0
    {
      if i >= start && i <= end
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

#[ derive( Debug ) ]
pub struct TSRouteMutation {}

impl TSRouteMutation
{
  fn reverse_subroute( hrng : Hrng, person : &mut TSPerson )
  {
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();
    let ( node1, node2 ) = person.route.choose_multiple( &mut *rng, 2 ).collect_tuple().unwrap();
    let start = node1.min( node2 );
    let end = node1.max( node2 );

    let mut new_route = person.route.iter().take( start.0 - 1 ).collect_vec();
    new_route.extend( person.route.iter().skip( start.0 - 1 ).take( end.0 - start.0 ).rev() );
    new_route.extend( person.route.iter().skip( end.0 - 1 ) );
    let new_route = new_route.into_iter().map( | n | *n ).collect_vec();
    
    person.route = new_route;
  }

  fn swap_nodes( hrng : Hrng, person : &mut TSPerson )
  {
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();

    let ( pos1, pos2 ) = ( 1..person.route.len() - 2 ).choose_multiple( &mut *rng, 2 ).into_iter().collect_tuple().unwrap();
    let node1 = person.route[ pos1 ];
    let node2 = std::mem::replace( &mut person.route[ pos2 ], node1 );
    let _ = std::mem::replace( &mut person.route[ pos1 ], node2 );
  }

  fn move_subroute( hrng :Hrng, person : &mut TSPerson )
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
    new_route.extend( sub_route.iter().take( insert_position - 1 ) );
    new_route.extend( person.route.iter().skip( start ).take( end - start ) );
    new_route.extend( sub_route.iter().skip( insert_position - 1 ) );

    person.route = new_route;
  }
}

impl MutationOperator for TSRouteMutation
{
  type Person = TSPerson;
  type Context = ();

  fn mutate( &self, hrng : Hrng, person : &mut Self::Person, _context : &Self::Context ) 
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