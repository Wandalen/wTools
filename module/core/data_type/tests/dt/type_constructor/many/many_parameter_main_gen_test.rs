#[ allow( unused_imports ) ]
use super::*;

// trace_macros!( true );
TheModule::types!
{
  #[ derive( Debug, Clone ) ]
  #[ derive( PartialEq, Default ) ]
  many Many : < T >;
}
// trace_macros!( false );

include!( "./many_parameter_main_test_only.rs" );
