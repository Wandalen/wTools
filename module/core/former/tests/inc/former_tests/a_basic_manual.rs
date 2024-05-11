#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq ) ]
pub struct Struct1
{
  pub int_1 : i32,
}

// == begin of generated

// = formed

#[ automatically_derived ]
impl Struct1
{

  #[ inline( always ) ]
  pub fn former() -> Struct1Former< Struct1FormerDefinition< (), Struct1, former::ReturnPreformed > >
  {
    Struct1Former
    ::< Struct1FormerDefinition< (), Struct1, former::ReturnPreformed > >
    ::new( former::ReturnPreformed )
  }

}

// = entity to former

impl< Definition > former::EntityToFormer< Definition > for Struct1
where
  Definition : former::FormerDefinition< Storage = Struct1FormerStorage >,
{
  type Former = Struct1Former< Definition >;
}

impl former::EntityToStorage for Struct1
{
  type Storage = Struct1FormerStorage;
}

impl< Context, Formed, End > former::EntityToDefinition< Context, Formed, End >
for Struct1
where
  End : former::FormingEnd< Struct1FormerDefinitionTypes< Context, Formed > >,
{
  type Definition = Struct1FormerDefinition< Context, Formed, End >;
  type Types = Struct1FormerDefinitionTypes< Context, Formed >;
}

impl< Context, Formed > former::EntityToDefinitionTypes< Context, Formed >
for Struct1
{
  type Types = Struct1FormerDefinitionTypes< Context, Formed >;
}

// = definition types

#[ derive( Debug ) ]
// pub struct Struct1FormerDefinitionTypes< Context = (), Formed = Struct1 >
pub struct Struct1FormerDefinitionTypes< Context, Formed >
{
  _phantom : core::marker::PhantomData< ( Context, Formed ) >,
}

impl< Context, Formed > Default for Struct1FormerDefinitionTypes< Context, Formed >
{
  fn default() -> Self
  {
    Self { _phantom : core::marker::PhantomData, }
  }
}

impl< Context, Formed > former::FormerDefinitionTypes
for Struct1FormerDefinitionTypes< Context, Formed >
{
  type Storage = Struct1FormerStorage;
  type Formed = Formed;
  type Context = Context;
}

// = definition

#[ derive( Debug ) ]
// pub struct Struct1FormerDefinition< Context = (), Formed = Struct1, End = former::ReturnPreformed >
pub struct Struct1FormerDefinition< Context, Formed, End >
{
  _phantom : core::marker::PhantomData< ( Context, Formed, End ) >,
}

impl< Context, Formed, End > Default for Struct1FormerDefinition< Context, Formed, End >
{
  fn default() -> Self
  {
    Self { _phantom : core::marker::PhantomData, }
  }
}

impl< Context, Formed, End > former::FormerDefinition for Struct1FormerDefinition< Context, Formed, End >
where
  End : former::FormingEnd< Struct1FormerDefinitionTypes< Context, Formed > >
{
  type Storage = Struct1FormerStorage;
  type Formed = Formed;
  type Context = Context;
  type Types = Struct1FormerDefinitionTypes< Context, Formed >;
  type End = End;
}

// pub type Struct1FormerWithClosure< Context, Formed > =
//   Struct1FormerDefinition< Context, Formed, former::FormingEndClosure< Struct1FormerDefinitionTypes< Context, Formed > > >;

// = storage

pub struct Struct1FormerStorage
{
  pub int_1 : ::core::option::Option< i32 >,
}

impl ::core::default::Default for Struct1FormerStorage
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    Self { int_1 : ::core::option::Option::None, }
  }
}

impl former::Storage for Struct1FormerStorage
{
  type Preformed = Struct1;
}

impl former::StoragePreform for Struct1FormerStorage
{
  // type Preformed = < Self as former::Storage >::Formed;
  fn preform( mut self ) -> Self::Preformed
  {
    let int_1 = if self.int_1.is_some()
    {
      self.int_1.take().unwrap()
    }
    else
    {
      {
        trait MaybeDefault< T >
        {
          fn maybe_default( self : & Self ) -> T
          {
            panic!( "Field 'int_1' isn't initialized" )
          }
        }

        impl< T > MaybeDefault< T > for & ::core::marker::PhantomData< T > {}
        impl< T > MaybeDefault< T > for ::core::marker::PhantomData< T >
        where T : ::core::default::Default,
        {
          fn maybe_default( self : & Self ) -> T { T::default() }
        }

        (& ::core::marker::PhantomData::< i32 >).maybe_default()
      }
    };
    let result = Struct1 { int_1, };
    return result;
  }
}

// = former mutator

impl< Context, Formed > former::FormerMutator
for Struct1FormerDefinitionTypes< Context, Formed >
{
}

// = former

pub struct Struct1Former
<
  Definition = Struct1FormerDefinition< (), Struct1, former::ReturnPreformed >,
>
where
  Definition : former::FormerDefinition< Storage = Struct1FormerStorage >,
{
  storage : Definition::Storage,
  context : ::core::option::Option< Definition::Context >,
  on_end : ::core::option::Option< Definition::End >,
}

#[ automatically_derived ]
impl< Definition > Struct1Former< Definition >
where
  Definition : former::FormerDefinition< Storage = Struct1FormerStorage >,

{

  #[ inline( always ) ]
  pub fn perform( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
  {
    let result = self.form();
    return result;
  }

  #[ inline( always ) ]
  pub fn new( on_end : Definition::End ) -> Self
  {
    Self::begin_coercing( None, None, on_end )
  }

  #[ inline( always ) ]
  pub fn new_coercing< IntoEnd >( end : IntoEnd ) -> Self
  where IntoEnd : Into< Definition::End >,
  {
    Self::begin_coercing( None, None, end, )
  }

  #[ inline( always ) ]
  pub fn begin
  (
    mut storage : core::option::Option< Definition::Storage >,
    context : core::option::Option< Definition::Context >,
    on_end : < Definition as former::FormerDefinition >::End,
  )
  -> Self
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
  pub fn begin_coercing< IntoEnd >
  (
    mut storage : core::option::Option< Definition::Storage >,
    context : core::option::Option< Definition::Context >,
    on_end : IntoEnd,
  )
  -> Self
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
    let mut context = self.context.take();
    < Definition::Types as former::FormerMutator >::form_mutation( &mut self.storage, &mut context );
    former::FormingEnd::< Definition::Types >::call( & on_end, self.storage, context )
  }

  #[ inline ]
  pub fn int_1< Src >( mut self, src : Src ) -> Self
  where Src : ::core::convert::Into< i32 >,
  {
    debug_assert!( self.storage.int_1.is_none() );
    self.storage.int_1 = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
    self
  }

}

// = preform with Storage::preform

impl< Definition > Struct1Former< Definition >
where
  Definition : former::FormerDefinition< Storage = Struct1FormerStorage, Formed = Struct1 >,
  Definition::Storage : former::StoragePreform< Preformed = Struct1 >,

{
  pub fn preform( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
  {
    former::StoragePreform::preform( self.storage )
  }
}

impl< Definition > former::FormerBegin< Definition >
for Struct1Former< Definition >
where
  Definition : former::FormerDefinition< Storage = Struct1FormerStorage >,

{

  #[ inline( always ) ]
  fn former_begin
  (
    storage : core::option::Option< Definition::Storage >,
    context : core::option::Option< Definition::Context >,
    on_end : Definition::End,
  )
  -> Self
  {
    debug_assert!( storage.is_none() );
    Self::begin( None, context, on_end )
  }

}

// == end of generated

include!( "./only_test/basic.rs" );
