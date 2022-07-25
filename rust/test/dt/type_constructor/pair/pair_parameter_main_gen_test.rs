#[ allow( unused_imports ) ]
use super::*;

// trace_macros!( true );
TheModule::types!
{

  pair Pair1 : f64, f32;

  #[ derive( Debug ) ]
  #[ derive( PartialEq, Clone ) ]
  pair Pair2 : f32, f64;

}
// trace_macros!( false );

include!( "./pair_parameter_main_test_only.rs" );
