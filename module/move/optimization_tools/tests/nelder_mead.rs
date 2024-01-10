use optimization_tools::*;
use nelder_mead::*;

#[ test ]
fn power_two() 
{
  let f = | x : Vec< f64 > | x[ 0 ] * x[ 0 ];
  let optimizer = NelderMeadOptimizer
  {
    f,
    step : 0.1,
    improvement_threshold : 10e-6,
    max_no_improvment_steps : 10,
    x0 : vec![ 3.0, 3.0, 3.0 ],
    max_iterations : 10000,
    alpha : 1.0,
    sigma : 0.5,
    gamma : 2.0,
    rho : -0.5,
  };

  let res = optimizer.optimize();
  assert!( res.1.abs() < 10e-6 );
}

#[ test ]
fn sin_cos() 
{
  let f = | x : Vec< f64 > | x[ 0 ].sin() * x[ 1 ].cos() * ( 1.0 / ( x[ 2 ].abs() + 1.0 ) );
  let optimizer = NelderMeadOptimizer
  {
    f,
    step : 0.1,
    improvement_threshold : 10e-6,
    max_no_improvment_steps : 10,
    max_iterations : 10000,
    x0 : vec![ 0.0, 0.0, 0.0 ],
    alpha : 1.0,
    sigma : 0.5,
    gamma : 2.0,
    rho : -0.5,
  };

  let res = optimizer.optimize();
  assert!( ( -1.5808971014312196 - res.0[ 0 ] ).abs() < f64::EPSILON );
  assert!( ( -0.9999447346002792 - res.1 ).abs() < f64::EPSILON );
}