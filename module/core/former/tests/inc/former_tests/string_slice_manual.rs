#[ allow( unused_imports ) ]
use super::*;

#[ derive(  Debug, PartialEq  ) ]
pub struct Struct1< 'a >
{
  pub string_slice_1 : &'a str,
}

// === begin_coercing of generated

#[ automatically_derived ]
impl< 'a > Struct1< 'a >
{

  #[ inline( always ) ]
  pub fn former() -> Struct1Former< 'a >
  {
    Struct1Former::new_coercing( former::ReturnPreformed )
  }
}

// = definition types

#[ derive( Debug ) ]
// pub struct Struct1FormerDefinitionTypes< 'a, Context = (), Formed = Struct1< 'a > >
pub struct Struct1FormerDefinitionTypes< 'a, Context, Formed >
{
  _phantom : core::marker::PhantomData< ( &'a(), Context, Formed ) >,
}

impl< 'a, Context, Formed > Default for Struct1FormerDefinitionTypes< 'a, Context, Formed >
{
  fn default() -> Self
  {
    Self { _phantom : core::marker::PhantomData, }
  }
}

impl< 'a, Context, Formed > former::FormerDefinitionTypes
for Struct1FormerDefinitionTypes< 'a, Context, Formed >
{
  type Storage = Struct1FormerStorage< 'a >;
  type Formed = Formed;
  type Context = Context;
}

// = former mutator

impl< 'a, Context, Formed > former::FormerMutator
for Struct1FormerDefinitionTypes< 'a, Context, Formed >
{
}

// = definition

#[ derive( Debug ) ]
// pub struct Struct1FormerDefinition< 'a, Context = (), Formed = Struct1< 'a >, End = former::ReturnPreformed >
pub struct Struct1FormerDefinition< 'a, Context, Formed, End >
{
  _phantom : core::marker::PhantomData< ( &'a(), Context, Formed, End ) >,
}

impl< 'a, Context, Formed, End > Default for Struct1FormerDefinition< 'a, Context, Formed, End >
{
  fn default() -> Self
  {
    Self { _phantom : core::marker::PhantomData, }
  }
}

impl< 'a, Context, Formed, End > former::FormerDefinition
for Struct1FormerDefinition< 'a, Context, Formed, End >
where
  End : former::FormingEnd< Struct1FormerDefinitionTypes< 'a, Context, Formed > >
{
  type Types = Struct1FormerDefinitionTypes< 'a, Context, Formed >;
  type End = End;
  type Storage = Struct1FormerStorage< 'a >;
  type Formed = Formed;
  type Context = Context;
}

// pub type Struct1FormerWithClosure< 'a, Context, Formed > =
//   Struct1FormerDefinition< 'a, Context, Formed, former::FormingEndClosure< Struct1FormerDefinitionTypes< 'a, Context, Formed > > >;

// = storage

pub struct Struct1FormerStorage< 'a >
{
  pub string_slice_1 : ::core::option::Option< &'a str >,
}

impl< 'a > ::core::default::Default for Struct1FormerStorage< 'a >
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    Self { string_slice_1 : ::core::option::Option::None, }
  }
}

impl< 'a > former::Storage for Struct1FormerStorage< 'a >
{
  type Preformed = Struct1< 'a >;
}

impl< 'a > former::StoragePreform for Struct1FormerStorage< 'a >
{
  // type Preformed = Struct1< 'a >;

  fn preform( mut self ) -> Self::Preformed
  // fn preform( mut self ) -> < Self as former::Storage >::Formed
  // fn preform( mut self ) -> Struct1< 'a >
  {
    let string_slice_1 = if self.string_slice_1.is_some()
    {
      self.string_slice_1.take().unwrap()
    }
    else
    {
      {
        trait MaybeDefault< T >
        {
          fn maybe_default( self : & Self ) -> T
          {
            panic!( "Field 'string_slice_1' isn't initialized" )
          }
        }

        impl< T > MaybeDefault< T > for & ::core::marker::PhantomData< T > {}
        impl< T > MaybeDefault< T > for ::core::marker::PhantomData< T >
        where T : ::core::default::Default,
        {
          fn maybe_default( self : & Self ) -> T { T::default() }
        }

        (& ::core::marker::PhantomData::< &'a str >).maybe_default()
      }
    };
    let result = Struct1 { string_slice_1, };
    return result;
  }
}

// = former

pub struct Struct1Former< 'a, Definition = Struct1FormerDefinition< 'a, (), Struct1< 'a >, former::ReturnPreformed > >
where
  // End : former::FormingEnd::< Definition::Types >,
  // Definition : former::FormerDefinition< End = End >,
  // Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage< 'a >, Formed = Formed, Context = Context >,
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage< 'a > >,
{
  storage : Definition::Storage,
  context : core::option::Option< Definition::Context >,
  on_end : core::option::Option< Definition::End >,
}

#[ automatically_derived ]
impl< 'a, Definition > Struct1Former< 'a, Definition >
where
  Definition : former::FormerDefinition< Storage = Struct1FormerStorage< 'a > >,
  // Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage< 'a > >,
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
  pub fn begin_coercing< IntoEnd >
  (
    mut storage : core::option::Option< Definition::Storage >,
    context : core::option::Option< Definition::Context >,
    on_end : IntoEnd,
  ) -> Self
  where IntoEnd : ::core::convert::Into< < Definition as former::FormerDefinition >::End >,
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
  pub fn string_slice_1< Src >( mut self, src : Src ) -> Self
  where Src : ::core::convert::Into< &'a str >,
  {
    debug_assert!( self.storage.string_slice_1.is_none() );
    self.storage.string_slice_1 = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
    self
  }
}

impl< 'a, Definition > Struct1Former< 'a, Definition >
where
  Definition : former::FormerDefinition< Storage = Struct1FormerStorage< 'a >, Formed = Struct1< 'a > >,
  // Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage< 'a >, Formed = Struct1< 'a > >,
  Definition::Storage : former::StoragePreform< Preformed = Struct1< 'a > >,
{
  pub fn preform( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
  {
    // panic!();
    former::StoragePreform::preform( self.storage )
  }
}

// === end of generated

include!( "./only_test/string_slice.rs" );
