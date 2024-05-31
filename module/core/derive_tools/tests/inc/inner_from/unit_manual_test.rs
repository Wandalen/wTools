use super::*;

#[ derive( Debug, Clone, Copy, PartialEq ) ]
pub struct UnitStruct;

impl From< UnitStruct > for ()
{
  #[ inline( always ) ]
  fn from( _src : UnitStruct ) -> Self
  {
    ()
  }
}

// include!( "./manual/basic.rs" );
include!( "./only_test/unit.rs" );
