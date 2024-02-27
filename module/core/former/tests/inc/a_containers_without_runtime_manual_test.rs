#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq ) ]
pub struct Struct1
{
  vec_1 : Vec< String >,
  hashmap_strings_1 : std::collections::HashMap< String, String >,
  hashset_strings_1 : std::collections::HashSet< String >,
}

//

impl Struct1
{
  pub fn former() -> Struct1Former
  {
    Struct1Former
    {
      vec_1 : core::option::Option::None,
      hashmap_strings_1 : core::option::Option::None,
      hashset_strings_1 : core::option::Option::None,
    }
  }
}

//

#[ derive( Debug ) ]
pub struct Struct1Former
{
  pub vec_1 : core::option::Option< Vec< String > >,
  pub hashmap_strings_1 : core::option::Option< std::collections::HashMap< String, String > >,
  pub hashset_strings_1 : core::option::Option< std::collections::HashSet< String > >,
}

//

impl Struct1Former
{
  fn form( mut self ) -> Struct1
  {

    let vec_1 = if self.vec_1.is_some()
    {
      self.vec_1.take().unwrap()
    }
    else
    {
      let val : Vec< String > = Default::default();
      val
    };

    let hashmap_strings_1 = if self.hashmap_strings_1.is_some()
    {
      self.hashmap_strings_1.take().unwrap()
    }
    else
    {
      let val : std::collections::HashMap< String, String > = Default::default();
      val
    };

    let hashset_strings_1 = if self.hashset_strings_1.is_some()
    {
      self.hashset_strings_1.take().unwrap()
    }
    else
    {
      let val : std::collections::HashSet< String > = Default::default();
      val
    };

    Struct1
    {
      vec_1,
      hashmap_strings_1,
      hashset_strings_1,
    }

  }

  pub fn vec_1< Src >( mut self, src : Src ) -> Self
  where Src : core::convert::Into< Vec< String > >
  {
    debug_assert!( self.vec_1.is_none() );
    self.vec_1 = Some( src.into() );
    self
  }

  pub fn hashmap_strings_1< Src >( mut self, src : Src ) -> Self
  where Src : core::convert::Into< std::collections::HashMap< String, String > >
  {
    debug_assert!( self.hashmap_strings_1.is_none() );
    self.hashmap_strings_1 = Some( src.into() );
    self
  }

  pub fn hashset_strings_1< Src >( mut self, src : Src ) -> Self
  where Src : core::convert::Into< std::collections::HashSet< String > >
  {
    debug_assert!( self.hashset_strings_1.is_none() );
    self.hashset_strings_1 = Some( src.into() );
    self
  }

}

//

include!( "only_test/containers_without_runtime.rs" );
