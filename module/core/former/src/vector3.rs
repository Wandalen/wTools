use super::*;
use axiomatic::*;

#[ allow( unused ) ]
use collection_tools::Vec;

/// Trait for containers that behave like a vector, providing an interface for element addition.
///
/// This trait enables the use of custom or standard vector-like containers within the builder pattern,
/// allowing for a unified and flexible approach to constructing collections.
///
pub trait VectorLike< E >
{
  /// Appends an element to the back of a formed.
  fn push( &mut self, element : E );
}

impl< E > VectorLike< E > for Vec< E >
{
  fn push( &mut self, element : E )
  {
    Vec::push( self, element );
  }
}

pub struct VectorDescriptor< E >
{
  _phantom : core::marker::PhantomData< E >,
}

impl< E > VectorDescriptor< E >
{
  fn new() -> Self
  {
    Self { _phantom : core::marker::PhantomData }
  }
}

impl< E > StoragePerform
for Vec< E >
{
  type Formed = Self;
  fn preform( self ) -> Self::Formed
  {
    self
  }
}

impl< E > FormerDescriptor
for VectorDescriptor< E >
{
  type Storage = Vec< E >;
  type Formed = Vec< E >;
}

/// A builder for constructing `VectorLike` containers, facilitating a fluent and flexible interface.
///
/// `VectorSubformer` leverages the `VectorLike` trait to enable the construction and manipulation
/// of vector-like containers in a builder pattern style, promoting readability and ease of use.
#[ derive( Debug, Default ) ]
pub struct VectorSubformer< E, Context, End >
where
  End : FormingEnd< VectorDescriptor< E >, Context >,
{
  formed : core::option::Option< < VectorDescriptor< E > as axiomatic::FormerDescriptor >::Formed >,
  context : core::option::Option< Context >,
  on_end : core::option::Option< End >,
}

impl< E, Context, End > VectorSubformer< E, Context, End >
where
  End : FormingEnd< VectorDescriptor< E >, Context >,
{

  /// Form current former into target structure.
  #[ inline( always ) ]
  pub fn form( mut self ) -> < VectorDescriptor< E > as axiomatic::FormerDescriptor >::Formed
  {
    let formed = if self.formed.is_some()
    {
      self.formed.take().unwrap()
    }
    else
    {
      let val = Default::default();
      val
    };
    formed
  }

  /// Begins the building process, optionally initializing with a context and formed.
  #[ inline( always ) ]
  pub fn begin
  (
    formed : core::option::Option< < VectorDescriptor< E > as axiomatic::FormerDescriptor >::Formed >,
    context : core::option::Option< Context >,
    on_end : End
  ) -> Self
  {
    Self
    {
      context,
      formed,
      on_end : Some( on_end ),
    }
  }

  /// Finalizes the building process, returning the formed or a context incorporating it.
  #[ inline( always ) ]
  pub fn end( mut self ) -> < VectorDescriptor< E > as axiomatic::FormerDescriptor >::Formed
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    let formed = self.form();
    on_end.call( formed, context )
  }

  /// Replaces the current formed with a provided one, allowing for a reset or redirection of the building process.
  #[ inline( always ) ]
  pub fn replace( mut self, vector : < VectorDescriptor< E > as axiomatic::FormerDescriptor >::Formed ) -> Self
  {
    self.formed = Some( vector );
    self
  }

}

impl< E > VectorSubformer< E, (), ReturnStorage >
where
{

  /// Initializes a new `VectorSubformer` instance, starting with an empty formed.
  /// This function serves as the entry point for the builder pattern.
  ///
  /// # Returns
  /// A new instance of `VectorSubformer` with an empty internal formed.
  ///
  #[ inline( always ) ]
  pub fn new() -> Self
  {
    Self::begin
    (
      None,
      None,
      ReturnStorage,
    )
  }

}

impl< E, Context, End > VectorSubformer< E, Context, End >
where
  End : FormingEnd< VectorDescriptor< E >, Context >,
{

  /// Appends an element to the end of the formed, expanding the internal collection.
  #[ inline( always ) ]
  pub fn push< IntoElement >( mut self, element : IntoElement ) -> Self
  where IntoElement : core::convert::Into< E >,
  {
    if self.formed.is_none()
    {
      self.formed = core::option::Option::Some( Default::default() );
    }
    if let core::option::Option::Some( ref mut formed ) = self.formed
    {
      formed.push( element.into() );
    }
    self
  }

}

//

// impl< Former, Context, End > FormerBegin< Formed, Formed, Context >
// for VectorSubformer< Former, Context, End >
// where
//   End : FormingEnd< VectorDescriptor< E >, Context >,
//   // Formed : VectorLike< E > + Default,
//   Former : FormerDescriptor,
// {
//   type End = End;
//
//   #[ inline( always ) ]
//   fn _begin
//   (
//     formed : core::option::Option< Formed >,
//     context : core::option::Option< Context >,
//     on_end : End,
//   ) -> Self
//   {
//     Self::begin( formed, context, on_end )
//   }
//
// }
