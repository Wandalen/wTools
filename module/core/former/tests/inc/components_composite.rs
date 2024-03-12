#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use former::{ SetComponent, SetWithType };

///
/// Options1
///

#[ derive( Debug, Default, PartialEq, TheModule::ComponentFrom, TheModule::SetComponent ) ]
// qqq : make these traits working for generic struct, use `split_for_impl`
pub struct Options1
{
  field1 : i32,
  field2 : String,
  field3 : f32,
}

///
/// Options2
///

#[ derive( Debug, Default, PartialEq, TheModule::ComponentFrom, TheModule::SetComponent ) ]
pub struct Options2
{
  field1 : i32,
  field2 : String,
}

///
/// Options2SetComponents.
///

pub trait Options2SetComponents< IntoT >
where
  IntoT : Into< i32 >,
  IntoT : Into< String >,
  IntoT : Clone,
{
  fn components_set( &mut self, component : IntoT );
}

impl< T, IntoT > Options2SetComponents< IntoT > for T
where
  T : former::SetComponent< i32, IntoT >,
  T : former::SetComponent< String, IntoT >,
  IntoT : Into< i32 >,
  IntoT : Into< String >,
  IntoT : Clone,
{
  #[ inline( always ) ]
  fn components_set( &mut self, component : IntoT )
  {
    former::SetComponent::< i32, _ >::set( self, component.clone() );
    former::SetComponent::< String, _ >::set( self, component.clone() );
  }
}

impl< T > From< T > for Options2
where
  T : Into< i32 >,
  T : Into< String >,
  T : Clone,
{
  #[ inline( always ) ]
  fn from( src : T ) -> Self
  {
    let field1 = Into::< i32 >::into( src.clone() );
    let field2 = Into::< String >::into( src.clone() );
    Options2
    {
      field1,
      field2,
    }
  }
}

//

include!( "only_test/components_composite.rs" );
