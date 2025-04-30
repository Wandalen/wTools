use super::*;
#[ allow( unused_imports ) ]
use component_model::Assign;

#[ derive( Default, PartialEq, Debug ) ]
struct TupleStruct( i32, String );

// Manual implementation for the first field (i32)
impl< IntoT > Assign< i32, IntoT > for TupleStruct
where
  IntoT : Into< i32 >,
{
  fn assign( &mut self, component : IntoT )
  {
    self.0 = component.into(); // Access field by index
  }
}

// Manual implementation for the second field (String)
impl< IntoT > Assign< String, IntoT > for TupleStruct
where
  IntoT : Into< String >,
{
  fn assign( &mut self, component : IntoT )
  {
    self.1 = component.into(); // Access field by index
  }
}

//

// Reuse the same test logic
include!( "./only_test/component_assign_tuple.rs" );