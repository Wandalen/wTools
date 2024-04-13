#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq ) ]
pub struct Struct1
{
  pub int_1 : i32,
}

// === begin of generated

// = formed

#[ automatically_derived ]
impl Struct1
{

  #[ inline( always ) ]
  pub fn former() -> Struct1Former< >
  {
    Struct1Former::< >::new( former::ReturnPreformed )
  }
}

// = definition types

#[ derive( Debug ) ]
pub struct Struct1FormerDefinitionTypes< Context = (), Formed = Struct1 >
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

impl< Context, Formed > former::FormerDefinitionTypes for Struct1FormerDefinitionTypes< Context, Formed >
{
  type Storage = Struct1FormerStorage;
  type Formed = Formed;
  type Context = Context;
}

// = definition

#[ derive( Debug ) ]
pub struct Struct1FormerDefinition< Context = (), Formed = Struct1, End = former::ReturnPreformed >
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
where End : former::FormingEnd< Struct1FormerDefinitionTypes< Context, Formed > >
{
  type Types = Struct1FormerDefinitionTypes< Context, Formed >;
  type End = End;
}

pub type Struct1FormerWithClosure< Context, Formed > =
  Struct1FormerDefinition< Context, Formed, former::FormingEndClosure< Struct1FormerDefinitionTypes< Context, Formed > > >;

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
  type Formed = Struct1;
}

impl former::StoragePreform for Struct1FormerStorage
{
  fn preform( mut self ) -> < Self as former::Storage >::Formed
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

// = former

pub struct Struct1Former< Definition = Struct1FormerDefinition >
where Definition : former::FormerDefinition, Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage >,
{
  storage : < Definition::Types as former::FormerDefinitionTypes >::Storage,
  context : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Context >,
  on_end : core::option::Option< Definition::End >,
}

#[ automatically_derived ]
impl< Definition > Struct1Former< Definition >
where Definition : former::FormerDefinition, Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage >,
{

  #[ inline( always ) ]
  pub fn perform( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
  {
    let result = self.form();
    return result;
  }

  #[ inline( always ) ]
  pub fn _new_precise( on_end : Definition::End ) -> Self
  {
    Self::begin( None, None, on_end )
  }

  #[ inline( always ) ]
  pub fn new< IntoEnd >( end : IntoEnd ) -> Self
  where IntoEnd : Into< Definition::End >,
  {
    Self::begin( None, None, end, )
  }

  #[ inline( always ) ]
  pub fn _begin_precise
  (
    mut storage : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Storage >,
    context : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Context >,
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
  pub fn begin< IntoEnd >
  (
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
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage, Formed = Struct1 >,
  < Definition::Types as former::FormerDefinitionTypes >::Storage : former::StoragePreform,
{
  pub fn preform( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
  {
    former::StoragePreform::preform( self.storage )
  }
}

// === end of generated

include!( "./only_test/basic.rs" );