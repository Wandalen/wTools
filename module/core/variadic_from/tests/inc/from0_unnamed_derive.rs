#[ allow( unused_imports ) ]
use super::*;
use the_module::exposed::*;

#[ derive( Debug, PartialEq, Default, VariadicFrom ) ]
struct Struct1();

impl From< () > for Struct1
{
  fn from( _a : () ) -> Self { Self::default() }
}

include!( "./only_test/from0.rs" );
