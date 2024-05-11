//! Implementation of Nelder–Mead method used to find the minimum of an objective function in a multidimensional space.
//! It operates by adjusting a simplex(geometric shape) to explore and converge toward the optimal solution.
//!

use std::
{
  collections::HashMap,
  fs::{ File, OpenOptions },
  ops::{ Bound, RangeBounds },
  sync::{ Arc, Mutex },
};
use deterministic_rand::{ Hrng, Seed, Rng };
use iter_tools::Itertools;
use rayon::iter::{ IntoParallelIterator, ParallelIterator };

use super::results_serialize::save_result;

/// Represents point in multidimensional space where optimization is performed.
#[ derive( Debug, Clone ) ]
pub struct Point
{
  /// Coordinates of the point.
  pub coords : Vec< f64 >,
}

impl Point
{
  /// Create new point from given coordinates.
  pub fn new( coords : Vec< f64 > ) -> Self
  {
    Self { coords : coords.into_iter().map( | elem | elem.into() ).collect_vec() }
  }
}

/// Represents geometric shape formed by a set of n+1 points in a multidimensional space, where n is a number of dimensions.
/// Simplex is used to navigate through solution space, adjusting its shape based on the performance of the objective function at different points.
#[ derive( Debug, Clone ) ]
pub struct Simplex
{
  /// Points of simplex.
  pub points : Vec< Point >,
}

/// Constraints for points of optimization process.
#[ derive( Debug, Clone ) ]
pub enum Constraints
{
  NoConstraints,
  WithConstraints( Vec< fn( &Point ) -> bool > ),
}

impl Constraints
{
  /// Add constraint to constraints list.
  pub fn add_constraint( &mut self, constraint : fn( &Point ) -> bool )
  {
    match self
    {
      Self::NoConstraints => *self = Self::WithConstraints( vec![ constraint ] ),
      Self::WithConstraints( constraints ) => constraints.push( constraint ),
    }
  }
}

#[ derive( Debug, Clone ) ]
pub struct Stats
{
  pub number_of_iterations : usize,
  pub number_of_starting_points : usize,
  pub resumed_after_stale : usize,
  pub starting_point : Point,
  pub differences : Vec< Vec< f64 > >,
  pub positive_change : Vec< usize >,
  pub cached_points : ( usize, usize ),
}

impl Stats
{
  pub fn new( starting_point : Point) -> Self
  {
    let dimensions = starting_point.coords.len();
    Self
    {
      number_of_iterations : 0,
      number_of_starting_points : 1,
      resumed_after_stale : 0,
      starting_point,
      differences : vec![ Vec::new(); dimensions ],
      positive_change : vec![ 0; dimensions ],
      cached_points : ( 0, 0 ),
    }
  }

  pub fn record_diff( &mut self, start_point : &Point, point : &Point )
  {
    for i in 0..start_point.coords.len()
    {
      self.differences[ i ].push( ( start_point.coords[ i ] - point.coords[ i ] ).into() )
    }
  }

  pub fn record_positive_change( &mut self, prev_point : &Point, point : &Point )
  {
    for i in 0..point.coords.len()
    {
      if ( prev_point.coords[ i ] - point.coords[ i ] ).abs() > 0.0
      {
        self.positive_change[ i ] += 1;
      }
    }
  }
}

/// Struct which holds initial configuration for NelderMead optimization, and can perform optimization if all necessary information were provided during initialization process.
#[ derive( Debug, Clone ) ]
pub struct Optimizer< R, F >
{
  /// Bounds for parameters of objective function, may be unbounded or bounded on one side.
  pub bounds : Vec< Option< R > >,
  /// Staring point for optimization process.
  pub start_point : Point,
  /// Initial simplex set in starting point.
  pub initial_simplex : Simplex,
  /// Function to optimize.
  pub objective_function : F,
  /// Threshold used to detect improvement in optimization process.
  /// If difference between current best value and previous best value is less than the threshold, it is considered that no improvement was achieved.
  pub improvement_threshold : f64,
  /// Max number of iteration for optimization process, stop execution if exceeded.
  pub max_iterations : usize,
  /// Max number of steps without improvement, stop execution if exceeded.
  pub max_no_improvement_steps : usize,
  /// Coefficient used for calculating reflection point - point opposite to one with the highest value of objective function.
  /// It is expected that lower values of objective function lie in the opposite direction from point with highest value.
  pub alpha : f64,
  /// Coefficient used for calculating expansion point.
  /// Expansion happents if previously calculated reflection point has the lowest value.
  /// If so, expand simplex in the same direction by calculating expansion point.
  pub gamma : f64,
  /// Coefficient used for calculating contraction point.
  /// Contraction happens when previously calculated reflection point is the worst point in the simplex.
  /// It means that minimum lies within the simplex, so contracting vertices helps to find better values.
  pub rho : f64,
  /// Coefficient used for shrinking simplex.
  /// If previously calculated contraction point doesn't improve the objective function shrinking is performed to adjust simplex size.
  /// Shrinking involves reducing the distance between the vertices of the simplex, making it smaller.
  pub sigma : f64,
  /// Values of objective function calculated in previous executions.
  pub calculated_results : Option< HashMap< super::Point, f64 > >,
  /// File for saving values of objective function during optimization process.
  pub save_results_file : Option< Arc< Mutex< File > > >,
  /// Additional constraint for coordinates of function.
  pub constraints : Constraints,
}

impl< R, F > Optimizer< R, F >
where R : RangeBounds< f64 > + Sync,
  F : Fn( &Point ) -> f64 + Sync,
{
  /// Create new instance of Nelder-Mead optimizer.
  pub fn new( objective_function : F ) -> Self
  {
    Self
    {
      objective_function,
      bounds : Vec::new(),
      start_point : Point::new( Vec::new() ),
      initial_simplex : Simplex { points : Vec::new() },
      improvement_threshold : 10e-6,
      max_iterations : 1000,
      max_no_improvement_steps : 10,
      alpha : 1.0,
      gamma : 2.0,
      rho : -0.5,
      sigma : 0.5,
      calculated_results : None,
      save_results_file : None,
      constraints : Constraints::NoConstraints,
    }
  }

  /// Add set of previosly calculated values of objective function.
  pub fn set_calculated_results( &mut self, res : HashMap< super::Point, f64 > )
  {
    self.calculated_results = Some( res );
  }

  /// Set file for saving results of calculations.
  pub fn set_save_results_file( &mut self, file_path : String )
  {
    let file_res = OpenOptions::new()
    .write( true )
    .append( true )
    .create( true )
    .open( file_path )
    ;

    if let Ok( file ) = file_res
    {
      self.save_results_file = Some( Arc::new( Mutex::new( file ) ) );
    }
  }

 /// Add constraint function.
  pub fn add_constraint( &mut self, constraint : fn( &Point ) -> bool )
  {
    self.constraints.add_constraint( constraint );
  }

  /// Calculate value of objective function at given point or get previously calculated value if such exists.
  pub fn evaluate_point( &self, p : &Point, stats : &mut Stats ) -> f64
  {
    if let Constraints::WithConstraints( constraint_vec ) = &self.constraints
    {
      let valid = constraint_vec.iter().fold( true, | acc, constraint | acc && constraint( p ) );
      if !valid
      {
        return f64::INFINITY;
      }
    }

    if let Some( points ) = &self.calculated_results
    {
      if let Some( value ) = points.get( &p.clone().into() )
      {
        stats.cached_points.0 += 1;
        return *value;
      }
    }
    let result = ( self.objective_function )( p );
    stats.cached_points.1 += 1;

    if let Some( file ) = &self.save_results_file
    {
      _ = save_result
      (
        p.clone().into(),
        result,
        file.clone(),
      );
    }

    result
  }

  /// Set bounds for parameters.
  pub fn set_bounds( &mut self, bounds : Vec< Option< R > > )
  {
    self.bounds = bounds
  }

  /// Set staring point for optimizer.
  pub fn set_starting_point( &mut self, p : Vec< Option< f64 > > )
  {
    self.calculate_start_point();
    for i in 0..p.len()
    {
      if let Some( value ) = p[ i ]
      {
        self.start_point.coords[ i ] = value.into()
      }
    }
  }

  /// Initialize simplex by providing its size for optimizer.
  pub fn set_simplex_size( &mut self, size : Vec< Option< f64 > > )
  {
    if self.start_point.coords.len() == 0
    {
      if self.bounds.len() != 0
      {
        self.calculate_start_point();
      }
      else
      {
        self.start_point.coords = vec![ 0.0; size.len() ];
      }
    }

    self.calculate_regular_simplex();

    for i in 0..size.len()
    {
      if let Some( size ) = size[ i ]
      {
        let mut x = self.start_point.clone();
        x.coords[ i ] += size;
        self.initial_simplex.points[ i + 1 ] = x;
      }
    }
  }

  /// Checks if point is in bounded region.
  pub fn in_bounds( &self, point : &Point ) -> bool
  {
    let coords = &point.coords;
    let mut res = false;
    for i in 0..coords.len()
    {
      if let Some( bound ) = &self.bounds[ i ]
      {
        if bound.contains( &coords[ i ] )
        {
          res = true;
        }
      }
    }
    res
  }

  /// Checks if point left the domain, if so, performs projection: all coordinates that lie out of domain bounds are set to closest coordinate included in bounded space.
  /// Returns projected point.
  fn check_bounds( &self, point : Point ) -> Point
  {
    let mut coords = point.coords;
    for i in 0..self.bounds.len()
    {
      if let Some( bound ) = &self.bounds[ i ]
      {
        if !bound.contains( &coords[ i ] )
        {
          match bound.start_bound()
          {
            Bound::Included( val ) =>
            {
              if val < &coords[ i ]
              {
                coords[ i ] = ( *val ).into();
              }
            },
            Bound::Excluded( val ) =>
            {
              if val <= &coords[ i ]
              {
                coords[ i ] = ( val + f64::EPSILON ).into();
              }
            },
            Bound::Unbounded => {}
          }
          match bound.end_bound()
          {
            Bound::Included( val ) =>
            {
              if val > &coords[ i ]
              {
                coords[ i ] = ( *val ).into();
              }
            },
            Bound::Excluded( val ) =>
            {
              if val >= &coords[ i ]
              {
                coords[ i ] = ( val - f64::EPSILON ).into();
              }
            },
            Bound::Unbounded => {}
          }
        }
      }
    }
    Point::new( coords )
  }

  fn calculate_regular_simplex( &mut self )
  {
    let n = self.start_point.coords.len() as f64;

    let p = ( 1.0 / ( n * 2f64.sqrt() ) ) * ( n - 1.0 + ( n + 1.0 ).sqrt() );
    let q = ( 1.0 / ( n * 2f64.sqrt() ) ) * ( ( n + 1.0 ).sqrt() - 1.0 );

    let mut points = Vec::new();

    points.push( self.start_point.clone() );

    for i in 1..self.start_point.coords.len() + 1
    {
      let mut coords = Vec::new();
      for j in 0..self.start_point.coords.len()
      {
        if j == i - 1
        {
          coords.push( self.start_point.coords[ j ] + p );
        }
        else
        {
          coords.push( self.start_point.coords[ j ] + q );
        }
      }

      points.push( Point::new( coords ) )
    }
    self.initial_simplex = Simplex { points }
  }

  fn calculate_start_point( &mut self )
  {
    let mut new_coords = Vec::new();
    for bound in &self.bounds
    {
      if let Some( bound ) = bound
      {
        if bound.start_bound() != Bound::Unbounded
        {
          let mut start_bound = 0.0;
          if let Bound::Excluded( val ) = bound.start_bound()
          {
            start_bound = *val;
          }
          if let Bound::Included( val ) = bound.start_bound()
          {
            start_bound = *val;
          }
          if bound.end_bound() != Bound::Unbounded
          {
            let mut end_bound = 0.0;
            if let Bound::Excluded( val ) = bound.end_bound()
            {
              end_bound = *val;
            }
            if let Bound::Included( val ) = bound.end_bound()
            {
              end_bound = *val;
            }
            new_coords.push( ( start_bound + end_bound ) / 2.0 )
          }
          else
          {
            new_coords.push( start_bound )
          }
        }
        else
        {
          if bound.end_bound() != Bound::Unbounded
          {
            let mut end_bound = 0.0;
            if let Bound::Excluded( val ) = bound.end_bound()
            {
              end_bound = *val;
            }
            if let Bound::Included( val ) = bound.end_bound()
            {
              end_bound = *val;
            }
            new_coords.push( end_bound )
          }
          else
          {
            new_coords.push( 0.0 )
          }
        }
      }
    }
    self.start_point = Point::new( new_coords );
  }

  /// Optimization starting from several random points.
  pub fn optimize_from_random_points( &mut self ) -> Result< Solution, Error >
  {
    let points_number = self.start_point.coords.len() * 4;
    let mut points = Vec::new();
    let hrng = Hrng::master_with_seed( Seed::default() );
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();

    for _ in 0..points_number
    {
      let mut point = Vec::new();

      for bound in &self.bounds
      {
        if let Some( bound ) = bound
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
          point.push( x );
        }
      }

      points.push( Point::new( point ) );
    }

    let results = points.into_par_iter().map( | point |
    {
      let mut stats = Stats::new( point.clone() );
      stats.number_of_starting_points = points_number;
      let x0 = point.clone();
      let dimensions = x0.coords.len();
      let mut prev_best = self.evaluate_point( &x0, &mut stats );
      let mut steps_with_no_improv = 0;
      let mut res = vec![ ( x0.clone(), prev_best ) ];

      for i in 1..=dimensions
      {
        let x = self.initial_simplex.points[ i ].clone();
        let score = self.evaluate_point( &x, &mut stats );
        res.push( ( x, score ) );
      }
      let mut iterations = 0;
      loop
      {
        res.sort_by( | ( _, a ), ( _, b ) | a.total_cmp( b ) );

        let best = res.first().clone().unwrap();

        if self.max_iterations <= iterations
        {
          stats.number_of_iterations = iterations;
          return Result::< Solution, Error >::Ok ( Solution 
          {
            point : res[ 0 ].0.clone(),
            objective : res[ 0 ].1,
            reason : TerminationReason::MaxIterations,
            stats : Some( stats ),
          } )
        }
  
        if best.1 < prev_best - self.improvement_threshold
        {
          if steps_with_no_improv > 0
          {
            stats.resumed_after_stale += 1;
          }
          steps_with_no_improv = 0;
          prev_best = best.1;
        }
        else
        {
          steps_with_no_improv += 1;
        }

        if steps_with_no_improv >= self.max_no_improvement_steps
        {
          stats.number_of_iterations = iterations;
          return Ok ( Solution 
          {
            point : res[ 0 ].0.clone(),
            objective : res[ 0 ].1,
            reason : TerminationReason::NoImprovement,
            stats : Some( stats ),
          } )
        }

        iterations += 1;
  
        //centroid
        let mut x0_center = vec![ 0.0; dimensions ];
        for ( point, _ ) in res.iter().take( res.len() - 1 )
        {
          for ( i, coordinate ) in point.coords.iter().enumerate()
          {
            x0_center[ i ] += coordinate / ( res.len() - 1 ) as f64;
          }
        }

        //reflection
        let worst_dir = res.last().clone().unwrap();
        let mut x_ref = vec![ 0.0; dimensions ];
        for i in 0..dimensions
        {
          x_ref[ i ] = x0_center[ i ] + self.alpha * ( x0_center[ i ] - worst_dir.0.coords[ i ] );
        }
        // check if point left the domain, if so, perform projection
        let x_ref = self.check_bounds( Point::new( x_ref ) );
        stats.record_diff( &self.start_point, &x_ref );

        let reflection_score = self.evaluate_point( &x_ref, &mut stats );
        let second_worst = res[ res.len() - 2 ].1;
        if res[ 0 ].clone().1 <= reflection_score && reflection_score < second_worst
        {
          let prev_point = res.pop().unwrap().0;
          stats.record_positive_change( &prev_point, &x_ref );
          res.push( ( x_ref, reflection_score ) );
          continue;
        }

        //expansion
        if reflection_score < res[ 0 ].1
        {
          let mut x_exp = vec![ 0.0; dimensions ];
          for i in 0..dimensions
          {
            x_exp[ i ] = x0_center[ i ] + self.gamma * ( x_ref.coords[ i ] - x0_center[ i ] );
          }
          // check if point left the domain, if so, perform projection
          let x_exp = self.check_bounds( Point::new( x_exp ) );
          stats.record_diff( &self.start_point, &x_exp );
          let expansion_score = self.evaluate_point( &x_exp, &mut stats );

          if expansion_score < reflection_score
          {
            let prev_point = res.pop().unwrap().0;
            stats.record_positive_change( &prev_point, &x_exp );
            res.push( ( x_exp, expansion_score ) );
            continue;

          }
          else
          {
            let prev_point = res.pop().unwrap().0;
            stats.record_positive_change( &prev_point, &x_ref );
            res.push( ( x_ref, reflection_score ) );
            continue;
          }
        }

        //contraction
        let mut x_con = vec![ 0.0; dimensions ];
        for i in 0..dimensions
        {
          x_con[ i ] = x0_center[ i ] + self.rho * ( x0_center[ i ] - worst_dir.0.coords[ i ] );
        }
        let x_con = self.check_bounds( Point::new( x_con ) );
        stats.record_diff( &self.start_point, &x_con );
        let contraction_score = self.evaluate_point( &x_con, &mut stats );

        if contraction_score < worst_dir.1
        {
          let prev_point = res.pop().unwrap().0;
          stats.record_positive_change( &prev_point, &x_con );
          res.push( ( x_con, contraction_score ) );
          continue;
        }

        //shrink
        let x1 = res[ 0 ].clone().0;
        let mut new_res = Vec::new();
        for ( point, _ ) in res
        {
          let mut x_shrink = vec![ 0.0; dimensions ];
          for i in 0..dimensions
          {
            x_shrink[ i ] = x1.coords[ i ] + self.sigma * ( point.coords[ i ] - x1.coords[ i ] );
          }
          let x_shrink = self.check_bounds( Point::new( x_shrink ) );
          stats.record_diff( &self.start_point, &x_shrink );
          let score = self.evaluate_point( &x_shrink, &mut stats );
          new_res.push( ( x_shrink, score ) );
        }
        res = new_res;
      }
    } ).collect::< Vec< _ > >();

    let results = results.into_iter().flatten().collect_vec();
    let res = results.into_iter().min_by( | res1, res2 | res1.objective.total_cmp( &res2.objective ) ).unwrap();
    Ok( res )
  }

  /// Optimize provided objective function with using initialized configuration.
  pub fn optimize( &mut self ) -> Result< Solution, Error >
  {
    let mut stats = Stats::new(  self.start_point.clone() );
    if self.start_point.coords.len() == 0
    {
      self.calculate_start_point();
    }

    if self.start_point.coords.len() == 0
    {
      return Err ( Error::StartPointError );
    }

    if self.initial_simplex.points.len() == 0
    {
      self.calculate_regular_simplex();
    }

    let x0 = self.start_point.clone();

    let dimensions = x0.coords.len();
    let mut prev_best = self.evaluate_point( &x0, &mut stats );
    let mut steps_with_no_improv = 0;
    let mut res = vec![ ( x0.clone(), prev_best ) ];

    for i in 1..=dimensions
    {
      let x = self.initial_simplex.points[ i ].clone();
      let score = self.evaluate_point( &x, &mut stats );
      res.push( ( x, score ) );
    }
    let mut iterations = 0;
    loop
    {
      res.sort_by( | ( _, a ), ( _, b ) | a.total_cmp( b ) );

      let best = res.first().clone().unwrap();

      if self.max_iterations <= iterations
      {
        return Ok ( Solution
        {
          point : res[ 0 ].0.clone(),
          objective : res[ 0 ].1,
          reason : TerminationReason::MaxIterations,
          stats : None,
        } )
      }

      iterations += 1;

      if best.1 < prev_best - self.improvement_threshold
      {
        steps_with_no_improv = 0;
        prev_best = best.1;
      }
      else
      {
        steps_with_no_improv += 1;
      }

      if steps_with_no_improv >= self.max_no_improvement_steps
      {
        return Ok ( Solution
        {
          point : res[ 0 ].0.clone(),
          objective : res[ 0 ].1,
          reason : TerminationReason::NoImprovement,
          stats : None,
        } )
      }

      //centroid
      let mut x0_center = vec![ 0.0; dimensions ];
      for ( point, _ ) in res.iter().take( res.len() - 1 )
      {
        for ( i, coordinate ) in point.coords.iter().enumerate()
        {
          x0_center[ i ] += coordinate / ( ( res.len() - 1 ) as f64 );
        }
      }

      //reflection
      let worst_dir = res.last().clone().unwrap();
      let mut x_ref = vec![ 0.0; dimensions ];
      for i in 0..dimensions
      {
        x_ref[ i ] = x0_center[ i ] + self.alpha * ( x0_center[ i ] - worst_dir.0.coords[ i ] );
      }
      // check if point left the domain, if so, perform projection
      let x_ref = self.check_bounds( Point::new( x_ref ) );

      let reflection_score = self.evaluate_point( &x_ref, &mut stats );
      let second_worst = res[ res.len() - 2 ].1;
      if res[ 0 ].clone().1 <= reflection_score && reflection_score < second_worst
      {
        res.pop();
        res.push( ( x_ref, reflection_score ) );
        continue;
      }

      //expansion
      if reflection_score < res[ 0 ].1
      {
        let mut x_exp = vec![ 0.0; dimensions ];
        for i in 0..dimensions
        {
          x_exp[ i ] = x0_center[ i ] + self.gamma * ( x_ref.coords[ i ] - x0_center[ i ] );
        }
        // check if point left the domain, if so, perform projection
        let x_exp = self.check_bounds( Point::new( x_exp ) );
        let expansion_score = self.evaluate_point( &x_exp, &mut stats );

        if expansion_score < reflection_score
        {
          res.pop();
          res.push( ( x_exp, expansion_score ) );
          continue;
        }
        else
        {
          res.pop();
          res.push( ( x_ref, reflection_score ) );
          continue;
        }
      }

      //contraction
      let mut x_con = vec![ 0.0; dimensions ];
      for i in 0..dimensions
      {
        x_con[ i ] = x0_center[ i ] + self.rho * ( x0_center[ i ] - worst_dir.0.coords[ i ] );
      }
      let x_con = self.check_bounds( Point::new( x_con ) );
      let contraction_score = self.evaluate_point( &x_con, &mut stats );

      if contraction_score < worst_dir.1
      {
        res.pop();
        res.push( ( x_con, contraction_score ) );
        continue;
      }

      //shrink
      let x1 = res[ 0 ].clone().0;
      let mut new_res = Vec::new();
      for ( point, _ ) in res
      {
        let mut x_shrink = vec![ 0.0; dimensions ];
        for i in 0..dimensions
        {
          x_shrink[ i ] = x1.coords[ i ] + self.sigma * ( point.coords[ i ] - x1.coords[ i ] );
        }
        let x_shrink = self.check_bounds( Point::new( x_shrink ) );
        let score = self.evaluate_point( &x_shrink, &mut stats );
        new_res.push( ( x_shrink, score ) );
      }

      res = new_res;
    }
  }
}

/// Result of optimization process.
#[ derive( Debug, Clone ) ]
pub struct Solution
{
  /// Point in which objective function had the lowest value at the moment of termination.
  pub point : Point,
  /// Lowest value of objective function found during optimization.
  pub objective : f64,
  /// Reason for termination.
  pub reason : TerminationReason,
  /// Staticstics.
  pub stats : Option< Stats >,
}

/// Reasons for termination of optimization process.
#[ derive( Debug, Clone, derive_tools::Display ) ] 
pub enum TerminationReason
{
  /// Reached limit of total iterations.
  MaxIterations,
  /// Reached limit of iterations without improvement in objective function values.
  NoImprovement,
}

/// Possible error when building NMOptimizer.
#[ derive( thiserror::Error, Debug ) ]
pub enum Error {
  /// Error for Simplex size that have less dimessions than starting point.
  #[ error( "simplex size must have exactly one value for every dimension" ) ]
  SimplexSizeDimError,

  /// Error if calculation of starting point failed.
  #[error("cannot calculate starting point, no bounds provided")]
  StartPointError,

  /// Error for given starting point that lies out of provided bounds.
  #[error("starting point is out of bounds")]
  StartPointOutOfBoundsError,
}
