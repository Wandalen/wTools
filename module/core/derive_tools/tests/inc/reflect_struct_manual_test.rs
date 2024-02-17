use super::*;
pub use TheModule::reflect;

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
  #[ inline( always ) ]
  fn reflect( &self ) -> impl reflect::Entity
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
  fn elements(&self) -> Box< dyn Iterator< Item = reflect::KeyVal > >
  {
    let result = vec!
    [
      reflect::KeyVal { key: "f1", val: Box::new( reflect::EntityDescriptor::<i32>::new() ) },
      reflect::KeyVal { key: "f2", val: Box::new( reflect::EntityDescriptor::<String>::new() ) },
      reflect::KeyVal { key: "f3", val: Box::new( reflect::EntityDescriptor::<&'static str>::new() ) },
    ];
    Box::new( result.into_iter() )
  }

}

include!( "./only_test/reflect_struct.rs" );
