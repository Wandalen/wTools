// module/core/component_model/tests/inc/components_tests/components_assign_tuple_manual.rs
use super::*;
#[ allow( unused_imports ) ]
use component_model::{Assign, AssignWithType};

// Define TupleStruct1 without derive
#[ derive( Debug, Default, PartialEq ) ]
struct TupleStruct1(i32, String, f32);

// Define TupleStruct2 without derive
#[ derive( Debug, Default, PartialEq ) ]
struct TupleStruct2(i32, String);

// Manual Assign impls for TupleStruct1
impl< IntoT > Assign< i32, IntoT > for TupleStruct1
where
  IntoT : Into< i32 >,
{
  fn assign( &mut self, component : IntoT ) {
    self.0 = component.into();
  }
}

impl< IntoT > Assign< String, IntoT > for TupleStruct1
where
  IntoT : Into< String >,
{
  fn assign( &mut self, component : IntoT ) {
    self.1 = component.into();
  }
}

impl< IntoT > Assign< f32, IntoT > for TupleStruct1
where
  IntoT : Into< f32 >,
{
  fn assign( &mut self, component : IntoT ) {
    self.2 = component.into();
  }
}

// Manual Assign impls for TupleStruct2
impl< IntoT > Assign< i32, IntoT > for TupleStruct2
where
  IntoT : Into< i32 >,
{
  fn assign( &mut self, component : IntoT ) {
    self.0 = component.into();
  }
}

impl< IntoT > Assign< String, IntoT > for TupleStruct2
where
  IntoT : Into< String >,
{
  fn assign( &mut self, component : IntoT ) {
    self.1 = component.into();
  }
}

// Implement From<&TupleStruct1> for the types present in TupleStruct2
impl From< &TupleStruct1 > for i32 {
  #[ inline( always ) ]
  fn from( src : &TupleStruct1 ) -> Self {
    src.0
  }
}

impl From< &TupleStruct1 > for String {
  #[ inline( always ) ]
  fn from( src : &TupleStruct1 ) -> Self {
    src.1.clone()
  }
}

// Manually define the ComponentsAssign trait and impl for TupleStruct2
pub trait TupleStruct2ComponentsAssign< IntoT >
where
  IntoT : Into< i32 >,
  IntoT : Into< String >,
  IntoT : Clone,
{
  fn tuple_struct_2_assign( &mut self, component : IntoT );
}

impl< T, IntoT > TupleStruct2ComponentsAssign< IntoT > for T
where
  T : component_model::Assign< i32, IntoT >,
  T : component_model::Assign< String, IntoT >,
  IntoT : Into< i32 >,
  IntoT : Into< String >,
  IntoT : Clone,
{
  #[ inline( always ) ]
  fn tuple_struct_2_assign( &mut self, component : IntoT ) {
    component_model::Assign::< i32, _ >::assign( self, component.clone() );
    component_model::Assign::< String, _ >::assign( self, component.clone() );
  }
}

// Re-include the test logic
include!("./only_test/components_assign_tuple.rs");
