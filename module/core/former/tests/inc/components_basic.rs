
///
/// Set component trait.
///

pub trait SetComponent< T, IntoT >
where
  IntoT : Into< T >,
{
  fn set( &mut self, component : IntoT );
}

///
/// Options1
///

#[ derive( Debug, Default, PartialEq ) ]
pub struct Options1
{
  field1 : i32,
  field2 : String,
  field3 : f32,
}

impl From< &Options1 > for i32
{
  #[ inline( always ) ]
  fn from( src : &Options1 ) -> Self
  {
    src.field1.clone()
  }
}

impl From< &Options1 > for String
{
  #[ inline( always ) ]
  fn from( src : &Options1 ) -> Self
  {
    src.field2.clone()
  }
}

impl From< &Options1 > for f32
{
  #[ inline( always ) ]
  fn from( src : &Options1 ) -> Self
  {
    src.field3.clone()
  }
}

impl< IntoT > SetComponent< i32, IntoT > for Options1
where
  IntoT : Into< i32 >,
{
  #[ inline( always ) ]
  fn set( &mut self, component : IntoT )
  {
    self.field1 = component.into().clone();
  }
}

impl< IntoT > SetComponent< String, IntoT > for Options1
where
  IntoT : Into< String >,
{
  #[ inline( always ) ]
  fn set( &mut self, component : IntoT )
  {
    self.field2 = component.into().clone();
  }
}

impl< IntoT > SetComponent< f32, IntoT > for Options1
where
  IntoT : Into< f32 >,
{
  #[ inline( always ) ]
  fn set( &mut self, component : IntoT )
  {
    self.field3 = component.into().clone();
  }
}

///
/// Options2
///

#[ derive( Debug, Default, PartialEq ) ]
pub struct Options2
{
  field1 : i32,
  field2 : String,
}

impl From< &Options2 > for i32
{
  #[ inline( always ) ]
  fn from( src : &Options2 ) -> Self
  {
    src.field1.clone()
  }
}

impl From< &Options2 > for String
{
  #[ inline( always ) ]
  fn from( src : &Options2 ) -> Self
  {
    src.field2.clone()
  }
}

impl< IntoT > SetComponent< i32, IntoT > for Options2
where
  IntoT : Into< i32 >,
{
  #[ inline( always ) ]
  fn set( &mut self, component : IntoT )
  {
    self.field1 = component.into().clone();
  }
}

impl< IntoT > SetComponent< String, IntoT > for Options2
where
  IntoT : Into< String >,
{
  #[ inline( always ) ]
  fn set( &mut self, component : IntoT )
  {
    self.field2 = component.into().clone();
  }
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
  T : SetComponent< i32, IntoT >,
  T : SetComponent< String, IntoT >,
  IntoT : Into< i32 >,
  IntoT : Into< String >,
  IntoT : Clone,
{
  #[ inline( always ) ]
  fn components_set( &mut self, component : IntoT )
  {
    SetComponent::< i32, _ >::set( self, component.clone() );
    SetComponent::< String, _ >::set( self, component.clone() );
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

///
/// Set with type.
///

pub trait SetWithType
{
  fn set_with_type< T, IntoT >( &mut self, component : IntoT )
  where
    IntoT : Into< T >,
    Self : SetComponent< T, IntoT >;
}

impl SetWithType for Options2
{

  #[ inline( always ) ]
  fn set_with_type< T, IntoT >( &mut self, component : IntoT )
  where
    IntoT : Into< T >,
    Self : SetComponent< T, IntoT >,
  {
    SetComponent::< T, IntoT >::set( self, component );
  }

}

//

include!( "only_test/components_basic.rs" );
