use std::cmp::{min, min_by, max_by, max};
use std::collections::{ HashMap, BTreeMap };
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
    .fold( 0, | acc, elem | acc + elem.as_millis() / size )
    ;
    Duration::from_millis( average as u64 )
  }

  pub fn get_min_points( &self ) -> ( Duration, ParamsCase )
  {
    let mut decrease_factor_range = ( self.lower_bound_case.temp_decrease, self.upper_bound_case.temp_decrease );
    let mut increase_factor_range = ( self.lower_bound_case.temp_increase, self.upper_bound_case.temp_increase );
    let mut gen_number_range = ( self.lower_bound_case.gen_number, self.upper_bound_case.gen_number );
    let mut start_case = ParamsCase {
      temp_decrease: ( decrease_factor_range.0 + decrease_factor_range.1 ) / 2.0,
      temp_increase : ( increase_factor_range.0 + increase_factor_range.1 ) / 2.0,
      gen_number : ( gen_number_range.0 + gen_number_range.1 ) / 2,
    };

    let mut start_duration = self.get_case_results( start_case.clone() );

    let df_step = decrease_factor_range.1 - decrease_factor_range.0 / 5.0;
    let if_step = increase_factor_range.1 - increase_factor_range.0 / 5.0;
    let gn_step = ( gen_number_range.1 - gen_number_range.0 ) / 5;

    decrease_factor_range = ( start_case.temp_decrease - df_step, start_case.temp_decrease + df_step );
    increase_factor_range = ( start_case.temp_increase - if_step, start_case.temp_increase + if_step );
    gen_number_range = ( start_case.gen_number - gn_step, start_case.gen_number + gn_step );

    let mut results = BTreeMap::new();
    for _ in 0..5
    {
      for _ in 0..10
      {
        let mut rng = rand::thread_rng();
        let step = rand::distributions::Uniform::new( decrease_factor_range.0, decrease_factor_range.1 );
        let d_factor = rand::distributions::Distribution::sample(&step, &mut rng);
  
        let step = rand::distributions::Uniform::new( increase_factor_range.0, increase_factor_range.1 );
        let i_factor = rand::distributions::Distribution::sample(&step, &mut rng);
  
        let step = rand::distributions::Uniform::new( gen_number_range.0, gen_number_range.1 );
        let gen_num = rand::distributions::Distribution::sample( &step, &mut rng );

        let case = ParamsCase::new( d_factor, i_factor, gen_num );
  
        let result = self.get_case_results( case.clone() );

        results.insert( result, case );
      }
      for res in &results
      {
        println!("{:?}", res );
      }
      println!("");

      let candidate = results.pop_first().unwrap();
      if candidate.0 < start_duration
      {
        start_case = candidate.1;
        start_duration = candidate.0;
      }

      decrease_factor_range = 
      ( 
        max_by( start_case.temp_decrease - df_step, self.lower_bound_case.temp_decrease, | v1, v2 | v1.total_cmp( v2 ) ), 
        min_by( start_case.temp_decrease + df_step, self.upper_bound_case.temp_increase, | v1, v2 | v1.total_cmp( v2 ) ) 
      );
      increase_factor_range = 
      ( 
        max_by( start_case.temp_increase - if_step, self.lower_bound_case.temp_increase, | v1, v2 | v1.total_cmp( v2 ) ), 
        min_by( start_case.temp_increase + if_step, self.upper_bound_case.temp_increase, | v1, v2 | v1.total_cmp( v2 ) )
      );
      gen_number_range = 
      ( 
        max( start_case.gen_number - gn_step, self.lower_bound_case.gen_number ),
        min( start_case.gen_number + gn_step, self.upper_bound_case.gen_number ) 
      );

    }
    results.pop_first().unwrap().clone()
  }
}

pub fn calculate_difficulty( sudoku : Board ) -> f64
{
  let mut possible_values: Vec< Vec <Vec < usize > > > = vec![ vec![ vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9 ]; 9 ]; 9 ];

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
