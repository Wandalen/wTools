use super::*;
pub use TheModule::reflect;

#[ derive( Debug, Clone, PartialEq ) ]
pub struct Struct1
{
  pub f1 : i32,
  pub f2 : String,
  pub f3 : &'static str,
}

impl reflect::Entity for Struct1
{
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
  fn reflect_elements( &self ) -> Box< dyn Iterator< Item = reflect::KeyVal > + '_ >
  {
    let result = vec!
    [
      reflect::KeyVal { key : "f1", val : Box::new( self.f1 ) },
      reflect::KeyVal { key : "f2", val : Box::new( self.f2.clone() ) },
      reflect::KeyVal { key : "f3", val : Box::new( self.f3 ) },
    ];
    Box::new( result.into_iter() )
  }
}

include!( "./only_test/reflect_struct.rs" );
