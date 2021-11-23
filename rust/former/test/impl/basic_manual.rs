
extern crate former_runtime;

#[derive( Debug, PartialEq )]
pub struct Command
{
  pub int_1 : i32,
  string_1 : String,
  vec_1 : Vec< String >,
  hashmap_strings_1 : std::collections::HashMap< String, String >,
  int_optional_1 : core::option::Option< i32 >,
  string_optional_1 : Option< String >,
}

//

impl Command
{
  pub fn former() -> CommandFormer
  {
    CommandFormer
    {
      int_1 : core::option::Option::None,
      string_1 : core::option::Option::None,
      vec_1 : core::option::Option::None,
      hashmap_strings_1 : core::option::Option::None,
      int_optional_1 : core::option::Option::None,
      string_optional_1 : core::option::Option::None,
    }
  }
}

//

#[derive( Debug )]
pub struct CommandFormer
{
  pub int_1 : core::option::Option< i32 >,
  pub string_1 : core::option::Option< String >,
  pub vec_1 : core::option::Option< Vec< String > >,
  pub hashmap_strings_1 : core::option::Option< std::collections::HashMap< String, String > >,
  pub int_optional_1 :  core::option::Option< i32 >,
  pub string_optional_1 : core::option::Option< String >,
}

//

impl CommandFormer
{
  fn form( mut self ) -> Command
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

    Command
    {
      int_1,
      string_1,
      vec_1,
      hashmap_strings_1,
      int_optional_1,
      string_optional_1,
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

  pub fn vec_1( mut self ) -> former_runtime::VectorFormer
  <
    String,
    Vec< String >,
    CommandFormer,
    impl Fn( &mut CommandFormer, core::option::Option< Vec< String > > )
  >
  {
    let container = self.vec_1.take();
    let on_end = | former : &mut CommandFormer, container : core::option::Option< Vec< String > > |
    {
      former.vec_1 = container;
    };
    former_runtime::VectorFormer::new( self, container, on_end )
  }

  pub fn hashmap_strings_1( mut self ) -> former_runtime::HashmapFormer
  <
    String,
    String,
    std::collections::HashMap< String, String >,
    CommandFormer,
    impl Fn( &mut CommandFormer, core::option::Option< std::collections::HashMap< String, String > > )
  >
  {
    let container = self.hashmap_strings_1.take();
    let on_end = | former : &mut CommandFormer, container : core::option::Option< std::collections::HashMap< String, String > > |
    {
      former.hashmap_strings_1 = container;
    };
    former_runtime::HashmapFormer::new( self, container, on_end )
  }

  pub fn string_optional_1< Src >( mut self, src : Src ) -> Self
  where Src : core::convert::Into< String >
  {
    debug_assert!( self.string_optional_1.is_none() );
    self.string_optional_1 = Some( src.into() );
    self
  }

}

//

include!( "basic_test.rs" );
