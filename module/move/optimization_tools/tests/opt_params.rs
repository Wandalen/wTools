use std::ops::{ Bound, RangeBounds };

use iter_tools::Itertools;
use optimization_tools::{ optimal_params_search::nelder_mead::Stats, * };
use optimal_params_search::OptimalParamsConfig;
use problems::{ sudoku::*, traveling_salesman::* };
use hybrid_optimizer::*;
use tabled::{ builder::Builder, settings::Style };


mod tools;
use tools::*;

fn named_results_list< R : RangeBounds< f64 > >( params : Vec< f64 >, stats : Stats, bounds : Vec< Option< R > > ) -> Vec< Vec< String > >
{
  let mut str_params = Vec::new();
  str_params.push( format!( "{:.4}", params[ 0 ] ) );
  str_params.push( format!( "{:?}", params[ 1 ] as usize ) );
  str_params.push( format!( "{:.2}", params[ 2 ] ) );
  str_params.push( format!( "{:.2}", params[ 3 ] ) );
  str_params.push( format!( "{:.2}", ( 1.0 - params[ 2 ] - params[ 3 ] ) ) );
  str_params.push( format!( "{:?}", params[ 4 ] as usize ) );
  str_params.push( format!( "{}", params[ 5 ] as usize ) );
  str_params.push( format!( "{}", params[ 6 ] as usize ) );

  let mut start_params = Vec::new();
  start_params.push( format!( "{:.4}", stats.starting_point.coords[ 0 ] ) );
  start_params.push( format!( "{:?}", stats.starting_point.coords[ 1 ].into_inner() as usize ) );
  start_params.push( format!( "{:.2}", stats.starting_point.coords[ 2 ] ) );
  start_params.push( format!( "{:.2}", stats.starting_point.coords[ 3 ] ) );
  start_params.push( format!( "{:.2}", ( 1.0 - stats.starting_point.coords[ 2 ].into_inner() - stats.starting_point.coords[ 3 ].into_inner() ) ) );
  start_params.push( format!( "{}", stats.starting_point.coords[ 4 ].into_inner() as usize ) );
  start_params.push( format!( "{}", stats.starting_point.coords[ 5 ].into_inner() as usize ) );
  start_params.push( format!( "{}", stats.starting_point.coords[ 6 ].into_inner() as usize ) );

  let params_name = 
  [
    "temperature decrease coefficient",
    "max mutations per dynasty",
    "mutation rate",
    "crossover rate",
    "elitism rate",
    "max stale iterations",
    "population size",
    "dynasties limit",
  ];

  let mut diff_sum_vec = stats.differences.iter().map( | vec | vec.iter().fold( 0.0, | acc, val | acc + val.abs() ) ).map( | val | format!( "{:.2}", val ) ).collect_vec();
  diff_sum_vec.insert( 4, String::from( "-" ) );

  let mut expectation_vec = Vec::new();
  for i in 0..stats.differences.len()
  { 
    expectation_vec.push( format!( "{:.2}", stats.differences[ i ].iter().fold( 0.0, | acc, val | acc + ( val + stats.starting_point.coords[ i ].into_inner() ) / stats.differences[ i ].len() as f64 ) ) );
  }
  expectation_vec.insert( 4, String::from( "-" ) );

  let mut bounds_vec = bounds.iter().map( | bounds | 
    {
      let mut str = String::from( "-" );
      if let Some( range ) = bounds
      {
        let mut upper = String::new();
        let mut lower = String::new();
        match range.start_bound()
        {
          Bound::Included( val ) =>
          {
            upper = format!( "[ {:.2}", val );
          },
          Bound::Excluded( val ) =>
          {
            upper = format!( "( {:.2}", val );
          },
          Bound::Unbounded => {}
        }

        match range.end_bound()
        {
          Bound::Included( val ) =>
          {
            lower = format!( "{:.2} ]", val );
          },
          Bound::Excluded( val ) =>
          {
            lower = format!( "{:.2} )", val );
          },
          Bound::Unbounded => {}
        }
        str = format!( "{}; {}", upper, lower );
      }
      str
    } ).collect_vec();
  bounds_vec.insert( 4, String::from( "-" ) );

  let mut list = Vec::new();

  for i in 0..params_name.len()
  {
    list.push( vec![ params_name[ i ].to_owned(), start_params[ i ].clone(), bounds_vec[ i ].clone(), diff_sum_vec[ i ].clone(), expectation_vec[ i ].clone(), str_params[ i ].clone() ] );
  }

  list
}

type ResWithStats = Vec< Vec< String > >;

fn write_results(
  filename : String,
  title : String,
  mut hybrid_res : ResWithStats,
  mut sa_res : ResWithStats,
  mut ga_res : ResWithStats,
) -> Result< (), std::io::Error >
{
  let mut file = std::fs::File::create( format!( "{}.md", filename ) )?;
  std::io::Write::write( &mut file, format!( "# {}\n\n", title ).as_bytes() )?;

  for ( mode, params ) in &mut [ ( "hybrid", &mut hybrid_res ), ( "SA", &mut sa_res ), ( "GA", &mut ga_res ) ]
  {
    std::io::Write::write(&mut file, format!( "## For {}:\n\n", mode ).as_bytes() )?;
    let exec_time = params.pop().unwrap();
    std::io::Write::write(&mut file, format!( " - {}: {}\n\n", exec_time[ 0 ], exec_time[ 1 ] ).as_bytes() )?;
    let level = params.pop().unwrap();
    std::io::Write::write(&mut file, format!( " - {}: {}\n\n", level[ 0 ], level[ 1 ] ).as_bytes() )?;
    std::io::Write::write(&mut file, format!( " - parameters: \n\n" ).as_bytes() )?;

    let mut builder = Builder::default();

    let row = [ "", "starting value", "bounds", "sum of differences", "mathematical expectation", "calculated value" ].into_iter().map( str::to_owned ).collect_vec();
    builder.push_record( row );

    for i in 0..params.len()
    {
      let mut row = Vec::new();
    
      if *mode == "SA" && [ 2, 3, 4, 6 ].contains( &i )
      {
        row.push( format!( "<em>{}</em>", params[ i ][ 0 ].clone().replace( " ", "\n") ) );
      }
      else 
      {
        row.push( params[ i ][ 0 ].clone().replace( " ", "\n") );
      }

      row.extend( params[ i ].iter().skip( 1 ).cloned() );
      builder.push_record( row );
      
    }

    let table = builder.build().with( Style::modern() ).to_string();
    std::io::Write::write( &mut file, format!( "```\n{}\n```", table ).as_bytes() )?;

    std::io::Write::write( &mut file, format!("\n\n\n" ).as_bytes() )?;
  }

  //final table
  std::io::Write::write(&mut file, format!( "## Summary:\n" ).as_bytes() )?;
  let mut builder = Builder::default();
  let mut headers = vec![ String::from( "mode" ) ];
  for i in 0..hybrid_res.len()
  {
    headers.push( hybrid_res[ i ][ 0 ].clone().replace( " ", "\n") );
  }

  builder.push_record( headers );
  for ( mode, params ) in [ ( "hybrid", &hybrid_res ), ( "SA", &sa_res ), ( "GA", &ga_res ) ]
  {
    let mut row = Vec::new();
    for i in 0..params.len() + 1
    {
      if i == 0
      {
        row.push( mode.to_owned() );
      }
      else
      {
        row.push( params[ i - 1 ].last().unwrap().clone() );
      }
    }

    builder.push_record( row );
  }

  let table = builder.build().with( Style::modern() ).to_string();
  std::io::Write::write( &mut file, format!( "```\n{}\n```", table ).as_bytes() )?;

  Ok( () )
}

#[ ignore ]
#[ test ]
fn find_opt_params_sudoku() -> Result< (), Box< dyn std::error::Error > >
{
  let easy = r#"
  080924060
  920060105
  360080029
  408209600
  106003802
  002806390
  840690070
  009705208
  075040036
  "#;

  logger_init();
  log::set_max_level( log::LevelFilter::Info );

  let dir_path = format!( "{}/target", crate::simplex::drawing::workspace_dir().to_string_lossy() );
  _ = std::fs::create_dir( &dir_path );
  let path = format!( "{}/output_sudoku", dir_path );

  let config = OptimalParamsConfig::default();
  let initial = SudokuInitial::new( Board::from( easy ) );

  let hybrid_problem = Problem::new(
    initial.clone(),
    BestRowsColumnsCrossover,
    RandomPairInBlockMutation,
  );
  let starting_params = hybrid_optimizer::starting_params_for_hybrid()?;
  let res = optimal_params_search::find_hybrid_optimal_params(
    config.clone(),
    starting_params.clone(),
    hybrid_problem,
    Some( path.clone() ),
  );
  assert!( res.is_ok() );

  let mut hybrid_res = Vec::new();
  if let Ok( solution ) = res
  {
    hybrid_res = named_results_list( solution.point.coords.into_iter().map( | val | val.into_inner() ).collect_vec(), solution.stats.unwrap(), starting_params.bounds );
    hybrid_res.push( vec![ String::from( "level" ), format!( "{:?}", Board::from( easy ).calculate_level() ) ] );
    hybrid_res.push( vec![ String::from( "execution time" ), format!( "{:.3}s", solution.objective ) ] );
  }

  // SA
  let hybrid_problem = Problem::new(
    initial.clone(),
    BestRowsColumnsCrossover,
    RandomPairInBlockMutation,
  );
  let starting_params = hybrid_optimizer::starting_params_for_sa()?;
  let res = optimal_params_search::find_hybrid_optimal_params(
    config.clone(),
    starting_params.clone(),
    hybrid_problem,
    Some( path.clone() ),
  );
  assert!( res.is_ok() );

  let mut sa_res = Vec::new();
  if let Ok( solution ) = res
  {
    sa_res = named_results_list( solution.point.coords.into_iter().map( | val | val.into_inner() ).collect_vec(), solution.stats.unwrap(), starting_params.bounds );
    sa_res.push( vec![ String::from( "level" ), format!( "{:?}", Board::from( easy ).calculate_level() ) ] );
    sa_res.push( vec![ String::from( "execution time" ), format!( "{:.3}s", solution.objective ) ] );
  }

  // GA
  let hybrid_problem = Problem::new(
    initial.clone(),
    BestRowsColumnsCrossover,
    RandomPairInBlockMutation,
  );
  let starting_params = hybrid_optimizer::starting_params_for_ga()?;
  let res = optimal_params_search::find_hybrid_optimal_params(
    config,
    starting_params.clone(),
    hybrid_problem,
    Some( path ),
  );
  assert!( res.is_ok() );

  let mut ga_res = Vec::new();
  if let Ok( solution ) = res
  {
    ga_res = named_results_list( solution.point.coords.into_iter().map( | val | val.into_inner() ).collect_vec(), solution.stats.unwrap(), starting_params.bounds );
    ga_res.push( vec![ String::from( "level" ), format!( "{:?}", Board::from( easy ).calculate_level() ) ] );
    ga_res.push( vec![ String::from( "execution time" ), format!( "{:.3}s", solution.objective ) ] );
  }
  write_results( String::from( "sudoku_results" ), String::from( "Sudoku Problem" ), hybrid_res, sa_res, ga_res )?;
  Ok( () )
}

#[ ignore ]
#[ test ]
fn find_opt_params_tsp() -> Result< (), Box< dyn std::error::Error > >
{
  logger_init();
  log::set_max_level( log::LevelFilter::Info );

  let dir_path = format!( "{}/target", crate::simplex::drawing::workspace_dir().to_string_lossy() );
  _ = std::fs::create_dir( &dir_path );
  let path = format!( "{}/output_tsp", dir_path );

  let config = OptimalParamsConfig::default();
  let graph = TSPGraph::default();
  let number_of_nodes = graph.nodes().len();
  let initial = TSProblem { graph, starting_node : NodeIndex( 1 ) };

  let hybrid_problem = Problem::new(
    initial.clone(),
    OrderedRouteCrossover,
    TSRouteMutation,
  );
  let starting_params = hybrid_optimizer::starting_params_for_hybrid()?;
  let res = optimal_params_search::find_hybrid_optimal_params(
    config.clone(),
    starting_params.clone(),
    hybrid_problem,
    Some( path.clone() ),
  );
  assert!( res.is_ok() );
  let mut hybrid_res = Vec::new();
  if let Ok( solution ) = res
  {
    hybrid_res = named_results_list( solution.point.coords.into_iter().map( | val | val.into_inner() ).collect_vec(), solution.stats.unwrap(), starting_params.bounds );
    hybrid_res.push( vec![ String::from( "number of nodes" ), number_of_nodes.to_string() ] );
    hybrid_res.push( vec![ String::from( "execution time" ), format!( "{:.3}s", solution.objective ) ] );
  }

  // SA
  let hybrid_problem = Problem::new(
    initial.clone(),
    OrderedRouteCrossover,
    TSRouteMutation,
  );
  let starting_params = hybrid_optimizer::starting_params_for_sa()?;
  let res = optimal_params_search::find_hybrid_optimal_params(
    config.clone(),
    starting_params.clone(),
    hybrid_problem,
    Some( path.clone() ),
  );
  assert!( res.is_ok() );
  let mut sa_res = Vec::new();
  if let Ok( solution ) = res
  {
    sa_res = named_results_list( solution.point.coords.into_iter().map( | val | val.into_inner() ).collect_vec(), solution.stats.unwrap(), starting_params.bounds );
    sa_res.push( vec![ String::from( "number of nodes" ), number_of_nodes.to_string() ] );
    sa_res.push( vec![ String::from( "execution time" ), format!( "{:.3}s", solution.objective ) ] );
  }

  // GA
  let hybrid_problem = Problem::new(
    initial,
    OrderedRouteCrossover,
    TSRouteMutation,
  );
  let starting_params = hybrid_optimizer::starting_params_for_ga()?;
  let res = optimal_params_search::find_hybrid_optimal_params(
    config,
    starting_params.clone(),
    hybrid_problem,
    Some( path ),
  );
  assert!( res.is_ok() );
  let mut ga_res = Vec::new();
  if let Ok( solution ) = res
  {
    ga_res = named_results_list( solution.point.coords.into_iter().map( | val | val.into_inner() ).collect_vec(), solution.stats.unwrap(), starting_params.bounds );
    ga_res.push( vec![ String::from( "number of nodes" ), number_of_nodes.to_string() ] );
    ga_res.push( vec![ String::from( "execution time" ), format!( "{:.3}s", solution.objective ) ] );
  }

  write_results( String::from( "tsp_results" ), String::from( "Traveling Salesman Problem" ), hybrid_res, sa_res, ga_res )?;
  Ok( () )
}