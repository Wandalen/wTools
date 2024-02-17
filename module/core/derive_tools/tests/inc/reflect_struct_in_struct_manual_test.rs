use super::*;
pub use TheModule::reflect;

#[ derive( Debug, Clone, PartialEq ) ]
pub struct Struct1
{
  pub f1 : i32,
  pub f2 : String,
  pub f3 : Struct2,
}

#[ derive( Debug, Clone, PartialEq ) ]
pub struct Struct2
{
  pub s1 : i32,
  pub s2 : String,
  pub s3 : &'static str,
}

// --

#[ derive( PartialEq, Debug ) ]
pub struct EntityDescriptor< I : reflect::Instance >
{
  _phantom : core::marker::PhantomData< I >,
}

impl< I : reflect::Instance > EntityDescriptor< I >
{
  /// Constructor of the descriptor of type.
  #[ inline( always ) ]
  pub fn new() -> Self
  {
    let _phantom = core::marker::PhantomData::< I >;
    Self { _phantom }
  }
  /// Constructor of the descriptor of type.
}

// --

impl reflect::Instance for Struct1
{
  #[ inline( always ) ]
  fn Reflect() -> impl reflect::Entity
  {
    EntityDescriptor::< Self >::new()
  }
}

impl Reflect::Instance for Struct2
{
  #[ inline( always ) ]
  fn Reflect() -> impl reflect::Entity
  {
    EntityDescriptor::< Self >::new()
  }
}

impl reflect::Entity for EntityDescriptor< Struct1 >
{
  #[ inline( always ) ]
  fn is_container( &self ) -> bool
  {
    true
  }
  #[ inline( always ) ]
  fn len( &self ) -> usize
  {
    3
  }
  #[ inline( always ) ]
  fn type_name( &self ) -> &'static str
  {
    core::any::type_name::< Struct1 >()
  }
  #[ inline( always ) ]
  fn type_id( &self ) -> core::any::TypeId
  {
    core::any::TypeId::of::< Struct1 >()
  }
  #[ inline( always ) ]
  fn elements(&self) -> Box< dyn Iterator< Item = reflect::KeyVal > >
  {
    let result = vec!
    [
      reflect::KeyVal { key: "f1", val: Box::new( < i32 as reflect::Instance >::Reflect() ) },
      reflect::KeyVal { key: "f2", val: Box::new( < String as reflect::Instance >::Reflect() ) },
      reflect::KeyVal { key: "f3", val: Box::new( < Struct2 as reflect::Instance >::Reflect() ) },
    ];
    Box::new( result.into_iter() )
  }

}

impl reflect::Entity for EntityDescriptor< Struct2 >
{
  #[ inline( always ) ]
  fn is_container( &self ) -> bool
  {
    true
  }
  #[ inline( always ) ]
  fn len( &self ) -> usize
  {
    3
  }
  #[ inline( always ) ]
  fn type_name( &self ) -> &'static str
  {
    core::any::type_name::< Struct2 >()
  }
  #[ inline( always ) ]
  fn type_id( &self ) -> core::any::TypeId
  {
    core::any::TypeId::of::< Struct2 >()
  }
  #[ inline( always ) ]
  fn elements(&self) -> Box< dyn Iterator< Item = reflect::KeyVal > >
  {
    let result = vec!
    [
      reflect::KeyVal { key: "s1", val: Box::new( < i32 as reflect::Instance >::Reflect() ) },
      reflect::KeyVal { key: "s2", val: Box::new( < String as reflect::Instance >::Reflect() ) },
      reflect::KeyVal { key: "s3", val: Box::new( < &'static str as reflect::Instance >::Reflect() ) },
    ];
    Box::new( result.into_iter() )
  }

}

include!( "./only_test/reflect_struct_in_struct.rs" );
