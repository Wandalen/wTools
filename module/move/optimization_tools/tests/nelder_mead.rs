use optimization_tools::*;
use nelder_mead::*;

#[ test ]
fn power_two() 
{
  let f = | x : Vec< f64 > | x[ 0 ] * x[ 0 ];
  let optimizer = NelderMeadOptimizer::default();

  let res = optimizer.optimize( f, vec![ 3.0, 3.0, 3.0 ], vec![ 0.1, 0.1, 0.1 ] );
  assert!( res.1.abs() < 10e-6 );
}

#[ test ]
fn sin_cos() 
{
  let f = | x : Vec< f64 > | x[ 0 ].sin() * x[ 1 ].cos() * ( 1.0 / ( x[ 2 ].abs() + 1.0 ) );
  let optimizer = NelderMeadOptimizer::default();

  let res = optimizer.optimize( f, vec![ 0.0, 0.0, 0.0 ], vec![ 0.1, 0.1, 0.1 ] );
  assert!( ( -1.5808971014312196 - res.0[ 0 ] ).abs() < f64::EPSILON );
  assert!( ( -1.0 - res.1 ).abs() <= 10e-5 );
}

#[ test ]
fn rosenbrock() 
{
  let f = | x : Vec< f64 > | ( 1.0 - x[ 0 ] ).powi( 2 ) + 100.0 * ( x[ 1 ] - x[ 0 ].powi( 2 )).powi( 2 ) ;
  let optimizer = NelderMeadOptimizer::default();

  let res = optimizer.optimize( f, vec![ 0.0, 0.0 ], vec![ 0.1, 0.1 ] );
  assert!( ( 1.0 - res.0[ 0 ] ).abs() < 10e-5 );
  assert!( ( 1.0 - res.0[ 1 ] ).abs() < 10e-5 );
  assert!( res.1 < 10e-5 );
}

#[ test ]
fn himmelblau() 
{
  let f = | x : Vec< f64 > | ( x[ 0 ].powi( 2 ) + x[ 1 ] -11.0 ).powi( 2 ) + ( x[ 0 ] + x[ 1 ].powi( 2 ) - 7.0 ).powi( 2 ) ;
  let mut optimizer = NelderMeadOptimizer::default();
  optimizer.max_no_improvement_steps = 15;

  let res = optimizer.optimize( f, vec![ 0.0, 0.0 ], vec![ 0.1, 0.1 ] );
  let mut is_one_of_minima_points = false;

  for minima in [ ( 3.0, 2.0 ), ( -2.805118, 3.131312 ), ( -3.779310, -3.283186 ), ( 3.584428, -1.848126 ) ]
  {
    if ( ( minima.0 - res.0[ 0 ] ).abs() < 10e-5 ) && ( ( minima.1 - res.0[ 1 ] ).abs() < 10e-5 )
    {
        is_one_of_minima_points = true;
    }
  }
  assert!( is_one_of_minima_points );
  assert!( res.1 < 10e-5 );
}