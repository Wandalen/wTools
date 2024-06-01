use super::*;

#[ derive( Debug, Clone, Copy, PartialEq ) ]
struct UnitStruct;

impl UnitStruct
{
  #[ inline( always ) ]
  fn new( _src : () ) -> Self
  {
    Self
  }
}

include!( "./only_test/unit.rs" );
