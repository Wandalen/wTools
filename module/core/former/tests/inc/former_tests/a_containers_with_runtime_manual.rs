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
  pub fn former() -> Struct1Former< Struct1, the_module::ReturnContainer >
  {
    Struct1Former::< Struct1, the_module::ReturnContainer >::new()
  }
}

// generated by former
pub struct Struct1FormerContainer
{
  pub vec_1 : core::option::Option< Vec< String > >,
  pub hashmap_strings_1 : core::option::Option< std::collections::HashMap< String, String > >,
  pub hashset_strings_1 : core::option::Option< std::collections::HashSet< String > >,
}

impl Default for Struct1FormerContainer
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
  __FormerEnd = the_module::ReturnContainer,
>
where
  __FormerEnd : the_module::ToSuperFormer< Struct1, __FormerContext >,
{
  container : Struct1FormerContainer,
  context : core::option::Option< __FormerContext >,
  on_end : core::option::Option< __FormerEnd >,
}

impl< __FormerContext, __FormerEnd > Struct1Former< __FormerContext, __FormerEnd >
where
  __FormerEnd: the_module::ToSuperFormer<Struct1, __FormerContext>,
{

  #[ inline( always ) ]
  fn form( mut self ) -> Struct1
  {

    let vec_1 = if self.container.vec_1.is_some()
    {
      self.container.vec_1.take().unwrap()
    }
    else
    {
      let val : Vec< String > = Default::default();
      val
    };

    let hashmap_strings_1 = if self.container.hashmap_strings_1.is_some()
    {
      self.container.hashmap_strings_1.take().unwrap()
    }
    else
    {
      let val : std::collections::HashMap< String, String > = Default::default();
      val
    };

    let hashset_strings_1 = if self.container.hashset_strings_1.is_some()
    {
      self.container.hashset_strings_1.take().unwrap()
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
  pub fn new() -> Struct1Former<Struct1, the_module::ReturnContainer>
  {
    Struct1Former::
    <
      Struct1,
      the_module::ReturnContainer,
    >::begin(None, the_module::ReturnContainer)
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
      container : core::default::Default::default(),
      context : context,
      on_end : ::core::option::Option::Some( on_end ),
    }
  }

  #[ inline( always ) ]
  pub fn end( mut self ) -> __FormerContext
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    let container = self.form();
    on_end.call( container, context )
  }

  pub fn vec_1( mut self ) -> the_module::VectorSubformer
  <
    String,
    Vec< String >,
    Self,
    impl the_module::ToSuperFormer< Vec< String >, Self >,
  >
  {
    let container = self.container.vec_1.take();
    let on_end = | container : Vec< String >, super_former : core::option::Option< Self > | -> Self
    {
      let mut super_former = super_former.unwrap();
      super_former.container.vec_1 = Some( container );
      super_former
    };
    the_module::VectorSubformer::< String, Vec< String >, Self, _ >::begin( Some( self ), container, on_end )
  }

  pub fn hashmap_strings_1( mut self ) -> the_module::HashMapSubformer
  <
    String,
    String,
    std::collections::HashMap< String, String >,
    Self,
    impl the_module::ToSuperFormer< std::collections::HashMap< String, String >, Self >,
  >
  {
    let container = self.container.hashmap_strings_1.take();
    let on_end = | container : std::collections::HashMap< String, String >, super_former : core::option::Option< Self > | -> Self
    {
      let mut super_former = super_former.unwrap();
      super_former.container.hashmap_strings_1 = Some( container );
      super_former
    };
    the_module::HashMapSubformer::begin( Some( self ), container, on_end )
  }

  pub fn hashset_strings_1( mut self ) -> the_module::HashSetSubformer
  <
    String,
    std::collections::HashSet< String >,
    Self,
    impl the_module::ToSuperFormer< std::collections::HashSet< String >, Self >,
  >
  {
    let container = self.container.hashset_strings_1.take();
    let on_end = | container : std::collections::HashSet< String >, super_former : core::option::Option< Self > | -> Self
    {
      let mut super_former = super_former.unwrap();
      super_former.container.hashset_strings_1 = Some( container );
      super_former
    };
    the_module::HashSetSubformer::begin( Some( self ), container, on_end )
  }

}

//

include!( "../only_test/containers_with_runtime.rs" );