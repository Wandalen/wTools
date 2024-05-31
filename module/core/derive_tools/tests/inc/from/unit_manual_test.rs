use super::*;

#[ derive( Debug, Clone, Copy, PartialEq ) ]
struct UnitStruct;

impl From< () > for UnitStruct
{
  #[ inline( always ) ]
  fn from( _src : () ) -> Self
  {
    Self
  }
}

include!( "./only_test/unit.rs" );
