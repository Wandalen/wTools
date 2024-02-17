use super::*;
pub use TheModule::reflect;

#[ derive( Debug, Clone, PartialEq ) ]
pub struct Struct1
{
  pub f1 : i32,
  pub f2 : String,
  pub f3 : &'static str,
}

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

impl reflect::Instance for Struct1 {}
impl reflect::EntityInterface for EntityDescriptor< Struct1 >
{
  type I = Struct1;
  #[ inline( always ) ]
  fn reflect_is_container( &self ) -> bool
  {
    true
  }
  #[ inline( always ) ]
  fn reflect_len( &self ) -> usize
  {
    3
  }
  #[ inline( always ) ]
  // fn reflect_elements( &self ) -> Box< dyn Iterator< Item = reflect::KeyVal< Box< dyn reflect::Instance > > > >
  fn reflect_elements( &self ) -> Box< dyn Iterator< Item = reflect::KeyVal< Box< dyn reflect::EntityInterface< I = dyn reflect::AnyInstance > > > > >
  {
    // let x = Box::new( reflect::EntityDescriptor::< i32 >::new() );
    let boxed_descriptor_as_trait : Box< dyn reflect::EntityInterface< I = dyn reflect::AnyInstance > >
      = Box::new( reflect::EntityDescriptor::< i32 >::new() );
    let result = vec!
    [
      // reflect::KeyVal { key: "f1", val: boxed_descriptor_as_trait },
      // Other KeyVal instances
    ];
    // let result = vec!
    // [
    //   reflect::KeyVal { key : "f1", val: Box::new( reflect::EntityDescriptor::< i32 >::new() ) },
    //   // reflect::KeyVal { key : "f2", val : Box::new( self.f2.clone() ) },
    //   // reflect::KeyVal { key : "f3", val : Box::new( self.f3 ) },
    // ];
    Box::new( result.into_iter() )
  }
}

// include!( "./only_test/reflect_struct.rs" );
