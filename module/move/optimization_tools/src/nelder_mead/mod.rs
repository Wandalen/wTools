//! Implementation of Nelder–Mead method used to find the minimum of an objective function in a multidimensional space.
//! It operates by adjusting a simplex(geometric shape) to explore and converge toward the optimal solution.
//! 

use std::ops::{ Bound, Range, RangeBounds, RangeInclusive };

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
    Self { coords }
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

/// Struct which holds initial configuration for NelderMead optimization, and can perform optimization if all necessary information were provided during initialization process.
#[ derive( Debug, Clone ) ] 
pub struct NelderMeadOptimizer< R >
{
  bounds : Vec< Option< R > >,
  start_point : Point,
  initial_simplex : Simplex,
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
}

impl< R : RangeBounds< f64 > > Default for NelderMeadOptimizer< R >
{
  fn default() -> Self 
  {
    Self
    {
      bounds : Vec::new(),
      start_point : Point::new( vec![ 0.0 ] ),
      initial_simplex : Simplex { points : Vec::new() },
      improvement_threshold : 10e-6,
      max_iterations : 1000,
      max_no_improvement_steps : 10,
      alpha : 1.0,
      gamma : 2.0,
      rho : -0.5,
      sigma : 0.5,
    }
  }
}

impl< R : RangeBounds< f64 > > NelderMeadOptimizer< R >
{
  /// Create new optimizer with default configuration and privided space bounds, and with uninitialized starting point and simplex. 
  pub fn set_bounds( &mut self, bounds : Vec< Option< R > > )
  {
    self.bounds = bounds
  }

  /// Set staring point for optimizer with initialized bounds.
  pub fn set_starting_point( &mut self, p : Point )
  {
    self.start_point = p;
  }

  /// Initialize simplex by providing its size for optimizer with initialized starting point.
  pub fn set_simplex_size( &mut self, size : Vec< f64 > )
  {

    let mut points = vec![ self.start_point.clone() ];
    for i in 0..size.len()
    {
      let mut x = self.start_point.clone();
      x.coords[ i ] += size[ i ];
      points.push( x );

    }

    self.initial_simplex = Simplex { points };

  }

  /// Checks if point is in bounded region.
  pub fn in_bounds( &self, point : &Point ) -> bool
  {
    let coords = &point.coords;
    let mut res = false;
    for i in 0..coords.len()
    {
      if let Some( bound ) = self.bounds[ i ]
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
    for i in 0..coords.len()
    {
      if let Some( bound ) = self.bounds[ i ]
      {
        if !bound.contains( &coords[ i ] )
        {
          match bound.start_bound()
          {
            Bound::Included( val ) => 
            {
              if val < &coords[ i ] 
              {
                coords[ i ] = *val;
              }
            },
            Bound::Excluded( val ) => 
            {
              if val <= &coords[ i ] 
              {
                coords[ i ] = val + f64::EPSILON;
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
                coords[ i ] = *val;
              }
            },
            Bound::Excluded( val ) => 
            {
              if val >= &coords[ i ] 
              {
                coords[ i ] = val - f64::EPSILON;
              }
            },
            Bound::Unbounded => {}
          }
        }
      }
    }
    Point::new( coords )
  }

  /// Optimize provided objective function with using initialized configuration.
  pub fn optimize< F >( &self, f : F ) -> Solution
  where F : Fn( Point ) -> f64
  {
    let x0 = self.start_point.clone();
    
    let dimensions = x0.coords.len();
    let mut prev_best = f( x0.clone() );
    let mut steps_with_no_improv = 0;
    let mut res = vec![ ( x0.clone(), prev_best ) ];

    for i in 1..=dimensions
    {
      let x = self.initial_simplex.points[ i ].clone();
      let score = f( x.clone() );
      res.push( ( x, score ) );
    }

    let mut iterations = 0;
    loop
    {
      res.sort_by( | ( _, a ), ( _, b ) | a.total_cmp( b ) );

      let best = res.first().clone().unwrap();

      if self.max_iterations <= iterations
      {
        return Solution 
        {
          point : res[ 0 ].0.clone(),
          objective : res[ 0 ].1,
          reason : TerminationReason::MaxIterations,
        }
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
        return Solution 
        {
          point : res[ 0 ].0.clone(),
          objective : res[ 0 ].1,
          reason : TerminationReason::NoImprovement,
        }
      }

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

      let reflection_score = f( x_ref.clone() );
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
        let expansion_score = f( x_exp.clone() );

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
      let x_con = Point::new( x_con );
      let contraction_score = f( x_con.clone() );

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
        let x_shrink = Point::new( x_shrink );
        let score = f( x_shrink.clone() );
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
}

/// Reasons for termination of optimization process.
#[ derive( Debug, Clone ) ] 
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
  #[ error( "optimizer must operate on space with at least 1 dimension" ) ]
  ZeroDimError,

  #[ error( "simplex size must have exactly one value for every dimension" ) ]
  SimplexSizeDimError,

  #[error("staring point must have same dimensions as bounded space")]
  PointDimError,

  #[error("starting point is out of bounds")]
  StartPointOutOfBoundsError,
}
