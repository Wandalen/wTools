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

  pub fn vec_1( mut self ) -> former::runtime::VectorFormer
  <
    String,
    Vec< String >,
    Self,
    impl Fn( &mut Self, core::option::Option< Vec< String > > ),
  >
  {
    let container = self.vec_1.take();
    let on_end = | former : &mut Self, container : core::option::Option< Vec< String > > |
    {
      former.vec_1 = container;
    };
    former::runtime::VectorFormer::new( self, container, on_end )
  }

  // #[ derive( Debug, PartialEq ) ]
  // pub struct Struct1
  // {
  //   pub int_1 : i32,
  //   string_1 : String,
  //   int_optional_1 : core::option::Option< i32 >,
  //   string_optional_1 : Option< String >,
  //   #[ former( former::runtime::VectorFormer ) ]
  //   vec_1 : Vec< String >,
  //   #[ former( former::runtime::HashMapFormer ) ]
  //   hashmap_strings_1 : std::collections::HashMap< String, String >,
  //   #[ former( former::runtime::HashSetFormer ) ]
  //   hashset_strings_1 : std::collections::HashSet< String >,
  // }

  // xxx
  pub fn hashmap_strings_1( mut self ) -> former::runtime::HashMapFormer
  <
    String,
    String,
    std::collections::HashMap< String, String >,
    Struct1Former,
    impl Fn( &mut Struct1Former, core::option::Option< std::collections::HashMap< String, String > > )
  >
  {
    let container = self.hashmap_strings_1.take();
    let on_end = | former : &mut Struct1Former, container : core::option::Option< std::collections::HashMap< String, String > > |
    {
      former.hashmap_strings_1 = container;
    };
    former::runtime::HashMapFormer::new( self, container, on_end )
  }

  pub fn hashset_strings_1( mut self ) -> former::runtime::HashSetFormer
  <
    String,
    std::collections::HashSet< String >,
    Struct1Former,
    impl Fn( &mut Struct1Former, core::option::Option< std::collections::HashSet< String > > )
  >
  {
    let container = self.hashset_strings_1.take();
    let on_end = | former : &mut Struct1Former, container : core::option::Option< std::collections::HashSet< String > > |
    {
      former.hashset_strings_1 = container;
    };
    former::runtime::HashSetFormer::new( self, container, on_end )
  }

}

//

include!( "only_test/containers_with_runtime.rs" );
