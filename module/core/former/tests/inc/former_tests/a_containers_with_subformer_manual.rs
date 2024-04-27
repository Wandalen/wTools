#[ allow( unused_imports ) ]
use super::*;

#[ derive( Default, Debug, PartialEq ) ]
pub struct Struct1
{
  vec_1 : Vec< String >,
  hashmap_1 : std::collections::HashMap< String, String >,
  hashset_1 : std::collections::HashSet< String >,
}

// == begin of generated

#[ automatically_derived ]
impl< > Struct1< >
where
{
  #[ inline( always ) ]
  pub fn former() -> Struct1Former< Struct1FormerDefinition< (), Struct1< >, former::ReturnPreformed > >
  {
    Struct1Former::< Struct1FormerDefinition< (), Struct1< >, former::ReturnPreformed > >::new_coercing( former::ReturnPreformed )
  }
}

// = types

#[ derive( Debug ) ]
pub struct Struct1FormerDefinitionTypes< __Context = (), __Formed = Struct1< >, >
where
{
  _phantom : core::marker::PhantomData< ( __Context, __Formed ) >,
}

impl< __Context, __Formed, > ::core::default::Default for Struct1FormerDefinitionTypes< __Context, __Formed, >
where
{
  fn default() -> Self
  {
    Self
    {
      _phantom : core::marker::PhantomData,
    }
  }
}

impl< __Context, __Formed, > former::FormerDefinitionTypes
for Struct1FormerDefinitionTypes< __Context, __Formed, >
where
{
  type Storage = Struct1FormerStorage< >;
  type Formed = __Formed;
  type Context = __Context;
}

// = definition

#[ derive( Debug ) ]
pub struct Struct1FormerDefinition< __Context = (), __Formed = Struct1< >, __End = former::ReturnPreformed, >
where
{
  _phantom : core::marker::PhantomData< ( __Context, __Formed, __End ) >,
}

impl< __Context, __Formed, __End, > ::core::default::Default for Struct1FormerDefinition< __Context, __Formed, __End, >
where
{
  fn default() -> Self
  {
    Self
    {
      _phantom : core::marker::PhantomData,
    }
  }
}

impl< __Context, __Formed, __End, > former::FormerDefinition
for Struct1FormerDefinition< __Context, __Formed, __End, >
where
  __End : former::FormingEnd< Struct1FormerDefinitionTypes< __Context, __Formed, > >,
{
  type Types = Struct1FormerDefinitionTypes< __Context, __Formed, >;
  type End = __End;
  type Storage = Struct1FormerStorage< >;
  type Formed = __Formed;
  type Context = __Context;
}

// = storage

pub struct Struct1FormerStorage< >
where
{
  pub vec_1 : ::core::option::Option< Vec< String > >,
  pub hashmap_1 : ::core::option::Option< std::collections::HashMap< String, String > >,
  pub hashset_1 : ::core::option::Option< std::collections::HashSet< String > >,
}

impl< > ::core::default::Default for Struct1FormerStorage< >
where
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

impl< > former::Storage for Struct1FormerStorage< >
where
{
  type Formed = Struct1< >;
}

impl< > former::StoragePreform for Struct1FormerStorage< >
where
{
  type Preformed = Struct1< >;

  fn preform( mut self ) -> Self::Preformed
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
          fn maybe_default( self : & Self ) -> T
          {
            panic!( "Field 'vec_1' isn't initialized" )
          }
        }

        impl< T > MaybeDefault< T > for &::core::marker::PhantomData< T >
        {
        }

        impl< T > MaybeDefault< T > for ::core::marker::PhantomData< T >
        where T : ::core::default::Default,
        {
          fn maybe_default( self : & Self ) -> T
          {
            T::default()
          }
        }

        (&::core::marker::PhantomData::< Vec< String > >).maybe_default()
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
          fn maybe_default( self : & Self ) -> T
          {
            panic!( "Field 'hashmap_1' isn't initialized" )
          }
        }

        impl< T > MaybeDefault< T > for &::core::marker::PhantomData< T >
        {
        }

        impl< T > MaybeDefault< T > for ::core::marker::PhantomData< T >
        where T : ::core::default::Default,
        {
          fn maybe_default( self : & Self ) -> T
          {
            T::default()
          }
        }

        (&::core::marker::PhantomData::< std::collections::HashMap< String, String > >).maybe_default()
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

        impl< T > MaybeDefault< T > for &::core::marker::PhantomData< T >
        {
        }

        impl< T > MaybeDefault< T > for ::core::marker::PhantomData< T >
        where T : ::core::default::Default,
        {
          fn maybe_default( self : &Self ) -> T
          {
            T::default()
          }
        }

        (&::core::marker::PhantomData::< std::collections::HashSet< String > >).maybe_default()
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

pub struct Struct1Former< Definition = Struct1FormerDefinition< (), Struct1< >, former::ReturnPreformed >, >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage< > >,
{
  storage : < Definition::Types as former::FormerDefinitionTypes >::Storage,
  context : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Context >,
  on_end : core::option::Option< Definition::End >,
}

#[ automatically_derived ]
impl< Definition, > Struct1Former< Definition, >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage< > >,
{

  #[ inline( always ) ]
  pub fn new_precise( on_end : Definition::End ) -> Self
  {
    Self::begin_coercing( None, None, on_end )
  }

  #[ inline( always ) ]
  pub fn new_coercing< IntoEnd >( end : IntoEnd ) -> Self
  where
    IntoEnd : Into< Definition::End >,
  {
    Self::begin_coercing( None, None, end )
  }

  #[ inline( always ) ]
  pub fn begin_precise
  (
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

  #[ inline( always ) ]
  pub fn begin_coercing< IntoEnd >(
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

  #[ inline( always ) ]
  pub fn form( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
  {
    self.end()
  }

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
    Former2 : former::FormerBegin< former::VectorDefinition< String, Self, Self, Struct1FormerAssignVec1End, > >,
  {
    Former2::former_begin( None, Some( self ), Struct1FormerAssignVec1End )
  }

  #[ inline( always ) ]
  pub fn vec_1( self ) ->
  former::ContainerSubformer::< String, former::VectorDefinition< String, Self, Self, Struct1FormerAssignVec1End > >
  {
    self.vec_1_set::< former::ContainerSubformer::< String, former::VectorDefinition< String, Self, Self, Struct1FormerAssignVec1End > >>()
  }

  #[ inline( always ) ]
  pub fn hashmap_1_set< Former2 >( self ) -> Former2
  where
    Former2 : former::FormerBegin< former::HashMapDefinition< String, String, Self, Self, Struct1FormerAssignHashmap1End, > >,
  {
    Former2::former_begin( None, Some( self ), Struct1FormerAssignHashmap1End )
  }


  #[ inline( always ) ]
  pub fn hashmap_1( self ) -> former::ContainerSubformer::< (String, String), former::HashMapDefinition< String, String, Self, Self, Struct1FormerAssignHashmap1End > >
  {
    self.hashmap_1_set::< former::ContainerSubformer::< (String, String), former::HashMapDefinition< String, String, Self, Self, Struct1FormerAssignHashmap1End > >>()
  }


  #[ inline( always ) ]
  pub fn hashset_1_set< Former2 >( self ) -> Former2
  where
    Former2 : former::FormerBegin< former::HashSetDefinition< String, Self, Self, Struct1FormerAssignHashset1End, > >,
  {
    Former2::former_begin( None, Some( self ), Struct1FormerAssignHashset1End )
  }


  #[ inline( always ) ]
  pub fn hashset_1( self ) -> former::ContainerSubformer::< String, former::HashSetDefinition< String, Self, Self, Struct1FormerAssignHashset1End > >
  {
    self.hashset_1_set::< former::ContainerSubformer::< String, former::HashSetDefinition< String, Self, Self, Struct1FormerAssignHashset1End > >>()
  }
}

// = former :: preform

impl< Definition, > Struct1Former< Definition, >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage< >, Formed = Struct1< > >,
  < Definition::Types as former::FormerDefinitionTypes >::Storage : former::StoragePreform,
  < Definition::Types as former::FormerDefinitionTypes >::Storage : former::StoragePreform< Preformed = Struct1< > >,
{
  pub fn preform( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
  {
    former::StoragePreform::preform( self.storage )
  }
}

// = former :: perform

#[ automatically_derived ]
impl< Definition, > Struct1Former< Definition, >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage< >, Formed = Struct1< > >,
{
  #[ inline( always ) ]
  pub fn perform( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
  {
    let result = self.form();
    return result;
  }
}

// = subformer

#[ allow( dead_code ) ]
pub type Struct1Subformer< __Superformer, __End >
= Struct1Former< Struct1FormerDefinition< __Superformer, __Superformer, __End, >, >;

// pub type Struct1FormerWithClosure< __Context, __Formed, > = Struct1FormerDefinition< __Context, __Formed, former::FormingEndClosure< Struct1FormerDefinitionTypes< __Context, __Formed, > > >;

// = subformer end

#[ allow( dead_code ) ]
pub trait Struct1SubformerEnd< SuperFormer >
where
  Self : former::FormingEnd< Struct1FormerDefinitionTypes< SuperFormer, SuperFormer >, >,
{}

impl< SuperFormer, T > Struct1SubformerEnd< SuperFormer > for T
where
  Self : former::FormingEnd< Struct1FormerDefinitionTypes< SuperFormer, SuperFormer >, >,
{}

// = end handlers

#[ allow( non_camel_case_types ) ]
pub struct Struct1FormerAssignVec1End;

#[ automatically_derived ]
impl< Definition, > former::FormingEnd
< former::VectorDefinition< String, Struct1Former< Definition, >, Struct1Former< Definition, >, former::NoEnd >, >
for Struct1FormerAssignVec1End
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage< > >,
{
  #[ inline( always ) ]
  fn call( &self, storage : Vec< String >, super_former : Option< Struct1Former< Definition, > >, ) ->
  Struct1Former< Definition, >
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

#[ allow( non_camel_case_types ) ]
pub struct Struct1FormerAssignHashmap1End;

#[ automatically_derived ]
impl< Definition, >
former::FormingEnd
<
  former::HashMapDefinition< String, String, Struct1Former< Definition, >, Struct1Former< Definition, >, former::NoEnd >,
>
for Struct1FormerAssignHashmap1End
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage< > >,
{
  #[ inline( always ) ]
  fn call( &self, storage : std::collections::HashMap< String, String >, super_former : Option< Struct1Former< Definition, > >, ) -> Struct1Former< Definition, >
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

#[ allow( non_camel_case_types ) ]
pub struct Struct1FormerAssignHashset1End;

#[ automatically_derived ]
impl< Definition, > former::FormingEnd
<
  former::HashSetDefinition< String, Struct1Former< Definition, >, Struct1Former< Definition, >, former::NoEnd >,
>
for Struct1FormerAssignHashset1End
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage< > >,
{
  #[ inline( always ) ]
  fn call( &self, storage : std::collections::HashSet< String >, super_former : Option< Struct1Former< Definition, > >, ) -> Struct1Former< Definition, >
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

// == end of generated

include!( "./only_test/containers_with_subformer.rs" );
