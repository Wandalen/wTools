use std::collections::HashMap;
use std::time::{ Instant, Duration };

use iter_tools::Itertools;

use crate::sudoku::{ Board, BlockIndex };

#[ derive( Debug, Clone ) ] 
pub struct ParamsCase
{
  pub temp_decrease : f64,
  pub temp_increase : f64,
  pub gen_number : usize,
}


impl ParamsCase
{
  pub fn new( temp_decrease : f64, temp_increase: f64, gen_number : usize ) -> Self
  {
    Self
    {
      temp_decrease,
      temp_increase,
      gen_number,
    }
  }
}

pub struct ParamsFitChecker< F >
{
  pub proc : F,
  pub lower_bound_case : ParamsCase,
  pub upper_bound_case : ParamsCase,
  pub number_of_iterations : usize,
}

impl< F > ParamsFitChecker< F >
where F : Fn( ParamsCase )
{
  pub fn get_case_results( &self, case : ParamsCase ) -> Duration
  {
    let mut results: Vec< Duration > = Vec::new();
    for _ in 0..self.number_of_iterations
    {
      let val = case.clone();
      let now = Instant::now();
      ( self.proc )( val );
      let elapsed = now.elapsed();
      results.push( elapsed );
    }
    let size = results.len() as u128;
    let average = results
    .into_iter()
    .fold(0, | acc, elem | acc + elem.as_millis() / size )
    ;
    Duration::from_millis( average as u64 )
  }

  pub fn get_min_points( &self ) -> ParamsCase
  {
    let mut decrease_factor_range = ( self.lower_bound_case.temp_decrease, self.upper_bound_case.temp_decrease );
    let mut increase_factor_range = ( self.lower_bound_case.temp_increase, self.upper_bound_case.temp_increase );
    let mut gen_number_range = ( self.lower_bound_case.gen_number, self.upper_bound_case.gen_number );

    let mut res : ParamsCase = self.lower_bound_case.clone();

    for _ in 0..5
    {
      let mut results = Vec::new();
      for _ in 0..10
      {
        let step = rand::distributions::Uniform::new( decrease_factor_range.0, decrease_factor_range.1 );
        let mut rng = rand::thread_rng();
        let d_factor = rand::distributions::Distribution::sample(&step, &mut rng);
  
        let step = rand::distributions::Uniform::new( increase_factor_range.0, increase_factor_range.1 );
        let mut rng = rand::thread_rng();
        let i_factor = rand::distributions::Distribution::sample(&step, &mut rng);
  
        let step = rand::distributions::Uniform::new( gen_number_range.0, gen_number_range.1 );
        let mut rng = rand::thread_rng();
        let gen_num = rand::distributions::Distribution::sample( &step, &mut rng );

        let case = ParamsCase::new( d_factor, i_factor, gen_num );
  
        let result = self.get_case_results( case.clone() );
        println!("{:?}", result);
        results.push( ( result, case ) );
      }
      let upper_res = results
      .iter()
      .sorted_by( | ( res1, _ ), ( res2, _ ) |  res1.cmp( res2 ) )
      .take( 3 )
      .collect_vec()
      ;

      decrease_factor_range.0 = upper_res
      .iter()
      .map( | ( _, case ) | case.temp_decrease )
      .min_by( | val1, val2 | val1.total_cmp( val2 ) )
      .unwrap()
      ;

      decrease_factor_range.1 = upper_res
      .iter()
      .map( | ( _, case ) | case.temp_decrease )
      .max_by( | val1, val2 | val1.total_cmp( val2 ) )
      .unwrap()
      ;

      increase_factor_range.0 = upper_res
      .iter()
      .map( | ( _, case ) | case.temp_increase )
      .min_by( | val1, val2 | val1.total_cmp( val2 ) )
      .unwrap()
      ;

      increase_factor_range.1 = upper_res
      .iter()
      .map( | ( _, case ) | case.temp_increase )
      .max_by( | val1, val2 | val1.total_cmp( val2 ) )
      .unwrap()
      ;

      gen_number_range.0 = upper_res
      .iter()
      .map( | ( _, case ) | case.gen_number )
      .min()
      .unwrap()
      ;

      gen_number_range.1 = upper_res
      .iter()
      .map( | ( _, case ) | case.gen_number )
      .max()
      .unwrap()
      ;

      res = upper_res.first().unwrap().1.clone();
    }

    res
  }
}

pub fn calculate_difficulty( sudoku : Board ) -> f64
{
  let mut possible_values: Vec<Vec<Vec<usize>>> = vec![ vec![ vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9 ]; 9 ]; 9 ];

  let _clues = sudoku
  .cells()
  .filter( | cell | cell.1 != 0.into() )
  .map( | cell | ( usize::from( cell.1 ), cell.0.row(), cell.0.col()) )
  .for_each( | ( val, row, col ) | 
  {
    for (index, possible_vals ) in possible_values[ row as usize ].iter_mut().enumerate()
    {
      if index == col as usize
      {
        *possible_vals = possible_vals.iter().filter( | &&v | v == val ).map( | v | *v ).collect_vec();
      }
      else 
      {
        if possible_vals.contains( &val )
        {
          *possible_vals = possible_vals.iter().filter( | &&v | v != val ).map( | v | *v ).collect_vec();
        }
      }
    }

    for ( index, possible_vals ) in possible_values.iter_mut().enumerate()
    {
      if index != row as usize
      {
        if possible_vals[ col as usize  ].contains( &val )
        {
          possible_vals[ col as usize  ] = possible_vals[ col as usize  ].iter().filter( | &&v | v != val ).map( | v | *v ).collect_vec();
        }
      }
    }

    let block = BlockIndex::from( crate::sudoku::CellIndex::from( ( col, row ) ) );
    let ( cols, rows ) = block.cells_intervals();
    for i in rows
    {
      for j in cols.clone()
      {
        if !( row as usize == i && col as usize == j )
        {
          if possible_values[ i ][ j ].contains( &val )
          {
            possible_values[ i ][ j ] = possible_values[ i ][ j ].iter().filter( | &&v | v != val ).map( | v | *v ).collect_vec();
          }
        }
      }
    }
  } );

  let mut possibilities_count = HashMap::new();

  for row in &possible_values
  {
    for val in row
    {
      possibilities_count.entry( val.len() ).and_modify( | num | *num += 1 ).or_insert_with( || 1usize );
    }
  }
  let coeff = possibilities_count.into_iter().fold( 0, | acc, val | acc + val.0 * val.1 )  as f64 / 81.0 ;
  coeff
}
