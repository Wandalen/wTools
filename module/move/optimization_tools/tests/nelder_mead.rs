use std::ops::Range;

use optimization_tools::*;
use nelder_mead::*;

#[ test ]
fn power_two() -> Result< (), nelder_mead::Error >
{
  let f = | x : Point | x.coords[ 0 ] * x.coords[ 0 ];
  let mut optimizer = NelderMeadOptimizer::default();
  optimizer.bounds = vec![ Some( -1.0..=8.0 ), Some( 2.0..=4.0 ), Some( 3.0..=6.0 ) ];
  optimizer.start_point = Point::new( vec![ 3.0, 3.0, 3.0 ] );
  optimizer.set_simplex_size( vec![ 0.1, 0.1, 0.1 ] );

  let res = optimizer.optimize( f )?;
  assert!( res.objective.abs() < 10e-6 );

  Ok( () )
}

#[ test ]
fn sin_cos() -> Result< (), nelder_mead::Error >
{
  let f = | x : Point | x.coords[ 0 ].sin() * x.coords[ 1 ].cos() * ( 1.0 / ( x.coords[ 2 ].abs() + 1.0 ) );
  let mut optimizer: NelderMeadOptimizer<Range< f64 >> = NelderMeadOptimizer::default();
  optimizer.start_point = Point::new( vec![ 0.0, 0.0, 0.0 ] );
  optimizer.set_simplex_size( vec![ 0.1, 0.1, 0.1 ] );

  let res = optimizer.optimize( f )?;
  assert!( ( -1.5808971014312196 - res.point.coords[ 0 ] ).abs() < 10e-5 );
  assert!( ( -1.0 - res.objective ).abs() <= 10e-5 );

  Ok( () )
}

#[ test ]
fn rosenbrock() -> Result< (), nelder_mead::Error >
{
  let f = | x : Point | ( 1.0 - x.coords[ 0 ] ).powi( 2 ) + 100.0 * ( x.coords[ 1 ] - x.coords[ 0 ].powi( 2 )).powi( 2 ) ;
  let mut optimizer: NelderMeadOptimizer< Range< f64 > > = NelderMeadOptimizer::default();
  optimizer.start_point = Point::new( vec![ 0.0, 0.0 ] );
  optimizer.set_simplex_size( vec![ 0.1, 0.1 ] );

  let res = optimizer.optimize( f )?;
  assert!( ( 1.0 - res.point.coords[ 0 ] ).abs() < 10e-5 );
  assert!( ( 1.0 - res.point.coords[ 1 ] ).abs() < 10e-5 );
  assert!( res.objective < 10e-5 );

  Ok( () )
}

#[ test ]
fn himmelblau() -> Result< (), nelder_mead::Error >
{
  let f = | x : Point | ( x.coords[ 0 ].powi( 2 ) + x.coords[ 1 ] -11.0 ).powi( 2 ) + ( x.coords[ 0 ] + x.coords[ 1 ].powi( 2 ) - 7.0 ).powi( 2 ) ;
  let mut optimizer: NelderMeadOptimizer< Range< f64 > > = NelderMeadOptimizer::default();
  optimizer.start_point = Point::new( vec![ 0.0, 0.0 ] );
  optimizer.set_simplex_size( vec![ 0.1, 0.1 ] );
  optimizer.max_no_improvement_steps = 15;

  let res = optimizer.optimize( f )?;
  let mut is_one_of_minima_points = false;

  for minima in [ ( 3.0, 2.0 ), ( -2.805118, 3.131312 ), ( -3.779310, -3.283186 ), ( 3.584428, -1.848126 ) ]
  {
    if ( ( minima.0 - res.point.coords[ 0 ] ).abs() < 10e-5 ) && ( ( minima.1 - res.point.coords[ 1 ] ).abs() < 10e-5 )
    {
        is_one_of_minima_points = true;
    }
  }
  assert!( is_one_of_minima_points );
  assert!( res.objective < 10e-5 );

  Ok( () )
}