#[ allow( unused_imports ) ]
use super::*;

the_module::types!
{
  #[ derive( Debug, Clone ) ]
  #[ derive( PartialEq, Default ) ]
  single Single : < T >;
}

include!( "./single_parameter_main_test_only.rs" );
