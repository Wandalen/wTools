#[ allow( unused_imports ) ]
use super::*;
use the_module::exposed::*;

// xxx
// #[ derive( Debug, PartialEq, Default, VariadicFrom ) ]
#[ derive( Debug, PartialEq, Default ) ]
struct Struct1;

impl From1< () > for Struct1
{
  fn from1( _a : () ) -> Self { Self::default() }
}

impl From< () > for Struct1
{
  fn from( _a : () ) -> Self { Self::default() }
}

include!( "./only_test/from0.rs" );
