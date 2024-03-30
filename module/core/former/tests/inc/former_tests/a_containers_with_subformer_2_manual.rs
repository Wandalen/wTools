#[ allow( unused_imports ) ]
use super::*;

// xxx : take care

#[ derive( Debug, PartialEq ) ]
pub struct Struct1
{
  vec_1 : Vec< String >,
  hashmap_strings_1 : std::collections::HashMap< String, String >,
  hashset_strings_1 : std::collections::HashSet< String >,
}

// = formed

impl Struct1
{
  pub fn former() -> Struct1Former< Struct1, the_module::ReturnPreformed >
  {
    Struct1Former::< Struct1, the_module::ReturnPreformed >::new()
  }
}

// = storage

// generated by former
pub struct Struct1FormerStorage
{
  pub vec_1 : ::core::option::Option< Vec< String > >,
  pub hashmap_strings_1 : ::core::option::Option< std::collections::HashMap< String, String > >,
  pub hashset_strings_1 : ::core::option::Option< std::collections::HashSet< String > >,
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

// = former

pub struct Struct1Former
<
  Context = Struct1,
  End = the_module::ReturnPreformed,
>
where
  End : the_module::FormingEnd< Struct1, Context >,
{
  storage : Struct1FormerStorage,
  context : ::core::option::Option< Context >,
  on_end : ::core::option::Option< End >,
}

impl< Context, End > Struct1Former< Context, End >
where
  End : the_module::FormingEnd< Struct1, Context >,
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

  // #[ inline( always ) ]
  // pub fn new() -> Struct1Former<Struct1, the_module::ReturnPreformed>
  // {
  //   Struct1Former::
  //   <
  //     Struct1,
  //     the_module::ReturnPreformed,
  //   >::begin(None, the_module::ReturnPreformed)
  // }

  #[ inline( always ) ]
  pub fn begin
  (
    mut storage : ::core::option::Option< Struct1FormerStorage >,
    context : ::core::option::Option< Context >,
    on_end : End,
  ) -> Self
  {
    if storage.is_none()
    {
      storage = Some( Default::default() );
    }
    Self
    {
      storage : storage.unwrap(),
      context,
      on_end : ::core::option::Option::Some( on_end ),
    }
  }

  #[ inline( always ) ]
  pub fn end( mut self ) -> Context
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    let formed = self.form();
    on_end.call( formed, context )
  }

  #[ inline( always ) ]
  pub fn __vec_1< Former2 >( self ) -> Former2
  where
    Former2 : former::FormerBegin
    <
      Vec< String >,
      Vec< String >,
      Self, End = former::FormingEndClosure< Vec< String >, Self >,
    >,
  {
    let on_end = | formed : Vec< String >, super_former : ::core::option::Option< Self > | -> Self
    {
      let mut super_former = super_former.unwrap();
      if let Some( ref mut field ) = super_former.storage.vec_1
      {
        former::ContainerAssign::assign( field, formed );
      }
      else
      {
        super_former.storage.vec_1 = Some( formed );
      }
      super_former
    };
    Former2::_begin( None, Some( self ), former::FormingEndClosure::new( on_end ) )
  }

  // xxx2 : continue
  pub fn vec_1( self ) -> the_module::VectorSubformer
  <
    String,
    Vec< String >,
    Self,
    impl the_module::FormingEnd< Vec< String >, Self >,
  >
  {
    self.__vec_1::< the_module::VectorSubformer::< _, _, _, _ > >()
  }

  // pub fn vec_1( mut self ) -> the_module::VectorSubformer
  // <
  //   String,
  //   Vec< String >,
  //   Self,
  //   impl the_module::FormingEnd< Vec< String >, Self >,
  // >
  // {
  //   let formed = self.storage.vec_1.take();
  //   let on_end = | formed : Vec< String >, super_former : ::core::option::Option< Self > | -> Self
  //   {
  //     let mut super_former = super_former.unwrap();
  //     super_former.storage.vec_1 = Some( formed );
  //     super_former
  //   };
  //   the_module::VectorSubformer::< String, Vec< String >, Self, _ >::begin( Some( self ), formed, on_end )
  // }

  pub fn hashmap_strings_1( mut self ) -> the_module::HashMapSubformer
  <
    String,
    String,
    std::collections::HashMap< String, String >,
    Self,
    impl the_module::FormingEnd< std::collections::HashMap< String, String >, Self >,
  >
  {
    let formed = self.storage.hashmap_strings_1.take();
    let on_end = | formed : std::collections::HashMap< String, String >, super_former : ::core::option::Option< Self > | -> Self
    {
      let mut super_former = super_former.unwrap();
      super_former.storage.hashmap_strings_1 = Some( formed );
      super_former
    };
    the_module::HashMapSubformer::begin( formed, Some( self ), on_end )
  }

  pub fn hashset_strings_1( mut self ) -> the_module::HashSetSubformer
  <
    String,
    std::collections::HashSet< String >,
    Self,
    impl the_module::FormingEnd< std::collections::HashSet< String >, Self >,
  >
  {
    let formed = self.storage.hashset_strings_1.take();
    let on_end = | formed : std::collections::HashSet< String >, super_former : ::core::option::Option< Self > | -> Self
    {
      let mut super_former = super_former.unwrap();
      super_former.storage.hashset_strings_1 = Some( formed );
      super_former
    };
    the_module::HashSetSubformer::begin( formed, Some( self ), on_end )
  }

}

// impl< Context, End > Struct1Former< Context, End >
// where
//   End: the_module::FormingEnd<Struct1, Context>,

impl Struct1Former< Struct1, the_module::ReturnPreformed >
{

  #[ inline( always ) ]
  pub fn new() -> Self
  {
    Self::begin( None, None, the_module::ReturnPreformed )
  }

}

//

// impl< Context, End > Struct1Former< Context, End >
// where
//   End : the_module::FormingEnd< Struct1, Context >,

impl< Context, End > former::FormerBegin< Struct1FormerStorage, Struct1, Context >
for Struct1Former< Context, End >
where
  End : the_module::FormingEnd< Struct1, Context >,
{
  type End = End;

  #[ inline( always ) ]
  fn _begin
  (
    storage : core::option::Option< Struct1FormerStorage >, /* xxx2 : that should be storage */
    context : core::option::Option< Context >,
    on_end : End,
  ) -> Self
  {
    debug_assert!( storage.is_none() );
    Self::begin( None, context, on_end )
  }

}

//

include!( "./only_test/containers_with_subformer.rs" );