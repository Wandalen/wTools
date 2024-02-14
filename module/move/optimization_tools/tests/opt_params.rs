use optimization_tools::{ sudoku::Board, hybrid_opt_params::OptimalParamsConfig, * };
use optimization::*;

mod tools;
use tools::*;

fn write_results( filename : String, title : String, params : Vec< f64 > ) -> Result< (), std::io::Error >
{
  let mut file = std::fs::File::create( format!( "{}.md", filename ) )?;
  std::io::Write::write(&mut file, format!( "{}\n\n", title).as_bytes() )?;
  std::io::Write::write(&mut file, b"For parameters:\n")?;

  std::io::Write::write( &mut file,format!( " - temperature decrease coefficient : {:.4};\n", params[ 0 ] ).as_bytes() )?;
  std::io::Write::write( &mut file,format!( " - max mutations per dynasty : {:?};\n", params[ 1 ] as usize ).as_bytes() )?;
  std::io::Write::write( &mut file,format!( " - mutation rate : {:.1};\n", params[ 2 ] ).as_bytes() )?;
  std::io::Write::write( &mut file,format!( " - crossover rate : {:.1};\n", params[ 3 ] ).as_bytes() )?;
  std::io::Write::write( &mut file,format!( " - max stale iterations : {:?};\n", params[ 4 ] as usize ).as_bytes() )?;

  let columns = [ "Level", "Population size", "Dynasties limit", "Execution time" ];
  let mut title = String::from( "| " );
  let mut line = String::from( "|-" );
  let mut result = String::from( "| " );
  let res_columns = 
  [ 
    String::from( "Easy" ), 
    ( params[ 5 ] as usize ).to_string(), 
    ( params[ 6 ] as usize ).to_string(), 
    format!( "{:.3}s", params[ 7 ] )
  ];
  for ( index, column ) in columns.iter().enumerate()
  {
    title.push_str( column );
    result.push_str( &res_columns[ index ] );
    for _ in 0..column.len()
    {
      line.push( '-' );
    }
    for _ in 0..( 20 - column.len() )
    {
      title.push( ' ' );
      line.push( '-' );
    }
    for _ in 0..( 20 - res_columns[ index ].len() )
    {
      result.push( ' ' );
    }
    line.push_str( "-|-" );
    title.push_str( " | " );
    result.push_str( " | " );
  }

  std::io::Write::write( &mut file, format!("\n\n{}\n", title ).as_bytes() )?;
  std::io::Write::write( &mut file, format!("{}\n", line ).as_bytes() )?;
  std::io::Write::write( &mut file, format!("{}\n", result ).as_bytes() )?;

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
  log::set_max_level( log::LevelFilter::Warn );

  let config = OptimalParamsConfig::default();
  let initial = SudokuInitial::new( Board::from( easy ) );

  let hybrid_problem = Problem::new( initial.clone(), BestRowsColumnsCrossover{}, RandomPairInBlockMutation{} );

  let res = hybrid_opt_params::find_hybrid_optimal_params( config, optimization::starting_params_for_hybrid()?, hybrid_problem );
  assert!( res.is_ok() );
  if let Ok( solution ) = res
  {
    let mut result = solution.point.coords.clone();
    result.push( solution.objective );
    write_results( String::from( "sudoku_results" ), String::from( "Sudoku Problem" ), result )?;
  }
  
  Ok( () )
}

#[ ignore ]
#[ test ]
fn find_opt_params_tsp() -> Result< (), Box< dyn std::error::Error > > 
{
  logger_init();
  log::set_max_level( log::LevelFilter::Warn );

  let config = OptimalParamsConfig::default();
  let initial = TSProblem { graph : TSPGraph::default(), starting_node : NodeIndex( 1 ) };
  let hybrid_problem = Problem::new( initial, OrderedRouteCrossover{}, TSRouteMutation{} );

  let res = hybrid_opt_params::find_hybrid_optimal_params( config, optimization::starting_params_for_hybrid()?, hybrid_problem );
  assert!( res.is_ok() );
  if let Ok( solution ) = res
  {
    let mut result = solution.point.coords.clone();
    result.push( solution.objective );
    write_results( String::from( "tsp_results" ), String::from( "Traveling Salesman Problem" ), result )?;
  }
  Ok( () )
}