use super::*;
pub use the_module::reflect;

#[ derive( Debug, Clone, PartialEq ) ]
pub struct Struct1< 'a, 'b >
{
  pub f1 : &'a i32,
  pub f2 : i32,
  pub f3 : &'b str,
}

// --

#[ derive( PartialEq, Debug ) ]
pub struct EntityDescriptor< 'a, 'b, I : reflect::Instance >
{
  _phantom : core::marker::PhantomData< ( &'a (), &'b (), I ) >,
}

impl< 'a, 'b, I : reflect::Instance > EntityDescriptor< 'a, 'b, I >
{
  /// Constructor of the descriptor.
  #[ inline( always ) ]
  pub fn new() -> Self
  {
    let _phantom = core::marker::PhantomData::< ( &'a (), &'b (), I ) >;
    Self { _phantom }
  }
}

// qqq : qqq for Yulia : implement derive ReflectInstance
impl< 'a, 'b > reflect::Instance for Struct1< 'a, 'b >
{
  type Entity = EntityDescriptor::< 'a, 'b, Self >;
  #[ inline( always ) ]
  fn Reflect() -> Self::Entity
  {
    EntityDescriptor::< Self >::new()
  }
}

// --

impl< 'a, 'b > reflect::Entity for EntityDescriptor< 'a, 'b, Struct1< 'a, 'b > >
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
    core::any::type_name::< Struct1< 'a, 'b > >()
  }

  #[ inline( always ) ]
  fn type_id( &self ) -> core::any::TypeId
  {
    core::any::TypeId::of::< Struct1< 'static, 'static > >()
  }

  #[ inline( always ) ]
  fn elements(&self) -> Box< dyn Iterator< Item = reflect::KeyVal > >
  {
    let result = vec!
    [
      reflect::KeyVal { key : reflect::Primitive::str( "f1" ), val : Box::new( < &'static i32 as reflect::Instance >::Reflect() ) },
      reflect::KeyVal { key : reflect::Primitive::str( "f2" ), val : Box::new( < i32 as reflect::Instance >::Reflect() ) },
      reflect::KeyVal { key : reflect::Primitive::str( "f3" ), val : Box::new( < &'static str as reflect::Instance >::Reflect() ) },
    ];
    Box::new( result.into_iter() )
  }

}

include!( "./only_test/reflect_struct_with_lifetime.rs" );
