//! Optimal parameters search using Simulated Annealing.

use std::ops::{ Bound, RangeBounds };

use deterministic_rand::{ Hrng, Seed, seq::IteratorRandom, Rng };
use rayon::iter::{ IndexedParallelIterator, ParallelIterator };
use super::nelder_mead::{ self, Point, Solution, TerminationReason };

/// Optimizer for optimal parameters search using Simmulated Annealing.
#[ derive( Debug, Clone ) ] 
pub struct Optimizer< R, F >
{
  /// Bounds for parameters of objective function.
  pub bounds : Vec< R >,

  /// Oblective function to optimize.
  pub objective_function : F,

  /// Iterations limit, execution stops when exceeded.
  pub max_iterations : usize,
}

impl< R : RangeBounds< f64 > + Sync, F : Fn( nelder_mead::Point ) -> f64 + Sync > Optimizer< R, F >
{
  /// Calculate the initial temperature for the optimization process.
  pub fn initial_temperature( &self ) -> f64
  {
    use statrs::statistics::Statistics;
    let hrng = Hrng::master_with_seed( Seed::default() );
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();

    let mut starting_point = Vec::new();

    for bound in &self.bounds
    {
      let start = match bound.start_bound() 
      {
        Bound::Included( start ) => *start,
        Bound::Excluded( start ) => *start + f64::EPSILON,
        Bound::Unbounded => unreachable!(),
      };
      let end = match bound.end_bound() {
        Bound::Included( end ) => *end + f64::EPSILON,
        Bound::Excluded( end ) => *end,
        Bound::Unbounded => unreachable!(),
      };
      
      let x = rng.gen_range( start..end );
      starting_point.push( x );
    }

    const N : usize = 10;
    let mut costs : [ f64 ; N ] = [ 0.0 ; N ];
    for i in 0..N
    {
      let mut candidate = starting_point.clone();
      let position = rng.gen_range( 0..candidate.len() );
      let bound = &self.bounds[ position ];
      
        let start = match bound.start_bound() 
        {
          Bound::Included( start ) => *start,
          Bound::Excluded( start ) => *start + f64::EPSILON,
          Bound::Unbounded => unreachable!(),
        };
        let end = match bound.end_bound() {
          Bound::Included( end ) => *end + f64::EPSILON,
          Bound::Excluded( end ) => *end,
          Bound::Unbounded => unreachable!(),
        };
        
        let x = rng.gen_range( start..end );
        candidate[ position ] = x;
        costs[ i ] = ( self.objective_function )( Point::new( candidate ) );
    }
    costs[..].std_dev().into()
  }

  /// Find optimal solution for objective function using Simulated Annealing. 
  pub fn optimize( &self ) -> Result< Solution, nelder_mead::Error >
  {
    let hrng = Hrng::master_with_seed( Seed::default() );
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();

    let mut starting_point = Vec::new();

    for bound in &self.bounds
    {
      let start = match bound.start_bound() 
      {
        Bound::Included( start ) => *start,
        Bound::Excluded( start ) => *start + f64::EPSILON,
        Bound::Unbounded => unreachable!(),
      };
      let end = match bound.end_bound() {
        Bound::Included( end ) => *end + f64::EPSILON,
        Bound::Excluded( end ) => *end,
        Bound::Unbounded => unreachable!(),
      };
      
      let x = rng.gen_range( start..end );
      starting_point.push( x );
    }

    let mut iterations = 0;
    let mut expected_number_of_candidates = 4;
    let mut point = starting_point.clone();
    let mut value = ( self.objective_function )( Point::new( starting_point ) );
    drop( rng );

    let mut best_found = ( point.clone(), value.clone() );
    let mut temperature = self.initial_temperature();

    loop
    {
      if iterations > self.max_iterations
      {
        break;
      }
      
      let solutions = rayon::iter::repeat( () )
      .take( expected_number_of_candidates )
      .enumerate()
      .map( | ( i, _ ) | hrng.child( i ) )
      .flat_map( | hrng | 
        {
          let rng_ref = hrng.rng_ref();
          let mut rng = rng_ref.lock().unwrap();
          let mut candidate = point.clone();
          let position = rng.gen_range( 0..candidate.len() );
          let bound = &self.bounds[ position ];
          
            let start = match bound.start_bound() 
            {
              Bound::Included( start ) => *start,
              Bound::Excluded( start ) => *start + f64::EPSILON,
              Bound::Unbounded => unreachable!(),
            };
            let end = match bound.end_bound() {
              Bound::Included( end ) => *end + f64::EPSILON,
              Bound::Excluded( end ) => *end,
              Bound::Unbounded => unreachable!(),
            };
            
            let x = rng.gen_range( start..end );
            candidate[ position ] = x;
            
          let candidate_value = ( self.objective_function )( Point::new( candidate.clone() ) );
        
          let difference = candidate_value - value;
          let threshold = ( - difference / temperature ).exp();
          let rand : f64 = rng.gen();
          let vital = rand < threshold;  
          if vital
          {
            Some( ( candidate, candidate_value ) )
          }
          else
          {
            None
          }
                
        } )
      .collect::< Vec< _ > >()
      ;

      if solutions.len() > 0
      {
        let rng_ref = hrng.rng_ref();
        let mut rng = rng_ref.lock().unwrap();
        
        if let Some( index ) = ( 0..solutions.len() - 1 ).choose( &mut *rng )
        {
          point = solutions[ index ].0.clone();
          value = solutions[ index ].1;
        }
        else 
        {
          point =  solutions[ 0 ].0.clone();
          value = solutions[ 0 ].1;
        }  
        if value < best_found.1
        {
          best_found = ( point.clone(), value );
        }
      }
      else 
      {
        if expected_number_of_candidates < 32
        {
          expected_number_of_candidates += 4;
        }
      }

      temperature *= 0.999;
      iterations += 1;
    }

    Ok ( Solution
    {
      point : Point::new( best_found.0.clone() ),
      objective : best_found.1,
      reason : TerminationReason::MaxIterations,
      stats : None,
    } )
  }
}
