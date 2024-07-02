use super::*;

#[ derive( Debug, Clone, Copy, PartialEq ) ]
struct UnitStruct;

impl UnitStruct
{
  #[ inline( always ) ]
  fn new() -> Self
  {
    Self
  }
}

include!( "./only_test/unit.rs" );
