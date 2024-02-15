//! Example usage of hybrid optimizer for finding optimal route in traveling salesman problem.
//!

use optimization_tools::*;
use problems::traveling_salesman::*;
use hybrid_optimizer::*;

fn main()
{
  // Create new graph with distances between edges.
  let mut graph = TSPGraph::new();
  graph.add_edge( NodeIndex( 1 ), NodeIndex( 2 ), 10.0 );
  graph.add_edge( NodeIndex( 1 ), NodeIndex( 3 ), 15.0 );
  graph.add_edge( NodeIndex( 1 ), NodeIndex( 4 ), 20.0 );
  graph.add_edge( NodeIndex( 2 ), NodeIndex( 3 ), 35.0 );
  graph.add_edge( NodeIndex( 2 ), NodeIndex( 4 ), 25.0 );
  graph.add_edge( NodeIndex( 3 ), NodeIndex( 4 ), 30.0 );  

  // Create initial TS configuration, passing created graph and starting node.
  let tsp_initial = TSProblem::new( graph, NodeIndex( 1 ) );  
  
  // Create hybrid optimization problem with TS configuration, crossover operator and mutation operator,
  // specific for TS problem.
  let tsp = Problem::new( tsp_initial, OrderedRouteCrossover{}, TSRouteMutation{} );  
  
  // Create new hybrid optimizer with default configuration, and TS hybrid optimization problem.
  let optimizer = HybridOptimizer::new( Config::default(), tsp )
  // If desired, update certain configuration values for optimizer.
  .set_population_size( 100 )
  .set_dynasties_limit( 100 );

  // Perform optimization of given problem. Result includes best found solution and reason for termination 
  // of optimization process.
  let ( reason, solution ) = optimizer.optimize();

  // Print results.
  println!( "reason : {:?}", reason );

  if let Some( solution ) = solution
  {
    println!( "route : {:?}", solution.route );
    println!( "distance : {:?}", solution.distance );
  }
  
}