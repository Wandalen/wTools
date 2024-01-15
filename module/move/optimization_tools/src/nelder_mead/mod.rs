//! Implementation of Nelder–Mead method used to find the minimum of an objective function in a multidimensional space.
//!
//! 

use std::ops::{ Bound, Range, RangeBounds };

/// 
#[ derive( Debug, Clone ) ] 
pub struct Point
{
  pub coords : Vec< f64 >,
}

impl Point
{
  pub fn new( coords : Vec< f64 > ) -> Self
  {
    Self { coords }
  }
}

#[ derive( Debug, Clone ) ] 
pub struct Simplex
{
  pub points : Vec< Point >,
}

pub struct NoBounds {}
pub struct NoPoint {}
pub struct NoSimplex {}
pub struct NelderMeadOptimizer< B, P, S >
{
  config : NelderMeadConfig,
  bounds : B,
  starting_point : P,
  initial_simplex : S,
}
impl NelderMeadOptimizer<NoBounds, NoPoint, NoSimplex> 
{
  pub fn new() -> Self
  {
    Self {
      config : NelderMeadConfig::default(),
      bounds : NoBounds{},
      starting_point : NoPoint{},
      initial_simplex : NoSimplex{},
    }
  }

  pub fn new_bounded< R : RangeBounds< f64 > >( bounds : Vec< R > ) -> NelderMeadOptimizer< Vec< R >, NoPoint, NoSimplex >
  {
    NelderMeadOptimizer 
    {
      config : NelderMeadConfig::default(),
      bounds,
      starting_point : NoPoint{},
      initial_simplex : NoSimplex{},
    }
}
}

impl< S, R > NelderMeadOptimizer< Vec< R >, NoPoint, S >
where R : RangeBounds< f64 >
{
  pub fn starting_point( self, p : Point ) -> NelderMeadOptimizer< Vec< R >, Point, S > 
  {
    NelderMeadOptimizer 
    {
      config : self.config,
      bounds : self.bounds,
      starting_point : p,
      initial_simplex : self.initial_simplex,
    }
  }
}
impl< S > NelderMeadOptimizer< Vec< std::ops::RangeInclusive< f64 > >, NoPoint, S >
{
  pub fn random_starting_point( self ) -> NelderMeadOptimizer< Vec< std::ops::RangeInclusive< f64 > >, Point, S > 
  {
    let mut coords = Vec::new();
    let mut rng = rand::thread_rng();
    
    for range in &self.bounds
    {
      let x = rand::Rng::gen_range(&mut rng, range.clone() );
      coords.push( x );
    }

    NelderMeadOptimizer 
    {
      config : self.config,
      bounds : self.bounds,
      starting_point : Point::new( coords ),
      initial_simplex : self.initial_simplex,
    }
  }
}

impl< S > NelderMeadOptimizer< NoBounds, NoPoint, S >
{
  pub fn starting_point( self, p : Point ) -> NelderMeadOptimizer< Vec< Range< f64 > >, Point, S > 
  {
    let mut bounds = Vec::new();
    for _ in 0..p.coords.len()
    {
      bounds.push( f64::NEG_INFINITY..f64::INFINITY );
    }
    NelderMeadOptimizer 
    {
      config : self.config,
      bounds,
      starting_point : p,
      initial_simplex : self.initial_simplex,
    }
  }
}

impl< R > NelderMeadOptimizer< Vec< R >, Point, NoSimplex >
where R : RangeBounds< f64 >
{
  pub fn simplex_size( self, size : Vec< f64 > ) -> NelderMeadOptimizer< Vec< R >, Point, Simplex > 
  {
    let mut points = vec![ self.starting_point.clone() ];
    for i in 0..size.len()
    {
      let mut x = self.starting_point.clone();
      x.coords[ i ] += size[ i ];
      points.push( x );

    }
    NelderMeadOptimizer 
    {
      config : self.config,
      bounds : self.bounds,
      starting_point : self.starting_point,
      initial_simplex : Simplex { points },
    }
  }
}

impl< S > NelderMeadOptimizer< Vec< std::ops::RangeInclusive< f64 > >, NoPoint, S >
{
  pub fn random_simplex_size( self ) -> NelderMeadOptimizer< Vec< std::ops::RangeInclusive< f64 > >, Point, S > 
  {
    let mut coords = Vec::new();
    let mut rng = rand::thread_rng();
    
    for range in &self.bounds
    {
      let x = rand::Rng::gen_range(&mut rng, range.clone() );
      coords.push( x );
    }

    NelderMeadOptimizer {
      config : self.config,
      bounds : self.bounds,
      starting_point : Point::new( coords ),
      initial_simplex : self.initial_simplex,
    }
  }
}

impl< R, P, S > NelderMeadOptimizer< R, P, S >
{
  pub fn set_improvement_threshold( mut self, thr : f64 ) -> Self
  {
    self.config.improvement_threshold = thr;
    self
  }

  pub fn set_max_iterations( mut self, iters : usize ) -> Self
  {
    self.config.max_iterations = iters;
    self
  }
  pub fn set_max_no_improvement_steps( mut self, steps : usize ) -> Self
  {
    self.config.max_no_improvement_steps = steps;
    self
  }
  
  pub fn set_alpha( mut self, a : f64 ) -> Self
  {
    self.config.alpha = a;
    self
  }
  pub fn set_gamma( mut self, g : f64 ) -> Self
  {
    self.config.gamma = g;
    self
  }
  pub fn set_rho( mut self, r : f64 ) -> Self
  {
    self.config.rho = r;
    self
  }
  pub fn set_sigma( mut self, s : f64 ) -> Self
  {
    self.config.sigma = s;
    self
  }
}

impl< R > NelderMeadOptimizer< Vec< R >, Point, Simplex >
where R : RangeBounds< f64 >
{
  fn check_bounds( &self, point : Point ) -> Point
  {
    // check if point left the domain, if so, perform projection:
    // x[ i ] = min or x[ i ] = max
    let mut coords = point.coords;
    for i in 0..coords.len()
    {
      if !self.bounds[ i ].contains( &coords[ i ] )
      {
        match self.bounds[ i ].start_bound()
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
        match self.bounds[ i ].end_bound()
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
    Point::new( coords )
  }
  /// Perform optimization.
  pub fn optimize< F >( &self, f : F ) -> ( Point, f64 )
  where F : Fn( Point ) -> f64
  {
    let x0 = self.starting_point.clone();
    
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
      println!("{:?}", res);

      let best = res.first().clone().unwrap();

      if self.config.max_iterations <= iterations
      {
        return res[ 0 ].clone();
      }

      iterations += 1;

      if best.1 < prev_best - self.config.improvement_threshold
      {
        steps_with_no_improv = 0;
        prev_best = best.1;
      }
      else
      {
        steps_with_no_improv += 1;   
      }

      if steps_with_no_improv >= self.config.max_no_improvement_steps
      {
        return res[ 0 ].clone();
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
        x_ref[ i ] = x0_center[ i ] + self.config.alpha * ( x0_center[ i ] - worst_dir.0.coords[ i ] );
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
          x_exp[ i ] = x0_center[ i ] + self.config.gamma * ( x_ref.coords[ i ] - x0_center[ i ] );
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
        x_con[ i ] = x0_center[ i ] + self.config.rho * ( x0_center[ i ] - worst_dir.0.coords[ i ] );
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
          x_shrink[ i ] = x1.coords[ i ] + self.config.sigma * ( point.coords[ i ] - x1.coords[ i ] );
        }
        let x_shrink = Point::new( x_shrink );
        let score = f( x_shrink.clone() );
        new_res.push( ( x_shrink, score ) );
      }

      res = new_res;
      println!("{:?}", res);
    }
  }
}

#[ derive( Debug, Clone ) ] 
pub struct NelderMeadConfig
{
  /// Threshold used to detect improvement in optimization process.
  /// If difference between current best value and previous best value is less than the threshold, it is considered that no improvement was achieved.
  pub improvement_threshold : f64,
  /// Max number of iteration for optimization process.
  pub max_iterations : usize,
  /// Max number of steps without improvement, stop execution if exceeded.
  pub max_no_improvement_steps : usize,
  /// Coefficient used for calculating reflection point.
  pub alpha : f64,
  /// Coefficient used for calculating expansion point.
  pub gamma : f64,
  /// Coefficient used for calculating contraction point.
  pub rho : f64,
  /// Coefficient used for shrinking simplex.
  pub sigma : f64,
}

impl Default for NelderMeadConfig
{
  fn default() -> Self {
      Self
      {
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
