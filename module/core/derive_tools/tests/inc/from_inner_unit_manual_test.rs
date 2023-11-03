use super::*;

#[ derive( Debug, Clone, Copy, PartialEq ) ]
struct UnitStruct;

impl From< () > for UnitStruct
{
  #[ inline( always ) ]
  fn from( src : () ) -> Self
  {
    Self
  }
}

include!( "./only_test/from_inner_unit.rs" );
