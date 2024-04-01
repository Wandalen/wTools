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
  pub fn former() -> Struct1Former< Struct1, the_module::ReturnFormed >
  {
    Struct1Former::< Struct1, the_module::ReturnFormed >::new()
  }
}

// generated by former
pub struct Struct1FormerStorage
{
  pub vec_1 : core::option::Option< Vec< String > >,
  pub hashmap_strings_1 : core::option::Option< std::collections::HashMap< String, String > >,
  pub hashset_strings_1 : core::option::Option< std::collections::HashSet< String > >,
}

impl Default for Struct1FormerStorage
{

  #[ inline( always ) ]
  fn default() -> Self
  {
    Self
    {
      vec_1 : None,
      hashmap_strings_1 : None,
      hashset_strings_1 : None,
    }
  }

}

//

pub struct Struct1Former
<
  __FormerContext = Struct1,
  __FormerEnd = the_module::ReturnFormed,
>
where
  __FormerEnd : the_module::FormingEnd< Struct1, __FormerContext >,
{
  storage : Struct1FormerStorage,
  context : core::option::Option< __FormerContext >,
  on_end : core::option::Option< __FormerEnd >,
}

impl< __FormerContext, __FormerEnd > Struct1Former< __FormerContext, __FormerEnd >
where
  __FormerEnd: the_module::FormingEnd<Struct1, __FormerContext>,
{

  #[ inline( always ) ]
  fn form( mut self ) -> Struct1
  {

    let vec_1 = if self.storage.vec_1.is_some()
    {
      self.storage.vec_1.take().unwrap()
    }
    else
    {
      let val : Vec< String > = Default::default();
      val
    };

    let hashmap_strings_1 = if self.storage.hashmap_strings_1.is_some()
    {
      self.storage.hashmap_strings_1.take().unwrap()
    }
    else
    {
      let val : std::collections::HashMap< String, String > = Default::default();
      val
    };

    let hashset_strings_1 = if self.storage.hashset_strings_1.is_some()
    {
      self.storage.hashset_strings_1.take().unwrap()
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

  #[ inline( always ) ]
  pub fn perform(self) -> Struct1
  {
    let result = self.form();
    return result;
  }

  #[ inline( always ) ]
  pub fn new() -> Struct1Former<Struct1, the_module::ReturnFormed>
  {
    Struct1Former::
    <
      Struct1,
      the_module::ReturnFormed,
    >::begin(None, the_module::ReturnFormed)
  }

  #[ inline( always ) ]
  pub fn begin
  (
    context : core::option::Option< __FormerContext >,
    on_end : __FormerEnd,
  ) -> Self
  {
    Self
    {
      storage : core::default::Default::default(),
      context : context,
      on_end : ::core::option::Option::Some( on_end ),
    }
  }

  #[ inline( always ) ]
  pub fn end( mut self ) -> __FormerContext
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    let formed = self.form();
    on_end.call( formed, context )
  }

  pub fn vec_1< Src >( mut self, src : Src ) -> Self
  where Src : core::convert::Into< Vec< String > >
  {
    debug_assert!( self.storage.vec_1.is_none() );
    self.storage.vec_1 = Some( src.into() );
    self
  }

  pub fn hashmap_strings_1< Src >( mut self, src : Src ) -> Self
  where Src : core::convert::Into< std::collections::HashMap< String, String > >
  {
    debug_assert!( self.storage.hashmap_strings_1.is_none() );
    self.storage.hashmap_strings_1 = Some( src.into() );
    self
  }

  pub fn hashset_strings_1< Src >( mut self, src : Src ) -> Self
  where Src : core::convert::Into< std::collections::HashSet< String > >
  {
    debug_assert!( self.storage.hashset_strings_1.is_none() );
    self.storage.hashset_strings_1 = Some( src.into() );
    self
  }

}

//

include!( "../only_test/containers_without_runtime.rs" );
