#[ allow( unused_imports ) ]
use super::*;
// use TheModule::*;

#[derive( Debug, PartialEq )]
pub struct Struct1
{
  pub int_1 : i32,
  string_1 : String,
  int_optional_1 : core::option::Option< i32 >,
  string_optional_1 : Option< String >,
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
      int_1 : core::option::Option::None,
      string_1 : core::option::Option::None,
      int_optional_1 : core::option::Option::None,
      string_optional_1 : core::option::Option::None,
      vec_1 : core::option::Option::None,
      hashmap_strings_1 : core::option::Option::None,
      hashset_strings_1 : core::option::Option::None,
    }
  }
}

//

#[derive( Debug )]
pub struct Struct1Former
{
  pub int_1 : core::option::Option< i32 >,
  pub string_1 : core::option::Option< String >,
  pub int_optional_1 :  core::option::Option< i32 >,
  pub string_optional_1 : core::option::Option< String >,
  pub vec_1 : core::option::Option< Vec< String > >,
  pub hashmap_strings_1 : core::option::Option< std::collections::HashMap< String, String > >,
  pub hashset_strings_1 : core::option::Option< std::collections::HashSet< String > >,
}

//

impl Struct1Former
{
  fn form( mut self ) -> Struct1
  {

    let int_1 = if self.int_1.is_some()
    {
      self.int_1.take().unwrap()
    }
    else
    {
      let val : i32 = Default::default();
      val
    };

    let string_1 = if self.string_1.is_some()
    {
      self.string_1.take().unwrap()
    }
    else
    {
      let val : String = Default::default();
      val
    };

    let int_optional_1 = if self.int_optional_1.is_some()
    {
      Some( self.int_optional_1.take().unwrap() )
    }
    else
    {
      None
    };

    let string_optional_1 = if self.string_optional_1.is_some()
    {
      Some( self.string_optional_1.take().unwrap() )
    }
    else
    {
      None
    };

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
      int_1,
      string_1,
      int_optional_1,
      string_optional_1,
      vec_1,
      hashmap_strings_1,
      hashset_strings_1,
    }

  }

  pub fn int_1< Src >( mut self, src : Src ) -> Self
  where Src : core::convert::Into< i32 >,
  {
    debug_assert!( self.int_1.is_none() );
    self.int_1 = Some( src.into() );
    self
  }

  pub fn string_1< Src >( mut self, src : Src ) -> Self
  where Src : core::convert::Into< String >,
  {
    debug_assert!( self.string_1.is_none() );
    self.string_1 = Some( src.into() );
    self
  }

  pub fn string_optional_1< Src >( mut self, src : Src ) -> Self
  where Src : core::convert::Into< String >
  {
    debug_assert!( self.string_optional_1.is_none() );
    self.string_optional_1 = Some( src.into() );
    self
  }

  pub fn vec_1( mut self ) -> former::runtime::VectorFormer
  <
    String,
    Vec< String >,
    Struct1Former,
    impl Fn( &mut Struct1Former, core::option::Option< Vec< String > > )
  >
  {
    let container = self.vec_1.take();
    let on_end = | former : &mut Struct1Former, container : core::option::Option< Vec< String > > |
    {
      former.vec_1 = container;
    };
    former::runtime::VectorFormer::new( self, container, on_end )
  }

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

include!( "basic_runtime_only_test.rs" );
