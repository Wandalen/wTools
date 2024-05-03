use iter_tools::Itertools;
use optimization_tools::*;
use problems::traveling_salesman::*;
use hybrid_optimizer::*;
use test_tools::prelude::*;
use deterministic_rand::{ Seed, Hrng };

mod tools;
use tools::*;

#[ test ]
fn tsp_person()
{
  logger_init();

  let hrng = Hrng::master_with_seed( Seed::default() );
  let graph = TSPGraph::default();

  let tsp_initial = TSProblem{ graph, starting_node : NodeIndex( 1 ) };
  let population = tsp_initial.initial_population( hrng.clone(), 1 );
  let person = population[ 0 ].clone();

  log::trace!( "{person:#?}" );
  a_id!( person.route[ 0 ], NodeIndex( 1 ) );
  a_id!( person.route.len(), 5 );
  a_id!( person.route[ person.route.len() - 1 ], NodeIndex( 1 ) );

  let unique = person.route.iter().unique().collect_vec();

  a_id!( person.route.len() - 1, unique.len() );

}

#[ test ]
fn tsp_person_mutate()
{
  logger_init();

  let hrng = Hrng::master_with_seed( Seed::from_integer(1) );
  let graph = TSPGraph::default();

  let tsp_initial = TSProblem{ graph, starting_node : NodeIndex( 1 ) };
  let population = tsp_initial.initial_population( hrng.clone(), 1 );
  let mut person = population[ 0 ].clone();

  log::trace!( "{person:#?}" );

  TSRouteMutation::swap_nodes( hrng.clone(), &mut person );

  log::trace!( "{person:#?}" );

  a_id!( person.route[ 0 ], NodeIndex( 1 ) );
  a_id!( person.route.len(), 5 );
  a_id!( person.route[ person.route.len() - 1 ], NodeIndex( 1 ) );

  let unique = person.route.iter().unique().collect_vec();

  a_id!( person.route.len() - 1, unique.len() );

  TSRouteMutation::reverse_subroute( hrng.clone(), &mut person );

  log::trace!( "{person:#?}" );

  a_id!( person.route[ 0 ], NodeIndex( 1 ) );
  a_id!( person.route.len(), 5 );
  a_id!( person.route[ person.route.len() - 1 ], NodeIndex( 1 ) );

  let unique = person.route.iter().unique().collect_vec();

  a_id!( person.route.len() - 1, unique.len() );

  TSRouteMutation::move_subroute( hrng.clone(), &mut person );

  log::trace!( "{person:#?}" );

  a_id!( person.route[ 0 ], NodeIndex( 1 ) );
  a_id!( person.route.len(), 5 );
  a_id!( person.route[ person.route.len() - 1 ], NodeIndex( 1 ) );

  let unique = person.route.iter().unique().collect_vec();

  a_id!( person.route.len() - 1, unique.len() );
}

#[ ignore ]
#[ test ]
fn find_route()
{
  logger_init();
  log::set_max_level( log::LevelFilter::Warn );

  let graph = TSPGraph::default();

  let tsp_initial = TSProblem{ graph, starting_node : NodeIndex( 1 ) };

  let tsp = Problem::new( tsp_initial, OrderedRouteCrossover{}, TSRouteMutation{} );

  let optimizer = HybridOptimizer::new( Config::default(), tsp )
  .set_population_size( 100 )
  .set_dynasties_limit( 100 );

  log::set_max_level( log::LevelFilter::max() );
  let ( reason, solution ) = optimizer.optimize();

  log::trace!( "reason : {reason}" );
  a_true!( solution.is_some() );
  let solution = solution.unwrap();
  log::trace!( "{solution:#?}" );
  log::trace!( "{:#?}", solution.route );
  a_id!( solution.fitness(), 80 );
}
