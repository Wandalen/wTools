use iter_tools::Itertools;
use optimization_tools::{ optimal_params_search::nelder_mead::Stats, * };
use optimal_params_search::OptimalParamsConfig;
use problems::{ sudoku::*, traveling_salesman::* };
use hybrid_optimizer::*;

mod tools;
use tools::*;

fn named_results_list( params : Vec< f64 >, stats : Stats ) -> Vec< ( String, Option< String >, String ) >
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

  let mut stats_vec = stats.diff_sum.iter().cloned().map( | val |  Some( format!( "{:.2}", val ) ) ).collect_vec();
  stats_vec.insert( 4, None );

  let mut list = Vec::new();

  for ( ( name, stats ), param ) in params_name.into_iter().zip( stats_vec ).zip( str_params )
  {
    list.push( ( name.to_owned(), stats, param ) );
  }

  list
}

type ResWithStats = Vec< ( String, Option< String >, String ) >;

fn write_results(
  filename : String,
  title : String,
  hybrid_res : ResWithStats,
  sa_res : ResWithStats,
  ga_res : ResWithStats,
) -> Result< (), std::io::Error >
{
  let mut file = std::fs::File::create( format!( "{}.md", filename ) )?;
  std::io::Write::write( &mut file, format!( "{}\n\n", title ).as_bytes() )?;

  for ( mode, params ) in [ ( "hybrid", &hybrid_res ), ( "SA", &sa_res ), ( "GA", &ga_res ) ]
  {
    std::io::Write::write(&mut file, format!( "For {} parameters:\n", mode ).as_bytes() )?;
    for i in 0..params.len()
    {
      let mut stats_str = String::new();
      if let Some( stats ) = &params[ i ].1
      {
        stats_str = format!( ", sum of differences: {}", stats );
      }
      if mode == "SA"
      {
        if [ 2, 3, 4, 6 ].contains( &i )
        {
          std::io::Write::write( &mut file,format!( " - <em>{} : {}</em>{};\n", params[ i ].0, params[ i ].2, stats_str ).as_bytes() )?;
          continue;
        }
      }
      std::io::Write::write( &mut file,format!( " - {} : {}{};\n", params[ i ].0, params[ i ].2, stats_str ).as_bytes() )?;
    }

    std::io::Write::write( &mut file, format!("\n\n\n" ).as_bytes() )?;
  }

  //table
  use markdown_table::MarkdownTable;

  let mut table_vec = Vec::new();
  let mut headers = vec![ String::from( "mode" ) ];
  for i in 0..hybrid_res.len()
  {
    headers.push( hybrid_res[ i ].0.clone().replace( " ", "\n") );
  }

  table_vec.push( headers );
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
        row.push( params[ i - 1 ].2.clone() );
      }
    }

    table_vec.push( row );
  }

  let table = MarkdownTable::new( table_vec ).as_markdown().unwrap();

  std::io::Write::write( &mut file, format!( "{}", table ).as_bytes() )?;

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
  let res = optimal_params_search::find_hybrid_optimal_params(
    config.clone(),
   hybrid_optimizer::starting_params_for_hybrid()?,
   hybrid_problem,
   Some( path.clone() ),
  );
  assert!( res.is_ok() );

  let mut hybrid_res = Vec::new();
  if let Ok( solution ) = res
  {
    hybrid_res = named_results_list( solution.point.coords.into_iter().map( | val | val.into_inner() ).collect_vec(), solution.stats.unwrap() );
    hybrid_res.push( ( String::from( "level" ), None, format!( "{:?}", Board::from( easy ).calculate_level() ) ) );
    hybrid_res.push( ( String::from( "execution time" ), None, format!( "{:.3}s", solution.objective ) ) );
  }

  // SA
  let hybrid_problem = Problem::new(
    initial.clone(),
    BestRowsColumnsCrossover,
    RandomPairInBlockMutation,
  );
  let res = optimal_params_search::find_hybrid_optimal_params(
    config.clone(),
    hybrid_optimizer::starting_params_for_sa()?,
    hybrid_problem,
    Some( path.clone() ),
  );
  assert!( res.is_ok() );

  let mut sa_res = Vec::new();
  if let Ok( solution ) = res
  {
    sa_res = named_results_list( solution.point.coords.into_iter().map( | val | val.into_inner() ).collect_vec(), solution.stats.unwrap() );
    sa_res.push( ( String::from( "level" ), None, format!( "{:?}", Board::from( easy ).calculate_level() ) ) );
    sa_res.push( ( String::from( "execution time" ), None, format!( "{:.3}s", solution.objective ) ) );
  }

  // GA
  let hybrid_problem = Problem::new(
    initial.clone(),
    BestRowsColumnsCrossover,
    RandomPairInBlockMutation,
  );
  let res = optimal_params_search::find_hybrid_optimal_params(
    config,
    hybrid_optimizer::starting_params_for_ga()?,
    hybrid_problem,
    Some( path ),
  );
  assert!( res.is_ok() );

  let mut ga_res = Vec::new();
  if let Ok( solution ) = res
  {
    ga_res = named_results_list( solution.point.coords.into_iter().map( | val | val.into_inner() ).collect_vec(), solution.stats.unwrap() );
    ga_res.push( ( String::from( "level" ), None, format!( "{:?}", Board::from( easy ).calculate_level() ) ) );
    ga_res.push( ( String::from( "execution time" ), None, format!( "{:.3}s", solution.objective ) ) );
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
  let res = optimal_params_search::find_hybrid_optimal_params(
    config.clone(),
    hybrid_optimizer::starting_params_for_hybrid()?,
    hybrid_problem,
    Some( path.clone() ),
  );
  assert!( res.is_ok() );
  let mut hybrid_res = Vec::new();
  if let Ok( solution ) = res
  {
    hybrid_res = named_results_list( solution.point.coords.into_iter().map( | val | val.into_inner() ).collect_vec(), solution.stats.unwrap() );
    hybrid_res.push( ( String::from( "number of nodes" ), None, number_of_nodes.to_string() ) );
    hybrid_res.push( ( String::from( "execution time" ), None, format!( "{:.3}s", solution.objective ) ) );
  }

  // SA
  let hybrid_problem = Problem::new(
    initial.clone(),
    OrderedRouteCrossover,
    TSRouteMutation,
  );
  let res = optimal_params_search::find_hybrid_optimal_params(
    config.clone(),
    hybrid_optimizer::starting_params_for_sa()?,
    hybrid_problem,
    Some( path.clone() ),
  );
  assert!( res.is_ok() );
  let mut sa_res = Vec::new();
  if let Ok( solution ) = res
  {
    sa_res = named_results_list( solution.point.coords.into_iter().map( | val | val.into_inner() ).collect_vec(), solution.stats.unwrap() );
    sa_res.push( ( String::from( "number of nodes" ), None, number_of_nodes.to_string() ) );
    sa_res.push( ( String::from( "execution time" ), None, format!( "{:.3}s", solution.objective ) ) );
  }

  // GA
  let hybrid_problem = Problem::new(
    initial,
    OrderedRouteCrossover,
    TSRouteMutation,
  );
  let res = optimal_params_search::find_hybrid_optimal_params(
    config,
    hybrid_optimizer::starting_params_for_ga()?,
    hybrid_problem,
    Some( path ),
  );
  assert!( res.is_ok() );
  let mut ga_res = Vec::new();
  if let Ok( solution ) = res
  {
    ga_res = named_results_list( solution.point.coords.into_iter().map( | val | val.into_inner() ).collect_vec(), solution.stats.unwrap() );
    ga_res.push( ( String::from( "number of nodes" ), None, number_of_nodes.to_string() ) );
    ga_res.push( ( String::from( "execution time" ), None, format!( "{:.3}s", solution.objective ) ) );
  }

  write_results( String::from( "tsp_results" ), String::from( "Traveling Salesman Problem" ), hybrid_res, sa_res, ga_res )?;
  Ok( () )
}