use std::ops::{ Bound, RangeBounds };

use iter_tools::Itertools;
use optimization_tools::{ optimal_params_search::nelder_mead::Stats, * };
use optimal_params_search::OptimalParamsConfig;
use problems::{ sudoku::*, traveling_salesman::* };
use hybrid_optimizer::*;
use tabled::{ builder::Builder, settings::Style };


mod tools;
use tools::*;

pub struct Statistics
{
  pub table_params : Vec< Vec< String > >,
  pub list_params : Vec< ( String, String ) >,
}

impl Statistics
{
  pub fn new() -> Self
  {
    Self
    {
      table_params : Vec::new(),
      list_params : Vec::new(),
    }
  }
}

fn named_results_list< R : RangeBounds< f64 > >
(
  params : Vec< f64 >,
  stats : Stats,
  bounds : Vec< Option< R > >,
) -> Vec< Vec< String > >
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
  start_params.push( format!( "{:?}", stats.starting_point.coords[ 1 ] as usize ) );
  start_params.push( format!( "{:.2}", stats.starting_point.coords[ 2 ] ) );
  start_params.push( format!( "{:.2}", stats.starting_point.coords[ 3 ] ) );
  start_params.push( format!( "{:.2}", ( 1.0 - stats.starting_point.coords[ 2 ] - stats.starting_point.coords[ 3 ] ) ) );
  start_params.push( format!( "{}", stats.starting_point.coords[ 4 ] as usize ) );
  start_params.push( format!( "{}", stats.starting_point.coords[ 5 ] as usize ) );
  start_params.push( format!( "{}", stats.starting_point.coords[ 6 ] as usize ) );

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

  let mut diff_sum_vec = stats.differences
  .iter()
  .map( | vec | format!( "{:.2}", vec.iter().fold( 0.0, | acc, val | acc + val.abs() ) ) )
  .collect_vec()
  ;

  diff_sum_vec.insert( 4, String::from( "-" ) );

  let mut expectation_vec = Vec::new();
  for i in 0..stats.differences.len()
  { 
    expectation_vec.push
    (
      format!
      (
        "{:.2}",
        stats.differences[ i ]
        .iter()
        .fold( 0.0, | acc, val | acc + val.abs() / stats.differences[ i ].len() as f64 )
      )
    );
  }
  expectation_vec.insert( 4, String::from( "-" ) );

  let mut bounds_vec = bounds.iter().map( | bounds | 
    {
      let mut str = ( String::from( "-" ), String::from( "-" ) );
      if let Some( range ) = bounds
      {
        let mut upper = String::new();
        let mut lower = String::new();
        match range.start_bound()
        {
          Bound::Included( val ) =>
          {
            lower = format!( "{:.2}", val );
          },
          Bound::Excluded( val ) =>
          {
            lower = format!( "{:.2}", val );
          },
          Bound::Unbounded => {}
        }

        match range.end_bound()
        {
          Bound::Included( val ) =>
          {
            upper = format!( "{:.2}", val );
          },
          Bound::Excluded( val ) =>
          {
            upper = format!( "{:.2}", val );
          },
          Bound::Unbounded => {}
        }
        str = ( lower, upper );
      }
      str
    } ).collect_vec();
  bounds_vec.insert( 4, ( String::from( "-" ), String::from( "-" ) ) );

  let mut change_vec = Vec::new();
  for i in 0..stats.positive_change.len()
  { 
    change_vec.push( format!( "{}", stats.positive_change[ i ] ) );
  }
  // elitism
  change_vec.insert( 4, String::from( "-" ) );

  let mut list = Vec::new();

  for i in 0..params_name.len()
  {
    list.push
    ( 
      vec!
      [
        params_name[ i ].to_owned(),
        start_params[ i ].clone(),
        bounds_vec[ i ].0.clone(),
        bounds_vec[ i ].1.clone(),
        diff_sum_vec[ i ].clone(),
        expectation_vec[ i ].clone(),
        change_vec[ i ].clone(),
        str_params[ i ].clone()
      ]
    );
  }

  list
}

fn write_results
(
  filename : String,
  title : String,
  mut hybrid_res : Statistics,
  mut sa_res : Statistics,
  mut ga_res : Statistics,
) -> Result< (), std::io::Error >
{
  let mut file = std::fs::File::create( format!( "{}.md", filename ) )?;
  std::io::Write::write( &mut file, format!( "# {}\n\n", title ).as_bytes() )?;

  for ( mode, params ) in &mut [ ( "hybrid", &mut hybrid_res ), ( "SA", &mut sa_res ), ( "GA", &mut ga_res ) ]
  {
    std::io::Write::write(&mut file, format!( "## For {}:\n\n", mode ).as_bytes() )?;
    for param in &params.list_params
    {
      std::io::Write::write(&mut file, format!( " - {}: {}\n\n", param.0, param.1 ).as_bytes() )?;
    }

    std::io::Write::write(&mut file, format!( " - parameters: \n\n" ).as_bytes() )?;

    let mut builder = Builder::default();
    let head_row = [ "", "start", "min", "max", "sum of diff", "expected", "changes", "final" ]
    .into_iter()
    .map( str::to_owned )
    .collect_vec()
    ;

    builder.push_record( head_row.clone() );

    for i in 0..params.table_params.len()
    {
      let mut row = Vec::new();
    
      if *mode == "SA" && [ 2, 3, 4, 6 ].contains( &i )
      {
        row.push( format!( "{}", params.table_params[ i ][ 0 ].clone().replace( " ", "\n") ) );
      }
      else 
      {
        row.push( params.table_params[ i ][ 0 ].clone().replace( " ", "\n") );
      }

      row.extend( params.table_params[ i ].iter().skip( 1 ).cloned() );
      builder.push_record( row );
      
    }

    let table = builder.build().with( Style::modern() ).to_string();
    std::io::Write::write( &mut file, format!( "```\n{}\n```", table ).as_bytes() )?;
    std::io::Write::write( &mut file, format!("\n\n\n" ).as_bytes() )?;

    std::io::Write::write(&mut file, format!( "#### List:\n" ).as_bytes() )?;
    let problem_level = if params.list_params[ params.list_params.len() - 2 ].0 == String::from( "level" )
    {
      " - `level` : sudoku board difficulty level\n"
    }
    else
    {
      " - `number of nodes` : number of nodes in graph representing cities from traveling salesman problem\n"
    };

    let list_legend = concat!
    (
      "\n\n",
      " - `max number of iterations` : limit of total iterations of optimization process, termination condition\n",
      " - `max no improvement iterations` : max amount of steps performed without detected improvement, termination condition\n",
      " - `improvement threshold` : minimal value detected as improvement in objective function result\n",
      " - `termination reason` : the reason why optimization process was stopped\n",
      " - `iterations number` : actual number of iterations performed during optimization\n",
      " - `resumed after stale` : how many times optimization progress was resumed after some iterations without improvement\n",
      " - `points from cache` : points calculated during previous optimizations and read from cache\n",
    );

    std::io::Write::write(&mut file, list_legend.as_bytes() )?;
    std::io::Write::write(&mut file, problem_level.as_bytes() )?;
    std::io::Write::write(&mut file, b" - `execution time` : duration of shortest found hybrid optimization process using final parameters, measured in seconds\n" )?;
    std::io::Write::write(&mut file, format!( "#### Table:\n" ).as_bytes() )?;
    let str_legend = concat!
    (
      " - `start` : initial value of parameter in starting point\n",
      " - `min` : lower bound of parameter\n",
      " - `max` : upper bound of parameter\n",
      " - `sum of diff` : sum of absolute differences between starting value and next value\n",
      " - `expected` : mathematical expectation of difference between starting value and next value\n",
      " - `changes` : number of successful changes of parameter value to more optimal\n",
      " - `final` : calculated value of parameter for which execution time was the lowest\n",
    );
  
    std::io::Write::write( &mut file, str_legend.as_bytes() )?;
  }

  //final table
  std::io::Write::write(&mut file, format!( "## Summary:\n" ).as_bytes() )?;
  let mut builder = Builder::default();
  let mut headers = vec![ String::from( "mode" ) ];
  for i in 0..hybrid_res.table_params.len()
  {
    headers.push( hybrid_res.table_params[ i ][ 0 ].clone().replace( " ", "\n") );
  }

  headers.push( String::from( "execution\ntime" ) );

  builder.push_record( headers );
  for ( mode, params ) in [ ( "hybrid", &hybrid_res ), ( "SA", &sa_res ), ( "GA", &ga_res ) ]
  {
    let mut row = Vec::new();
    for i in 0..params.table_params.len() + 1
    {
      if i == 0
      {
        row.push( mode.to_owned() );
      }
      else
      {
        row.push( params.table_params[ i - 1 ].last().unwrap().clone() );
      }
    }
    row.push( params.list_params.last().unwrap().1.clone() );

    builder.push_record( row );
  }

  let table = builder.build().with( Style::modern() ).to_string();
  std::io::Write::write( &mut file, format!( "```\n{}\n```", table ).as_bytes() )?;

  let final_legend = concat!
  (
    "\n\n",
    " - `temperature decrease coefficient` : coefficient by which temperature is lowered at each iteration of optimization process\n",
    " - `max mutations per dynasty` : max number of mutations used to produce vital individual in dynasty\n",
    " - `mutation rate` : percent of individuals in population that are created using mutation\n",
    " - `crossover rate` : percent of individuals in population that are created using crossover of selected parents\n",
    " - `elitism rate` : percent of most fit individuals in population that are cloned without changes\n",
    " - sum of mutation rate, crossover rate and elitism rate always equals 1\n",
    " - `max stale iterations` : max allowed number of iterations that do not produce individuals with better fittness\n",
    " - `population size` : number of individuals in population\n",
    " - `dynasties limit` : max number of dynasties of new solutions produced during optimization process, terminates if exceeded\n",
    " - `execution time` : time spent searching for optimal solution, measured in seconds\n",
  );
  std::io::Write::write( &mut file, final_legend.as_bytes() )?;

  std::io::Write::write(&mut file, format!( "## To run:\n" ).as_bytes() )?;
  std::io::Write::write( &mut file, b" - Sudoku problem:\n" )?;
  std::io::Write::write( &mut file, b"`cargo test -- --ignored find_opt_params_sudoku`\n" )?;
  std::io::Write::write( &mut file, b" - Traveling salesman problem:\n" )?;
  std::io::Write::write( &mut file, b"`cargo test -- --ignored find_opt_params_tsp`\n" )?;

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

  let mut hybrid_res = Statistics::new();
  let mut sa_res = Statistics::new();
  let mut ga_res = Statistics::new();
  for mode in [ "hybrid", "sa", "ga" ]
  {
    let mut starting_params = hybrid_optimizer::starting_params_for_hybrid()?;
    match mode
    {
      "hybrid" => {},
      "sa" => starting_params = hybrid_optimizer::starting_params_for_sa()?,
      "ga" => starting_params = hybrid_optimizer::starting_params_for_ga()?,
      _ => unreachable!(),
    }

    let hybrid_problem = Problem::new
    (
      initial.clone(),
      BestRowsColumnsCrossover,
      RandomPairInBlockMutation,
    );

    let res = optimal_params_search::find_hybrid_optimal_params
    (
      config.clone(),
      starting_params.clone(),
      hybrid_problem,
      Some( path.clone() ),
    );
    assert!( res.is_ok() );
  
    if let Ok( solution ) = res
    {
      assert!( solution.stats.is_some() );
      let stats = solution.stats.clone().unwrap();
      let cached = stats.cached_points;
      let final_res = Statistics
      {
        table_params : named_results_list
        (
          solution.point.coords
          .into_iter()
          .map( | val | val )
          .collect_vec(),
          solution.stats.unwrap(),
          starting_params.bounds,
        ),
        list_params : vec!
        [ 
          ( String::from( "max number of iterations" ), format!( "{}", config.max_iterations ) ),
          ( String::from( "max no improvement iterations " ), format!( "{}", config.max_no_improvement_steps ) ),
          ( String::from( "improvement threshold " ), format!( "{}s", config.improvement_threshold ) ),
          ( String::from( "termination reason" ), format!( "{}", solution.reason ) ),
          ( String::from( "iterations number" ), format!( "{}", stats.number_of_iterations ) ),
          ( String::from( "resumed after stale" ), format!( "{}", stats.resumed_after_stale ) ),
          ( String::from( "points from cache" ), format!( "{}/{}", cached.0, cached.1 + cached.0 ) ),
          ( String::from( "level" ), format!( "{:?}", Board::from( easy ).calculate_level() ) ),
          ( String::from( "execution time" ), format!( "{:.3}s", solution.objective ) ),
        ]
      };

      match mode
      {
        "hybrid" => hybrid_res = final_res,
        "sa" => sa_res = final_res,
        "ga" => ga_res = final_res,
        _ => unreachable!(),
      }
    }
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
  let res = optimal_params_search::find_hybrid_optimal_params
  (
    config.clone(),
    starting_params.clone(),
    hybrid_problem,
    Some( path.clone() ),
  );
  assert!( res.is_ok() );
  let mut hybrid_res = Statistics::new();
  if let Ok( solution ) = res
  {
    let cached = solution.stats.clone().unwrap().cached_points;
    hybrid_res = Statistics
    {
      table_params : named_results_list
      (
        solution.point.coords
        .into_iter()
        .map( | val | val )
        .collect_vec(),
        solution.stats.unwrap(),
        starting_params.bounds,
      ),
      list_params : vec!
      [
        ( String::from( "max number of iterations" ), format!( "{}", config.max_iterations ) ),
        ( String::from( "max no improvement iterations " ), format!( "{}", config.max_no_improvement_steps ) ),
        ( String::from( "improvement threshold " ), format!( "{}s", config.improvement_threshold ) ),
        ( String::from( "calculated points" ), format!( "{} from {}", cached.1, cached.1 + cached.0 ) ),
        ( String::from( "points from cache" ), format!( "{} from {}", cached.0, cached.1 + cached.0 ) ),
        ( String::from( "number of nodes" ), format!( "{}", number_of_nodes ) ),
        ( String::from( "execution time" ), format!( "{:.3}s", solution.objective ) ),
      ]
    }
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
  let mut sa_res = Statistics::new();
  if let Ok( solution ) = res
  {
    let cached = solution.stats.clone().unwrap().cached_points;
    sa_res = Statistics
    {
      table_params : named_results_list
      (
        solution.point.coords
        .into_iter()
        .map( | val | val )
        .collect_vec(),
        solution.stats.unwrap(),
        starting_params.bounds,
      ),
      list_params : vec!
      [
        ( String::from( "max number of iterations" ), format!( "{}", config.max_iterations ) ),
        ( String::from( "max no improvement iterations " ), format!( "{}", config.max_no_improvement_steps ) ),
        ( String::from( "improvement threshold " ), format!( "{}s", config.improvement_threshold ) ),
        ( String::from( "calculated points" ), format!( "{} from {}", cached.1, cached.1 + cached.0 ) ),
        ( String::from( "points from cache" ), format!( "{} from {}", cached.0, cached.1 + cached.0 ) ),
        ( String::from( "number of nodes" ), format!( "{}", number_of_nodes ) ),
        ( String::from( "execution time" ), format!( "{:.3}s", solution.objective ) ),
      ]
    }
  }

  // GA
  let hybrid_problem = Problem::new(
    initial,
    OrderedRouteCrossover,
    TSRouteMutation,
  );
  let starting_params = hybrid_optimizer::starting_params_for_ga()?;
  let res = optimal_params_search::find_hybrid_optimal_params(
    config.clone(),
    starting_params.clone(),
    hybrid_problem,
    Some( path ),
  );
  assert!( res.is_ok() );
  let mut ga_res = Statistics::new();
  
  if let Ok( solution ) = res
  {
    let cached = solution.stats.clone().unwrap().cached_points;
    ga_res = Statistics
    {
      table_params : named_results_list
      (
        solution.point.coords
        .into_iter()
        .map( | val | val )
        .collect_vec(),
        solution.stats.unwrap(),
        starting_params.bounds,
      ),
      list_params : vec!
      [
        ( String::from( "max number of iterations" ), format!( "{}", config.max_iterations ) ),
        ( String::from( "max no improvement iterations " ), format!( "{}", config.max_no_improvement_steps ) ),
        ( String::from( "improvement threshold " ), format!( "{}s", config.improvement_threshold ) ),
        ( String::from( "calculated points" ), format!( "{} from {}", cached.1, cached.1 + cached.0 ) ),
        ( String::from( "points from cache" ), format!( "{} from {}", cached.0, cached.1 + cached.0 ) ),
        ( String::from( "number of nodes" ), format!( "{}", number_of_nodes ) ),
        ( String::from( "execution time" ), format!( "{:.3}s", solution.objective ) ),
      ]
    }
  }

  write_results( String::from( "tsp_results" ), String::from( "Traveling Salesman Problem" ), hybrid_res, sa_res, ga_res )?;
  Ok( () )
}
