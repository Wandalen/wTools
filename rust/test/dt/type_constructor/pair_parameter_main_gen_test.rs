#[ allow( unused_imports ) ]
use super::*;

TheModule::types!
{

  pair Pair1 : f64, f32;

  #[ derive( Debug ) ]
  #[ derive( PartialEq, Clone ) ]
  pair Pair2 : f32, f64;

}

include!( "./pair_parameter_main_test_only.rs" );
