#[ allow( unused_imports ) ]
use super::*;

#[ derive( Default, Debug, PartialEq ) ]
pub struct Struct1
{
  vec_1 : Vec< String >,
  hashmap_1 : std::collections::HashMap< String, String >,
  hashset_1 : std::collections::HashSet< String >,
}

// = generated

#[ automatically_derived ]
impl Struct1
{
  #[ inline( always ) ]
  pub fn former() -> Struct1Former< >
  {
    Struct1Former::<>::new( the_module::ReturnPreformed )
  }
}

// = definition

#[ derive( Debug ) ]
pub struct Struct1FormerDefinitionTypes< Context = (), Formed = Struct1 >
{
  _phantom : core::marker::PhantomData< ( Context, Formed ) >,
}

impl< Context, Formed > Default for Struct1FormerDefinitionTypes< Context, Formed >
{
  fn default() -> Self
  {
    Self
    {
      _phantom : core::marker::PhantomData,
    }
  }
}

#[ derive( Debug ) ]
pub struct Struct1FormerDefinition< Context = (), Formed = Struct1, End = former::ReturnPreformed >
{
  _phantom : core::marker::PhantomData< ( Context, Formed, End ) >,
}

impl< Context, Formed, End > Default for Struct1FormerDefinition< Context, Formed, End >
{
  fn default() -> Self
  {
    Self
    {
      _phantom : core::marker::PhantomData,
    }
  }
}

impl< Context, Formed > former::FormerDefinitionTypes for Struct1FormerDefinitionTypes< Context, Formed >
{
  type Storage = Struct1FormerStorage;
  type Formed = Formed;
  type Context = Context;
}

impl< Context, Formed, End > former::FormerDefinition for Struct1FormerDefinition< Context, Formed, End >
where
  End : former::FormingEnd< Struct1FormerDefinitionTypes< Context, Formed > >,
{
  type Types = Struct1FormerDefinitionTypes< Context, Formed >;
  type End = End;
}

pub type Struct1FormerWithClosure< Context, Formed > = Struct1FormerDefinition< Context, Formed, former::FormingEndClosure< Struct1FormerDefinitionTypes< Context, Formed > > >;

// = storage

#[ doc = "Container of a corresponding former." ]
pub struct Struct1FormerStorage
{
  #[ doc = r" A field" ]
  pub vec_1 : ::core::option::Option< Vec< String > >,

  #[ doc = r" A field" ]
  pub hashmap_1 : ::core::option::Option< std::collections::HashMap< String, String > >,

  #[ doc = r" A field" ]
  pub hashset_1 : ::core::option::Option< std::collections::HashSet< String > >,
}

impl ::core::default::Default for Struct1FormerStorage
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    Self
    {
      vec_1 : ::core::option::Option::None,
      hashmap_1 : ::core::option::Option::None,
      hashset_1 : ::core::option::Option::None,
    }
  }
}

impl former::Storage for Struct1FormerStorage
{
  type Formed = Struct1;
}

impl former::StoragePreform for Struct1FormerStorage
{
  fn preform( mut self ) -> < Self as former::Storage >::Formed
  {
    let vec_1 = if self.vec_1.is_some()
    {
      self.vec_1.take().unwrap()
    }
    else
    {
      {
        trait MaybeDefault< T >
        {
          fn maybe_default( self : &Self ) -> T
          {
            panic!( "Field 'vec_1' isn't initialized" )
          }
        }
        impl< T > MaybeDefault< T > for &::core::marker::PhantomData< T > {}
        impl< T > MaybeDefault< T > for ::core::marker::PhantomData< T >
        where T : ::core::default::Default,
        {
          fn maybe_default( self : &Self ) -> T
          {
            T::default()
          }
        }
        ( &::core::marker::PhantomData::< Vec< String > > ).maybe_default()
      }
    };
    let hashmap_1 = if self.hashmap_1.is_some()
    {
      self.hashmap_1.take().unwrap()
    }
    else
    {
      {
        trait MaybeDefault< T >
        {
          fn maybe_default( self : &Self ) -> T
          {
            panic!( "Field 'hashmap_1' isn't initialized" )
          }
        }
        impl< T > MaybeDefault< T > for &::core::marker::PhantomData< T > {}
        impl< T > MaybeDefault< T > for ::core::marker::PhantomData< T >
        where T : ::core::default::Default,
        {
          fn maybe_default( self : &Self ) -> T
          {
            T::default()
          }
        }
        ( &::core::marker::PhantomData::< std::collections::HashMap< String, String > > ).maybe_default()
      }
    };
    let hashset_1 = if self.hashset_1.is_some()
    {
      self.hashset_1.take().unwrap()
    }
    else
    {
      {
        trait MaybeDefault< T >
        {
          fn maybe_default( self : &Self ) -> T
          {
            panic!( "Field 'hashset_1' isn't initialized" )
          }
        }
        impl< T > MaybeDefault< T > for &::core::marker::PhantomData< T > {}
        impl< T > MaybeDefault< T > for ::core::marker::PhantomData< T >
        where T : ::core::default::Default,
        {
          fn maybe_default( self : &Self ) -> T
          {
            T::default()
          }
        }
        ( &::core::marker::PhantomData::< std::collections::HashSet< String > > ).maybe_default()
      }
    };
    let result = Struct1
    {
      vec_1,
      hashmap_1,
      hashset_1,
    };
    return result;
  }
}

// = former

#[ doc = " Object to form [Struct1]. If field's values is not set then default value of the field is set.\n\nFor specifying custom default value use attribute `default`. For example:\n```\n\nuse former::Former;\n#[ derive( Former ) ]\npub struct Struct1\n{\n  #[ default( 31 ) ]\n  field1 : i32,\n}\n\n```\n" ]
pub struct Struct1Former< Definition = Struct1FormerDefinition >
where
  Definition : former::FormerDefinition,
  < Definition::Types as former::FormerDefinitionTypes >::Storage : former::StoragePreform,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage >,
{
  storage : < Definition::Types as former::FormerDefinitionTypes >::Storage,
  context : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Context >,
  on_end : core::option::Option< Definition::End >,
}

#[ automatically_derived ]
impl< Definition > Struct1Former< Definition >
where
  Definition : former::FormerDefinition,
  < Definition::Types as former::FormerDefinitionTypes >::Storage : former::StoragePreform,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage >,
{
  #[ doc = r"" ]
  #[ doc = r" Finish setting options and call perform on formed entity." ]
  #[ doc = r"" ]
  #[ doc = r" If `perform` defined then associated method is called and its result returned instead of entity." ]
  #[ doc = r" For example `perform()` of structure with : `#[ perform( fn after1() -> &str ) ]` returns `&str`." ]
  #[ doc = r"" ]
  #[ inline( always ) ]
  pub fn perform( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
  {
    let result = self.form();
    return result;
  }

  #[ doc = r"" ]
  #[ doc = r" Construct new instance of former with default parameters." ]
  #[ doc = r"" ]
  #[ inline( always ) ]
  pub fn _new_precise( on_end : Definition::End ) -> Self
  {
    Self::begin( None, None, on_end )
  }

  #[ doc = r"" ]
  #[ doc = r" Construct new instance of former with default parameters." ]
  #[ doc = r"" ]
  #[ inline( always ) ]
  pub fn new< IntoEnd >( end : IntoEnd ) -> Self
  where
    IntoEnd : Into< Definition::End >,
  {
    Self::begin( None, None, end, )
  }

  #[ doc = r"" ]
  #[ doc = r" Begin the process of forming. Expects context of forming to return it after forming." ]
  #[ doc = r"" ]
  #[ inline( always ) ]
  pub fn _begin_precise(
    mut storage : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Storage >,
    context : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Context >,
    on_end : < Definition as former::FormerDefinition >::End,
  ) -> Self
  {
    if storage.is_none()
    {
      storage = Some( ::core::default::Default::default() );
    }
    Self
    {
      storage : storage.unwrap(),
      context : context,
      on_end : ::core::option::Option::Some( on_end ),
    }
  }

  #[ doc = r"" ]
  #[ doc = r" Begin the process of forming. Expects context of forming to return it after forming." ]
  #[ doc = r"" ]
  #[ inline( always ) ]
  pub fn begin< IntoEnd >(
    mut storage : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Storage >,
    context : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Context >,
    on_end : IntoEnd,
  ) -> Self
  where
    IntoEnd : ::core::convert::Into< < Definition as former::FormerDefinition >::End >,
  {
    if storage.is_none()
    {
      storage = Some( ::core::default::Default::default() );
    }
    Self
    {
      storage : storage.unwrap(),
      context : context,
      on_end : ::core::option::Option::Some( ::core::convert::Into::into( on_end ) ),
    }
  }

  #[ doc = r"" ]
  #[ doc = r" End the process of forming returning original context of forming." ]
  #[ doc = r"" ]
  #[ inline( always ) ]
  pub fn form( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
  {
    self.end()
  }

  #[ doc = r"" ]
  #[ doc = r" End the process of forming returning original context of forming." ]
  #[ doc = r"" ]
  #[ inline( always ) ]
  pub fn end( mut self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    former::FormingEnd::< Definition::Types >::call( &on_end, self.storage, context )
  }

  #[ inline( always ) ]
  pub fn vec_1_set< Former2 >( self ) -> Former2
  where
    Former2 : former::FormerBegin
    <
      former::VectorDefinition
      <
        String,
        Self,
        Self,
        Struct1FormerVec1End,
      >
    >,
  {
    Former2::_begin( None, Some( self ), Struct1FormerVec1End )
  }

  pub fn vec_1( self ) ->
  former::ContainerSubformer::
  <
    String, former::VectorDefinition< String, Self, Self, Struct1FormerVec1End >
  >
  {
    self.vec_1_set::< former::ContainerSubformer::
    <
      String, former::VectorDefinition< String, Self, Self, Struct1FormerVec1End >
    >>()
  }

  #[ inline( always ) ]
  pub fn hashmap_1_set< Former2 >( self ) -> Former2
  where
    Former2 : former::FormerBegin
    <
      former::HashMapDefinition
      <
        String,
        String,
        Self,
        Self,
        Struct1FormerHashmap1End,
      >
    >,
  {
    Former2::_begin( None, Some( self ), Struct1FormerHashmap1End )
  }

  pub fn hashmap_1( self ) ->
  former::ContainerSubformer::
  <
    ( String, String ), former::HashMapDefinition< String, String, Self, Self, Struct1FormerHashmap1End >
  >
  {
    self.hashmap_1_set::< former::ContainerSubformer::
    <
      ( String, String ), former::HashMapDefinition< String, String, Self, Self, Struct1FormerHashmap1End >
    > >()
  }

  #[ inline( always ) ]
  pub fn hashset_1_set< Former2 >( self ) -> Former2
  where
    Former2 : former::FormerBegin
    <
      former::HashSetDefinition
      <
        String,
        Self,
        Self,
        Struct1FormerHashset1End,
      >
    >,
  {
    Former2::_begin( None, Some( self ), Struct1FormerHashset1End )
  }

  pub fn hashset_1( self ) ->
  former::ContainerSubformer::
  <
    String, former::HashSetDefinition< String, Self, Self, Struct1FormerHashset1End >
  >
  {
    self.hashset_1_set::< former::ContainerSubformer::
    <
      String, former::HashSetDefinition< String, Self, Self, Struct1FormerHashset1End >
    >>()
  }

}

impl< Definition > Struct1Former< Definition >
where
  Definition : former::FormerDefinition,
  < Definition::Types as former::FormerDefinitionTypes >::Storage : former::StoragePreform,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage, Formed = Struct1 >,
{
  pub fn preform( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
  {
    former::StoragePreform::preform( self.storage )
  }
}

// zzz : description
/// Return original former after subformer for `vec_1` is done.
#[ allow( non_camel_case_types ) ]
pub struct Struct1FormerVec1End;
#[ automatically_derived ]
impl< Definition > former::FormingEnd
<
  former::VectorDefinition< String, Struct1Former< Definition >, Struct1Former< Definition >, former::NoEnd >,
>
for Struct1FormerVec1End
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes
  <
    Storage = Struct1FormerStorage
  >,
{
  #[ inline( always ) ]
  fn call( &self, storage : Vec< String >, super_former : Option< Struct1Former< Definition > > ) -> Struct1Former< Definition >
  {
    let mut super_former = super_former.unwrap();
    if let Some( ref mut field ) = super_former.storage.vec_1
    {
      former::ContainerAssign::assign( field, storage );
    }
    else
    {
      super_former.storage.vec_1 = Some( storage );
    }
    super_former
  }
}

// zzz : description
/// Return original former after subformer for `hashmap_string_1` is done.
#[ allow( non_camel_case_types ) ]
pub struct Struct1FormerHashmap1End;
#[ automatically_derived ]
impl< Definition > former::FormingEnd
<
  former::HashMapDefinition< String, String, Struct1Former< Definition >, Struct1Former< Definition >, former::NoEnd >,
>
for Struct1FormerHashmap1End
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes
  <
    Storage = Struct1FormerStorage
  >,
{
  #[ inline( always ) ]
  fn call( &self, storage : std::collections::HashMap< String, String >, super_former : Option< Struct1Former< Definition > > ) -> Struct1Former< Definition >
  {
    let mut super_former = super_former.unwrap();
    if let Some( ref mut field ) = super_former.storage.hashmap_1
    {
      former::ContainerAssign::assign( field, storage );
    }
    else
    {
      super_former.storage.hashmap_1 = Some( storage );
    }
    super_former
  }
}

// zzz : description
/// Return original former after subformer for `hashset_string_1` is done.
#[ allow( non_camel_case_types ) ]
pub struct Struct1FormerHashset1End;
#[ automatically_derived ]
impl< Definition > former::FormingEnd
<
  former::HashSetDefinition< String, Struct1Former< Definition >, Struct1Former< Definition >, former::NoEnd >,
>
for Struct1FormerHashset1End
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes
  <
    Storage = Struct1FormerStorage
  >,
{
  #[ inline( always ) ]
  fn call( &self, storage : std::collections::HashSet< String >, super_former : Option< Struct1Former< Definition > > ) -> Struct1Former< Definition >
  {
    let mut super_former = super_former.unwrap();
    if let Some( ref mut field ) = super_former.storage.hashset_1
    {
      former::ContainerAssign::assign( field, storage );
    }
    else
    {
      super_former.storage.hashset_1 = Some( storage );
    }
    super_former
  }
}

// = end of generated

include!( "./only_test/containers_with_subformer.rs" );
