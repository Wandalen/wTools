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
      reflect::KeyVal { key : "f3", val : Box::new( self.f3.clone() ) },
    ];
    Box::new( result.into_iter() )
  }
}

impl reflect::Entity for Struct2
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
      reflect::KeyVal { key : "s1", val : Box::new( self.s1 ) },
      reflect::KeyVal { key : "s2", val : Box::new( self.s2.clone() ) },
      reflect::KeyVal { key : "s3", val : Box::new( self.s3 ) },
    ];
    Box::new( result.into_iter() )
  }
}

include!( "./only_test/reflect_struct_in_struct.rs" );
