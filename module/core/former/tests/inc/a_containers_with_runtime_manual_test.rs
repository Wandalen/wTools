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
  #[ inline( always ) ]
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

  // pub fn vec_1( mut self ) -> former::runtime::VectorSubformer
  // <
  //   String,
  //   Vec< String >,
  //   Self,
  //   impl Fn( &mut Self, core::option::Option< Vec< String > > ),
  // >
  // {
  //   let container = self.vec_1.take();
  //   let on_end = | former : &mut Self, container : core::option::Option< Vec< String > > |
  //   {
  //     former.vec_1 = container;
  //   };
  //   former::runtime::VectorSubformer::begin( self, container, on_end )
  // }

  pub fn vec_1( mut self ) -> former::runtime::VectorSubformer
  <
    String,
    Vec< String >,
    Struct1Former,
    impl Fn( Vec< String >, core::option::Option< Self > ) -> Self
  >
  {
    let container = self.vec_1.take();
    let on_end = | container : Vec< String >, former : core::option::Option< Self > | -> Self
    {
      let mut former = former.unwrap();
      former.vec_1 = Some( container );
      former
    };
    former::runtime::VectorSubformer::begin( Some( self ), container, on_end )
  }

  pub fn hashmap_strings_1( mut self ) -> former::runtime::HashMapSubformer
  <
    String,
    String,
    std::collections::HashMap< String, String >,
    Struct1Former,
    impl Fn( std::collections::HashMap< String, String >, core::option::Option< Self > ) -> Self
  >
  {
    let container = self.hashmap_strings_1.take();
    let on_end = | container : std::collections::HashMap< String, String >, former : core::option::Option< Self > | -> Self
    {
      let mut former = former.unwrap();
      former.hashmap_strings_1 = Some( container );
      former
    };
    former::runtime::HashMapSubformer::begin( Some( self ), container, on_end )
  }

  pub fn hashset_strings_1( mut self ) -> former::runtime::HashSetSubformer
  <
    String,
    std::collections::HashSet< String >,
    Struct1Former,
    impl Fn( std::collections::HashSet< String >, core::option::Option< Self > ) -> Self
  >
  {
    let container = self.hashset_strings_1.take();
    let on_end = | container : std::collections::HashSet< String >, former : core::option::Option< Self > | -> Self
    {
      let mut former = former.unwrap();
      former.hashset_strings_1 = Some( container );
      former
    };
    former::runtime::HashSetSubformer::begin( Some( self ), container, on_end )
  }

}

//

include!( "only_test/containers_with_runtime.rs" );
