use super::*;
pub use the_module::reflect;

#[ derive( Debug, Clone, PartialEq ) ]
pub struct Struct1
{
  pub f1 : i32,
  pub f2 : String,
  pub f3 : &'static str,
}

// --

#[ derive( PartialEq, Debug ) ]
pub struct EntityDescriptor< I : reflect::Instance >
{
  _phantom : core::marker::PhantomData< I >,
}

//
// xxx : qqq : qqq for Yulia : implement derive Phantom
//
// #[ derive( PartialEq, Debug ) ]
// pub struct EntityDescriptor< I : reflect::Instance >
// {
//   _phantom : core::marker::PhantomData< I >,
// }
//
// #[ derive( PartialEq, Debug, Phantom ) ]
// pub struct EntityDescriptor< I : Instance >;
//
// #[ derive( PartialEq, Debug, Phantom ) ]
// pub struct EntityDescriptor< I : Instance > {};
//
// #[ derive( PartialEq, Debug ) ]
// pub struct EntityDescriptor< 'a, 'b, I : reflect::Instance >
// {
//   _phantom : core::marker::PhantomData< ( &'a (), &'b (), I ) >,
// }
//

impl< I : reflect::Instance > EntityDescriptor< I >
{
  /// Constructor of the descriptor.
  #[ inline( always ) ]
  pub fn new() -> Self
  {
    let _phantom = core::marker::PhantomData::< I >;
    Self { _phantom }
  }
}

// qqq : qqq for Yulia : implement derive ReflectInstance
impl reflect::Instance for Struct1
{
  type Entity = EntityDescriptor::< Self >;
  #[ inline( always ) ]
  fn Reflect() -> Self::Entity
  {
    EntityDescriptor::< Self >::new()
  }
}

// --

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
      reflect::KeyVal { key : reflect::Primitive::str( "f1" ), val : Box::new( < i32 as reflect::Instance >::Reflect() ) },
      reflect::KeyVal { key : reflect::Primitive::str( "f2" ), val : Box::new( < String as reflect::Instance >::Reflect() ) },
      reflect::KeyVal { key : reflect::Primitive::str( "f3" ), val : Box::new( < &'static str as reflect::Instance >::Reflect() ) },
    ];
    Box::new( result.into_iter() )
  }

}

include!( "./only_test/reflect_struct.rs" );
